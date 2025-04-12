use std::{
    io::{Read, Write},
    net::SocketAddr,
};

use anyhow::{Context, Result};
use socket2::{Domain, Protocol, Socket, Type};

pub struct SocketLog {
    socket: Socket,
}

impl SocketLog {
    pub fn new() -> Result<Self> {
        Ok(Self {
            socket: Socket::new(Domain::IPV4, Type::STREAM, Some(Protocol::TCP))
                .context("无法启动socket")?,
        })
    }

    pub fn SocketInit(&mut self) -> Result<()> {
        self.socket.set_reuse_address(true)?;

        let addr: SocketAddr = "0.0.0.0:25560".parse()?;
        self.socket.bind(&addr.into())?;
        self.socket.listen(128)?;

        Ok(())
    }

    pub fn ReceiveLog(&mut self) -> Result<String> {
        let (client_socket, _) = self.socket.accept()?;
        let mut stream = std::net::TcpStream::from(client_socket);
        let mut buffer = [0u8; 1024];
        let bytes_read = stream.read(&mut buffer)?;
        Ok(String::from_utf8_lossy(&buffer[..bytes_read]).to_string())
    }

    pub fn SendLog(&mut self, content: &str) -> Result<()> {
        let (client_socket, _) = self.socket.accept()?;
        let mut stream = std::net::TcpStream::from(client_socket);
        stream.write_all(content.as_bytes())?;
        Ok(())
    }
}
