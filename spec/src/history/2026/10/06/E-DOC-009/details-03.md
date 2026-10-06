---
schema: 1
id: E-DOC-009
---

# E-DOC-009 details 3

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
pnpm exec prettier --check docs/site/assets/embed.js docs/site/assets/layout.css docs/site/assets/theme.js scripts/docs/assets.mjs scripts/docs-assets.test.mjs docs/maintenance/coverage.json web/src/lib/ui/store-help/StoreHelp.svelte web/src/lib/ui/store-help/StoreHelpSkeleton.svelte web/src/lib/ui/store-help/help-session.ts web/tests/help-session.test.ts web/tests/browser/help.spec.ts web/tests/browser/help-theme.spec.ts
```

**Notes**

format.log; changed spec/evidence Markdown separately formatted.
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

143 tasks and 619 evidence records; progress.log.
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

12 protected files and 5 canonical chapters; spec-first.log.
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
pnpm spec:validate
```

**Notes**

7 validators, 3019 chapters and 175 requirements; spec-validate.log.
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
node scripts/docs.mjs stage
```

**Notes**

Final manual staged into the already-built browser app; docs-build-final.log.
<!-- /run -->

<!-- run -->
### Run 6

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

Final current manual cold run also covers selecting the current Help page without a document request. Full Chromium/WebKit family passes again, 20/20; browser-cold-current.log.
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

Rebuilt the final responsive skeleton after live screenshot review exposed reversed medium-width rail breakpoints; application review updated and closure remains 139 assets / 15,235,464 bytes. web-build-responsive.log.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: pass
failures: 0
passed: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec playwright test --config .local/help-verification/playwright.cold.config.ts --grep 'Help shows a skeleton'
```

**Notes**

Final Chromium/WebKit cold loading regression covers 1400px desktop, 1070px tablet (navigation hidden, contents visible) and 390px mobile (both rails hidden), with palette/loaded boundary agreement. browser-responsive.log. Unchanged navigation/theme family retains its previous 20/20 cold result.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pass
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

[]
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Qualified narrow Help UI/documentation acceptance. Initial loading mirrors manual navigation/content/contents geometry and palette; subsequent content reads keep real chrome. Native error responses/readiness failures use session app Retry toasts without inline duplicates, preserving dismissal during retry. Portable same-origin document replacement cancels obsolete fetches, normalizes relative assets, uses Material lifecycle and retains fonts/header/search/theme with browser history. Strict initial Vite/preview validation replaces permissive startup; complete running rebuilds still update atomically. Sidebar/Close interactions use application foreground tints/prominent Buzz settings; separators are tinted and sticky header is opaque. Read-only live verification on 7460 confirmed loaded Help, sticky opaque background, selected tint and themed separator; screenshot .local/help-verification/live-help.png. No sample graph/fixture mutations. Optional global architecture failure is unchanged and outside this Help acceptance; Rust/root aggregates, Storybook unrelated to Help, screenshot baselines, registry publication and desktop packaged WKWebView qualification not run.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

M19 remains Partial; complete registry and packaged WKWebView release qualification separately. Existing oversized Stacks conflict catalog story needs its own architecture repair.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: Update UI/documentation/API/architecture/governance topics for themed
  loading, bounded app Retry errors, document-retaining navigation and strict
  direct Vite startup. Asset manifest/CSP and repository authority unchanged.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: Add focused acceptance evidence to M19; registry and packaged desktop
  qualification are still open.

```
<!-- /evidence-field -->

