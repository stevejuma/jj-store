---
schema: 1
id: E-READ-028
---

# E-READ-028

<!-- evidence-field:id markdown -->
## Id

E-READ-028
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M4.1
- M13.4
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-04
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change pkvpnwnz
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Shared native/local resident LRU admission before allocation growth
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust1.98.1; jj-lib0.45.1; Node26.9.0; pnpm10.32.1; macOS ARM64.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Authoritative identities, JJ format6, wire/task DTOs, cursor formats and publication unchanged. The existing shared1GiB allocation pool now weakly registers completed immutable native/local handles and reclaims idle LRU owners before refusing new growth. Local evicted tasks reset; current reads keep exact-operation rebuild-required recovery.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-jj/src/query/memory.rs
- crates/jj-store-jj/src/query/memory/retained.rs
- crates/jj-store-jj/src/query/cache/resident.rs
- crates/jj-store-jj/src/local_browser/preparation.rs
- crates/jj-store-service/src/local_index_tasks.rs
- spec/src/jj-integration.md
- spec/src/local-jj.md
- spec/src/mutations.md
- spec/src/cli.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-JJ-003
- JST-JJ-005
- JST-LOCAL-004
- JST-MUT-001
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
cargo test -p jj-store-jj --lib --locked shared_pool_reclaims_idle_native_and_local_generations_before_growth -- --nocapture (before)
```

**Notes**

A deterministic100-unit admission fixture refuses50new units while two40-unit generations occupy cache ownership and one is idle. shared-eviction-before.log. Initial slot registration alone does not solve admission.
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
cargo test -p jj-store-jj --lib --locked query::memory -- --nocapture (after)
```

**Notes**

5pass. Exact regression, LRU access ordering, oversized refusal without cache loss, nonblocking busy-slot refusal, last-reader charge lifetime and index framing guards. shared-eviction-after.log.
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
cargo check -p jj-store-jj -p jj-store-service --lib --locked
```

**Notes**

Affected libraries pass/31.06s. package-check-028.log.
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
cargo test -p jj-store-cli --test native_lifecycle --locked graph_acceptance:: -- --nocapture
```

**Notes**

29pass/6.91s;175unrelated tests excluded. Prepared projection recovery, revocation, quota/corruption, two concurrent preparation/read scenarios and pinned historical resident scopes remain correct with evictable native holders. shared-eviction-native.log.
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
cargo test -p jj-store-cli --test local_browser --locked index_tasks:: -- --nocapture
```

**Notes**

Incorrect module filter selected0tests; compile-only observation, not verification evidence. Corrected exact module below. shared-eviction-local.log.
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
cargo test -p jj-store-cli --test local_browser --locked index_tasks_acceptance:: -- --nocapture
```

**Notes**

5pass/6.84s;24unrelated tests excluded. Exact historical task reuse/eviction/restart, cancellation/expiry, no-poll expiry and reader-held reservation lifetime pass with shared local holders. shared-eviction-local-after.log. Native and local CLI links retain inherited compact-unwind section-size warning.
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
cargo fmt -p jj-store-jj -p jj-store-service; cargo fmt -p jj-store-jj -p jj-store-service -- --check; pnpm exec prettier --write spec/src/jj-integration.md spec/src/local-jj.md spec/src/cli.md; pnpm exec prettier --check spec/src/jj-integration.md spec/src/local-jj.md spec/src/cli.md
```

**Notes**

Focused source/chapter formatting. format-rust-028.log/format-docs-028.log.
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
pnpm spec:first (initial)
```

**Notes**

Protected service-file map additionally requires mutations.md. Added explicit read-only eviction/accepted-command/receipt authority boundary without weakening the map. spec-first-028.log.
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
pnpm exec prettier --write spec/src/mutations.md; pnpm spec:first; pnpm spec:validate
```

**Notes**

5protected files/4chapters;5validators/23chapters/172requirements pass. format-mutations-028.log/spec-first-028-after.log/spec-028.log.
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
pnpm check:progress
```

**Notes**

138tasks; bookkeeping-only progress gate passes. progress-028.log. Historical evidence bytes preserved.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Deterministic small-pool admission and direct native/local consumers are
  delivery evidence, not concurrent large-generation/RSS qualification. The
  existing1GiB ceiling remains unchanged. Complete combined
  pressure/lifecycle/scale qualification stays deferred.
- Native immutable cache generations, additional-operation construction
  before-growth admission and shorter locking; prepared local
  operation/prefix/comparison and indexed filtering; deep/native RPC lookup;
  root-bound collaboration/search and lightweight visible repository-card
  loading remain delivery work.
- Comprehensive integration/contracts, cold browser, Linux/native million-commit
  fixtures, release/RSS, fault/race/real-RustFS, registry-only and Help review
  refresh remain the separate later qualification slice. Main/original dev
  services/Linux source untouched.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Continue high-priority performance delivery in the retained temporary workspace, including bounded visible repository loading and collaboration/page projections.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Reclaim idle native/local read generations before shared memory growth while preserving active ownership; no milestone or full-plan completion.
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-jj/src/query/memory/retained.rs#shared_pool_reclaims_idle_native_and_local_generations_before_growth
- crates/jj-store-jj/src/query/memory/retained.rs#shared_pool_eviction_follows_access_order_and_preserves_oversized_denials
- crates/jj-store-jj/src/query/memory/retained.rs#shared_pool_admission_does_not_wait_on_busy_cache_slots
- crates/jj-store-cli/tests/local_browser/index_tasks.rs#local_task_eviction_keeps_reservation_until_active_reader_exits
- crates/jj-store-cli/tests/native_lifecycle/graph_cache.rs#warm_graph_starts_while_another_operation_is_preparing
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


