mod igd;
mod nonblocking_stdin;
mod server;

use std::net::{IpAddr, SocketAddr};

use crate::igd::GatewayExt;

use anyhow::{Result, anyhow, bail};
use clap::Parser;
use igd::PortMappingProtocol;
use tracing::{Level, info};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short = 'H', long, value_parser = parse_host)]
    host: IpAddr,

    #[arg(short = 'p', long, value_parser = parse_port_mapping, required = true)]
    port: (u16, u16),
}

fn parse_host(host: &str) -> Result<IpAddr> {
    host.parse().map_err(|_| anyhow!("failed to parse host"))
}

fn parse_port_mapping(s: &str) -> Result<(u16, u16)> {
    let Some((external, internal)) = s.split_once(':') else {
        bail!("expected format external_port:internal_port");
    };

    let external = external.parse::<u16>()?;
    let internal = internal.parse::<u16>()?;
    Ok((external, internal))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let args = Args::parse();

    let gw = GatewayExt::search().await?;
    let (external_port, internal_port) = args.port;
    let local_addr = SocketAddr::new(args.host, internal_port);
    gw.add_port(PortMappingProtocol::TCP, external_port, local_addr)
        .await?;

    let external_ip = gw.get_external_ip().await?;
    info!("Your external ip: {external_ip}");

    let _ = server::run(internal_port).await;

    gw.cleanup().await;

    Ok(())
}
