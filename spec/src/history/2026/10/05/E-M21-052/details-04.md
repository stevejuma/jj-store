---
schema: 1
id: E-M21-052
---

# E-M21-052 details 4

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
pnpm dev:backend:build
```

**Notes**

backend-build.log: 2m23s; existing compact-unwind linker advisory. Development binary only.
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
JJ_STORE_DEV_PORT=7476 JJ_STORE_DEV_BACKEND_PORT=7478 pnpm dev:backend:restart
```

**Notes**

backend-restart.log: native backend updated; Vite and fixture preserved.
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
CUA existing live tab 20 getAXState
```

**Notes**

Live 7476 workspace verified read-only after HMR: focused tabs, workspace controls and Unstaged present.
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
node scripts/stacks-browser-acceptance.mjs -g cold_native_selected_file_and_hunk_mutations_with_undo_redo (eleventh)
```

**Notes**

browser-mutations-eleventh.log: File drop rejected after concurrent refresh.
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
node scripts/stacks-browser-acceptance.mjs -g cold_native_selected_file_and_hunk_mutations_with_undo_redo (twelfth)
```

**Notes**

browser-mutations-twelfth.log: Retargeting the semantic stage button and current graph did not repair missing cached selection projection.
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
node scripts/stacks-browser-acceptance.mjs -g cold_native_selected_file_and_hunk_mutations_with_undo_redo (thirteenth)
```

**Notes**

browser-mutations-thirteenth.log: Native event tracing narrowed failure to drop feedback; no browser exceptions.
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
node scripts/stacks-browser-acceptance.mjs -g cold_native_selected_file_and_hunk_mutations_with_undo_redo (fourteenth)
```

**Notes**

browser-mutations-fourteenth.log: Post-dispatch trace proved stale selection feedback despite a visible lazy file row; no synthetic drag or assertion weakening.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/stack-workspace-lazy-controller.test.ts -t 'same-scope refresh and history' (before)
```

**Notes**

loaded-file-projection-before.log: same-scope refresh lost loaded WIP/commit selection models.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/stack-workspace-lazy-controller.test.ts -t 'same-scope refresh and history' (after)
```

**Notes**

loaded-file-projection-after.log: one passes; loaded rows stay valid for drop and inline staging through refresh/history.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/stack-workspace-segments.test.ts web/tests/vite/production-stack-adapter.test.ts web/tests/vite/stack-workspace-lazy-controller.test.ts web/tests/vite/stack-workspace.test.ts web/tests/vite/production-stacks-lifecycle.test.ts web/tests/vite/production-store-settings.test.ts web/tests/vite/index-freshness.test.ts web/tests/vite/production-stacks-application.test.ts web/tests/vite/notification-service.test.ts (final projection)
```

**Notes**

loaded-file-units.log: 107 focused units pass across nine files.
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
pnpm docs:check
```

**Notes**

final-docs-check.log: source reviews/generated references and documentation closure pass before final file-projection repair.
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
pnpm docs:review application (file projection)
```

**Notes**

loaded-file-doc-review.log: reviewed affected current source against existing browsing/editing manuals.
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
pnpm exec prettier --write docs/maintenance/coverage.json web/tests/real/stacks-browser.spec.ts
```

**Notes**

completion-format-repair.log: repair metadata formatting left by docs:review.
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
pnpm exec prettier --write web/src/lib/controller/stack-workspace/controller-files.ts web/src/lib/controller/stack-workspace/controller-snapshot.ts web/tests/vite/stack-workspace-lazy-controller.test.ts web/tests/real/stacks-browser.spec.ts spec/src/ui.md
```

**Notes**

loaded-file-format.log: final repair files formatted.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
node scripts/stacks-browser-acceptance.mjs -g cold_native_selected_file_and_hunk_mutations_with_undo_redo (fifteenth)
```

**Notes**

browser-mutations-fifteenth.log: Native selected-file and hunk transfer succeeded; retained WIP patch did not reload after operation change.
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
node scripts/stacks-browser-acceptance.mjs -g cold_native_selected_file_and_hunk_mutations_with_undo_redo (sixteenth)
```

**Notes**

browser-mutations-sixteenth.log: Temporary patch tracing proved loaded:true with unloaded file content at the new operation; no new patch request.
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
node scripts/stacks-browser-acceptance.mjs -g cold_native_selected_file_and_hunk_mutations_with_undo_redo (seventeenth)
```

**Notes**

browser-mutations-seventeenth.log: Real hunk drag began on selectable diff content and did not reach the assignment dialog.
<!-- /run -->

<!-- run -->
### Run 18

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
node scripts/stacks-browser-acceptance.mjs -g cold_native_selected_file_and_hunk_mutations_with_undo_redo (eighteenth)
```

**Notes**

browser-mutations-eighteenth.log: Reproduced hunk drag targeting failure; no native transfer dispatched.
<!-- /run -->
<!-- /evidence-field -->

