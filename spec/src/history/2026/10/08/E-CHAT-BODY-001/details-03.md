---
schema: 1
id: E-CHAT-BODY-001
---

# E-CHAT-BODY-001 details 3

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 7
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
```

**Notes**

Pending evidence had three bare test-file references and four uncovered helper/navigation paths. Tests now use the validator-supported named test definitions, with unchanged assertions; regenerated files include fresh coverage.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 7
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first
```

**Notes**

The same pending evidence schema/coverage defects prevented selecting a trustworthy change set. Corrected ledger acceptance is recorded below.
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
pnpm check:architecture
```

**Notes**

Final audit after story extraction and native hover assertions reports only inherited scripts/dev-ui.acceptance.mjs:311 over 300. jj file show -r @- scripts/dev-ui.acceptance.mjs and countedLines returned 311 for both parent and current. No baseline or unrelated script changed.
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
node catalog/fixtures/delta-chat/browser.acceptance.mjs
```

**Notes**

Final cold isolated run: all native acceptance passes, including real CSS hover transparency for group/tool/file headers, raw-icon hover visibility and narrow pointer movement from rail to card. Zero page errors; owned browser and server cleaned up.
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
pnpm web:check
```

**Notes**

Final navigation correction: 0 errors and 0 warnings.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit catalog/fixtures/delta-chat/adapter.test.ts
```

**Notes**

7/7 after selecting standard test() definitions supported by the existing named-evidence validator. Assertions and test behavior are unchanged.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentChatBody.stories.svelte
```

**Notes**

After the native rail-to-card correction, the full affected family passes 11/11 with its original Vitest config and accessibility checks.
<!-- /run -->

<!-- run -->
### Run 8

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

After the native rail-to-card correction: 0 errors, 0 warnings and 313 catalog modules pass boundaries.
<!-- /run -->

<!-- run -->
### Run 9

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

Final evidence schema/path coverage passes for 146 tasks and 668 evidence records. Bookkeeping only; no milestone status expansion.
<!-- /run -->

<!-- run -->
### Run 10

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

Final protected change selection passes: 34 protected files and 3 canonical chapters.
<!-- /run -->

<!-- run -->
### Run 11

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

Final consuming document gates pass: 5 validators, 3232 chapters and 181 requirements.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --write catalog/components/store-application/AgentChatBody.stories.svelte catalog/fixtures/delta-chat/browser.acceptance.mjs spec/src/current/contracts/ui/36-commentable-chat-body-prototype.md
```

**Notes**

Corrected story attributes and transparent-header wording; generated evidence keeps its canonical generator format.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --write web/src/lib/ui/store-agent-chat-body/ChatNavigation.svelte catalog/fixtures/delta-chat/adapter.test.ts
```

**Notes**

Pointer exit cleanup and standard test definitions are formatted.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --check catalog/components/store-application/AgentChatBody.stories.svelte catalog/fixtures/ChatBodyFixture.svelte catalog/fixtures/delta-chat/acceptance-helpers.ts catalog/fixtures/delta-chat/acceptance-plays.ts catalog/fixtures/delta-chat/adapter.test.ts catalog/fixtures/delta-chat/adapter.ts catalog/fixtures/delta-chat/example-source.ts catalog/fixtures/delta-chat/offsets.ts catalog/fixtures/delta-chat/patches.ts catalog/fixtures/delta-chat/playground.ts catalog/fixtures/delta-chat/reference.json spec/src/current/contracts/api/41-interactive-agent-execution.md spec/src/current/contracts/ui/36-commentable-chat-body-prototype.md spec/src/current/governance/spec-governance.md spec/src/current/verification/ui-02.md spec/src/current/verification/ui.md web/src/lib/ui/store-agent-chat-body/ChatCommentArea.svelte web/src/lib/ui/store-agent-chat-body/ChatContent.svelte web/src/lib/ui/store-agent-chat-body/ChatNavigation.svelte web/src/lib/ui/store-agent-chat-body/ChatRawToggle.svelte web/src/lib/ui/store-agent-chat-body/ChatTextBlock.svelte web/src/lib/ui/store-agent-chat-body/ChatToolBlocks.svelte web/src/lib/ui/store-agent-chat-body/ChatToolPanel.svelte web/src/lib/ui/store-agent-chat-body/ChatTools.svelte web/src/lib/ui/store-agent-chat-body/ChatTurnCard.svelte web/src/lib/ui/store-agent-chat-body/StoreAgentChatBody.svelte web/src/lib/ui/store-agent-chat-body/anchors.ts web/src/lib/ui/store-agent-chat-body/comment-widgets.ts web/src/lib/ui/store-agent-chat-body/comments.svelte.ts web/src/lib/ui/store-agent-chat-body/custom-interaction.ts web/src/lib/ui/store-agent-chat-body/markdown-blocks.ts web/src/lib/ui/store-agent-chat-body/markdown-decorations.ts web/src/lib/ui/store-agent-chat-body/model.ts web/src/lib/ui/store-agent-chat-body/navigation-turns.ts web/src/lib/ui/store-agent-chat-body/text-editor.ts web/src/lib/ui/store-agent-chat-body/text-reader.css web/src/lib/ui/store-agent-chat-body/tool-blocks.ts
```

**Notes**

All matched authored component, fixture and contract sources use Prettier style. Generated evidence/navigation are validated by their consuming gates.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Human comparison with the live Delta popover and actual platform IME remain
  unqualified.
- Inherited architecture line-limit failure and existing M24
  real-adapter/permission/desktop/platform gates remain open.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Delta 0.1.0; projection version 1; JJ 0.45.1; Node 26.9.0; pnpm 10.32.1; Svelte 5.57.1; Mira 0.0.2; Storybook 10.5.7; Vitest 4.1.11; Playwright 1.63.0. Design Core uses the existing sibling file-source resolution, not a new released-registry claim.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Delta anchors use UTF-8 bytes; CodeMirror comment offsets use UTF-16 into one exact immutable block. Sanitized UI projection v1 retains captured patch hunks and status. Turn summaries are captured final-response first paragraphs, not newly generated summaries; +175/-31 are captured patch lines, not a reconstructed net repository diff. No storage/WAL/manifest/API or AgentStore format changes.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

One UI/catalog surface and direct fixture consumers. Production lives under web/src/lib; catalog owns projection and local host state. UI type checks, this story family, adapter units, ownership, source budgets, cold UI acceptance and consuming document gates apply. Root aggregates, Rust, production browser/API admission, backend integration, package publication, native platform IME, broad unrelated stories and visual baselines are out of scope. Read tools show captured output only. No development backend restart/reset or installed CLI replacement.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Review the running catalog reference/playground. Qualify live integration through a separately governed event adapter, durable comments and streaming reconciliation; retain independent M24 gates.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-UI-001/JST-UI-015 add the isolated catalog prototype in
  ui/36-commentable-chat-body-prototype.md. JST-API-001, storage, publication
  and live capture authority remain unchanged; owning API/governance notes
  record this boundary.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M24 remains In progress. This UI-only prototype does not qualify
  additional real adapters, live permission/cancellation, packaged
  desktop/platform or visual gates.

```
<!-- /evidence-field -->

