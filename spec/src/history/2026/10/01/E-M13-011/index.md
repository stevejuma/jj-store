---
schema: 1
id: E-M13-011
---

# E-M13-011

<!-- evidence-field:id markdown -->
## Id

E-M13-011
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M13
- M13.3
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-01
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

filesystem-git
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

qtxvrqqm
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-service/src/local_repository/errors.rs
- crates/jj-store-service/src/local_repository/review_commands.rs
- crates/jj-store-service/src/local_repository/review_io.rs
- crates/jj-store-service/src/local_repository/review_store.rs
- crates/jj-store-service/src/local_repository/review_types.rs
- crates/jj-store-service/src/local_repository.rs
- crates/jj-store-service/tests/local_review.rs
- spec/src/collaboration.md
- spec/src/local-jj.md
- spec/src/mutations.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-service/tests/local_review.rs#local_review_reads_leave_metadata_and_actor_absent
- crates/jj-store-service/tests/local_review.rs#local_review_preserves_domains_replays_after_gc_and_pins_old_threads
- crates/jj-store-service/tests/local_review.rs#local_review_unknown_outcome_retries_exact_request_after_restart
- crates/jj-store-service/tests/local_review.rs#local_review_rejects_foreign_and_unqualified_anchors_without_publication
- crates/jj-store-service/tests/local_review.rs#local_review_selected_inventory_must_contain_the_complete_root_closure
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M13-011 details 2](details-02.md)
