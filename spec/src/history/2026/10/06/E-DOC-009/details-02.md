---
schema: 1
id: E-DOC-009
---

# E-DOC-009 details 2

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
node --test --test-name-pattern='Vite refuses a missing initial Help bundle before listening' scripts/docs-assets.test.mjs
```

**Notes**

Before implementation: missing expected startup rejection; startup-before.log.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec playwright test --config .local/help-verification/playwright.config.ts --project chromium --grep 'Help shows a skeleton|Help page navigation keeps|failed Help shows'
```

**Notes**

Before implementation: missing skeleton/toast and reloaded frame; browser-before.log. Earlier runner attempts failed to start: duplicate inherited webServer config caused a 30-second startup timeout/occupied port; isolated config now spreads the base instead of concatenating servers.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: pass
failures: 0
passed: 21
hasNote: true
noteEncoding: markdown

```

```shell
node --test scripts/docs-assets.test.mjs scripts/dev-ui.test.mjs scripts/docs.test.mjs
```

**Notes**

assets.log. Missing/invalid initial bundles rejected; validated rebuild retention, cold publication and observer cleanup pass. Earlier direct assets/dev-ui run passed 12 and docs run passed 8; preview regression brings final total to 21.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: pass
failures: 0
passed: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec tsx --test web/tests/help-session.test.ts
```

**Notes**

Exact frame/origin admission and bounded typed errors; message.log.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:check
```

**Notes**

Intermediate check found the required notification action id missing; fixed before final checks.
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
pnpm web:check
```

**Notes**

0 errors and warnings; web-check.log.
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
pnpm catalog:check
```

**Notes**

0 errors/warnings; 251 owned modules; catalog-check.log.
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
pnpm docs:review application "Reviewed Help's themed responsive loading skeleton, sidebar/close interaction styling, bounded Retry toasts and retained portable navigation; user-facing manual prose remains accurate."
```

**Notes**

Application-only review; no unrelated source topic acknowledged. Review updated during implementation as presentation changed.
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
pnpm docs:build
```

**Notes**

Repeated sequentially after generated Help changes; final closure has 139 assets, 15,235,464 bytes, 9 light/dark Mermaid diagrams and valid links/anchors. docs-build-final.log.
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
pnpm docs:check
```

**Notes**

Review, generated-reference and closure checks pass; docs-check.log. Final themed closure additionally validated through web:build.
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
pnpm web:build
```

**Notes**

Production app and generated manual staged together. Chunk-size/performance advisories remain informational; web-build.log.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: pass
failures: 0
passed: 5
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec playwright test --config .local/help-verification/playwright.config.ts --project chromium --grep 'Help shows a skeleton|Help page navigation keeps|failed Help shows|Help page loads|superseded Help'
```

**Notes**

Source-focused regressions pass. Intermediate runs had a stale manual during rebuilding and a page.goBack wait for a top-level load event; same-document joint-history checks now observe actual content. browser-focused.log.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: pass
failures: 0
passed: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec playwright test --config .local/help-verification/playwright.config.ts --project chromium --grep 'Help shows a skeleton'
```

**Notes**

skeleton-layout.log: initial 288/224px rails, responsive hiding and loaded content boundary/background agreement.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec playwright test --config .local/help-verification/playwright.cold.config.ts
```

**Notes**

First cold run: 17 pass; retained search-result headings made two old broad h1 locators ambiguous, and WebKit exposed an iframe readiness race. browser-cold.log.
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
pnpm exec playwright test --config .local/help-verification/playwright.cold.config.ts
```

**Notes**

Next cold run: 18 pass; remaining generic h1 click was still ambiguous with retained search results. Scoped all article heading assertions/actions; browser-cold-final.log.
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: pass
failures: 0
passed: 20
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec playwright test --config .local/help-verification/playwright.cold.config.ts
```

**Notes**

Fresh built-bundle service with explicit existing target/debug/examples/browser_preview, no reused listener. Full direct Chromium/WebKit Help family passes: offline search/diagrams, palette/prominent selection, opaque sticky header, loading skeleton geometry, actual-response Retry toast, cancellation, history, focus, drafts, mobile and standalone. browser-cold-pass.log.
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
pnpm check:ui-ownership
```

**Notes**

Owned presentation boundary validated; ownership.log.
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
pnpm check:architecture
```

**Notes**

Unrelated existing catalog/components/store-stack-workspace/StoreStackWorkspaceConflicts.stories.svelte has 440 counted lines over 400. Verified byte-identical to parent. Every changed production/style/script file is within its architecture limit; no Help violation. architecture.log.
<!-- /run -->
<!-- /evidence-field -->

