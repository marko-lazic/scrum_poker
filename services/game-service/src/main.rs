use Instrument;
use common::{CardRequest, CardResponse};
use tracing::*;
use wtransport::endpoint::IncomingSession;
use wtransport::{Endpoint, Identity, ServerConfig};

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    common::init_logging()?;

    let config = ServerConfig::builder()
        .with_bind_default(4433)
        .with_identity(Identity::self_signed(["localhost"]).unwrap())
        .build();

    let server = Endpoint::server(config)?;

    info!("Server ready! On https://127.0.0.1:4433");

    for id in 0.. {
        let incoming_session = server.accept().await;
        tokio::spawn(handle_connection(incoming_session).instrument(info_span!("Connection", id)));
    }

    Ok(())
}

async fn handle_connection(incoming_session: IncomingSession) {
    let result = handle_connection_impl(incoming_session).await;
    if let Err(e) = result {
        error!("Connection error: {:?}", e);
    }
}

async fn handle_connection_impl(incoming_session: IncomingSession) -> anyhow::Result<()> {
    let mut buffer = vec![0; 65536].into_boxed_slice();

    info!("Waiting for session request...");

    let session_request = incoming_session.await?;

    info!(
        "New session: Authority: '{}', Path: '{}'",
        session_request.authority(),
        session_request.path()
    );

    let connection = session_request.accept().await?;

    info!("Waiting for data from client...");

    let (mut send_stream, mut recv_stream) = connection.accept_bi().await?;
    info!("Accepted BI stream");

    let bytes_read = match recv_stream.read(&mut buffer).await? {
        Some(bytes_read) => {
            if bytes_read == buffer.len() {
                warn!(
                    "Message might have been truncated - buffer of {} bytes was filled completely",
                    buffer.len()
                );
            }
            bytes_read
        }
        None => {
            warn!("Client closed the stream without sending data");
            return Ok(());
        }
    };

    // Deserialize the action from MessagePack
    let request = rmp_serde::from_slice::<CardRequest>(&buffer[..bytes_read])?;
    info!("Received (bi) '{request:?}' from client");

    // Process the action (statelessly)
    let response = CardResponse {
        status: "success".to_string(),
        message: format!("Player {} chosen {}", request.player_id, request.card),
    };

    // Serialize the response to MessagePack
    let response_bytes = rmp_serde::to_vec(&response)?;

    // Send the response back to the client
    send_stream.write_all(&response_bytes).await?;
    send_stream.finish().await?; // Close the stream
    connection.close(0u32.into(), b"Server is done");
    Ok(())
}
