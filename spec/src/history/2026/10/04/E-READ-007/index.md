---
schema: 1
id: E-READ-007
---

# E-READ-007

<!-- evidence-field:id markdown -->
## Id

E-READ-007
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M4.1
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-04
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change ywpqrklo
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Reserved interactive capacity during index preparation
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; jj-lib 0.45.1; pnpm 10.32.1.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No persisted or HTTP DTO changes. Internal worker diagnostics report configured interactive capacity plus one preparation slot.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/tests/native_lifecycle/content_admission.rs
- crates/jj-store-cli/tests/native_lifecycle/index_preparation.rs
- crates/jj-store-cli/tests/native_lifecycle/index_tasks.rs
- crates/jj-store-cli/tests/native_lifecycle/query_workers.rs
- crates/jj-store-jj/src/query/mod.rs
- crates/jj-store-jj/src/query/workers.rs
- spec/src/api.md
- spec/src/cli.md
- spec/src/jj-integration.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-JJ-003
- JST-JJ-005
- JST-API-001
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/native_lifecycle/index_tasks.rs#index_preparation_reserves_interactive_capacity
- crates/jj-store-cli/tests/native_lifecycle/index_preparation.rs#index_preparation_cancels_pending_work_and_retains_shared_slot_until_exit
- crates/jj-store-cli/tests/native_lifecycle/query_workers.rs#query_workers_mixed_capacity_clones_and_errors_release_slots
- crates/jj-store-cli/tests/native_lifecycle/content_admission.rs#query_shape_rejects_commit_merge_amplification_without_narrowing_persistence
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
cargo test -p jj-store-cli --test native_lifecycle --locked index_preparation_reserves_interactive_capacity -- --nocapture
```

**Notes**

Before: blocked preparation starved interactive file reader. Removed redundant drop-of-Copy warning in test.
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
cargo test -p jj-store-cli --test native_lifecycle --locked index_preparation_reserves_interactive_capacity -- --nocapture
```

**Notes**

1 passed: capacity-one interactive file and operation reads succeed during gated preparation; second namespace returns Busy; canceled worker releases capacity.
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
cargo check -p jj-store-jj -p jj-store-service -p jj-store-cli --all-targets --locked
```

**Notes**

Affected packages and direct consumers checked.
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
pnpm spec:first
```

**Notes**

Missing mapped CLI chapter corrected with explicit serving-composition acceptance.
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
pnpm check:architecture; pnpm exec prettier --check spec/src/api.md spec/src/jj-integration.md spec/src/cli.md; pnpm spec:first; cargo fmt --all -- --check
```

**Notes**

930 production UI modules; focused docs and Rust formatting; spec-first 5 protected files/3 chapters.
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
cargo test -p jj-store-cli --test native_lifecycle --locked -- --test-threads=2
```

**Notes**

Escalated to whole native lifecycle binary because shared scheduling spans graphs/content/commands. 190 passed/2 failed in83.50s. Both failures used operation-only reads to trigger commit/view amplification checks; updated to indexed all() queries and preserved Limit assertions. Existing linker unwind-size advisory retained.
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
cargo test -p jj-store-cli --test native_lifecycle --locked content_acceptance::admission_acceptance:: -- --test-threads=2
```

**Notes**

All4 admission tests pass after correcting the two request shapes. No production admission weakened. Full binary not repeated after test-only corrections; this is not reported as a clean full-suite run. Final check:progress, spec:first, spec:validate and cargo fmt checks pass (136 tasks, 6 protected files/4 chapters, 167 requirements).
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Shared cache namespace locking and existing index size/rebuild algorithms
  remain unchanged. Full bounded disk preparation, local historical point/graph
  lookup, immutable segments/quotas, operation prefix/copy/search/RPC and
  collaboration projections remain pending.
- Linux/native2m/near-capacity/cold/real-RustFS qualification and E-READ-005
  release/contract gaps remain pending. Running dev app unchanged. Unrelated
  root aggregate and unchanged UI/browser lanes out of scope.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Commit reserved-worker slice after ledger checks, then continue historical reads and bounded preparation.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


