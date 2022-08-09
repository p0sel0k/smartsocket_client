use anyhow::Result;
use smartsocket_client::SocketClient;

#[tokio::main]
async fn main() -> Result<()> {
    let addr = "127.0.0.1:8095";
    let mut client = SocketClient::connect_to_socket(addr).await?;
    println!("Client succcessfully connected to SmartSocket");
    println!(
        "Socket on {} has power: {} ",
        addr,
        client.get_power().await?
    );
    Ok(())
}
