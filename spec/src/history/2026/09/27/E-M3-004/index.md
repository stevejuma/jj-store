---
schema: 1
id: E-M3-004
---

# E-M3-004

<!-- evidence-field:id markdown -->
## Id

E-M3-004
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M3
- M3.1
- M3.2
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-27
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

authenticated-native-lifecycle
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- Cargo.lock
- Cargo.toml
- crates/jj-store-cli/Cargo.toml
- crates/jj-store-cli/tests/native_lifecycle.rs
- crates/jj-store-jj/Cargo.toml
- crates/jj-store-jj/src/backend.rs
- crates/jj-store-jj/src/factories.rs
- crates/jj-store-jj/src/lib.rs
- crates/jj-store-jj/src/operation_stores.rs
- crates/jj-store-jj/src/policy.rs
- crates/jj-store-jj/src/runtime.rs
- crates/jj-store-jj/tests/native_operations.rs
- crates/jj-store-service/Cargo.toml
- crates/jj-store-service/src/guard.rs
- crates/jj-store-service/src/lib.rs
- crates/jj-store-service/src/native.rs
- crates/jj-store-wal/src/publication.rs
- spec/src/architecture.md
- spec/src/cli.md
- spec/src/jj-integration.md
- spec/src/mutations.md
- spec/src/publication.md
- spec/src/security.md
- docs/evidence/m3-native-session-rustfs.json
- spec/src/compatibility.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/native_lifecycle.rs#authenticated_native_workspace_edit_describe_reopen_and_cache_rebuild
- crates/jj-store-cli/tests/native_lifecycle.rs#independent_workspaces_preserve_concurrent_operations_and_unsnapshotted_edits
- crates/jj-store-cli/tests/native_lifecycle.rs#transition_identity_survives_session_restart_and_late_duplicates
- crates/jj-store-cli/tests/native_lifecycle.rs#protected_history_rejects_hidden_rewrites_without_predecessor_metadata
- crates/jj-store-cli/tests/native_lifecycle.rs#protected_bookmark_also_protects_its_commit_identity
- crates/jj-store-cli/tests/native_lifecycle.rs#committed_superseded_operation_cannot_be_resurrected_under_a_new_request
- crates/jj-store-cli/tests/native_lifecycle.rs#current_permissions_revocation_and_repository_binding_cover_native_access
- crates/jj-store-cli/tests/native_lifecycle.rs#concurrent_workspace_name_claims_cannot_alias_physical_clients
- crates/jj-store-cli/tests/native_lifecycle.rs#malformed_native_publications_stale_strict_intents_and_unknown_retries
- crates/jj-store-cli/tests/native_lifecycle.rs#protection_can_freeze_multiple_existing_divergent_incarnations
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M3-004 details 2](details-02.md)
- [E-M3-004 details 3](details-03.md)
