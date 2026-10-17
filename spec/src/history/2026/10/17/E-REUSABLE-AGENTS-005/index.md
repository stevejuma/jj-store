---
schema: 1
id: E-REUSABLE-AGENTS-005
---

# E-REUSABLE-AGENTS-005

<!-- evidence-field:id markdown -->
## Id

E-REUSABLE-AGENTS-005
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M33
- M33.3
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-ARCH-005
- JST-ARCH-007
- JST-COLLAB-031
- JST-COMPAT-001
- JST-DOC-001
- JST-GOV-011
- JST-GOV-002
- JST-CLI-004
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-17
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- .changeset/injected-prompt-coordination.md
- Cargo.lock
- crates/jj-store-conversations/Cargo.toml
- crates/jj-store-conversations/README.md
- crates/jj-store-conversations/src/coordination.rs
- crates/jj-store-conversations/src/lib.rs
- crates/jj-store-conversations/tests/coordination.rs
- crates/jj-store-server/src/agent_execution/conversations.rs
- crates/jj-store-server/src/agent_execution/mod.rs
- crates/jj-store-server/src/agent_execution/turn_coordination.rs
- docs/local-compute.md
- docs/maintenance/coverage.json
- docs/site/technical/conversations.md
- spec/src/current/contracts/architecture/23-public-distribution.md
- spec/src/current/contracts/architecture/26-reusable-agent-workflows.md
- spec/src/current/contracts/cli/35-interactive-agent-execution.md
- spec/src/current/contracts/compatibility/13-interactive-agent-execution.md
- spec/src/current/contracts/documentation/08-interactive-agent-execution.md
- spec/src/current/progress/M33/history/M33/index.md
- spec/src/current/progress/M33/history/M33.3/index.md
- spec/src/current/progress/M33/index.md
- spec/src/history/2026/10/17/E-REUSABLE-AGENTS-005/SUMMARY.nav.md
- spec/src/history/2026/10/17/E-REUSABLE-AGENTS-005/details-02.md
- spec/src/history/2026/10/17/E-REUSABLE-AGENTS-005/details-03.md
- spec/src/history/2026/10/17/E-REUSABLE-AGENTS-005/index.md
- spec/src/history/2026/10/17/SUMMARY.nav.md
- spec/src/history/2026/10/17/index.md
<!-- /evidence-field -->

<!-- evidence-field:removed_files list -->
## Removed files

[]
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-conversations/tests/coordination.rs#every_context_and_durable_prepared_fence_precedes_first_dispatch
- crates/jj-store-conversations/tests/coordination.rs#context_failure_releases_prior_reservations_without_any_dispatch_or_replay
- crates/jj-store-conversations/tests/coordination.rs#unknown_delivery_is_retained_and_only_proven_not_sent_releases
- crates/jj-store-conversations/tests/coordination.rs#journal_failure_before_prepared_blocks_dispatch_and_recovery_returns_retained_intent
- crates/jj-store-conversations/tests/coordination.rs#retained_intents_recheck_admission_and_reject_changed_request_contents
- crates/jj-store-cli/tests/agent_execution/conversations.rs#repository_free_chat_reuses_isolated_bindings_and_never_replays_after_restart
<!-- /evidence-field -->

