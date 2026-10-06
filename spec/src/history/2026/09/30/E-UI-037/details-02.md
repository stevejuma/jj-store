---
schema: 1
id: E-UI-037
---

# E-UI-037 details 2

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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-mutations/StoreConflictEditor.stories.svelte -t 'Shell merge surface'
```

**Notes**

Initial test-name filter matched no generated story test names; 5 skipped and no acceptance evidence.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-mutations/StoreConflictEditor.stories.svelte
```

**Notes**

Before production changes: 5 failed at the missing structural-sheet region. Ignored .scratch/conflict-sheet-before.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-mutations/StoreConflictEditor.stories.svelte
```

**Notes**

After and final narrow runs: 5 Chromium interaction/accessibility checks pass, including dominant editor geometry, Rust tokens in all three panes, accept-all right, disabled header mutation/refresh, Escape/focus restoration, atomic apply, staged merge, unknown recovery and stale fingerprint. Stories remain visual-pending.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-mutations/StoreConflictEditor.stories.svelte catalog/components/store-mutations/StoreRepositoryActions.stories.svelte catalog/components/store-application/StoreApplication.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte catalog/components/store-application/RepositoryApplication.stories.svelte
```

**Notes**

30 directly affected shell, repository action and conflict scenarios pass twice. Direct-consumer coverage follows the new root-level sheet ownership; unrelated stories stay out of scope.
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
pnpm exec tsx --test web/tests/store-conflict-model.test.ts web/tests/production-store-conflicts.test.ts
```

**Notes**

5 exact native command/upload tests pass: full IDs and fingerprints, complete atomic paths, scoped bytes, upload bounds and executable/copy-term binding.
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
pnpm web:check
```

**Notes**

Initial and final web checks pass with zero errors/warnings.
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
pnpm catalog:check
```

**Notes**

Initial and final catalog checks pass with zero errors/warnings and 75-module catalog boundary audit.
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
pnpm check:architecture
```

**Notes**

289 production/catalog/script modules pass ownership and no-growth checks; repeated after final host and styling changes.
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
pnpm web:build
```

**Notes**

Fresh built UI passes. Existing nonfatal Design Core CSS minification and bundle warnings remain. No Rust build. Rebuilt after compact Left/Right labels removed source-term metadata from the editor header.
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
pnpm exec playwright test --config .scratch/workspace-browser.config.mts web/tests/browser/conflicts.spec.ts
```

**Notes**

3 cold built-UI browser tests pass twice. LocalTrusted bootstrap replaces the stale token-control bootstrap in this exact test. Both accept-all controls preserve term 0/2 and full fingerprints; custom edit clears prior selection; staged bytes remain exact and persist through path switching and preview/back; subsequent edits invalidate stale upload selection; desktop and 390px mobile root sheet dismiss cleanly. Fresh isolated Vite preview on 7471 shuts down after each run.
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
pnpm exec prettier --write web/src/lib/ui/store-conflicts web/src/lib/ui/store-application/StoreApplication.svelte web/src/lib/ui/store-mutations/StoreRepositoryActions.svelte catalog/fixtures/StoreConflictEditorFixture.svelte catalog/components/store-mutations/StoreConflictEditor.stories.svelte spec/src/ui.md
```

**Notes**

Changed implementation, fixtures and canonical chapter formatted; additional exact changed-file subsets were formatted after edits.
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
node .scratch/restart-workspace-ui.mjs
```

**Notes**

Owned demo restarted to load each final UI build using existing explicit development binaries and preserved repository data. Final PID 31688 ready at 127.0.0.1:7460. No reset/reseed, Rust rebuild or unrelated daemon restart.
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
CUA live built-UI verification
```

**Notes**

Separate hidden tab 5 verifies both header-menu and context-aware right-rail entry points. Final compact Left/Resolved/Right editor is 501px of the 664px sheet. Main and right rail are inert while open and restored on Cancel; focus returns to Resolve conflicts. Built-in accept-all left changes center content and Preview 0/1 to 1/1. No live upload, mutation preview or apply was invoked. Screenshot .scratch/conflict-sheet.png saved and temporary tab closed; user tabs preserved.
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
pnpm exec prettier --write web/tests/browser/conflicts.spec.ts
```

**Notes**

Exact cold browser scenarios formatted after LocalTrusted migration and mobile acceptance additions.
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
pnpm exec prettier --write web/src/lib/ui/store-application/StoreApplication.svelte web/src/lib/ui/store-application/StoreApplicationHeaderActions.svelte web/src/lib/ui/store-conflicts/StoreConflictEditor.svelte web/src/lib/ui/store-mutations/StoreRepositoryActions.svelte spec/src/api.md spec/src/verification.md
```

**Notes**

Final root ownership, header disable guard, focus target and canonical documents formatted.
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
pnpm exec prettier --write catalog/components/store-mutations/StoreConflictEditor.stories.svelte
```

**Notes**

Final disabled-header regression assertions formatted.
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
pnpm exec prettier --write web/src/lib/ui/store-conflicts/StoreConflictPath.svelte
```

**Notes**

Final minimal pane labels formatted.
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
pnpm exec prettier --write spec/src/milestones.md
```

**Notes**

Same-change evidence ledger and task row formatted.
<!-- /run -->
<!-- /evidence-field -->

