# Contributing to ibus-hold

Thank you for your interest in contributing! This guide will help you get started.

## Getting Started

### Prerequisites

- **Rust toolchain:** <https://rustup.rs>
- **Linux system** with IBus (preinstalled on GNOME, KDE, XFCE)
- **Git**

### Setting Up Your Development Environment

```sh
# Clone the repository
git clone https://github.com/geosp/ibus-hold.git
cd ibus-hold

# Build the workspace
cargo build --workspace --all-targets

# Run tests
cargo test --workspace

# Check formatting and lints
cargo fmt --all --check
cargo clippy --workspace --all-targets
```

## Types of Contributions

### 🐛 Reporting Bugs

If you find a bug, please [open an issue](https://github.com/geosp/ibus-hold/issues) with:

- **Title:** Clear, concise description
- **Environment:** Linux distribution, IBus version, rustc version
- **Reproduction steps:** Exact steps to reproduce the issue
- **Expected behavior:** What should happen
- **Actual behavior:** What actually happens
- **Logs:** Run `RUST_LOG=debug ibus-daemon -rxv` and attach relevant logs

### ✨ Adding Features

Before starting a major feature:

1. **Check existing issues** to avoid duplicate work
2. **Open an issue** with `[Feature Request]` in the title describing your idea
3. **Wait for feedback** from maintainers
4. **Implement** once approved

### 🌍 Adding Keyboard Layouts

Adding support for a new keyboard layout is straightforward:

1. **Edit** `ibus-hold/src/layouts.rs`
2. **Add your layout** to the `LAYOUTS` constant following the existing pattern:
   ```rust
   ("your_code", "Your Language (Press and Hold)", "your_layout"),
   ```
3. **Regenerate** the component XML:
   ```sh
   cargo run -p ibus-hold -- --print-component-xml > ibus-hold/data/ibus-hold.xml
   ```
4. **Test** on your system:
   ```sh
   cargo build --release -p ibus-hold
   sudo install -Dm755 target/release/ibus-engine-hold /usr/libexec/ibus-engine-hold
   sudo install -Dm644 ibus-hold/data/ibus-hold.xml /usr/share/ibus/component/ibus-hold.xml
   ibus restart
   ```
5. **Open a PR** with your changes

**Example:** To add Esperanto layout:
```rust
("epo", "Esperanto (Press and Hold)", "epo"),
```

### 📖 Documentation Improvements

- **Installation guide:** [doc/ibus-install.md](doc/ibus-install.md)
- **README:** [README.md](README.md)
- **API docs:** Rust doc comments in source files

Documentation PRs are always welcome, especially:
- Clarifying confusing sections
- Adding troubleshooting steps
- Translating to other languages
- Adding examples

### 🏗️ Library Development (ibus-engine)

The `ibus-engine` crate is a pure-Rust IBus library. Contributions include:

- **Bug fixes** to D-Bus communication
- **Performance improvements**
- **API enhancements** for engine developers
- **Tests** for edge cases

See [ibus-engine/](ibus-engine/README.md) for details on the library's architecture.

## Development Workflow

### 1. Create a Feature Branch

```sh
git checkout -b feature/your-feature-name
# or for bug fixes:
git checkout -b fix/bug-description
```

### 2. Make Your Changes

- **Keep commits atomic** — one logical change per commit
- **Write clear commit messages** — describe *why*, not just *what*
- **Test as you go** — run tests frequently

### 3. Run Quality Checks

Before pushing, ensure everything passes:

```sh
# Format code
cargo fmt --all

# Lint and check
cargo clippy --workspace --all-targets

# Run tests
cargo test --workspace

# Build release version
cargo build --release --workspace

# Check documentation
cargo doc --no-deps
```

### 4. Push and Create a Pull Request

```sh
git push origin feature/your-feature-name
```

Then open a PR on GitHub with:

- **Clear title** — what does this PR do?
- **Description** — why is this change needed? (reference issues if applicable)
- **Testing notes** — how to verify the change works
- **Screenshots/GIFs** — for UI changes

## Code Style

### Rust Style

We follow standard Rust conventions:

- **Formatting:** Enforced by `cargo fmt`
- **Linting:** Checked by `cargo clippy`
- **Comments:** Only where clarification is truly needed; prefer self-documenting code

### Commit Message Style

```
Brief description (50 chars max)

Optional longer explanation. Wrap at 72 characters.
- Use bullet points for lists
- Explain *why*, not just *what*

Closes #issue-number (if applicable)
```

Example:
```
Add French (Canada) keyboard layout

Add support for Canadian French (ca_FR) layout, including
variants for accented characters. Tested on Fedora 44 with
GNOME 45.

Closes #23
```

## Testing

### Running Tests

```sh
# All tests
cargo test --workspace

# Specific package
cargo test -p ibus-hold

# Specific test
cargo test --lib ibus_hold::layouts

# With output
cargo test -- --nocapture
```

### Manual Testing

For UI changes, install locally and test:

```sh
cargo build --release -p ibus-hold
sudo install -Dm755 target/release/ibus-engine-hold /usr/libexec/ibus-engine-hold
sudo install -Dm644 ibus-hold/data/ibus-hold.xml /usr/share/ibus/component/ibus-hold.xml
ibus restart
```

Then:
1. Open **Settings → Keyboard → Input Sources**
2. Add the new layout
3. Test in your favorite text editor
4. Report any issues in the PR

## Release Process

*(For maintainers)*

To publish a release:

```sh
# 1. Update version in Cargo.toml files
#    ibus-hold/Cargo.toml
#    ibus-hold/fedora/ibus-hold.spec

# 2. Regenerate component XML
cargo run -p ibus-hold -- --print-component-xml > ibus-hold/data/ibus-hold.xml

# 3. Commit changes
git add -A
git commit -m "Release v0.1.3"

# 4. Tag and push
git tag v0.1.3
git push origin main v0.1.3

# 5. GitHub Actions builds .rpm and .deb packages automatically

# 6. Publish to crates.io (after cargo login)
cargo publish -p ibus-engine
cargo publish -p ibus-hold
```

## Getting Help

- **Questions?** Open an issue with `[Question]` label
- **Need clarification?** Comment on the issue or PR
- **Want to discuss ideas?** Start a discussion on GitHub
- **Found a security issue?** Please email privately to the maintainers

## Code of Conduct

This project is committed to providing a welcoming, inclusive environment. All contributors are expected to:

- Be respectful and professional
- Accept constructive criticism gracefully
- Focus on collaboration and learning
- Report harmful behavior to maintainers

## License

By contributing, you agree that your contributions will be licensed under the same terms as the project:
- **LGPL-2.1-or-later** or **GPL-3.0-or-later**, at your option

---

**Thank you for contributing!** Your effort helps make ibus-hold better for the Linux community. 🎉
