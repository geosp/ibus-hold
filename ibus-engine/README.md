# ibus-engine

Write [IBus](https://github.com/ibus/ibus) input method engines in pure Rust.

`ibus-engine` talks to ibus-daemon directly over D-Bus (via
[zbus](https://github.com/dbus2/zbus)), without GObject or any C bindings.

- Implement `IBusEngine` to receive key events and engine callbacks (focus,
  reset, enable/disable, candidate clicks, ...).
- Use the automatically implemented `IBusEngineBackend` to commit text, show
  preedit and auxiliary text, and show a `LookupTable` (candidate popup).
- Implement `IBusFactory` to create engines by name, then call `IBus::new` to
  connect and register.

```toml
[dependencies]
ibus-engine = "0.3"
```

See the [API docs](https://docs.rs/ibus-engine) for a complete minimal engine.
For a real-world example, see
[ibus-hold](https://github.com/geosp/ibus-hold/tree/main/ibus-hold), a
macOS-style press-and-hold accent picker built with this crate.

## Features

- `tokio` (default): use the tokio runtime for D-Bus.
- `async-io`: use the async-io runtime instead
  (`default-features = false, features = ["async-io"]`).

## License

`LGPL-2.1-or-later` or `GPL-3.0-or-later`, at your option.

Based on code from [fm-elpac/librush](https://github.com/fm-elpac/librush) by
secext2022.
