---
schema: 1
id: E-SETTINGS-SPLIT-001
---

# E-SETTINGS-SPLIT-001 validation continued

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
pnpm dev:ui
```

**Notes**

Cold UI launch and final cold restart: Vite at 7460, existing backend at 7462, complete .local/dev-demo fixture preserved. No reset, migration, Rust rebuild or baseline update. /tmp/jj-settings-final-dev-ui.log.
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
JJ_STORE_REAL_BASE_URL=http://127.0.0.1:7460 JJ_STORE_REAL_REPOSITORY=c235f6f48a44e496f4f22a150a7a0356 JJ_STORE_SETTINGS_LOCAL_REPOSITORY=8c25cc47eb5e7545a3c143105bfc84ff pnpm exec playwright test --config web/playwright.real.config.ts web/tests/real/repository-settings.spec.ts
```

**Notes**

Final fresh-browser run: all five pass. Exact Stacks route/Details state, no ordinary reads on entry, profile filtering, deep links/reload/Back/Forward, global installation tabs, mobile folder draft retention and Axe, stable Sync refresh. Earlier cold run: three pass / one fail due startup readiness and the wrong rail label; corrected 30-second startup readiness and the actual Stack workspace views label, then exact Stacks pass. /tmp/jj-settings-final-real.log.
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
JJ_STORE_REAL_BASE_URL=http://127.0.0.1:7460 JJ_STORE_REAL_REPOSITORY=80d8731bbc255ca6373c797e8b9eb4b4 JJ_STORE_SETTINGS_LOCAL_REPOSITORY=8c25cc47eb5e7545a3c143105bfc84ff pnpm exec playwright test --config web/playwright.real.config.ts web/tests/real/repository-settings.spec.ts -g 'Sync observation settles'
```

**Notes**

Live user repository: nine GET /local/sync reads in 1.5 seconds instead of at most two. Completion recreated a derived destination object and restarted the observer. /tmp/jj-settings-flicker-before.log.
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
JJ_STORE_REAL_BASE_URL=http://127.0.0.1:7460 JJ_STORE_REAL_REPOSITORY=80d8731bbc255ca6373c797e8b9eb4b4 JJ_STORE_SETTINGS_LOCAL_REPOSITORY=8c25cc47eb5e7545a3c143105bfc84ff pnpm exec playwright test --config web/playwright.real.config.ts web/tests/real/repository-settings.spec.ts -g 'Sync observation settles'
```

**Notes**

Exact live regression passes after observing the stable category primitive. Status settles and manual Refresh adds exactly one read. Independent CUA inspection confirms actual Sync subtabs, no visible repeated category heading and no console errors. /tmp/jj-settings-flicker-after.log.
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
pnpm web:check; pnpm catalog:check
```

**Notes**

Final type checks: zero errors/warnings; catalog boundaries pass (394 modules). Intermediate catalog checks exposed nine fixture/story typing errors, then the native capability union and one extracted helper identifier; corrected without assertions or boundary allowlists. A type-only API client import was rejected by catalog ownership and removed in favor of explicit native fixture narrowing. /tmp/jj-settings-final-web-check.log and /tmp/jj-settings-final-catalog-check.log.
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
pnpm check:ui-ownership; pnpm check:boundaries
```

**Notes**

UI ownership passes; package boundaries pass (17 packages). New view, category model, button, guard and header/layout composition remain application-owned. /tmp/jj-settings-final-ownership.log and /tmp/jj-settings-final-boundaries.log.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 18
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:architecture
```

**Notes**

Inherited baseline had 19 source-budget failures. Final has 18; new violations removed, route writer and catalog/header/connection helpers extracted, changed inherited controller reduced from 320 to 316 counted lines. No baseline altered. /tmp/jj-settings-final-architecture.log.
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
pnpm evidence:new -- --id E-SETTINGS-SPLIT-001 --task M4.18 --task M4.20 --task M14.2 --task M16 --requirement JST-UI-012 --requirement JST-UI-014 --requirement JST-UI-016 --requirement JST-UI-020 --requirement JST-API-001
```

**Notes**

Created the new skeleton; whole-tree navigation regeneration was blocked by the pre-existing oversized E-PROJECT-ROWS-001/index.md. Restored incidental generator edits only. Completed this record and selected history/date navigation locally. Removed unrelated M16 from the final task selection; added directly affected M14.4/.5/.6.
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
pnpm list @lapismd/design-core @lapismd/mira @lapismd/spec-validator --depth 0; jj version; node --version; pnpm --version
```

**Notes**

Installed released packages: Design Core 0.3.1, Mira 0.0.3, Spec Validator 0.1.4. Package manifests/lockfile and development data are unchanged. JJ 0.45.1; Node 26.9.0; pnpm 10.32.1.
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
pnpm spec:links
```

**Notes**

Book links/anchors valid: 2558 generated HTML files and 96461 local references. This is link evidence, not a regenerated built-browser qualification. /tmp/jj-settings-final-links.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreRepositorySettings.stories.svelte catalog/components/store-application/StoreSearchSettings.stories.svelte catalog/components/store-application/StoreStorageSettings.stories.svelte catalog/components/store-application/StoreSyncSettings.stories.svelte catalog/components/store-application/StoreWorkspaceSettings.stories.svelte catalog/components/store-application/StoreServerSettings.stories.svelte
```

**Notes**

All six direct Settings consumer families: 35 pass / 0 fail after the exact LocalOnly regression pass. /tmp/jj-settings-final-settings-consumers-after.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/intentional-thread-workspaces.test.ts web/tests/vite/thread-workspace-publication.test.ts
```

**Notes**

Five pass for the unchanged extracted thread-publication callback consumer. /tmp/jj-settings-final-thread-consumers.log.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/repository-settings-ownership.test.ts
```

**Notes**

New integration regression: one pass / one fail. Advancing the code-read generation through browser history and repository Back discarded a pending Sync control response and left changing=true. /tmp/jj-settings-ownership-before.log.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/repository-settings-ownership.test.ts
```

**Notes**

Exact repaired boundary: two pass. Settings accepts replies from the still-connected owner despite code-selection changes; repository replacement still rejects late replies. /tmp/jj-settings-ownership-after.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/settings-freshness.test.ts web/tests/vite/production-search-settings.test.ts web/tests/vite/production-storage-settings.test.ts web/tests/vite/production-sync-settings.test.ts web/tests/vite/production-automatic-refresh.test.ts web/tests/vite/repository-settings-ownership.test.ts web/tests/vite/repository-settings-restoration.test.ts
```

**Notes**

28 pass after the owner-lifetime repair. /tmp/jj-settings-final-lifecycle.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/repository-settings-ownership.test.ts web/tests/vite/production-store-settings.test.ts web/tests/vite/index-freshness.test.ts web/tests/vite/managed-code-index.test.ts web/tests/vite/kind-preparation.test.ts
```

**Notes**

37 pass / one new index-ownership fixture failure: missing selected operation prevented preparation from starting. Corrected that fixture to carry the retained operation; no product assertion changed. /tmp/jj-settings-final-index-ownership.log.
<!-- /run -->
<!-- /evidence-field -->
