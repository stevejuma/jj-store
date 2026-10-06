---
schema: 1
id: E-M5-001
---

# E-M5-001

<!-- evidence-field:id markdown -->
## Id

E-M5-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M5
- M5.1
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-27
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

native-command-preparation
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-jj/src/commands/mod.rs
- crates/jj-store-jj/src/commands/access.rs
- crates/jj-store-jj/src/commands/execute.rs
- crates/jj-store-jj/src/lib.rs
- crates/jj-store-jj/src/query/access.rs
- crates/jj-store-jj/src/query/mod.rs
- crates/jj-store-jj/src/query/workers.rs
- crates/jj-store-service/src/commands.rs
- crates/jj-store-service/src/lib.rs
- crates/jj-store-cli/tests/native_lifecycle.rs
- crates/jj-store-cli/tests/native_lifecycle/commands.rs
- docs/evidence/m5-command-preparation.md
- spec/src/mutations.md
- spec/src/jj-integration.md
- spec/src/cli.md
- spec/src/compatibility.md
- spec/src/verification.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/native_lifecycle/commands.rs#typed_describe_prepares_native_descendants_without_publication
- crates/jj-store-cli/tests/native_lifecycle/commands.rs#typed_new_merges_ordered_parents_and_moves_only_logical_workspace
- crates/jj-store-cli/tests/native_lifecycle/commands.rs#typed_bookmarks_compare_complete_conflicted_targets_and_publish_once
- crates/jj-store-cli/tests/native_lifecycle/commands.rs#typed_commands_reject_readers_stale_root_and_uncommitted_targets
- crates/jj-store-cli/tests/native_lifecycle/commands.rs#typed_preparation_limits_and_protected_publication_leave_authority_unchanged
- crates/jj-store-cli/tests/native_lifecycle/commands.rs#typed_cancellation_retains_shared_worker_until_native_exit
- crates/jj-store-cli/tests/native_lifecycle/commands.rs#typed_preparation_rejects_snapshot_changes_during_staging
- crates/jj-store-cli/tests/native_lifecycle/commands.rs#typed_preparation_rechecks_revocation_during_staging
- crates/jj-store-cli/tests/native_lifecycle/commands.rs#typed_commands_distinguish_hidden_noop_and_ambiguous_heads
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M5-001 details 2](details-02.md)
- [E-M5-001 details 3](details-03.md)
