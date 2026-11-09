# Installing and using ibus-hold on Fedora

Fedora Workstation (GNOME) ships with ibus already installed and running, so
nothing extra is needed besides this engine. To check:

```sh
ibus version        # e.g. IBus 1.5.34
```

If it is missing (e.g. on another desktop): `sudo dnf install ibus`.

## Install

### Option A: build and install by hand

```sh
cargo build --release -p ibus-hold
sudo install -Dm755 target/release/ibus-engine-hold /usr/libexec/ibus-engine-hold
sudo install -Dm644 ibus-hold/fedora/ibus-hold.xml /usr/share/ibus/component/ibus-hold.xml
ibus restart
```

### Option B: build an RPM

```sh
sudo dnf install rpm-build rpmdevtools cargo rust
rpmdev-setuptree
git archive --prefix=ibus-hold-0.1.0/ -o ~/rpmbuild/SOURCES/ibus-hold-0.1.0.tar.gz HEAD
rpmbuild -bb ibus-hold/fedora/ibus-hold.spec
sudo dnf install ~/rpmbuild/RPMS/x86_64/ibus-hold-0.1.0-1.fc*.x86_64.rpm
ibus restart
```

## Enable it

1. Open **Settings → Keyboard → Input Sources → Add Input Source**.
2. Choose **English**, then **English (Press and Hold)**.
3. Select it in the top bar (or with Super+Space), and remove the plain
   "English (US)" source if you only want this one.

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

## Troubleshooting

Run ibus in the foreground with debug logs:

```sh
RUST_LOG=debug ibus-daemon -rxv
```

If "English (Press and Hold)" does not show up in Settings, refresh the
component cache with `ibus write-cache && ibus restart`, then log out and in
again.
