---
schema: 1
id: E-AGENT-RESUME-001
---

# E-AGENT-RESUME-001

<!-- evidence-field:id markdown -->
## Id

E-AGENT-RESUME-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M24
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-COLLAB-034
- JST-COLLAB-035
- JST-COLLAB-036
- JST-UI-015
- JST-UI-018
- JST-API-001
- JST-API-006
- JST-CLI-004
- JST-GOV-002
- JST-GOV-008
- JST-DOC-001
- JST-DOC-002
- JST-DOC-003
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-10
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-application/AgentChatRuntime.stories.svelte
- catalog/fixtures/agent-command-receipts.ts
- crates/jj-store-cli/tests/agent_execution/conversation_failures.rs
- crates/jj-store-server/src/agent_execution/conversation_frames.rs
- crates/jj-store-server/src/agent_execution/fleet.rs
- crates/jj-store-server/src/agent_execution/mod.rs
- crates/jj-store-server/src/agent_execution/participation_resume.rs
- docs/maintenance/coverage.json
- docs/site/guides/agent-execution.md
- docs/site/guides/settings.md
- docs/site/technical/lifecycle.md
- scripts/dev-agent-probe.mjs
- scripts/dev-agent-runtime.mjs
- scripts/dev-agent-runtime.test.mjs
- scripts/dev-ui.test.mjs
- spec/src/current/contracts/api/41-interactive-agent-execution.md
- spec/src/current/contracts/cli/35-interactive-agent-execution.md
- spec/src/current/contracts/collaboration/37-reusable-agent-conversations.md
- spec/src/current/contracts/documentation/01-application-documentation.md
- spec/src/current/contracts/ui/37-document-conversations.md
- spec/src/current/governance/spec-governance.md
- spec/src/current/progress/M24/footer-review.md
- spec/src/current/progress/M24/index.md
- web/src/lib/controller/agent-execution-refresh.ts
- web/src/lib/controller/agent-execution.svelte.ts
- web/src/lib/controller/agent-runtime-observation.ts
- web/src/lib/controller/agent-selection-history.ts
- web/src/lib/conversations/prepare-standalone-runtime.ts
- web/src/lib/conversations/standalone-send.svelte.ts
- web/src/lib/ui/store-agent-execution/StoreAgentConversation.svelte
- web/src/lib/ui/store-agent-execution/conversation-error.ts
- web/src/lib/ui/store-agent-execution/runtime-preparation.ts
- web/tests/vite/agent-freshness.test.ts
- web/tests/vite/agent-runtime-preparation.test.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- scripts/dev-agent-runtime.test.mjs#version 3 alone does not qualify a helper
  that rejects prepared context
- web/tests/vite/agent-freshness.test.ts#publication refresh retains visible
  messages until replacement history arrives
- web/tests/vite/agent-freshness.test.ts#failed publication recapture retries
  authority while retaining old rows
- web/tests/vite/agent-runtime-preparation.test.ts#Send prepares a disconnected
  session once and observes readiness without dispatching a prompt
- web/tests/vite/agent-runtime-preparation.test.ts#changing selection during
  opening does not retarget the captured submission
- crates/jj-store-cli/tests/agent_execution/conversation_failures.rs#resume_without_an_acknowledged_turn_creates_context_session_without_loading
- crates/jj-store-cli/tests/agent_execution/conversation_failures.rs#rejected_resume_finishes_receipt_preserves_worker_and_never_replays
- crates/jj-store-cli/tests/agent_execution/conversation_failures.rs#rejected_resume_configuration_settles_its_correlated_receipt
- crates/jj-store-cli/tests/agent_execution/conversation_failures.rs#stop_during_resume_cancels_opening_without_resurrection_or_prompt_replay
<!-- /evidence-field -->

<!-- evidence-field:removed_tests list -->
## Removed tests

- crates/jj-store-cli/tests/agent_execution/conversation_failures.rs#rejected_resume_finishes_receipt_retires_worker_and_never_replays
<!-- /evidence-field -->

<!-- generated-navigation -->
## Pages

- [E-AGENT-RESUME-001 details 2](details-02.md)
- [E-AGENT-RESUME-001 details 3](details-03.md)
- [E-AGENT-RESUME-001 details 4](details-04.md)
