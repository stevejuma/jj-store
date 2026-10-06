---
schema: 1
id: E-M9-033
---

# E-M9-033

<!-- evidence-field:id markdown -->
## Id

E-M9-033
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M9
- M9.6
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-29
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

collaboration-cold-copy-qualification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/tests/agent_integrations.rs
- crates/jj-store-wal/src/catalog.rs
- crates/jj-store-wal/src/publication.rs
- crates/jj-store-wal/src/publication/agent.rs
- crates/jj-store-wal/src/publication/native_copy.rs
- crates/jj-store-wal/src/publication/review.rs
- crates/jj-store-wal/src/records.rs
- crates/jj-store-wal/tests/native_copy.rs
- spec/src/cli.md
- spec/src/collaboration.md
- spec/src/milestones.md
- spec/src/publication.md
- spec/src/verification.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/agent_integrations.rs#real_path_qualification_accepts_codex_and_opencode_and_rejects_cursor_drift
- crates/jj-store-wal/tests/native_copy.rs#native_copy_restores_code_agent_review_and_installs_manifest_last
- crates/jj-store-cli/tests/agent_kingfisher.rs#kingfisher_redacts_secrets_and_normalizes_workspace_paths
- crates/jj-store-cli/tests/agent_hook.rs#hook_ingestion_survives_restart_orders_sequences_and_never_persists_raw_input
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo check -p jj-store-wal --locked
```

**Notes**

The first native-copy compile reached the new boundary and exposed incorrect use of the transaction-only committed API plus a move in an async match guard. The implementation now enumerates WAL references through the selected history catalog and compares existing destination bytes in the match body.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-wal --test native_copy native_copy_restores_code_agent_review_and_installs_manifest_last --locked -- --exact
```

**Notes**

The first exact regression compile used a nonexistent Completed session state; the fixture now uses the owned Ended state.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-wal --test native_copy native_copy_restores_code_agent_review_and_installs_manifest_last --locked -- --exact
```

**Notes**

The exact bounded regression copies one stored native operation plus nonempty AgentStore and ReviewStore closures, installs the manifest last, resolves a lost final acknowledgement by exact read-back, leaves no authority after a too-small recovery bound and cold-reopens all three roots.
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
JJ_STORE_QUALIFY_REAL_HARNESSES=1 DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test agent_integrations real_path_qualification_accepts_codex_and_opencode_and_rejects_cursor_drift --locked -- --exact
```

**Notes**

The real PATH probe accepts installed Codex 0.156.1 and OpenCode 1.18.30, records installed Cursor 2026.09.26-dd393fe as unsupported against 3.22.7, rejects its enablement and previews both supported hook shapes only in disposable state/workspace directories.
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
command -v codex; codex --version; command -v cursor-agent; cursor-agent --version; command -v opencode; opencode --version
```

**Notes**

The qualified executables are /opt/homebrew/bin/codex 0.156.1, /Users/stevejuma/.local/bin/cursor-agent 2026.09.26-dd393fe and /opt/homebrew/bin/opencode 1.18.30.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test agent_kingfisher --locked; DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test agent_hook --locked
```

**Notes**

All three Kingfisher and all three installed-hook security/recovery regressions pass. The cold medium-rules scan completed in 91.98 seconds and cache reuse in 3.48 seconds; raw secrets and workspace paths never enter durable capture state.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreCollaboration.stories.svelte
```

**Notes**

All six focused Chromium collaboration interactions and automatic accessibility checks pass. Visual tests remain skipped by user direction and stories remain visual-pending.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo clippy -p jj-store-wal --test native_copy --locked -- -D warnings
```

**Notes**

The first focused lint rejected the fixture constructor's composite return type. A named test-only alias now keeps the same contract without suppressing the lint.
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
pnpm spec:validate
```

**Notes**

All 89 canonical requirements validate after closing the seven collaboration rows.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first
```

**Notes**

Eight protected implementation files map to the required publication, CLI, collaboration and milestone chapters.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
```

**Notes**

The ledger checker rejected a Storybook file path as a named Rust or JavaScript test reference. The passing story command remains recorded as a run and the invalid test-list entry was removed.
<!-- /run -->

<!-- run -->
### Run 12

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

All 13 package dependency boundaries remain valid.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo clippy -p jj-store-wal --test native_copy --locked -- -D warnings; DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo clippy -p jj-store-cli --test agent_integrations --locked -- -D warnings; DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo fmt --all --check
```

**Notes**

The WAL copy implementation, its exact regression and the real-harness registry target are warning-free; the Rust workspace is formatted.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
wc -l crates/jj-store-wal/src/publication/native_copy.rs crates/jj-store-wal/src/catalog.rs crates/jj-store-wal/tests/native_copy.rs crates/jj-store-cli/tests/agent_integrations.rs
```

**Notes**

The new production copy module has 266 lines, the extended catalog has 289, the native-copy regression has 442 and the integration regression has 207; all remain under their governed budgets.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:validate; pnpm spec:first; pnpm check:progress; pnpm exec prettier --check spec/src/cli.md spec/src/collaboration.md spec/src/milestones.md spec/src/publication.md spec/src/verification.md
```

**Notes**

All 89 requirements, eight protected files across four canonical chapters and 61 progress tasks validate; every changed specification file is formatted.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pass
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

[]
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M9-033 details 2](details-02.md)
