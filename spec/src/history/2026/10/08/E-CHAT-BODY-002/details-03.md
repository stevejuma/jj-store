---
schema: 1
id: E-CHAT-BODY-002
---

# E-CHAT-BODY-002 details 3

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
pnpm catalog:check
```

**Notes**

Final source and composer: 0 errors, 0 warnings; 320 catalog modules pass boundaries.
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
pnpm check:progress
```

**Notes**

Final fresh evidence coverage and schema pass: 146 tasks, 669 records. Evidence bookkeeping only; M24 status remains unchanged.
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
pnpm spec:first
```

**Notes**

Final protected change selection passes: 34 protected files, 2 canonical chapters.
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
pnpm exec spec-validator validate --only summary,fileLimits,governance,verification,book
```

**Notes**

Final consuming document gates pass: 5 validators and 181 requirements. Generated navigation/evidence remain within page limits.
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
pnpm check:ui-ownership
```

**Notes**

Final composer/footer and renderer ownership pass. No production imports of catalog state or Community checkout sources.
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
pnpm exec storybook dev --host 127.0.0.1 --port 6048 --no-open --disable-telemetry --ci
```

**Notes**

The earlier review server was no longer listening (ECONNREFUSED). Restarted only the owned catalog preview in this workspace; final index.json GET returned 200 and contained the Delta main-thread reference. No app backend or fixture reset. Preview remains running.
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
pnpm exec prettier --check catalog/components/store-application/AgentChatBody.stories.svelte catalog/fixtures/ChatBodyFixture.svelte catalog/fixtures/delta-chat/acceptance-helpers.ts catalog/fixtures/delta-chat/acceptance-plays.ts catalog/fixtures/delta-chat/adapter.test.ts catalog/fixtures/delta-chat/adapter.ts catalog/fixtures/delta-chat/large-capture-play.ts catalog/fixtures/delta-chat/large-captures.ts catalog/fixtures/delta-chat/local-reactions.svelte.ts catalog/fixtures/delta-chat/message-action-play.ts catalog/fixtures/delta-chat/middle-fold.test.ts catalog/fixtures/delta-chat/next-message-play.ts catalog/fixtures/delta-chat/next-message.svelte.ts catalog/fixtures/delta-chat/playground.ts spec/src/current/contracts/api/41-interactive-agent-execution.md spec/src/current/contracts/ui/36-commentable-chat-body-prototype.md web/src/lib/ui/store-agent-chat-body/ChatContent.svelte web/src/lib/ui/store-agent-chat-body/ChatEndHint.svelte web/src/lib/ui/store-agent-chat-body/ChatHoverActions.svelte web/src/lib/ui/store-agent-chat-body/ChatMessageFooter.svelte web/src/lib/ui/store-agent-chat-body/ChatNavigation.svelte web/src/lib/ui/store-agent-chat-body/ChatNextMessage.svelte web/src/lib/ui/store-agent-chat-body/ChatTextBlock.svelte web/src/lib/ui/store-agent-chat-body/ChatToolBlocks.svelte web/src/lib/ui/store-agent-chat-body/ChatTools.svelte web/src/lib/ui/store-agent-chat-body/StoreAgentChatBody.svelte web/src/lib/ui/store-agent-chat-body/anchors.ts web/src/lib/ui/store-agent-chat-body/chat-body.css web/src/lib/ui/store-agent-chat-body/middle-fold.ts web/src/lib/ui/store-agent-chat-body/model.ts web/src/lib/ui/store-agent-chat-body/text-editor.ts web/src/lib/ui/store-agent-chat-body/text-reader.css
```

**Notes**

All matched changed authored UI/fixture/contract sources pass formatting; M24/API were also explicitly formatted. Evidence/navigation use their generator format and consuming validators.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Human visual comparison/native platform IME remain unqualified; no baseline
  approval.
- Inherited architecture audit failure and independent M24 real-adapter,
  permission, desktop/platform gates remain open.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Delta reference 0.1.0; JJ 0.45.1; Node 26.9.0; pnpm 10.32.1; Svelte 5.57.1; Mira 0.0.2; Storybook 10.5.7; Vitest 4.1.11; Playwright 1.63.0. Existing sibling Design Core file-source resolution is unchanged.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Retained projection v1 and all raw captured text/bytes are unchanged. Presentation adds context blocks, caller-owned reaction models/callbacks and optional composer/footer snippets. No native storage, WAL, manifest, HTTP/ACP or AgentStore/ReviewStore format changes. Local next-message fixtures have explicit synthetic identities and no agent response.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

One UI/catalog surface and direct fixture consumers. Focused type checks, affected story family, adapter/fold units, cold ephemeral Storybook/native browser, ownership and consuming document gates apply. Root aggregate suites, Rust, backend/agent dispatch, publication, native IME and visual baselines are out of scope. Native checks own/clean only their browser and ephemeral Storybook server; the review preview on 6048 remains running.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Review the running body reference and focused interaction stories. Live streaming adapters, durable comments/reactions and agent submission remain separately governed work.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-UI-001/JST-UI-015 refine the isolated prototype in ui/36.
  JST-API-001 and persistence authority stay unchanged, explicitly confirmed in
  api/41. All callbacks in the catalog are local UI state.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M24 remains In progress. No screenshot, fixture or UI-only result closes
  its real-adapter, permission/cancellation, desktop/platform or visual
  qualification gates.

```
<!-- /evidence-field -->

