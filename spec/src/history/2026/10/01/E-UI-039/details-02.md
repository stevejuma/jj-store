---
schema: 1
id: E-UI-039
---

# E-UI-039 details 2

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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-mutations/StoreConflictEditor.stories.svelte
```

**Notes**

Before: 1 failed, 5 passed; missing Conflict file browser complementary landmark. Log .scratch/conflict-browser-before.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-mutations/StoreConflictEditor.stories.svelte
```

**Notes**

First after run failed during Vite virtual CSS imports for unchanged Design Core Field/Checkbox; no tests executed. Fresh retry loaded those components and exposed the implementation error recorded next. Log .scratch/conflict-browser-after.log.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 6
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-mutations/StoreConflictEditor.stories.svelte
```

**Notes**

Fresh retry: 6 failed plus 6 matching unhandled Svelte errors. Undefined per-path merged bindings were invalid with the child fallback. Function bindings now return initial descriptor bytes and retain drafts in the sheet owner; redundant child initialization was removed. Log .scratch/conflict-browser-after-retry.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-mutations/StoreConflictEditor.stories.svelte
```

**Notes**

After draft fix: 1 failed, 5 passed. The new test queried any footer and matched the MergeEditor internal status footer. Scope corrected to the sheet-owned footer, retaining the no-Cancel and header-button assertions. Log .scratch/conflict-browser-final-story.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-mutations/StoreConflictEditor.stories.svelte
```

**Notes**

Final: all 6 Chromium interaction/accessibility stories pass. Verifies exact shared file listing, continuous 1px border and body height, header icon actions, no sheet footer/Cancel, hide/show, mode selection, per-path draft retention, active-path actions, atomic apply, unknown recovery, stale rejection, syntax tokens and shell focus/inert restoration. Log .scratch/conflict-browser-story-pass.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-mutations/StoreConflictEditor.stories.svelte catalog/components/store-mutations/StoreRepositoryActions.stories.svelte catalog/components/store-application/StoreApplication.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte catalog/components/store-application/RepositoryApplication.stories.svelte
```

**Notes**

31 directly affected conflict, repository action and application/shell stories pass. No unrelated suite escalation. Log .scratch/conflict-browser-consumers.log.
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
pnpm exec tsx --test web/tests/store-conflict-model.test.ts web/tests/production-store-conflicts.test.ts
```

**Notes**

5 exact command/upload tests pass: native fingerprints/terms, complete atomic paths, scoped handles, bytes and bounds.
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

Initial and final checks pass with zero errors/warnings; final log .scratch/conflict-browser-web-check-final.log.
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

Initial and final checks pass with zero errors/warnings and 75-module catalog boundary audit; final log .scratch/conflict-browser-catalog-check-final.log.
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
pnpm check:architecture
```

**Notes**

Initial and final 291-module boundary and size audits pass. Header actions and body layout are separate small owners.
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
pnpm web:build
```

**Notes**

Fresh production UI build passes. Existing nonfatal Design Core CSS minification and bundle-size warnings remain. No Rust rebuild. Log .scratch/conflict-browser-build.log.
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
pnpm exec playwright test --config .scratch/workspace-browser.config.mts web/tests/browser/conflicts.spec.ts
```

**Notes**

First cold built-UI run: 1 failed, 2 passed. Nested body shell made the background main locator ambiguous. The test now excludes the sheet-owned main and retains the same background inert/focus assertions. Log .scratch/conflict-browser-cold.log.
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
pnpm exec playwright test --config .scratch/workspace-browser.config.mts web/tests/browser/conflicts.spec.ts
```

**Notes**

Final cold built-UI run: 3 passed. Header stage uploads exact active-path draft bytes; atomic preview retains full native binding; preview/back and file switching preserve draft, subsequent edits invalidate staged selection, both accept-all terms remain exact, 390px mobile browser opens over the body and file selection returns to the editor, close/Escape restores background/focus. Fresh isolated Vite preview on 7471 shuts down after run. Log .scratch/conflict-browser-cold-final.log.
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
node .scratch/restart-workspace-ui.mjs
```

**Notes**

Owned demo ready at 127.0.0.1:7460, PID 41996, with existing explicit development binaries and repository data preserved. No reseed/reset or unrelated restart.
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
CUA live built-UI verification
```

**Notes**

Temporary hidden tab 8 verifies header toggle and real conflict file listing. Sidebar and body both 610px high with 1px right border; merge editor 594px of 664px sheet; header owns toggle/Delete/Stage/Preview/Close, with no sheet footer. Screenshot .scratch/conflict-file-browser.png saved and tab closed; user tabs preserved. No live upload, preview or apply.
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
pnpm exec prettier --write web/src/lib/ui/store-conflicts/StoreConflictEditor.svelte web/src/lib/ui/store-conflicts/StoreConflictPath.svelte web/src/lib/ui/store-conflicts/StoreConflictBody.svelte web/src/lib/ui/store-conflicts/StoreConflictHeaderActions.svelte catalog/components/store-mutations/StoreConflictEditor.stories.svelte spec/src/ui.md spec/src/api.md
```

**Notes**

Production owners, focused story and canonical chapters formatted; exact two-file draft-binding subset also formatted after fix.
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
pnpm exec prettier --write web/tests/browser/conflicts.spec.ts web/src/lib/ui/store-conflicts/store-conflicts.css
```

**Notes**

Focused cold browser scenarios and consumer styles formatted; browser assertions formatted again after explicit background-main selector.
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
pnpm exec prettier --write web/src/lib/ui/store-conflicts/StoreConflictEditor.svelte web/src/lib/ui/store-conflicts/StoreConflictPath.svelte web/src/lib/ui/store-conflicts/StoreConflictBody.svelte web/src/lib/ui/store-conflicts/StoreConflictHeaderActions.svelte web/src/lib/ui/store-conflicts/store-conflicts.css catalog/components/store-mutations/StoreConflictEditor.stories.svelte web/tests/browser/conflicts.spec.ts spec/src/api.md spec/src/ui.md spec/src/verification.md spec/src/milestones.md
```

**Notes**

All 11 changed files formatted.
<!-- /run -->
<!-- /evidence-field -->

