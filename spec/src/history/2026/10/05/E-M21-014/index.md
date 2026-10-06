---
schema: 1
id: E-M21-014
---

# E-M21-014

<!-- evidence-field:id markdown -->
## Id

E-M21-014
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M21
- M21.2
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-05
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

nzpmmsoo
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Nonblocking foreground JJ working-copy coordination
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; pinned JJ 0.45.1 Git SHA-1 profile; real /opt/homebrew/bin/jj CLI contention and actual subprocess exit in disposable local fixtures; macOS ARM64.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No wire, native, capsule, companion, journal, WAL or manifest encoding changes. Uses the existing working_copy.lock Unix flock protocol with descriptor-relative no-follow open and single-link identity checks; retained empty lock file is compatible with upstream process-loss recovery. One shared preparation permit remains occupied for the lease lifetime.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-jj/src/lib.rs
- crates/jj-store-jj/src/local_browser/working_tree/current/physical.rs
- crates/jj-store-jj/src/local_browser/working_tree/current.rs
- crates/jj-store-jj/src/local_browser/working_tree.rs
- crates/jj-store-jj/src/local_execution/guard.rs
- crates/jj-store-jj/src/local_execution/mod.rs
- crates/jj-store-jj/tests/stack_foreground/fixture.rs
- crates/jj-store-jj/tests/stack_foreground/lifecycle.rs
- crates/jj-store-jj/tests/stack_foreground.rs
- docs/implementation-plan.md
- docs/maintenance/coverage.json
- docs/site/technical/local.md
- spec/src/architecture.md
- spec/src/documentation.md
- spec/src/jj-integration.md
- spec/src/local-jj.md
- spec/src/verification.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Exact foreground regression cannot import the absent local_execution coordinator. Stock JJ used as the actual foreground-lock consumer.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

One owning JJ integration primitive. Exact lease acquisition/revalidation, busy admission, cancellation, real CLI contention/process loss and unsafe/replaced metadata or lock identity. Existing bounded read methods are reused unchanged; no operation preparation, publication or checkout method is exposed.
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-jj/tests/stack_foreground.rs#foreground_lease_matches_stock_jj_and_preserves_dirty_checkout
- crates/jj-store-jj/tests/stack_foreground.rs#competing_foreground_lease_is_busy_and_release_restores_admission
- crates/jj-store-jj/tests/stack_foreground.rs#stale_selection_and_replaced_lock_fail_without_writing_checkout
- crates/jj-store-jj/tests/stack_foreground.rs#unsafe_lock_entries_are_rejected_without_touching_their_targets
- crates/jj-store-jj/tests/stack_foreground.rs#missing_checkout_state_is_not_initialized_and_editor_bytes_are_separate
- crates/jj-store-jj/tests/stack_foreground/lifecycle.rs#external_heads_and_replaced_working_copy_directory_invalidate_the_lease
- crates/jj-store-jj/tests/stack_foreground/lifecycle.rs#cancellation_after_lock_releases_foreground_admission_and_preserves_state
- crates/jj-store-jj/tests/stack_foreground/lifecycle.rs#foreground_process_loss_releases_lock_without_changing_checkout
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M21-014 details 2](details-02.md)
- [E-M21-014 details 3](details-03.md)
