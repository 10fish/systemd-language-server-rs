# Systemd Language Server

[English🇺🇸](README.md) | [简体中文🇨🇳](README_CN.md)

[![Crates.io](https://img.shields.io/crates/v/systemd-language-server.svg)](https://crates.io/crates/systemd-language-server)
[![Build Status](https://github.com/10fish/systemd-language-server-rs/workflows/Rust/badge.svg)](https://github.com/10fish/systemd-language-server-rs/actions)
[![codecov](https://codecov.io/gh/10fish/systemd-language-server-rs/branch/main/graph/badge.svg)](https://codecov.io/gh/10fish/systemd-language-server-rs)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Downloads](https://img.shields.io/crates/d/systemd-language-server.svg)](https://crates.io/crates/systemd-language-server)
[![Rust Version](https://img.shields.io/badge/rust-1.70%2B-blue.svg)](https://www.rust-lang.org/)
[![dependency status](https://deps.rs/repo/github/10fish/systemd-language-server-rs/status.svg)](https://deps.rs/repo/github/10fish/systemd-language-server-rs)

一个基于 Rust 和 Language Server Protocol (LSP) 的 systemd unit 文件语言服务器，提供语法高亮、自动补全、错误检查等功能。

## 功能特性

- 支持 systemd unit 文件的语法高亮
- 提供智能自动补全建议
- 实时语法错误检查
- 支持跳转到定义
- 悬停提示文档

## 安装

### 从源码构建

确保您已安装 Rust 工具链，然后执行：

```bash
git clone https://github.com/10fish/systemd-language-server-rs.git
cd systemd-language-server-rs
cargo build --release
```

编译后的二进制文件将位于 `target/release/systemd-language-server`。

### 使用 Cargo 安装

```bash
cargo install systemd-language-server
```

## 使用方法

### 作为独立服务器运行

```bash
systemd-language-server
```

### 与编辑器集成

#### VS Code

1. 安装 [vscode-languageclient](https://marketplace.visualstudio.com/items?itemName=ms-vscode.vscode-languageserver-node-example) 扩展
2. 在设置中配置 systemd-language-server 的路径

#### Neovim

使用 [nvim-lspconfig](https://github.com/neovim/nvim-lspconfig) 配置：

```lua
require'lspconfig'.systemd_ls.setup{
  cmd = { "systemd-language-server" },
  filetypes = { "systemd" },
  root_dir = function() return vim.loop.cwd() end
}
```

## 配置选项

在 `.systemd-ls.json` 文件中可以配置以下选项：

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

## 开发

### 依赖项

- Rust 1.70+
- tokio 异步运行时
- tower-lsp 库

### 构建与测试

```bash
# 构建项目
cargo build

# 运行测试
cargo test

# 运行 linter
cargo clippy
```

### 测试

本项目包含多种类型的测试，以确保代码质量和功能正常：

#### 单元测试

单元测试验证解析和验证 systemd unit 文件的核心功能：

- `tests/systemd_unit_tests.rs`：测试各种 systemd unit 文件类型（service、socket、timer、mount）
- `tests/diagnostics_tests.rs`：测试错误检测和验证功能

#### 集成测试

`tests/integration_tests.rs` 中包含一个简化版的集成测试。这是一个占位测试，可以在将来需要时进行扩展。

运行特定测试：

```bash
cargo test --test systemd_unit_tests
```

运行测试并显示详细输出：

```bash
cargo test -- --nocapture
```

## 贡献指南

欢迎提交 Pull Request 和 Issue！请确保：

1. 为新功能添加测试
2. 更新文档
3. 遵循项目的代码风格

## 许可证

本项目采用 MIT 许可证 - 详见 [LICENSE](LICENSE) 文件。

## 致谢

- [tower-lsp](https://github.com/ebkalderon/tower-lsp) - Rust 的 LSP 实现
- [systemd](https://systemd.io/) - 提供了 unit 文件规范 