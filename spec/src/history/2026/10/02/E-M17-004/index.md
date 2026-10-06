---
schema: 1
id: E-M17-004
---

# E-M17-004

<!-- evidence-field:id markdown -->
## Id

E-M17-004
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

bound-diagnostic-gap
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-collab/src/capture_binding.rs
- crates/jj-collab/src/commands.rs
- crates/jj-collab/src/objects.rs
- crates/jj-collab/src/threads.rs
- crates/jj-store-cli/tests/agent_capture_publication.rs
- crates/jj-store-cli/tests/native_lifecycle/http_threads_reactions.rs
- crates/jj-store-rpc/tests/collab.rs
- crates/jj-store-service/src/local_repository/agent_thread_binding.rs
- crates/jj-store-service/src/local_repository/collab_commands.rs
- crates/jj-store-service/tests/local_agent_binding.rs
- crates/jj-store-wal/src/publication.rs
- crates/jj-store-wal/src/publication/agent_capture_binding.rs
- crates/jj-store-wal/src/publication/agent_read_engine.rs
- crates/jj-store-wal/src/publication/agent_thread_binding.rs
- crates/jj-store-wal/src/publication/collab_commands.rs
- crates/jj-store-wal/tests/bound_capture.rs
- schemas/openapi.json
- spec/src/api.md
- spec/src/cli.md
- spec/src/collaboration.md
- spec/src/compatibility.md
- spec/src/jj-integration.md
- spec/src/local-jj.md
- spec/src/milestones.md
- spec/src/mutations.md
- spec/src/publication.md
- spec/src/ui.md
- spec/src/verification.md
- web/src/lib/api/generated.ts
- web/src/lib/ui/store-threads/thread-context.ts
- web/tests/vite/thread-context.test.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-service/tests/local_agent_binding.rs#bound_diagnostic_capture_retains_thread_intent_without_inventing_session
- crates/jj-store-wal/tests/bound_capture.rs#bound_diagnostic_gap_selects_atomic_evidence_and_preserves_unknown_restart_receipt
- crates/jj-store-wal/tests/bound_capture.rs#stale_denied_or_misleading_bound_gap_selects_neither_root
- crates/jj-store-rpc/tests/collab.rs#diagnostic_gap_binding_survives_preparation_publication_and_transport_reconstruction
- web/tests/vite/thread-context.test.ts#capture gaps open the agent directory
  without a fictional session or review
- crates/jj-store-cli/tests/agent_capture_publication.rs#bound_diagnostic_gap_drains_after_restart_without_a_session_edit
- crates/jj-store-service/tests/local_agent_binding.rs#bound_gap_with_observed_assignment_preserves_the_real_session_and_rejects_stale_intent
- crates/jj-store-cli/tests/native_lifecycle/http_threads_reactions.rs#http_threads_reactions_keep_exact_targets_assets_signed_roots_and_receipts
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M17-004 details 2](details-02.md)
- [E-M17-004 details 3](details-03.md)
- [E-M17-004 details 4](details-04.md)
