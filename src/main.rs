mod igd;
mod nonblocking_stdin;
mod server;

use crate::igd::GatewayExt;

use igd::PortMappingProtocol;
use tracing::Level;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    let args = std::env::args().collect::<Vec<_>>();
    let host = args.get(1).expect("missing host");
    let port = args
        .get(2)
        .expect("missing port")
        .parse::<u16>()
        .expect("bad port number");

    let gw = GatewayExt::search().await?;
    gw.add_port(
        PortMappingProtocol::TCP,
        port,
        format!("{host}:{port}").parse().unwrap(),
    )
    .await?;

    let external_ip = gw.get_external_ip().await?;
    println!("Your external ip: {external_ip}");

    let _ = server::run().await;

    gw.cleanup().await;

    Ok(())
}
