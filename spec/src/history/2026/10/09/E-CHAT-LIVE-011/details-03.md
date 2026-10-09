---
schema: 1
id: E-CHAT-LIVE-011
---

# E-CHAT-LIVE-011 details 3

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
node --input-type=module (updateNavigation from scripts/spec/navigation-update.mjs)
```
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm spec:validate --only governance,summary,fileLimits,book
```
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first
```

**Notes**

Generated M25 navigation formatting lacked fresh evidence. Restored that unrelated noise before the final gate.
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
pnpm exec prettier --check catalog/components/store-application/AgentChatChanges.stories.svelte catalog/components/store-application/AgentChatLayout.stories.svelte catalog/components/store-application/AgentChatRuntime.stories.svelte catalog/components/store-application/AgentProviderConfiguration.stories.svelte catalog/components/store-application/ConversationContinuity.stories.svelte catalog/fixtures/ConversationContinuityFixture.svelte docs/ui-provenance.json spec/src/current/contracts/api/41-interactive-agent-execution.md spec/src/current/contracts/ui/35-compute-defaults-and-sent-context.md spec/src/current/contracts/ui/37-document-conversations.md spec/src/current/governance/spec-governance.md spec/src/current/progress/M24/index.md web/src/lib/ui/community-chat-composer/CommunityChatComposer.svelte web/src/lib/ui/community-chat-composer/CommunityComposerSendActions.svelte web/src/lib/ui/community-chat-composer/CommunityComposerSurface.svelte web/src/lib/ui/community-chat-composer/composer-document-actions.svelte.ts web/src/lib/ui/community-chat-composer/composer-selection.ts web/src/lib/ui/community-chat-composer/props.ts web/src/lib/ui/store-agent-chat/StoreAgentChatSidebar.svelte web/src/lib/ui/store-agent-chat/StoreLiveConversation.svelte web/src/lib/ui/store-agent-chat-body/ChatMessageFooter.svelte web/src/lib/ui/store-agent-chat-body/ChatNextMessage.svelte web/src/lib/ui/store-agent-chat-body/StoreAgentChatBody.svelte web/src/lib/ui/store-agent-chat-body/model.ts web/src/lib/ui/store-agent-execution/StoreAgentConversation.svelte web/src/lib/ui/store-collaboration/StoreRecordedConversation.svelte web/src/lib/ui/store-settings/StoreSettingsDisclosure.svelte web/src/lib/ui/store-threads/StoreThreadConversation.svelte web/src/lib/ui/store-threads/StoreThreadPendingActions.svelte
```

**Notes**

Every changed source, attribution and current contract file passes.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm check:progress
```
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm spec:validate --only governance,summary,fileLimits,book
```
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
pnpm spec:first
```

**Notes**

Final gate after restoring unrelated generated M25/Bootstrap formatting.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Sixteen older story failures reproduce on unchanged parent; this is not a
  full-suite pass.
- Existing development acceptance-script size gap and temporary foundational
  dependency setup remain separate.
- M24 independent provider/permission, native IME, desktop/platform and
  visual-baseline gates remain open.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64; Node 26.9.0; pnpm 10.32.1; Vitest 4.1.11. Existing temporary foundational dependency validation setup retained. Storybook emits pre-existing missing-root-Svelte-config notices.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No API, request, publication, receipt, preference or immutable source format changes. Editor actions/selection are component-owned session state. Earlier E-CHAT-LIVE-010 generated navigation now links its existing validation page; retained evidence fields are unchanged.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

UI input layout and square disclosure hover, shared Classic/Document editor presentation and direct consumer acceptance. The 53-file surface run is required because the extracted default Classic surface serves ordinary forms/reviews while Document serves all conversation hosts. No Rust or root aggregate lanes, backend rebuild/reset, preference save or new live prompt.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Continue independent M24 qualification and repair the separately reproduced older story expectations.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-UI-001/012/015/018 specify inline Document input, fixed footer
  controls, draft/selection parity and square Advanced hover. JST-API-001
  contracts are unchanged.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M24 remains In progress; baseline story failures, native IME,
  independent execution and visual gates remain explicit.

```
<!-- /evidence-field -->

