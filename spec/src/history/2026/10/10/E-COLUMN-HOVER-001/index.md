---
schema: 1
id: E-COLUMN-HOVER-001
---

# E-COLUMN-HOVER-001

<!-- evidence-field:id markdown -->
## Id

E-COLUMN-HOVER-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M4.13
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-006
- JST-UI-004
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-10
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-application/RepositoryApplication.stories.svelte
- catalog/components/store-application/StoreCollaboration.stories.svelte
- catalog/components/store-application/repository-layout-story-assertions.ts
- catalog/components/store-stack-workspace/StoreStackWorkspaceApplication.stories.svelte
- catalog/components/store-stack-workspace/StoreStackWorkspaceConflicts.stories.svelte
- catalog/components/store-stack-workspace/StoreStacksNavigation.stories.svelte
- spec/src/current/progress/M4/index.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

[]
<!-- /evidence-field -->

<!-- evidence-field:stories list -->
## Stories

- JJ Store/Application/Repository workspace
- JJ Store/Application/Persistent layout
- JJ Store/Application/Review discussion empty
- StoreStackWorkspaceApplication
- StoreStackWorkspaceConflicts
- StoreStacksNavigation
<!-- /evidence-field -->

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-application/RepositoryApplication.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte -t 'RepositoryWorkspace|PersistentLayout' --maxWorkers=1
```

**Notes**

RepositoryWorkspace and PersistentLayout used representative loading headers as readiness signals. The original product assertions remained unchanged during repair; see .local/column-preview/consumer-readiness-regression.log.
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
pnpm test:storybook catalog/components/store-application/RepositoryApplication.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte -t 'RepositoryWorkspace|PersistentLayout' --maxWorkers=1
```

**Notes**

The two exact repository workspace/persistence cases pass after waiting for real application view imports; eight unrelated cases filtered out.
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
pnpm test:storybook catalog/components/store-application/StoreCollaboration.stories.svelte -t ReviewDiscussionEmpty --maxWorkers=1
```

**Notes**

Exact review empty-state case passes; nine other stories filtered out.
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
pnpm test:storybook catalog/components/store-stack-workspace/StoreStackWorkspaceApplication.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceConflicts.stories.svelte catalog/components/store-stack-workspace/StoreStacksNavigation.stories.svelte --maxWorkers=1
```

**Notes**

All 21 application/conflict/navigation stories pass. Their beforeEach waits for the existing application import promises. No original assertion or timeout changed.
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
pnpm web:check
```

**Notes**

Zero errors. A concurrent skeleton warning initially remained; its owner subsequently fixed it.
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
pnpm catalog:check
```

**Notes**

Final check: zero errors/warnings, catalog boundary audit covers 381 production modules. Earlier attempts rejected a concurrent skeleton CSS warning.
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
pnpm check:ui-ownership
```

**Notes**

Final ownership check passes after the separate skeleton task supplied its missing provenance. No boundary exception was added.
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
pnpm install --lockfile=false --prefer-offline
```

**Notes**

Installs the prepared default Design Core archive with source guard and preview defaults; no tracked manifest or lockfile modification belongs to this slice.
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
node .local/column-preview/refresh-ui.mjs
```

**Notes**

Preflighted current ownership and refreshed frontend only; final owned refresh used frontend PID 36786 while preserving backend PID 36193. The prior Vite cache is retained in ignored storage.
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
pnpm test:storybook catalog/components/store-stack-workspace --maxWorkers=1
```

**Notes**

Final larger run stalled during dependency optimization before reporting tests and was interrupted with exit 130. This is a harness attempt, not a product failure count or a suite pass.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config .local/column-preview/vitest.acceptance.config.mjs --project storybook catalog/components/store-stack-workspace --maxWorkers=1
```

**Notes**

First ignored wrapper had an incorrect import path and failed before tests. Corrected wrapper isolated the cache, then an independent task created .local/agent-resume-parent/web/tsconfig.json and catalog/tsconfig.json; Vite forced a full reload. That startup attempt was interrupted without any reported product-test result. Final ignored wrapper also ignores temporary .local files; assertions, testTimeout and browser settings are unchanged.
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
pnpm check:progress
```

**Notes**

Working copy changed during revision-pinned validation because other tasks are editing the default workspace. This does not establish an evidence validation result.
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
JJ_STORE_CHECK_REVISION=458805787e6319e8a2272a42af2f0524c25b0a4f pnpm check:progress
```

**Notes**

Mixed dirty-work snapshot reports missing fresh evidence coverage for other tasks and generated navigation. The final owned slice will be checked at an immutable file-specific commit, excluding unrelated work.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Finish final wider consumer acceptance and record document validation.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64; direct shell/source Node 26.9.0, consumer pnpm-managed Node 24.10.0; pnpm 10.32.1; JJ 0.45.1; Design Core prepared archive 0.1.2, source Svelte 5.56.1; consumer Svelte 5.57.1; source Playwright 1.61.1. Registry manifests retain their existing ranges.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Application layout v1 is unchanged. Preview state is transient and never serialized. Existing full native identities, routes, fixtures and controller interfaces are unchanged.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Source implementation belongs to Design Core DC-SHA-036 in lrvkzxkr (5842cf0f), followed by nqrooplr for disposed interaction handlers. Columns default to previewOnHover=true, previewDelay=600 and previewDismissDelay=120. Collapsed/custom/fixed and floating sticky rails share the retained interactive contents host; compact/touch retain explicit controls. Source changes are landed above the preserved default AI/diff changes soqpuosr and nyxnzqvv. This consumer slice corrects readiness in six catalog files after the separate representative-skeleton change mpyvzmqo. It does not add consumer CSS, source aliases or a dependency patch. Source records retain the complete qualification and inherited notice history. Root aggregate, Rust/storage, registry publication and visual baselines are out of scope under targeted verification.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Finish consumer verification; retain the existing M4.13 registry-release next action.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-COLUMN-HOVER-001 details 2](details-02.md)
