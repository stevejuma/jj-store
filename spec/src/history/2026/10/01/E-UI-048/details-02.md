---
schema: 1
id: E-UI-048
---

# E-UI-048 details 2

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 6
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/NativeMessageActions.stories.svelte catalog/components/store-application/NativeReviewDetail.stories.svelte catalog/components/store-application/RepositoryLayout.stories.svelte
```

**Notes**

Baseline: flattened copy actions and missing Overview inset fail five scenarios; the initial review scroll fixture fit the viewport and was constrained to reproduce overflow. One scenario passes.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/NativeReviewDetail.stories.svelte catalog/components/store-application/StoreFileEditor.stories.svelte
```

**Notes**

Before: scrolling moves the review header 240px; Mira gutter starts 79px from the pane edge. Two other stories pass.
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
pnpm exec playwright test --config web/playwright.config.ts web/tests/browser/file-editor-navigation.spec.ts
```

**Notes**

Before: two tests pass; strict production CSP blocks the injected Mira theme, so centering is reproduced by Storybook and the development demo.
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
pnpm web:check
```

**Notes**

Zero errors and warnings.
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
pnpm check:architecture
```

**Notes**

445 modules pass.
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
pnpm check:boundaries
```

**Notes**

15 packages pass.
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
pnpm web:build
```

**Notes**

Initial fresh output builds; existing dependency CSS and chunk-size warnings retained.
<!-- /run -->

<!-- run -->
### Run 8

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

Testing Library role options reject Playwright exact:true; changed the exact Copy match to a regular expression.
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

Zero errors/warnings and 111 catalog modules pass after option repair.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/NativeMessageActions.stories.svelte catalog/components/store-application/NativeReviewDetail.stories.svelte catalog/components/store-application/RepositoryLayout.stories.svelte catalog/components/store-application/StoreFileEditor.stories.svelte
```

**Notes**

Initial runner optimized new icons and reloaded during import: two file import failures and one message failure; no configuration changes. repository-polish-after.log retains diagnostics.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/NativeMessageActions.stories.svelte catalog/components/store-application/NativeReviewDetail.stories.svelte catalog/components/store-application/RepositoryLayout.stories.svelte catalog/components/store-application/StoreFileEditor.stories.svelte
```

**Notes**

Fresh runner: eight stories pass; message helper unnecessarily opened Copy before Edit and raced root-menu focus. Helper now waits for focus and opens only the requested menu.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/NativeMessageActions.stories.svelte
```

**Notes**

Three message stories pass: keyboard submenu, full clipboard values, five distinct decorative icons, editing and unknown-outcome draft retention.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/NativeMessageActions.stories.svelte catalog/components/store-application/NativeReviewDetail.stories.svelte catalog/components/store-application/RepositoryLayout.stories.svelte catalog/components/store-application/StoreFileEditor.stories.svelte
```

**Notes**

Final exact regression: nine interaction/accessibility stories pass. repository-polish-regression-final.log.
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
pnpm exec playwright test --config web/playwright.config.ts web/tests/browser/file-editor-navigation.spec.ts web/tests/browser/repository-navigation.spec.ts web/tests/browser/editor-focus.spec.ts
```

**Notes**

Collection stopped because editor-focus extracted an old inline CSP source location. Updated to the actual STRICT_CSP constant with an explicit missing-policy error; production policy unchanged.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec playwright test --config web/playwright.config.ts web/tests/browser/file-editor-navigation.spec.ts web/tests/browser/repository-navigation.spec.ts web/tests/browser/editor-focus.spec.ts
```

**Notes**

Five scenarios pass; two navigation expectations omitted the existing Projects tab. Added the canonical seventh label; no production tab changes.
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
pnpm exec playwright test --config web/playwright.config.ts web/tests/browser/file-editor-navigation.spec.ts web/tests/browser/repository-navigation.spec.ts web/tests/browser/editor-focus.spec.ts
```

**Notes**

Seven cold built-app scenarios pass: editor width, focus under strict production CSP, navigation and mobile Overview inset. repository-polish-browser-repaired.log.
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/NativeAgentMenus.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte catalog/components/store-application/StoreCollaboration.stories.svelte catalog/components/store-application/RepositoryReview.stories.svelte catalog/components/store-work/StoreWork.stories.svelte
```

**Notes**

22 stories pass across Agents, Reviews, Tasks and connected app; directory geometry must account for the requested padded body and sidebar border.
<!-- /run -->

<!-- run -->
### Run 18

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/RepositoryReview.stories.svelte
```

**Notes**

After precise content-box assertions, the directory detects a duplicate outer frame inherited from generic Community styling. Removed the frame in this owned directory adaptation; inner table card remains.
<!-- /run -->
<!-- /evidence-field -->

