//! Write [IBus](https://github.com/ibus/ibus) input method engines in pure
//! rust, talking to ibus-daemon directly over D-Bus (no GObject, no C).
//!
//! The main pieces live in [`ibus`]:
//!
//! - [`ibus::IBusEngine`]: implement this to receive key events and other
//!   engine callbacks (focus, reset, enable, candidate clicks, ...).
//! - [`ibus::IBusEngineBackend`]: automatically implemented for every engine;
//!   use it to commit text, show preedit/auxiliary text, and show a
//!   [`ibus::LookupTable`] (candidate popup).
//! - [`ibus::IBusFactory`]: creates engine instances by name.
//! - [`ibus::IBus`]: connects to ibus-daemon and registers your factory.
//!
//! # Example
//!
//! An engine that captures nothing (every key passes through to the app):
//!
//! ```no_run
//! use ibus_engine::ibus::{IBus, IBusEngine, IBusFactory, get_ibus_addr};
//!
//! #[derive(Debug, Clone)]
//! struct MyEngine;
//!
//! // all methods have default implementations
//! impl IBusEngine for MyEngine {}
//!
//! #[derive(Debug, Clone)]
//! struct MyFactory;
//!
//! impl IBusFactory<MyEngine> for MyFactory {
//!     fn create_engine(&mut self, name: String) -> Result<MyEngine, String> {
//!         if name == "my-engine" {
//!             Ok(MyEngine)
//!         } else {
//!             Err(format!("unknown engine: {name}"))
//!         }
//!     }
//! }
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let addr = get_ibus_addr()?;
//!     let _bus = IBus::new(addr, MyFactory, "org.example.MyEngine".to_string()).await?;
//!     // keep the process (and the D-Bus connection) alive
//!     std::future::pending::<()>().await;
//!     Ok(())
//! }
//! ```
//!
//! The engine must also be declared in an ibus component XML file in
//! `/usr/share/ibus/component/`; see the `ibus-hold` crate in this repository
//! for a complete, installable example.
#![deny(unsafe_code)]

pub mod ibus;
