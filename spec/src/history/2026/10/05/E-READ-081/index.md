---
schema: 1
id: E-READ-081
---

# E-READ-081

<!-- evidence-field:id markdown -->
## Id

E-READ-081
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M13.4
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-05
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

wswkyyxy
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Ordinary graph navigation without unused change inventory
<!-- /evidence-field -->

<!-- evidence-field:versions list -->
## Versions

- JJ/jj-lib 0.45.1
- Svelte 5.57.1
- Vitest 4.1.11
- macOS ARM64
<!-- /evidence-field -->

<!-- evidence-field:formats list -->
## Formats

- No API, index or authority format change
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

UI-only correction found during original Linux acceptance. Commits joins automatic exact-operation core recovery; Commits/Branches commit and diff selection avoids an unused change-reference lookup. Files reference selection remains unchanged.
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-005
- JST-GOV-002
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- web/src/lib/controller/production-store-index.ts
- web/src/lib/controller/production-reference-reader.ts
- web/tests/vite/managed-code-index.test.ts
- web/tests/vite/production-reference-reader.test.ts
- spec/src/ui.md
- spec/src/api.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/vite/managed-code-index.test.ts#managed history recovery preserves
  the exact selection and does not prepare advanced inventories
- web/tests/vite/production-reference-reader.test.ts#selected commit and diff do
  not resolve the unused change picker
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/managed-code-index.test.ts web/tests/vite/production-reference-reader.test.ts
```

**Notes**

.local/bounded-reads/managed/ui-navigation-before.log: 13 pass, 3 failures reproduce missing log recovery and redundant exact change lookup for log/branches.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/managed-code-index.test.ts web/tests/vite/production-reference-reader.test.ts web/tests/vite/production-store-settings.test.ts web/tests/vite/production-store-data-source.test.ts web/tests/vite/production-automatic-refresh.test.ts web/tests/vite/production-route-restoration.test.ts
```

**Notes**

.local/bounded-reads/managed/ui-navigation-after.log: 44 passed in 5 real files; the route-restoration path supplied does not exist and contributes no test.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/managed-code-index.test.ts web/tests/vite/production-reference-reader.test.ts web/tests/vite/production-store-settings.test.ts web/tests/vite/index-freshness.test.ts web/tests/vite/local-browser.test.ts web/tests/vite/production-store-data-source.test.ts web/tests/vite/production-automatic-refresh.test.ts web/tests/vite/kind-preparation.test.ts web/tests/vite/kind-preparation-http.test.ts
```

**Notes**

.local/bounded-reads/managed/ui-navigation-consumers-final.log: 85 passed in 9 directly affected controller/transport files.
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

.local/bounded-reads/managed/ui-navigation-type.log: zero diagnostics.
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
pnpm spec:validate
```

**Notes**

.local/bounded-reads/managed/ui-navigation-canonical-final.log
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
pnpm check:progress
```

**Notes**

.local/bounded-reads/managed/ui-navigation-progress-final.log
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
pnpm spec:first && pnpm check:architecture && pnpm check:ui-ownership
```

**Notes**

.local/bounded-reads/managed/ui-navigation-governance-final.log
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
pnpm exec prettier --check web/src/lib/controller/production-store-index.ts web/src/lib/controller/production-reference-reader.ts web/tests/vite/managed-code-index.test.ts web/tests/vite/production-reference-reader.test.ts spec/src/api.md spec/src/ui.md spec/src/milestones.md
```

**Notes**

.local/bounded-reads/managed/ui-navigation-format-check-final.log
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
pnpm spec:first
```

**Notes**

.local/bounded-reads/managed/ui-navigation-governance.log: missing mapped api chapter; contract explanation added.
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
pnpm check:progress
```

**Notes**

.local/bounded-reads/managed/ui-navigation-progress.log: named-test checker requires test() declarations; retained original name and grouped route cases in named tests.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:notes list -->
## Notes

- Actual browser network inspection identified HTTP 409 index_rebuild_required
  from References/Changes exact lookup at selected change
  64f40fdf0665e203f66cea2aee9f6e67. Route restoration never consumes that choice
  on Commits/Branches; removing it allows the exact commit and diff reads to
  proceed. No backend optimization or advanced inventory preparation is added.
- Before/after tests preserve exact operation, commit and change selections.
  Existing explicit advanced preparation, capability gating, stale navigation
  and bookmark selection regressions remain.
- "Focused UI impact only: no Rust rebuild, full contract recapture, unrelated
  suite or visual baseline update. Live cold Linux acceptance and backend
  restart reuse follow after landing."
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Original 7470 Linux browser/restart and source-byte acceptance still pending.
  Advanced search/evolution/copy/deep-query, authenticated automatic maintenance
  and complete scale/fault/RustFS/release/visual qualification remain deferred.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Land this UI correction and record final original Linux acceptance separately.
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


