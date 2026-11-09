# Installing and using ibus-hold

ibus-hold works on any Linux desktop that uses the IBus input method framework.
GNOME has IBus built in, so it is the easiest setup. Tested on Fedora 44
Workstation (GNOME); other distributions and desktops should work but are
untested.

## Requirements

- **IBus.** Preinstalled on GNOME desktops (Fedora Workstation, Ubuntu, ...).
  Check with `ibus version`. If it is missing:
  - Fedora: `sudo dnf install ibus`
  - Debian / Ubuntu: `sudo apt install ibus`
  - Arch: `sudo pacman -S ibus`
- **A Rust toolchain** to build it: <https://rustup.rs>

## Install

```sh
cargo build --release -p ibus-hold
sudo install -Dm755 target/release/ibus-engine-hold /usr/libexec/ibus-engine-hold
sudo install -Dm644 ibus-hold/data/ibus-hold.xml /usr/share/ibus/component/ibus-hold.xml
ibus restart
```

Some distributions keep IBus engines in `/usr/lib/ibus/` instead of
`/usr/libexec/`. Either location works, as long as the `<exec>` path in
`ibus-hold.xml` matches where you installed the program.

### Fedora: build an RPM instead

```sh
sudo dnf install rpm-build rpmdevtools cargo rust
rpmdev-setuptree
git archive --prefix=ibus-hold-0.1.0/ -o ~/rpmbuild/SOURCES/ibus-hold-0.1.0.tar.gz HEAD
rpmbuild -bb ibus-hold/fedora/ibus-hold.spec
sudo dnf install ~/rpmbuild/RPMS/x86_64/ibus-hold-0.1.0-1.fc*.x86_64.rpm
ibus restart
```

## Enable it

### GNOME

1. Open **Settings → Keyboard → Input Sources → Add Input Source**.
2. Click **⋮** at the bottom of the list, search for `press`, and choose
   **English (Press and Hold)**. (If Settings was already open before
   `ibus restart`, close and reopen it first.)
3. Switch to it with Super+Space or the keyboard menu in the top bar. Remove
   the plain "English (US)" source if you only want this one.

Or from a terminal (this replaces your current input sources):

```sh
gsettings set org.gnome.desktop.input-sources sources "[('xkb', 'us'), ('ibus', 'hold')]"
```

### Other desktops (untested)

1. Make IBus the active input method.
   - KDE Plasma on Wayland: **System Settings → Keyboard → Virtual Keyboard →
     IBus Wayland**.
   - Other desktops / X11: start `ibus-daemon -drx` at login and set
     `GTK_IM_MODULE=ibus`, `QT_IM_MODULE=ibus`, `XMODIFIERS=@im=ibus` in your
     environment.
2. Run `ibus-setup`, open the **Input Method** tab, and add
   **English → English (Press and Hold)**.

The engine uses the `us` keyboard layout. For another layout, change
`<layout>` in `ibus-hold.xml`, reinstall it, and run `ibus restart`.

## Use it

- Tap a letter: it is typed as usual.
- Hold `e` (about half a second): a popup shows `è é ê ë ē ė ę`.
  - `1` ~ `9` picks a variant.
  - Arrow keys + Enter pick the highlighted one; a click works too.
  - Esc keeps the plain letter, Backspace drops it.
  - Any other key types the plain letter and continues normally.
- Hold Shift too (`E`) for uppercase variants.

Letters with variants by default: a c e i l n o s u y z.

## Configure

Optional file `~/.config/ibus-hold/config`:

```text
# delay before the popup opens, in milliseconds (default 500)
delay 400
# a letter followed by its variants; replaces the built-in list
e é è ê ë
# add a letter that has no built-in variants
t ţ ť
# an empty list disables the popup for that letter
y
```

Uppercase variants are derived automatically. Run `ibus restart` after
changing the file.

## Uninstall

```sh
sudo rm /usr/libexec/ibus-engine-hold /usr/share/ibus/component/ibus-hold.xml
ibus restart
```

(or `sudo dnf remove ibus-hold` if you installed the RPM), then remove the
input source in your desktop settings.

## Troubleshooting

Run IBus in the foreground with debug logs:

```sh
RUST_LOG=debug ibus-daemon -rxv
```

If "English (Press and Hold)" does not show up, check that IBus sees it with
`ibus list-engine | grep hold`. If it is listed, reopen your settings app; if
not, run `ibus write-cache && ibus restart`, then log out and in again.
