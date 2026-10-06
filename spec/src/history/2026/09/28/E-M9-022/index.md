---
schema: 1
id: E-M9-022
---

# E-M9-022

<!-- evidence-field:id markdown -->
## Id

E-M9-022
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
- crates/jj-store-cli/src/agent_removal.rs
- crates/jj-store-cli/src/agent_setup.rs
- crates/jj-store-cli/src/agent_setup_files.rs
- crates/jj-store-cli/src/lib.rs
- crates/jj-store-cli/tests/agent_setup.rs
- docs/cli/agent.md
- spec/src/architecture.md
- spec/src/cli.md
- spec/src/collaboration.md
- spec/src/milestones.md
- spec/src/verification.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/agent_setup.rs#removal_restores_exact_original_and_disables_integration
- crates/jj-store-cli/tests/agent_setup.rs#removal_rejects_drift_and_deletes_only_an_owned_new_target
- crates/jj-store-cli/tests/agent_setup.rs#setup_command_selects_one_explicit_integration
<!-- /evidence-field -->

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-cli --test agent_setup --locked
```

**Notes**

The test-first compile proved that neither the removal request/function nor the remove command variant existed.
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
cargo test -p jj-store-cli --test agent_setup --locked
```

**Notes**

The first implementation compile found one server-policy error that needed explicit coercion into the CLI authority error boundary.
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
cargo test -p jj-store-cli --test agent_setup --locked
```

**Notes**

All six focused setup and removal cases pass, including exact restoration, created-file deletion, disabled policy, dry-run, idempotence, crash tombstone recovery and drift preservation.
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
pnpm check:boundaries
```

**Notes**

All 13 package dependency boundaries pass with removal retained inside the CLI-owned device lifecycle.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first
```

**Notes**

The test-first specification gate required the changed agent command and setup boundary to update its mapped canonical CLI chapter.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:boundaries; pnpm spec:validate; pnpm spec:first; pnpm check:progress
```

**Notes**

Boundary and specification gates passed; the progress checker caught that renaming the existing setup grammar test invalidated two historical evidence references, so the stable test name was retained.
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
cargo test -p jj-store-cli --test agent_setup --test agent_integrations --locked
```

**Notes**

All six setup/removal regressions and three registry regressions pass, including exact restoration, created-file deletion, disabled policy, dry-run, idempotence, crash tombstone recovery, drift and unexpected ownership-file preservation, and unchanged integration discovery.
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
cargo clippy -p jj-store-cli --lib --test agent_setup --test agent_integrations --locked -- -D warnings; cargo fmt --all -- --check
```

**Notes**

The changed CLI library and both directly affected regressions are warning-free and formatted. Production modules remain at or below 266 physical lines and the setup regression has 417.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:boundaries; pnpm spec:validate; pnpm spec:first; pnpm check:progress; pnpm exec prettier --check docs/cli/agent.md spec/src/architecture.md spec/src/cli.md spec/src/collaboration.md spec/src/milestones.md spec/src/verification.md
```

**Notes**

All 13 package boundaries, 89 requirements, eight protected files across four canonical chapters and 61 progress tasks pass; every changed document is formatted.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Execute pinned real-harness capability and smoke tests in M9.6; Cursor remains
  unsupported on the current host
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; setup ownership record v1; Codex 0.156.1; Cursor project hooks version 1 with synthetic 3.22.7 acceptance; OpenCode 1.18.30.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Adds no serialized format. Removal consumes the existing exact ownership record, restores or deletes the target before atomically renaming ownership state to an integration-specific private tombstone, synchronizes its parent and records disabled/not-installed policy only after cleanup.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


