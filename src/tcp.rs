use crate::nonblocking_stdin;
use crate::server::Server;

use std::net::SocketAddr;

use anyhow::Result;
use async_trait::async_trait;
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufWriter};
use tokio::net::{TcpListener, TcpStream, ToSocketAddrs};
use tracing::info;

struct Connection {
    addr: SocketAddr,
    stream: BufWriter<TcpStream>,
}

impl Connection {
    async fn handle(&mut self) -> Result<()> {
        let mut stdin = nonblocking_stdin::stdin();
        let mut stdout = tokio::io::stdout();
        loop {
            let Connection { stream, .. } = self;

            let mut buffer = [0u8; 1024];
            tokio::select! {
                Ok(n)  = stream.read(&mut buffer) => {
                    if n == 0 {
                        break;
                    }
                    stdout.write_all(&buffer[..n]).await?;
                    stdout.flush().await?;
                }
                Some(line) = stdin.read_line() => {
                    stream.write_all(line.as_bytes()).await?;
                    stream.write_u8(b'\n').await?;
                    stream.flush().await?;
                }
                else => break,
            }
        }
        info!("connection from {} terminated", self.addr);

        Ok(())
    }
}

pub struct TcpServer {
    listener: TcpListener,
}

impl TcpServer {
    pub async fn try_new(addr: impl ToSocketAddrs) -> Result<Self> {
        Ok(Self {
            listener: TcpListener::bind(addr).await?,
        })
    }
}

#[async_trait]
impl Server for TcpServer {
    async fn run(&mut self) -> Result<()> {
        loop {
            // accept() errors are non recoverable.
            let (stream, addr) = self.listener.accept().await?;
            info!("new connection from: {addr}");

            let mut connection = Connection {
                addr,
                stream: BufWriter::new(stream),
            };

            connection.handle().await?;
        }
    }

}
