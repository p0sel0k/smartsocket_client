use anyhow::Result;
use async_io::tcp::{read_exact_async, write_all_async};
use std::io;
use thiserror::Error;
use tokio::net::{TcpStream, ToSocketAddrs};

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
    pub async fn connect_to_socket<Addr>(addr: Addr) -> SocketClientResult<Self>
    where
        Addr: ToSocketAddrs,
    {
        let stream = TcpStream::connect(addr).await?;
        Self::try_handshake(stream).await
    }

    async fn try_handshake(stream: TcpStream) -> SocketClientResult<Self> {
        write_all_async(&stream, b"smart").await?;
        let mut buf = [0; 4];
        read_exact_async(&stream, &mut buf).await?;
        if &buf != b"home" {
            let msg = format!("recieved string is: {:?}", buf);
            return Err(SocketClientError::BadHandshake(msg));
        }
        Ok(Self { stream })
    }

    pub async fn get_power(&mut self) -> SocketClientResult<String> {
        let send_buf = "get power".as_bytes();
        let mut recieve_buf_len = [0; 4];
        let send_len = send_buf.len() as u32;
        write_all_async(&self.stream, send_len.to_be_bytes().as_slice()).await?;
        write_all_async(&self.stream, send_buf).await?;
        read_exact_async(&self.stream, &mut recieve_buf_len).await?;
        let recieve_len = u32::from_be_bytes(recieve_buf_len);
        let mut recieve_buf = vec![0; recieve_len as _];
        println!("Response len is: {}", recieve_len);
        read_exact_async(&self.stream, &mut recieve_buf).await?;
        String::from_utf8(recieve_buf).map_err(|_| SocketClientError::BadEncoding)
    }

    pub async fn switch(&mut self) -> SocketClientResult<String> {
        let send_buf = "switch it".as_bytes();
        let mut recieve_buf_len = [0; 4];
        let send_len = send_buf.len() as u32;
        write_all_async(&self.stream, send_len.to_be_bytes().as_slice()).await?;
        write_all_async(&self.stream, send_buf).await?;
        read_exact_async(&self.stream, &mut recieve_buf_len).await?;
        let recieve_len = u32::from_be_bytes(recieve_buf_len);
        let mut recieve_buf = vec![0; recieve_len as _];
        read_exact_async(&self.stream, &mut recieve_buf).await?;
        String::from_utf8(recieve_buf).map_err(|_| SocketClientError::BadEncoding)
    }
}
