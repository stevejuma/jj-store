---
schema: 1
id: E-M13-008
---

# E-M13-008

<!-- evidence-field:id markdown -->
## Id

E-M13-008
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

zyryzwyz
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/src/agent_commands/provenance.rs
- crates/jj-store-cli/src/agent_commands/types.rs
- crates/jj-store-cli/src/agent_commands.rs
- crates/jj-store-cli/src/agent_provenance_journal.rs
- crates/jj-store-cli/src/lib.rs
- crates/jj-store-cli/src/local_diagnostics.rs
- crates/jj-store-cli/tests/local_agent.rs
- crates/jj-store-cli/tests/local_agent_provenance_cli.rs
- crates/jj-store-service/src/local_repository/agent_provenance_pages.rs
- crates/jj-store-service/src/local_repository.rs
- docs/cli/agent.md
- spec/src/cli.md
- spec/src/local-jj.md
- spec/src/mutations.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/local_agent_provenance_cli.rs#executable_provenance_pages_bind_snapshots_and_corrections
- crates/jj-store-cli/tests/local_agent_provenance_cli.rs#executable_unknown_provenance_retries_without_source_operations_or_document
- crates/jj-store-cli/tests/local_agent_provenance_cli.rs#executable_empty_provenance_pages_leave_metadata_and_actor_absent
- crates/jj-store-cli/tests/local_agent_provenance_cli.rs#executable_checkpoint_retains_exact_session_and_association_versions
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M13-008 details 2](details-02.md)
- [E-M13-008 details 3](details-03.md)
