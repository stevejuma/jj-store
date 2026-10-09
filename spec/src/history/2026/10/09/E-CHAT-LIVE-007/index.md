---
schema: 1
id: E-CHAT-LIVE-007
---

# E-CHAT-LIVE-007

<!-- evidence-field:id markdown -->
## Id

E-CHAT-LIVE-007
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M24
- M25
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-015
- JST-COLLAB-034
- JST-COLLAB-035
- JST-COLLAB-036
- JST-API-001
- JST-MUT-001
- JST-CLI-004
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-09
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Installed Codex sample-repository live acceptance and regression repairs
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change szuqsumm; jj-acp source change lwlxyuwu / 9a3d50a4
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

The controlled native/local flows passed in E-CHAT-LIVE-006, but the preserved development fixture had not exercised real installed Codex with the document comments. The first live send crashed the native service; later attempts exposed a blocked catalog preview, stale helper and stranded receipt recovery.
<!-- /evidence-field -->

<!-- evidence-field:findings markdown -->
## Findings

1. Native session detail nested a 71,232-byte future inside conversation origin publication and overflowed the default request stack. Heap allocation preserves the exact reader contract and makes the size regression pass.
2. Background managed-local model discovery started compute while holding mutation admission and entered macOS credential interaction. The process sample identifies that path. Discovery now uses only an already-running endpoint and reports unavailable otherwise; explicit starts retain authority.
3. The staged control-version-3 helper predated the committed separate-context implementation and rejected context as invalid_request. Current jj-acp also rejected empty authored text with nonempty prepared context. The source fix accepts that comments-only case; empty/whitespace-only input stays invalid. The rebuilt helper is staged in an ignored validation directory, not installed globally.
4. Managed chat cleared pending at dispatching admission, leaving the shared composer with no usable recovery action. It now retains the exact request until acknowledgement; matching live events read exact child receipts without another send. Failure/unknown states remain recoverable.
5. A committed empty authored-text part produced no presentation segments, removing its comments-only sent summary. Empty user rows now retain their canonical batch and turn when feedback becomes committed.
6. Cold screenshot inspection exposed 494px of trailing padding per inline Markdown document after another Mira stylesheet loaded. The owned Markdown integration now uses Mira's existing embed mode; a late-styles regression fails at 450px before the fix and passes at zero after it. Live comment cards are 24px tall. This is consumer configuration, not an upstream workaround.
<!-- /evidence-field -->

<!-- evidence-field:live_run markdown -->
## Live run

The in-app browser used http://127.0.0.1:7460/ against the preserved main development fixture and the chat-body source/binary. The Local Git-backed JJ sample is 61c5a6c6e2dfb74cd9d636dee2138b41; native personal metadata home e1fb58c68afe46ad9943fe5ee56230dd publishes its conversation records. No source edit was requested; observed tools only read README.md and src/example.ts.

Installed Codex ACP 1.13.1, Nyra, advertised gpt-5.6-luna/medium produced the real response. Session ddcc5f115b95bf017564735592db880f / conversation 9897e726022a4e35a347efad10b7442e then received three comments at two exact anchors: two at Browser serving is pending, one at greeting. Add another creates the second same-anchor comment. Editing the queue updates inline text. Document to Classic to Document retains all three drafts. Cmd+Enter from the comment editor sends a comments-only prompt.

The agent emitted two comment-reply blocks in one assistant turn, sharing one answer for the first two comments and returning café 🦀 in the other. No protocol instructions were added to authored text. Three green reply buttons navigate to two validated persisted destinations. Go to source focuses the original greeting selection. The comments-only summary survives publication and remount. First clean backend restart retained all 353 original events and two links without replay; an explicit later Resume adds lifecycle events, reaching 372, without another prompt.

A second managed-sidebar session a9692fd4cbeaacdb6d93b83d47a21d81 / conversation 1ddc9b8084e94d548ed30ddd0c222957 verifies automatic acknowledgement and a mixed text/comment send. Both clear automatically without Check retained request. The real reply becomes a persisted exact destination; pending count is zero and composer is empty. Sent context visibly contains the separate comment_reply_protocol and immutable supplied references. The second session publishes all 199 events and one answer link. The final fresh Vite/backend startup preserves the fixture and all three links. A final Go to reply click focuses the exact committed answer after restart. The retained browser screenshot is .local/chat-live-verified.jpg; it shows both answers in the same turn, compact comment cards and the expected interrupted/no-replay status after restart.

Earlier failed/unknown sessions remain in history. Check retained request and Keep unresolved in history were exercised, with no automatic replay. Restart-related interrupted status is retained until explicit Resume. A transient Agent execution signal timed out notification occurred during resume and cleared after live status recovered; it did not duplicate a prompt.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-service/src/collaboration/agent_details.rs
- crates/jj-store-service/tests/agent_sessions.rs
- crates/jj-store-server/src/agent_execution/local_agent.rs
- crates/jj-store-server/src/agent_execution/provider_catalog.rs
- crates/jj-store-cli/tests/agent_execution/providers.rs
- web/src/lib/ui/community-markdown-content/CommunityMarkdownContent.svelte
- catalog/components/store-application/ConversationMarkdown.stories.svelte
- web/src/lib/controller/agent-chat.svelte.ts
- web/src/lib/controller/agent-chat-receipts.ts
- web/src/lib/conversations/agent-adapter.ts
- web/tests/vite/agent-chat.test.ts
- web/tests/vite/conversation-projection.test.ts
- spec/src/current/contracts/ui/37-document-conversations.md
- spec/src/current/contracts/collaboration/40-local-compute-and-agent-configuration.md
- spec/src/current/contracts/cli/35-interactive-agent-execution.md
- spec/src/current/contracts/api/41-interactive-agent-execution.md
- spec/src/current/contracts/mutations/22-interactive-agent-execution.md
- spec/src/current/progress/M24/index.md
- spec/src/current/progress/M25/index.md
- spec/src/current/progress/M24/history/M24/index.md
- spec/src/current/progress/M25/history/M25/index.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-service/tests/agent_sessions.rs#session_detail_resolves_current_stable_identity_and_exact_links
- crates/jj-store-cli/tests/agent_execution/providers.rs#managed_catalog_does_not_start_compute_or_mutate_the_journal
- web/tests/vite/agent-chat.test.ts#dispatching retains the exact request until an agent acknowledgement
- web/tests/vite/agent-chat.test.ts#live acknowledgement refreshes the retained receipt without dispatching again
- web/tests/vite/conversation-projection.test.ts#committed comments-only user rows retain their canonical batch and turn
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Independent M24 live permission/cancellation, other adapters, native IME, platform/desktop and visual gates remain; these real Codex runs do not guarantee model compliance.
- M25 signed macOS/WKWebView/Linux and paired-inference qualification remains separate.
- Pre-existing scripts/dev-ui.acceptance.mjs has 311 counted lines against a 300 limit. No cap or baseline was relaxed.
- Historical quote fragments retain exact selected Markdown bytes; the greeting selection includes a leading delimiter. No quote-based relocation or historical rewriting was introduced.
- Temporary staged helper must be explicitly selected on future validation restarts until ordinary distribution resources are rebuilt. No package release or installed CLI replacement was performed.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64; Rust 1.98.1; JJ 0.45.1; Node 26.9.0; pnpm 10.32.1; Vitest 4.1.11; installed Codex ACP 1.13.1; advertised gpt-5.6-luna, medium. Owned debug CLI/server and rebuilt jj-acp control version 3. Linker emitted the existing __eh_frame size warning; Vitest emitted Svelte-config and Node deprecation notices.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No new durable or public DTO format. ThreadStore selectors, UTF-8 hashes, AgentStore captures, versioned comment-reply links, manifest authority and request receipts retain existing identities. Prepared-context wire acceptance is repaired in its owning jj-acp source.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Focused cross-layer live acceptance and its direct service/server/controller/presentation boundaries. No root aggregate, Rust workspace, remote deployment, package publication, main-source modification, fixture reset or visual-baseline update. Main fixture state contains the authorized sample conversations.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Review the retained live sessions and continue the independently gated M24/M25 qualification; rebuild regular development/distribution resources from the fixed helper when integrating.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: UI and collaboration/CLI chapters require receipt-safe acknowledgement,
  committed comments-only summaries and read-only catalog discovery.
  JST-MUT-001 and JST-API-001 keep their authority and public formats; the
  detail-reader allocation is an unchanged-contract repair with focused proof.
```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M24 and M25 remain In progress. Real installed Codex now has comments-only,
  mixed-send and linked-answer evidence, while independent permissions, adapters,
  IME, desktop/platform and visual gates stay open.
```
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-CHAT-LIVE-007 validation 1](details-02.md)
- [E-CHAT-LIVE-007 validation 2](details-03.md)
- [E-CHAT-LIVE-007 final validation](details-04.md)
