//! `ibus-engine-hold`: macOS-style press-and-hold accent picker for ibus.
//!
//! Hold a letter such as `e` to get a popup with é è ê ë ...; press `1` ~ `9`
//! (or use the arrow keys and Enter) to pick one. A normal tap types the
//! letter as usual.
//!
//! ibus-daemon starts this program (see `fedora/ibus-hold.xml`) with `--ibus`.
use std::error::Error;

use librush::ibus::{IBus, get_ibus_addr};
use log::info;

mod accents;
mod engine;
mod state;

/// D-Bus name, must match `<component><name>` in the component XML.
const BUS_NAME: &str = "io.github.geosp.IBusHold";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    if std::env::args().any(|a| a == "--version") {
        println!("ibus-engine-hold {}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }
    // other arguments (`--ibus` from ibus-daemon) need no handling

    let config = accents::load();
    info!(
        "delay {} ms, {} letters",
        config.delay.as_millis(),
        config.accents.len()
    );

    let addr = get_ibus_addr()?;
    info!("ibus addr: {addr}");
    let _bus = IBus::new(addr, engine::HoldFactory::new(config), BUS_NAME.to_string()).await?;
    info!("ready");

    // keep the process (and the D-Bus connection) alive
    std::future::pending::<()>().await;
    Ok(())
}
