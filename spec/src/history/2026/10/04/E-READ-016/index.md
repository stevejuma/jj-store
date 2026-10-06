---
schema: 1
id: E-READ-016
---

# E-READ-016

<!-- evidence-field:id markdown -->
## Id

E-READ-016
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

enclosing Jujutsu change rmmkypyx
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Shared native/local generation reservation accounting
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust1.98.1; jj-lib0.45.1; macOS ARM64.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Repository persistence unchanged. Disposable reservation descriptor version1 inside existing private cache namespace; lifetime file lock is process-owned. No new HTTP contract.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-jj/src/query/cache/reservation.rs
- crates/jj-store-jj/src/query/cache.rs
- spec/src/cli.md
- spec/src/jj-integration.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-JJ-003
- JST-JJ-005
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-jj/src/query/cache/reservation.rs#reservations_share_native_repository_total_and_lifetime_admission
- crates/jj-store-jj/src/query/cache/reservation.rs#stale_reserved_generations_are_discarded_before_reuse
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
Source inspection: query/cache.rs and local_browser/preparation.rs
```

**Notes**

Before: native namespace accounts only its slots; local construction relies on caller-provided reservation with no shared admission API. New locked reservation provides the composition boundary; service wiring remains next.
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

5passed. Repository and total exhaustion seen by native admission, short namespace lock, final-reader lifetime, stale staging removal, existing native quota/LRU/checksum tests.
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

24passed. Full graph module justified by shared cache accounting/recovery impact. Existing linker unwind-size advisory remains.
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
cargo check -p jj-store-jj -p jj-store-service -p jj-store-cli --all-targets --locked
```

**Notes**

Affected package and direct consumers checked.
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
pnpm exec prettier --write spec/src/jj-integration.md spec/src/cli.md; pnpm spec:first; pnpm spec:validate; pnpm check:boundaries; cargo fmt --all -- --check; pnpm check:progress
```

**Notes**

Canonical source mapping,167requirements,17package boundaries,format and ledger checks.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- API is not yet connected to local preparation/service task ownership; that
  integration must retain the reservation inside actual worker/prepared handles
  until all readers exit. Reservation conservatively holds full entry allowance
  including4096bytes descriptor overhead; filesystem/root ownership remains
  trusted server configuration.
- Lock lifetime and orphan cleanup exercised with independent handles and
  simulated owner exit. Independent executable crash/platform qualification
  remains pending. The namespace lease is released after admission; native
  preparation still uses its existing namespace-lock duration.
- No running service changed. Local history/task/HTTP/browser delivery, larger
  native limits, other endpoint/collaboration/card work and
  Linux/native2m/10krefs/deep-page/near-capacity/cold/real-RustFS acceptance
  remain pending. UI/full-contract/root aggregate lanes out of scope; inherited
  gates remain.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Commit shared reservation admission, then retain reservations inside local preparation and service task lifecycle.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


