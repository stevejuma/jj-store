---
schema: 1
id: E-M5-003
---

# E-M5-003

<!-- evidence-field:id markdown -->
## Id

E-M5-003
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

typed-http-mutation-api
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- Cargo.lock
- README.md
- crates/jj-store-cli/tests/native_lifecycle/http_mutations.rs
- crates/jj-store-cli/tests/native_lifecycle/http_reads.rs
- crates/jj-store-cli/tests/native_lifecycle.rs
- crates/jj-store-http/src/contract/tests.rs
- crates/jj-store-http/src/contract.rs
- crates/jj-store-http/src/dto.rs
- crates/jj-store-http/src/error.rs
- crates/jj-store-http/src/lib.rs
- crates/jj-store-http/src/mutations.rs
- crates/jj-store-http/src/routes.rs
- crates/jj-store-service/src/mutations.rs
- crates/jj-store-wal/Cargo.toml
- crates/jj-store-wal/src/publication.rs
- docs/evidence/m5-http-mutations.md
- schemas/README.md
- schemas/openapi.json
- scripts/contracts.mjs
- scripts/contracts.test.mjs
- scripts/contracts.types.ts
- spec/src/api.md
- spec/src/architecture.md
- spec/src/cli.md
- spec/src/compatibility.md
- spec/src/milestones.md
- spec/src/mutations.md
- spec/src/publication.md
- spec/src/spec-governance.md
- spec/src/verification.md
- web/src/lib/api/generated.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/native_lifecycle/http_mutations.rs#http_mutations_preview_apply_status_replay_and_contract_capture
- crates/jj-store-cli/tests/native_lifecycle/http_mutations.rs#http_mutations_authentication_strict_json_limits_and_stale_requests_publish_nothing
- crates/jj-store-cli/tests/native_lifecycle/http_mutations.rs#http_mutations_run_all_advertised_commands_and_late_bookmark_retries
- crates/jj-store-cli/tests/native_lifecycle/http_mutations.rs#http_mutations_enforce_protection_revocation_and_principal_scoped_status
- crates/jj-store-cli/tests/native_lifecycle/http_mutations.rs#http_mutations_small_result_budget_preserves_unknown_identity_and_recoverable_receipt
- crates/jj-store-cli/tests/native_lifecycle/http_mutations.rs#http_mutations_deadline_after_manifest_cas_retains_request_identity
- crates/jj-store-cli/tests/native_lifecycle/http_reads.rs#http_mutation_previews_share_native_worker_admission_with_file_reads
- crates/jj-store-http/src/contract/tests.rs#mutation_contracts_require_typed_intent_and_write_permission
- scripts/contracts.test.mjs#mutation_requests_are_strict_and_unknown_outcomes_retain_identity
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M5-003 details 2](details-02.md)
