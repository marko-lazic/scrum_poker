use tracing::{info, warn};
use wtransport::{ClientConfig, Endpoint};
use common::{CardRequest, CardResponse};

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    common::init_logging()?;
    let config = ClientConfig::builder()
        .with_bind_default()
        .with_no_cert_validation()
        .build();

    let connection = Endpoint::client(config)?
        .connect("https://[::1]:4433")
        .await?;

    let request = CardRequest {
        player_id: "123".to_string(),
        room_id: "sp".to_string(),
        event: "CARD_CHOSEN".to_string(),
        card: "5".to_string(),
    };

    let serialized = rmp_serde::to_vec(&request).expect("Serialization failed");
    println!("Serialized CardRequest (MessagePack): {:?}", serialized);

    let (mut send_stream, mut recv_stream) = connection.open_bi().await?.await?;

    send_stream.write_all(serialized.as_slice()).await?;
    send_stream.finish().await?;

    let mut buffer = vec![0; 65536].into_boxed_slice();
    if let Some(bytes_read) = recv_stream.read(&mut buffer).await? {
        if bytes_read > 0 {
            // Deserialize the response
            let response = rmp_serde::from_slice::<CardResponse>(&buffer[..bytes_read])?;
            info!("Received response: {:?}", response);
        } else {
            warn!("Received an empty response from the server");
        }
    } else {
        warn!("Failed to read response from the server");
    }

    // Close the connection
    connection.close(0u32.into(), b"Client is done");

    Ok(())
}