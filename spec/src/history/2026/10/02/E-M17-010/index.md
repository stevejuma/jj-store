---
schema: 1
id: E-M17-010
---

# E-M17-010

<!-- evidence-field:id markdown -->
## Id

E-M17-010
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M17
- M17.2
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-02
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

native-codex-tool-result-resolution
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-agent/src/lib.rs
- crates/jj-agent/src/normalization.rs
- crates/jj-agent/src/normalize.rs
- crates/jj-agent/src/normalizers/codex.rs
- crates/jj-agent/src/normalizers/codex_ids.rs
- crates/jj-agent/src/normalizers/codex_tool_result.rs
- crates/jj-agent/src/normalizers/codex_transcript.rs
- crates/jj-agent/src/normalizers/mod.rs
- crates/jj-agent/src/tool_result.rs
- crates/jj-agent/tests/codex_transcript.rs
- crates/jj-store-service/src/content_search/cursors.rs
- crates/jj-store-service/tests/local_agent/native_results.rs
- crates/jj-store-service/tests/local_agent.rs
- crates/jj-store-wal/src/publication/agent_capture.rs
- crates/jj-store-wal/src/publication/agent_capture_results.rs
- crates/jj-store-wal/src/publication.rs
- crates/jj-store-wal/tests/agent_catalog/native_results.rs
- crates/jj-store-wal/tests/agent_catalog.rs
- spec/src/collaboration.md
- spec/src/compatibility.md
- spec/src/local-jj.md
- spec/src/milestones.md
- spec/src/mutations.md
- spec/src/publication.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-agent/tests/codex_transcript.rs#native_tool_results_enrich_known_calls_without_claiming_success
- crates/jj-agent/tests/codex_transcript.rs#malformed_results_and_foreign_sessions_cannot_invent_calls_or_block_later_records
- crates/jj-agent/tests/codex_transcript.rs#native_result_candidates_are_transient_and_reject_wrong_resolution_identity
- crates/jj-store-wal/tests/agent_catalog/native_results.rs#result_only_capture_enriches_selected_call_and_preserves_old_root
- crates/jj-store-wal/tests/agent_catalog/native_results.rs#unmatched_results_keep_diagnostics_without_adopting_foreign_calls
- crates/jj-store-wal/tests/agent_catalog/native_results.rs#native_results_preserve_selected_terminal_evidence_and_turn_identity
- crates/jj-store-service/tests/local_agent/native_results.rs#local_native_result_enrichment_retains_selected_calls_receipts_and_pins_across_reopen
- crates/jj-store-service/src/content_search/cursors.rs#capacity_restart_and_query_deadline_fail_closed_without_eviction
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M17-010 details 2](details-02.md)
- [E-M17-010 details 3](details-03.md)
- [E-M17-010 details 4](details-04.md)
