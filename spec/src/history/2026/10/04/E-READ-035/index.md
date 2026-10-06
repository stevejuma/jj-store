---
schema: 1
id: E-READ-035
---

# E-READ-035

<!-- evidence-field:id markdown -->
## Id

E-READ-035
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

enclosing Jujutsu change lltppqpv
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Separate native missing-primary construction from namespace admission and install complete immutable generations against current slots
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust1.98.1; jj-lib0.45.1; macOS ARM64.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No authority, repository, storage or public wire changes. Reservation version1 bytes unchanged; its filesystem path is process-only bookkeeping. Primary staging shares the native/local full-allowance quota, independent reservation lock and actual worker lifetime. Private output verifies fixed default index type, operation linkage and segment hash before framing/decoder admission. Primary completion seals hashes outside the namespace where precise stamps are supported, reacquires with current budget/cancellation, converts only its own reservation to final bytes and selects a currently free slot.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/tests/native_lifecycle/graph_cache.rs
- crates/jj-store-cli/tests/native_lifecycle/graphs.rs
- crates/jj-store-jj/src/query/cache.rs
- crates/jj-store-jj/src/query/cache/constructed.rs
- crates/jj-store-jj/src/query/cache/reservation.rs
- crates/jj-store-jj/src/query/index.rs
- crates/jj-store-jj/src/query/mod.rs
- docs/implementation-plan.md
- spec/src/architecture.md
- spec/src/cli.md
- spec/src/jj-integration.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-JJ-003
- JST-JJ-005
- JST-CLI-003
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
cargo test -p jj-store-cli --test native_lifecycle --locked cold_primary_construction_releases_namespace_and_preserves_other_generations -- --nocapture (before)
```

**Notes**

Paused primary construction holds namespace and independent unprepared graph returns Busy. native-primary-lock-before-035.log.
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
cargo test -p jj-store-cli --test native_lifecycle --locked cold_primary_construction_releases_namespace_and_preserves_other_generations -- --nocapture (initial implementation)
```

**Notes**

Unavailable because the private verifier originally expected only segment/link output while primary initialization also owns its fixed default type file. Added explicit primary type validation; historical two-file validation stays strict. native-primary-lock-after-035.log.
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
cargo test -p jj-store-cli --test native_lifecycle --locked cold_primary_construction_releases_namespace_and_preserves_other_generations -- --nocapture (type corrected)
```

**Notes**

1pass/4.05s, including separate-pin installation and concurrent same-pin replacement without duplicate slots. native-primary-lock-after-035-final.log. Final sealed-generation and cancellation consumer checks follow separately.
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
cargo test -p jj-store-cli --test native_lifecycle --locked cold_primary_construction_releases_namespace_and_preserves_other_generations -- --nocapture (sealed generation)
```

**Notes**

1pass/1.73s. native-primary-lock-after-035-complete.log.
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
cargo test -p jj-store-cli --test native_lifecycle --locked cold_primary_construction_releases_namespace_and_preserves_other_generations -- --nocapture (final)
```

**Notes**

1pass/1.18s. Independent cold readers retain charged reservation, distinct pins survive completion, same-pin replacement retains one slot and manifest unchanged. native-primary-final-035.log.
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
cargo test -p jj-store-cli --test native_lifecycle --locked canceled_primary_installation_releases_its_reservation_while_namespace_is_busy -- --nocapture
```

**Notes**

1pass/0.82s. Foreign namespace lock remains held while canceled completed construction exits promptly, removes its private reservation, and permits retry. native-primary-cancel-035.log.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-cli --test native_lifecycle --locked graph_acceptance:: -- --nocapture (initial)
```

**Notes**

35pass/1fail/11.15s. Old synchronous-cancellation fixture expected namespace Busy. Corrected to require charged full reservation and held shared graph/content permit while independent work proceeds; release gate before assertions. One background gate panic followed that failed assertion. native-graph-consumers-035.log.
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
cargo test -p jj-store-jj --lib --locked query::cache:: -- --nocapture
```

**Notes**

7pass/0.20s. Shared reservations, stale recovery, hash/link/type verification, repository totals, nonhash warm reuse and unretained cleanup both before and after rename. native-cache-units-035.log.
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
cargo test -p jj-store-cli --test native_lifecycle --locked graph_query_cancellation_retains_worker_permit_until_exit_and_deadlines_bound_waits -- --nocapture
```

**Notes**

1pass/0.27s. Synchronous canceled worker retains shared graph/content capacity and charged reservation until exit; independent namespace read succeeds, then cleanup/retry and deadline rejection remain checked. native-primary-worker-cancel-035.log.
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
cargo test -p jj-store-cli --test native_lifecycle --locked graph_acceptance:: -- --nocapture (final)
```

**Notes**

36pass/13.60s. Complete directly affected graph/cache/preparation/predicate/decorator/current authorization/identity/cancellation family. native-graph-consumers-035-final.log.
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
cargo test -p jj-store-jj --lib --locked query::memory:: -- --nocapture
```

**Notes**

5pass/0.01s. Shared native/local frame admission, LRU and last-reader lifetime retained. native-memory-units-035.log.
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
cargo fmt -p jj-store-jj -p jj-store-cli -- --check; pnpm exec prettier --write spec/src/jj-integration.md spec/src/cli.md spec/src/architecture.md docs/implementation-plan.md spec/src/milestones.md; pnpm spec:first; pnpm spec:validate; pnpm check:progress; jj --no-pager diff --git plus added-line whitespace validation
```

**Notes**

Focused formatting,8protected files/4chapters,5validators/23chapters/172requirements,138task bookkeeping and JJ whitespace pass. native-rust-format-035-final.log/native-doc-format-035-final.log/native-spec-first-035-final.log/native-spec-035-final.log/native-progress-035-final.log/native-diff-035.patch. Root aggregates/full contracts/UI builds/scale fixtures/comprehensive qualification not run. Inherited oversized __eh_frame linker warning remains.
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
cargo test -p jj-store-cli --test local_browser --locked preparation_acceptance:: -- --nocapture
```

**Notes**

3pass/1intentionally ignored large-repository smoke/8.40s. Shared reservation/writer local consumers preserve source binding, cancellation, conflict and pinned upstream semantics. native-local-reservation-consumers-035.log. Large fixture stays deferred.
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
cargo check -p jj-store-jj -p jj-store-cli --lib --locked
```

**Notes**

Affected JJ/CLI libraries and direct transitive consumers compile/2m36s. native-packages-035.log. Public wire schemas unchanged; focused generation not required.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-READ-035 details 2](details-02.md)
