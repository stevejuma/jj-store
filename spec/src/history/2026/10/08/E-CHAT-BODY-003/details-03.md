---
schema: 1
id: E-CHAT-BODY-003
---

# E-CHAT-BODY-003 details 3

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
pnpm spec:first
```

**Notes**

8 protected files, 2 canonical owning chapters.
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
pnpm exec spec-validator validate --only summary,fileLimits,governance,verification,book
```

**Notes**

5 validators, 3240 chapters, 181 requirements. Generated evidence/navigation within Markdown limits.
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
pnpm exec prettier --check catalog/fixtures/delta-chat/adapter.test.ts catalog/fixtures/delta-chat/patch-presentation.test.ts web/src/lib/ui/store-agent-chat-body/ChatTools.svelte web/src/lib/ui/store-agent-chat-body/ChatToolBlocks.svelte web/src/lib/ui/store-agent-chat-body/navigation-turns.ts web/src/lib/ui/store-agent-chat-body/patch-lines.ts web/src/lib/ui/store-agent-chat-body/text-editor.ts web/src/lib/ui/store-agent-chat-body/text-reader.css spec/src/current/contracts/ui/36-commentable-chat-body-prototype.md spec/src/current/contracts/api/41-interactive-agent-execution.md spec/src/current/progress/M24/index.md
```

**Notes**

Navigation generation reformatted the M24 table after its authored formatting. Only M24 failed. Reformatting after generation resolves this; no product source failure.
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
pnpm exec prettier --write spec/src/current/progress/M24/index.md
```

**Notes**

Applied authored formatting after navigation generation.
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
pnpm exec prettier --check catalog/fixtures/delta-chat/adapter.test.ts catalog/fixtures/delta-chat/patch-presentation.test.ts web/src/lib/ui/store-agent-chat-body/ChatTools.svelte web/src/lib/ui/store-agent-chat-body/ChatToolBlocks.svelte web/src/lib/ui/store-agent-chat-body/navigation-turns.ts web/src/lib/ui/store-agent-chat-body/patch-lines.ts web/src/lib/ui/store-agent-chat-body/text-editor.ts web/src/lib/ui/store-agent-chat-body/text-reader.css spec/src/current/contracts/ui/36-commentable-chat-body-prototype.md spec/src/current/contracts/api/41-interactive-agent-execution.md spec/src/current/progress/M24/index.md
```

**Notes**

All matched authored sources pass. Evidence pages use generator formatting and consuming document checks.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Live visual Delta comparison remains unverified after native timeout,
  unavailable screen capture and browser policy rejection; supplied screenshots
  and source were reviewed. No visual baseline approval.
- Native platform IME and independent M24 adapter/permission/desktop/platform
  gates remain open.
- Inherited architecture source-budget failure remains separate.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Delta 0.1.0; JJ 0.45.1; Node 26.9.0; pnpm 10.32.1; Svelte 5.57.1; Mira 0.0.2; Storybook 10.5.7; Vitest 4.1.11; Playwright 1.63.0. Existing inherited Design Core sibling file resolution unchanged.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Fixture projection v1, exact source bytes/offsets, UI public models and all persistence/HTTP/ACP/AgentStore formats unchanged. Patch classification is a display-only derivation; no net diff or missing source coordinates inferred.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

One UI/catalog surface and direct consumers. Exact units, 14-story family, web/catalog checks, cold automated fixture input, ownership/source budgets and consuming document gates. Root aggregate suites, Rust, backend/dispatch, releases, baselines and native IME remain out of scope. Test processes clean up independently; review server on 6048 remains available. Command logs are ignored .local/chat-review-*; shell redirection does not change the recorded test selection.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Review live Delta visually when native/browser inspection is available. Keep live integration, streaming, durable discussions/reactions and backend Send separately governed.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-UI-001/JST-UI-015 in ui/36 describe title alignment, full-line
  backgrounds and header-aware captured counts. JST-API-001 remains explicitly
  unchanged in api/41; no callback/persistence authority change.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M24 stays In progress. Focused UI/screenshot review and automated
  fixture acceptance close no live visual, native IME, real-adapter,
  permissions/cancellation, desktop or platform gate.

```
<!-- /evidence-field -->

