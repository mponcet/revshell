use std::net::{IpAddr, SocketAddr};
use std::sync::Mutex;

use anyhow::{Result, anyhow};
pub use igd_next::PortMappingProtocol;
pub use igd_next::aio::Gateway;
use igd_next::aio::tokio::Tokio;
pub use igd_next::aio::tokio::search_gateway;
use tracing::debug;

struct PortMapping {
    protocol: PortMappingProtocol,
    external_port: u16,
}

pub struct GatewayExt {
    gw: Gateway<Tokio>,
    mapping: Mutex<Option<PortMapping>>,
}

impl GatewayExt {
    pub async fn search() -> Result<Self> {
        Ok(Self {
            gw: search_gateway(Default::default()).await?,
            mapping: Mutex::new(None),
        })
    }

    pub async fn add_port(
        &self,
        protocol: PortMappingProtocol,
        external_port: u16,
        local_addr: SocketAddr,
    ) -> anyhow::Result<()> {
        self.gw
            .add_port(protocol, external_port, local_addr, 3600, "revshell")
            .await?;

        *self.mapping.lock().unwrap() = Some(PortMapping {
            protocol,
            external_port,
        });

        Ok(())
    }

    pub async fn get_external_ip(&self) -> Result<IpAddr> {
        self.gw
            .get_external_ip()
            .await
            .map_err(|_| anyhow!("failed to get external ip"))
    }

    pub async fn cleanup(self) {
        let mapping = { self.mapping.lock().unwrap() }.take();
        if let Some(PortMapping {
            protocol,
            external_port,
        }) = mapping
        {
            debug!("removing port mapping: external_port={external_port}");
            let res = self.gw.remove_port(protocol, external_port).await;
            if res.is_err() {
                debug!("failed to remove port mapping: external_port={external_port}");
            }
        }
    }
}
