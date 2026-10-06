---
schema: 1
id: E-READ-009
---

# E-READ-009

<!-- evidence-field:id markdown -->
## Id

E-READ-009
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

enclosing Jujutsu change vtookwlp
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Native graph admission without complete catalog enumeration
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; jj-lib0.45.1; pnpm10.32.1.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Public DTOs, native objects, manifest/WAL and existing disposable cache format unchanged. Hard index-size default remains50000; inline default remains1000.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/tests/native_lifecycle/graphs.rs
- crates/jj-store-cli/tests/native_lifecycle/index_tasks.rs
- crates/jj-store-jj/src/query/access.rs
- crates/jj-store-jj/src/query/graphs.rs
- crates/jj-store-jj/src/query/index.rs
- crates/jj-store-jj/src/query/mod.rs
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

- crates/jj-store-cli/tests/native_lifecycle/graphs.rs#native_graph_pages_and_preparation_do_not_enumerate_inventory
- crates/jj-store-cli/tests/native_lifecycle/graphs.rs#graph_query_rebuild_limits_staged_rejection_and_current_revocation
- crates/jj-store-cli/tests/native_lifecycle/index_preparation.rs#index_preparation_is_explicit_snapshot_bound_and_disposable
- crates/jj-store-cli/tests/native_lifecycle/index_tasks.rs#index_tasks_retention_expiry_and_failures_are_explicit
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
cargo test -p jj-store-cli --test native_lifecycle --locked native_graph_pages_and_preparation_do_not_enumerate_inventory -- --nocapture
```

**Notes**

Before: preparation failed IndexBudget when capability allowed checked point membership but rejected inventory enumeration.
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
cargo test -p jj-store-cli --test native_lifecycle --locked native_graph_pages_and_preparation_do_not_enumerate_inventory -- --nocapture
```

**Notes**

1 passed. Strengthened final fixture also exercises inline threshold1 before preparation and warm reuse with complete inventory forbidden.
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
cargo test -p jj-store-cli --test native_lifecycle --locked graph_acceptance:: -- --test-threads=2
```

**Notes**

19 passed: native graph parity, cold/warm hard limits, cache recovery/corruption/eviction, resource cancellation and original-operation cursor rebuild.
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
target/debug/deps/native_lifecycle-e6652c34f7ed2d93 content_acceptance:: --test-threads=2
```

**Notes**

95 passed/1 failed. Task retention fixture started a second cache writer without awaiting completion, then expected the separate limited executor to reach IndexBudget. Actual admission now requires cache access, so Busy may legitimately precede index admission. Updated fixture to await Ready before asserting hard limit; retained IndexBudget assertion.
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
cargo test -p jj-store-cli --test native_lifecycle --locked content_acceptance::index_task_acceptance:: -- --test-threads=2
```

**Notes**

Final5 task tests pass, including hard-limit status, revocation, expiry, cancellation, coalescing and reserved interactive capacity. No production limit weakened to repair fixture timing.
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
target/debug/deps/native_lifecycle-e6652c34f7ed2d93 pagination_acceptance:: --test-threads=2
```

**Notes**

4 passed; exact snapshot/cursor identity, later publication, epoch and revocation behavior retained.
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
cargo check -p jj-store-jj -p jj-store-service -p jj-store-cli --all-targets --locked
```

**Notes**

Affected targets/direct consumers pass. Existing linker unwind-size advisory retained in test builds.
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
pnpm spec:first; cargo fmt --all -- --check; pnpm exec prettier --check spec/src/api.md spec/src/cli.md spec/src/jj-integration.md
```

**Notes**

Final check:progress, spec:first, spec:validate and cargo fmt checks pass:136 tasks,6 protected files/4 chapters,167 requirements. Contracts unchanged; no regeneration required.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Upstream cold builder still materializes commit history and hard limit
  remains50000. Bounded disk-backed preparation, larger qualified indexes, local
  graph preparation, immutable segment reuse,4GiB/repository and16GiB aggregate
  staging-inclusive quotas remain pending.
- File-predicate preflight, copy and operation-prefix lookup still enumerate
  inventory; uninstrumented native synchronous CPU/memory remains unqualified.
  Search/RPC and collaboration/card projections,
  native2m/near-capacity/cold/real-RustFS gates and E-READ-005
  release/full-capture gaps remain pending. Running app unchanged; unrelated
  root aggregates and unchanged UI/browser lanes out of scope.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Validate ledger, commit native admission slice, then implement bounded disk-backed preparation and immutable cache generations.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


