---
schema: 1
id: E-READ-038
---

# E-READ-038

<!-- evidence-field:id markdown -->
## Id

E-READ-038
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

enclosing Jujutsu change qrrmynyq
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Release the native cache namespace during existing-disk index decoding and selected-view projection recovery
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust1.98.1 (48a229cea); jj-lib0.45.1; stock JJ0.45.1; macOS ARM64.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No native authority, storage/wire, owned index format6, receipt version1 or reservation version1 changes. Nonresident verified disk generations transfer by rename into privately locked sized reservations before loading native operations/views, allocating JJ decode and projection derivation. Charge actual admitted data plus bounded old/new receipt and descriptor overhead; remove the old receipt before replacement so no uncharged duplicate exists. Reuse verified file stamps where supported; fallback platforms retain full checks. Current-slot installation, native integrity/access, hard/inline limits, live allocation leases and cleanup remain unchanged.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/tests/native_lifecycle/graph_disk_recovery.rs
- crates/jj-store-cli/tests/native_lifecycle/graphs.rs
- crates/jj-store-jj/src/query/cache/recovery.rs
- crates/jj-store-jj/src/query/cache/reservation.rs
- crates/jj-store-jj/src/query/cache.rs
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
cargo test -p jj-store-cli --test native_lifecycle --locked existing_disk_projection_recovery_releases_namespace_and_preserves_generations -- --nocapture (before)
```

**Notes**

1fail/0.39s: pause an existing-disk recovery after executor loss; unrelated native preparation returns Busy because the first request holds the namespace. disk-recovery-before-038.log.
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
cargo test -p jj-store-cli --test native_lifecycle --locked existing_disk_projection_recovery_releases_namespace_and_preserves_generations -- --nocapture (after and duplicate-pin extension)
```

**Notes**

1pass/2.20s then1pass/1.83s. Graph and explicit-preparation recovery admit concurrent other-operation construction; same-operation installation collapses to one generation. Exact old data/files and manifest remain unchanged. Active private charge is smaller than full construction allowance. disk-recovery-after-038.log/disk-recovery-duplicates-038.log.
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
cargo test -p jj-store-cli --test native_lifecycle --locked disk_recovery_acceptance:: -- --nocapture (initial and final)
```

**Notes**

2pass/1.30s then2pass/1.64s. Cancellation exits while namespace installation is locked, removes its private reservation and returns the actual worker permit. Rebuilding still returns the original pinned graph. disk-recovery-targets-038.log/disk-recovery-targets-final-038.log.
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
cargo test -p jj-store-jj --lib --locked query::cache:: -- --nocapture (initial and final)
```

**Notes**

8pass/0.19s then8pass/0.20s. Sized admission persists exact charges across reopen, rejects tiny/oversized/overflow requests without growth, and releases only at owner exit. Existing checksum/frame, namespace quota, stale reservation, private/renamed cleanup and warm verification consumers pass. disk-recovery-cache-units-038.log/disk-recovery-cache-units-final-038.log.
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
cargo test -p jj-store-cli --test native_lifecycle --locked graph_acceptance::cache_acceptance:: -- --nocapture
```

**Notes**

16pass/4.63s directly affected cache family: corrupt links/checksums/receipts, quotas/slots, symlink and authoritative-object refusal, current permission/identity/epoch, private cold/history construction, warm/resident scopes and cleanup. disk-recovery-cache-consumers-038.log.
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
cargo test -p jj-store-cli --test native_lifecycle --locked graph_acceptance::projection_acceptance:: -- --nocapture
```

**Notes**

2pass/1.84s: executor-loss recovery still enforces inline threshold and explicit preparation; selected rows/5,000 unrelated labels retain visibility and actual returned-work budgets. This focused fixture is not large-scale qualification. disk-recovery-projection-consumers-038.log.
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
cargo test -p jj-store-cli --test native_lifecycle --locked graph_acceptance::index_acceptance:: -- --nocapture
```

**Notes**

6pass/1.16s preparation policy, worker cancellation, independent work/membership limits, disposable generation and original cursor snapshot consumers. Total unique focused functional targets in this slice:34pass/0fail. disk-recovery-preparation-consumers-038.log.
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
cargo check -p jj-store-jj -p jj-store-cli --lib --locked
```

**Notes**

Affected JJ/CLI packages and direct transitive consumers compile/20.31s. disk-recovery-check-038.log. Inherited oversized __eh_frame linker warning remains in CLI test builds; no strict-lint or release pass is claimed.
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
rustfmt --edition 2024 changed JJ/CLI files; cargo fmt -p jj-store-jj -p jj-store-cli -- --check; pnpm exec prettier --write spec/src/jj-integration.md spec/src/architecture.md spec/src/cli.md docs/implementation-plan.md
```

**Notes**

Targeted Rust/canonical formatting passes. disk-recovery-format-038.log/disk-recovery-doc-format-038.log/disk-recovery-doc-format-final-038.log.
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
pnpm spec:validate; pnpm check:boundaries; pnpm check:architecture
```

**Notes**

5validators/23chapters/172requirements,17package dependency boundaries and972module architecture pass. disk-recovery-spec-038.log/disk-recovery-boundaries-038.log/disk-recovery-architecture-038.log.
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
pnpm exec prettier --write spec/src/milestones.md; pnpm check:progress; pnpm spec:first; pnpm exec prettier --check spec/src/jj-integration.md spec/src/architecture.md spec/src/cli.md spec/src/milestones.md docs/implementation-plan.md; cargo fmt -p jj-store-jj -p jj-store-cli -- --check; jj --no-pager diff --git plus added-line whitespace validation
```

**Notes**

138-task evidence bookkeeping,7protected paths/4canonical chapters, targeted canonical/Rust formatting and added-line whitespace pass. disk-recovery-ledger-format-038.log/disk-recovery-progress-038.log/disk-recovery-spec-first-038.log/disk-recovery-doc-check-038.log/disk-recovery-format-final-038.log/disk-recovery-final-038.patch. Complete qualification remains deferred.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Lightweight code/review repository summaries remain high-priority delivery
  work; current card metadata still uses bounded full graph/review presentation
  pages.
- Prepared local operation prefix/comparison and indexed filtered references,
  indexed deep operation/predecessor/copy/search, paged native RPC and bounded
  local operation-difference/change-group windows remain delivery work.
- Complete combined integration/contracts, Linux/native/near-capacity fixtures,
  measured RSS/release latency, cold production browser,
  restart/revocation/publication/fault/real-RustFS, registry-only qualification
  and Help/source-review refresh remain explicitly deferred. UI/transport/schema
  lanes are out of scope for this private Rust-only delivery. No milestone
  promotion, visual baseline, main/Linux source or original dev-service change.
- Inherited strict-lint qualification gaps recorded in E-READ-037 remain
  deferred and have not been silently waived.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Continue lightweight code/review summaries and remaining indexed operation/reference/deep lookup/RPC performance delivery; qualify the combined implementation in its later slice.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Existing native disk-generation projection recovery and directly shared cache reservation/install/cleanup consumers; no repository publication or read authority change.
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/native_lifecycle/graph_disk_recovery.rs#existing_disk_projection_recovery_releases_namespace_and_preserves_generations
- crates/jj-store-cli/tests/native_lifecycle/graph_disk_recovery.rs#canceled_disk_projection_recovery_cleans_up_while_namespace_is_busy
- crates/jj-store-jj/src/query/cache/reservation.rs#sized_recovery_reservations_charge_actual_allowance_and_reject_unbounded_requests
- crates/jj-store-jj/src/query/cache/reservation.rs#reservations_share_native_repository_total_and_lifetime_admission
- crates/jj-store-jj/src/query/cache.rs#unretained_primary_cleanup_removes_private_and_renamed_generations
- crates/jj-store-cli/tests/native_lifecycle/graph_projection.rs#native_prepared_projection_is_recovered_explicitly_after_executor_restart
- crates/jj-store-cli/tests/native_lifecycle/graph_cache.rs#graph_cache_recovery_never_follows_symlinks_or_repairs_authoritative_objects
- crates/jj-store-cli/tests/native_lifecycle/index_preparation.rs#index_preparation_cancels_pending_work_and_retains_shared_slot_until_exit
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


