# Installing and using ibus-hold

**ibus-hold** brings macOS-style press-and-hold accent selection to Linux desktops using the IBus input method framework. It has been tested on Fedora 44 Workstation with GNOME; other distributions and desktops may work but are untested.

- 🌍 **17 keyboard layouts** supported (US, UK, German, French, Spanish, Portuguese, Italian, Scandinavian, Polish, Turkish, and more)
- ⚡ **Lightweight** — Rust implementation without GObject or C bindings; IBus is required
- 🎯 **Familiar UI** — if you've used macOS, you already know how to use it
- ⚙️ **Customizable** — configure delay, variants, and more in `~/.config/ibus-hold/config`
- 🔧 **Easy to install** — `.rpm` and `.deb` packages for x86_64 and ARM64

> **GNOME users on Fedora?** Jump to [Fedora Workstation: step by step](#fedora-workstation-step-by-step) — you'll be set up in 2 minutes.

## Fedora Workstation: step by step

These steps take about two minutes. You need Fedora Workstation (GNOME), which
already includes IBus.

**1. Install the package.** `dnf` can download it straight from the release;
`$(uname -m)` picks x86_64 or aarch64 for your machine:

```sh
sudo dnf install https://github.com/geosp/ibus-hold/releases/download/v0.1.3/ibus-hold-0.1.3-1.fc44.$(uname -m).rpm
```

(For a newer release, change both `0.1.3`s to its version, as listed on the
[Releases page](https://github.com/geosp/ibus-hold/releases).)

**2. Restart IBus** so it loads the new engine:

```sh
ibus restart
```

Check that it worked: `ibus list-engine | grep -c Hold` should print `17`
(one entry per keyboard layout).

**3. Add the input source.**

1. Stop any running copy of Settings, because it keeps running in the
   background after its window closes and won't see the new engine:

   ```sh
   pkill -f gnome-control-center
   ```

   Then open **Settings → Keyboard**.
2. Under **Input Sources**, click **+ Add Input Source**.
3. Click **⋮** at the bottom of the language list, type `press` in the search
   box, and choose the entry for your keyboard, e.g.
   **English (US, Press and Hold)**. Click **Add**.
4. Optional: remove your old plain layout (e.g. "English (US)") with its **⋮ →
   Remove** menu, so Press and Hold is always active.

**4. Switch to it** with **Super+Space** (or click the keyboard indicator in the
top bar and pick it). The indicator shows **Hå** when it is active.

**5. Try it.** In any text field, hold `e` for half a second, then press `2` to
type `é`. A normal tap still types `e`. See [Use it](#use-it) for all keys.

**To uninstall:** remove the input source in Settings, then run
`sudo dnf remove ibus-hold && ibus restart`.

If something doesn't work, see [Troubleshooting](#troubleshooting).

## Other distributions

### Requirements

- **IBus.** Preinstalled on GNOME desktops (Fedora Workstation, Ubuntu, ...).
  Check with `ibus version`. If it is missing:
  - Fedora: `sudo dnf install ibus`
  - Debian / Ubuntu: `sudo apt install ibus`
  - Arch: `sudo pacman -S ibus`
- **A Rust toolchain**, only if you build from source: <https://rustup.rs>

### Install a release package

Download the package for your system from the
[Releases page](https://github.com/geosp/ibus-hold/releases) (x86_64/amd64 or
aarch64/arm64), then:

- Fedora: `sudo dnf install ./ibus-hold-*.rpm`
- Debian 12+ / Ubuntu 22.04+: `sudo apt install ./ibus-hold_*.deb`

Then run `ibus restart` and [enable it](#enable-it).

### Build from source

```sh
cargo build --release -p ibus-hold
sudo install -Dm755 target/release/ibus-engine-hold /usr/libexec/ibus-engine-hold
sudo install -Dm644 ibus-hold/data/ibus-hold.xml /usr/share/ibus/component/ibus-hold.xml
ibus restart
```

Some distributions keep IBus engines in `/usr/lib/ibus/` instead of
`/usr/libexec/`. Either location works, as long as the `<exec>` path in
`ibus-hold.xml` matches where you installed the program.

### Build your own Fedora RPM

```sh
sudo dnf install rpm-build rpmdevtools cargo rust
rpmdev-setuptree
git archive --prefix=ibus-hold-0.1.3/ -o ~/rpmbuild/SOURCES/ibus-hold-0.1.3.tar.gz HEAD
rpmbuild -bb ibus-hold/fedora/ibus-hold.spec
sudo dnf install ~/rpmbuild/RPMS/x86_64/ibus-hold-0.1.3-1.fc*.x86_64.rpm
ibus restart
```

## Enable it

### GNOME

1. Open **Settings → Keyboard → Input Sources → Add Input Source**.
2. Click **⋮** at the bottom of the list, search for `press`, and choose the
   entry for your keyboard layout, e.g. **English (US, Press and Hold)** or
   **German (Press and Hold)**. (If the entries are missing, run
   `pkill -f gnome-control-center` and open Settings again: it keeps running
   in the background with the old list.)
3. Switch to it with Super+Space or the keyboard menu in the top bar. Remove
   the plain "English (US)" source if you only want this one.

Or from a terminal (this replaces your current input sources; use the engine
name for your layout from the table below):

```sh
gsettings set org.gnome.desktop.input-sources sources "[('ibus', 'hold')]"
```

### Other desktops (untested)

1. Make IBus the active input method.
   - KDE Plasma on Wayland: **System Settings → Keyboard → Virtual Keyboard →
     IBus Wayland**.
   - Other desktops / X11: start `ibus-daemon -drx` at login and set
     `GTK_IM_MODULE=ibus`, `QT_IM_MODULE=ibus`, `XMODIFIERS=@im=ibus` in your
     environment.
2. Run `ibus-setup`, open the **Input Method** tab, and add the
   **(Press and Hold)** entry for your layout.

## Keyboard layouts

Each entry loads its own keyboard layout, so pick the one matching your
keyboard. The popup works the same in all of them.

| Entry | Engine name | Layout |
|---|---|---|
| English (US, Press and Hold) | `hold` | `us` |
| English (UK, Press and Hold) | `hold:gb` | `gb` |
| German (Press and Hold) | `hold:de` | `de` |
| German (Switzerland, Press and Hold) | `hold:ch` | `ch` |
| French (Press and Hold) | `hold:fr` | `fr` |
| French (Canada, Press and Hold) | `hold:ca` | `ca` |
| Spanish (Press and Hold) | `hold:es` | `es` |
| Spanish (Latin American, Press and Hold) | `hold:latam` | `latam` |
| Portuguese (Press and Hold) | `hold:pt` | `pt` |
| Portuguese (Brazil, Press and Hold) | `hold:br` | `br` |
| Italian (Press and Hold) | `hold:it` | `it` |
| Swedish (Press and Hold) | `hold:se` | `se` |
| Norwegian (Press and Hold) | `hold:no` | `no` |
| Danish (Press and Hold) | `hold:dk` | `dk` |
| Finnish (Press and Hold) | `hold:fi` | `fi` |
| Polish (Press and Hold) | `hold:pl` | `pl` |
| Turkish (Press and Hold) | `hold:tr` | `tr` |

Missing your layout? Add a line to `LAYOUTS` in `ibus-hold/src/layouts.rs`,
regenerate the component file, and open a pull request:

```sh
cargo run -p ibus-hold -- --print-component-xml > ibus-hold/data/ibus-hold.xml
```

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

(or `sudo dnf remove ibus-hold` / `sudo apt remove ibus-hold` if you
installed a package), then remove the
input source in your desktop settings.

## Troubleshooting

Run IBus in the foreground with debug logs:

```sh
RUST_LOG=debug ibus-daemon -rxv
```

If the "(Press and Hold)" entries do not show up, check that IBus sees it with
`ibus list-engine | grep hold`. If it is listed, run `pkill -f gnome-control-center`
and open Settings again; if not, run `ibus write-cache && ibus restart`, then log out and in again.
