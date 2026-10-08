---
schema: 1
id: E-CHAT-BODY-005
---

# E-CHAT-BODY-005 details 2

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
jj --no-pager st
```

**Notes**

Clean before edits; preserve existing focus-clearance and prototype changes.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentChatBody.stories.svelte -t 'AppBodyShell|MultiFile'
```

**Notes**

Before icon markup edits: 1 matching story passed, 13 skipped. The complete final family covers all file-disclosure stories.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentChatBody.stories.svelte -t DeltaMainThreadReference
```

**Notes**

Exact before syntax regression: the numbered README had no cm-heading node. 1 failed, 13 skipped; chat-readme-before.log. This assertion requires the actual heading text and a computed color distinct from ordinary source text.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit catalog/fixtures/delta-chat/captured-code.test.ts
```

**Notes**

Initial 2/3 passed. The test selected the first heading token (#), but the grammar splits its marker and content. It now selects the highlighted heading content while retaining all source/offset expectations; no production algorithm weakening.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit catalog/fixtures/delta-chat/captured-code.test.ts
```

**Notes**

Exact 3/3 pass: numbered Markdown headings/links, neutral prefixes, multiline Rust comments, Unicode UTF-16 positions, unnumbered footer content and honest unknown-language fallback.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentChatBody.stories.svelte -t 'DeltaMainThreadReference|AppBodyShell'
```

**Notes**

2 selected stories pass, 12 skipped. README heading text has syntax color; icon markup preserves adjacent glyph geometry, shell/read focus, transparent parent and navigation behavior.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentChatBody.stories.svelte -t DeltaMainThreadReference
```

**Notes**

Exact original syntax target rerun: 1 passed, 13 skipped; heading content/color regression passes.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentChatBody.stories.svelte
```

**Notes**

14/14 affected interaction/accessibility stories, with the actual captured README regression included. Glyph proximity, file collapse/comment retention, read-pane focus, immutable source, XML/reactions, navigation and draft/footer remain covered.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit catalog/fixtures/delta-chat/adapter.test.ts catalog/fixtures/delta-chat/middle-fold.test.ts catalog/fixtures/delta-chat/patch-presentation.test.ts catalog/fixtures/delta-chat/captured-code.test.ts
```

**Notes**

Final 16/16 focused units. Existing +175/-31 captured patch totals, Unicode mapping, adapter identity and folds remain qualified.
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
pnpm web:check
```

**Notes**

0 errors, 0 warnings.
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
pnpm catalog:check
```

**Notes**

0 errors, 0 warnings; 322 catalog modules pass boundaries.
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
node catalog/fixtures/delta-chat/browser.acceptance.mjs
```

**Notes**

Cold existing automated pointer/keyboard harness passes with fresh isolated Storybook/browser and zero page errors. Immutable comments, real hover transparency, focus, custom sections, resize/remount/mobile reads, folds, reactions and multiline draft remain covered. Processes clean up; no direct browser-tool retry or visual baseline comparison.
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
pnpm check:ui-ownership
```

**Notes**

Owned Community tool/source-tokenization references inspected before public Design Core ghost-button tokens. New component/helpers are app-owned; Mira public APIs and released dependencies only. No Community copy/provenance change or production catalog import.
<!-- /run -->

<!-- run -->
### Run 14

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

Only inherited scripts/dev-ui.acceptance.mjs:311 counted lines over 300. A read-only comparison proves it is byte-identical to parent ef33f926. Changed sources meet their budgets; no unrelated script or baseline change.
<!-- /run -->

<!-- run -->
### Run 15

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

146 tasks; 672 records. New evidence schema/coverage passes; M24 unchanged.
<!-- /run -->

<!-- run -->
### Run 16

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

12 protected files, 2 owning canonical chapters.
<!-- /run -->

<!-- run -->
### Run 17

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

5 validators; 181 requirements. Evidence/navigation stay within Markdown limits.
<!-- /run -->

<!-- run -->
### Run 18

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

Authored milestone formatted after navigation generation.
<!-- /run -->
<!-- /evidence-field -->

