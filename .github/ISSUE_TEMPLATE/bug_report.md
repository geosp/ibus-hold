---
name: "🐛 Bug Report"
about: "Report a bug or issue with ibus-hold"
title: "[Bug] "
labels: ["bug"]
---

## Description
A clear, concise description of the bug.

## Environment
- **Linux Distribution:** (e.g., Fedora 44, Ubuntu 22.04, Arch Linux)
- **Desktop Environment:** (e.g., GNOME 45, KDE Plasma 6, XFCE 4.18)
- **ibus-hold version:** (e.g., 0.1.2; run `ibus list-engine | grep hold`)
- **IBus version:** (run `ibus version`)
- **Rustc version:** (run `rustc --version`, if built from source)

## Steps to Reproduce
1. ...
2. ...
3. ...

## Expected Behavior
Describe what should happen.

## Actual Behavior
Describe what actually happens instead.

## Debug Logs
If relevant, attach logs from:
```sh
RUST_LOG=debug ibus-daemon -rxv
```
(Run this command, reproduce the bug, then copy the output)

## Screenshots / Additional Context
Any GIFs, screenshots, or other information that would help diagnose the issue.
