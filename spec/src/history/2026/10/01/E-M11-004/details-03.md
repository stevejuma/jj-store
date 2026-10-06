---
schema: 1
id: E-M11-004
---

# E-M11-004 details 3

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
pnpm exec prettier --write scripts/{check-progress,governance.test}.mjs spec/src/{spec-governance,ui}.md catalog/components/store-application/StoreRepositoryNavigation.stories.svelte catalog/components/store-mutations/StoreConflictEditor.stories.svelte
```

**Notes**

Formatted checker and corrected direct-consumer stories/canonical docs.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreRepositoryNavigation.stories.svelte catalog/components/store-mutations/StoreConflictEditor.stories.svelte -t 'RightRailNavigation|HeaderActionsAndFileBrowser'
```

**Notes**

Both exact originally failing stories pass on a fresh Vitest/Vite browser process; ten other stories skipped in this focused run.
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
node --test scripts/dev-demo-collaboration.test.mjs scripts/dev-demo-repository.test.mjs scripts/dev-demo-state.test.mjs scripts/governance.test.mjs scripts/web-architecture.test.mjs
```

**Notes**

36 full direct-consumer source tests pass; includes the disposable real JJ merge and multi-parent evidence variant regression.
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
pnpm check:progress
```

**Notes**

86 tasks pass using actual two-parent merge; same-change ledger enforcement intact.
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
pnpm spec:first --base 4a224b913d6c64630b62d0ddfdef4903b6657839 --head e407caa957ed506619ad02bf382d2b994af1b908
```

**Notes**

Published explicit-base gate audits all 155 protected files against 13 changed canonical chapters; full integration from shared ancestor rather than either arbitrary parent.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/StoreWork.stories.svelte catalog/components/store-application/StoreApplication.stories.svelte catalog/components/store-application/StoreEvolutionColumn.stories.svelte catalog/components/store-application/StoreRepositoryNavigation.stories.svelte catalog/components/store-application/StoreFileEditor.stories.svelte catalog/components/store-application/StoreCollaboration.stories.svelte catalog/components/store-mutations/StoreConflictEditor.stories.svelte catalog/components/store-mutations/StoreMutationCommitPicker.stories.svelte
```

**Notes**

Final cold Vitest/Vite process: all 42 directly affected stories in eight files pass; includes WorkStore, shell, evolution, seven-destination navigation, file editor/composers, conflict sheet/header/browser and native commit picker. Interaction and accessibility enforced; no visual baselines changed.
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

Final corrected-story type check: zero errors/warnings and 81 passing catalog modules.
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
pnpm exec prettier --write spec/src/milestones.md
```

**Notes**

Formatted current integration evidence.
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
pnpm exec prettier --check scripts/{check-progress,governance.test}.mjs spec/src/{api,milestones,spec-governance,ui,verification}.md web/src/lib/controller/{contracts.ts,production-store-lifecycle.svelte.ts,production-store-view.ts,store-route-codec.ts,store-route-history.svelte.ts} web/tests/{store-route-codec.test.ts,vite/store-route-history.test.ts} catalog/components/store-application/StoreRepositoryNavigation.stories.svelte catalog/components/store-mutations/StoreConflictEditor.stories.svelte
```

**Notes**

All merged overlap and manual integration files pass formatting.
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
pnpm spec:validate
```

**Notes**

19 chapters and 115 requirements pass after governance fix.
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
JJ_STORE_DEV_PORT=7480 JJ_STORE_DEV_RUSTFS_RUNTIME=/Users/stevejuma/code/lapismd/jj-store/runtime/embedded-rustfs/target/debug/jj-store-rustfs pnpm web:test:real
```

**Notes**

Fresh merged main binaries/physical stack pass startup gates: first 1024ms, persisted restart 872ms, full fixture preparation 347175ms, 171488KiB RSS. Five real scenarios pass in 51.4s, including WorkStore; atomic conflict scenario still targets the old modal/path-heading/Use-left controls from feature ancestry. Current main uses a named merge-sheet region, native conflict region and Accept All Incoming Changes from Left. Updated only those selectors, retaining identical preview/apply requests, exact conflict.txt path, select_term=0 and full fingerprint. Runner stopped/deleted its owned root; transient checkout hooks removed.
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
pnpm exec prettier --write web/tests/real/native-repository.spec.ts spec/src/ui.md
```

**Notes**

Formatted current merge-surface cold acceptance selectors and owning chapter.
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
pnpm exec prettier --check scripts/{check-progress,governance.test}.mjs spec/src/{api,milestones,spec-governance,ui,verification}.md web/src/lib/controller/{contracts.ts,production-store-lifecycle.svelte.ts,production-store-view.ts,store-route-codec.ts,store-route-history.svelte.ts} web/tests/{store-route-codec.test.ts,vite/store-route-history.test.ts,real/native-repository.spec.ts} catalog/components/store-application/StoreRepositoryNavigation.stories.svelte catalog/components/store-mutations/StoreConflictEditor.stories.svelte
```

**Notes**

Includes final real merge-sheet regression; all changed and automatically merged overlaps formatted.
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
pnpm check:progress
```

**Notes**

86 tasks pass with final real-regression evidence registered; original parent objects remain exact.
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
JJ_STORE_DEV_PORT=7480 JJ_STORE_DEV_RUSTFS_RUNTIME=/Users/stevejuma/code/lapismd/jj-store/runtime/embedded-rustfs/target/debug/jj-store-rustfs JJ_STORE_REAL_KEEP=1 pnpm web:test:real
```

**Notes**

Final fresh merged-main physical stack passes all six real scenarios in 47.1s, including the exact formerly failing atomic-conflict case. First 616ms, persisted restart 1119ms, full fixture preparation 330084ms, server/storage RSS 155376KiB. Native IDs, complete route preservation, identical preview/apply payloads, select_term=0/full conflict fingerprint, WorkStore links/tasks/subtasks/state/comments/tombstones, unknown receipt recovery, explicit refresh/reload, axe and narrow-screen gates retained. Startup limits unchanged at 30000ms/524288KiB.
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
JJ_STORE_DEV_ROOT=/var/folders/_3/k1jv9rtn47s_0k5bm6qn_lk80000gn/T/jj-store-real-browser-UAUSzm pnpm dev:stop
```

**Notes**

Stopped final owned test server/storage. Verified no root-owned processes, removed only its disposable root, and verified transient checkout hooks absent.
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
pnpm spec:validate
```

**Notes**

Final completed integration: 19 chapters / 115 requirements.
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
pnpm check:progress
```

**Notes**

Final completed two-parent integration: 86 tasks; every original parent evidence entry preserved.
<!-- /run -->
<!-- /evidence-field -->

