use anyhow::Result;
use std::{
    io::{self, Read, Write},
    net::{TcpStream, ToSocketAddrs},
};
use thiserror::Error;

pub type SocketClientResult<T> = Result<T, SocketClientError>;

#[derive(Debug, Error)]
pub enum SocketClientError {
    #[error("Unexpected handshake: {0}")]
    BadHandshake(String),
    #[error("Io error")]
    Io(#[from] io::Error),
}

pub struct SocketClient {
    stream: TcpStream,
}

impl SocketClient {
    pub fn connect_to_socket<Addr>(addr: Addr) -> SocketClientResult<Self>
    where
        Addr: ToSocketAddrs,
    {
        let stream = TcpStream::connect(addr)?;
        Self::try_handshake(stream)
    }

    fn try_handshake(mut stream: TcpStream) -> SocketClientResult<Self> {
        stream.write_all(b"smart")?;
        let mut buf = [0; 6];
        stream.read_exact(&mut buf)?;
        if &buf != b"socket" {
            let msg = format!("recieved string is: {:?}", buf);
            return Err(SocketClientError::BadHandshake(msg));
        }
        Ok(Self { stream })
    }

    pub fn get_power(&mut self) -> SocketClientResult<u32> {
        let send_buf = "get power".as_bytes();
        let mut recieve_buf = [0; 4];
        let len = send_buf.len() as u32;
        self.stream.write_all(len.to_be_bytes().as_slice())?;
        self.stream.write_all(send_buf)?;
        self.stream.read_exact(&mut recieve_buf)?;
        let power = u32::from_be_bytes(recieve_buf);
        Ok(power)
    }
}
