---
schema: 1
id: E-UI-033
---

# E-UI-033 details 3

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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreRepositoryBranchContext.stories.svelte catalog/components/store-application/StoreCollaboration.stories.svelte
```

**Notes**

Eight passed and one failed. New body-content assertion used the wrong accessible region name Changed files in commit; corrected it to the existing Changed files label.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreRepositoryBranchContext.stories.svelte -t Actions
```

**Notes**

Exact bookmark action story passes. Create remains in the rail heading; Move remains available through Repository actions; selected changed-file content remains in the body.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreRepositoryNavigation.stories.svelte catalog/components/store-application/StoreApplication.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte catalog/components/store-application/StoreFileEditor.stories.svelte catalog/components/store-application/RepositoryReview.stories.svelte catalog/components/store-application/StoreWorkspaces.stories.svelte catalog/components/store-application/RepositoryLoading.stories.svelte
```

**Notes**

After moving notices to the body, 33 passed and two failed: a new outer wrapper disturbed file-tree layout and rendered all 600 virtual options. Moved notices into the existing shell body-frame, preserving the file browser and canvas parent layout.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreRepositoryApplication.stories.svelte catalog/components/store-application/RepositoryReview.stories.svelte -t 'LargeVirtualizedRepository|RepositoryDirectoryNavigation'
```

**Notes**

Both exact layout regressions pass; seven unrelated stories skipped. File tree resizing and bounded virtual rows restored.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreRepositoryNavigation.stories.svelte catalog/components/store-application/StoreApplication.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte catalog/components/store-application/StoreFileEditor.stories.svelte catalog/components/store-application/RepositoryReview.stories.svelte catalog/components/store-application/StoreWorkspaces.stories.svelte catalog/components/store-application/RepositoryLoading.stories.svelte catalog/components/store-application/StoreRepositoryBranchContext.stories.svelte catalog/components/store-application/StoreCollaboration.stories.svelte
```

**Notes**

All 44 interactions and blocking accessibility tests pass across nine directly affected files after rail cleanup and body-layout repair.
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
pnpm check:architecture
```

**Notes**

New collaboration assertion formatting put its existing story at 401 counted lines, over the 400-line story budget.
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
pnpm check:architecture
```

**Notes**

Used equivalent compact toBeNull assertion; architecture passes across 283 modules without changing the budget.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreCollaboration.stories.svelte -t RevisionCollaborationContext
```

**Notes**

Exact collaboration interaction/a11y scenario passes after compacting the equivalent absence assertion: one passed, seven skipped.
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
pnpm exec playwright test --config .scratch/workspace-browser.config.mts repository-navigation.spec.ts
```

**Notes**

Three cold browser tests pass, adding request-error presentation in the main body with no extra panels in the right rail.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec playwright test --config .scratch/workspace-browser.config.mts repository-navigation.spec.ts navigation.spec.ts workspaces.spec.ts reads.spec.ts mutations.spec.ts
```

**Notes**

21 passed, two failed after moving previously hidden request notices into the body. Axe found inherited destructive-description contrast of 4.49:1 and an unavailable workspace rendered its error twice. Read request notices now use the supported neutral Alert with a warning icon; failed workspace navigation uses its existing dedicated body state once.
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
pnpm exec playwright test --config .scratch/workspace-browser.config.mts reads.spec.ts workspaces.spec.ts -g 'native_graphs_files_and_source_rendering_are_accessible|missing workspace stays explicit and can recover to repository overview'
```

**Notes**

Both exact browser regressions pass with unchanged accessibility and single-alert assertions.
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
pnpm web:check
```

**Notes**

Repeated after the rail/notices changes: zero errors and warnings.
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
pnpm catalog:check
```

**Notes**

Repeated after the rail/notices changes: zero errors and warnings; catalog ownership boundaries pass.
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
pnpm web:build
```

**Notes**

Repeated cold UI builds after rail cleanup, body-layout repair and read-notice composition all succeed with the existing non-fatal Design Core and chunk-size warnings.
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
node .scratch/restart-workspace-ui.mjs
```

**Notes**

Owned UI-only restarts after rail cleanup and notice changes preserve existing demo data and explicit development binaries. Last recorded pre-margin PID 93335 at http://127.0.0.1:7460/.
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
pnpm exec playwright test --config .scratch/workspace-browser.config.mts repository-navigation.spec.ts navigation.spec.ts workspaces.spec.ts reads.spec.ts mutations.spec.ts
```

**Notes**

After the final header-margin request, this run had 22 passing tests and one missing-workspace timeout. It overlapped rebuilding the preview assets; the previously failing accessibility test passed. Retained the unchanged workspace assertion and repeated acceptance against the stable completed bundle.
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
pnpm exec playwright test --config .scratch/workspace-browser.config.mts workspaces.spec.ts -g 'missing workspace stays explicit and can recover to repository overview'
```

**Notes**

Fresh preview after the build completed: the exact missing-workspace case passes once; no product or assertion change.
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
pnpm exec playwright test --config .scratch/workspace-browser.config.mts repository-navigation.spec.ts navigation.spec.ts workspaces.spec.ts reads.spec.ts mutations.spec.ts
```

**Notes**

Final stable cold built-bundle run: all 23 directly affected browser tests pass, including native graph/source accessibility, mutation recovery, history/reload, exact workspace identity, missing workspace recovery, right-rail navigation, 390px keyboard overflow and notices in the main body.
<!-- /run -->
<!-- /evidence-field -->

