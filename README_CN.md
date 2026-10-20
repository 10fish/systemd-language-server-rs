# Systemd Language Server

[English🇺🇸](README.md) | [简体中文🇨🇳](README_CN.md)

[![Crates.io](https://img.shields.io/crates/v/systemd-language-server.svg)](https://crates.io/crates/systemd-language-server)
[![Build Status](https://github.com/10fish/systemd-language-server-rs/workflows/CI/badge.svg)](https://github.com/10fish/systemd-language-server-rs/actions)
[![codecov](https://codecov.io/gh/10fish/systemd-language-server-rs/branch/main/graph/badge.svg)](https://codecov.io/gh/10fish/systemd-language-server-rs)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Downloads](https://img.shields.io/crates/d/systemd-language-server.svg)](https://crates.io/crates/systemd-language-server)
[![Rust Version](https://img.shields.io/badge/rust-1.85%2B-blue.svg)](https://www.rust-lang.org/)
[![dependency status](https://deps.rs/repo/github/10fish/systemd-language-server-rs/status.svg)](https://deps.rs/repo/github/10fish/systemd-language-server-rs)

一个处于 pre-alpha 阶段、使用 Rust 编写的 systemd unit 文件语言服务器。

## 功能特性

- 带版本检查的全文文档同步
- 带准确 UTF-16 范围的语法和 section 诊断
- section、directive 和部分枚举值补全
- 常见 section 和 directive 的悬停文档
- 识别常见 unit 类型和 `.d/*.conf` drop-in

跳转到定义、运行时配置和 semantic highlighting 尚未实现。TextMate 语法高亮由编辑器扩展提供。

## 安装

### 预编译二进制文件

您可以从 [GitHub Releases](https://github.com/10fish/systemd-language-server-rs/releases) 页面下载适用于各种平台的预编译二进制文件。

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

目前还没有官方 VS Code 扩展。可以通过支持配置任意 stdio language server 的扩展或客户端启动本服务。

#### Neovim

在 systemd 文件 buffer 中使用 Neovim 内置 LSP 客户端启动：

```lua
vim.lsp.start({
  name = "systemd-language-server",
  cmd = { "systemd-language-server" },
  root_dir = vim.fn.getcwd(),
})
```

## 配置

当前 pre-alpha 版本尚未实现运行时配置。

## 开发

### 依赖项

- Rust 1.85+（edition 2024）
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

`tests/integration_tests.rs` 通过 JSON-RPC 驱动服务端，覆盖文档打开、修改、关闭、诊断、补全和悬停。

运行特定测试：

```bash
cargo test --test systemd_unit_tests
```

运行测试并显示详细输出：

```bash
cargo test -- --nocapture
```

### 持续集成与部署

本项目使用 GitHub Actions 进行持续集成和部署：

- **CI 工作流**：在每次推送和拉取请求时运行测试、代码检查和代码覆盖率分析
- **发布工作流**：当推送新标签时，自动构建并发布多平台二进制文件

创建新版本发布的步骤：

1. 更新 `Cargo.toml` 中的版本号
2. 提交更改
3. 创建并推送新标签：
   ```bash
   git tag v0.1.0
   git push origin v0.1.0
   ```
4. GitHub Actions 工作流将自动构建并发布该版本

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
