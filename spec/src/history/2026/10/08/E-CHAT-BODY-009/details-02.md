---
schema: 1
id: E-CHAT-BODY-009
---

# E-CHAT-BODY-009 details 2

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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentChatBody.stories.svelte -t FooterRuntimeControls
```

**Notes**

Initial missing Model control, 20 skipped. .local/chat-footer-before.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentChatBody.stories.svelte -t FooterRuntimeControls
```

**Notes**

Initial implemented controls: 1 passed, 20 skipped. .local/chat-footer-after.log.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm catalog:check
```

**Notes**

Three uses of Playwright-only exact in Testing Library getByRole options. Removed the unsupported options; string names already match exactly. .local/chat-footer-catalog.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentChatBody.stories.svelte -t FooterRuntimeControls
```

**Notes**

Width regression: expected 128 to be at least 223; 20 skipped. .local/chat-footer-width-before.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentChatBody.stories.svelte -t FooterRuntimeControls
```

**Notes**

1 passed, 20 skipped. Menu width/viewport and no horizontal overflow, state selection, focus return, preserved draft, historical avatar and icon-only Send. .local/chat-footer-width-after.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit catalog/fixtures/delta-chat/runtime.test.ts
```

**Notes**

1 runtime snapshot test. .local/chat-footer-runtime-test.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit catalog/fixtures/delta-chat/adapter.test.ts catalog/fixtures/delta-chat/comments.test.ts catalog/fixtures/delta-chat/runtime.test.ts
```

**Notes**

14 tests across 3 direct consumer files. .local/chat-footer-units.log.
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

0 errors, 0 warnings. .local/chat-footer-web.log.
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

0 errors, 0 warnings; 336 catalog modules pass boundaries. .local/chat-footer-catalog-final.log.
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

Owned UI boundary passes. .local/chat-footer-ownership.log.
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

Unchanged pre-existing scripts/dev-ui.acceptance.mjs has 311 counted lines against 300. Parent and working source are byte-identical (SHA-1 c7d8b75d2516534aae8fa48a07c133f7583a3aed), as recorded in E-CHAT-BODY-008. .local/chat-footer-architecture.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentChatBody.stories.svelte
```

**Notes**

21 affected chat interaction/accessibility stories pass, including comment sends/navigation, shell/sidebar geometry and footer controls. .local/chat-footer-stories-final.log. The earlier pre-width family also passed 21 tests in .local/chat-footer-stories.log.
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
node catalog/fixtures/delta-chat/browser.acceptance.mjs > .local/chat-footer-cold-run.log 2>&1
```

**Notes**

Fresh isolated Storybook/Chromium passes keyboard and pointer input, menu widths at desktop/390px, Escape/focus return, loaded harness image, retained drafts, compact footer and icon-only Send, plus existing comment, code gutter, full-height sidebar, resize and navigation coverage. The browser and private server are cleaned up; the user preview remains running. Vite logged ResizeObserver undelivered-notification diagnostics during geometry checks; the pageerror assertion and all acceptance checks passed.
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

- Unchanged scripts/dev-ui.acceptance.mjs line-limit failure remains open.
- Visual baselines remain unapproved; native IME, live
  adapter/permission/cancellation and desktop qualification remain separate.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Node v26.9.0; pnpm 10.32.1; jj 0.45.1; Vitest 4.1.11; existing released Design Core and Mira resolution unchanged.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No HTTP, ACP, AgentStore or persisted format changes. Existing owned harness assets are reused.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

One catalog-only UI presentation surface. Exact footer regression, affected chat interaction/accessibility family, direct adapter/comment/runtime units, web/catalog type checks, isolated cold browser acceptance, ownership and document checks. Rust, root aggregates, unrelated test families and visual baseline updates are out of scope.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Review the HMR preview on 6048; retain M24 In progress and qualify live integration and independent visual/platform gates separately.
<!-- /evidence-field -->

