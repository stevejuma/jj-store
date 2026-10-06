---
schema: 1
id: E-M13-027
---

# E-M13-027

<!-- evidence-field:id markdown -->
## Id

E-M13-027
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M13.5
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-04
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

real-rustfs
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

nmqnqtuz
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Creation and conversion primitives committed as c21a0e4e and 4022d470. Independent destination imports, durable installation orchestration and conversion UI not qualified.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64; pinned JJ 0.45.1, Git SHA-1; own workspace target; isolated reused embedded RustFS executable; locked dependencies; registry Design Core.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Version-1 owned archive/intent/report and digest-selected private installation journal. Original archive retained before staging; no new native persistence codec. Native manifests publish last; new colocated folders publish with synced no-replace rename. Fresh destination operations and typed identity maps; source grants/settings/credentials/journals do not become authority.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- Cargo.lock
- catalog/components/store-application/StoreRepositoryConversion.stories.svelte
- crates/jj-store-http/src/dto.rs
- crates/jj-store-http/src/local_browser/conversion.rs
- crates/jj-store-http/src/local_browser.rs
- crates/jj-store-jj/src/query/language/identity.rs
- crates/jj-store-server/Cargo.toml
- crates/jj-store-server/src/local_repositories/conversion/journal.rs
- crates/jj-store-server/src/local_repositories/conversion/prefix.rs
- crates/jj-store-server/src/local_repositories/conversion/preparation.rs
- crates/jj-store-server/src/local_repositories/conversion.rs
- crates/jj-store-server/src/local_repositories/native_creation.rs
- crates/jj-store-server/src/local_repositories.rs
- crates/jj-store-server/src/storage_management/journal.rs
- crates/jj-store-server/src/storage_management/mod.rs
- crates/jj-store-server/tests/capture_management/api_snapshot.rs
- crates/jj-store-server/tests/repository_conversion.rs
- crates/jj-store-service/src/administration.rs
- crates/jj-store-service/src/collab_reads/materialization.rs
- crates/jj-store-service/src/conversion/contracts.rs
- crates/jj-store-service/src/conversion/directory.rs
- crates/jj-store-service/src/conversion/import.rs
- crates/jj-store-service/src/conversion.rs
- crates/jj-store-service/src/local_browser.rs
- crates/jj-store-service/src/local_repository/conversion_import.rs
- crates/jj-store-service/src/local_repository/work.rs
- crates/jj-store-service/src/local_repository.rs
- crates/jj-store-service/tests/conversion.rs
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
- spec/src/publication.md
- spec/src/spec-governance.md
- spec/src/ui.md
- web/src/lib/api/generated.ts
- web/src/lib/controller/contracts.ts
- web/src/lib/controller/production-store-lifecycle.svelte.ts
- web/src/lib/controller/production-store-state.svelte.ts
- web/src/lib/controller/production-store-view.ts
- web/src/lib/controller/repository-conversion-manager.ts
- web/src/lib/ui/store-settings/StoreRepositoryConversion.svelte
- web/src/lib/ui/store-settings/StoreRepositoryConversionDialog.svelte
- web/src/lib/ui/store-settings/StoreRepositorySettings.svelte
- web/tests/browser/repository-conversion-real.mjs
- web/tests/browser/repository-conversion.spec.ts
- web/tests/repository-conversion-manager.test.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-server/tests/repository_conversion.rs#cold_rustfs_conversion_round_trip_retains_nonempty_history_across_restart
- crates/jj-store-server/tests/repository_conversion.rs#conversion_round_trip_preserves_source_and_publishes_new_colocated_folder
- crates/jj-store-server/tests/repository_conversion.rs#lost_final_manifest_ack_recovers_after_source_removal_and_restart
- crates/jj-store-server/tests/repository_conversion.rs#unfinished_preparation_retries_the_saved_archive_after_source_removal
- crates/jj-store-server/tests/repository_conversion.rs#retained_report_digest_rejects_altered_identity_maps
- crates/jj-store-server/tests/repository_conversion.rs#conversion_http_derives_admin_authority_and_captures_contracts
- crates/jj-store-service/tests/conversion.rs#local_history_archive_retains_every_selected_work_version_and_exact_git_evidence
- crates/jj-store-jj/src/query/language/identity.rs#secure_repository_config_requires_matching_identity_without_migration
- crates/jj-store-service/tests/collab_pagination/materialization.rs#expiry_eviction_and_failed_validation_rematerialize_from_storage
- crates/jj-store-cli/tests/native_lifecycle/http_local_management.rs#local_workspace_management_requires_local_admin_and_captures_contracts
- crates/jj-store-server/tests/capture_management/api_snapshot.rs#api_snapshot_progress_is_native_receipt_backed_and_body_free
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M13-027 details 2](details-02.md)
- [E-M13-027 details 3](details-03.md)
- [E-M13-027 details 4](details-04.md)
