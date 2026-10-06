---
schema: 1
id: E-UI-070
---

# E-UI-070 details 2

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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/StoreWork.stories.svelte -t Repository links
```

**Notes**

Name filter skipped8 stories; not acceptance. Retried the exact family without filter.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/StoreWork.stories.svelte
```

**Notes**

Before:1 expected modal regression fails,7 pass. Inline Repository ID present before dialog opens. /tmp/jj-project-repository-dialog-before.log.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/StoreWork.stories.svelte catalog/components/store-work/ProjectsRepositoryActions.stories.svelte
```

**Notes**

Initial new tests:wrong expected fixture ID in2 stories and stale dialog node after reopen in1. Corrected fixtures/selectors without changing assertions. /tmp/jj-project-repository-stories.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/StoreWork.stories.svelte catalog/components/store-work/ProjectsRepositoryActions.stories.svelte
```

**Notes**

12/12 focused stories including unknown link recovery/read-only access pass in a fresh Vitest browser/Vite process with blocking accessibility. /tmp/jj-project-repository-stories-retry.log.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_REAL_BASE_URL=http://127.0.0.1:7460 JJ_STORE_REAL_REPOSITORY=1bdb89c409c1b891ac7f180c01a265d5 JJ_STORE_REAL_PROJECT=38d1c167c245c42fd7d1818086a89ed1 pnpm exec playwright test --config web/playwright.real.config.ts web/tests/real/project-repository-actions.spec.ts
```

**Notes**

Initial desktop/mobile tests clicked Add while project details were still loading, so only repository item was available. Added a settled project readiness assertion. /tmp/jj-project-repository-browser.log.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_REAL_BASE_URL=http://127.0.0.1:7460 JJ_STORE_REAL_REPOSITORY=1bdb89c409c1b891ac7f180c01a265d5 JJ_STORE_REAL_PROJECT=38d1c167c245c42fd7d1818086a89ed1 pnpm exec playwright test --config web/playwright.real.config.ts web/tests/real/project-repository-actions.spec.ts
```

**Notes**

Desktop contrast audit sampled enabled Buttons during their disabled-opacity transition; mobile used an ambiguous status locator. Waited for public component animations to settle, retained the contrast rule and selected the unknown-registration status. /tmp/jj-project-repository-browser-retry.log.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 31
hasNote: true
noteEncoding: markdown

```

```shell
pnpm catalog:check
```

**Notes**

New story byRole exact option is unsupported by Testing Library typings; fixture version is number|bigint. Removed unsupported options without widening matching, incremented native version as bigint. /tmp/jj-project-repository-catalog.log.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:architecture
```

**Notes**

LocalFolders320 and StoreProjects321 counted lines exceeded300. Extracted owned toolbar and folder-dialog presentation. /tmp/jj-project-repository-architecture.log.
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
python3 UI component extraction
```

**Notes**

First scripted extraction stopped on an indentation-sensitive substring after the parent rewrite; restored the dialog through the new component and completed extraction before any acceptance. No unrelated files affected.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/StoreWork.stories.svelte catalog/components/store-work/ProjectsRepositoryActions.stories.svelte catalog/components/store-work/ProjectsDirectory.stories.svelte catalog/components/store-work/ProjectsHub.stories.svelte catalog/components/store-work/ProjectsFilters.stories.svelte catalog/components/store-work/ProjectsActivityControls.stories.svelte catalog/components/store-work/ProjectsActivity.stories.svelte catalog/components/store-work/TaskScroll.stories.svelte catalog/components/store-work/ProjectsReviewDetail.stories.svelte
```

**Notes**

46/46 in9 direct-consumer families with blocking accessibility. Final extraction/navigation rerun follows. /tmp/jj-project-repository-direct-stories.log.
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
JJ_STORE_REAL_BASE_URL=http://127.0.0.1:7460 JJ_STORE_REAL_REPOSITORY=1bdb89c409c1b891ac7f180c01a265d5 JJ_STORE_REAL_PROJECT=38d1c167c245c42fd7d1818086a89ed1 pnpm exec playwright test --config web/playwright.real.config.ts web/tests/real/project-repository-actions.spec.ts
```

**Notes**

2/2 final desktop/mobile mock registration tests then3/3 including Settings Add/Reconnect rows. Normal axe rules, no overflow, keyboard focus return and identical request retry pass. All registration/picker writes intercepted; persistent fixture unchanged. /tmp/jj-project-repository-browser-{final,settings}.log.
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
pnpm web:check; pnpm catalog:check; pnpm check:ui-ownership; pnpm check:architecture
```

**Notes**

Relevant checks run separately. Final extracted source:0 web errors/warnings;0 catalog errors/warnings and132 catalog modules;1 ownership validator;748 architecture modules. /tmp/jj-project-repository-{web-extracted,catalog-final,ownership,architecture-final}.log. Initial and final web checks also passed.
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
pnpm exec prettier --write changed source, stories, fixtures, browser tests and canonical chapters
```

**Notes**

Formatting passes recorded in /tmp/jj-project-repository-{format,story-format,story-fix-format,browser-format,browser-fix-format,extract-format,settled-format,settings-format}.log. Final changed-file check follows.
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
cua_repl: temporary project tab18; open Add menu, local folder and Link repository dialogs; inspect geometry/focus; save screenshots; close tab
```

**Notes**

Live Settings rows have no horizontal overflow. Menu includes both dedicated icons; local-folder and linking dialogs open and restore focus on Cancel. No persistent requests submitted. User tab preserved.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/StoreWork.stories.svelte catalog/components/store-work/ProjectsRepositoryActions.stories.svelte catalog/components/store-work/ProjectsDirectory.stories.svelte catalog/components/store-work/ProjectsHub.stories.svelte catalog/components/store-work/ProjectsFilters.stories.svelte catalog/components/store-work/ProjectsActivityControls.stories.svelte catalog/components/store-work/ProjectsActivity.stories.svelte catalog/components/store-work/TaskScroll.stories.svelte catalog/components/store-work/ProjectsReviewDetail.stories.svelte catalog/components/store-work/NavigationPersistence.stories.svelte
```

**Notes**

47/47 in10 direct-consumer families after final extraction, fresh Vitest browser/Vite process with blocking accessibility. /tmp/jj-project-repository-direct-stories-final.log.
<!-- /run -->

<!-- run -->
### Run 16

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

Ledger tests entries initially used plain paths instead of path#literal_test_name; moved story/dynamic browser evidence to runs as in existing UI entries. /tmp/jj-project-repository-progress.log.
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
pnpm spec:first; pnpm spec:validate; pnpm check:progress; pnpm exec prettier --check all18 changed files
```

**Notes**

Consuming governance/format lanes completed before commit:14 protected files/4 chapters;5 validators/21 chapters/147 requirements;120 tasks;all changed files formatted. Evidence bookkeeping rerun after correction. /tmp/jj-project-repository-{spec-first,spec-validate,progress-final,format-check}.log.
<!-- /run -->
<!-- /evidence-field -->

