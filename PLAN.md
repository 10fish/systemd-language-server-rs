# systemd-language-server-rs 项目规划

更新日期：2026-10-19

## 1. 当前结论

项目目前处于 pre-alpha 阶段。P0 稳定性问题已经解决，核心 LSP 生命周期可以工作，但 schema 覆盖、跳转和配置等功能仍不足以作为稳定版本发布。

已有成果：

- 已建立 Rust crate、stdio LSP 入口和 GitHub Actions 工作流。
- 服务端声明了全文同步、补全和悬停能力。
- 打开或修改文档时会尝试解析文本并发布诊断。
- 有少量硬编码的 section、directive 补全和英文悬停说明。
- 已有 parser、schema、兼容 API 和真实 JSON-RPC 生命周期测试；当前 `cargo test --all-targets` 全部通过。
- 已发布过 `v0.1.0` 标签和多平台 release 工作流。

文档状态、普通 unit panic、错误必填语义、诊断 range、占位集成测试和 CI 质量门禁已在 P0 阶段修复。当前工作的重点转为扩大 schema 覆盖和完成 P1 功能。

## 2. 功能完成度

| 能力 | 当前状态 | 主要问题 |
| --- | --- | --- |
| LSP 启动与初始化 | 基本完成 | 缺少真实协议测试 |
| 文档同步 | P0 完成 | 全文同步、版本检查和关闭清理已有协议测试 |
| 语法解析 | P0 基线完成 | 已保留 range、重复指令和顺序；仍需扩大错误恢复测试 |
| 诊断 | P0 基线完成 | 已消除 panic 和错误必填语义；directive schema 诊断待 P1 完善 |
| 补全 | 基础可用 | 已按 unit/section/value 上下文提供结果，schema 覆盖仍有限 |
| Hover | 基础可用 | 状态读取和 token range 正常，文档覆盖仍有限 |
| Definition | 未实现 | README 声称支持，但服务端没有注册该能力 |
| 语法高亮 | 未实现 | 没有 semantic tokens，也没有随项目提供编辑器 grammar |
| 配置 | 未实现 | README 中的 `.systemd-ls.json` 没有读取逻辑 |
| 单元测试 | 很弱 | 主要只验证第三方 INI parser，而非本项目行为 |
| LSP 集成测试 | P0 完成 | 覆盖 open/change/close、diagnostics、completion 和 hover |
| CI | P0 完成 | test、fmt、严格 clippy 通过，重复 workflow 已移除 |
| 发布与文档 | 部分完成 | README 已与当前能力对齐，release artifact 仍需端到端验证 |

粗略完成度：基础工程约 75%，核心语言能力约 30%，可发布产品能力约 30%—35%。此数字用于排期，不作为质量指标。

## 3. 已解决的 P0 问题

以下问题已在 P0 实现中解决，保留记录供回归检查。

1. 文档状态没有持久化。
   - 已用并发安全的 `DocumentStore` 替换临时 clone，并加入版本检查和生命周期协议测试。

2. 普通 unit 的诊断会 panic。
   - 已用安全的 `UnitType` 识别和通用 fallback 替换不完整注册表，并支持 drop-in 类型推导。

3. schema 的 `required` 语义错误。
   - 已分离允许的 section 和 unit 特有的必要 section，不再把所有 directive 当作必填项。

4. parser 数据模型不适合 language server。
   - 已建立保留顺序、重复 directive、容错错误和 UTF-16 range 的最小 AST；旧 INI parser 仅保留为 0.1 公共 API 的兼容入口。

5. 测试没有覆盖真实服务端行为。
   - 已加入真实 JSON-RPC 测试，并断言 diagnostics 通知、completion、hover 和关闭后的状态清理。

6. 质量门禁失败且 CI 重复。
   - 已移除旧原型和重复 workflow；test、fmt、`clippy -D warnings` 现均通过。

## 4. 目标范围

### v0.2.0：可用的最小版本

支持常用的 system unit 文件：`service`、`socket`、`timer`、`path`、`mount`、`automount`、`swap`、`target`、`slice`、`scope`、`device`，以及 `.d/*.conf` drop-in。

必须交付：

- 稳定的全文文档同步；
- section、directive 和常见枚举值补全；
- section/directive hover 文档；
- 有准确 range 的语法及 schema 诊断；
- unit 引用的 go-to-definition；
- 配置加载；
- 真实 LSP 集成测试；
- README 与实际能力一致，CI 全绿。

### 暂不纳入 v0.2.0

- 完整复刻 `systemd-analyze verify` 的运行时验证；
- 对所有 systemd 版本和所有发行版私有 directive 的覆盖；
- rename、references、formatting、inlay hints；
- language server 自身提供 TextMate 语法高亮。语法高亮通常由编辑器扩展负责，semantic tokens 可在后续版本评估。

## 5. 建议架构

```text
LSP handlers
    │
    ├── DocumentStore (URI -> text/version/line index/parsed document)
    │                         │
    │                         └── Parser -> AST + source ranges + parse errors
    │
    ├── SchemaRegistry (unit type -> sections -> directives -> values/docs)
    │
    ├── DiagnosticsEngine (AST + schema -> diagnostics)
    ├── Completion/Hover (cursor context + schema)
    └── UnitIndex (workspace/search paths -> unit definitions)
```

主要设计决定：

- 自建轻量、容错的 systemd parser，不把通用 INI map 作为核心模型。编辑中的半成品文本也必须能产生部分 AST。
- AST 保留节点顺序、重复 directive、原始文本和 UTF-16 range。
- `SchemaRegistry` 统一驱动诊断、补全和 hover，避免三套硬编码数据互相漂移。
- schema 先覆盖常用 unit 类型和高频 directive，并记录适用 section、值类型、可重复性和简短文档。
- 工作区索引与系统目录索引分开；读取系统路径失败时降级，不影响编辑体验。
- 第一版继续使用 `TextDocumentSyncKind::FULL`，正确性稳定后再评估增量同步。

## 6. 实施路线图

以下估算按一名熟悉 Rust/LSP 的开发者计算，包含实现和测试，不包含长期维护。

### M0：恢复可信基线（1—2 天）

- [x] 修复 rustfmt 和 clippy 错误，使本地质量命令全绿。
- [x] 合并重复 CI，仅保留 build、test、fmt、clippy 和 coverage。
- [x] 修正 README 的仓库链接、Rust 最低版本和已实现功能。
- [x] 将当前版本明确标为 pre-alpha，避免 README 给出不可用的编辑器配置承诺。
- [x] 增加真实 JSON-RPC 生命周期测试。

验收：

- `cargo test --all-targets` 通过。
- `cargo fmt --all -- --check` 通过。
- `cargo clippy --all-targets -- -D warnings` 通过。
- README 中每个“已支持”能力都有对应测试。

### M1：文档模型与容错 parser（4—7 天）

- [x] 新增并发安全的 `DocumentStore`，保存 URI、版本、文本和解析结果。
- [x] 正确实现 open/change/close；拒绝或忽略过期版本。
- [x] 定义 `Document`、`Section`、`Directive`、`ParseError` 及 range。
- [x] 支持注释、空行、重复 directive、空值、续行和不完整 section/key。
- [x] 统一 UTF-8 字节偏移到 LSP UTF-16 position 的转换。
- [ ] 保留全文同步，删除重复的公开/私有 parse 函数。

验收：

- open 后能查询文本，change 后立即更新，close 后移除。
- parser 对合法重复 directive 不丢数据。
- 含中文或 emoji 的行能返回正确 UTF-16 range。
- 至少覆盖 20 个 parser/range 用例和文档版本用例。

### M2：统一 schema 与可信诊断（5—8 天）

- [ ] 用 `SchemaRegistry` 替换当前 `Systemd` trait、`Sytax` 和静态 map 原型。
- [ ] 区分 `allowed`、`required`、`repeatable` 和 `value_kind`。
- [ ] 先覆盖 `Unit`、`Install`、`Service`、`Socket`、`Timer`、`Path`、`Mount`。
- [ ] 再补齐 `Automount`、`Swap`、`Target`、`Slice`、`Scope`、`Device`。
- [ ] 为未知 section、未知 directive、非法枚举值、缺失必要项生成带 code 的诊断。
- [ ] 明确 drop-in 的规则：文件名为 `.conf` 时根据目录名推导 unit 类型，不能按扩展名判断。
- [ ] 对未知 unit 类型安全降级为通用 `[Unit]`/`[Install]` schema，禁止 panic。
- [ ] 评估可选调用 `systemd-analyze verify`，只作为额外诊断源，不作为核心 parser。

验收：

- 所有目标 unit 类型和未知扩展均不崩溃。
- 正常 service 示例没有“所有字段都必填”的误报。
- 每个诊断定位真实 token，并有稳定 code、severity 和消息。
- 至少 30 个 schema/diagnostic 测试，包含 drop-in、重复项和错误恢复。

### M3：补全与 Hover（4—6 天）

- [ ] 从 cursor position 推导 unit 类型、当前 section、key/value 上下文。
- [ ] 基于同一 schema 提供 section、directive、枚举值补全。
- [ ] 设置 `kind`、`detail`、documentation、insert text 和合理的替换 range。
- [ ] 已出现但不可重复的 directive 不再推荐。
- [ ] hover 仅在 cursor 位于对应 token 时出现，并显示简短说明、值类型和可选值。
- [ ] 为不完整输入（`[Ser`、`Rest`、`Type=`）提供稳定结果。

验收：

- `.service`、`.socket`、`.timer` 至少各有一组端到端补全测试。
- open/change 后补全和 hover 使用最新文本。
- schema、diagnostic、completion 和 hover 对同一 directive 的认识一致。

### M4：Definition、配置与工作区索引（4—7 天）

- [ ] 建立 unit name 到文件 URI 的索引。
- [ ] 索引 workspace folders、配置的搜索路径，并可选索引常见系统目录。
- [ ] 解析 `Requires=`、`Wants=`、`After=`、`Before=`、`PartOf=`、`OnFailure=`、`Unit=` 等引用。
- [ ] 实现 `textDocument/definition`，支持一行多个 unit 名称。
- [ ] 定义 typed configuration，并通过 LSP workspace configuration 或初始化选项读取。
- [ ] 支持诊断开关、额外搜索路径和目标 systemd 版本。
- [ ] 文件变化时增量更新索引。

验收：

- 本工作区和额外搜索路径中的引用可以跳转。
- 引用不存在或路径不可读时返回空结果且不报协议错误。
- 配置变化后无需重启即可影响诊断或索引。
- 正向、负向、多个候选和配置更新均有集成测试。

### M5：编辑器可用性与发布（3—5 天）

- [ ] 实现 document symbols，供编辑器 outline 使用。
- [ ] 增加 VS Code 或 Neovim 的可复现实测配置；至少选一个作为官方支持入口。
- [ ] 修复 release 打包目录和压缩逻辑，并验证 Linux、macOS、Windows artifact。
- [ ] 设置 `rust-version`，提交 `Cargo.lock`（二进制项目），生成 changelog。
- [ ] 更新中英文 README、支持矩阵、安装方式和故障排查。
- [ ] 用一个包含多种 unit/drop-in 的 fixture workspace 做发布前验收。

验收：

- 新环境可根据 README 安装并在编辑器中启动。
- release artifact 解压后可运行并完成 initialize/open/completion/hover/definition。
- CI 在支持的最低 Rust 版本和 stable Rust 上均通过。

预计 v0.2.0 总工期：17—28 个开发日。若先只交付 `.service` 并暂缓 definition，可在约 8—12 个开发日内形成可试用 alpha。

## 7. 后续版本候选

### v0.3.0

- semantic tokens（如果目标编辑器缺少成熟的 systemd grammar）；
- code actions：修正 section/directive 拼写、补齐括号等；
- signature help 或更丰富的值补全；
- workspace symbols 和 references；
- schema 的 systemd 版本条件及弃用提示。

### v1.0.0

- 明确兼容的 systemd 版本范围；
- 大型工作区性能基准和内存上限；
- fuzz/property tests 覆盖 parser，确保任意编辑文本不 panic；
- 稳定配置格式和诊断 code；
- 至少两个主流编辑器完成安装与回归验证。

## 8. 每个里程碑的完成定义

- 功能通过公共接口或真实 LSP JSON-RPC 测试验证，不以私有实现存在作为“完成”。
- 新代码通过 test、fmt、clippy，且正常编辑输入不会 panic。
- 用户可见能力同步更新 README 和示例。
- 诊断、补全与 hover 共用 schema，不重复维护事实数据。
- range 使用 LSP UTF-16 坐标，并有非 ASCII 测试。
- 性能敏感改动至少记录一个可复现的基准输入和结果。

## 9. 下一步建议

从 M0 和 M1 开始，不要继续向当前 `src/file/*.rs` 的 trait 原型批量填硬编码列表。第一批实现建议拆为四个小 PR：

1. CI/README 基线修复；
2. `DocumentStore` 与真实 LSP 生命周期测试；
3. 带 range 的容错 parser；
4. 最小 schema 和 `.service` 诊断。

完成这四步后，再扩展 unit 类型、补全和 hover，后续功能便可复用同一个可靠的数据模型。
