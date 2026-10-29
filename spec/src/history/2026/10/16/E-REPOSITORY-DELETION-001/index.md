---
schema: 1
id: E-REPOSITORY-DELETION-001
---

# E-REPOSITORY-DELETION-001

<!-- evidence-field:id markdown -->
## Id

E-REPOSITORY-DELETION-001
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-16
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M8a.9
- M4.18
- M13.4
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-ARCH-001
- JST-UI-012
- JST-UI-016
- JST-UI-019
- JST-LOCAL-004
- JST-OBJ-004
- JST-SEC-006
- JST-API-007
- JST-CLI-005
- JST-ARCH-005
- JST-COMPAT-005
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

repository-lifecycle-acceptance
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

Enclosing Jujutsu change nqrwyvsrpulqqqpnvwvmoqtystzknvqt.
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Clean working copy cb0f5c96 on sidebar regression a28e1ac3. General had no removal control. Folder removal already existed as a revision-guarded LocalTrusted installation API and preserved source folders. Native storage had no deletion port, lifecycle API or journal. The new General regression failed: 1 failed, 10 passed.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- .changeset/repository-removal.md
- Cargo.lock
- catalog/components/store-application/RepositoryDeletionRecovery.stories.svelte
- catalog/components/store-application/RepositoryRemoval.stories.svelte
- catalog/components/store-application/StoreRepositorySettings.stories.svelte
- catalog/fixtures/RepositoryDeletionRecoveryFixture.svelte
- catalog/fixtures/RepositoryRemovalFixture.svelte
- crates/jj-store-http/src/dto.rs
- crates/jj-store-http/src/lib.rs
- crates/jj-store-http/src/local_browser.rs
- crates/jj-store-http/tests/repository_deletion.rs
- crates/jj-store-object/Cargo.toml
- crates/jj-store-object/src/s3/deletion.rs
- crates/jj-store-object/src/s3.rs
- crates/jj-store-server/src/lib.rs
- crates/jj-store-server/src/process/authority.rs
- crates/jj-store-server/src/process/preparation.rs
- crates/jj-store-server/src/process.rs
- crates/jj-store-server/src/registry.rs
- crates/jj-store-server/src/registry_catalog.rs
- crates/jj-store-server/src/repository_deletion.rs
- crates/jj-store-server/tests/repository_deletion.rs
- crates/jj-store-service/src/lib.rs
- crates/jj-store-service/src/repository_deletion.rs
- docs/ui-provenance.json
- schemas/openapi.json
- spec/src/current/contracts/api/25-filesystem-local-repository-api.md
- spec/src/current/contracts/architecture/01-architecture.md
- spec/src/current/contracts/architecture/23-public-distribution.md
- spec/src/current/contracts/cli/14-local-browser-composition.md
- spec/src/current/contracts/compatibility/09-local-management-compatibility.md
- spec/src/current/contracts/mutations/01-typed-mutations.md
- spec/src/current/contracts/object-store/01-object-storage.md
- spec/src/current/contracts/object-store/02-requirements.md
- spec/src/current/contracts/security/04-provisioning-and-token-administration.md
- spec/src/current/contracts/ui/09-transient-store-settings.md
- spec/src/current/governance/spec-governance.md
- spec/src/current/progress/M13/history/M13.4/index.md
- spec/src/current/progress/M13/index.md
- spec/src/current/progress/M4/history/M4.18/index.md
- spec/src/current/progress/M4/index.md
- spec/src/current/progress/M8a/history/M8a.9/SUMMARY.nav.md
- spec/src/current/progress/M8a/history/M8a.9/index.md
- spec/src/current/progress/M8a/history/SUMMARY.nav.md
- spec/src/current/progress/M8a/history/index.md
- spec/src/current/progress/M8a/index.md
- spec/src/history/2026/10/16/E-REPOSITORY-DELETION-001/SUMMARY.nav.md
- spec/src/history/2026/10/16/E-REPOSITORY-DELETION-001/details-02.md
- spec/src/history/2026/10/16/E-REPOSITORY-DELETION-001/details-03.md
- spec/src/history/2026/10/16/E-REPOSITORY-DELETION-001/index.md
- spec/src/history/2026/10/16/SUMMARY.nav.md
- spec/src/history/2026/10/16/index.md
- web/src/lib/api/generated.ts
- web/src/lib/controller/contracts.ts
- web/src/lib/controller/local-repository-removal.ts
- web/src/lib/controller/native-repository-deletion.ts
- web/src/lib/controller/production-store-lifecycle.svelte.ts
- web/src/lib/controller/production-store-state.svelte.ts
- web/src/lib/controller/production-store-view.ts
- web/src/lib/controller/store-error.ts
- web/src/lib/ui/store-application/StoreApplication.svelte
- web/src/lib/ui/store-settings/StoreRepositoryDeletionRecovery.svelte
- web/src/lib/ui/store-settings/StoreRepositoryRemoval.svelte
- web/src/lib/ui/store-settings/StoreRepositorySettings.svelte
- web/tests/vite/local-repository-removal.test.ts
- web/tests/vite/native-repository-deletion.test.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-server/tests/repository_deletion.rs#permanent_deletion_fences_restarts_erases_objects_and_preserves_other_repositories
- crates/jj-store-object/src/s3/deletion.rs#cancelling_a_put_cannot_release_its_fence_before_storage_acknowledges
- crates/jj-store-object/src/s3/deletion.rs#a_foreign_listed_key_is_rejected_before_any_delete
- crates/jj-store-object/src/s3/deletion.rs#unknown_put_outcomes_block_erasure_before_list_or_delete
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64; Rust 1.99.0; jj 0.45.1; Node 24.10.0; pnpm 10.32.1; Vitest 4.1.11; quick-xml 0.38.4. Existing separately built pinned embedded RustFS executable used without modifying its source or live fixture. Embedded runtime SHA-256: c3641db219705a7956dcfc4935ff2043f714f159b36dd315f7a234e1a3fac590 (runtime/embedded-rustfs/target/debug/jj-store-rustfs).
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

New owner-only V1 installation deletion journal and public V1 delete/status DTOs; optional sorted deleted-repository IDs in existing V2 discovery catalogs (legacy omission preserved). Browser persists public original request/epoch/installation only. Ordinary ObjectStore port, native objects, WAL/receipt encodings, manifest CAS and local folder removal API remain unchanged.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Local Git-backed registration removal plus explicitly confirmed LocalTrusted Admin permanent managed native object erasure. UI General, confirmation, notifications, installation-owned reload recovery, API admission, server journal/catalog and privileged S3 fencing/erasure are one lifecycle feature. Versioned buckets refuse preflight; external S3, authenticated/shared erasure, independent backups and local folder deletion are out of scope. S3 runtime changes justify the complete object-crate suite plus direct native authorization/administration, installation and HTTP boundary suites; no root aggregate or full Rust workspace qualification.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-REPOSITORY-DELETION-001 details 2](details-02.md)
- [E-REPOSITORY-DELETION-001 details 3](details-03.md)
