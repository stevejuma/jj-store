---
schema: 1
id: E-DOC-005
---

# E-DOC-005

<!-- evidence-field:id markdown -->
## Id

E-DOC-005
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

enclosing Jujutsu change soumsxws
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Documentation navigation defaults, rail spacing and app-style search
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64; MkDocs 1.6.1 / Material 9.6.23; Playwright 1.63.0 Chromium and WebKit; existing foundational source validation setup.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Presentation and generated navigation defaults only; repository, API, preference and asset manifest formats unchanged.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- docs/site/assets/layout.css
- docs/site/assets/theme.js
- docs/site/contributing/documentation.md
- docs/site/guides/browsing.md
- mkdocs.yml
- spec/src/api.md
- spec/src/documentation.md
- spec/src/ui.md
- web/tests/browser/help-theme.spec.ts
- web/tests/browser/help.spec.ts
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/browser/help-theme.spec.ts#standalone documentation retains themed
  search and responsive navigation
- web/tests/browser/help-theme.spec.ts#embedded Help follows live application
  appearance across pages and reopening
- web/tests/browser/help.spec.ts#bundled Help preserves a draft without a
  repository and works without external network
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
jj --no-pager st; inspect Community/Design Core/app navigation sources and live rail geometry; pnpm exec playwright test --config web/playwright.docs.config.ts --project chromium --grep "standalone documentation retains" (before)
```

**Notes**

Exact test expected Original Dark but received Buzz Dark. Live desktop rails reserved 88px/24px inner padding, leaving 184px nav widths in 288px/224px rails. Landing groups were expanded. User requested full width, collapsed groups and Original defaults, then compact rows, removal of duplicate title, visible hover feedback, inset/gapped TOC links and muted app-style search.
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
pnpm web:check; pnpm check:ui-ownership; node --test scripts/docs.test.mjs
```

**Notes**

Zero web diagnostics, valid UI ownership; 7/7 tooling tests.
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
pnpm spec:first (initial)
```

**Notes**

Updated browser tests also map to api.md. Added the presentation-default/no-host-preference-or-admission-change contract to that chapter.
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
pnpm exec prettier --write <changed files>; node scripts/docs.mjs stage; pnpm exec playwright test --config web/playwright.docs.config.ts --project chromium --grep "standalone documentation retains"
```

**Notes**

Original/Original Dark switch and collapsed/expand/collapse navigation checks pass; generated links/anchors valid. Stage repeated after user-requested styling refinements; final closure 138 assets, 13222489 bytes, 9 light/dark diagram pairs.
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
pnpm exec playwright test --config web/playwright.docs.config.ts (first full run)
```

**Notes**

11/12 passed. WebKit exposed an existing unscoped h1 assertion racing navigation while search result headings remained present. Scoped the assertion to the reading panel without changing the expected destination. Exact WebKit draft/search regression then passed.
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
pnpm exec playwright test --config web/playwright.docs.config.ts --project webkit --grep "bundled Help preserves a draft"; pnpm exec playwright test --config web/playwright.docs.config.ts (after selector correction)
```

**Notes**

Exact WebKit 1/1 and complete cold Help lane 12/12 pass. Covers Original defaults, collapse/expansion, embedded live theme inheritance, local search, mobile drawer, Axe, external-network denial, unavailable docs recovery and draft/page/focus preservation.
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
pnpm exec playwright test --config web/playwright.docs.config.ts --grep "standalone documentation retains|bundled Help preserves a draft" (after row/hover refinements and again after final search styling)
```

**Notes**

4/4 Chromium/WebKit cases on each fresh native asset-router run after regenerated assets. Final search uses the same foreground 3%/8% idle/hover mix, inset border and muted placeholder as the app-owned search trigger. Complete theme/navigation lane already passed; later CSS-only refinements repeat their exact reader/search consumers.
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
pnpm spec:first; pnpm spec:validate; changed-file pnpm exec prettier --check
```

**Notes**

7 protected files / 3 chapters before ledger update; 5 validators, 23 chapters and 171 requirements. Formatting passes.
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
Refresh standalone preview; inspect final widths, row/TOC/search styles and actual hover; restart only documentation-owned preview listeners; curl asset/header checks
```

**Notes**

Verified Original theme, full-width desktop rails, hidden duplicate title, collapsed groups, 31.6px primary rows, actual hover=true with muted background, 10px TOC link inset and 4px item gap. Search uses foreground 3% idle mix and muted placeholder. Saved ignored .local/dev-demo/help-original-rails.png. Python preview had inconsistent cached/304 assets; same loopback 7480 server now uses no-store and unconditional asset responses, restoring complete JS/search initialization. User standalone tab refreshed without changing its selected page. Built app preview on 7484 restarted with latest assets; preserved partial fixture, existing five-second shutdown warning, no unrelated service changes.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
Refresh/reconnect existing in-app browser tab 4 after restarting its server
```

**Notes**

Computer-use CDP focus command timed out on the existing app tab across refresh, inspection and reconnection. Server remains available with new assets; automatic refresh of this existing browser tab is unverified. Cold embedded Chromium/WebKit acceptance passes independently. Temporary QA tab closed and user standalone tab retained.
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
pnpm check:progress; pnpm spec:first; pnpm spec:validate; changed-file pnpm exec prettier --check; curl http://127.0.0.1:7484/help/; parseLedger historical comparison; jj --no-pager st; jj --no-pager diff
```

**Notes**

Final checks after ledger update: 137 tasks; 7 protected files and 4 canonical chapters; 5 validators, 23 chapters and 171 requirements. Changed-file formatting passes, app Help returns HTTP 200, and all 417 historical evidence entries retain their original content and raw JSON prefix. Reviewed the complete slice before committing.
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

Shared generated navigation/theme defaults justify complete cold Help coverage plus tooling tests; later cosmetic refinements use exact direct consumers. All 417 historical evidence entries and raw JSON prefix preserved. M19 remains Partial with registry-only and packaged WKWebView gates unchanged. No Rust, full repository, catalog, visual baseline, publication or unrelated service work.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Use refreshed standalone docs on 7480; existing app Help may need a manual reload because its browser-control session timed out. M19 registry-only and packaged native-host gates remain separate.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


