---
schema: 1
id: E-CUSTOM-INDEX-LINT-001
---

# E-CUSTOM-INDEX-LINT-001

<!-- evidence-field:id markdown -->
## Id

E-CUSTOM-INDEX-LINT-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M29
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-JJ-003
- JST-JJ-005
- JST-LOCAL-003
- JST-LOCAL-004
- JST-LOCAL-005
- JST-COLLAB-004
- JST-COLLAB-007
- JST-API-003
- JST-API-006
- JST-API-011
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
- crates/jj-store-server/src/directory/audit.rs
- crates/jj-store-server/src/process/capture.rs
- crates/jj-store-server/src/process/preparation.rs
- crates/jj-store-server/src/process/reload_capture.rs
- crates/jj-store-server/src/remote_agents/mod.rs
- crates/jj-store-server/tests/capture_management/server_settings.rs
- crates/jj-store-service/src/agent_execution/context_submission.rs
- crates/jj-store-service/src/agent_feedback/source_parts.rs
- crates/jj-store-service/src/agent_feedback.rs
- crates/jj-store-service/src/collab_reads.rs
- crates/jj-store-service/src/local_repository/browser_collab.rs
- crates/jj-store-service/src/local_repository/collab_commands.rs
- crates/jj-store-service/src/native_inventory/cursors.rs
- crates/jj-store-service/src/native_inventory/families/cache.rs
- crates/jj-store-service/src/server_settings.rs
- crates/jj-store-wal/src/publication/collab_commands.rs
- crates/jj-store-jj/tests/local_stacks.rs
- crates/jj-store-http/src/contract/tests.rs
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-collab/tests/domain.rs#canonical_codec_rejects_unknown_fields_whitespace_and_wrong_kinds
- crates/jj-store-jj/tests/compatibility.rs#explicit_bounded_reads_preserve_divergent_heads
<!-- /evidence-field -->
<!-- generated-navigation -->

## Pages

- [E-CUSTOM-INDEX-LINT-001 details 2](details-02.md)
- [E-CUSTOM-INDEX-LINT-001 details 3](details-03.md)
