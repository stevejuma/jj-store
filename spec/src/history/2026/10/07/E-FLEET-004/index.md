---
schema: 1
id: E-FLEET-004
---

# E-FLEET-004

<!-- evidence-field:id markdown -->
## Id

E-FLEET-004
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M24
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-COLLAB-005
- JST-COLLAB-009
- JST-COLLAB-010
- JST-COLLAB-033
- JST-COLLAB-034
- JST-COLLAB-035
- JST-COLLAB-036
- JST-UI-015
- JST-DOC-001
- JST-DOC-002
- JST-GOV-008
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-07
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Rich agent tools, shared chat rows, exact tool discussions and separate ACP context
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

yyzxpuru (enclosing Jujutsu change; parent e1c39d71)
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-application/AgentPromptMarkup.stories.svelte
- catalog/components/store-application/AgentReplyIndependence.stories.svelte
- catalog/components/store-application/AgentRichTools.stories.svelte
- catalog/fixtures/AgentExecutionFixture.svelte
- catalog/fixtures/AgentRichToolsFixture.svelte
- catalog/fixtures/agent-chat-tool-events.ts
- catalog/fixtures/agent-discussion-fixture.ts
- crates/jj-agent/src/normalizers/controller.rs
- crates/jj-agent/tests/controller.rs
- crates/jj-collab/src/threads.rs
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
- docs/ui-provenance.json
- spec/src/current/contracts/collaboration/16-persistent-threads-and-reactions-m15.md
- spec/src/current/contracts/collaboration/37-reusable-agent-conversations.md
- spec/src/current/contracts/ui/34-interactive-agent-sessions.md
- spec/src/current/governance/spec-governance.md
- spec/src/current/progress/M24/index.md
- spec/src/history/2026/10/07/E-FLEET-004/SUMMARY.nav.md
- spec/src/history/2026/10/07/E-FLEET-004/index.md
- spec/src/history/2026/10/07/E-FLEET-004/attempts.md
- spec/src/history/2026/10/07/E-FLEET-004/qualification.md
- spec/src/history/2026/10/07/SUMMARY.nav.md
- spec/src/history/2026/10/07/index.md
- web/src/lib/ui/community-agent-activity/CommunityAgentTranscriptTools.svelte
- web/src/lib/ui/community-agent-activity/TranscriptToolDetail.svelte
- web/src/lib/ui/community-agent-activity/TranscriptToolRichDetail.svelte
- web/src/lib/ui/community-agent-activity/transcript-tool-adapters.ts
- web/src/lib/ui/community-agent-activity/transcript-tool-rich-detail.ts
- web/src/lib/ui/community-agent-activity/transcript-tool-rich.ts
- web/src/lib/ui/community-agent-activity/transcript-tool-values.ts
- web/src/lib/ui/community-message/CommunityMessage.svelte
- web/src/lib/ui/community-message/props.ts
- web/src/lib/ui/store-agent-chat/StoreAgentChatSidebar.svelte
- web/src/lib/ui/store-agent-execution/StoreAgentChatRows.svelte
- web/src/lib/ui/store-agent-execution/StoreAgentInboxMessage.svelte
- web/src/lib/ui/store-agent-execution/StoreAgentPromptContent.svelte
- web/src/lib/ui/store-agent-execution/StoreAgentTranscript.svelte
- web/src/lib/ui/store-agent-execution/activity-row-groups.ts
- web/src/lib/ui/store-agent-execution/presentation.ts
- web/src/lib/ui/store-collaboration/StoreAgentToolDetail.svelte
- web/src/lib/ui/store-collaboration/StoreAgentTools.svelte
- web/src/lib/ui/store-collaboration/StoreAgentTranscript.svelte
- web/src/lib/ui/store-collaboration/StoreMessageActions.svelte
- web/tests/browser/agent-tool-rich.spec.ts
- web/tests/vite/agent-chat.test.ts
- web/tests/vite/agent-tool-groups.test.ts
- web/tests/vite/agent-tool-rich.test.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/vite/agent-chat.test.ts#retains recorded lifecycle clocks and separates original text from context
- web/tests/vite/agent-tool-groups.test.ts#adjacent tools form one stable group without replacing or mutating exact rows
- web/tests/vite/agent-tool-groups.test.ts#session, turn and recorded day boundaries remain distinct
- web/tests/browser/agent-tool-rich.spec.ts#cold ACP tools show highlighted captured files and independent raw JSON views
- crates/jj-agent/tests/controller.rs#controller_tool_outputs_retain_raw_file_bytes_and_structured_evidence
- crates/jj-store-server/src/agent_execution/prompt_context/tests.rs#xml_context_keeps_captured_markup_inside_a_longer_fence
- crates/jj-store-server/src/agent_execution/prompt_context/tests.rs#legacy_projection_requires_the_exact_durable_original_and_preserves_evidence
- crates/jj-store-service/tests/local_collab/agent_discussion.rs#tool_feedback_targets_selected_output_and_retains_replies_and_reactions
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
cua_repl saved-session observation on 7474 and source inspection in Community then Design Core
```

**Notes**

Before: five tools rendered separately. Original width was 672px in a 988px body and the long first-call disclosure shrank to 4.40625px by 14px. After source layout changes, a residual global cap still limited the live group to 672px in 798px available. Removing that app cap gives 798/798px, all five chevrons 14x14px, 8px trigger inset, one five-call group and icon-only timed lifecycle markers. Reloaded saved context renders separately as agent_context. No Send or replay.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/agent-chat.test.ts web/tests/vite/agent-tool-groups.test.ts web/tests/vite/agent-tool-rich.test.ts
```

**Notes**

Final focused units: 23/23 passed. Covers exact turn/session/day grouping, stable group identity, XML projection and retained rich-data semantics.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentReplyIndependence.stories.svelte catalog/components/store-application/AgentRichTools.stories.svelte catalog/components/store-application/AgentToolGroups.stories.svelte --maxWorkers 1
```

**Notes**

Final directly affected stories: 17/17 passed in a fresh process, including independent replies, slim tool hover inset, grouping/rich/raw state, full width and lifecycle icons. Earlier exact grouped story failed for missing Expand 2 tool calls; the repair passed. A lowercase test filter skipped all tests; the exact generated PascalCase name was used instead. After removing the app width cap and repairing selector types, the two affected families reran: 11/11 passed.
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
pnpm web:check
```

**Notes**

Final lifecycle-icon source and final full-width web check: 0 errors and 0 warnings. Final catalog check after anchored-selector repair: 0 errors/warnings and 284-module boundary check passed.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test --locked -p jj-agent --test controller; CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test --locked -p jj-store-service --test local_collab
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test --locked -p jj-store-server --lib agent_execution::prompt_context; CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test --locked -p jj-collab
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo build -p jj-store-cli --bins --locked
```

**Notes**

Controller 6/6; full direct local collaboration binary 14/14 passed, including exact output discussion, reactions, nested context, cold reopen and receipts. Initial tool feedback run returned 3 pass/1 fail because rawOutput was not retained; the capture repair retains it without changing publication authority.

XML context regressions 2/2; jj-collab test groups 1+14+2+10 passed. Source facts are distinct from installed adapter interpretation.

Backend build passed after disk-pressure cache recovery. Nonfatal Darwin linker warning: __eh_frame exceeds compact unwind encoding limit. Owned preview backend activated at PID 54464; UI PID 29873 and unrelated services retained. No installed executable replacement.
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
Source repositories: jj-acp E-M12-002 checks; Design Core FileDiff and ToolCalls focused Storybook runs
```

**Notes**

ACP change ylkuyxzx source tests/build/spec/check pass; 27 protocol and 4 control tests passed, final full source rerun passed and locally integrated. Installed context interpretation remains pending there. Design Core FileDiff 11/11 and directly affected ToolCalls 10/10 passed. Core wider checks expose unrelated timeouts; full checks are not qualified. The final cold contrast repair and remaining lanes are recorded in qualification.md. No publication or baseline update.
<!-- /run -->
<!-- run -->
### Run 7

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown
```

```shell
pnpm exec playwright test --config .scratch/agent-fleet-playwright.config.ts web/tests/browser/agent-tool-rich.spec.ts
```

**Notes**

Final cold production browser: 1/1 passed in 25.5s after the recorded fixture, selector and contrast repairs. Uses fresh compiled host/output, one mocked Send, contextual chat at 1400px and wide inbox at 1800px. Asserts rich/raw views, captured path/bytes/language, exact tool Reply, no tool avatar, 8px hover inset, 14px chevron, full width beyond the prior cap, grouped tools, retained XML context, icon-only lifecycle with recorded times and zero Axe violations. No real prompt or control command. Final affected three-family story rerun after the contrast repair passes 17/17.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Final cold browser, affected stories, web/catalog, documentation, formatting, progress and spec validation pass. Source wider qualification/local integration remain pending explicit acceptance of the recorded gaps.
- Installed interpretation of embedded context resources, additional adapters, live permission/cancellation and desktop/platform qualification remain independent gates.
- Stories remain visual-pending; screenshot baseline approval and package publication were not requested.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64; Rust 1.98.1; JJ 0.45.1; Node 26.9.0; pnpm 10.32.1; Vitest 4.1.11. Community source 741b8285e2081f1c9af58afe5a9b72f538fad781 (Apache-2.0). Existing local Design Core validation setup; no package publication. Preview 7474/backend 7476 and live chat retained.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

AgentStore/ThreadStore objects, schemas, aliases, request identities and receipts retain their versions. Tool discussion validation admits existing ToolInvocation/ToolOutput targets. OwnedController captures rawOutput alongside structured content. Private prompt frames add separate XML context; ACP controls v2/v3 accept optional context and capability-gate embedded resources.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Captured reads, edits, shell and search tools have rich views with exact raw JSON and honest missing/partial/error states. Shared inbox/sidebar rows group consecutive tools within one session, turn and recorded day, retaining per-call details and collaboration actions. Tools fill their container with an 8px inset inside the trigger and 14px disclosure icons; the obsolete global 42rem cap is removed. Session start uses a clock icon; turn completion/cancellation/failure use their outcome icon, with recorded times, accessible labels and tooltips. Exact legacy intent data separates authored text from captured XML context without rewriting immutable evidence.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

The final app and source static checks pass; source full checks remain incomplete. Collect the pending visual outcome and explicit qualification decision. Commit and land the qualified slices only after the retained source gaps are resolved or explicitly accepted. Keep the fleet workspace and preview open.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: The owning UI and collaboration topics define shared chat rows, typed tool discussions, raw-output retention and separate XML context. Native storage formats, manifest authority, identity algorithms, reducers, authorization, replay fences and unsupported-profile refusals are unchanged.
```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M24 remains In progress. These regression repairs do not qualify additional adapters, permissions, desktop distribution or platforms.
```
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [Recorded failures and repairs](attempts.md)
- [Final chat qualification](qualification.md)
