use crate::tcp::TcpServer;
use crate::udp::UdpServer;

use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use anyhow::Result;
use async_trait::async_trait;
use tracing::{debug, error};

#[derive(Clone, Copy, Debug)]
pub enum Protocol {
    Udp,
    Tcp,
}

#[async_trait]
pub trait Server {
    async fn run(&mut self) -> Result<()>;
}

pub async fn run(bind_port: u16, protocol: Protocol) -> Result<()> {
    let bind_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), bind_port);
    let mut server = match protocol {
        Protocol::Udp => Box::new(UdpServer::try_new(bind_addr).await?) as Box<dyn Server>,
        Protocol::Tcp => Box::new(TcpServer::try_new(bind_addr).await?) as Box<dyn Server>,
    };

    tokio::select! {
        res = server.run() => {
            if let Err(err) = res {
                error!("main server loop failed: {err}");
            }
        }
        _ = tokio::signal::ctrl_c() => {
            debug!("shutting down");
        }
    }

    Ok(())
}
