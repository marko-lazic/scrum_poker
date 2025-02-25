use common::prelude::*;
use tracing::{info, warn};
use wtransport::{ClientConfig, Endpoint};

/// Reads and parses an RPC response from a stream
async fn read_response(recv_stream: &mut wtransport::RecvStream) -> anyhow::Result<RpcResponse> {
    let mut buffer = vec![0; 65536].into_boxed_slice();

    if let Some(bytes_read) = recv_stream.read(&mut buffer).await? {
        if bytes_read > 0 {
            let response = rmp_serde::from_slice::<RpcResponse>(&buffer[..bytes_read])?;
            return Ok(response);
        } else {
            return Err(anyhow::anyhow!(
                "Received an empty response from the server"
            ));
        }
    }

    Err(anyhow::anyhow!("Failed to read response from the server"))
}

/// Sends a request and returns the parsed result if successful
async fn call_api<T: serde::Serialize, R>(
    connection: &wtransport::Connection,
    method: &str,
    params: T,
) -> anyhow::Result<R>
where
    R: serde::de::DeserializeOwned,
{
    // Create the RpcRequest wrapper
    let rpc_request = RpcRequest {
        id: Some(1),
        method: method.to_string(),
        params: rmp_serde::to_vec(&params)?,
    };

    let serialized = rmp_serde::to_vec(&rpc_request)?;

    let (mut send_stream, mut recv_stream) = connection.open_bi().await?.await?;
    send_stream.write_all(serialized.as_slice()).await?;
    send_stream.finish().await?;

    let response = read_response(&mut recv_stream).await?;
    response
        .parse_result::<R>()
        .map_err(|e| anyhow::anyhow!("{}", e))
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    init_logging()?;
    let config = ClientConfig::builder()
        .with_bind_default()
        .with_no_cert_validation()
        .build();

    let connection = Endpoint::client(config)?
        .connect("https://[::1]:4433")
        .await?;

    let request = VoteRequest {
        player_id: "123".to_string(),
        room_id: "sp".to_string(),
        card: "5".to_string(),
    };

    // Use the new API call function
    match call_api::<_, VoteResponse>(&connection, "vote", request).await {
        Ok(result) => {
            info!("Vote successful: {:?}", result);
        }
        Err(e) => {
            warn!("Vote failed: {}", e);
        }
    }

    // Close the connection
    connection.close(0u32.into(), b"Client is done");

    Ok(())
}
