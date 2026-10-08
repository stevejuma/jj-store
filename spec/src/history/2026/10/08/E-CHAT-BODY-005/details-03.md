---
schema: 1
id: E-CHAT-BODY-005
---

# E-CHAT-BODY-005 details 3

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
pnpm exec prettier --check web/src/lib/ui/store-agent-chat-body/ChatDisclosureIcon.svelte web/src/lib/ui/store-agent-chat-body/ChatTools.svelte web/src/lib/ui/store-agent-chat-body/ChatToolBlocks.svelte web/src/lib/ui/store-agent-chat-body/StoreAgentChatBody.svelte web/src/lib/ui/store-agent-chat-body/chat-body.css web/src/lib/ui/store-agent-chat-body/text-reader.css web/src/lib/ui/store-agent-chat-body/captured-code.ts web/src/lib/ui/store-agent-chat-body/text-editor.ts web/src/lib/ui/store-agent-chat-body/model.ts catalog/fixtures/delta-chat/adapter.ts catalog/fixtures/delta-chat/captured-code.test.ts catalog/components/store-application/AgentChatBody.stories.svelte spec/src/current/contracts/ui/36-commentable-chat-body-prototype.md spec/src/current/contracts/api/41-interactive-agent-execution.md spec/src/current/progress/M24/index.md
```

**Notes**

All matched authored changes pass. Evidence uses its generator formatting and consuming validators.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Stories remain visual-pending; no visual baseline approval or native platform
  IME qualification.
- Inherited architecture audit gap and independent M24
  adapter/permission/cancellation/desktop/platform gates remain open.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Delta 0.1.0 fixture; JJ 0.45.1; Node 26.9.0; pnpm 10.32.1; Svelte 5.57.1; Mira 0.0.2; Storybook 10.5.7; Vitest 4.1.11; Playwright 1.63.0. Existing Design Core sibling file resolution unchanged.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Fixture projection v1 and exact captured text/UTF-8-to-UTF-16 source identities unchanged. Optional lineNumbered is an in-memory display marker only; no HTTP/ACP, WAL, manifest, AgentStore/ReviewStore or persisted schema changes.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

One UI/catalog surface and direct fixture consumers. Exact regressions, full 14-story family, 16 focused units, web/catalog checks, cold automated input, ownership/source budgets and consuming documents apply. Root aggregate/Rust, backend dispatch, publication, visual baselines and native IME remain out of scope. Logs under ignored .local/chat-*; no development service restart/reset.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Review icon-only hover and the captured README sidebar in the running reference. Retain separate visual/IME and independent live integration gates.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-UI-001/JST-UI-015 in ui/36 define the icon region, rounded heading
  and numbered capture projection. JST-API-001 is explicitly unchanged in
  api/41; models only add a display hint and no source bytes or authority
  change.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M24 remains In progress. UI fixture acceptance closes no visual/native
  IME, real-adapter, permissions/cancellation, desktop or platform gate.

```
<!-- /evidence-field -->

