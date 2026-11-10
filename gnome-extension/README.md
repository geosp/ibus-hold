# Press and Hold Accents: GNOME Shell extension

An optional companion to [ibus-hold](../README.md) for GNOME Shell 50. The
accent picker itself is the `ibus-hold` IBus engine; this extension makes it
easier to use from GNOME:

- **Quick Settings toggle**: turns Press and Hold on and off. Off switches to
  the plain keyboard layout it is based on (for example "English (US)").
- **Adds the input source for you** when Press and Hold is missing from your
  input sources, and switches to it. You can turn this off in the preferences.
- **Preferences**:
  - Keyboard layout: Automatic (matches your first keyboard layout) or any of
    the 17 layouts.
  - Hold delay: saved to `~/.config/ibus-hold/config`. Use **Restart IBus** to
    apply it.
- **Detects a missing engine**: shows a notification that links to the install
  guide.

The extension can't replace the package: extensions.gnome.org doesn't allow
bundled binaries. Install the engine first:
[doc/ibus-install.md](../doc/ibus-install.md).

## Install from source

```sh
cd gnome-extension
make install
```

Log out and back in (on Wayland GNOME Shell only picks up new extensions at
login), then:

```sh
gnome-extensions enable ibus-hold@geosp.github.io
gnome-extensions prefs ibus-hold@geosp.github.io   # optional
```

## Development

- `make test` runs the tests for the shared logic in `engines.js` (requires `gjs`).
- `make pack` builds `ibus-hold@geosp.github.io.shell-extension.zip`, the file
  you upload to extensions.gnome.org.
- Logs: `journalctl -f -o cat /usr/bin/gnome-shell`.

To test without logging out, use a nested shell (GNOME 49+ needs the
`mutter-devkit` package):

```sh
dbus-run-session gnome-shell --devkit --wayland
```
