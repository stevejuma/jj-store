---
schema: 1
id: E-UI-VIEW-LOADING-001
---

# E-UI-VIEW-LOADING-001 details 3

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
pnpm test:storybook (parent source comparison)
```

**Notes**

Both original and temporary outside-root middleware attempts failed to import the absolute Storybook setup file from the external /tmp root. Three suites loaded zero tests; this does not prove parent browser/story behavior. /tmp/view-loading-host-parent-proof.log and /tmp/view-loading-parent-stories-middleware.log.
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
pnpm test:storybook catalog/components/store-stack-workspace/StoreStackWorkspaceConflicts.stories.svelte
```

**Notes**

After restoring conflict overlays inside CommunityAppShell: 6 pass, 1 fails because details still show Loading Workspace details. Updated the owning rail helper to await Workspace actions; did not relax any assertions. /tmp/view-loading-context-fixed-stories.log.
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
pnpm test:storybook catalog/components/store-stack-workspace/StoreStackWorkspaceConflicts.stories.svelte
```

**Notes**

Final 7/7 pass, including deferred conflict loading, reopen/navigation and editor panes. /tmp/view-loading-context-rail-stories.log. This cold test process covers restored shell context and the affected helper consumers.
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
node --test scripts/view-module-urls.test.mjs
```

**Notes**

Before HMR-ID normalization: 3 pass, 1 failed on ?t query. Live HTTP also showed four multiline URL calls left unprepared by the older running plugin regex. /tmp/view-module-url-before.log and /tmp/view-module-live-before.js.
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
node --test scripts/view-module-urls.test.mjs
```

**Notes**

5/5: multiline trailing commas, HMR query IDs, every one of 24 view calls, separate catalog serving root, emitted build references and invalid targets. /tmp/view-module-url-final.log.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:build (intermediate lazy builds)
```

**Notes**

Intermediate builds exposed strict-entry/recursive splitting incompatibility, 499-file asset count and 18808-byte inventory exceeding native limits, unresolved file-reference hash placeholders in the manifest, and missing unsplit CSS. Compiler resolveFileUrl, safe chunk grouping, one stylesheet and stylesheet-root admission repaired these. /tmp/view-loading-native-diagnostic.log and /tmp/view-loading-retry-build-urls.log. No native limits changed.
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

Final pre-overlay build passes Vite, manifest and documentation stage. /tmp/view-loading-final-build.log. Catalog-independent dynamic page entries verified. Existing chunk-size warnings remain.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:build (after shell-context repair)
```

**Notes**

Vite compilation and manifest pass; final strict documentation stage rejects stale application review. /tmp/view-loading-overlay-final-build.log. Repaired review and reran exactly the failed stage below.
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
pnpm docs:review -- --id application --reason <review text>
```

**Notes**

Wrong CLI argument shape; topic-specific explanation rejected. Retried supported positional command; no checker bypass. /tmp/view-loading-doc-review-latest.log.
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
pnpm docs:review application "Prepared archive consumption, shared view imports, shell-owned conflict overlays and bounded dynamic asset inventory are documented in the install and browsing guides; controller selections, authentication and drafts retain their owners."
```

**Notes**

Reviewed current install/browsing guidance; updated only application reason/fingerprint and retained parent compact formatting for other topics. /tmp/view-loading-doc-review-repaired.log.
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
node scripts/docs.mjs stage
```

**Notes**

Exact failed stage now passes: 9 Mermaid diagrams offline, documentation built and staged. /tmp/view-loading-doc-stage-repaired.log.
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
pnpm build-storybook
```

**Notes**

Final static catalog build passes. /tmp/view-loading-catalog-final-build.log. Build output is independent of the live Vite/backend.
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
node --test scripts/view-module-urls.test.mjs scripts/local-package-pnpmfile.test.mjs scripts/web-manifest.test.mjs scripts/view-loading-assets.test.mjs
```

**Notes**

11/11: five URL tests, two hook tests, three manifest tests and one complete built-asset closure test. /tmp/view-loading-focused-final-all.log. Native final inventory: 242 files, 10732-byte inventory, 10911544 total bytes; largest 2326651 bytes.
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
pnpm exec playwright test --config .local/view-loading-proof/playwright.config.ts web/tests/browser/lazy-views.spec.ts (initial retry flows)
```

**Notes**

First retry of the same failed URL remained cached in Chrome. Added compiler-owned emitted entry URLs and explicit failure-count query to avoid negative cache; static original factory remains first attempt. This does not guess hashed filenames.
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
pnpm exec playwright test --config .local/view-loading-proof/playwright.config.ts web/tests/browser/lazy-views.spec.ts
```

**Notes**

Final 3/3 cold native-static flows: 503 explicit retry without reload/POST, unopened page chunks absent and loaded module reused, obsolete delayed view cannot render after navigation. /tmp/view-loading-overlay-cold-browser.log. Existing native browser_preview executable was reused; no Rust build.
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec playwright test --config .local/view-loading-proof/playwright.config.ts web/tests/browser/lazy-views.spec.ts (measurement addition)
```

**Notes**

A new source-URL assertion incorrectly classified a compiled .svelte.js chunk as a raw component. Corrected it to source pathname suffixes .svelte / .svelte.ts, retaining the intended delivery assertion. Two pass, one fail before correction. /tmp/view-loading-final-cold-measured.log.
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
pnpm exec playwright test --config .local/view-loading-proof/playwright.config.ts web/tests/browser/lazy-views.spec.ts
```

**Notes**

Final measured run 3/3 in 4.5s. Built Files fixture: 95 requests, rendered 514ms, DCL327.9ms, load391.1ms, FCP340ms; zero raw .svelte/.svelte.ts requests. This includes fixture APIs, not live service timing or a controlled before/after comparison. /tmp/view-loading-final-cold-measured-retry.log.
<!-- /run -->

<!-- run -->
### Run 18

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec playwright test --config .local/view-loading-proof/playwright.config.ts web/tests/browser/appearance.spec.ts web/tests/browser/file-editor-navigation.spec.ts web/tests/browser/navigation.spec.ts web/tests/browser/repository-navigation.spec.ts web/tests/browser/thread-performance.spec.ts
```

**Notes**

6 pass, 3 fail: mobile Appearance heading lookup expects old nesting; repository overview expects prior &gt;=12px insets while existing CSS says0; Thread reply reaction selector no longer matches default Document. /tmp/view-loading-consumer-browser.log. Broader reconciliation remains pending; not all were proved parent failures.
<!-- /run -->
<!-- /evidence-field -->

