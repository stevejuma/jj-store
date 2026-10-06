---
schema: 1
id: E-M11-004
---

# E-M11-004 details 2

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
jj new uonqxxvs nrtskmus -m "Merge native WorkStore into main workspace"
```

**Notes**

Two-parent merge in main default; sole ledger conflict reconciled by retaining every parent evidence object and adding four M11 task rows. Main independent M4.21-M4.29 task rows retained.
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
pnpm exec tsx --test web/tests/store-route-codec.test.ts
```

**Notes**

Seven combined native/evolution/WorkStore route tests.
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
pnpm web:check
```

**Notes**

Zero errors and warnings.
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
pnpm catalog:check
```

**Notes**

Zero errors and warnings; 81 catalog modules.
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

312 production modules; existing 300 counted-line limit unchanged.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/production-work.test.ts web/tests/vite/production-store-evolution.test.ts web/tests/vite/production-store-controller.test.ts web/tests/vite/production-store-route.test.ts
```

**Notes**

Two existing files pass eleven tests. Controller/route placeholder filenames matched no files; actual route history target is store-route-history.test.ts and is verified separately.
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
pnpm exec prettier --write spec/src/{api,ui,milestones}.md web/tests/vite/store-route-history.test.ts
```

**Notes**

Formatted integration docs, ledger and exact shared-history regression.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/store-route-history.test.ts -t 'Projects and Evo Log restore independent identities through browser history'
```

**Notes**

Exact new integration test passes; five unrelated tests skipped in this focused run.
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
pnpm web:test
```

**Notes**

All 58 web unit tests pass.
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
pnpm catalog:test
```

**Notes**

All 88 catalog/controller unit tests in 19 files pass.
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
pnpm spec:validate
```

**Notes**

19 chapters and 115 requirements pass.
<!-- /run -->

<!-- run -->
### Run 12

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

Default published validator assumes a single @-; merge has two parents. Supported explicit --base/--head selection below qualifies the complete common-ancestor-to-merge diff.
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
pnpm check:boundaries
```

**Notes**

15 package dependency boundaries pass.
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
pnpm check:progress
```

**Notes**

Progress CLI also assumed a single parent. Exact disposable JJ merge regression reproduced this; checker now enumerates all parent diffs/evidence and requires genuinely new integration evidence.
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
pnpm exec prettier --check spec/src/{api,milestones,ui,verification}.md web/src/lib/controller/{contracts.ts,production-store-lifecycle.svelte.ts,production-store-view.ts,store-route-codec.ts,store-route-history.svelte.ts} web/tests/{store-route-codec.test.ts,vite/store-route-history.test.ts}
```

**Notes**

Auto-merged overlapping UI sources and initial integration edits formatted.
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/StoreWork.stories.svelte catalog/components/store-application/StoreApplication.stories.svelte catalog/components/store-application/StoreEvolutionColumn.stories.svelte catalog/components/store-application/StoreRepositoryNavigation.stories.svelte catalog/components/store-application/StoreFileEditor.stories.svelte catalog/components/store-application/StoreCollaboration.stories.svelte catalog/components/store-mutations/StoreConflictEditor.stories.svelte catalog/components/store-mutations/StoreMutationCommitPicker.stories.svelte
```

**Notes**

40 pass / two fail: navigation retained the old six-tab expectation; conflict sidebar immediate lookup preceded virtual-row layout. Updated navigation to all seven destinations and awaited both exact native paths without weakening retained-draft/header assertions.
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
node --test --test-name-pattern=progress_checks_merge_against_every_parent scripts/governance.test.mjs
```

**Notes**

Before checker fix, disposable two-parent JJ merge fails with ambiguous @- instead of checking integration evidence.
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
node --test --test-name-pattern=progress_checks_merge_against_every_parent scripts/governance.test.mjs
```

**Notes**

Exact regression now rejects a merge that merely combines parent evidence, and accepts genuinely new merge evidence; owned disposable repo removed.
<!-- /run -->
<!-- /evidence-field -->

