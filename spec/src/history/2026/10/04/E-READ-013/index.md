---
schema: 1
id: E-READ-013
---

# E-READ-013

<!-- evidence-field:id markdown -->
## Id

E-READ-013
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

enclosing Jujutsu change mvlskuys
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Repository-scoped preparation cache quotas
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust1.98.1; jj-lib0.45.1; pnpm10.32.1; macOS ARM64.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Native formats, cache receipt and public DTOs unchanged; trusted cache policy adds repository byte ceiling. Defaults2GiB/entry,4GiB/repository and16GiB/shared namespace.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-jj/src/query/cache.rs
- spec/src/jj-integration.md
- spec/src/cli.md
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

- crates/jj-store-jj/src/query/cache.rs#default_cache_allows_large_preparation_with_a_bounded_total
- crates/jj-store-jj/src/query/cache.rs#repository_reservation_counts_all_epochs_and_preserves_other_repositories
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
cargo test -p jj-store-jj --lib --locked default_cache_allows_large_preparation_with_a_bounded_total -- --nocapture
```

**Notes**

Before:64MiB entry allowance rejected large preparation capacity expectation. Final entry allowance2GiB leaves headroom for multiple operation caches within4GiB repository allowance; aggregate16GiB.
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
cargo test -p jj-store-jj --lib --locked query::cache:: -- --nocapture
```

**Notes**

Initial and final3passed. Multiple repository/epoch receipts, physical receipt bytes, cold reservation, repository-first eviction, reduced-policy reopen and warm corruption detection.
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

24passed; cache retention/recovery, quotas, exact native index bytes, graph semantics, concurrent preparation, cancellation and revocation. Existing linker unwind-size advisory remains.
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
target/debug/deps/native_lifecycle-d9faf2e6f13fc0f1 content_acceptance::index_task_acceptance:: --test-threads=2
```

**Notes**

5passed: task lifecycle and reserved interactive capacity.
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
cargo check -p jj-store-jj -p jj-store-service -p jj-store-cli --all-targets --locked
```

**Notes**

Affected package and direct consumer targets pass.
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
pnpm exec prettier --write spec/src/jj-integration.md spec/src/cli.md; pnpm spec:first; pnpm spec:validate; cargo fmt --all -- --check; pnpm check:progress
```

**Notes**

Changed canonical chapter formatting, protected-source mapping, canonical validation and appended ledger consumers pass.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Explicit native preparation reserves staging, scratch, buffered output and
  receipt within one entry before growth. Per-repository accounting includes all
  epochs/operations in the shared serving namespace. Independent standalone
  roots remain independently configured.
- General inline upstream writes and additional-operation builds still use
  post-build size verification. Immutable generations and shared cross-profile
  admission remain pending; increased disk ceilings do not lift current
  commit,work,time or resident-memory limits.
- Larger native/local preparation, remaining endpoint/card/collaboration work
  and Linux/native2m/10krefs/near-capacity/real-RustFS qualification remain
  open. Running app unchanged. Unchanged UI,full contracts and root aggregates
  out of scope; E-READ-005 registry/full-capture limitations retained.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Commit the verified quota slice; continue shared bounded traversal and preparation for local history.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


