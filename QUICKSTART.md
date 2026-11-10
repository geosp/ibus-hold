# ⚡ Quick Start Guide

Get ibus-hold running in 2 minutes.

## For Fedora Workstation (GNOME)

The easiest installation:

```sh
# 1. Download and install
sudo dnf install https://github.com/geosp/ibus-hold/releases/download/v0.1.3/ibus-hold-0.1.3-1.fc44.$(uname -m).rpm

# 2. Reload IBus
ibus restart

# 3. Add to Input Sources
# Open Settings → Keyboard → Input Sources → Add
# Search for "Press and Hold" and select your language
# (e.g., "English (US, Press and Hold)")

# 4. Switch to it
# Press Super+Space to switch input methods
# (Or click the keyboard indicator in the top bar)
```

**Done!** Try it in any text field: hold `e` for half a second, then press `1-9` to pick an accent.

---

## For Other Linux Distributions

### Debian 12+ / Ubuntu 22.04+

```sh
# Download the .deb from https://github.com/geosp/ibus-hold/releases
sudo apt install ./ibus-hold_*.deb
ibus restart
```

Then follow steps 3-4 above, but use your desktop's input source settings:
- **Ubuntu GNOME:** Settings → Keyboard → Input Sources
- **KDE Plasma:** System Settings → Regional Settings → Input Method
- **XFCE:** Settings → Keyboard → Input Method

### Build from Source

```sh
git clone https://github.com/geosp/ibus-hold.git
cd ibus-hold

cargo build --release -p ibus-hold
sudo install -Dm755 target/release/ibus-engine-hold /usr/libexec/ibus-engine-hold
sudo install -Dm644 ibus-hold/data/ibus-hold.xml /usr/share/ibus/component/ibus-hold.xml
ibus restart
```

Then add the input source in your desktop settings.

---

## How to Use It

### Basic Usage

- **Tap** a letter → types the letter normally
- **Hold** a letter (0.5s) → shows accented variants
  - Press **1-9** to pick
  - Use **arrow keys + Enter** to select
  - Press **Esc** to cancel and keep the base letter
  - Any other key types the base letter and continues

### Example

```
Hold 'e' → shows [ è  é  ê  ë  ē  ė  ę ]
Press '2' → types 'é'
```

### Uppercase

Hold **Shift+letter** to get uppercase accented variants:

```
Hold Shift+E → shows [ È  É  Ê  Ë  Ē  Ė  Ę ]
```

### Default Letters with Accents

These letters have built-in accents: **a, c, e, i, l, n, o, s, u, y, z**

---

## Customize It

Edit `~/.config/ibus-hold/config` to customize:

```text
# Delay before popup (milliseconds, default 500)
delay 400

# Custom variants for a letter (replaces built-in)
e é è ê ë ē ė

# Add a letter without built-in variants
t ţ ť

# Disable popup for a letter (empty list)
y
```

Then run `ibus restart` to apply changes.

---

## Supported Layouts

| Language | Region | Engine Name |
|---|---|---|
| English | US | `hold` |
| English | UK | `hold:gb` |
| German | Germany | `hold:de` |
| German | Switzerland | `hold:ch` |
| French | France | `hold:fr` |
| French | Canada | `hold:ca` |
| Spanish | Spain | `hold:es` |
| Spanish | Latin America | `hold:latam` |
| Portuguese | Portugal | `hold:pt` |
| Portuguese | Brazil | `hold:br` |
| Italian | Italy | `hold:it` |
| Swedish | Sweden | `hold:se` |
| Norwegian | Norway | `hold:no` |
| Danish | Denmark | `hold:dk` |
| Finnish | Finland | `hold:fi` |
| Polish | Poland | `hold:pl` |
| Turkish | Turkey | `hold:tr` |

Missing your layout? [Add it!](CONTRIBUTING.md#-adding-keyboard-layouts)

---

## Troubleshooting

### Entries don't show in Input Sources

```sh
# Kill the settings app (it caches the list)
pkill -f gnome-control-center

# Rebuild the IBus cache
ibus write-cache && ibus restart

# Log out and back in if still missing
```

### Input method doesn't work

```sh
# Check IBus sees ibus-hold
ibus list-engine | grep hold

# View debug logs
RUST_LOG=debug ibus-daemon -rxv
```

### Uninstall

```sh
# Remove input source in your desktop settings first

# Then uninstall:
sudo dnf remove ibus-hold    # Fedora
sudo apt remove ibus-hold    # Debian/Ubuntu

# Or if built from source:
sudo rm /usr/libexec/ibus-engine-hold /usr/share/ibus/component/ibus-hold.xml

ibus restart
```

---

## Want More?

- **[Full Installation Guide](doc/ibus-install.md)** — detailed steps for all systems
- **[Contributing Guide](CONTRIBUTING.md)** — add features or layouts
- **[Library Docs](https://docs.rs/ibus-engine)** — build your own IBus engine in Rust
- **[GitHub Issues](https://github.com/geosp/ibus-hold/issues)** — report bugs or suggest features

---

**Enjoy typing with accents!** 🎉
