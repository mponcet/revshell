use crate::server::Server;

use anyhow::Result;
use async_trait::async_trait;
use tokio::net::ToSocketAddrs;

pub struct UdpServer;

impl UdpServer {
    pub async fn try_new(addr: impl ToSocketAddrs) -> Result<Self> {
        todo!()
    }
}

#[async_trait]
impl Server for UdpServer {
    async fn run(&mut self) -> Result<()> {
        todo!()
    }

    async fn shutdown(self: Box<Self>) {
        todo!()
    }
}
