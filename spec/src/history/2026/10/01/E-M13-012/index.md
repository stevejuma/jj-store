---
schema: 1
id: E-M13-012
---

# E-M13-012

<!-- evidence-field:id markdown -->
## Id

E-M13-012
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M13
- M13.3
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-01
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

workspace-integration
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

ptqprvlv
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Both workspaces clean. Main vqqlmmol/06da9dd2 retains Projects hub and native development sample seed; local qtxvrqqm/0823662b retains qualified M13.1/M13.2 and partial M13.3. Merge has four conflicts: paging visibility and spec tables/prose/evidence. No new M13 feature implementation is authorized in this integration.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- .storybook/vite.config.ts
- AGENTS.md
- Cargo.lock
- catalog/components/store-application/StoreConnection.stories.svelte
- catalog/components/store-application/StoreRepositoryNavigation.stories.svelte
- catalog/components/store-application/StoreRepositoryPicker.stories.svelte
- catalog/components/store-work/ProjectsDirectory.stories.svelte
- catalog/components/store-work/ProjectsHub.stories.svelte
- catalog/components/store-work/StoreWork.stories.svelte
- catalog/fixtures/ProjectsHubFixture.svelte
- catalog/fixtures/StoreWorkFixture.svelte
- catalog/fixtures/collaboration-review.ts
- catalog/fixtures/projects-hub-fixture.ts
- catalog/fixtures/projects-hub-layout.ts
- crates/jj-agent/src/codec.rs
- crates/jj-agent/src/indexes.rs
- crates/jj-agent/src/lib.rs
- crates/jj-agent/src/objects.rs
- crates/jj-agent/src/portable.rs
- crates/jj-agent/src/provenance.rs
- crates/jj-agent/src/provenance_facts.rs
- crates/jj-agent/tests/fixtures/native-association-v1.json
- crates/jj-agent/tests/portable.rs
- crates/jj-review/src/codec.rs
- crates/jj-review/src/lib.rs
- crates/jj-review/src/portable.rs
- crates/jj-review/tests/portable.rs
- crates/jj-store-cli/src/agent_capture.rs
- crates/jj-store-cli/src/agent_capture_intent.rs
- crates/jj-store-cli/src/agent_commands.rs
- crates/jj-store-cli/src/agent_commands/backend.rs
- crates/jj-store-cli/src/agent_commands/provenance.rs
- crates/jj-store-cli/src/agent_commands/types.rs
- crates/jj-store-cli/src/agent_provenance_journal.rs
- crates/jj-store-cli/src/agent_publication.rs
- crates/jj-store-cli/src/commands.rs
- crates/jj-store-cli/src/lib.rs
- crates/jj-store-cli/src/local_agent_publication.rs
- crates/jj-store-cli/src/local_diagnostics.rs
- crates/jj-store-cli/src/store_commands.rs
- crates/jj-store-cli/src/work_commands.rs
- crates/jj-store-cli/src/work_commands/client.rs
- crates/jj-store-cli/src/work_commands/output.rs
- crates/jj-store-cli/src/work_commands/task.rs
- crates/jj-store-cli/tests/local_agent.rs
- crates/jj-store-cli/tests/local_agent_provenance.rs
- crates/jj-store-cli/tests/local_agent_provenance/fixture.rs
- crates/jj-store-cli/tests/local_agent_provenance_cli.rs
- crates/jj-store-cli/tests/local_agent_recovery.rs
- crates/jj-store-cli/tests/local_work.rs
- crates/jj-store-cli/tests/native_lifecycle/http_collaboration_reviews.rs
- crates/jj-store-cli/tests/native_lifecycle/http_reads.rs
- crates/jj-store-cli/tests/native_lifecycle/http_work.rs
- crates/jj-store-http/src/dto.rs
- crates/jj-store-http/src/repositories.rs
- crates/jj-store-http/src/routes.rs
- crates/jj-store-http/src/work.rs
- crates/jj-store-jj/Cargo.toml
- crates/jj-store-jj/src/lib.rs
- crates/jj-store-jj/src/local_evidence/capture.rs
- crates/jj-store-jj/src/local_evidence/commits.rs
- crates/jj-store-jj/src/local_evidence/mod.rs
- crates/jj-store-jj/src/local_evidence/operations.rs
- crates/jj-store-jj/src/local_evidence/views.rs
- crates/jj-store-jj/src/local_evidence/wire.rs
- crates/jj-store-jj/src/local_metadata/blobs.rs
- crates/jj-store-jj/src/local_metadata/mod.rs
- crates/jj-store-jj/src/local_metadata/objects.rs
- crates/jj-store-jj/src/local_metadata/prepare.rs
- crates/jj-store-jj/src/local_metadata/types.rs
- crates/jj-store-jj/src/local_repository.rs
- crates/jj-store-jj/tests/local_evidence.rs
- crates/jj-store-jj/tests/local_metadata.rs
- crates/jj-store-model/src/ids.rs
- crates/jj-store-model/src/lib.rs
- crates/jj-store-model/src/portable.rs
- crates/jj-store-service/Cargo.toml
- crates/jj-store-service/src/collaboration.rs
- crates/jj-store-service/src/collaboration/review_models.rs
- crates/jj-store-service/src/collaboration/review_presentation.rs
- crates/jj-store-service/src/collaboration/review_threads.rs
- crates/jj-store-service/src/lib.rs
- crates/jj-store-service/src/local_repository.rs
- crates/jj-store-service/src/local_repository/actor.rs
- crates/jj-store-service/src/local_repository/agent_commands.rs
- crates/jj-store-service/src/local_repository/agent_descriptor.rs
- crates/jj-store-service/src/local_repository/agent_evidence.rs
- crates/jj-store-service/src/local_repository/agent_io.rs
- crates/jj-store-service/src/local_repository/agent_pages.rs
- crates/jj-store-service/src/local_repository/agent_provenance_pages.rs
- crates/jj-store-service/src/local_repository/agent_store.rs
- crates/jj-store-service/src/local_repository/errors.rs
- crates/jj-store-service/src/local_repository/records.rs
- crates/jj-store-service/src/local_repository/review_commands.rs
- crates/jj-store-service/src/local_repository/review_io.rs
- crates/jj-store-service/src/local_repository/review_store.rs
- crates/jj-store-service/src/local_repository/review_types.rs
- crates/jj-store-service/src/local_repository/work.rs
- crates/jj-store-service/src/local_repository/work_pages.rs
- crates/jj-store-service/src/work.rs
- crates/jj-store-service/src/work/browse.rs
- crates/jj-store-service/src/work/browse_models.rs
- crates/jj-store-service/src/work/cursors.rs
- crates/jj-store-service/src/work/reads.rs
- crates/jj-store-service/tests/local_agent.rs
- crates/jj-store-service/tests/local_agent_pages.rs
- crates/jj-store-service/tests/local_review.rs
- crates/jj-store-service/tests/local_work.rs
- crates/jj-store-service/tests/review_threads.rs
- crates/jj-store-service/tests/work_store.rs
- crates/jj-store-wal/src/publication.rs
- crates/jj-store-wal/src/publication/agent.rs
- crates/jj-store-wal/src/publication/agent_capture.rs
- crates/jj-store-wal/src/publication/agent_capture_projection.rs
- crates/jj-store-wal/src/publication/agent_capture_records.rs
- crates/jj-store-wal/src/publication/agent_capture_transcript.rs
- crates/jj-store-wal/src/publication/agent_catalog.rs
- crates/jj-store-wal/src/publication/agent_catalog_engine.rs
- crates/jj-store-wal/src/publication/agent_io.rs
- crates/jj-store-wal/src/publication/agent_provenance.rs
- crates/jj-store-wal/src/publication/agent_read_engine.rs
- crates/jj-store-wal/src/publication/agent_reads.rs
- crates/jj-store-wal/src/publication/replication_review.rs
- crates/jj-store-wal/src/publication/replication_review_graph.rs
- crates/jj-store-wal/src/publication/review.rs
- crates/jj-store-wal/src/publication/review_audit.rs
- crates/jj-store-wal/src/publication/review_audit_events.rs
- crates/jj-store-wal/src/publication/review_audit_relations.rs
- crates/jj-store-wal/src/publication/review_audit_replication.rs
- crates/jj-store-wal/src/publication/review_catalog.rs
- crates/jj-store-wal/src/publication/review_catalog_engine.rs
- crates/jj-store-wal/src/publication/review_command_apply.rs
- crates/jj-store-wal/src/publication/review_command_commit.rs
- crates/jj-store-wal/src/publication/review_command_history.rs
- crates/jj-store-wal/src/publication/review_command_relations.rs
- crates/jj-store-wal/src/publication/review_command_stage.rs
- crates/jj-store-wal/src/publication/review_command_update.rs
- crates/jj-store-wal/src/publication/review_commands.rs
- crates/jj-store-wal/src/publication/review_graph.rs
- crates/jj-store-wal/src/publication/review_io.rs
- crates/jj-store-wal/src/publication/review_projector.rs
- crates/jj-store-wal/src/publication/review_read_engine.rs
- crates/jj-store-wal/src/publication/review_reads.rs
- crates/jj-store-wal/tests/agent_provenance.rs
- crates/jj-store-wal/tests/review_projector.rs
- crates/jj-store-wal/tests/review_provider.rs
- crates/jj-store-wal/tests/support/review_provider.rs
- docs/cli/agent.md
- docs/cli/capabilities.md
- docs/cli/work.md
- package.json
- schemas/openapi.json
- scripts/dev-demo-work-samples.mjs
- scripts/dev-demo-work.mjs
- scripts/dev-demo-work.test.mjs
- scripts/dev-demo.mjs
- scripts/web-manifest.mjs
- scripts/web-manifest.test.mjs
- spec-validator.config.mjs
- spec/src/SUMMARY.md
- spec/src/api.md
- spec/src/architecture.md
- spec/src/cli.md
- spec/src/collaboration.md
- spec/src/compatibility.md
- spec/src/git-interop.md
- spec/src/identity.md
- spec/src/jj-integration.md
- spec/src/local-jj.md
- spec/src/milestones.md
- spec/src/mutations.md
- spec/src/projects.md
- spec/src/publication.md
- spec/src/spec-governance.md
- spec/src/ui.md
- spec/src/verification.md
- web/src/App.svelte
- web/src/lib/api/generated.ts
- web/src/lib/controller/contracts.ts
- web/src/lib/controller/http-repository-directory.ts
- web/src/lib/controller/http-work-data-source.ts
- web/src/lib/controller/production-projects-hub.svelte.ts
- web/src/lib/controller/production-projects-integration.ts
- web/src/lib/controller/production-route-restorer.ts
- web/src/lib/controller/production-store-actions.ts
- web/src/lib/controller/production-store-connect.ts
- web/src/lib/controller/production-store-controller.svelte.ts
- web/src/lib/controller/production-store-lifecycle.svelte.ts
- web/src/lib/controller/production-store-view.ts
- web/src/lib/controller/production-work.svelte.ts
- web/src/lib/controller/projects-home-read.ts
- web/src/lib/controller/projects-hub-types.ts
- web/src/lib/controller/projects-navigation.svelte.ts
- web/src/lib/controller/projects-repositories.svelte.ts
- web/src/lib/controller/projects-repository-types.ts
- web/src/lib/controller/projects-reviews.svelte.ts
- web/src/lib/controller/projects-route-codec.ts
- web/src/lib/controller/projects-view-cache.ts
- web/src/lib/controller/projects-work-registry.svelte.ts
- web/src/lib/controller/store-route-codec.ts
- web/src/lib/controller/store-route-history.svelte.ts
- web/src/lib/controller/work-types.ts
- web/src/lib/ui/community-navigation/CommunityNavigation.css
- web/src/lib/ui/community-navigation/CommunityNavigation.svelte
- web/src/lib/ui/community-navigation/CommunityNavigationTree.css
- web/src/lib/ui/community-navigation/icons.ts
- web/src/lib/ui/community-navigation/model.ts
- web/src/lib/ui/community-projects/CommunityProjectsRepositoryDocuments.svelte
- web/src/lib/ui/store-application/StoreApplication.svelte
- web/src/lib/ui/store-application/StoreConnection.svelte
- web/src/lib/ui/store-application/StoreRepositoryApplication.svelte
- web/src/lib/ui/store-application/StoreRepositoryPicker.svelte
- web/src/lib/ui/store-application/store-navigation.ts
- web/src/lib/ui/store-work/StoreProjects.svelte
- web/src/lib/ui/store-work/StoreProjectsBreadcrumb.svelte
- web/src/lib/ui/store-work/StoreProjectsDirectory.svelte
- web/src/lib/ui/store-work/StoreProjectsFilters.svelte
- web/src/lib/ui/store-work/StoreProjectsRail.svelte
- web/src/lib/ui/store-work/StoreProjectsRepositories.svelte
- web/src/lib/ui/store-work/StoreProjectsReviews.svelte
- web/src/lib/ui/store-work/StoreRepositoryActivity.svelte
- web/src/lib/ui/store-work/StoreRepositoryDirectoryRow.svelte
- web/src/lib/ui/store-work/StoreWork.svelte
- web/src/lib/ui/store-work/StoreWorkCollection.svelte
- web/src/lib/ui/store-work/StoreWorkComments.svelte
- web/src/lib/ui/store-work/StoreWorkDialog.svelte
- web/src/lib/ui/store-work/StoreWorkEmpty.svelte
- web/src/lib/ui/store-work/StoreWorkField.svelte
- web/src/lib/ui/store-work/StoreWorkLayoutControls.svelte
- web/src/lib/ui/store-work/StoreWorkMessageActions.svelte
- web/src/lib/ui/store-work/StoreWorkOwnershipGroups.svelte
- web/src/lib/ui/store-work/StoreWorkPicker.svelte
- web/src/lib/ui/store-work/StoreWorkProjectForm.svelte
- web/src/lib/ui/store-work/StoreWorkProjectRows.svelte
- web/src/lib/ui/store-work/StoreWorkRepositories.svelte
- web/src/lib/ui/store-work/StoreWorkReviewRow.svelte
- web/src/lib/ui/store-work/StoreWorkStateFilter.svelte
- web/src/lib/ui/store-work/StoreWorkStatus.svelte
- web/src/lib/ui/store-work/StoreWorkSubtasks.svelte
- web/src/lib/ui/store-work/StoreWorkTaskDetail.svelte
- web/src/lib/ui/store-work/StoreWorkTaskForm.svelte
- web/src/lib/ui/store-work/StoreWorkTaskRows.svelte
- web/src/lib/ui/store-work/StoreWorkTaskStateIcon.svelte
- web/src/lib/ui/store-work/community-work-breadcrumb.css
- web/src/lib/ui/store-work/community-work-cards.css
- web/src/lib/ui/store-work/community-work-collections.css
- web/src/lib/ui/store-work/community-work-directory.css
- web/src/lib/ui/store-work/community-work-disclosures.css
- web/src/lib/ui/store-work/community-work-forms.css
- web/src/lib/ui/store-work/community-work-layout.css
- web/src/lib/ui/store-work/community-work-reviews.css
- web/src/lib/ui/store-work/community-work-rows.css
- web/src/lib/ui/store-work/community-work-tasks.css
- web/src/lib/ui/store-work/community-work-thread.css
- web/src/lib/ui/store-work/projects-sidebar-model.ts
- web/src/lib/ui/store-work/repository-directory-presentation.ts
- web/src/lib/ui/store-work/store-projects.css
- web/src/lib/ui/store-work/store-work.css
- web/src/lib/ui/store-work/work-drafts.ts
- web/src/lib/ui/store-work/work-presentation.ts
- web/tests/real/work-seed.spec.ts
- web/tests/real/work-store.spec.ts
- web/tests/store-route-codec.test.ts
- web/tests/vite/production-store-bootstrap.test.ts
- web/tests/vite/production-work.test.ts
- web/tests/vite/projects-code-navigation.test.ts
- web/tests/vite/projects-directories.test.ts
- web/tests/vite/projects-hub.test.ts
- web/tests/vite/projects-repositories.test.ts
- web/tests/vite/store-route-history.test.ts
- web/tests/work-presentation.test.ts
- web/vite.config.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-service/tests/work_store.rs#work_pages_pin_roots_reauthorize_and_bind_query_without_publication
- crates/jj-store-service/tests/work_store.rs#work_browse_filters_complete_roots_and_pins_summary_counts
- crates/jj-store-service/tests/local_work.rs#local_pagination_remains_pinned_after_later_publications
- crates/jj-store-service/tests/local_review.rs#local_review_unknown_outcome_retries_exact_request_after_restart
- crates/jj-store-cli/tests/local_work.rs#jj_workspaces_share_local_actor_metadata_and_pinned_continuations
- crates/jj-store-jj/tests/local_metadata.rs#stock_jj_import_gc_and_undo_preserve_companion_metadata
- crates/jj-store-jj/tests/local_evidence.rs#capsule_retains_full_source_context_after_op_store_loss_and_git_gc
- crates/jj-store-wal/tests/review_projector.rs#independent_provider_preserves_native_command_bytes_and_pinned_stale_rules
- crates/jj-store-cli/tests/local_agent_provenance_cli.rs#executable_unknown_provenance_retries_without_source_operations_or_document
- crates/jj-store-cli/tests/executable_native.rs#custom_help_completion_build_info_and_ordinary_git_commands
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M13-012 details 2](details-02.md)
- [E-M13-012 details 3](details-03.md)
