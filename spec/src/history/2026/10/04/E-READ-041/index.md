---
schema: 1
id: E-READ-041
---

# E-READ-041

<!-- evidence-field:id markdown -->
## Id

E-READ-041
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M9.4
- M13.4
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-04
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change vtmlrlyl
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Root/window-bound numeric review activity reuse behind fresh native and local provider admission
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust1.98.1 (48a229cea); jj-lib0.45.1; stock JJ0.45.1; macOS ARM64.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Private process-local cache with at most128 fixed-size numeric entries, five-minute fixed expiry, idle LRU and active duplicate-load coalescing. Keys bind repository, native epoch/local profile, optional root digest, finite1..256 limit and exact ordered catalog/reference window including partial flag. No providers, snapshot envelopes, actors, paths, bodies or source objects retained. Public routes/schemas, native/WAL/companion/receipt/cursor formats and collaboration capacities unchanged.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-service/src/collaboration/review_activity/cache.rs
- crates/jj-store-service/src/collaboration/review_activity/cache_tests.rs
- crates/jj-store-service/src/collaboration/review_activity/tests.rs
- crates/jj-store-service/src/collaboration/review_activity.rs
- crates/jj-store-service/src/local_repository/browser_reviews.rs
- crates/jj-store-service/tests/local_review.rs
- crates/jj-store-service/tests/review_threads.rs
- docs/implementation-plan.md
- spec/src/api.md
- spec/src/architecture.md
- spec/src/collaboration.md
- spec/src/local-jj.md
- spec/src/milestones.md
- spec/src/mutations.md
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-COLLAB-004
- JST-COLLAB-007
- JST-LOCAL-004
- JST-API-001
- JST-API-004
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-service/src/collaboration/review_activity/tests.rs#activity_reuse_avoids_repeat_event_reads_for_the_same_admitted_root_window
- crates/jj-store-service/src/collaboration/review_activity/cache_tests.rs#reuse_isolated_by_root_limit_profile_epoch_and_exact_window
- crates/jj-store-service/src/collaboration/review_activity/cache_tests.rs#malformed_admission_is_rejected_even_after_warm_and_empty_reads
- crates/jj-store-service/src/collaboration/review_activity/cache_tests.rs#failed_derivations_are_not_retained_and_repaired_evidence_can_retry
- crates/jj-store-service/src/collaboration/review_activity/cache_tests.rs#retention_is_fixed_size_idle_lru_with_fixed_not_sliding_expiry
- crates/jj-store-service/src/collaboration/review_activity/cache_tests.rs#active_saturation_expiry_and_disabled_retention_fall_back_without_query_loss
- crates/jj-store-service/src/collaboration/review_activity/cache_tests.rs#concurrent_derivations_coalesce_and_canceled_initializer_releases_waiter
- crates/jj-store-service/tests/review_threads.rs#warm_review_activity_keeps_source_limits_access_and_complete_closure_admission
- crates/jj-store-service/tests/local_review.rs#warm_local_review_activity_returns_fresh_companion_and_keeps_identity_and_limits
- crates/jj-store-service/tests/local_review.rs#local_review_selected_inventory_must_contain_the_complete_root_closure
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
cargo test -p jj-store-service --lib --locked activity_reuse_avoids_repeat_event_reads -- --nocapture (before)
```

**Notes**

Pass-through scaffold: second request increases object reads5 to10; expected5. Exact1failure recorded in review-reuse-before-041.log.
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
cargo test -p jj-store-service --lib --locked activity_reuse_avoids_repeat_event_reads -- --nocapture (after)
```

**Notes**

1pass/16.20s compile. Second same-root/window read reuses facts with total5 object reads. Temporary unused ENTRY_BYTES warning subsequently removed. review-reuse-after-041.log.
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
cargo fmt -p jj-store-service (first new tests)
```

**Notes**

Test-fixture visibility replacement also touched a struct initializer. Corrected that one syntax error before final formatting.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 11
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-service --lib --locked collaboration::review_activity:: -- --nocapture (first new tests)
```

**Notes**

Test-only initializer syntax and ambiguous Error glob import produce11diagnostics. Explicit service Error import and initializer correction resolve them; no product assertions weakened. review-reuse-units-041.log.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 10
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-service --lib --locked collaboration::review_activity:: -- --nocapture (second new tests)
```

**Notes**

Initializer repaired; ambiguous Error test import still fails compilation. Resolved before final run. review-reuse-units-final-041.log.
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
cargo test -p jj-store-service --lib --locked collaboration::review_activity:: -- --nocapture (fixed new tests)
```

**Notes**

11pass/0.01s after6.09s compile. Exact reuse, malformed/empty input, root/limit/profile/epoch/window isolation, failure retry, fixed expiry/idle LRU/count/fixed-size cells, disabled/all-active fallback and true concurrent coalescing/canceled initializer retry. review-reuse-units-fixed-041.log.
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
cargo test -p jj-store-service --test review_threads --test local_review --locked warm_ -- --nocapture
```

**Notes**

2pass: native denial/tighter object cap/corrupt retained comment evidence and recovery; reopened local reuse/fresh unrelated companion pin/historical pin/foreign identity/tighter bytes. review-reuse-admission-041.log.
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
cargo test -p jj-store-service --test review_threads --test local_review --locked -- --nocapture
```

**Notes**

14pass:8local/7.14s plus6native/33.53s. Complete directly affected provider files, including warmed missing local closure rejection while exact historical selection still succeeds. Tests own disposable fixtures; source publication/actor boundaries retained. review-reuse-provider-regressions-041.log.
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
cargo test -p jj-store-service --lib --locked collaboration::review_activity:: -- --nocapture (final repository isolation)
```

**Notes**

11pass after final distinct-repository isolation proof, with no test warnings. review-reuse-units-admitted-041.log.
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
cargo check -p jj-store-service -p jj-store-http -p jj-store-rpc --locked
```

**Notes**

Affected service and direct transport packages check. No public contract change or generation needed. review-reuse-check-041.log.
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
pnpm check:boundaries
```

**Notes**

17package dependency boundaries pass. review-reuse-boundaries-041.log.
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
cargo fmt -p jj-store-service; cargo fmt -p jj-store-service -- --check
```

**Notes**

Final changed Rust formatting passes. review-reuse-rust-format-041.log.
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
pnpm exec prettier --write spec/src/api.md spec/src/architecture.md spec/src/collaboration.md spec/src/local-jj.md spec/src/mutations.md docs/implementation-plan.md
```

**Notes**

Changed canonical chapters and delivery plan formatted. review-reuse-format-041.log.
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
pnpm check:ui-ownership; pnpm check:progress; pnpm spec:first; pnpm spec:validate; pnpm exec prettier --write spec/src/milestones.md
```

**Notes**

Ownership1validator, progress138tasks, spec-first8protected files/6canonical chapters and specification5validators/23chapters/172requirements pass. Ledger formatted. review-reuse-ownership/progress/spec-first/spec/ledger-format-041.log. No milestone promotion or complete qualification claim.
<!-- /run -->
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-READ-041 details 2](details-02.md)
