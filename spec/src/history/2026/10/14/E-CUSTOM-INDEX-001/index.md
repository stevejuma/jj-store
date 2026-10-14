---
schema: 1
id: E-CUSTOM-INDEX-001
---

# E-CUSTOM-INDEX-001

<!-- evidence-field:id markdown -->
## Id

E-CUSTOM-INDEX-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M29
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-ADAPT-008
- JST-WF-006
- JST-WORK-005
- JST-WORK-009
- JST-COLLAB-004
- JST-API-009
- JST-CLI-004
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-14
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

nxyuuzpy
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- Cargo.lock
- Cargo.toml
- crates/jj-collab/src/commands.rs
- crates/jj-collab/src/conversation_metadata.rs
- crates/jj-collab/src/conversation_submission.rs
- crates/jj-collab/src/objects.rs
- crates/jj-collab/src/resolver.rs
- crates/jj-collab/src/threads.rs
- crates/jj-collab/tests/target_resolution.rs
- crates/jj-store-http/src/application_events/authorization.rs
- crates/jj-store-http/src/application_events/code_indexes.rs
- crates/jj-store-http/src/application_events/monitors.rs
- crates/jj-store-http/src/contract/tests.rs
- crates/jj-store-jj/src/local_browser/graphs.rs
- crates/jj-store-jj/src/local_browser/operation_store.rs
- crates/jj-store-jj/src/local_browser/preparation/comparison.rs
- crates/jj-store-jj/src/local_browser/references/prepared.rs
- crates/jj-store-jj/src/local_browser/scoped.rs
- crates/jj-store-jj/src/local_metadata/evidence_files.rs
- crates/jj-store-jj/src/local_stacks/conflicts.rs
- crates/jj-store-jj/src/local_stacks/hunks.rs
- crates/jj-store-jj/src/local_stacks/mod.rs
- crates/jj-store-jj/src/local_stacks/mutations.rs
- crates/jj-store-jj/src/local_stacks/policy.rs
- crates/jj-store-jj/src/local_stacks/preview.rs
- crates/jj-store-jj/src/local_stacks/topology.rs
- crates/jj-store-jj/src/query/blame.rs
- crates/jj-store-jj/src/query/content/native.rs
- crates/jj-store-jj/src/query/content/references/filter.rs
- crates/jj-store-jj/src/query/content/references/prepared.rs
- crates/jj-store-jj/src/query/graphs.rs
- crates/jj-store-jj/src/query/index/secondary.rs
- crates/jj-store-jj/src/query/index.rs
- crates/jj-store-jj/src/query/mod.rs
- crates/jj-store-jj/src/query/prepared/filters.rs
- crates/jj-store-jj/src/query/prepared/operations/history/evolution.rs
- crates/jj-store-jj/src/query/prepared/operations/history/predecessors.rs
- crates/jj-store-jj/src/query/prepared/tables.rs
- crates/jj-store-jj/src/query/projection.rs
- crates/jj-store-jj/tests/local_stacks.rs
- crates/jj-store-read-index/Cargo.toml
- crates/jj-store-read-index/src/codec.rs
- crates/jj-store-read-index/src/index.rs
- crates/jj-store-read-index/src/lib.rs
- crates/jj-store-read-index/src/storage.rs
- crates/jj-store-read-index/tests/costs.rs
- crates/jj-store-read-index/tests/persistence.rs
- crates/jj-store-read-index/tests/queries.rs
- crates/jj-store-server/src/agent_administration.rs
- crates/jj-store-server/src/agent_execution/chatgpt/callback.rs
- crates/jj-store-server/src/agent_execution/comment_answers.rs
- crates/jj-store-server/src/agent_execution/compute/mod.rs
- crates/jj-store-server/src/agent_execution/compute/process.rs
- crates/jj-store-server/src/agent_execution/configuration.rs
- crates/jj-store-server/src/agent_execution/conversations.rs
- crates/jj-store-server/src/agent_execution/delivery.rs
- crates/jj-store-server/src/agent_execution/feedback.rs
- crates/jj-store-server/src/agent_execution/metadata_jobs.rs
- crates/jj-store-server/src/agent_execution/mod.rs
- crates/jj-store-server/src/agent_execution/provider_launch.rs
- crates/jj-store-server/src/agent_execution/thread_workspace.rs
- crates/jj-store-server/src/agent_execution/workspace_inspection/comparison.rs
- crates/jj-store-server/src/agent_execution/workspace_inspection/content.rs
- crates/jj-store-server/src/agent_execution/workspace_inspection/source.rs
- crates/jj-store-server/src/agent_execution/workspace_inspection.rs
- crates/jj-store-server/src/agent_execution/workspace_turns.rs
- crates/jj-store-server/src/config/types.rs
- crates/jj-store-server/src/config.rs
- crates/jj-store-server/src/directory/audit.rs
- crates/jj-store-server/src/installation.rs
- crates/jj-store-server/src/process/capture.rs
- crates/jj-store-server/src/process/preparation.rs
- crates/jj-store-server/src/process/reload_capture.rs
- crates/jj-store-server/src/remote_agents/mod.rs
- crates/jj-store-server/src/runtime_reload/tests.rs
- crates/jj-store-server/tests/capture_management/server_settings.rs
- crates/jj-store-server/tests/configuration.rs
- crates/jj-store-server/tests/custom_data_indexes.rs
- crates/jj-store-service/Cargo.toml
- crates/jj-store-service/src/administration.rs
- crates/jj-store-service/src/agent_execution/context_submission.rs
- crates/jj-store-service/src/agent_feedback/source_parts.rs
- crates/jj-store-service/src/agent_feedback.rs
- crates/jj-store-service/src/collab_reads/materialization.rs
- crates/jj-store-service/src/collab_reads.rs
- crates/jj-store-service/src/collaboration/review_threads.rs
- crates/jj-store-service/src/collaboration.rs
- crates/jj-store-service/src/content_search/index.rs
- crates/jj-store-service/src/content_search/materialization.rs
- crates/jj-store-service/src/content_search.rs
- crates/jj-store-service/src/lib.rs
- crates/jj-store-service/src/local_repository/browser_collab.rs
- crates/jj-store-service/src/local_repository/collab_commands.rs
- crates/jj-store-service/src/native.rs
- crates/jj-store-service/src/native_inventory/cursors.rs
- crates/jj-store-service/src/native_inventory/families/cache.rs
- crates/jj-store-service/src/read_indexes.rs
- crates/jj-store-service/src/server_settings.rs
- crates/jj-store-service/src/transport.rs
- crates/jj-store-service/src/work/indexes.rs
- crates/jj-store-service/src/work/projection.rs
- crates/jj-store-service/src/work/reads.rs
- crates/jj-store-service/src/work.rs
- crates/jj-store-service/src/workflow/projection.rs
- crates/jj-store-service/src/workflow/reads.rs
- crates/jj-store-service/src/workflow.rs
- crates/jj-store-service/tests/workflow_store/index.rs
- crates/jj-store-service/tests/workflow_store.rs
- crates/jj-store-wal/src/publication/collab_commands.rs
- crates/jj-store-wal/src/publication/workflow.rs
- crates/jj-store-wal/tests/workflow_store.rs
- docs/maintenance/coverage.json
- docs/site/reference/configuration.md
- docs/site/technical/architecture.md
- docs/site/technical/publication.md
- docs/site/technical/reads.md
- scripts/check-boundaries.mjs
- spec/src/current/contracts/adapters/03-requirements.md
- spec/src/current/contracts/adapters/12-custom-data-read-indexes.md
- spec/src/current/contracts/adapters/SUMMARY.nav.md
- spec/src/current/contracts/adapters/index.md
- spec/src/current/contracts/architecture/22-custom-data-indexes.md
- spec/src/current/contracts/architecture/SUMMARY.nav.md
- spec/src/current/contracts/architecture/index.md
- spec/src/current/contracts/cli/39-custom-data-indexes.md
- spec/src/current/contracts/cli/SUMMARY.nav.md
- spec/src/current/contracts/cli/index.md
- spec/src/current/contracts/collaboration/25-root-pinned-review-store.md
- spec/src/current/contracts/documentation/10-workflows.md
- spec/src/current/contracts/mutations/25-custom-data-indexes.md
- spec/src/current/contracts/mutations/SUMMARY.nav.md
- spec/src/current/contracts/mutations/index.md
- spec/src/current/contracts/publication/09-workflowstore.md
- spec/src/current/governance/spec-governance.md
- spec/src/current/progress/M29/SUMMARY.nav.md
- spec/src/current/progress/M29/index.md
- spec/src/current/progress/SUMMARY.nav.md
- spec/src/current/progress/index.md
- spec/src/current/verification/SUMMARY.nav.md
- spec/src/current/verification/custom-data-indexes.md
- spec/src/current/verification/index.md
- spec-validator.config.mjs
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-service/tests/workflow_store/index.rs#persistent_generations_share_requests_but_require_cold_source_admission
- crates/jj-store-read-index/tests/queries.rs#incremental_replacements_removals_and_compaction_match_full_build
- crates/jj-store-read-index/tests/queries.rs#trigrams_verify_order_and_field_boundaries
- crates/jj-store-read-index/tests/persistence.rs#reopen_uses_exact_admitted_facts_and_changed_facts_rebuild
- crates/jj-store-read-index/tests/persistence.rs#generation_retention_is_bounded_without_touching_unowned_files
- crates/jj-store-read-index/tests/costs.rs#measured_scoped_query_costs_at_small_medium_and_near_limit_sizes
- crates/jj-store-wal/tests/workflow_store.rs#bounded_workflow_source_admission_preserves_native_state_and_never_publishes
- crates/jj-store-server/tests/custom_data_indexes.rs#custom_indexes_preserve_pinned_history_after_real_storage_restart_and_cache_loss
<!-- /evidence-field -->
<!-- generated-navigation -->

## Pages

- [E-CUSTOM-INDEX-001 details 2](details-02.md)
- [E-CUSTOM-INDEX-001 details 3](details-03.md)
- [E-CUSTOM-INDEX-001 details 4](details-04.md)
