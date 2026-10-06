---
schema: 1
id: E-UI-049
---

# E-UI-049 details 2

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 9
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreApplication.stories.svelte catalog/components/store-application/RepositoryLayout.stories.svelte catalog/components/store-application/NativeReviewDetail.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte catalog/components/store-application/StoreFileEditor.stories.svelte catalog/components/store-work/ProjectsFilters.stories.svelte catalog/components/store-work/StoreWork.stories.svelte
```

**Notes**

Baseline: 9 fail, 22 pass; missing compact header insets and Markdown token colors. .scratch/header-source-before.log.
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
pnpm exec playwright test --config web/playwright.config.ts web/tests/browser/editor-focus.spec.ts -g 'draft Source'
```

**Notes**

Strict production CSP baseline: no keyword token in draft Source. .scratch/source-csp-before.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreApplication.stories.svelte catalog/components/store-application/RepositoryLayout.stories.svelte catalog/components/store-application/NativeReviewDetail.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte catalog/components/store-application/StoreFileEditor.stories.svelte catalog/components/store-work/ProjectsFilters.stories.svelte catalog/components/store-work/StoreWork.stories.svelte
```

**Notes**

30 pass; initial CodeMirror measurement differs by 11.1875px before layout settles. Wait for the existing &lt;=1px assertion, retaining its tolerance. .scratch/header-source-after.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreApplication.stories.svelte catalog/components/store-application/RepositoryLayout.stories.svelte catalog/components/store-application/NativeReviewDetail.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte catalog/components/store-application/StoreFileEditor.stories.svelte catalog/components/store-work/ProjectsFilters.stories.svelte catalog/components/store-work/StoreWork.stories.svelte
```

**Notes**

31 pass in fresh Storybook runner. .scratch/header-source-final.log.
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
pnpm check:architecture
```

**Notes**

StoreRepositoryApplication story exceeds its 400-line budget at 405 counted lines. .scratch/header-source-architecture.log.
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

Initial cleanup leaves 401 counted lines. Extract the two reusable source-highlight assertions. .scratch/header-source-architecture-final.log.
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

446 modules; shared source-highlight assertion helper keeps story within its budget. .scratch/header-source-architecture-repaired.log; repeated after dependency refresh in header-source-architecture-static.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/RepositoryReview.stories.svelte catalog/components/store-application/NativeAgentMenus.stories.svelte catalog/components/store-application/StoreRepositoryBranchContext.stories.svelte catalog/components/store-work/ProjectsHub.stories.svelte catalog/components/store-mutations/StoreConflictEditor.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte catalog/components/store-application/StoreFileEditor.stories.svelte
```

**Notes**

27 pass; Column Canvas header remains 12px in catalog because preview.css owns its parallel host theme rule. Set the same public padding token there. .scratch/header-source-consumers.log.
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
pnpm exec playwright test --config web/playwright.config.ts web/tests/browser/editor-focus.spec.ts web/tests/browser/file-editor-navigation.spec.ts web/tests/browser/repository-navigation.spec.ts web/tests/browser/local-workspaces.spec.ts web/tests/browser/local-search-settings.spec.ts web/tests/browser/local-sync.spec.ts
```

**Notes**

10 pass; draft Source token text exists but color equals foreground under strict CSP. Source Design Core injects its token CSS even for spans; fix in the owning package. .scratch/header-source-browser.log.
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
pnpm dev:restart
```

**Notes**

Initial app build/restart completes before the strict-policy color regression exposes the shared source defect. .scratch/header-source-dev-restart.log.
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
pnpm install --lockfile=false
```

**Notes**

Refresh the installed Design Core file package through the existing governed local-package hook. Published manifest range and tracked registry lockfile unchanged. .scratch/header-source-package-refresh.log.
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
pnpm dev:restart
```

**Notes**

Rebuild bundled Design Core highlighting CSS; owned LocalTrusted demo ready at port 7460, search ready, native repo/installation identity and RustFS volume retained. .scratch/header-source-dev-restart-static.log. Native compilation here is startup evidence, not Rust qualification.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreApplication.stories.svelte catalog/components/store-application/RepositoryLayout.stories.svelte catalog/components/store-application/NativeReviewDetail.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte catalog/components/store-application/StoreFileEditor.stories.svelte catalog/components/store-work/ProjectsFilters.stories.svelte catalog/components/store-work/StoreWork.stories.svelte catalog/components/store-application/RepositoryReview.stories.svelte catalog/components/store-application/NativeAgentMenus.stories.svelte catalog/components/store-application/StoreRepositoryBranchContext.stories.svelte catalog/components/store-work/ProjectsHub.stories.svelte catalog/components/store-mutations/StoreConflictEditor.stories.svelte
```

**Notes**

Final complete direct impact surface: 51 interactions/accessibility stories in 12 families pass with refreshed public source package. .scratch/header-source-stories-static.log.
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
pnpm exec playwright test --config web/playwright.config.ts web/tests/browser/editor-focus.spec.ts web/tests/browser/file-editor-navigation.spec.ts web/tests/browser/repository-navigation.spec.ts web/tests/browser/local-workspaces.spec.ts web/tests/browser/local-search-settings.spec.ts web/tests/browser/local-sync.spec.ts
```

**Notes**

10 pass including strict-CSP draft token colors and safe literal script bytes; mobile Search sticky-header comparison measures before shell mode settles, producing an exact 1px difference. .scratch/header-source-browser-static.log.
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
pnpm exec playwright test --config web/playwright.config.ts web/tests/browser/local-search-settings.spec.ts
```

**Notes**

Wait for public app-shell data-display-mode=mobile before measuring. The unchanged &lt;1px sticky assertion now passes in a cold fresh browser; together with the preceding ten successes all 11 scenarios are covered. .scratch/header-source-search-mobile-final.log.
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
pnpm web:check
```

**Notes**

Zero errors/warnings before and after source package refresh. .scratch/header-source-web-check.log and header-source-web-static.log.
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
pnpm catalog:check
```

**Notes**

Zero errors/warnings; 112 catalog modules. Initial, final-helper and refreshed-package runs pass. .scratch/header-source-catalog-check.log, header-source-catalog-final.log and header-source-catalog-static.log.
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
pnpm check:boundaries
```

**Notes**

15 packages. .scratch/header-source-boundaries.log.
<!-- /run -->
<!-- /evidence-field -->

