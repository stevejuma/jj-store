---
schema: 1
id: E-WORKFLOW-HISTORY-001
---

# E-WORKFLOW-HISTORY-001 details 2

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
jj --no-pager st
```

**Notes**

Before: clean zklzvpqw cf8a7cf4 with parent 10134798. Existing History was editor-owned modal; no workflow URL selection or task-style grouping. User later observed legacy modal CSS collisions on new page details and a bootstrap metadata-home empty flash. Existing development data and user tab retained.
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
Read owning UI/API chapters, M28 and Community/Design Core references
```

**Notes**

Read maintained task list/details, shared tabs, source identity, Settings/CommandView, disclosure and Empty/Tooltip controls. Community is reference only; new modules are owned composition recorded in provenance.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:architecture
```

**Notes**

Intermediate 341 counted lines required catalog lifecycle extraction. Later skeleton additions produced 307, 304 and 301 counts; equivalent loading helpers and a single-use author expression removed duplication. Final below passes.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:check
```

**Notes**

Initial checks passed. Intermediate source copy controls had two nullable-ID errors; discovery change initially referenced controllers before declaration (two errors). Corrected null fallbacks and declaration order; final zero errors/warnings below.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 11
hasNote: true
noteEncoding: markdown

```

```shell
pnpm catalog:check
```

**Notes**

Initial mock browser lacked go, plus state-capture warning. Intermediate nullable IDs/unsupported ByRoleOptions exact (6 errors), pending fixture null result (1), and declaration/literal status typing (3). Corrected; final zero errors/warnings below.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 10
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-workflows/WorkflowHistory.stories.svelte catalog/components/store-workflows/StoreWorkflows.stories.svelte
```

**Notes**

Initial isolated runs: Vite optimizer reload plus contrast failures (4 failed), then two assertion failures in each of two runs (2 each). Fixed icons/foreground contrast, modal-transition wait and status option matching. Fourth run reloaded for new pin-off dependency, exposing two duplicate Run evidence landmarks. Evidence wrapper now has distinct parent sections.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-workflows/WorkflowHistory.stories.svelte catalog/components/store-workflows/StoreWorkflows.stories.svelte
```

**Notes**

Fifth: 10 pass/1 failed tooltip role assertion. Added explicit tooltip roles. Sixth: 11 pass/2 failed, closed Collapsible content stays mounted and oversized search needed Enter. Seventh: 12 pass/1 failed mounted-content assertion. Corrected to assert collapsed state and invisibility. Eighth: 15 pass after adding discovery skeleton stories.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/workflows-route.test.ts web/tests/vite/store-route-history.test.ts
```

**Notes**

20 route tests pass initially. Workflow URL validation covers duplicate/invalid fields, full uint64 pins and Settings precedence; browser history restoration retained.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/workflow-history.test.ts web/tests/vite/workflows-route.test.ts web/tests/vite/store-route-history.test.ts
```

**Notes**

24 tests pass: pinned run/catalog/evidence continuations, root and foreign-attempt refusal, stale detail response, wrong pin, and unknown-command bytes retained across refresh.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-workflows/WorkflowHistory.stories.svelte catalog/components/store-workflows/StoreWorkflows.stories.svelte catalog/components/store-workflows/WorkflowRun.stories.svelte catalog/components/store-workflows/WorkflowChoices.stories.svelte catalog/components/store-workflows/WorkflowTriggers.stories.svelte catalog/components/store-workflows/WorkflowEditor.stories.svelte
```

**Notes**

First regression run: 30 pass/1 fail because fixture URL output introduced another status landmark. Changed test-only URL output to a neutral span.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-workflows/WorkflowHistory.stories.svelte catalog/components/store-workflows/StoreWorkflows.stories.svelte catalog/components/store-workflows/WorkflowRun.stories.svelte catalog/components/store-workflows/WorkflowChoices.stories.svelte catalog/components/store-workflows/WorkflowTriggers.stories.svelte catalog/components/store-workflows/WorkflowEditor.stories.svelte
```

**Notes**

Final 31 tests across six workflow story files pass, including configured Axe, retained editor drafts, Mine/Other profile avatars, filters/routes, retry toasts, source ID controls, header tooltips, collapsed empty sections and representative bootstrap/detail skeletons. /tmp/jj-store-workflow-history-regressions-2.log.
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
pnpm exec vitest run --config /tmp/jj-store-workflow-run-mobile.config.mts --project storybook catalog/components/store-workflows/WorkflowHistory.stories.svelte
```

**Notes**

9 history stories pass at 390x844 with configured Axe. Temporary config changes only viewport. Fresh isolated Vitest/Vite browser processes exercise UI lifecycle without restarting the development service. /tmp/jj-store-workflow-history-mobile.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/workflow-history.test.ts web/tests/vite/workflows-route.test.ts web/tests/vite/store-route-history.test.ts web/tests/vite/workflow-run-source.test.ts web/tests/vite/workflow-code-options.test.ts
```

**Notes**

Final 32 tests across five focused unit files pass; source-read helpers and prior revision selections remain qualified. /tmp/jj-store-workflow-history-unit-final.log.
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
pnpm web:check
```

**Notes**

Final zero errors/warnings. /tmp/jj-store-workflow-history-web-final-4.log.
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
pnpm catalog:check
```

**Notes**

Final zero errors/warnings; catalog boundaries 428 modules. /tmp/jj-store-workflow-history-catalog-final-4.log.
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
pnpm check:architecture
```

**Notes**

Final 1829 modules pass. /tmp/jj-store-workflow-history-architecture-final-5.log.
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
pnpm check:ui-ownership
```

**Notes**

1 validator passes; all added owned production modules recorded. /tmp/jj-store-workflow-history-ownership.log.
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
cua_repl: isolated background workflow history tab, saved run, section expansion, Pin and reload
```

**Notes**

Live native run 6ec48651e74d9c668259be6ca6b03c45 in home c235f6f48a44e496f4f22a150a7a0356. Fixed metadata layout, row inset/no shadow, steve profile hydration and exact operation/commit controls observed. Pin produced at=103; fresh reload displayed Loading run details instead of metadata-home prompt. Run remains pending. No execution, Cancel/Retry submission, enable change, reset or publication. Screenshots /tmp/jj-store-workflow-history-rows.png and /tmp/jj-store-workflow-history-detail.png; user-owned tab untouched.
<!-- /run -->
<!-- /evidence-field -->

