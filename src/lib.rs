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

    #[error("BadEncoding")]
    BadEncoding,
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
        let mut buf = [0; 4];
        stream.read_exact(&mut buf)?;
        if &buf != b"home" {
            let msg = format!("recieved string is: {:?}", buf);
            return Err(SocketClientError::BadHandshake(msg));
        }
        Ok(Self { stream })
    }

    pub fn get_power(&mut self) -> SocketClientResult<String> {
        let send_buf = "get power".as_bytes();
        let mut recieve_buf_len = [0; 4];
        let send_len = send_buf.len() as u32;
        self.stream.write_all(send_len.to_be_bytes().as_slice())?;
        self.stream.write_all(send_buf)?;
        self.stream.read_exact(&mut recieve_buf_len)?;
        let recieve_len = u32::from_be_bytes(recieve_buf_len);
        let mut recieve_buf = vec![0; recieve_len as _];
        println!("Response len is: {}", recieve_len);
        self.stream.read_exact(&mut recieve_buf)?;
        String::from_utf8(recieve_buf).map_err(|_| SocketClientError::BadEncoding)
    }

    pub fn switch(&mut self) -> SocketClientResult<String> {
        let send_buf = "switch it".as_bytes();
        let mut recieve_buf_len = [0; 4];
        let send_len = send_buf.len() as u32;
        self.stream.write_all(send_len.to_be_bytes().as_slice())?;
        self.stream.write_all(send_buf)?;
        self.stream.read_exact(&mut recieve_buf_len)?;
        let recieve_len = u32::from_be_bytes(recieve_buf_len);
        let mut recieve_buf = vec![0; recieve_len as _];
        self.stream.read_exact(&mut recieve_buf)?;
        String::from_utf8(recieve_buf).map_err(|_| SocketClientError::BadEncoding)
    }
}
