---
schema: 1
id: E-M4-016
---

# E-M4-016

<!-- evidence-field:id markdown -->
## Id

E-M4-016
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M4
- M4.2
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-27
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

generated-openapi-and-typescript-contracts
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- Cargo.lock
- Cargo.toml
- README.md
- crates/jj-store-cli/tests/native_lifecycle/http_reads.rs
- crates/jj-store-http/Cargo.toml
- crates/jj-store-http/examples/openapi.rs
- crates/jj-store-http/src/contract.rs
- crates/jj-store-http/src/contract/tests.rs
- crates/jj-store-http/src/dto.rs
- crates/jj-store-http/src/error.rs
- crates/jj-store-http/src/lib.rs
- crates/jj-store-http/src/routes.rs
- crates/jj-store-jj/Cargo.toml
- crates/jj-store-jj/src/native_dto.rs
- crates/jj-store-jj/src/query/cache.rs
- crates/jj-store-jj/src/query/content.rs
- crates/jj-store-jj/src/query/content/conflicts.rs
- crates/jj-store-jj/src/query/content/diff.rs
- crates/jj-store-jj/src/query/content/operations.rs
- crates/jj-store-jj/src/query/content/trees.rs
- crates/jj-store-jj/src/query/mod.rs
- crates/jj-store-model/Cargo.toml
- crates/jj-store-model/src/ids.rs
- crates/jj-store-service/Cargo.toml
- crates/jj-store-service/src/events.rs
- crates/jj-store-service/src/index_tasks.rs
- crates/jj-store-service/src/reads.rs
- docs/evidence/m4-generated-contracts.md
- package.json
- pnpm-lock.yaml
- schemas/README.md
- schemas/openapi.json
- scripts/contracts-validation.mjs
- scripts/contracts.mjs
- scripts/contracts.test.mjs
- scripts/contracts.types.ts
- scripts/tsconfig.contracts.json
- spec-validator.config.mjs
- spec/src/api.md
- spec/src/architecture.md
- spec/src/cli.md
- spec/src/compatibility.md
- spec/src/identity.md
- spec/src/jj-integration.md
- spec/src/milestones.md
- spec/src/mutations.md
- spec/src/spec-governance.md
- spec/src/verification.md
- web/src/lib/api/generated.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-http/src/contract/tests.rs#contract_distinguishes_optional_inputs_from_nullable_outputs
- crates/jj-store-http/src/contract/tests.rs#contract_preserves_native_shapes_and_hex_id_widths
- crates/jj-store-http/src/contract/tests.rs#contract_inventory_covers_security_methods_media_and_task_bodies
- crates/jj-store-cli/tests/native_lifecycle/http_reads.rs#http_contract_capture_matches_registered_routes
- scripts/contracts.test.mjs#openapi_document_validates_without_external_references
- scripts/contracts.test.mjs#native_hex_rejects_short_uppercase_and_untyped_values
- scripts/contracts.test.mjs#frozen_native_shapes_preserve_nullability_and_tuples
- scripts/contracts.test.mjs#tagged_union_and_required_nullable_fields_fail_closed
- scripts/contracts.test.mjs#response_validation_rejects_missing_contracts_and_wrong_media
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M4-016 details 2](details-02.md)
