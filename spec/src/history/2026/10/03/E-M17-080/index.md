---
schema: 1
id: E-M17-080
---

# E-M17-080

<!-- evidence-field:id markdown -->
## Id

E-M17-080
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M17
- M17.2
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-03
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

opencode-api-import-boundary
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change ypoxtqtz
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-capture/src/opencode_checkpoint.rs
- crates/jj-store-capture/tests/opencode_checkpoint.rs
- crates/jj-store-server/src/capture_import_state/types.rs
- crates/jj-store-server/src/lib.rs
- crates/jj-store-server/src/opencode_import.rs
- crates/jj-store-server/src/opencode_read_api.rs
- crates/jj-store-server/src/opencode_source/snapshot.rs
- crates/jj-store-server/tests/opencode_import.rs
- spec/src/cli.md
- spec/src/collaboration.md
- spec/src/compatibility.md
- spec/src/verification.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-server/tests/opencode_import.rs#idle_api_history_selects_two_verified_prefix_passes_and_preserves_exact_intent
- crates/jj-store-server/tests/opencode_import.rs#selected_api_prefix_refuses_rewrite_truncation_directory_and_endpoint_rebinding
- crates/jj-store-server/tests/opencode_import.rs#empty_api_history_has_a_qualified_zero_count_and_refuses_fake_byte_positions
- crates/jj-store-server/tests/opencode_import.rs#selection_verifies_again_and_never_exposes_a_boundary_after_prefix_change
- crates/jj-store-capture/tests/opencode_checkpoint.rs#shared_message_prefix_preserves_the_original_snapshot_hash_bytes
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M17-080 details 2](details-02.md)
- [E-M17-080 details 3](details-03.md)
