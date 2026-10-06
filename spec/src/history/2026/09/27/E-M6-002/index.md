---
schema: 1
id: E-M6-002
---

# E-M6-002

<!-- evidence-field:id markdown -->
## Id

E-M6-002
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M6
- M6.2
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-27
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

local-tantivy
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- Cargo.lock
- Cargo.toml
- crates/jj-store-tantivy/Cargo.toml
- crates/jj-store-tantivy/src/disk.rs
- crates/jj-store-tantivy/src/document.rs
- crates/jj-store-tantivy/src/lib.rs
- crates/jj-store-tantivy/src/projection.rs
- crates/jj-store-tantivy/src/search.rs
- crates/jj-store-tantivy/tests/durability.rs
- crates/jj-store-tantivy/tests/support/mod.rs
- docs/evidence/m6-tantivy-generations.md
- docs/licenses/NOTICE.md
- docs/upstream-lock.md
- spec/src/adapters.md
- spec/src/architecture.md
- spec/src/compatibility.md
- spec/src/milestones.md
- spec/src/security.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-tantivy/tests/durability.rs#tantivy_recovers_committed_payload_without_repeating_effects
- crates/jj-store-tantivy/tests/durability.rs#tantivy_crashes_before_pointer_replay_and_after_reader_recover
- crates/jj-store-tantivy/tests/durability.rs#tantivy_generation_pagination_retains_old_files_and_checks_each_authorization
- crates/jj-store-tantivy/tests/durability.rs#tantivy_missing_corrupt_artifacts_and_config_changes_rebuild
- crates/jj-store-tantivy/tests/durability.rs#tantivy_limits_and_writer_owner_fail_without_advancing
- crates/jj-store-tantivy/tests/durability.rs#tantivy_scope_changes_remove_stale_content_and_replay_upserts
- crates/jj-store-tantivy/tests/durability.rs#tantivy_remote_acknowledgement_loss_recovers_real_index
- crates/jj-store-tantivy/tests/durability.rs#tantivy_retained_generation_budget_recovers_after_reader_release
- crates/jj-store-tantivy/tests/durability.rs#tantivy_restored_older_generation_replays_without_cursor_regression
- crates/jj-store-tantivy/tests/durability.rs#tantivy_rechecks_authorization_before_returning_query_results
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M6-002 details 2](details-02.md)
