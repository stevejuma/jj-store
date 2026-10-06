---
schema: 1
id: E-M9-001
---

# E-M9-001

<!-- evidence-field:id markdown -->
## Id

E-M9-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M9
- M9.1
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-28
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

implementation
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- AGENTS.md
- crates/jj-store-cli/tests/administration_rpc.rs
- crates/jj-store-cli/tests/native_lifecycle/conflict_uploads.rs
- crates/jj-store-http/src/uploads.rs
- crates/jj-store-rpc/proto/native.proto
- crates/jj-store-rpc/src/administration.rs
- crates/jj-store-service/src/mutations.rs
- crates/jj-store-service/src/reads.rs
- crates/jj-store-service/src/uploads.rs
- crates/jj-store-wal/src/publication.rs
- crates/jj-store-wal/src/publication/application.rs
- crates/jj-store-wal/src/publication/collaboration.rs
- crates/jj-store-wal/src/publication/domains.rs
- crates/jj-store-wal/src/publication/reader.rs
- crates/jj-store-wal/src/records.rs
- crates/jj-store-wal/tests/publication.rs
- schemas/openapi.json
- scripts/check-progress.mjs
- scripts/governance.test.mjs
- spec-validator.config.mjs
- spec/src/SUMMARY.md
- spec/src/api.md
- spec/src/cli.md
- spec/src/collaboration.md
- spec/src/compatibility.md
- spec/src/jj-integration.md
- spec/src/milestones.md
- spec/src/mutations.md
- spec/src/publication.md
- spec/src/spec-governance.md
- spec/src/verification.md
- web/src/lib/api/generated.ts
- web/src/lib/controller/http-store-data-source.ts
- web/src/lib/controller/store-mutation-types.ts
- web/tests/production-inspection-pages.test.ts
- web/tests/production-store-conflicts.test.ts
- web/tests/production-store-loaders.test.ts
- web/tests/production-store-mutations.test.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-wal/tests/publication.rs#domain_publications_preserve_roots_without_staling_code
- crates/jj-store-wal/tests/publication.rs#legacy_repository_enables_collaboration_without_rewriting_code_history
- crates/jj-store-wal/tests/publication.rs#collaboration_enablement_resolves_unknown_manifest_outcomes
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M9-001 details 2](details-02.md)
