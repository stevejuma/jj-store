---
schema: 1
id: E-FRESH-004
---

# E-FRESH-004 details 2

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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/application-freshness.test.ts web/tests/vite/index-freshness.test.ts web/tests/vite/projects-freshness.test.ts web/tests/vite/automatic-refresh.test.ts web/tests/vite/automatic-store-refresh.test.ts web/tests/vite/automatic-work-refresh.test.ts web/tests/vite/production-automatic-refresh.test.ts web/tests/vite/production-store-settings.test.ts web/tests/vite/production-search-settings.test.ts web/tests/vite/production-storage-settings.test.ts web/tests/vite/production-sync-settings.test.ts web/tests/vite/projects-hub.test.ts web/tests/vite/projects-agents.test.ts web/tests/vite/projects-directories.test.ts web/tests/vite/projects-repositories.test.ts web/tests/vite/projects-repository-visibility.test.ts web/tests/vite/capture-settings.test.ts web/tests/vite/capture-management.test.ts web/tests/vite/repository-thread-navigation.test.ts web/tests/vite/local-browser.test.ts
```

**Notes**

132 tests/20 direct consumer files passed after integration.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/settings-freshness.test.ts web/tests/vite/projects-freshness.test.ts web/tests/vite/index-freshness.test.ts web/tests/vite/application-freshness.test.ts web/tests/vite/capture-management.test.ts web/tests/vite/capture-settings.test.ts
```

**Notes**

30 focused final event/Settings/capture/task regressions passed.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/projects-freshness.test.ts web/tests/vite/projects-hub.test.ts
```

**Notes**

13 cases passed after helper extraction. First integrated run exposed 3 failures: Projects async ordering fixed, Agents fixture links corrected, expiry uses typed API error.
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

0 errors/warnings before final helper extraction; final repeat pending.
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
pnpm catalog:check
```

**Notes**

0 errors/warnings, 190 catalog modules; no visual baseline qualification.
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
pnpm check:architecture
```

**Notes**

986 modules; intermediate 305-line controller fixed by helper extraction.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config web/vitest.config.ts web/tests/vite/projects-hub.test.ts web/tests/vite/projects-agents.test.ts web/tests/vite/projects-repositories.test.ts web/tests/vite/projects-repository-visibility.test.ts web/tests/vite/application-freshness.test.ts web/tests/vite/production-automatic-refresh.test.ts
```

**Notes**

Incorrect nonexistent configuration path; rerun with owning catalog config passed.
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
pnpm web:build
```

**Notes**

First stage required refreshed services documentation fingerprint. Concurrent later docs staging/checks raced over generated docs/build; serialize final stage.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/application-freshness.test.ts web/tests/vite/index-freshness.test.ts web/tests/vite/projects-freshness.test.ts web/tests/vite/settings-freshness.test.ts web/tests/vite/production-automatic-refresh.test.ts web/tests/vite/production-store-settings.test.ts web/tests/vite/capture-settings.test.ts web/tests/vite/capture-management.test.ts
```

**Notes**

47 final direct freshness cases passed.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/terminal.test.ts
```

**Notes**

5 terminal stream/ACK regressions pass; terminal ownership remains independent.
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
pnpm web:check
```

**Notes**

0 errors and warnings after stable derived subscription boundaries.
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
pnpm check:architecture
```

**Notes**

986 modules pass after final extraction and reactivity fix.
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
pnpm check:progress
```

**Notes**

Rebased M20 row initially sat outside its table and formatter wrapped it. Moved into Tasks and gates; repaired new evidence schema and named references.
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
pnpm check:progress
```

**Notes**

139 tasks; evidence bookkeeping only.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Cold built-bundle native/real-RustFS and filesystem idle, five-minute renewal,
  external operation, replay/offline and task qualification pending.
  Documentation staging rerun and final integrated checks pending. Root
  aggregate and visual baselines out of scope.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Finish serial docs stage, start isolated cold fixture, qualify traffic and recovery, then commit verified slices.
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M20
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-04
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

zrmqwtuqxqun
<!-- /evidence-field -->

