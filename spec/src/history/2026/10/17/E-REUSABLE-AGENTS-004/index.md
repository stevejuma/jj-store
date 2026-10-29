---
schema: 1
id: E-REUSABLE-AGENTS-004
---

# E-REUSABLE-AGENTS-004

<!-- evidence-field:id markdown -->
## Id

E-REUSABLE-AGENTS-004
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

- .changeset/shared-conversation-stream.md
- Cargo.lock
- Cargo.toml
- crates/jj-store-conversations/Cargo.toml
- crates/jj-store-conversations/LICENSE.md
- crates/jj-store-conversations/NOTICE.md
- crates/jj-store-conversations/README.md
- crates/jj-store-conversations/src/control_v3.rs
- crates/jj-store-conversations/src/lib.rs
- crates/jj-store-conversations/src/privacy.rs
- crates/jj-store-conversations/tests/control_v3.rs
- crates/jj-store-server/Cargo.toml
- crates/jj-store-server/src/agent_execution/conversation_stream/tests.rs
- crates/jj-store-server/src/agent_execution/conversation_stream.rs
- crates/jj-store-server/src/agent_execution/runtime_privacy/tests.rs
- crates/jj-store-server/src/agent_execution/runtime_privacy.rs
- docs/local-compute.md
- docs/maintenance/coverage.json
- docs/site/technical/conversations.md
- mkdocs.yml
- package.json
- scripts/check-boundaries.mjs
- scripts/ci-selection.test.mjs
- scripts/conversations-features.test.mjs
- spec/src/current/contracts/architecture/23-public-distribution.md
- spec/src/current/contracts/architecture/26-reusable-agent-workflows.md
- spec/src/current/contracts/cli/35-interactive-agent-execution.md
- spec/src/current/contracts/compatibility/13-interactive-agent-execution.md
- spec/src/current/contracts/documentation/08-interactive-agent-execution.md
- spec/src/current/governance/23-shared-ai-clients.md
- spec/src/current/governance/spec-governance.md
- spec/src/current/progress/M33/history/M33/index.md
- spec/src/current/progress/M33/history/M33.3/index.md
- spec/src/current/progress/M33/index.md
- spec/src/history/2026/10/17/E-REUSABLE-AGENTS-004/SUMMARY.nav.md
- spec/src/history/2026/10/17/E-REUSABLE-AGENTS-004/details-02.md
- spec/src/history/2026/10/17/E-REUSABLE-AGENTS-004/details-03.md
- spec/src/history/2026/10/17/E-REUSABLE-AGENTS-004/index.md
- spec/src/history/2026/10/17/SUMMARY.nav.md
- spec/src/history/2026/10/17/index.md
- spec-validator.config.mjs
<!-- /evidence-field -->

<!-- evidence-field:removed_files list -->
## Removed files

[]
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-conversations/tests/control_v3.rs#injected_privacy_flushes_before_boundaries_and_every_emitted_frame_is_scanned
- crates/jj-store-conversations/tests/control_v3.rs#participants_keep_separate_assistant_and_tool_snapshots
- crates/jj-store-conversations/tests/control_v3.rs#participation_capacity_refuses_new_ids_without_blocking_an_existing_participant
- crates/jj-store-server/src/agent_execution/conversation_stream/tests.rs#fleet_privacy_isolated_by_participation_preserves_terminal_assistant_text
- crates/jj-store-server/src/agent_execution/runtime_privacy/tests.rs#streamed_privacy_precedes_journal_and_terminal_frames
<!-- /evidence-field -->

