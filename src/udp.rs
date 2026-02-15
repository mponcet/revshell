use crate::nonblocking_stdin;
use crate::server::Server;

use std::net::SocketAddr;

use anyhow::Result;
use async_trait::async_trait;
use tokio::io::AsyncWriteExt;
use tokio::net::{ToSocketAddrs, UdpSocket};
use tracing::info;

pub struct UdpServer {
    socket: UdpSocket,
    client_addr: Option<SocketAddr>,
}

impl UdpServer {
    pub async fn try_new(addr: impl ToSocketAddrs) -> Result<Self> {
        let socket = UdpSocket::bind(addr).await?;
        Ok(Self {
            socket,
            client_addr: None,
        })
    }
}

#[async_trait]
impl Server for UdpServer {
    async fn run(&mut self) -> Result<()> {
        let mut stdin = nonblocking_stdin::stdin();
        let mut stdout = tokio::io::stdout();
        let mut buffer = [0u8; 1024];

        loop {
            tokio::select! {
                Ok((len, addr)) = self.socket.recv_from(&mut buffer) => {
                    if len == 0 {
                        break;
                    }

                    if self.client_addr.is_none() {
                        self.client_addr = Some(addr);
                        info!("new connection from: {}", addr);
                    }

                    stdout.write_all(&buffer[..len]).await?;
                    stdout.flush().await?;
                }
                Some(line) = stdin.read_line() => {
                    if let Some(addr) = self.client_addr {
                        self.socket.send_to(line.as_bytes(), addr).await?;
                        self.socket.send_to(b"\n", addr).await?;
                    }
                }
                else => break,
            }
        }

        if let Some(addr) = self.client_addr {
            info!("connection from {} terminated", addr);
        }

        Ok(())
    }

}
