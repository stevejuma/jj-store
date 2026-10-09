---
schema: 1
id: E-CHAT-LIVE-015
---

# E-CHAT-LIVE-015 details 2

<!-- evidence-field:remaining list -->
## Remaining

- M24 independent adapter/provider, live permissions/cancellation, native IME,
  desktop/platform and visual approval gates stay open.
- Previously recorded broad legacy-story and development acceptance-script size
  gaps stay in E-CHAT-LIVE-011; unrelated lanes were not repeated.
- Existing temporary foundational dependency validation setup remains unchanged;
  no manifests or lockfiles modified.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Node 26.9.0; pnpm 10.32.1; jj 0.45.1; Vitest 4.1.11; macOS ARM64.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Internal presentation props add loading and emptyState. HTTP/ACP, native/local storage, request/receipt, publication, source mapping and preference formats are unchanged.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

UI-only conversation loading/empty presentation and Sent-context hover, with direct shared-renderer/live-host/fixture/story consumers. Rust, generated public contracts, root aggregate lanes, built bundle and visual baseline updates are out of scope. Fresh isolated Vitest/Vite runs and a fresh live-page read check cover this presentation flow; backend startup/lifecycle is unchanged.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Keep M24 In progress and continue its independent qualification gates.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-UI-015 specifies compact retained-history loading and completed
  icon-bearing empty states. JST-UI-012 specifies context hover even when
  expanded. JST-API-001 authority, paging and execution contracts remain
  unchanged.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M24 stays In progress. Focused presentation verification does not close
  independent provider, platform, native IME or visual approval gates.

```
<!-- /evidence-field -->

<!-- evidence-field:id markdown -->
## Id

E-CHAT-LIVE-015
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M24
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-015
- JST-UI-012
- JST-API-001
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-09
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Compact conversation history loading and honest empty presentation
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change ypmnkvsl
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Three exact regressions fail: the selected-history body uses a full message skeleton instead of the compact loading control; completed empty history is plain text; expanded context restores its resting background on hover. The shared body unconditionally renders No captured messages beside its loading snippet.
<!-- /evidence-field -->

<!-- evidence-field:findings markdown -->
## Findings

ChatBodyProps now exposes explicit loading and a host emptyState snippet. The shared Document/Classic body shows a centered disabled ghost xs Button with a decorative public Spinner above retained items, marks its log busy, suppresses emptiness while loading, and composes public shadcn Empty with a Lucide message icon after a completed empty read. Standalone live sessions forward their existing eventsLoading flag; the duplicate beforeItems message skeleton is removed. Draft-chat hosts supply their existing setup Empty as the replacement, avoiding a second default empty state. Loading does not replace existing message/editor nodes or clear drafts. The owned Sent-context trigger has the user-requested themed hover surface while collapsed or expanded, including unknown delivery; keyboard disclosure and rendered/raw state remain intact. Community Conversation and Design Core MessageList were inspected; their previous-history control uses a compact ghost Button and Spinner. This is owned application composition using released foundations, with no new copied source file, dependency or provenance boundary.
<!-- /evidence-field -->

<!-- evidence-field:live markdown -->
## Live

Fresh in-app browser verification used the preserved live sample on port 7460. Context delivery unknown has a 24px trigger and actual pointer hover changes its theme background, including when expanded. To make loading observable, a temporary non-Document Fetch interception delayed only the selected retained-event GET for ddcc5f115b95bf017564735592db880f. The loader measures 24px, its center offset is 0px, its spinner exists, and no premature empty text or full message skeleton appears. Screenshot: .local/chat-history-loading-live.png. Releasing the held request and clearing interception restores normal pagination; after all pages finish the loader disappears and historical content remains. Selecting existing empty session a4eb430fc5a1ebe5dc4f9495d9a79815 shows exactly one Empty with one icon and no loader. Warning/error logs are empty in this flow, including ResizeObserver. Temporary whole-network latency delayed Vite module loading and was restored to zero before the focused history check; raw CDP rejected unrestricted interception and Fetch.disable, so the accepted Fetch resource pattern and an empty pattern array were used, with the held read explicitly continued. No dispatch, resume, configuration save, service restart or persisted browser change was performed. The temporary tab was closed.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- web/src/lib/ui/store-agent-chat-body/StoreAgentChatBody.svelte
- web/src/lib/ui/store-agent-chat-body/model.ts
- web/src/lib/ui/store-agent-chat-body/chat-body.css
- web/src/lib/ui/store-agent-chat/StoreLiveConversation.svelte
- web/src/lib/ui/store-agent-chat/StoreAgentChatSidebar.svelte
- web/src/lib/ui/store-agent-execution/StoreAgentConversation.svelte
- web/src/lib/ui/store-agent-execution/StoreAgentSentContext.svelte
- catalog/components/store-application/AgentChatBody.stories.svelte
- catalog/components/store-application/AgentSessions.stories.svelte
- catalog/components/store-application/AgentContextDelivery.stories.svelte
- catalog/fixtures/ChatBodyFixture.svelte
- catalog/fixtures/AgentContextDeliveryFixture.svelte
- spec/src/current/contracts/ui/34-interactive-agent-sessions.md
- spec/src/current/contracts/ui/35-compute-defaults-and-sent-context.md
- spec/src/current/contracts/ui/37-document-conversations.md
- spec/src/current/contracts/api/41-interactive-agent-execution.md
- spec/src/current/progress/M24/index.md
- spec/src/current/progress/M24/history/M24/index.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

[]
<!-- /evidence-field -->

