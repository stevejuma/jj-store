---
schema: 1
id: E-CHAT-MARKDOWN-001
---

# E-CHAT-MARKDOWN-001

<!-- evidence-field:id markdown -->
## Id

E-CHAT-MARKDOWN-001
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-11
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M24
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-001
- JST-UI-015
- JST-UI-018
- JST-COLLAB-031
- JST-COLLAB-032
- JST-COLLAB-033
- JST-COLLAB-034
- JST-COLLAB-035
- JST-COLLAB-036
- JST-DOC-001
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- scripts/local-package-pnpmfile.cjs
- scripts/local-package-pnpmfile.test.mjs
- web/src/lib/conversations/file-links.ts
- web/src/lib/conversations/workspace-browser.svelte.ts
- web/src/lib/conversations/workspace-observation.ts
- web/src/lib/ui/store-agent-chat/StoreLiveConversation.svelte
- web/src/lib/ui/store-agent-chat/StoreAgentChangesPanel.svelte
- web/src/lib/ui/store-agent-chat/agent-changes-alert.css
- web/src/lib/ui/store-agent-chat-body/ChatTextBlock.svelte
- web/src/lib/ui/store-agent-chat-body/markdown-decorations.ts
- web/src/lib/ui/store-agent-chat-body/model.ts
- web/src/lib/ui/store-agent-chat-body/path-label.ts
- web/src/lib/ui/store-agent-chat-body/text-editor.ts
- web/src/lib/ui/store-agent-workspace/StoreWorkspaceInspector.svelte
- docs/ui-provenance.json
- docs/maintenance/coverage.json
- docs/site/guides/browsing.md
- docs/site/technical/capture.md
- docs/site/technical/lifecycle.md
- catalog/fixtures/delta-chat/file-links.test.ts
- catalog/fixtures/delta-chat/markdown-reading.test.ts
- catalog/fixtures/MarkdownReadingFixture.svelte
- catalog/components/store-application/MarkdownReading.stories.svelte
- catalog/components/store-application/AgentChatChanges.stories.svelte
- crates/jj-store-capture/src/transcript_sanitization.rs
- crates/jj-store-capture/src/transcript_sanitization/stream_text.rs
- crates/jj-store-capture/tests/transcript_sanitization.rs
- crates/jj-store-server/src/agent_execution/mod.rs
- crates/jj-store-server/src/agent_execution/runtime.rs
- crates/jj-store-server/src/agent_execution/runtime_privacy.rs
- crates/jj-store-server/src/agent_execution/runtime_privacy/tests.rs
- crates/jj-store-server/src/agent_execution/conversation_frames.rs
- crates/jj-store-server/src/agent_execution/conversation_stream.rs
- crates/jj-store-server/src/agent_execution/conversation_stream/tests.rs
- spec/src/current/contracts/ui/37-document-conversations.md
- spec/src/current/contracts/collaboration/36-interactive-agent-execution.md
- spec/src/history/2026/10/index.md
- spec/src/history/2026/10/SUMMARY.nav.md
- spec/src/history/2026/10/11/index.md
- spec/src/history/2026/10/11/SUMMARY.nav.md
- spec/src/history/2026/10/11/E-CHAT-MARKDOWN-001/SUMMARY.nav.md
- spec/src/history/2026/10/11/E-CHAT-MARKDOWN-001/details-02.md
- spec/src/history/2026/10/11/E-CHAT-MARKDOWN-001/details-03.md
- spec/src/history/2026/10/11/E-CHAT-MARKDOWN-001/index.md
- spec/src/current/contracts/collaboration/37-reusable-agent-conversations.md
- spec/src/history/2026/10/11/E-CHAT-MARKDOWN-001/details-04.md
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

pupxkumv (enclosing Jujutsu change; default workspace)
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- catalog/fixtures/delta-chat/file-links.test.ts#file links resolve absolute,
  relative and privacy-substituted destinations to the same binding
- catalog/fixtures/delta-chat/file-links.test.ts#file links refuse unsafe
  schemes, traversal, unrelated roots and ambiguous repositories
- catalog/fixtures/delta-chat/markdown-reading.test.ts#reading history renders
  hyphen bullets without changing source offsets
- catalog/fixtures/delta-chat/markdown-reading.test.ts#reading history renders
  angle-bracket repository file destinations as links
- crates/jj-store-capture/tests/transcript_sanitization.rs#streaming_paths_are_private_at_every_chunk_boundary
- crates/jj-store-server/src/agent_execution/runtime_privacy/tests.rs#streamed_privacy_precedes_journal_and_terminal_frames
- crates/jj-store-server/src/agent_execution/runtime_privacy/tests.rs#message_and_tool_boundaries_flush_without_joining_distinct_answers
- crates/jj-store-server/src/agent_execution/conversation_stream/tests.rs#fleet_privacy_isolated_by_participation_preserves_terminal_assistant_text
- crates/jj-store-cli/tests/agent_execution/conversations.rs#repository_free_chat_reuses_isolated_bindings_and_never_replays_after_restart
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Default-workspace repair of agent transcript reading and streamed privacy paths, with a trailing Retry column for unavailable changed-file alerts. Read-only history retains immutable source text and comment offsets. No session prompts, demo reset, package publication, public schema changes or credentials were introduced. Unrelated concurrent repository/story/evidence changes are excluded from this slice.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Node 26.9.0; pnpm 10.32.1; Rust 1.98.1; jj 0.45.1; Vitest 4.1.11 / Chromium. Local Mira 0.0.2 built archive from clean source d7f7a75f (ttkyrotr), includes the upstream directly-mounted list-marker fix. Registry pnpm-lock.yaml restored after optional local validation install. Existing Design Core local built archive retained. Kingfisher 2.7.0 compiled cache reused outside checkouts.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

CommonMark inline Links including angle-bracket destinations and :line suffixes; immutable ChatBodyBlock source offsets; ACP activity frames and terminal records remain their existing v2/v3 wire profiles. Bounded memory-only stream prefix state (512 KiB input, 256 fleet participation buffers); no new persistence format or native identity.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-UI-001/015/018 now explicitly cover Mira list reading, validated
  workspace file-link navigation, display-only historical root labels and
  trailing vertically centered retry actions. JST-COLLAB-031..033 now state
  cross-chunk privacy normalization before scanner/journal/publication. Native
  authority, exact source identities, drafts, authorization, no-replay and
  immutable message bytes remain unchanged. JST-COLLAB-034..036 now apply that
  same boundary explicitly to per-participation fleet buffers.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: "Reviewed M24: remains In progress. Focused source, interaction and live
  read-only evidence qualifies this repair only. Independent adapter,
  permission, native IME, desktop, platform and production cold gates remain
  open; visual baselines are unapproved. Broader diagnostics below remain
  explicit and do not qualify a full suite."

```
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-CHAT-MARKDOWN-001 details 2](details-02.md)
- [E-CHAT-MARKDOWN-001 details 3](details-03.md)
- [E-CHAT-MARKDOWN-001 details 4](details-04.md)
