---
schema: 1
id: E-M14-004
---

# E-M14-004

<!-- evidence-field:id markdown -->
## Id

E-M14-004
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M14
- M14.3
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-01
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

implementation
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/src/agent_capture.rs
- crates/jj-store-cli/src/agent_capture_files.rs
- crates/jj-store-cli/src/agent_capture_intent.rs
- crates/jj-store-cli/src/agent_publication.rs
- crates/jj-store-cli/src/local_agent_publication.rs
- crates/jj-store-cli/tests/agent_capture_publication.rs
- crates/jj-store-cli/tests/agent_capture_spool.rs
- crates/jj-store-cli/tests/local_agent_recovery.rs
- spec/src/cli.md
- spec/src/collaboration.md
- spec/src/compatibility.md
- spec/src/local-jj.md
- spec/src/milestones.md
- spec/src/verification.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/agent_capture_spool.rs#sanitized_capture_and_prepared_publication_survive_restart
- crates/jj-store-cli/tests/agent_capture_spool.rs#capture_spool_rejects_concurrent_owners_and_changed_retries
- crates/jj-store-cli/tests/agent_capture_spool.rs#capture_spool_enforces_byte_bounds_and_rejects_corruption
- crates/jj-store-cli/tests/agent_capture_spool.rs#capture_lock_protocol_excludes_legacy_writers_in_both_directions
- crates/jj-store-cli/tests/agent_capture_spool.rs#concurrent_handles_preserve_queue_byte_budget_and_prepared_identity
- crates/jj-store-cli/tests/agent_capture_spool.rs#enqueue_enforces_entry_limit_before_acknowledging
- crates/jj-store-cli/tests/agent_capture_spool.rs#capture_lock_and_pending_symlinks_fail_closed
- crates/jj-store-cli/tests/agent_capture_publication.rs#captured_entries_publish_in_source_sequence_order
- crates/jj-store-cli/tests/agent_capture_publication.rs#unknown_publication_survives_restart_and_retries_exact_request
- crates/jj-store-cli/tests/agent_capture_publication.rs#mismatched_commit_never_completes_prepared_capture
- crates/jj-store-cli/tests/agent_capture_publication.rs#repository_mismatch_fails_before_any_publication
- crates/jj-store-cli/tests/agent_capture_publication.rs#publication_wait_allows_capture_from_another_process
- crates/jj-store-cli/tests/agent_capture_publication.rs#killed_drainer_preserves_prepared_capture_and_releases_lease
- crates/jj-store-cli/tests/local_agent_recovery.rs#unknown_local_capture_restarts_with_exact_intent_before_later_batches
- crates/jj-store-cli/tests/local_agent_recovery.rs#mismatched_receipts_and_repository_binding_never_complete_local_captures
- crates/jj-store-cli/tests/local_agent_recovery.rs#local_publication_releases_enqueue_lock_and_cancellation_preserves_intent
- crates/jj-store-cli/tests/local_agent.rs#cold_hook_sanitizes_and_publishes_to_local_git_without_native_connection
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test --locked -p jj-store-cli --test agent_capture_publication publication_wait_allows_capture_from_another_process -- --exact
```

**Notes**

Before implementation: exact regression failed because a fresh enqueue child could not open the spool while native preparation waited; the lifetime capture lock was held.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test --locked -p jj-store-cli --test agent_capture_publication publication_wait_allows_capture_from_another_process -- --exact
```

**Notes**

After split-lock implementation: exact regression passed; two fresh child processes enqueue during preparation and publication waits.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test --locked -p jj-store-cli --test agent_capture_spool --test agent_capture_publication --test local_agent_recovery
```

**Notes**

Native five passed; spool six passed and count-budget fixture failed because 4096 valid entries exceeded its unrelated 4 MiB fixture budget. Set this count-specific fixture to the production 64 MiB budget; no production byte limit relaxed. Local recovery binary had not run after Cargo stopped.
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
cargo fmt -p jj-store-cli
```

**Notes**

Formatted changed CLI implementation/tests.
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
pnpm spec:first
```

**Notes**

Eight protected files and three canonical chapters accepted before final evidence additions.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test --locked -p jj-store-cli --test agent_capture_spool --test agent_capture_publication --test local_agent_recovery --test agent_hook
```

**Notes**

19 passed: six native publication, seven spool, three hook and three local recovery. Two helper tests are default-ignored but explicitly run in fresh subprocesses by passing parent regressions. Includes preparation/publication concurrency, abrupt process termination, cancelled local/native workers, exact-intent recovery, admitted prepared principal, 4096-entry/byte limits, symlinks and legacy lock compatibility. The unchanged Kingfisher hook fixture compiled fresh rules and passed in 91.38 seconds.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo clippy --locked -p jj-store-cli --lib --test agent_capture_spool --test agent_capture_publication --test local_agent_recovery --test agent_hook --test local_agent -- -D warnings
```

**Notes**

Focused CLI library and direct-consumer test lint/check passed with no warnings.
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
cargo fmt -p jj-store-cli -- --check
```

**Notes**

Changed Rust package formatting passed.
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
pnpm exec prettier --write spec/src/cli.md spec/src/collaboration.md spec/src/local-jj.md spec/src/compatibility.md spec/src/verification.md spec/src/milestones.md
```

**Notes**

Changed canonical chapters and evidence formatted. The same file list also passed pnpm exec prettier --check; final progress/spec-first/spec validation passed after cold CLI results were recorded.
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
pnpm spec:validate; pnpm spec:first; pnpm check:progress
```

**Notes**

132 requirements validated; eight protected files and five canonical chapters accepted; progress accepts 111 tasks.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test --locked -p jj-store-cli --test local_agent cold_hook_sanitizes_and_publishes_to_local_git_without_native_connection -- --exact
```

**Notes**

One cold end-to-end CLI test passed in 94.82 seconds after compilation, one unrelated case filtered. Fresh disposable Git-backed workspace, explicit development binary, qualified fixture executable, setup and warmed Kingfisher cache, sanitized hook, native-RPC-free local publication and drain. No installed harness/service or existing repository modified.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
jj --no-pager diff --check
```

**Notes**

Tooling-only invocation error: JJ 0.45.1 does not support Git-style --check. No diff check executed; package rustfmt and changed-document Prettier checks provide formatting validation.
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

<!-- evidence-field:scope markdown -->
## Scope

Single Rust CLI capture-coordination behavior and its direct native/local/hook consumers. Subprocess crash fixtures use fake publication replies and do not claim new RustFS qualification. Public wire schemas and UI unchanged; full Rust/workspace/browser/root aggregate and release lanes out of scope.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

V1 PendingCaptureV1 bytes and stable request/publication identities unchanged. Device-local capture.lock becomes shared for current handles while excluding legacy exclusive owners; new mutation.lock and drain.lock protect bounded I/O and single publication ownership. Enclosing Jujutsu change is the revision reference.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; JJ/jj-lib 0.45.1; macOS ARM64; existing locked Kingfisher 2.7.0 and sanitizer profile. All cargo verification uses CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target; source edits stay in jj-store-local-settings.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Add typed agent inspection/setup previews and server-owned bounded recovery for verified folder links, retaining admitted-principal binding and qualified executable ownership.
<!-- /evidence-field -->

<!-- evidence-field:deferred list -->
## Deferred

- "M14.3 remains In progress: no service-owned automatic recovery worker or
  agent enable/setup/remove Settings UI yet."
- Editable Search, Storage, Sync and integrated live Settings qualification
  remain M14.4-M14.7.
<!-- /evidence-field -->

<!-- evidence-field:artifacts markdown -->
## Artifacts

Ignored .scratch/drain-before.log, drain-after.log, drain-focused.log and drain-focused-final.log; all test subprocess state is disposable and owned child processes are cleaned up. Final package/cold CLI logs: .scratch/drain-clippy.log and .scratch/drain-cold-cli.log.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


