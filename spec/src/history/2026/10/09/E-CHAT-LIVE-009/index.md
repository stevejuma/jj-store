---
schema: 1
id: E-CHAT-LIVE-009
---

# E-CHAT-LIVE-009

<!-- evidence-field:id markdown -->
## Id

E-CHAT-LIVE-009
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M24
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-015
- JST-UI-001
- JST-API-001
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-09
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Execute output projection and plain avatar status dots
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change kvkxyvks
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

The real completed Codex execute tool fell back to its JSON snapshot. Its retained rawOutput.formatted_output existed in the execution journal, but its older committed output held only terminal references. Exact stdout aliases were also omitted by the document source-part filter. Agent avatar presence contained status symbols.
<!-- /evidence-field -->

<!-- evidence-field:findings markdown -->
## Findings

The document adapter accepts stdout, formatted_output and aggregated_output with exact original JSON pointers, unchanged text and Unicode offsets. Empty output remains empty; stderr has a separate label. Terminal-only committed captures display the retained command/output as a Shell section while retaining whole-section comment semantics; they do not invent an immutable text range. Unsupported terminal references retain their raw evidence. The raw toggle remains independent. Agent status indicators contain no symbol and remain circular, with success/primary/warning/destructive/neutral state tones, accessible labels and tooltips. Community tool/avatar source was inspected before released Design Core primitives; no upstream package repair was needed.
<!-- /evidence-field -->

<!-- evidence-field:live markdown -->
## Live review

A fresh page opened sample session ddcc5f115b95bf017564735592db880f. App-owned feedback GET returned only /0/type and /0/terminalId for its committed tool_output target. The matching completed execution event retained rawOutput.formatted_output. After reload, expanding the real command shows its command, README and exact greeting output as Shell; raw/rich switching returns to that section. Agent avatar runtime-symbol count is zero. Final browser warning/error logs are empty, including ResizeObserver. Screenshot: .local/execute-output-fixed.jpg. No agent prompt, filesystem read tool or capture mutation was dispatched. Earlier direct HTTP probe received the expected 403; no admission bypass was attempted. Browser CDP observed the app-authorized requests. Background tabs needed reload to load changed modules.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- web/src/lib/conversations/tool-adapter.ts
- web/src/lib/ui/community-member-avatar/CommunityMemberAvatar.svelte
- web/src/lib/ui/community-member-avatar/community-member-avatar.css
- web/tests/vite/conversation-projection.test.ts
- catalog/components/store-application/AgentChatChanges.stories.svelte
- catalog/components/store-application/AgentTerminalOutput.stories.svelte
- catalog/fixtures/AgentTerminalOutputFixture.svelte
- spec/src/current/contracts/ui/34-interactive-agent-sessions.md
- spec/src/current/contracts/ui/37-document-conversations.md
- spec/src/current/contracts/api/41-interactive-agent-execution.md
- spec/src/current/progress/M24/index.md
- spec/src/current/progress/M24/history/M24/index.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/vite/conversation-projection.test.ts#execute output uses committed stdout parts instead of the terminal snapshot envelope
- web/tests/vite/conversation-projection.test.ts#execute output aliases preserve empty and stderr captures without fabricating terminal text
- web/tests/vite/conversation-projection.test.ts#terminal-only committed captures show retained execution output with whole-section comments
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Broader agent family has 14 failures across six older story files, all reproduced unchanged on parent f11e9b9b. Their default-view/reply/Send and menu expectations require independent follow-up; this slice does not claim that family passed.
- Existing development acceptance script architecture-size gap and locked temporary foundational dependency setup remain separate.
- M24 independent adapter, live permission, native IME and desktop/platform gates remain open. Visual baselines are unapproved.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64; Node 26.9.0; pnpm 10.32.1; Vitest 4.1.11. Existing foundational dependency setup retained. Storybook emits the existing missing-root-Svelte-config notices.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No DTO, immutable capture, publication, receipt or persisted source format changes. Terminal-only captures remain unchanged; displayed journal text cannot become an exact range selector. Existing whole-section selectors remain compatible.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

UI-only document tool adapter and shared agent-avatar status presentation, their direct unit/story consumers and live sample inspection. Wider agent/project families were run because the avatar is shared; parent-baseline reproduction isolates pre-existing failures. No Rust or root aggregate lanes. A disposable JJ baseline workspace used the same existing dependencies and was forgotten and removed after testing; primary live source and fixture were preserved.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Continue M24 qualification and separately repair the older agent stories for the Document default and preserved Classic behavior.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-UI-001 and JST-UI-015 specify execute output projections and plain coloured status dots. JST-API-001 source/feedback/publication contracts are unchanged.
```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M24 stays In progress. Focused repairs do not close independent delivery, permission, platform, native IME or visual gates; older story failures remain explicit.
```
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-CHAT-LIVE-009 validation](details-02.md)
