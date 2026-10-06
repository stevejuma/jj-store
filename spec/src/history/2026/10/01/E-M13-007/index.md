---
schema: 1
id: E-M13-007
---

# E-M13-007

<!-- evidence-field:id markdown -->
## Id

E-M13-007
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

filesystem-git
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

nxwrlsnv
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-agent/src/indexes.rs
- crates/jj-agent/src/objects.rs
- crates/jj-agent/tests/portable.rs
- crates/jj-store-cli/tests/local_agent_provenance/fixture.rs
- crates/jj-store-cli/tests/local_agent_provenance.rs
- crates/jj-store-jj/src/local_evidence/capture.rs
- crates/jj-store-jj/tests/local_evidence.rs
- crates/jj-store-service/src/local_repository/agent_commands.rs
- crates/jj-store-service/src/local_repository/agent_descriptor.rs
- crates/jj-store-service/src/local_repository/agent_evidence.rs
- crates/jj-store-service/src/local_repository/agent_io.rs
- crates/jj-store-service/src/local_repository/agent_store.rs
- crates/jj-store-service/src/local_repository.rs
- crates/jj-store-wal/src/publication/agent.rs
- crates/jj-store-wal/src/publication/agent_provenance.rs
- crates/jj-store-wal/src/publication.rs
- crates/jj-store-wal/tests/agent_provenance.rs
- spec/src/cli.md
- spec/src/collaboration.md
- spec/src/jj-integration.md
- spec/src/local-jj.md
- spec/src/mutations.md
- spec/src/publication.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/local_agent_provenance.rs#portable_association_retains_exact_code_after_restart_op_loss_and_git_gc
- crates/jj-store-cli/tests/local_agent_provenance.rs#provenance_corrections_are_stale_safe_and_replay_original_receipts
- crates/jj-store-cli/tests/local_agent_provenance.rs#invalid_capsules_commits_sessions_and_dependencies_never_publish
- crates/jj-store-cli/tests/local_agent_provenance.rs#portable_checkpoint_pins_agent_versions_and_independent_outcomes
- crates/jj-store-cli/tests/local_agent_provenance.rs#concurrent_disjoint_provenance_preserves_shared_relation_history
- crates/jj-store-cli/tests/local_agent_provenance.rs#provenance_unknown_outcome_restarts_with_exact_original_request
- crates/jj-store-cli/tests/local_agent_provenance.rs#original_local_agent_descriptor_remains_readable_before_v2_publication
- crates/jj-store-wal/tests/agent_provenance.rs#shared_native_provenance_projects_stale_safe_catalog_and_relations
- crates/jj-store-jj/tests/local_evidence.rs#retained_capsule_blob_without_its_code_closure_is_rejected
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M13-007 details 2](details-02.md)
- [E-M13-007 details 3](details-03.md)
