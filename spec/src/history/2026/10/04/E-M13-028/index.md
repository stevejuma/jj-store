---
schema: 1
id: E-M13-028
---

# E-M13-028

<!-- evidence-field:id markdown -->
## Id

E-M13-028
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M13.5
- M14
- M14.2
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-04
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

main-workspace-integration
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu merge rryrrvzz
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Main nrwwnrqz/336e300b is clean and owns newer capture/desktop work. Verified creation/conversion history ends at nmqnqtuz/c6eeec7a in repository-conversion. Five additive canonical-document conflicts and two milestone regions require preserving both parent histories.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64; locked main-workspace dependencies; JJ 0.45.1 SHA-1 profile; persistent main LocalTrusted Vite fixture at ports 7460/7462.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No additional product, API, object, manifest, WAL, archive or companion format during integration. Native publication and new-folder export authority retain E-M13-027 qualification.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- Cargo.lock
- catalog/components/store-application/StoreRepositoryConversion.stories.svelte
- crates/jj-store-http/src/dto.rs
- crates/jj-store-http/src/local_browser/conversion.rs
- crates/jj-store-http/src/local_browser.rs
- crates/jj-store-jj/src/conversion.rs
- crates/jj-store-jj/src/lib.rs
- crates/jj-store-jj/src/local_evidence/capture.rs
- crates/jj-store-jj/src/local_evidence/mod.rs
- crates/jj-store-jj/tests/conversion.rs
- crates/jj-store-server/Cargo.toml
- crates/jj-store-server/src/local_repositories/conversion/journal.rs
- crates/jj-store-server/src/local_repositories/conversion/prefix.rs
- crates/jj-store-server/src/local_repositories/conversion/preparation.rs
- crates/jj-store-server/src/local_repositories/conversion.rs
- crates/jj-store-server/src/local_repositories/native_creation.rs
- crates/jj-store-server/src/local_repositories.rs
- crates/jj-store-server/src/process/preparation.rs
- crates/jj-store-server/tests/capture_management/api_snapshot.rs
- crates/jj-store-server/tests/native_creation.rs
- crates/jj-store-server/tests/repository_conversion.rs
- crates/jj-store-service/src/administration.rs
- crates/jj-store-service/src/conversion/agent.rs
- crates/jj-store-service/src/conversion/collab.rs
- crates/jj-store-service/src/conversion/contracts.rs
- crates/jj-store-service/src/conversion/directory.rs
- crates/jj-store-service/src/conversion/history.rs
- crates/jj-store-service/src/conversion/import.rs
- crates/jj-store-service/src/conversion/review.rs
- crates/jj-store-service/src/conversion.rs
- crates/jj-store-service/src/lib.rs
- crates/jj-store-service/src/local_browser.rs
- crates/jj-store-service/src/local_repository/conversion.rs
- crates/jj-store-service/src/local_repository/conversion_import.rs
- crates/jj-store-service/src/local_repository/work.rs
- crates/jj-store-service/src/local_repository.rs
- crates/jj-store-service/tests/conversion.rs
- crates/jj-store-wal/src/publication/native_copy.rs
- crates/jj-store-wal/src/publication/review_catalog_engine.rs
- crates/jj-store-wal/src/publication/work.rs
- crates/jj-store-wal/src/publication.rs
- schemas/openapi.json
- scripts/contracts.mjs
- spec/src/api.md
- spec/src/architecture.md
- spec/src/cli.md
- spec/src/collaboration.md
- spec/src/compatibility.md
- spec/src/jj-integration.md
- spec/src/local-jj.md
- spec/src/milestones.md
- spec/src/mutations.md
- spec/src/projects.md
- spec/src/publication.md
- spec/src/spec-governance.md
- spec/src/ui.md
- web/src/lib/api/generated.ts
- web/src/lib/controller/contracts.ts
- web/src/lib/controller/native-repository-manager.ts
- web/src/lib/controller/production-store-lifecycle.svelte.ts
- web/src/lib/controller/production-store-state.svelte.ts
- web/src/lib/controller/production-store-view.ts
- web/src/lib/controller/repository-conversion-manager.ts
- web/src/lib/ui/store-application/StoreApplicationNavigation.svelte
- web/src/lib/ui/store-settings/StoreNativeRepositoryCreate.svelte
- web/src/lib/ui/store-settings/StoreRepositoryConversion.svelte
- web/src/lib/ui/store-settings/StoreRepositoryConversionDialog.svelte
- web/src/lib/ui/store-settings/StoreRepositorySettings.svelte
- web/src/lib/ui/store-work/StoreProjectsAddMenu.svelte
- web/tests/browser/native-repository-creation.spec.ts
- web/tests/browser/repository-conversion-real.mjs
- web/tests/browser/repository-conversion.spec.ts
- web/tests/native-repository-manager.test.ts
- web/tests/repository-conversion-manager.test.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-server/tests/native_creation.rs#native_creation_http_is_local_admin_only_and_captures_contracts
- crates/jj-store-server/tests/repository_conversion.rs#conversion_round_trip_preserves_source_and_publishes_new_colocated_folder
- web/tests/repository-conversion-manager.test.ts#conversion preserves exact
  source and destination intent through lost response, reload and retry
- web/tests/browser/repository-conversion.spec.ts#cold conversion status retries
  preserve a dismissed error episode
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M13-028 details 2](details-02.md)
