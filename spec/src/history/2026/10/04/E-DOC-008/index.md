---
schema: 1
id: E-DOC-008
---

# E-DOC-008

<!-- evidence-field:id markdown -->
## Id

E-DOC-008
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

enclosing Jujutsu change otplssvv
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Help development preflight and manifest-driven asset reload
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Node26.9.0; Vite8.3.1; Playwright1.63.0; pnpm10.32.1; macOS ARM64.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Existing bounded Help manifest version1 and same-origin static delivery unchanged. No API, repository or storage format change.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- scripts/dev-ui.mjs
- scripts/dev-ui.test.mjs
- scripts/docs/assets.mjs
- scripts/docs-assets.test.mjs
- spec/src/documentation.md
- spec/src/spec-governance.md
- spec/src/cli.md
- docs/site/contributing/development.md
- docs/site/contributing/documentation.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- scripts/dev-ui.test.mjs#missing or invalid Help leaves running services and
  runtime markers intact
- scripts/docs-assets.test.mjs#Help recovers from a missing initial bundle when
  its manifest is published
- scripts/docs-assets.test.mjs#Help retains validated bytes until a safe
  complete replacement is published
- scripts/docs-assets.test.mjs#Help releases its manifest observers when the
  server closes
- scripts/docs-assets.test.mjs#a cold Vite server reloads rebuilt Help through
  real filesystem publication events
- scripts/docs-assets.test.mjs#superseded Help publications settle on the latest
  validated bundle
<!-- /evidence-field -->

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
node --test scripts/dev-ui.test.mjs scripts/docs.test.mjs (before changes)
```

**Notes**

14existing cases pass; before-existing.log.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
node --test --test-name-pattern='missing or invalid Help|Help recovers|Help retains|Help releases|cold Vite' scripts/dev-ui.test.mjs scripts/docs-assets.test.mjs (before implementation)
```

**Notes**

5new regressions reproduce preflight omission, sticky missing map, no rebuilt bundle adoption and missing observer lifecycle; before-regressions.log.
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
node --test --test-name-pattern='missing or invalid Help|Help recovers|Help retains|Help releases|cold Vite' scripts/dev-ui.test.mjs scripts/docs-assets.test.mjs (after implementation)
```

**Notes**

5exact regressions pass. Initial broad directory watch produced Vite HTML reload notices; narrowed observation to the manifest and added no-full-reload/cold directory replacement assertions. after-regressions.log.
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
node --test scripts/dev-ui.test.mjs scripts/docs.test.mjs scripts/docs-assets.test.mjs
```

**Notes**

20direct cases pass after manifest-only observation; direct-tests.log. Cold existing-directory replacement and no full-reload assertions pass.
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
node --test --test-name-pattern='cold Vite' scripts/docs-assets.test.mjs (missing parent directory extension)
```

**Notes**

Cold real-filesystem probe found that Vite could miss a manifest created below a missing build directory. cold-watcher.log. Replaced shared Vite observation with a recursive filesystem observer on the tracked documentation parent, filtered to manifest/directory hints; serial validation closes the startup subscribe/read gap.
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
node --test scripts/dev-ui.test.mjs scripts/docs-assets.test.mjs scripts/docs.test.mjs (final observer)
```

**Notes**

20pass. Real missing-bundle creation, full build-directory replacement, safe adoption, last validated bytes, overlapping publications and observer shutdown pass. No application full-reload broadcasts. direct-tests-final.log, direct-tests-final2.log.
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
pnpm docs:check
```

**Notes**

9Mermaid diagrams;139files/13,698,340bytes; generated references and links/anchors valid. docs-check.log.
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
pnpm web:check; pnpm spec:validate; pnpm check:boundaries; pnpm check:architecture; pnpm spec:first; pnpm check:progress
```

**Notes**

0errors/warnings;5validators/23chapters/172requirements;17packages;987modules;6protected files/4canonical chapters;139tasks. web-check-final.log, spec-validate.log, boundaries.log, architecture.log, spec-first.log, progress.log.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec playwright test --config .local/help-assets-fix/playwright.config.ts (initial cold harness)
```

**Notes**

Harness timed out before tests: defineConfig(base, override) concatenated its inherited Rust preview server with the intended Vite server. Replaced server configuration in a single object; no assertions or product behavior changed. cold-help-browser.log.
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
pnpm exec playwright test --config .local/help-assets-fix/playwright.config.ts (corrected fresh Vite harness)
```

**Notes**

12pass/1.8m across Chromium/WebKit:6existing Help cases each. Draft/focus preservation, local offline search/diagrams, themes, deep links/mobile navigation, missing-bundle retry and installation Help all pass. API bootstrap is mocked; isolated Vite has no backend, so local catalog connection refusal in sidebar fixtures is expected. No native/production HTTP qualification inferred. cold-help-browser-final.log.
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
node main prepareUiDevelopment preflight; fetch updated Help; compare runtime process markers
```

**Notes**

Current main validates its bundle and serves the updated manual withHTTP200. VitePID53716 and backendPID36259 retained; no backend replacement. live-preflight.log.
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
pnpm exec prettier --write changed scripts, manual pages and canonical chapters/ledger
```

**Notes**

Changed files formatted; format.log, format-observer.log, format-final.log. Scope preserves all471prior evidence objects and139task states.
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

<!-- evidence-field:next markdown -->
## Next

Retain Help development startup/publication/lifecycle regressions. M19 remains Partial with separate registry and packaged WKWebView release gates. Production Rust asset delivery, root aggregates, unrelated repository/browser suites, additional-platform filesystem qualification and visual baselines are outside this development-only slice. No new public contract or storage format. Current manual is rebuilt and adopted by the running Vite instance.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


