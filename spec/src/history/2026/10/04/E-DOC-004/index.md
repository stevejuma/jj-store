---
schema: 1
id: E-DOC-004
---

# E-DOC-004

<!-- evidence-field:id markdown -->
## Id

E-DOC-004
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

enclosing Jujutsu change wtyoqnzs
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Help search alignment and TOC divider refinement
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64; MkDocs 1.6.1 and Material 9.6.23; Playwright 1.63.0 Chromium/WebKit; existing source validation setup.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Presentation CSS only; no asset manifest, repository, API or preference format change.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- docs/site/assets/layout.css
- spec/src/documentation.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/browser/help-theme.spec.ts#standalone documentation retains themed
  search and responsive navigation
- web/tests/browser/help.spec.ts#bundled Help preserves a draft without a
  repository and works without external network
<!-- /evidence-field -->

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
jj --no-pager st; inspect Community and Design Core search/input sources; live Help geometry and computed-style inspection
```

**Notes**

Clean workspace. Before: TOC left border 1px; search x=1151.8 vs visible TOC x=1127 at 1351px viewport; search used muted background while reading panel used background. User confirmed alignment to the TOC rail.
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
pnpm exec prettier --check docs/site/assets/layout.css spec/src/documentation.md (initial)
```

**Notes**

Canonical prose wrapping needed formatting; repaired with the same files passed to prettier --write.
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
pnpm exec prettier --write docs/site/assets/layout.css spec/src/documentation.md; node scripts/docs.mjs stage
```

**Notes**

Fresh strict generated docs and stage: 138 admitted assets, 13232381 bytes; links and anchors valid, 9 diagrams with light/dark variants. Explicit source review checks current.
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
pnpm exec playwright test --config web/playwright.docs.config.ts --grep "standalone documentation retains|bundled Help preserves a draft"
```

**Notes**

4/4 exact Chromium/WebKit tests on a fresh native asset-router process: standalone search, theme switch, Axe and mobile drawer; embedded search, network independence, keyboard dismissal and draft preservation. Existing tests retained without new CSS-mirroring assertions.
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
pnpm web:check; pnpm check:ui-ownership
```

**Notes**

Zero web errors/warnings; UI ownership validator passes.
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
Verify preview process ownership; graceful restart only documentation preview; curl app/help; reload app and inspect current themed Help
```

**Notes**

App/help HTTP 200 at 7484. After: search and TOC both x=1127, search width 212px; search and reading panel both oklch(1 0 0); TOC left border 0px. Ignored screenshot .local/dev-demo/help-search-alignment.png. Existing five-second shutdown warning recurred; preview restarted with preserved partial fixture and no reset or unrelated process restart.
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
pnpm spec:first; pnpm spec:validate; pnpm check:progress; changed-file pnpm exec prettier --check
```

**Notes**

1 protected file and 2 canonical chapters; 5 validators, 23 chapters and 171 requirements; 137 tasks. Changed CSS/prose formatting passes; historical ledger entries remain unchanged.
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

Narrow application-owned documentation stylesheet refinement. Fresh generated assets justify cold exact search/reader tests; generator implementation, theme lifecycle and application components unchanged. Full Help, tooling, catalog, unrelated UI/Rust/root aggregate lanes and visual baselines are out of scope. Preserve all 416 prior evidence objects and raw JSON prefix. M19 remains Partial with E-DOC-001 release gates unchanged.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Review the refined Help search and TOC in the running documentation-workspace preview.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


