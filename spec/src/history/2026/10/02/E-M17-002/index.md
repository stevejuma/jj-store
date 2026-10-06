---
schema: 1
id: E-M17-002
---

# E-M17-002

<!-- evidence-field:id markdown -->
## Id

E-M17-002
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M17
- M17.1
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-02
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

capture-publication-regressions
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-agent/src/activity.rs
- crates/jj-agent/src/capture.rs
- crates/jj-agent/src/reducer.rs
- crates/jj-store-wal/src/publication.rs
- crates/jj-store-wal/src/publication/agent_capture.rs
- crates/jj-store-wal/src/publication/agent_capture_records.rs
- crates/jj-store-wal/src/publication/agent_capture_tools.rs
- crates/jj-store-wal/tests/agent_catalog.rs
- crates/jj-store-service/tests/local_agent.rs
- spec/src/collaboration.md
- spec/src/publication.md
- spec/src/local-jj.md
- spec/src/mutations.md
- spec/src/milestones.md
- spec/src/verification.md
- crates/jj-store-jj/src/local_browser/operations.rs
- spec/src/jj-integration.md
- crates/jj-store-service/src/content_search/extract.rs
- spec/src/adapters.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-wal/tests/agent_catalog.rs#diagnostic_only_capture_retains_evidence_and_allows_later_source_progress
- crates/jj-store-wal/tests/agent_catalog.rs#separately_published_tool_updates_preserve_content_terminal_state_and_conflict_coverage
- crates/jj-store-service/tests/local_agent.rs#diagnostic_and_partial_tool_captures_survive_local_restart_without_state_loss
- crates/jj-store-wal/tests/agent_catalog.rs#diagnostic_only_capture_rejects_unsupported_normalizer_profile_without_selection
- crates/jj-store-service/tests/content_search.rs#current_full_bodies_edits_tombstones_and_scopes_are_manifest_selected
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M17-002 details 2](details-02.md)
- [E-M17-002 details 3](details-03.md)
- [E-M17-002 details 4](details-04.md)
