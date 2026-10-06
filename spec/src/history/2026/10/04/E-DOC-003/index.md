---
schema: 1
id: E-DOC-003
---

# E-DOC-003

<!-- evidence-field:id markdown -->
## Id

E-DOC-003
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M19
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-04
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change nsnyowlt
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Application-styled documentation and live Help appearance
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64; Svelte 5.57.1; MkDocs 1.6.1 and Material 9.6.23; Playwright 1.63.0 Chromium and WebKit; existing local Design Core validation hook.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No repository, API, browser or documentation manifest format change. Generated closure adds local app palette/font assets and dark Mermaid variants.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- docs/maintenance/coverage.json
- docs/site/assets/layout.css
- docs/site/assets/manual.css
- docs/site/assets/theme.js
- docs/site/contributing/documentation.md
- docs/site/guides/browsing.md
- mkdocs.yml
- scripts/docs/diagrams.mjs
- scripts/docs/hooks.py
- scripts/docs/site.py
- scripts/docs/theme.mjs
- scripts/docs.mjs
- scripts/docs.test.mjs
- spec/src/api.md
- spec/src/documentation.md
- spec/src/spec-governance.md
- spec/src/ui.md
- web/playwright.docs.config.ts
- web/src/lib/ui/store-help/StoreHelp.svelte
- web/tests/browser/help-theme.spec.ts
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/browser/help-theme.spec.ts#embedded Help follows live application
  appearance across pages and reopening
- web/tests/browser/help-theme.spec.ts#standalone documentation retains themed
  search and responsive navigation
- scripts/docs.test.mjs#generated closure requires both local diagram theme
  variants
<!-- /evidence-field -->

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
jj --no-pager st; inspect Community presentation, Design Core shell/button/font sources and application appearance; pnpm exec playwright test --config web/playwright.docs.config.ts --project chromium --grep "embedded Help follows" (before)
```

**Notes**

Clean working change. Exact new regression failed because the generated manual did not inherit Nord from the application.
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
pnpm exec playwright test --config web/playwright.docs.config.ts --project chromium --grep "embedded Help follows" (intermediate)
```

**Notes**

Two successive assertion failures: equivalent CSS custom-property hex/HSL serializations and ambiguous .md-typeset locator. Compare rendered background colors and scope typography to article; top-frame initialization preserves preference updates. Exact Chromium then passed.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec playwright test --config web/playwright.docs.config.ts (first theme suite)
```

**Notes**

10/12 passed. Both standalone dark Axe checks exposed Material slate link specificity overriding app tokens (4.23 contrast). Raised token mapping specificity; full contrast assertions retained.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec playwright test --config web/playwright.docs.config.ts --project chromium --grep "standalone documentation retains" (intermediate)
```

**Notes**

Mobile menu click exposed Material direction-selector specificity: the widened drawer still used its old left offset and intercepted the toolbar. Matched the direction selector and retained mobile title positioning. Another run used fill without Material keyboard search events; changed to actual typing as in existing Help tests. Exact Chromium then passed.
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
pnpm exec playwright test --config web/playwright.docs.config.ts (second theme suite)
```

**Notes**

11/12 passed. Chromium stalled during the second drawer boundingBox call, which remained pending until context cleanup; no failure screenshot completed. WebKit and the preceding exact Chromium run passed. Preserve geometry assertion and rerun unchanged after final heading CSS.
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
node --test scripts/docs.test.mjs
```

**Notes**

7/7 tooling regressions, including valid, absent, remote and empty dark-diagram variants; bounded closure/staging and source drift remain covered.
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
pnpm web:check; pnpm catalog:check; pnpm check:architecture; pnpm check:ui-ownership
```

**Notes**

Zero web/catalog diagnostics; 182 catalog modules; 932 architecture modules; ownership valid. Web/catalog rerun after final Svelte edits also passed.
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
pnpm web:build; node scripts/docs.mjs stage; pnpm docs:review application <appearance review>; pnpm docs:review
```

**Notes**

Fresh production web build and strict final docs generation; 138 files, 13232172 bytes, valid links and anchors; 9 diagrams with light/dark variants and local fonts. Existing foundational CSS :global and bundle-size warnings remain. User requested TOC heading refinement: transparent muted heading without Material white shadow at intermediate widths. All explicit source reviews current.
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
Changed-file pnpm exec prettier --write/--check; uv run --locked python -m py_compile scripts/docs/hooks.py scripts/docs/site.py; pnpm spec:first; pnpm spec:validate
```

**Notes**

Formatting and Python syntax pass; spec-first 16 protected files and 4 canonical chapters; 5 validators, 23 chapters and 171 requirements. Historical ledger prefix excluded from reformatting.
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
pnpm exec playwright test --config web/playwright.docs.config.ts --project chromium --grep "standalone documentation retains"; pnpm exec playwright test --config web/playwright.docs.config.ts (final)
```

**Notes**

Exact standalone Chromium passed unchanged after final CSS, then 12/12 complete cold Help tests passed in 1.2m. Fresh native asset-router process, actual generated assets, live Nord/Buzz/Original Dark inheritance and font sizes, dark/light SVG loads, standalone palette/search/mobile drawer, external-network denial, CSP, Axe, draft/page/focus retention and unavailable-doc recovery. Earlier renderer stall did not recur; no timeout or assertion weakening.
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
Graceful restart of only the documentation config process; curl app/help; browser reload, inherited-theme and computed TOC heading inspection
```

**Notes**

Loopback 7484 app/Help HTTP 200. Existing five-second shutdown warning recurred; read-only preview restarted with preserved partial fixture. App and Help both Buzz; TOC heading has transparent background and no shadow at the user viewport. Full app reload refreshed cached stylesheet. Saved ignored .local/dev-demo/help-app-theme.png; no visual baseline changes. Other deployments untouched.
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
pnpm check:progress; parseLedger historical comparison
```

**Notes**

137 tasks; all 415 historical evidence objects and their raw JSON prefix preserved. M19 remains Partial. No storage/identity release qualification claimed.
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

Generated manual presentation and read-only appearance inheritance only. Existing app palettes and released fonts are bundled; host attributes are observed without parent writes, preference storage access or a new messaging/API protocol. Complete Help browser lane justified by shared generated assets and embedded/standalone consumers. All 415 previous evidence entries and raw JSON prefix preserved. M19 remains Partial; foundational registry-only and packaged WKWebView release gates remain in E-DOC-001. Unrelated Rust, root aggregate, full browser/Storybook/storage lanes and visual baselines are out of scope.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Review themed Help in the documentation workspace preview on 7484. Foundational registry-only and packaged WKWebView release qualification remain separate M19 gates.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


