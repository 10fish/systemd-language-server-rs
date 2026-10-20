# Systemd Language Server

[English🇺🇸](README.md) | [简体中文🇨🇳](README_CN.md)

[![Crates.io](https://img.shields.io/crates/v/systemd-language-server.svg)](https://crates.io/crates/systemd-language-server)
[![Build Status](https://github.com/10fish/systemd-language-server-rs/workflows/CI/badge.svg)](https://github.com/10fish/systemd-language-server-rs/actions)
[![codecov](https://codecov.io/gh/10fish/systemd-language-server-rs/branch/main/graph/badge.svg)](https://codecov.io/gh/10fish/systemd-language-server-rs)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Downloads](https://img.shields.io/crates/d/systemd-language-server.svg)](https://crates.io/crates/systemd-language-server)
[![Rust Version](https://img.shields.io/badge/rust-1.85%2B-blue.svg)](https://www.rust-lang.org/)
[![dependency status](https://deps.rs/repo/github/10fish/systemd-language-server-rs/status.svg)](https://deps.rs/repo/github/10fish/systemd-language-server-rs)

A pre-alpha Rust language server for systemd unit files.

## Features

- Full-document synchronization with version tracking
- Syntax and section diagnostics with accurate UTF-16 ranges
- Section, directive, and selected value completions
- Hover documentation for common sections and directives
- Recognition of common unit types and `.d/*.conf` drop-ins

Go-to-definition, runtime configuration, and semantic highlighting are planned but are not implemented yet. TextMate syntax highlighting remains the responsibility of the editor extension.

## Installation

### Pre-built Binaries

You can download pre-built binaries for various platforms from the [GitHub Releases](https://github.com/10fish/systemd-language-server-rs/releases) page.

### Building from Source

Ensure you have the Rust toolchain installed, then run:

```bash
git clone https://github.com/10fish/systemd-language-server-rs.git
cd systemd-language-server-rs
cargo build --release
```

The compiled binary will be located at `target/release/systemd-language-server`.

### Installing with Cargo

```bash
cargo install systemd-language-server
```

## Usage

### Running as a Standalone Server

```bash
systemd-language-server
```

### Editor Integration

#### VS Code

An official VS Code extension is not available yet. The server can be used through an extension or client that supports configuring an arbitrary stdio language server.

#### Neovim

Start the server from a systemd file buffer with Neovim's built-in LSP client:

```lua
vim.lsp.start({
  name = "systemd-language-server",
  cmd = { "systemd-language-server" },
  root_dir = vim.fn.getcwd(),
})
```

## Configuration

Runtime configuration is not implemented in the current pre-alpha release.

## Development

### Dependencies

- Rust 1.85+ (edition 2024)
- tokio async runtime
- tower-lsp library

### Building and Testing

```bash
# Build the project
cargo build

# Run tests
cargo test

# Run linter
cargo clippy
```

### Testing

The project includes several types of tests to ensure code quality and functionality:

#### Unit Tests

Unit tests verify the core functionality of parsing and validating systemd unit files:

- `tests/systemd_unit_tests.rs`: Tests for various systemd unit file types (service, socket, timer, mount)
- `tests/diagnostics_tests.rs`: Tests for error detection and validation

#### Integration Tests

`tests/integration_tests.rs` drives the server through JSON-RPC and covers document open/change/close, diagnostics, completion, and hover.

To run a specific test:

```bash
cargo test --test systemd_unit_tests
```

To run tests with detailed output:

```bash
cargo test -- --nocapture
```

### Continuous Integration and Deployment

This project uses GitHub Actions for continuous integration and deployment:

- **CI Workflow**: Runs tests, linting, and code coverage on every push and pull request
- **Release Workflow**: Automatically builds and publishes binaries for multiple platforms when a new tag is pushed

To create a new release:

1. Update the version in `Cargo.toml`
2. Commit the changes
3. Create and push a new tag:
   ```bash
   git tag v0.1.0
   git push origin v0.1.0
   ```
4. The GitHub Actions workflow will automatically build and publish the release

## Contributing

Pull Requests and Issues are welcome! Please ensure:

1. Add tests for new features
2. Update documentation
3. Follow the project's code style

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgements

- [tower-lsp](https://github.com/ebkalderon/tower-lsp) - LSP implementation for Rust
- [systemd](https://systemd.io/) - Provides the unit file specification
