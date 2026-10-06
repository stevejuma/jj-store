---
schema: 1
id: E-M9-008
---

# E-M9-008

<!-- evidence-field:id markdown -->
## Id

E-M9-008
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M9
- M9.2
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-28
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

implementation
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-agent/src/lib.rs
- crates/jj-agent/src/normalize.rs
- crates/jj-agent/src/reducer.rs
- crates/jj-agent/src/normalizers/mod.rs
- crates/jj-agent/src/normalizers/codex.rs
- crates/jj-agent/src/normalizers/cursor.rs
- crates/jj-agent/src/normalizers/cursor_tool.rs
- crates/jj-agent/src/normalizers/opencode.rs
- crates/jj-agent/src/normalizers/opencode_tools.rs
- crates/jj-agent/tests/fixtures/codex-0.156.1-hooks.json
- crates/jj-agent/tests/fixtures/cursor-3.22.7-hooks.json
- crates/jj-agent/tests/fixtures/opencode-1.18.30-plugin.json
- crates/jj-agent/tests/normalizers.rs
- spec/src/collaboration.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-agent/tests/normalizers.rs#pinned_harness_fixtures_normalize_to_common_activity
- crates/jj-agent/tests/normalizers.rs#lifecycle_records_share_ids_and_reducer_never_regresses_terminal_state
- crates/jj-agent/tests/normalizers.rs#unsupported_versions_degrade_without_parsing_native_records
- crates/jj-agent/tests/normalizers.rs#resumed_sessions_and_mixed_sources_keep_stable_ids
- crates/jj-agent/tests/normalizers.rs#cursor_file_observations_do_not_invent_tools_and_unknown_tools_stay_other
- crates/jj-agent/tests/normalizers.rs#malformed_unknown_and_embedded_version_failures_are_safe_diagnostics
- crates/jj-agent/tests/normalizers.rs#reducer_keeps_conflicting_terminal_states_explicit
<!-- /evidence-field -->

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-agent --test normalizers --locked
```

**Notes**

The test-first compile exposed the missing public normalizer profile, normalized record/batch, diagnostic, dispatch and reducer APIs.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-agent --test normalizers --locked
```

**Notes**

Seven focused tests pass for all three pinned fixtures, deterministic replay, stable resumed-session IDs, output-before-input reduction, terminal conflict reporting, conservative file/tool handling and safe version or malformed-record diagnostics.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo fmt --all -- --check
```

**Notes**

The focused formatter reported mechanical Rust wrapping after the first implementation pass.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-agent --locked; DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo clippy -p jj-agent --all-targets --locked -- -D warnings; DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo fmt --all -- --check
```

**Notes**

All AgentStore model and normalizer tests pass; the affected crate is warning-free and formatted. Every new production module is below 300 physical lines and the normalizer test is below 500.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:boundaries; pnpm test:governance; pnpm spec:validate; pnpm spec:first; pnpm check:progress; pnpm spec:build; pnpm exec prettier --check spec/src/collaboration.md spec/src/milestones.md
```

**Notes**

Package boundaries, governance, canonical requirement and progress checks pass; the canonical book builds and changed specification prose is formatted.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Owned integration discovery, durable spool and capture publication path
- Released registry acceptance for @lapismd/agent-redaction
- Pinned real-harness capability matrix and smoke tests in M9.6
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; Codex CLI 0.156.1 installed contract; Cursor 3.22.7 synthetic contract fixture only; OpenCode plugin/SDK 1.18.30 with recorded npm integrity; @lapismd/agent-redaction 0.1.0 remains sibling-resolved.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

NormalizerProfileV1 dispatches exact version pairs. NormalizationBatchV1 retains source-ordered lifecycle records, scoped stable IDs, explicit aliases and safe diagnostics. ReducedToolsV1 enriches stable tools without terminal regression or implicit conflict resolution.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


