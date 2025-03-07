# Systemd Language Server

[English🇺🇸](README.md) | [简体中文🇨🇳](README_CN.md)

[![Crates.io](https://img.shields.io/crates/v/systemd-language-server.svg)](https://crates.io/crates/systemd-language-server)
[![Build Status](https://github.com/10fish/systemd-language-server-rs/workflows/Rust/badge.svg)](https://github.com/10fish/systemd-language-server-rs/actions)
[![codecov](https://codecov.io/gh/10fish/systemd-language-server-rs/branch/main/graph/badge.svg)](https://codecov.io/gh/10fish/systemd-language-server-rs)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Downloads](https://img.shields.io/crates/d/systemd-language-server.svg)](https://crates.io/crates/systemd-language-server)
[![Rust Version](https://img.shields.io/badge/rust-1.70%2B-blue.svg)](https://www.rust-lang.org/)
[![dependency status](https://deps.rs/repo/github/10fish/systemd-language-server-rs/status.svg)](https://deps.rs/repo/github/10fish/systemd-language-server-rs)

A Rust and Language Server Protocol (LSP) based language server for systemd unit files, providing syntax highlighting, auto-completion, error checking, and more.

## Features

- Syntax highlighting for systemd unit files
- Intelligent auto-completion suggestions
- Real-time syntax error checking
- Support for jumping to definitions
- Hover documentation tooltips

## Installation

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

1. Install the [vscode-languageclient](https://marketplace.visualstudio.com/items?itemName=ms-vscode.vscode-languageserver-node-example) extension
2. Configure the systemd-language-server path in settings

#### Neovim

Use [nvim-lspconfig](https://github.com/neovim/nvim-lspconfig) configuration:

```lua
require'lspconfig'.systemd_ls.setup{
  cmd = { "systemd-language-server" },
  filetypes = { "systemd" },
  root_dir = function() return vim.loop.cwd() end
}
```

## Configuration Options

You can configure the following options in the `.systemd-ls.json` file:

```json
{
  "systemd": {
    "unitSearchPaths": ["/etc/systemd/system", "/usr/lib/systemd/system"],
    "diagnostics": {
      "enabled": true
    }
  }
}
```

## Development

### Dependencies

- Rust 1.70+
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

A simplified integration test is included in `tests/integration_tests.rs`. This serves as a placeholder and can be expanded in the future when needed.

To run a specific test:

```bash
cargo test --test systemd_unit_tests
```

To run tests with detailed output:

```bash
cargo test -- --nocapture
```

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