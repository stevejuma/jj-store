---
schema: 1
id: E-M9-017
---

# E-M9-017

<!-- evidence-field:id markdown -->
## Id

E-M9-017
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
- crates/jj-store-cli/src/agent_setup.rs
- crates/jj-store-cli/src/agent_setup_files.rs
- crates/jj-store-cli/src/agent_setup_templates.rs
- crates/jj-store-cli/src/lib.rs
- crates/jj-store-cli/tests/agent_setup.rs
- docs/cli/agent.md
- spec/src/architecture.md
- spec/src/cli.md
- spec/src/collaboration.md
- spec/src/verification.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/agent_setup.rs#setup_merges_only_owned_hooks_and_preserves_original_backups
- crates/jj-store-cli/tests/agent_setup.rs#setup_recovers_owned_state_and_refuses_drifted_configuration
- crates/jj-store-cli/tests/agent_setup.rs#setup_rejects_project_configuration_symlinks
- crates/jj-store-cli/tests/agent_setup.rs#setup_command_selects_one_explicit_integration
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test agent_setup --locked
```

**Notes**

The test-first compile reported the intentionally missing agent_setup module before the owned setup implementation existed.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test agent_setup --locked
```

**Notes**

Restart and drift recovery passed; the preservation case exposed a test assertion expecting a shell string where the OpenCode plugin deliberately uses a safer argv array.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo clippy -p jj-store-cli --test agent_setup --test agent_integrations --locked -- -D warnings
```

**Notes**

The first targeted Clippy run found one unused test-helper lifetime; production behavior was unchanged.
<!-- /run -->

<!-- run -->
### Run 4

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

The final ownership-record validation addition required one rustfmt line wrap.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test agent_setup --locked; DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test agent_integrations --locked
```

**Notes**

All four setup regressions and all three shared registry regressions pass, covering namespaced merges and exact backups, restart/idempotency and drift refusal, project symlink rejection, explicit setup grammar and unchanged support probing.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo clippy -p jj-store-cli --test agent_setup --test agent_integrations --locked -- -D warnings; DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo fmt --all -- --check
```

**Notes**

The affected CLI library plus both directly affected integration targets are warning-free and formatted.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
wc -l crates/jj-store-cli/src/{agent_commands,agent_integrations,agent_setup,agent_setup_files,agent_setup_templates}.rs crates/jj-store-cli/tests/agent_setup.rs
```

**Notes**

The production modules contain 179, 293, 181, 197 and 148 physical lines; the focused regression contains 239. Every file stays below its governed limit.
<!-- /run -->

<!-- run -->
### Run 8

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

All 13 package boundaries and 26 governance tests pass; all 85 requirements validate, eight protected files map to four canonical chapters, all 60 progress tasks are consistent and every changed prose file is formatted.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Package and wire the jj agent hook ingestion path through
  @lapismd/agent-redaction into the durable capture spool and publication
  recovery
- Add drift-safe owned removal after the ingesting setup is qualified
- Execute pinned real-harness capability and smoke tests in M9.6; Cursor remains
  unsupported on the current host
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; setup ownership record v1; Codex 0.156.1 from openai/codex rust-v0.156.1 source b412ff32c417f855c2b2d1581b77058eed87c84b; Cursor project hooks version 1 with synthetic 3.22.7 acceptance; OpenCode 1.18.30 from anomalyco/opencode v1.18.30 source 3104c1428ec91f809e5ab86631300de41eb6952e.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Project-scoped Codex and Cursor JSON documents retain unrelated keys and append only the owned hook command. OpenCode uses one namespaced TypeScript plugin and an argv spawn. The private ownership record binds integration, exact target, original optional SHA-256, owned SHA-256 and exact original bytes; setup writes the record before atomic project replacement, resumes only the recorded original-to-owned transition and treats every other state as drift.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


