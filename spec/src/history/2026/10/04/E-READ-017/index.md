---
schema: 1
id: E-READ-017
---

# E-READ-017

<!-- evidence-field:id markdown -->
## Id

E-READ-017
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M4.1
- M13.5
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-04
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change wpnsnxtz
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Local prepared-history task lifecycle and retained-reader quota ownership
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust1.98.1; jj-lib0.45.1; pnpm10.32.1; macOS ARM64.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Repository persistence and wire routes unchanged. New service-local LocalIndexTask/State DTOs are not yet exposed by HTTP. Reserved Prepared handle owns the shared cache reservation.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/tests/local_browser/index_tasks.rs
- crates/jj-store-cli/tests/local_browser.rs
- crates/jj-store-jj/src/local_browser/mod.rs
- crates/jj-store-jj/src/local_browser/preparation.rs
- crates/jj-store-jj/src/query/mod.rs
- crates/jj-store-service/src/lib.rs
- crates/jj-store-service/src/local_index_tasks.rs
- spec/src/api.md
- spec/src/cli.md
- spec/src/jj-integration.md
- spec/src/local-jj.md
- spec/src/mutations.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-JJ-003
- JST-JJ-005
- JST-LOCAL-004
- JST-API-004
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/local_browser/index_tasks.rs#local_tasks_retain_exact_history_evict_under_quota_and_reset_after_restart
- crates/jj-store-cli/tests/local_browser/index_tasks.rs#local_task_cancellation_and_expiry_release_preparation_without_blocking_reads
- crates/jj-store-cli/tests/local_browser/index_tasks.rs#local_task_ready_expiry_releases_cache_without_polling
- crates/jj-store-cli/tests/local_browser/index_tasks.rs#local_task_eviction_keeps_reservation_until_active_reader_exits
- crates/jj-store-cli/tests/local_browser/index_tasks.rs#local_interactive_authorization_waits_obey_initial_and_final_deadlines
<!-- /evidence-field -->

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
Source inspection: local_browser::Executor::worker/prepare_index and service index task composition
```

**Notes**

Before: no local prepared task owner, and local initial/final authorization waits sat outside request timeout/cancellation. New service binds source generation/path/operation and retains shared reservation in the actual Prepared handle.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
cargo check -p jj-store-service --all-targets --locked
```

**Notes**

Initial2missing is_cancelled method errors. Added cancellation observation/await API and fixed stats ownership before final compilation.
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
cargo check -p jj-store-service --all-targets --locked
```

**Notes**

Passed after initial API corrections.
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
cargo test -p jj-store-cli --test local_browser --locked index_tasks_acceptance:: -- --nocapture
```

**Notes**

Initial3passed, expanded4passed. Exact pin after newer heads, coalescing, low-quota eviction, current denial/generation, restart reset, cancellation/expiry while initial preparation authorization waits, available interactive reads, automatic ready expiry, and evicted active-reader quota until canceled worker exits. Logs local-index-tasks-regression[-final].log.
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
cargo test -p jj-store-cli --test local_browser --locked -- --test-threads=2
```

**Notes**

19passed/1ignored. Full local suite justified by shared worker authorization/deadline behavior; external large-repository probe not run. Subsequently added exact deadline test is recorded separately.
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
cargo test -p jj-store-cli --test local_browser --locked local_interactive_authorization_waits_obey_initial_and_final_deadlines -- --nocapture
```

**Notes**

1passed. Both initial and final blocked authorization expire within100ms request budget and release interactive worker capacity.
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

Affected package/direct consumers checked; existing test linker unwind-size advisory remains.
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
pnpm exec prettier --write spec/src/local-jj.md spec/src/jj-integration.md spec/src/cli.md spec/src/api.md spec/src/mutations.md; pnpm spec:first; pnpm spec:validate; pnpm check:boundaries; cargo fmt --all -- --check; pnpm check:progress
```

**Notes**

Protected source/chapter mapping,167requirements,17package boundaries,format and ledger checks.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Service facade is not yet wired into HTTP/local browser composition. Current
  running app and public local capability remain unchanged. HTTP cold-log
  routing, exact preparation/start/status/cancel contracts and client state
  delivery are next.
- Default task construction deadline1hour, completed TTL5minutes, retained32.
  Background supervisors release on eviction/teardown; no credentials remain in
  completed entries. Ready generation eviction is non-authoritative; a new exact
  preparation may be required. Active readers keep reservation and resident
  until their actual worker exits.
- Native large limits, remaining implicit-import paths, prepared
  historical/prefix/copy/search/RPC and collaboration/card work remain open.
  Linux/native2m/10krefs/deep-page/near-capacity/cold/real-RustFS qualification
  pending. Unchanged UI/native graph/full-contract/root aggregates out of scope;
  E-READ-005 registry/full-contract limits retained.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Commit task lifecycle, then expose local preparation and prepared log reads in HTTP and the profile-aware browser controller.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


