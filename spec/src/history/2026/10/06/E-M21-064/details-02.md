---
schema: 1
id: E-M21-064
---

# E-M21-064 details 2

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
jj --no-pager st/diff; jj workspace list; jj --no-pager log -r '43f6e70b & ::@' --no-graph
```

**Notes**

Work is already in default/main above merge076e0b23; earlier Apply/Unapply43f6e70b is an ancestor. No separate landing merge needed. Evidence in .local/stack-selection-refinements/. Community and Design Core sources inspected; JJ and shadcn skills applied.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceSelection.stories.svelte
```

**Notes**

before.log: both repeated-selection stories fail against original UI.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceSelection.stories.svelte
```

**Notes**

after.log:2/2 pass after user-action toggle helpers; programmatic selection remains idempotent.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/stack-workspace-selection.test.ts web/tests/vite/stack-workspace-lazy-controller.test.ts web/tests/vite/stack-workspace-target-history.test.ts web/tests/vite/stack-workspace.test.ts web/tests/vite/stack-workspace-topology.test.ts
```

**Notes**

unit.log:71/71 across5 files; lazy history, topology, target, retained drafts/selections and command locking covered.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook <Graph,Footer,Inline,Target,ProjectsHub>
```

**Notes**

stories.log:17 pass/5 fail. Graph assertions incorrectly expected auto instead of public styled viewport scroll; now accept either and measure actual overflow/motion. Footer CSS hover needs native pointer, not synthetic hover.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook <Graph,Footer,ProjectsHub>
```

**Notes**

stories-recheck.log:5 pass/2 failed tests plus ProjectsHub import failure from Vite dependency reoptimization. Notification success had empty message and was suppressed; supplied Folder path copied. Fresh process resolves import reload.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceSelection.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceGraph.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceFooter.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceBranches.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspace.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceResize.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceRefinements.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceHeaderDrops.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceDrag.stories.svelte catalog/components/store-stack-workspace/StoreStackTopology.stories.svelte catalog/components/store-work/ProjectsHub.stories.svelte
```

**Notes**

stories-final.log:57 pass/2 fail. Removed graphInset left an undefined branch-file indent; restored fixed2rem. Initial minimum card copy width forced ordinary cards into scrollbar space; reduced copy minimum to18rem while retaining DAG-width-dependent overflow.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceHeaderDrops.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceRefinements.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceGraph.stories.svelte
```

**Notes**

spacing-recheck.log:19/19, including unweakened edge scrollbar/inset assertions and native horizontal/vertical movement.
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
pnpm install --force --offline --lockfile=false
```

**Notes**

dependency-refresh.log: refreshed existing local Design Core source dependency; portable package manifests and lockfile unchanged.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceConflicts.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceSelection.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceGraph.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceFooter.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceBranches.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspace.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceResize.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceRefinements.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceHeaderDrops.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceDrag.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceInline.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceTarget.stories.svelte catalog/components/store-stack-workspace/StoreStackTopology.stories.svelte catalog/components/store-work/ProjectsHub.stories.svelte
```

**Notes**

consumer-final.log:70 pass/1 fail across14 files after refresh. New spacing assertion mistakenly compared two nonadjacent conflict rows, with a resolved row between (32px). Replaced with stronger pairwise measurements of every adjacent file/directory row in all3 modes.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/community-projects/CommunityProjectsRepositoryLogFiles.stories.svelte catalog/components/store-mutations/StoreConflictEditor.stories.svelte
```

**Notes**

file-list-consumers.log:11/11 in other direct File Listing consumers.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:check; pnpm catalog:check
```

**Notes**

Initial web/catalog runs found missing notification message, nullable folder path, and unsupported Testing Library exact option. Fixed before qualification; no warning suppressions.
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
pnpm web:check; pnpm catalog:check
```

**Notes**

web-qualified.log/catalog-qualified.log:0 errors/warnings;251 catalog modules.
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
pnpm check:architecture; pnpm spec:first
```

**Notes**

Initial cap check found301-line lane; removed duplicated locked guard already enforced by helper. Initial spec-first required api.md for controller boundary; added presentation-only policy.
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
pnpm check:architecture; pnpm check:ui-ownership; pnpm spec:first; pnpm spec:validate; pnpm exec prettier --check <changed files>
```

**Notes**

Final architecture1163 modules; ownership;21 protected files/2 chapters;5 validators/23 chapters/173 requirements; targeted formatting pass.
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
Owned Vite-only restart via inspectRuntimeProcesses/stopRuntimeProcesses/start/ready; GET http://127.0.0.1:7476/; CUA existing and fresh verification tabs
```

**Notes**

Vite restarted from main with refreshed source (PID78723); backend77067 and shared fixture retained. HTTP/admission readiness pass. CUA CDP/tab navigation timed out in both existing and fresh tabs, so direct live visual verification is unavailable; no sample mutations performed.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceConflicts.stories.svelte
```

**Notes**

conflict-gap-final.log:6/6 pass, including 2px separation between every adjacent file/directory row in list, folder and package modes. Alongside the 70 passing final-consumer stories and11 additional direct-consumer stories, all82 distinct affected stories pass after corrections.
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
Design Core: pnpm exec vitest run --project storybook src/shared/diff/file-listing/FileListing.stories.svelte
```

**Notes**

Separate source change ynmwovuk introduces measured row spacing through public CSS token. Exact reproduction before:2 fail/6 pass; after:8/8, including virtual scroll continuation and keyboard focus. Logs copied to this evidence directory.
<!-- /run -->
<!-- /evidence-field -->

