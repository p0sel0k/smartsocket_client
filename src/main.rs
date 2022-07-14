use anyhow::Result;
use smartsocket_client::SocketClient;

fn main() -> Result<()> {
    let addr = "127.0.0.1:8095";
    let mut client = SocketClient::connect_to_socket(addr)?;
    println!("Client succcessfully connected to SmartSocket");
    println!("Socket on {} has power: {} ", addr, client.get_power()?);
    Ok(())
}
