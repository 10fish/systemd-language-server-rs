# systemd-language-server-rs Development Plan

This plan is organized as an ordered checklist you can execute item by item.

## Milestones

- [ ] M1: Stability baseline (finish all P0)
- [ ] M2: Core feature completeness (finish all P1)
- [ ] M3: Production readiness and UX polish (finish all P2)

## P0 - Blocking Issues (Must complete first)

### P0-1. Fix document state management

- [ ] Goal: Ensure opened/changed document content is actually stored and read by LSP features.
- [ ] Tasks:
  - [ ] Add a dedicated text document store module (for example `src/store.rs`).
  - [ ] Replace `Backend.documents: HashMap<Url, String>` with a concurrent store (`RwLock<HashMap<Url, String>>` or equivalent).
  - [ ] Update `did_open`, `did_change`, `did_close` to mutate shared state directly (no clone-and-drop).
  - [ ] Update `completion` and `hover` read path to use the shared store.
- [ ] Acceptance criteria:
  - [ ] After `did_open`, completion/hover use latest content.
  - [ ] After `did_change`, completion/hover reflect edited content immediately.
  - [ ] After `did_close`, document is removed and diagnostics are cleared.
  - [ ] Add tests that fail before fix and pass after fix.

### P0-2. Complete and harden unit type registry

- [ ] Goal: Avoid runtime panic and support all declared systemd unit file families.
- [ ] Tasks:
  - [ ] Register at least: `unit`, `service`, `socket`, `timer`, `mount`, `automount`, `path`, `slice`, `scope`, `device`, `swap`, `target`.
  - [ ] Implement safe fallback for unknown extension (fallback to `unit`).
  - [ ] Remove `unwrap()` on registry lookup in diagnostics path.
  - [ ] Fill currently empty files under `src/file/*.rs` with minimal valid implementations.
- [ ] Acceptance criteria:
  - [ ] No panic on unknown extension or unsupported file.
  - [ ] Known extension routes to matching schema.
  - [ ] Basic diagnostics run for all supported unit families.
  - [ ] Add tests for known + unknown extension behavior.

### P0-3. Add accurate diagnostic ranges

- [ ] Goal: Every diagnostic points to the correct section/key location.
- [ ] Tasks:
  - [ ] Add diagnostics mapping module (for example `src/diagnostics.rs`).
  - [ ] Build line/column index for section headers and key lines.
  - [ ] Replace placeholder range `(0,0)-(0,1)` with computed ranges.
  - [ ] Add diagnostic code values (syntax vs semantic categories).
- [ ] Acceptance criteria:
  - [ ] Section-level errors highlight real section lines.
  - [ ] Key-level errors highlight real key lines.
  - [ ] Syntax errors include useful location.
  - [ ] At least 10 diagnostics tests assert exact line positions.

### P0-4. Add real LSP integration tests

- [ ] Goal: Protect core LSP workflow from regression.
- [ ] Tasks:
  - [ ] Expand `tests/integration_tests.rs` beyond placeholder.
  - [ ] Add tests for `did_open -> diagnostics`.
  - [ ] Add tests for `did_change -> diagnostics update`.
  - [ ] Add tests for `completion` and `hover`.
  - [ ] Add tests for `did_close -> diagnostics cleared`.
- [ ] Acceptance criteria:
  - [ ] LSP integration tests are part of default `cargo test`.
  - [ ] Core request/notification flow is covered end-to-end.

## P1 - Core Language Server Completeness

### P1-1. Move completion/hover to schema-driven engine

- [ ] Goal: Replace hard-coded suggestions with unit-aware schema data.
- [ ] Tasks:
  - [ ] Add `src/schema/` module for section/key/value definitions.
  - [ ] Route completion by `(extension, section, key-context)`.
  - [ ] Provide value-level completion for enumerated directives (for example `Type=`, `Restart=`).
  - [ ] Bind hover docs to schema entries.
- [ ] Acceptance criteria:
  - [ ] Completion covers section names, directive keys, and common directive values.
  - [ ] Hover is available for major sections and directives from schema.
  - [ ] Unknown key handling is consistent with diagnostics.

### P1-2. Implement go-to-definition for unit references

- [ ] Goal: Deliver actual `textDocument/definition` support claimed by README.
- [ ] Tasks:
  - [ ] Add `definition_provider` module.
  - [ ] Parse references in directives like `After=`, `Requires=`, `Wants=`, `PartOf=`.
  - [ ] Resolve target files in workspace and configured search paths.
  - [ ] Return `Location`/`LocationLink` results.
- [ ] Acceptance criteria:
  - [ ] `definition` works for valid referenced units.
  - [ ] Missing target returns empty result without errors.
  - [ ] Add integration tests for positive and negative cases.

### P1-3. Implement runtime config loading

- [ ] Goal: Make `.systemd-ls.json` configuration real.
- [ ] Tasks:
  - [ ] Add `src/config.rs` and typed config model.
  - [ ] Load config from workspace root.
  - [ ] Apply `unitSearchPaths` for definition lookup.
  - [ ] Apply `diagnostics.enabled` toggle.
- [ ] Acceptance criteria:
  - [ ] Diagnostics can be disabled by config.
  - [ ] Custom search paths influence definition resolution.
  - [ ] Config read failures are reported safely (no crash).

### P1-4. Align README with implemented capabilities

- [ ] Goal: Keep docs truthful and test-backed.
- [ ] Tasks:
  - [ ] Update README/README_CN feature list to match actual behavior.
  - [ ] Add supported unit type matrix.
  - [ ] Add minimal examples for completion/hover/definition.
- [ ] Acceptance criteria:
  - [ ] Every listed feature has at least one corresponding test.
  - [ ] No unsupported feature remains documented as available.

## P2 - Production Readiness and UX

### P2-1. Add Document Symbols

- [ ] Goal: Enable outline view in editors.
- [ ] Tasks:
  - [ ] Implement `textDocument/documentSymbol`.
  - [ ] Return section and directive symbols.
- [ ] Acceptance criteria:
  - [ ] `[Unit]`, `[Service]`, etc. appear in outline.
  - [ ] Symbol ranges are accurate.

### P2-2. Add Code Actions (quick fixes)

- [ ] Goal: Provide one-click fixes for common issues.
- [ ] Tasks:
  - [ ] Implement `textDocument/codeAction`.
  - [ ] Add quick fixes for unknown section/key, missing bracket, and empty value hints.
- [ ] Acceptance criteria:
  - [ ] At least 5 code actions are test-covered.
  - [ ] Suggested edit applies cleanly and resolves target diagnostic.

### P2-3. Add workspace indexer and file watching

- [ ] Goal: Scale definition/references across large workspaces.
- [ ] Tasks:
  - [ ] Build in-memory index for unit names to file paths.
  - [ ] Refresh index incrementally on file create/change/delete.
- [ ] Acceptance criteria:
  - [ ] Definition remains responsive on medium/large workspace.
  - [ ] Index updates correctly after file operations.

### P2-4. Add quality gates and CI hardening

- [ ] Goal: Prevent quality regressions.
- [ ] Tasks:
  - [ ] Enforce `cargo fmt --check`.
  - [ ] Enforce `cargo clippy -D warnings`.
  - [ ] Split CI into unit/integration layers.
- [ ] Acceptance criteria:
  - [ ] CI fails on warnings or formatting issues.
  - [ ] Main branch remains test-green.

## Recommended Execution Order

- [ ] Step 1: P0-1 document state management
- [ ] Step 2: P0-2 unit registry completeness and panic removal
- [ ] Step 3: P0-3 diagnostic range mapping
- [ ] Step 4: P0-4 LSP integration test baseline
- [ ] Step 5: P1-1 schema-driven completion/hover
- [ ] Step 6: P1-2 go-to-definition
- [ ] Step 7: P1-3 config loading
- [ ] Step 8: P1-4 README alignment
- [ ] Step 9: P2 items in order (2-1 -> 2-4)

## Definition of Done (Project Level)

- [ ] No panic in normal LSP workflow.
- [ ] Completion, hover, diagnostics, and definition are all integration-tested.
- [ ] README and README_CN match actual shipped capabilities.
- [ ] CI quality gates are enabled and passing.
