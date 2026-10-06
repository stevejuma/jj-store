---
schema: 1
id: E-M9-016
---

# E-M9-016

<!-- evidence-field:id markdown -->
## Id

E-M9-016
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

- crates/jj-store-cli/src/agent_commands.rs
- crates/jj-store-cli/src/agent_integrations.rs
- crates/jj-store-cli/src/agent_probe.rs
- crates/jj-store-cli/src/commands.rs
- crates/jj-store-cli/src/lib.rs
- crates/jj-store-cli/src/store_commands.rs
- crates/jj-store-cli/src/token_commands.rs
- crates/jj-store-cli/tests/agent_integrations.rs
- docs/cli/agent.md
- spec/src/architecture.md
- spec/src/cli.md
- spec/src/collaboration.md
- spec/src/verification.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/agent_integrations.rs#registry_probes_pinned_harnesses_and_persists_explicit_enablement
- crates/jj-store-cli/tests/agent_integrations.rs#registry_reports_missing_and_failed_probes_without_forwarding_output
- crates/jj-store-cli/tests/agent_integrations.rs#agent_command_requires_explicit_noninteractive_enablement
<!-- /evidence-field -->

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test agent_integrations --locked
```

**Notes**

The test-first compile reported the intentionally missing agent_integrations module before the owned registry implementation existed.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test agent_integrations --locked
```

**Notes**

The production-size extraction exposed one stale internal ConnectionArgs import; the import was moved to the extracted store_commands owner without changing the public command grammar.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test agent_integrations --locked
```

**Notes**

All three focused registry and command regressions pass: exact support classification and restart persistence, secret-free failed probes, and explicit noninteractive dry-run grammar.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo clippy -p jj-store-cli --test agent_integrations --locked -- -D warnings; DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo fmt --all -- --check
```

**Notes**

The affected CLI library and focused integration target are warning-free and the Rust workspace is formatted.
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
wc -l crates/jj-store-cli/src/{commands,store_commands,agent_commands,agent_integrations,agent_probe}.rs
```

**Notes**

The production modules contain 287, 135, 144, 276 and 131 physical lines respectively, each below the 300-line limit.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:boundaries; pnpm test:governance; pnpm spec:validate; pnpm spec:first; pnpm check:progress; pnpm exec prettier --check docs/cli/agent.md spec/src/architecture.md spec/src/cli.md spec/src/collaboration.md spec/src/verification.md spec/src/milestones.md
```

**Notes**

All 13 package boundaries and 26 governance tests pass; all 85 requirements validate, nine protected files map to four canonical chapters, all 60 progress tasks are consistent and every changed prose file is formatted.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Drift-safe owned Codex and OpenCode hook/plugin setup plus capture ingestion;
  Cursor remains version-unsupported on the qualification host
- Released registry acceptance for @lapismd/agent-redaction
- Pinned real-harness capability matrix and smoke tests in M9.6
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; device-local agent integration registry v1; Codex 0.156.1; Cursor 3.22.7; OpenCode 1.18.30.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

AgentDeviceStatusV1 binds a nonzero stable DeviceId, RepositoryId and canonical workspace to explicit Codex, Cursor and OpenCode entries. Detection support, desired enablement and setup state remain separate. Registry replacement is locked, atomic, synchronized and user-restricted; probes have a two-second timeout and 256-byte stdout limit while stderr is discarded.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


