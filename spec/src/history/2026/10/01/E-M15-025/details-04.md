---
schema: 1
id: E-M15-025
---

# E-M15-025 details 4

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
pnpm exec playwright test --config /tmp/jj-thread-fix-playwright.config.mjs threads-alignment.spec.ts (rerun 6)
```

**Notes**

All Thread/forum/metadata/reply/popover/sidebar geometry and width restoration checks pass, including Thread axe. New Projects navigation axe exposes a missing h1. /tmp/jj-thread-fix-browser6.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application catalog/components/store-work catalog/components/store-threads (first full direct shell consumers)
```

**Notes**

121 passing interactions across 32 files; browser RPC connection closed before 2 files ran, at StoreSyncSettings. Do not claim a suite pass. Cold full rerun required. /tmp/jj-thread-fix-shell-consumer-stories.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/ProjectsDirectory.stories.svelte catalog/components/store-application/StoreSyncSettings.stories.svelte (heading before)
```

**Notes**

6 / 7 pass; explicit Projects h1 regression fails, isolated Sync settings interactions pass with no RPC errors. /tmp/jj-thread-fix-project-heading-before.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/ProjectsDirectory.stories.svelte catalog/components/store-application/StoreSyncSettings.stories.svelte
```

**Notes**

7 / 7 pass after using h1 with unchanged copied header typography. /tmp/jj-thread-fix-project-heading-after.log.
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
pnpm web:build (build 9)
```

**Notes**

Fresh production assets include corrected Projects page heading; existing chunk-size advisory only. /tmp/jj-thread-fix-build9.log.
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
pnpm web:check; pnpm catalog:check (heading iteration)
```

**Notes**

Web 0 errors/warnings; catalog finds invalid testing-library exact option in new assertion. Its string name is exact by default; remove unsupported option. /tmp/jj-thread-fix-web-check9.log, catalog-check7.log.
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
pnpm exec prettier --write (heading/spec/ledger iterations 17-18); pnpm exec prettier --check (JJ changed paths)
```

**Notes**

Scoped formatting passes. /tmp/jj-thread-fix-format17.log, format18.log, ledger-format.log, format-check.log. Original historical ledger objects preserved and rechecked.
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
pnpm check:progress (initial)
```

**Notes**

Evidence result partial is outside the existing result enum; use pending until final completion. /tmp/jj-thread-fix-progress.log.
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
pnpm check:progress (corrected)
```

**Notes**

119 task records pass; /tmp/jj-thread-fix-progress2.log.
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
pnpm spec:validate; pnpm spec:first; pnpm spec:first (after heading)
```

**Notes**

5 validators / 145 requirements; protected source scope 34 then 37 files across 4 canonical chapters. /tmp/jj-thread-fix-spec-validate.log, spec-first.log, spec-first2.log.
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
pnpm check:architecture; pnpm check:ui-ownership (final production code)
```

**Notes**

627 modules within unchanged budgets, complete owned source provenance closure. /tmp/jj-thread-fix-architecture9.log, ownership4.log.
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
pnpm catalog:check (final heading regression)
```

**Notes**

0 errors and warnings; 121 catalog modules pass. /tmp/jj-thread-fix-catalog-check8.log.
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
pnpm exec playwright test --config /tmp/jj-thread-fix-playwright.config.mjs threads-alignment.spec.ts (final cold run)
```

**Notes**

1 / 1 passes in 13.7s with a fresh isolated preview on 7458. Simultaneous standard pane and outer rail, borders, independent toggles and width restoration, forum composer, metadata, replies, hover clearance, reaction actors, scoped search and native links pass. Threads and Projects axe pass. /tmp/jj-thread-fix-browser7.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application catalog/components/store-work catalog/components/store-threads (second combined run)
```

**Notes**

96 checks in 24 / 34 files pass before browser RPC closes at ProjectsActivity; 10 files unrun, no assertion failure among completed tests. Not a suite pass. Run the complete same families in separate fresh processes, without harness or assertion changes. /tmp/jj-thread-fix-shell-consumer-stories2.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application
```

**Notes**

93 / 93 interaction and accessibility checks across all 23 direct application consumer files pass with a fresh process, no unhandled errors. /tmp/jj-thread-fix-app-stories-final.log.
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work
```

**Notes**

26 / 26 interaction and accessibility checks across all 8 WorkStore direct consumer files pass with a fresh process, no unhandled errors. /tmp/jj-thread-fix-work-stories-final.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-threads (final full family)
```

**Notes**

8 / 8 interaction and accessibility checks across all 3 Thread direct consumer files pass in a fresh process, no unhandled errors. Complete separately qualified application/work/threads families: 127 checks across 34 files. /tmp/jj-thread-fix-threads-stories-final.log.
<!-- /run -->

<!-- run -->
### Run 18

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --write spec/src/ui.md spec/src/milestones.md; pnpm exec prettier --check (all 41 changed files); pnpm check:progress; pnpm spec:validate; pnpm spec:first (final)
```

**Notes**

Formatting passes, 119 task records, 5 validators / 21 chapters / 145 requirements, and 37 protected files across 4 canonical chapters pass. All 246 historical evidence records remain semantically unchanged. /tmp/jj-thread-fix-format19.log, format-check-final.log, progress3.log, spec-validate2.log, spec-first3.log. Final ledger recording repeats these document consumers before commit.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pass
<!-- /evidence-field -->

