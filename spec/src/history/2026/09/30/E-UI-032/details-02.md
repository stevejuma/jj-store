---
schema: 1
id: E-UI-032
---

# E-UI-032 details 2

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
pnpm exec tsx --test web/tests/store-route-codec.test.ts
```

**Notes**

Before implementation: four existing tests passed and the workspace round-trip regression failed because workspace context was lost.
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
pnpm exec tsx --test web/tests/store-route-codec.test.ts web/tests/production-store-loaders.test.ts
```

**Notes**

All 15 route and native-loader tests pass after implementation.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/store-workspaces.test.ts web/tests/vite/store-route-history.test.ts catalog/controller/store-controller.test.ts
```

**Notes**

After adding first-page continuation coverage, 17 tests passed; one new assertion expected a prose error rather than the existing pagination snapshot mismatch error. The assertion was corrected to preserve the actual failure contract.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/store-workspaces.test.ts web/tests/vite/store-route-history.test.ts catalog/controller/store-controller.test.ts
```

**Notes**

Initially 17 passed; final 18 pass, including seven workspace regressions, first-page snapshot continuity, shared commit identities, history, missing workspace, explicit revision inspection and working-copy Refresh.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/production-store-bootstrap.test.ts web/tests/vite/production-store-collaboration.test.ts web/tests/vite/production-store-settings.test.ts web/tests/vite/production-store-events.test.ts web/tests/vite/production-file-reader.test.ts
```

**Notes**

All 22 directly affected controller tests pass, repeated after the final reference-reader adjustment. Output captured in ignored .scratch/workspace-controller-tests.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreWorkspaces.stories.svelte
```

**Notes**

Five production-component stories pass interaction and accessibility: picker/tab history, keyboard search and overview, collapsed sidebar, mobile header and file-edit guard. Repeated after the final Workspace title, icon and smaller picker changes; output in ignored .scratch/workspace-final-picker-stories.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application catalog/components/community-projects/CommunityProjectsRepositoryLog.stories.svelte catalog/components/community-repository-branch-command/CommunityRepositoryBranchCommand.stories.svelte
```

**Notes**

64 directly affected stories passed; BookmarkCards expected only the commit callback argument. Its assertion now also requires the exact workspace name instead of dropping the identity.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/community-repository-branch-command/CommunityRepositoryBranchCommand.stories.svelte catalog/components/store-application/StoreWorkspaces.stories.svelte
```

**Notes**

All eight stories pass, including the repaired BookmarkCards identity assertion and five picker stories.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreApplication.stories.svelte catalog/components/store-application/StoreWorkspaces.stories.svelte
```

**Notes**

After the final presentation tweak, all 11 shell stories passed; the picker file failed import because the Tailwind/Vite style transform reported Invalid declaration: createVirtualizer in unchanged Design Core FileListingVirtualRows. A fresh isolated picker run then passed all five stories without source, assertion or dependency changes.
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
pnpm web:check
```

**Notes**

Final production Svelte/TypeScript diagnostics: zero errors and warnings.
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
pnpm catalog:check
```

**Notes**

Final catalog Svelte diagnostics: zero errors and warnings; 71 module boundaries pass.
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
pnpm check:architecture
```

**Notes**

All 278 production module boundaries and size budgets pass. The working-copy card, snapshot helpers and catalog navigation were extracted to retain existing module budgets.
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
pnpm web:build
```

**Notes**

Repeated after the final reader and visual changes; production build and web manifest pass. Existing Design Core global-selector minification warnings and large-chunk warning remain non-fatal. Final output in ignored .scratch/workspace-web-build.log.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec playwright test --config .scratch/workspace-browser.config.mts workspaces.spec.ts navigation.spec.ts
```

**Notes**

First cold attempt: two navigation tests passed; both new tests timed out because their locator expected textbox Search workspaces. The Command View input is a combobox labelled Select workspace.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec playwright test --config .scratch/workspace-browser.config.mts workspaces.spec.ts navigation.spec.ts
```

**Notes**

Second cold attempt: two navigation tests passed; both new tests still timed out because the native accessible name uses the Command View root label rather than the explicit input aria-label. The test now selects the observed combobox name Select workspace.
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
pnpm exec playwright test --config .scratch/workspace-browser.config.mts workspaces.spec.ts navigation.spec.ts
```

**Notes**

All four cold built-bundle tests pass, repeated after the final Workspace header/padding change. Two native workspace names share one commit across two reference pages; selection survives tabs, reload, Back/Forward and explicit Refresh, passes exact native log context, and recovers explicitly from an unavailable workspace. The ignored config uses a fresh UI preview on port 7471 and pre-existing captured native responses; no Cargo fixture regeneration or server build. Repeated after making the error heading describe the repository view, so an unavailable explicit reference is not mislabelled as a missing workspace.
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
node .scratch/restart-workspace-ui.mjs
```

**Notes**

The owned LocalTrusted demo was restarted using existing explicit development binaries and the rebuilt UI only. Repository 1bdb89c409c1b891ac7f180c01a265d5 and embedded RustFS state persist. Final server PID 90121; origin http://127.0.0.1:7460/. No reset, reseed, installation or unrelated service restart.
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
CUA live browser acceptance at http://127.0.0.1:7460/
```

**Notes**

The native demo workspace selects its recorded working copy and writes workspace=demo alongside at_op. Reload restores that context; the section reads Workspace with a workspace icon. The button measures 32px high with 8px inline padding; screenshot saved in ignored .scratch/workspace-picker.png. Sidebar repository items retain their existing structure. The original browser tab could not be controlled after a focus-emulation timeout; a fresh in-app verification tab was used and retained.
<!-- /run -->
<!-- /evidence-field -->

