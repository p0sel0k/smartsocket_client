use anyhow::Result;
use smartsocket_client::SocketClient;

fn main() -> Result<()> {
    let addr = "localhost:8095";
    let mut client = SocketClient::connect_to_socket(addr)?;
    println!("Client succcessfully connected to SmartSocket");
    println!(
        "Socket on {} power has power: {} ",
        addr,
        client.get_power()?
    );
    Ok(())
}
