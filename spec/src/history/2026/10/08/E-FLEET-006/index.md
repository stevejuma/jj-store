---
schema: 1
id: E-FLEET-006
---

# E-FLEET-006

<!-- evidence-field:id markdown -->
## Id

E-FLEET-006
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M24
- M25
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-COLLAB-034
- JST-COLLAB-036
- JST-UI-012
- JST-UI-015
- JST-DOC-001
- JST-DOC-002
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-08
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Rebase onto Compute main, preserve shared chats and prepare confirmed workspace retirement
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

yyzxpuru (enclosing Jujutsu change; main parent tvyovoys / 45497b21; original fleet tree 44901e87)
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-application/AgentChatChanges.stories.svelte
- catalog/components/store-application/AgentChatFeedback.stories.svelte
- catalog/components/store-application/AgentPromptMarkup.stories.svelte
- catalog/components/store-application/AgentReplyIndependence.stories.svelte
- catalog/components/store-application/AgentRichTools.stories.svelte
- catalog/fixtures/AgentExecutionFixture.svelte
- catalog/fixtures/AgentFleetApplicationFixture.svelte
- catalog/fixtures/AgentRichToolsFixture.svelte
- catalog/fixtures/agent-chat-tool-events.ts
- catalog/fixtures/agent-discussion-fixture.ts
- catalog/fixtures/agent-execution-status.ts
- catalog/fixtures/agent-fleet-changes.ts
- crates/jj-agent/src/normalizers/controller.rs
- crates/jj-agent/tests/controller.rs
- crates/jj-collab/src/threads.rs
- crates/jj-store-cli/tests/agent_execution/conversations.rs
- crates/jj-store-server/src/agent_execution/commands.rs
- crates/jj-store-server/src/agent_execution/conversation_prompts.rs
- crates/jj-store-server/src/agent_execution/feedback.rs
- crates/jj-store-server/src/agent_execution/mod.rs
- crates/jj-store-server/src/agent_execution/prompt_context/tests.rs
- crates/jj-store-server/src/agent_execution/prompt_context.rs
- crates/jj-store-service/src/agent_execution/context.rs
- crates/jj-store-service/src/agent_execution/feedback_subject.rs
- crates/jj-store-service/src/agent_execution.rs
- crates/jj-store-service/src/agent_feedback.rs
- crates/jj-store-service/src/collab_reads/agent_feedback.rs
- crates/jj-store-service/src/local_repository/agent_feedback.rs
- crates/jj-store-service/tests/local_collab/agent_discussion.rs
- docs/maintenance/coverage.json
- docs/site/guides/agent-execution.md
- docs/site/technical/architecture.md
- docs/site/technical/lifecycle.md
- docs/ui-provenance.json
- spec/src/current/contracts/collaboration/16-persistent-threads-and-reactions-m15.md
- spec/src/current/contracts/collaboration/37-reusable-agent-conversations.md
- spec/src/current/contracts/documentation/08-interactive-agent-execution.md
- spec/src/current/contracts/ui/34-interactive-agent-sessions.md
- spec/src/current/contracts/ui/35-compute-defaults-and-sent-context.md
- spec/src/current/governance/spec-governance.md
- spec/src/current/progress/M24/history/M24/index.md
- spec/src/current/progress/M24/index.md
- spec/src/current/progress/M25/history/M25/index.md
- spec/src/current/progress/M25/index.md
- spec/src/history/2026/10/07/E-COMPUTE-001/index.md
- spec/src/history/2026/10/07/E-EXEC-022/index.md
- spec/src/history/2026/10/07/E-FLEET-004/SUMMARY.nav.md
- spec/src/history/2026/10/07/E-FLEET-004/attempts.md
- spec/src/history/2026/10/07/E-FLEET-004/index.md
- spec/src/history/2026/10/07/E-FLEET-004/qualification.md
- spec/src/history/2026/10/07/SUMMARY.nav.md
- spec/src/history/2026/10/07/index.md
- spec/src/history/2026/10/08/E-COMPUTE-002/index.md
- spec/src/history/2026/10/08/E-COMPUTE-003/index.md
- spec/src/history/2026/10/08/E-FLEET-005/SUMMARY.nav.md
- spec/src/history/2026/10/08/E-FLEET-005/index.md
- spec/src/history/2026/10/08/E-FLEET-006/SUMMARY.nav.md
- spec/src/history/2026/10/08/E-FLEET-006/index.md
- spec/src/history/2026/10/08/SUMMARY.nav.md
- spec/src/history/2026/10/08/index.md
- web/src/lib/controller/agent-workspace-changes.ts
- web/src/lib/ui/community-agent-activity/CommunityAgentTranscriptTools.svelte
- web/src/lib/ui/community-agent-activity/TranscriptToolDetail.svelte
- web/src/lib/ui/community-agent-activity/TranscriptToolRichDetail.svelte
- web/src/lib/ui/community-agent-activity/transcript-tool-adapters.ts
- web/src/lib/ui/community-agent-activity/transcript-tool-rich-detail.ts
- web/src/lib/ui/community-agent-activity/transcript-tool-rich.ts
- web/src/lib/ui/community-agent-activity/transcript-tool-values.ts
- web/src/lib/ui/community-message/CommunityMessage.svelte
- web/src/lib/ui/community-message/props.ts
- web/src/lib/ui/store-agent-chat/StoreAgentBindings.svelte
- web/src/lib/ui/store-agent-chat/StoreAgentChangesDiff.svelte
- web/src/lib/ui/store-agent-chat/StoreAgentChangesPanel.svelte
- web/src/lib/ui/store-agent-chat/StoreAgentChatSidebar.svelte
- web/src/lib/ui/store-agent-execution/StoreAgentChatRows.svelte
- web/src/lib/ui/store-agent-execution/StoreAgentConversation.svelte
- web/src/lib/ui/store-agent-execution/StoreAgentExecutionNotice.svelte
- web/src/lib/ui/store-agent-execution/StoreAgentInboxMessage.svelte
- web/src/lib/ui/store-agent-execution/StoreAgentPromptContent.svelte
- web/src/lib/ui/store-agent-execution/StoreAgentTranscript.svelte
- web/src/lib/ui/store-agent-execution/activity-row-groups.ts
- web/src/lib/ui/store-agent-execution/presentation.ts
- web/src/lib/ui/store-collaboration/StoreAgentToolDetail.svelte
- web/src/lib/ui/store-collaboration/StoreAgentTools.svelte
- web/src/lib/ui/store-collaboration/StoreAgentTranscript.svelte
- web/src/lib/ui/store-collaboration/StoreMessageActions.svelte
- web/tests/browser/agent-chat-changes.spec.ts
- web/tests/browser/agent-tool-rich.spec.ts
- web/tests/vite/agent-chat.test.ts
- web/tests/vite/agent-tool-groups.test.ts
- web/tests/vite/agent-tool-rich.test.ts
- web/tests/vite/agent-workspace-changes.test.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/vite/agent-chat.test.ts#context delivery updates one authored row without replacing its recorded clock or XML
- web/tests/vite/agent-chat.test.ts#retains recorded lifecycle clocks and separates original text from context
- web/tests/browser/agent-chat-changes.spec.ts#cold chats show live avatars, composer reconnect warnings and exact saved changes
- web/tests/browser/agent-tool-rich.spec.ts#cold ACP tools show highlighted captured files and independent raw JSON views
- crates/jj-agent/tests/controller.rs#controller_tool_outputs_retain_raw_file_bytes_and_structured_evidence
- crates/jj-agent/tests/controller.rs#sent_context_preserves_human_text_snapshot_and_message_identity_after_delivery
- crates/jj-store-server/src/agent_execution/prompt_context/tests.rs#xml_context_keeps_captured_markup_inside_a_longer_fence
- crates/jj-store-server/src/agent_execution/delivery/tests.rs#context_delivery_tracks_acknowledged_start_and_unknown_restart_without_reconstructing
- crates/jj-store-service/tests/local_collab/agent_discussion.rs#tool_feedback_targets_selected_output_and_retains_replies_and_reactions
- crates/jj-store-cli/tests/agent_execution/conversations.rs#repository_free_chat_reuses_isolated_bindings_and_never_replays_after_restart
<!-- /evidence-field -->

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown
```

```shell
jj --no-pager st
jj workspace list
jj --no-pager log -n 14
jj rebase -s yyzxpuru -d 45497b21
jj resolve --list
```

**Notes**

Before: main is clean at nzzkxopz / 8fa8cd0b, parent tvyovoys / 45497b21, with five integrated Compute commits since the fleet parent e1c39d71. Fleet yyzxpuru / 44901e87 contains E-FLEET-004/005. Rebase produces eighteen conflicts; resolution retains both branches, including local agent registration, model discovery fixtures, Compute manuals, Sent context and rich/shared chat rows. Final conflict listing is empty. The first resolver script stopped only at a subsequent unmatched command-frame line; all conflict resolutions were already retained, and the actual recorded-frame path was then updated explicitly. Shell discovery initially lacked jj under login:false; explicit /opt/homebrew/bin/jj is used, without installed-binary changes.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown
```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/agent-chat.test.ts web/tests/vite/agent-tool-groups.test.ts web/tests/vite/agent-tool-rich.test.ts web/tests/vite/agent-workspace-changes.test.ts web/tests/vite/agent-model-catalog.test.ts
pnpm web:check
pnpm catalog:check
pnpm check:architecture
```

**Notes**

Units pass 36/36 across five files, including one-row context delivery with original clock/XML retention. Web and catalog checks pass with zero errors/warnings; catalog boundary audit covers 297 modules. The architecture checker first rejects AgentExecutionFixture at 303 counted lines; its unchanged setup construction moves into its existing status fixture module. Exact rerun passes 1353 modules and the final catalog check remains clean. Logs: /tmp/jj-store-fleet-rebase-unit.log, /tmp/jj-store-fleet-rebase-web-check.log, /tmp/jj-store-fleet-rebase-catalog-check-final.log and /tmp/jj-store-fleet-rebase-architecture-final.log.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown
```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentSessions.stories.svelte catalog/components/store-application/AgentSessionSettings.stories.svelte catalog/components/store-application/AgentHarnessModels.stories.svelte catalog/components/store-application/AgentExecution.stories.svelte catalog/components/store-application/AgentChatRuntime.stories.svelte catalog/components/store-application/AgentDiscussion.stories.svelte catalog/components/store-application/AgentRevisionIndex.stories.svelte catalog/components/store-application/AgentChatLayout.stories.svelte catalog/components/store-application/AgentReplyIndependence.stories.svelte catalog/components/store-application/AgentInboxNavigation.stories.svelte catalog/components/store-application/AgentChatFeedback.stories.svelte catalog/components/store-application/AgentFleet.stories.svelte catalog/components/store-application/AgentDefaults.stories.svelte catalog/components/store-application/AgentChatChanges.stories.svelte catalog/components/store-application/AgentPromptMarkup.stories.svelte catalog/components/store-application/AgentRichTools.stories.svelte catalog/components/store-application/LocalCompute.stories.svelte
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentChatFeedback.stories.svelte
```

**Notes**

Escalation selects all consumers of the merged execution/fleet fixtures and shared chat rows, plus rich tools and Compute's captured-context disclosure. Initial run: 93 passed / one failed among 94 tests in seventeen families. Turn completion uses the shared system message still expected visible text despite the explicitly requested icon-only contract. It now asserts the accessible group, outcome icon, recorded time, title, shared system divider and log placement. Exact family rerun passes 4/4 without weakening lifecycle assertions. Final affected-family rerun passes 94/94 in 71.58 seconds. Logs: /tmp/jj-store-fleet-rebase-stories.log, /tmp/jj-store-fleet-rebase-feedback-rerun.log and /tmp/jj-store-fleet-rebase-stories-final.log. Visual baselines remain unapproved.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown
```

```shell
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo check --locked -p jj-agent -p jj-store-service -p jj-store-server
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test --locked -p jj-agent --test controller
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test --locked -p jj-store-server --lib agent_execution::prompt_context::tests
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test --locked -p jj-store-server --lib agent_execution::delivery::tests
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test --locked -p jj-store-service --test local_collab agent_discussion::tool_feedback_targets_selected_output_and_retains_replies_and_reactions -- --exact
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test --locked -p jj-store-cli --test agent_execution conversations::repository_free_chat_reuses_isolated_bindings_and_never_replays_after_restart -- --exact
CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test --locked -p jj-store-cli --test agent_execution conversations::repository_free_chat_reuses_isolated_bindings_and_never_replays_after_restart -- --exact
```

**Notes**

The three-package boundary check passes. Controller 7/7, XML context 2/2, delivery 6/6 and exact tool feedback 1/1 pass. The CLI integration build initially fails before tests with ENOSPC; filesystem availability is 134 MiB. No cargo/rustc remains active. Only the ignored Rust incremental cache is cleared, freeing approximately eighteen GiB; sources, runtime data, binaries and the active main service are retained. Exact conversation retry disables incremental output and passes 1/1 in 119.82 seconds. This controlled-harness test strengthens initial-message checks to require authored text separate from XML, and captured wireText/wireContext matching the sanitized frame, retaining all isolated-binding/restart/no-replay assertions. Logs: /tmp/jj-store-fleet-rebase-rust-outcomes.json (initial compile refusal) and /tmp/jj-store-fleet-rebase-rust-conversation-rerun.log (passing retry). The resource refusal is not a functional test failure.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown
```

```shell
pnpm exec vite build --config web/vite.config.ts
node scripts/web-manifest.mjs
pnpm exec playwright test --config .scratch/agent-fleet-playwright.config.ts web/tests/browser/agent-tool-rich.spec.ts web/tests/browser/agent-chat-changes.spec.ts
```

**Notes**

Fresh production assets/manifest pass. Both exact cold browser cases pass 2/2 in 11.7 seconds with the existing isolated host and captured native fixtures. Rich/raw tools, full-width groups, icons/times, feedback, selected avatar presence, composer warning recovery, retained drafts, bounded saved comparisons and diff dialogs retain their assertions and zero Axe violations. Send/control traffic is mocked only; no real prompt, resume or control is issued. Existing CSS/chunk warnings stay explicit. Logs: /tmp/jj-store-fleet-rebase-production.log and /tmp/jj-store-fleet-rebase-cold.log.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown
```

```shell
pnpm docs:review
PATH=/Users/stevejuma/.local/bin:$PATH CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target node scripts/docs.mjs stage
pnpm progress:history
pnpm exec prettier --write spec/src/current/progress/M24/history/M24/index.md spec/src/current/progress/M25/history/M25/index.md
pnpm exec prettier --check --ignore-unknown <jj-diff-changed-paths>
pnpm check:ui-ownership
pnpm spec:validate
pnpm check:progress
```

**Notes**

Initial documentation staging refuses stale collaboration review; the next review also identifies services and configuration. Topic-specific reviews cover separate authored/XML context, exact committed tool targets and unchanged authority/lifecycle controls. Guide corrections distinguish the legacy one-worker profile from the four-worker fleet. A subsequent staging attempt cannot find uv; adding its existing user binary directory resolves it. Documentation stages nine local diagrams and passes strict build/site auditing. Ownership passes one validator; spec validation passes seven validators, 3194 chapters and 181 requirements. Formatting initially finds only two regenerated history pages and is corrected. Progress initially refuses a pending run result; the completed run is corrected and checked before commit. The changed-path formatting command expands jj diff --name-only through Python subprocess, without shell interpolation. Main integration is the next verified slice; deletion is a separate user-confirmed action. The independently verified private backup at main .local/workspace-retirement/agent-fleet-2026-10-08-beb70383 contains 4462 regular files / 104196187 bytes with matching hashes. Only the stopped native socket and rebuildable outputs are excluded. Original runtime paths are documented for restoration; the live main fixture is not replaced. Logs: /tmp/jj-store-fleet-rebase-docs-stage-final.log, /tmp/jj-store-fleet-rebase-ownership.log, /tmp/jj-store-fleet-rebase-spec.log and /tmp/jj-store-fleet-rebase-progress-final.log. Root aggregate/source-wide/platform and visual-baseline lanes remain out of scope under the targeted policy.
<!-- /run -->

<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pass
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

[]
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64; Jujutsu 0.45.1; Node 26.9.0 for these login-shell checks, main dev UI retained on Node 24.10.0; pnpm 10.32.1; Rust 1.98.1; Vitest 4.1.11. Existing local Design Core validation setup is retained without publication or installed adapter replacement.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Native identities, reference/receipt/WAL authority, AgentStore object formats and profile refusals remain unchanged. New retained prompt frames carry separate authored text/XML context and captured Sent context wireText/wireContext. Legacy observations remain immutable. Context delivery enriches the same message identity and original clock. No public API/schema generation changed.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

The user requested rebasing onto main, ensuring all changes are in its workspace and deleting the temporary workspace after confirmation. This directly authorizes local integration following the previously disclosed source qualification gaps; it does not authorize publication, baseline updates or treating wider source checks as passing. Conflict resolution preserves the integrated Compute settings and model features, lifecycle/tool/file-change presentation, exact feedback and both context mechanisms. UI fixtures retain defaults and catalog discovery while adding completed changes/reconnect cases. Cross-layer verification uses focused checks for each directly affected boundary, without unrelated root aggregate lanes.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Commit the verified merged slice and advance the clean default workspace onto it with a separate integration record. Retain the active main UI and request confirmation for removing only jj-store-agent-fleet. Keep independent M24/M25 and wider Design Core gates open.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-COLLAB-036 combines sanitized XML prompt blocks with captured Sent context, preserving delivery identities/clocks. JST-UI-012/JST-UI-015 retain both chat disclosures and lifecycle icons. JST-DOC-001/JST-DOC-002 record exact wire blocks and the local integration scope. Storage/receipt/native identity, authority, model/secret controls and prior Compute gates remain unchanged.
```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M24 and M25 remain In progress. Local integration does not close additional adapters, live permission/cancellation, session Undo, source full checks or signed platform distribution gates. The earlier source failures remain recorded and are accepted only for the user-requested local integration.
```
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


