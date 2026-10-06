---
schema: 1
id: E-M13-002
---

# E-M13-002

<!-- evidence-field:id markdown -->
## Id

E-M13-002
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M13
- M13.2
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-01
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

filesystem-git
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

uklylvot
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- Cargo.lock
- crates/jj-store-cli/src/commands.rs
- crates/jj-store-cli/src/lib.rs
- crates/jj-store-cli/src/local_diagnostics.rs
- crates/jj-store-cli/src/store_commands.rs
- crates/jj-store-cli/src/work_commands/client.rs
- crates/jj-store-cli/src/work_commands/output.rs
- crates/jj-store-cli/src/work_commands/task.rs
- crates/jj-store-cli/src/work_commands.rs
- crates/jj-store-cli/tests/local_work.rs
- crates/jj-store-jj/src/lib.rs
- crates/jj-store-jj/src/local_metadata/types.rs
- crates/jj-store-jj/src/local_repository.rs
- crates/jj-store-service/Cargo.toml
- crates/jj-store-service/src/lib.rs
- crates/jj-store-service/src/local_repository/actor.rs
- crates/jj-store-service/src/local_repository/work.rs
- crates/jj-store-service/src/local_repository/work_pages.rs
- crates/jj-store-service/src/local_repository.rs
- crates/jj-store-service/src/work/reads.rs
- crates/jj-store-service/src/work.rs
- crates/jj-store-service/tests/local_work.rs
- docs/cli/capabilities.md
- docs/cli/work.md
- spec/src/architecture.md
- spec/src/cli.md
- spec/src/jj-integration.md
- spec/src/local-jj.md
- spec/src/milestones.md
- spec/src/mutations.md
- spec/src/projects.md
- spec/src/verification.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-service/tests/local_work.rs#local_reads_do_not_initialize_metadata_or_actor_state
- crates/jj-store-service/tests/local_work.rs#local_commands_replay_original_results_after_reconstruction_and_git_gc
- crates/jj-store-service/tests/local_work.rs#local_pagination_remains_pinned_after_later_publications
- crates/jj-store-service/tests/local_work.rs#local_task_hierarchy_and_stale_edits_use_shared_domain_rules
- crates/jj-store-service/tests/local_work.rs#local_threaded_comments_keep_edits_and_tombstones_in_immutable_history
- crates/jj-store-service/tests/local_work.rs#invalid_first_commands_leave_the_repository_uninitialized
- crates/jj-store-service/tests/local_work.rs#forged_continuations_cannot_read_staged_metadata_candidates
- crates/jj-store-service/tests/local_work.rs#concurrent_local_writers_share_actor_and_preserve_disjoint_projects
- crates/jj-store-cli/tests/local_work.rs#project_task_comment_commands_work_without_native_connection_or_server
- crates/jj-store-cli/tests/local_work.rs#jj_workspaces_share_local_actor_metadata_and_pinned_continuations
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M13-002 details 2](details-02.md)
- [E-M13-002 details 3](details-03.md)
