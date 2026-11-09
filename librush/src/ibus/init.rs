//! ibus-related initialization
use std::error::Error;

use log::debug;
use zbus::{Connection, connection::Builder, names::WellKnownName};

use super::IBusErr;

pub async fn connect_ibus(addr: String) -> Result<Connection, Box<dyn Error>> {
    let c = Builder::address(addr.as_str())?.build().await?;

    // ibus init: get unique_name
    // Source file: `ibus/src/ibusbus.c`
    // Function: `ibus_bus_new()` -> `ibus_bus_connect()` -> `ibus_bus_hello()`
    // -> `g_dbus_connection_get_unique_name()`
    let n = c
        .unique_name()
        .ok_or(IBusErr::new("can not get dbus unique_name".to_string()))?;
    debug!("unique_name: {}", n);

    Ok(c)
}

/// ibus init: request name
///
/// Source file: `ibus/src/ibusbus.c`
/// Function: `ibus_bus_request_name()`
pub async fn request_name(c: &Connection, name: String) -> Result<(), Box<dyn Error>> {
    let n = WellKnownName::try_from(name)?;

    c.request_name(n).await?;
    Ok(())
}
