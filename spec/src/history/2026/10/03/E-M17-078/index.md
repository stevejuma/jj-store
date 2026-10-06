---
schema: 1
id: E-M17-078
---

# E-M17-078

<!-- evidence-field:id markdown -->
## Id

E-M17-078
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

api-snapshot-source-management
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change ttlntprt
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-capture/src/reader_publication/snapshot.rs
- crates/jj-store-capture/src/reader_publication.rs
- crates/jj-store-capture/tests/snapshot_publication.rs
- crates/jj-store-http/src/contract/capture_tests.rs
- crates/jj-store-server/src/capture_management/sources.rs
- crates/jj-store-server/src/capture_supervisor/opencode/scheduling.rs
- crates/jj-store-server/src/capture_supervisor/opencode/source.rs
- crates/jj-store-server/src/capture_supervisor/opencode.rs
- crates/jj-store-server/src/capture_supervisor/sources.rs
- crates/jj-store-server/src/capture_supervisor.rs
- crates/jj-store-server/src/opencode_source.rs
- crates/jj-store-server/tests/capture_management/api_snapshot.rs
- crates/jj-store-server/tests/capture_management/imports.rs
- crates/jj-store-server/tests/capture_management.rs
- crates/jj-store-service/src/capture_management/sources.rs
- schemas/openapi.json
- scripts/contracts.mjs
- spec/src/api.md
- spec/src/cli.md
- spec/src/collaboration.md
- spec/src/compatibility.md
- spec/src/mutations.md
- spec/src/spec-governance.md
- spec/src/verification.md
- web/src/lib/api/generated.ts
- web/src/lib/controller/capture-response-validation.ts
- web/src/lib/controller/capture-snapshot-validation.ts
- web/src/lib/controller/capture-validation.ts
- web/tests/vite/capture-management.test.ts
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-server/tests/capture_management/api_snapshot.rs#api_snapshot_progress_is_native_receipt_backed_and_body_free
- crates/jj-store-capture/tests/snapshot_publication.rs#page_inspection_is_bounded_and_keeps_independent_snapshot_witnesses
- web/tests/vite/capture-management.test.ts#API snapshot source coverage
  preserves exact positions and unavailable publication
- web/tests/vite/capture-management.test.ts#API snapshot coverage rejects
  malformed counts and unproved publication
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M17-078 details 2](details-02.md)
- [E-M17-078 details 3](details-03.md)
