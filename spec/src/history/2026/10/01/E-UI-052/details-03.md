---
schema: 1
id: E-UI-052
---

# E-UI-052 details 3

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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreRepositoryApplication.stories.svelte -t LargeVirtualizedRepository
```

**Notes**

Read-only untouched-main baseline: 1 failed/5 skipped with the same aria-controls failure and inherited Bits dismissible-layer derived_inert teardown error; /tmp/jj-appearance-baseline-large.log.
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
pnpm web:build
```

**Notes**

Final production rebuild after related corrections; manifest/local assets generated. Existing large-chunk warning retained; /tmp/jj-appearance-qualified-build.log.
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
pnpm exec playwright test -c .scratch/appearance.playwright.config.ts
```

**Notes**

Final cold rebuilt UI: 1/1; reload, cross-tab, OS mode, desktop/mobile axe and reduced motion, geometry/zoom, no CSP violations or external requests in the Threads/Appearance flow; /tmp/jj-appearance-qualified-cold.log.
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
pnpm exec tsx scripts/appearance-palettes.mts --check
```

**Notes**

Final generated-byte check for all 62 palettes; /tmp/jj-appearance-palettes-verified.log.
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
pnpm check:architecture
```

**Notes**

Final owned-source graph and source budgets pass for 707 modules; /tmp/jj-appearance-architecture-last.log.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook
```

**Notes**

Final broad UI run: 4 failed/34 passed files of 51; 4 failed/151 passed tests and 6 errors before a browser connection closed. Failures are inherited commit/comparison/repository picker aria-controls violations and MobileHeaderFilters retaining a search draft. All four reproduce read-only on untouched main. Related Appearance regressions pass focused runs; Bits dismissible/scrollbar teardown errors remain explicit. /tmp/jj-appearance-all-stories-final.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/ProjectsFilters.stories.svelte -t MobileHeaderFilters
```

**Notes**

Read-only untouched-main baseline: 1 failed/1 skipped; reopened mobile search has empty value instead of draft, matching the final broad lane. /tmp/jj-appearance-baseline-mobile-filters.log.
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
pnpm web:check
```

**Notes**

Final production typecheck after all corrections: zero errors/warnings; /tmp/jj-appearance-web-check-complete.log.
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
pnpm catalog:check
```

**Notes**

Final catalog typecheck and boundaries: zero errors/warnings, 125 modules; /tmp/jj-appearance-catalog-check-last.log.
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
pnpm check:ui-ownership
```

**Notes**

Final ownership validator passes with 21 chapters/146 requirements; no Community package, source aliases or checkout dependencies. /tmp/jj-appearance-ownership-last.log.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --check .
```

**Notes**

Repository-wide check found the Appearance ledger formatting, corrected, and inherited crates/jj-agent/tests/fixtures/native-association-v1.json formatting. The unrelated fixture is unchanged; /tmp/jj-appearance-format-last.log.
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
pnpm exec prettier --check --ignore-unknown <all paths from jj --no-pager diff --name-only>
```

**Notes**

Every changed source, story, generated stylesheet, manifest and document passes targeted formatting after ledger correction.
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
pnpm spec:validate
```

**Notes**

Verification table supports only Implemented/Planned; the initial Partial row was corrected to Implemented with explicit local-source and release limits. Milestone M16 retains Partial rather than Done.
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
pnpm spec:first
```

**Notes**

135 protected files and six canonical chapters, before final evidence completion; /tmp/jj-appearance-spec-first-last.log.
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
pnpm check:progress
```

**Notes**

120 tasks; evidence bookkeeping only, before final evidence completion; /tmp/jj-appearance-progress-last.log.
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
pnpm checks
```

**Notes**

Design Core isolated source: earlier complete run failed WebKit duplicate native labels after other lanes passed; corrected in the owning library. Intermediate governance retries also exposed atomic acceptance form/word budgets. A later 205/207-file interaction run stalled and was terminated, never counted as a pass. Source logs /tmp/dc-appearance-checks*.log.
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
pnpm checks
```

**Notes**

Design Core final source qualification: 715 unit tests; 641 Storybook tests/207 files; static build/audit, formatting/types/governance; 14 workspace, 11 shell, 3 AI and 35 Shadcn Chromium/WebKit browser tests, one configured engine skip. /tmp/dc-appearance-checks-qualified.log.
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
pnpm spec:check; pnpm exec prettier --check spec/src/verification.md
```

**Notes**

Design Core final evidence edit: all three spec lanes and targeted formatting pass. Source committed as yovkqwtu / 2ad3d8fa; source workspace is clean.
<!-- /run -->
<!-- /evidence-field -->

