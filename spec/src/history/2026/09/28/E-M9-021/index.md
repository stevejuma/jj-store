---
schema: 1
id: E-M9-021
---

# E-M9-021

<!-- evidence-field:id markdown -->
## Id

E-M9-021
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
- crates/jj-store-cli/src/agent_hook.rs
- crates/jj-store-cli/src/agent_publication.rs
- crates/jj-store-cli/src/commands.rs
- crates/jj-store-cli/src/lib.rs
- crates/jj-store-cli/tests/agent_capture_publication.rs
- docs/cli/agent.md
- spec/src/architecture.md
- spec/src/cli.md
- spec/src/collaboration.md
- spec/src/milestones.md
- spec/src/publication.md
- spec/src/verification.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/agent_capture_publication.rs#captured_entries_publish_in_source_sequence_order
- crates/jj-store-cli/tests/agent_capture_publication.rs#unknown_publication_survives_restart_and_retries_exact_request
- crates/jj-store-cli/tests/agent_capture_publication.rs#mismatched_commit_never_completes_prepared_capture
- crates/jj-store-cli/tests/agent_capture_publication.rs#repository_mismatch_fails_before_any_publication
- crates/jj-store-cli/tests/agent_hook.rs#hook_ingestion_survives_restart_orders_sequences_and_never_persists_raw_input
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
cargo test -p jj-store-cli --test agent_capture_publication --locked
```

**Notes**

The test-first compile proved that the CLI had no AgentStore capture publication worker module.
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
cargo test -p jj-store-cli --test agent_capture_publication --locked
```

**Notes**

All four worker regressions pass: source order overrides batch filenames, unknown outcomes survive restart with the exact prepared request, mismatched receipts cannot complete an entry and repository drift blocks every network call.
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
cargo test -p jj-store-cli --test agent_hook hook_ingestion_survives_restart_orders_sequences_and_never_persists_raw_input --locked -- --exact
```

**Notes**

The existing real Kingfisher hook regression still passes for restart, concurrent sequence allocation, omissions and absence of raw secret input in durable state.
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

All 13 package dependency boundaries pass with recovery owned by the CLI and native transport retained behind its existing client.
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

The test-first specification gate required the changed CLI command boundary to update its mapped canonical CLI chapter.
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
cargo test -p jj-store-cli --test agent_capture_publication --locked; cargo test -p jj-store-rpc --test agent_publication --locked
```

**Notes**

The repository-qualified Apple linker, CMake and Boost environment passes all four CLI recovery cases and the native LocalTrusted prepare, commit, replay and unknown-outcome transport case.
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
cargo clippy -p jj-store-cli --lib --test agent_capture_publication --test agent_capture_spool --test agent_hook --locked -- -D warnings; cargo fmt --all -- --check
```

**Notes**

The changed CLI library and three directly affected regressions are warning-free and formatted. The recovery module has 206 physical lines and its regression has 257.
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
pnpm check:boundaries; pnpm spec:validate; pnpm spec:first; pnpm check:progress; pnpm exec prettier --check docs/cli/agent.md spec/src/architecture.md spec/src/cli.md spec/src/collaboration.md spec/src/milestones.md spec/src/publication.md spec/src/verification.md
```

**Notes**

All 13 package boundaries, 89 requirements, seven protected files across five canonical chapters and 61 progress tasks pass; every changed document is formatted.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Add drift-safe owned integration removal after ingesting setup is qualified
- Execute pinned real-harness capability and smoke tests in M9.6; Cursor remains
  unsupported on the current host
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; capture spool format v1; native RPC package jj_store.native.v1; AgentStore delta schema v1; manifest format 2.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Adds no serialized format. Separate SHA-256 domains derive 16-byte request and publication IDs from repository plus capture batch ID; the bounded receipt result carries a fixed version tag and exact batch ID. The worker persists the returned AgentDeltaRequest before exact publication and removes it only for a matching committed receipt.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


