---
schema: 1
id: E-TEST-REPAIR-002
---

# E-TEST-REPAIR-002

<!-- evidence-field:schema yaml -->
## Schema

```yaml
1

```
<!-- /evidence-field -->

<!-- evidence-field:id markdown -->
## Id

E-TEST-REPAIR-002
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-10
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
- JST-COLLAB-034
- JST-COLLAB-035
- JST-COLLAB-036
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-application/AgentDiscussion.stories.svelte
- catalog/components/store-application/AgentReplyIndependence.stories.svelte
- catalog/fixtures/AgentInlineCommentsFixture.svelte
- catalog/fixtures/agent-inline-comment-play.ts
- catalog/fixtures/agent-inline-comment-transport.ts
- spec/src/current/progress/M24/SUMMARY.index-index/002/index.md
- spec/src/current/progress/M24/history/M24/index.md
- spec/src/current/progress/M24/index.md
- spec/src/current/progress/M24/test-repairs-review.md
- spec/src/history/2026/10/10/E-TEST-REPAIR-002/SUMMARY.nav.md
- spec/src/history/2026/10/10/E-TEST-REPAIR-002/details-02.md
- spec/src/history/2026/10/10/E-TEST-REPAIR-002/details-03.md
- spec/src/history/2026/10/10/E-TEST-REPAIR-002/index.md
- spec/src/history/2026/10/10/SUMMARY.nav.md
- spec/src/history/2026/10/10/index.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/vite/agent-discussions.test.ts#an ended agent accepts ordinary
  replies after feedback refresh fails
- web/tests/vite/agent-discussions.test.ts#discussion without a feed loads once
  and never polls
- web/tests/vite/conversation-projection.test.ts#several validated answers
  retain ordinary prose in one turn and shared comments use one destination
- web/tests/vite/conversation-projection.test.ts#foreign or malformed
  unvalidated tags stay readable and changed captures disable selection
- web/tests/vite/conversation-projection.test.ts#multiple agents merge immutable
  comment revisions while preserving separate answer links
- web/tests/vite/conversation-projection.test.ts#patch file and numbered-read
  slices preserve exact captured pointers and Unicode offsets
- web/tests/vite/conversation-projection.test.ts#recorded rich captures without
  a plain-text mapping support whole sections without inventing ranges
- web/tests/vite/conversation-projection.test.ts#shared answers repeated in
  feedback and spanning tool boundaries have stable unique fragments
- web/tests/vite/conversation-projection.test.ts#committed comments-only user
  rows retain their canonical batch and turn
- web/tests/vite/conversation-projection.test.ts#execute output uses committed
  stdout parts instead of the terminal snapshot envelope
- web/tests/vite/conversation-projection.test.ts#execute output aliases preserve
  empty and stderr captures without fabricating terminal text
- web/tests/vite/conversation-projection.test.ts#terminal-only committed
  captures show retained execution output with whole-section comments
- web/tests/vite/conversation-projection.test.ts#recorded tools retain command
  and output without writable capture targets
- web/tests/vite/conversation-reactions.test.ts#linked answer reactions use the
  capture's repository reader and original row identity
- web/tests/vite/conversation-session.test.ts#session drafts survive remounts
  and exact publication reconciles without duplicates
- web/tests/vite/conversation-session.test.ts#unknown publication freezes the
  original snapshot and rejects duplicate sends until receipt recovery
- web/tests/vite/conversation-session.test.ts#editing a sent comment retains its
  old answered revision and prepares a checked replacement
- web/tests/vite/conversation-session.test.ts#published mixed submission with
  failed delivery retains one request without replaying delivered recipients
- web/tests/vite/conversation-session.test.ts#successful receipt clears only the
  original text and retains the canonical publication ID
- web/tests/vite/conversation-session.test.ts#authoritative tombstones remove
  published revisions while unavailable reads and local drafts retain them
<!-- /evidence-field -->

<!-- evidence-field:story_cases list -->
## Story cases

- catalog/components/store-application/AgentDiscussion.stories.svelte#Drafts at
  different message anchors remain independent and submit together
- catalog/components/store-application/AgentDiscussion.stories.svelte#Immutable
  message reactions use the shared toolbar and reaction bar
- catalog/components/store-application/AgentDiscussion.stories.svelte#Inline
  range comments synchronize the queue survive remount and send exact sources
- catalog/components/store-application/AgentDiscussion.stories.svelte#Unsupported
  repositories explain unavailable comments without dispatch
- catalog/components/store-application/AgentDiscussion.stories.svelte#Unpublished
  assistant content stays readable until exact commenting is available
- catalog/components/store-application/AgentDiscussion.stories.svelte#Unknown
  comment batches retain the exact request for receipt recovery
- catalog/components/store-application/AgentReplyIndependence.stories.svelte#Offline
  inline drafts stay editable through failed feedback until the session resumes
- catalog/components/store-application/AgentReplyIndependence.stories.svelte#Multiple
  comments and exact revision edits share an anchor while the agent works
- catalog/components/store-application/AgentReplyIndependence.stories.svelte#Sent
  context and captured tool output use the current inline comment queue
- catalog/components/store-application/AgentReplyIndependence.stories.svelte#Consecutive
  tool disclosures retain raw mode and exact pending sources independently
<!-- /evidence-field -->

<!-- evidence-field:removed_tests list -->
## Removed tests

- catalog/components/store-application/AgentDiscussion.stories.svelte#Resume
  replay preserves exact replies and recovers a retained draft
- catalog/components/store-application/AgentDiscussion.stories.svelte#Exact
  reactions share the message toolbar and bar
- catalog/components/store-application/AgentDiscussion.stories.svelte#Saved
  replies retain drafts and do not prompt
- catalog/components/store-application/AgentDiscussion.stories.svelte#Unsupported
  repository retains reply draft without dispatch
- catalog/components/store-application/AgentDiscussion.stories.svelte#Reply
  waits for original publication
- catalog/components/store-application/AgentDiscussion.stories.svelte#Unknown
  reply keeps exact request for status recovery
- catalog/components/store-application/AgentReplyIndependence.stories.svelte#Offline
  replies stay editable and save after feedback refresh fails
- catalog/components/store-application/AgentReplyIndependence.stories.svelte#Ordinary
  nested replies and edits stay independent of a working agent
- catalog/components/store-application/AgentReplyIndependence.stories.svelte#Shared
  chat rows show timed lifecycle XML context and exact tool replies
- catalog/components/store-application/AgentReplyIndependence.stories.svelte#Consecutive
  tools expand together and retain exact call actions
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-TEST-REPAIR-002 details 2](details-02.md)
- [E-TEST-REPAIR-002 details 3](details-03.md)
