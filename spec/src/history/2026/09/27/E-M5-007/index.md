---
schema: 1
id: E-M5-007
---

# E-M5-007

<!-- evidence-field:id markdown -->
## Id

E-M5-007
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M5
- M5.3
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-27
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

authorized-conflict-uploads
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- Cargo.lock
- crates/jj-store-cli/tests/native_lifecycle/conflict_uploads.rs
- crates/jj-store-cli/tests/native_lifecycle/resolve.rs
- crates/jj-store-cli/tests/native_lifecycle/upload_faults.rs
- crates/jj-store-http/Cargo.toml
- crates/jj-store-http/src/contract.rs
- crates/jj-store-http/src/contract/tests.rs
- crates/jj-store-http/src/dto.rs
- crates/jj-store-http/src/error.rs
- crates/jj-store-http/src/lib.rs
- crates/jj-store-http/src/mutations.rs
- crates/jj-store-http/src/routes.rs
- crates/jj-store-http/src/uploads.rs
- crates/jj-store-jj/src/commands/execute.rs
- crates/jj-store-jj/src/commands/mod.rs
- crates/jj-store-jj/src/commands/resolve.rs
- crates/jj-store-model/src/ids.rs
- crates/jj-store-model/tests/framing.rs
- crates/jj-store-server/src/process.rs
- crates/jj-store-service/src/commands.rs
- crates/jj-store-service/src/lib.rs
- crates/jj-store-service/src/uploads.rs
- docs/evidence/m5-conflict-uploads.md
- schemas/openapi.json
- scripts/contracts.mjs
- scripts/contracts.test.mjs
- spec/src/api.md
- spec/src/architecture.md
- spec/src/cli.md
- spec/src/compatibility.md
- spec/src/identity.md
- spec/src/jj-integration.md
- spec/src/milestones.md
- spec/src/mutations.md
- spec/src/security.md
- spec/src/spec-governance.md
- web/src/lib/api/generated.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/native_lifecycle/conflict_uploads.rs#conflict_upload_http_stages_verified_binary_without_publication_then_resolves
- crates/jj-store-cli/tests/native_lifecycle/conflict_uploads.rs#conflict_upload_reuse_checks_principal_snapshot_path_and_verified_bytes
- crates/jj-store-cli/tests/native_lifecycle/conflict_uploads.rs#conflict_upload_atomic_mixed_resolution_retains_copy_history_and_other_conflicts
- crates/jj-store-cli/tests/native_lifecycle/upload_faults.rs#conflict_upload_lost_ack_repeats_same_record_after_restart_and_stale_binding_rejects
- crates/jj-store-cli/tests/native_lifecycle/upload_faults.rs#conflict_upload_revocation_during_body_and_busy_deadline_reject_before_remote_writes
- crates/jj-store-cli/tests/native_lifecycle/upload_faults.rs#conflict_upload_http_bounds_bodies_and_authorizes_before_consumption
- crates/jj-store-service/src/uploads.rs#conflict_upload_record_v1_has_canonical_independent_digest_and_scope_binding
- crates/jj-store-http/src/contract/tests.rs#conflict_upload_contract_has_binary_body_exact_snapshot_and_write_permission
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M5-007 details 2](details-02.md)
