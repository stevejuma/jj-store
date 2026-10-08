---
schema: 1
id: E-CHAT-BODY-008
---

# E-CHAT-BODY-008 details 2

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentChatBody.stories.svelte -t ResizableReadSidebar
```

**Notes**

Initial missing resize control. After user selected full-height shell composition, the updated exact regression also failed once before wiring the shell slider. After composition, a no-gap assertion reproduced the 8px default gap. Reusing CommunityAppShell fixes it without a gap override. Logs: .local/chat-resize-before.log, .local/chat-sidebar-before.log, .local/chat-sidebar-gap-before.log.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm catalog:check
```

**Notes**

An early body-only resize prototype produced a Svelte static accessibility warning. That temporary implementation was removed when switching to the standard shell resize control. Final source has no suppressions or warnings.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentChatBody.stories.svelte -t ResizableReadSidebar
```

**Notes**

The initial header-height probe used the synthetic playground toolbar, whose extra demo controls wrap. The geometry probe now uses the actual reference story; the original 107.578125 versus 49 probe is retained in .local/chat-sidebar-header-before.log.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentChatBody.stories.svelte -t CapturedLineNumberGutter
```

**Notes**

Before gutter implementation: no captured-number gutter. .local/chat-gutter-before.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentChatBody.stories.svelte -t 'CapturedLineNumberGutter|ResizableReadSidebar'
```

**Notes**

Both exact regressions pass; 18 skipped. .local/chat-gutter-after.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit catalog/fixtures/delta-chat/captured-code.test.ts catalog/fixtures/delta-chat/middle-fold.test.ts
```

**Notes**

7 unit tests across 2 files; gutter labels/gaps/Unicode offsets, syntax projection and middle-fold anchors. .local/chat-final-unit.log.
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

20 affected chat interaction/accessibility stories pass. Includes full-height/no-gap geometry, keyboard resizing and bounds, retained widths, matching reference header dividers, wrapped-line/gutter alignment, folded labels, immutable source and exact comment selection. .local/chat-final-stories.log.
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
pnpm web:check
```

**Notes**

0 errors and 0 warnings. .local/chat-final-web.log.
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
pnpm catalog:check
```

**Notes**

0 errors and 0 warnings; catalog boundaries pass (333 modules). .local/chat-final-catalog.log.
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
pnpm check:ui-ownership
```

**Notes**

Owned presentation boundary passes. .local/chat-final-ownership.log.
<!-- /run -->

<!-- run -->
### Run 11

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

Unchanged pre-existing scripts/dev-ui.acceptance.mjs has 311 counted lines versus the 300 limit. Parent and working SHA-1 both c7d8b75d2516534aae8fa48a07c133f7583a3aed. No changed implementation exceeds the budget.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress; pnpm spec:first
```

**Notes**

Initial evidence scaffold used a prose result instead of the required pass enum. Both evidence consumers rejected it. A second attempt used pass with remaining gaps, also rejected by both consumers. Final metadata uses pending with the unchanged qualification gaps retained.
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
node catalog/fixtures/delta-chat/browser.acceptance.mjs > .local/chat-final-cold.log 2>&1
```

**Notes**

Fresh isolated Storybook/Chromium: pointer drag, bounds, full-height joined edges, stable comment editor DOM/focus, retained drafts, fixed footer, constrained/mobile/restore, close/reopen/raw views, gutter wrapping/folding, matching dividers and file-switch width retention. Existing comment/keyboard/touch scenarios pass. Browser/server cleaned up; user preview untouched.
<!-- /run -->

<!-- run -->
### Run 14

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
### Run 15

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm spec:first
```
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm exec spec-validator validate --only summary,fileLimits,governance,verification,book
```
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm exec prettier --check <changed authored files>
```
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Unrelated existing scripts/dev-ui.acceptance.mjs line-limit failure remains
  open.
- Visual baselines are unapproved; native IME, live
  adapter/permission/cancellation and desktop qualification remain separate.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Node v26.9.0; pnpm 10.32.1; jj 0.45.1; Vitest 4.1.11; existing Design Core and Mira resolution unchanged.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Captured immutable text/UTF-16 anchors unchanged; no HTTP, ACP, AgentStore or persisted format changes.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

One UI prototype presentation surface. Affected chat Storybook family, exact code/folding units, web/catalog types, cold isolated browser, ownership and document checks. Rust, root aggregate, unrelated suites and visual baseline updates are out of scope.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Review the existing 6048 reference story; keep M24 In progress and qualify live integration/visual gates independently.
<!-- /evidence-field -->

