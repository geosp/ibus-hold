# ibus-hold

<https://github.com/geosp/ibus-hold>

![CI](https://github.com/geosp/ibus-hold/actions/workflows/ci.yml/badge.svg)

Write IBus input method engines in Rust, and a macOS-style press-and-hold
accent picker built with it, for Linux desktops that use IBus (such as GNOME).

This repository is a Cargo workspace with two crates:

- [`librush/`](librush/): a library for writing
  [IBus](https://github.com/ibus/ibus) input method engines in pure rust. It
  talks to ibus-daemon directly over D-Bus (via
  [zbus](https://github.com/dbus2/zbus/)), without GObject or C bindings.
  See the crate docs (`cargo doc -p librush --open`) for an example.
- [`ibus-hold/`](ibus-hold/): a macOS-style **press-and-hold** accent picker
  built on librush. Hold `e` to get `è é ê ë ē ė ę`, then press a number to
  pick one. A normal tap types the letter as usual.

## Quick start

```sh
cargo build --release -p ibus-hold
sudo install -Dm755 target/release/ibus-engine-hold /usr/libexec/ibus-engine-hold
sudo install -Dm644 ibus-hold/data/ibus-hold.xml /usr/share/ibus/component/ibus-hold.xml
ibus restart
```

On GNOME, then go to **Settings → Keyboard → Input Sources → Add → English →
English (Press and Hold)**.

Requirements: IBus (preinstalled on GNOME desktops) and a Rust toolchain
(<https://rustup.rs>).

See [doc/ibus-install.md](doc/ibus-install.md) for other desktops, a Fedora
RPM package, configuration, and troubleshooting.

## Using the library in another project

```toml
[dependencies]
librush = { git = "https://github.com/geosp/ibus-hold" }
```

Features:

- `tokio` (default): use the tokio runtime for D-Bus.
- `async-io`: use the async-io runtime instead
  (`default-features = false, features = ["async-io"]`).

Implement `IBusEngine` (key events, focus, reset, ...) and `IBusFactory`,
then call `IBus::new`. `IBusEngineBackend` lets your engine commit text, set
preedit/auxiliary text, and show a `LookupTable` (candidate popup).
`ibus-hold/src/engine.rs` is a complete real-world example.

Note: this library is used via git (or a path dependency); it is not published
on crates.io.

## Development

```sh
cargo fmt --all --check
cargo test --workspace
cargo clippy --workspace --all-targets
```

Tested with rustc 1.99.1 and IBus 1.5.34 on Fedora 44 Workstation (GNOME).
Other distributions and desktops with IBus should work but are untested.

Keep dependencies current: run `cargo update` after new rustc releases or
important security updates, then rebuild and retest.

## Links

- IBus: Intelligent Input Bus for Linux/Unix

  <https://github.com/ibus/ibus>

- D-Bus is a message bus system, a simple way for applications to talk to one
  another

  <https://www.freedesktop.org/wiki/Software/dbus/>

- zbus: A Rust API for D-Bus communication

  <https://github.com/dbus2/zbus/>

- Rust Programming Language

  <https://www.rust-lang.org/>

- GObject: The GLib Object System

  <https://docs.gtk.org/gobject/>

- GNOME Shell: next generation desktop shell

  <https://wiki.gnome.org/Projects/GnomeShell>
  <https://gitlab.gnome.org/GNOME/gnome-shell/>

- GNOME 3.6 integrated input method

  <https://help.gnome.org/misc/release-notes/3.6/i18n-ibus.html>

## LICENSE

This repository is released under: `LGPL-2.1-or-later` or `GPL-3.0-or-later`, at
your option.

The `librush` library is based on code from
[fm-elpac/librush](https://github.com/fm-elpac/librush) by secext2022.
