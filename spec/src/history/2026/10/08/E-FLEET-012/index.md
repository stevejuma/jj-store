---
schema: 1
id: E-FLEET-012
---

# E-FLEET-012

<!-- evidence-field:id markdown -->
## Id

E-FLEET-012
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M24
- M25
- M11.2
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-COLLAB-034
- JST-COLLAB-035
- JST-COLLAB-036
- JST-UI-019
- JST-API-001
- JST-CLI-004
- JST-WORK-005
- JST-WORK-006
- JST-WORK-008
- JST-DOC-001
- JST-DOC-002
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-08
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Markdown agent context, incremental delivery and bound CLI coordination
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change lptotxop above b50c5b02
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Clean working copy lptotxop. Existing prompt-context tests: 2 passed; execution/inbox unit baseline: 8 passed. New native-Markdown regression failed once against the JSON/text-fenced renderer. Serialized context changes reset bootstrap and shared transcripts displayed both XML context and a Sent context footer.
<!-- /evidence-field -->

<!-- evidence-field:findings markdown -->
## Findings

One typed sanitized document now supplies Markdown sections and exact captured/wire bytes. Per-session bootstrap is independent of origin/binding revisions and exact reply delivery; reservations settle only on matching started events. Legacy journals and immutable captures retain their meaning. Fresh adapters receive bounded authored history with omission disclosure. One collapsed context inspector precedes messages across inbox, sidebar and recorded sessions. The verified bundled custom jj is exported through a reserved environment variable and contextual sessions get a metadata-home coordination checkout outside code attachments. Real CLI acceptance found a pre-existing task --repository/global -R argument collision; task linkage is now --target-repository. Final isolated native cold acceptance passed, including mismatched binding refusal, retained failure and explicit recovery. Named implementation targets pass; the evidence remains pending solely for the separately recorded pre-existing architecture check and independent milestone qualification gaps.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Cross-layer controller/service, CLI/workspace and three shared transcript surfaces. Escalation covers the full ACP fixture binary and all four directly affected story families, affected package checks, exact service reply tests, work/review CLI tests and isolated native real-RustFS built-UI restart acceptance. Root aggregates, full Rust workspace, unrelated web/catalog families, live providers, downloads, packaged desktop/WKWebView/Linux and visual baselines are out of scope. No installed CLI replacement, credential use, reset, publication or storage format migration.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- Cargo.lock
- catalog/components/store-application/AgentContextDelivery.stories.svelte
- catalog/components/store-application/AgentPromptMarkup.stories.svelte
- catalog/components/store-application/AgentReplyIndependence.stories.svelte
- catalog/fixtures/AgentContextDeliveryFixture.svelte
- crates/jj-store-cli/src/work_commands/task.rs
- crates/jj-store-cli/tests/agent_execution/context_delivery.rs
- crates/jj-store-cli/tests/agent_execution.rs
- crates/jj-store-cli/tests/task_commands.rs
- crates/jj-store-jj/src/agent_workspaces/provision.rs
- crates/jj-store-server/Cargo.toml
- crates/jj-store-server/src/agent_execution/commands.rs
- crates/jj-store-server/src/agent_execution/configuration/tests.rs
- crates/jj-store-server/src/agent_execution/configuration.rs
- crates/jj-store-server/src/agent_execution/conversation_frames.rs
- crates/jj-store-server/src/agent_execution/conversation_prompts.rs
- crates/jj-store-server/src/agent_execution/conversations.rs
- crates/jj-store-server/src/agent_execution/coordination.rs
- crates/jj-store-server/src/agent_execution/delivery/tests.rs
- crates/jj-store-server/src/agent_execution/delivery.rs
- crates/jj-store-server/src/agent_execution/journal.rs
- crates/jj-store-server/src/agent_execution/launch.rs
- crates/jj-store-server/src/agent_execution/mod.rs
- crates/jj-store-server/src/agent_execution/prompt_context/cli.rs
- crates/jj-store-server/src/agent_execution/prompt_context/history.rs
- crates/jj-store-server/src/agent_execution/prompt_context/markdown.rs
- crates/jj-store-server/src/agent_execution/prompt_context/origin.rs
- crates/jj-store-server/src/agent_execution/prompt_context/tests.rs
- crates/jj-store-server/src/agent_execution/prompt_context/tracking.rs
- crates/jj-store-server/src/agent_execution/prompt_context.rs
- crates/jj-store-server/src/agent_execution/sent_context.rs
- crates/jj-store-server/src/process/preparation.rs
- crates/jj-store-service/src/agent_execution/context.rs
- crates/jj-store-service/tests/local_collab/agent_context.rs
- docs/cli/work.md
- docs/maintenance/coverage.json
- docs/site/cli/jj-project.md
- docs/site/cli/jj-task.md
- docs/site/getting-started/work.md
- docs/site/guides/agent-execution.md
- docs/site/guides/browsing.md
- docs/site/reference/environment.md
- docs/site/technical/architecture.md
- docs/site/technical/lifecycle.md
- docs/site/technical/local.md
- scripts/agent-context-acceptance.py
- scripts/agent-context-browser.mjs
- spec/src/current/contracts/api/41-interactive-agent-execution.md
- spec/src/current/contracts/architecture/20-interactive-agent-execution.md
- spec/src/current/contracts/cli/10-workstore-service-and-transports.md
- spec/src/current/contracts/cli/35-interactive-agent-execution.md
- spec/src/current/contracts/collaboration/37-reusable-agent-conversations.md
- spec/src/current/contracts/documentation/08-interactive-agent-execution.md
- spec/src/current/contracts/jj-integration/32-interactive-agent-workspaces.md
- spec/src/current/contracts/local-jj/10-local-agentstore-browser-selection.md
- spec/src/current/contracts/mutations/22-interactive-agent-execution.md
- spec/src/current/contracts/ui/34-interactive-agent-sessions.md
- spec/src/current/governance/spec-governance.md
- spec/src/current/progress/M11/history/M11.2/index.md
- spec/src/current/progress/M11/index.md
- spec/src/current/progress/M24/history/M24/index.md
- spec/src/current/progress/M24/index.md
- spec/src/current/progress/M25/history/M25/index.md
- spec/src/current/progress/M25/index.md
- web/src/lib/ui/store-agent-execution/StoreAgentChatRows.svelte
- web/src/lib/ui/store-agent-execution/StoreAgentInboxMessage.svelte
- web/src/lib/ui/store-agent-execution/StoreAgentPromptContent.svelte
- web/src/lib/ui/store-agent-execution/StoreAgentSentContext.svelte
- web/src/lib/ui/store-collaboration/StoreAgentTranscript.svelte
- web/tests/browser/agent-tool-rich.spec.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-server/src/agent_execution/prompt_context/tests.rs#context_sections_render_native_markdown_and_keep_literal_code
- crates/jj-store-server/src/agent_execution/prompt_context/tests.rs#document_bounds_and_snapshot_are_exact
- crates/jj-store-server/src/agent_execution/delivery/tests.rs#section_deltas_reserve_in_order_acknowledge_release_and_survive_restart
- crates/jj-store-cli/tests/agent_execution/context_delivery.rs#markdown_deltas_and_bound_task_cli_survive_loaded_and_fresh_sessions
- crates/jj-store-cli/tests/task_commands.rs#task_target_does_not_shadow_the_global_checkout_argument
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-FLEET-012 details 2](details-02.md)
- [E-FLEET-012 details 3](details-03.md)
