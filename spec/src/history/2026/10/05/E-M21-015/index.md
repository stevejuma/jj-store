---
schema: 1
id: E-M21-015
---

# E-M21-015

<!-- evidence-field:id markdown -->
## Id

E-M21-015
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

xtvzomsr
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Real exact unpublished local reword preparation under foreground coordination
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; pinned JJ 0.45.1 Git SHA-1 profile; real stock JJ isolated source fixtures and actual integration transaction/rebase/candidate write; macOS ARM64.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No existing wire, capsule, retained-ref, journal, companion, native/WAL/manifest encodings changed. New owned private-provider Reword/PreparedCode reports carry exact source/prepared capsules, stable rewrite groups, ordered local bookmark terms and before/after checkout trees. Existing evidence admission remains bounded at 100,000 commit records/16 MiB before the qualified reword profile admits at most 4096 source records, 1024 local bookmarks and 65536 message bytes; preparation deadline 30 seconds. Affected signed or unresolved-tree inputs remain unsupported.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-jj/src/local_execution/mod.rs
- crates/jj-store-jj/src/local_execution/reword/report.rs
- crates/jj-store-jj/src/local_execution/reword.rs
- crates/jj-store-jj/tests/stack_reword_preparation/lifecycle.rs
- crates/jj-store-jj/tests/stack_reword_preparation.rs
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

Exact reword provider regression fails with three absent export/method diagnostics. The implementation then fixes one pinned WorkspaceName conversion compile diagnostic before obtaining any passing product result.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

One JJ integration boundary: foreground-owned blocking candidate worker, exact operation/view/workspace source, visible membership and complete descendant closure, real unpublished reword/rebase transaction, source/prepared evidence and invariant report. Candidate creation never publishes or updates checkout; cancellation keeps lock/worker ownership until exit. Direct tests compose the existing retainer with fixture keys; runtime intent/journal composition is not inferred.
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-jj/tests/stack_reword_preparation.rs#exact_reword_preparation_retains_change_tree_and_dirty_checkout_without_publication
- crates/jj-store-jj/tests/stack_reword_preparation.rs#invalid_reword_root_and_hidden_targets_create_no_published_state
- crates/jj-store-jj/tests/stack_reword_preparation.rs#ancestor_reword_reports_all_descendants_and_exact_bookmark_terms
- crates/jj-store-jj/tests/stack_reword_preparation.rs#actual_prepared_reword_is_compatible_with_reserved_ref_retention
- crates/jj-store-jj/tests/stack_reword_preparation/lifecycle.rs#cancellation_after_candidate_write_releases_lease_only_when_worker_exits
- crates/jj-store-jj/tests/stack_reword_preparation/lifecycle.rs#authorization_loss_after_candidate_write_exposes_no_preparation_or_publication
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M21-015 details 2](details-02.md)
- [E-M21-015 details 3](details-03.md)
