---
schema: 1
id: E-M13-019
---

# E-M13-019

<!-- evidence-field:id markdown -->
## Id

E-M13-019
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M13.4
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-LOCAL-004
- JST-UI-005
- JST-API-002
- JST-WORK-005
- JST-COLLAB-005
- JST-SEC-006
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

profile-aware-local-browser-ui
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- AGENTS.md
- crates/jj-store-cli/tests/local_browser.rs
- crates/jj-store-http/src/assets.rs
- crates/jj-store-http/src/local_browser.rs
- crates/jj-store-http/tests/assets.rs
- scripts/dev-demo-local-folder.mjs
- scripts/dev-demo-profiles.acceptance.mjs
- scripts/dev-demo-profiles.mjs
- scripts/dev-demo-repository.test.mjs
- scripts/dev-demo.mjs
- scripts/local-browser-acceptance.mjs
- spec/src/api.md
- spec/src/cli.md
- spec/src/collaboration.md
- spec/src/local-jj.md
- spec/src/milestones.md
- spec/src/projects.md
- spec/src/security.md
- spec/src/spec-governance.md
- spec/src/ui.md
- web/src/lib/api/local-client.ts
- web/src/lib/api/local-events.ts
- web/src/lib/api/repositories.ts
- web/src/lib/controller/contracts.ts
- web/src/lib/controller/http-local-code-data-source.ts
- web/src/lib/controller/http-local-store-data-source.ts
- web/src/lib/controller/http-local-work-data-source.ts
- web/src/lib/controller/http-repository-directory.ts
- web/src/lib/controller/http-store-data-source.ts
- web/src/lib/controller/local-folder-manager.ts
- web/src/lib/controller/local-repository-directory.ts
- web/src/lib/controller/native-activity-reader.ts
- web/src/lib/controller/native-activity-types.ts
- web/src/lib/controller/native-agent-context.ts
- web/src/lib/controller/native-data-source.ts
- web/src/lib/controller/native-presentation.ts
- web/src/lib/controller/production-evolution-reader.ts
- web/src/lib/controller/production-file-pages.ts
- web/src/lib/controller/production-inspection-pages.ts
- web/src/lib/controller/production-profile-route.ts
- web/src/lib/controller/production-projects-hub.svelte.ts
- web/src/lib/controller/production-projects-integration.ts
- web/src/lib/controller/production-snapshot.ts
- web/src/lib/controller/production-store-collaboration.svelte.ts
- web/src/lib/controller/production-store-conflicts.ts
- web/src/lib/controller/production-store-controller.svelte.ts
- web/src/lib/controller/production-store-events.ts
- web/src/lib/controller/production-store-index.ts
- web/src/lib/controller/production-store-loaders.ts
- web/src/lib/controller/production-store-mutations.ts
- web/src/lib/controller/production-store-state.svelte.ts
- web/src/lib/controller/production-store-view.ts
- web/src/lib/controller/production-work.svelte.ts
- web/src/lib/controller/projects-activity.svelte.ts
- web/src/lib/controller/projects-hub-types.ts
- web/src/lib/controller/projects-repository-types.ts
- web/src/lib/controller/projects-reviews.svelte.ts
- web/src/lib/controller/repository-profile.ts
- web/src/lib/controller/repository-read-types.ts
- web/src/lib/controller/repository-source.ts
- web/src/lib/controller/review-comment-anchor.ts
- web/src/lib/controller/store-error.ts
- web/src/lib/controller/store-query-keys.ts
- web/src/lib/controller/store-route-codec.ts
- web/src/lib/controller/work-types.ts
- web/src/lib/graph.ts
- web/src/lib/ui/store-application/StoreApplication.svelte
- web/src/lib/ui/store-application/StoreRepositoryApplication.svelte
- web/src/lib/ui/store-application/StoreRepositoryTabs.svelte
- web/src/lib/ui/store-collaboration/StoreAgentContext.svelte
- web/src/lib/ui/store-collaboration/StoreAgentTranscript.svelte
- web/src/lib/ui/store-collaboration/StoreCommentableCommitDiff.svelte
- web/src/lib/ui/store-conflicts/StoreConflictEditor.svelte
- web/src/lib/ui/store-conflicts/StoreConflictHeaderActions.svelte
- web/src/lib/ui/store-conflicts/StoreConflictPath.svelte
- web/src/lib/ui/store-conflicts/model.ts
- web/src/lib/ui/store-settings/StoreLocalFolders.svelte
- web/src/lib/ui/store-settings/StoreRepositorySettings.svelte
- web/src/lib/ui/store-settings/StoreServiceSettings.svelte
- web/src/lib/ui/store-work/StoreProjectsFilterControls.svelte
- web/tests/real/local-browser.spec.ts
- web/tests/real/local-mixed.spec.ts
- web/tests/real/native-repository.spec.ts
- web/tests/real/project-polish.spec.ts
- web/tests/real/projects-activity.spec.ts
- web/tests/real/work-store.spec.ts
- web/tests/vite/local-browser.test.ts
- web/tests/vite/production-store-bootstrap.test.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/local_browser.rs#filesystem_runtime_registers_and_reads_without_native_storage
- scripts/dev-demo-repository.test.mjs#local demo admission reuses canonical
  fixture binding and registration receipt
- web/tests/vite/local-browser.test.ts#local_transport_preserves_git_ids_and_independent_metadata_pin
- web/tests/vite/local-browser.test.ts#folder_add_survives_lost_response_reload_and_receipt_recovery
- web/tests/vite/local-browser.test.ts#concurrent_local_snapshot_reads_share_capture_and_refresh_advances_metadata_explicitly
- web/tests/vite/local-browser.test.ts#history_restoration_reconnects_repository_before_validating_profile_specific_ids
- web/tests/vite/production-store-bootstrap.test.ts#LocalTrusted bootstrap
  exposes Projects for several repositories
- web/tests/real/local-browser.spec.ts#local_repository_opens_files_history_agents_and_accessible_folder_controls
- web/tests/real/local-browser.spec.ts#local_projects_tasks_comments_match_cli_and_recover_unknown_outcomes
- web/tests/real/local-browser.spec.ts#local_folder_reconnect_and_removal_preserve_code_and_collaboration
- web/tests/real/local-mixed.spec.ts#developer_experience_opens_linked_git_repository_and_preserves_mixed_navigation
- web/tests/real/native-repository.spec.ts#LocalTrusted bootstrap opens
  requested repository without browser credentials
- crates/jj-store-http/tests/assets.rs#strict_browser_csp_allows_only_empty_style_attribute_reset
- crates/jj-store-http/tests/assets.rs#browser_assets_are_allowlisted_preloaded_and_sandboxed
- web/tests/real/work-store.spec.ts#native projects tasks and discussion persist
  on RustFS and refresh explicitly
- web/tests/vite/local-browser.test.ts#local_reads_retry_busy_without_advancing_pins_or_replaying_writes
- web/tests/vite/local-browser.test.ts#local_refresh_does_not_reuse_an_older_in_flight_capture
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M13-019 details 2](details-02.md)
- [E-M13-019 details 3](details-03.md)
- [E-M13-019 details 4](details-04.md)
- [E-M13-019 details 5](details-05.md)
- [E-M13-019 details 6](details-06.md)
