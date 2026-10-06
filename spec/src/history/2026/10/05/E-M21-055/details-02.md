---
schema: 1
id: E-M21-055
---

# E-M21-055 details 2

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 1
detail: "Before context retention: 6 passed/1 failed; before-read.log."
hasNote: false
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/stack-workspace-reads.test.ts
```
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: pass
failures: 0
detail: "Same target after retention: 7 passed; read-after.log."
hasNote: false
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/stack-workspace-reads.test.ts
```
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 1
detail: "Before shared surface: 1 failed, 4 skipped; before-conflict.log."
hasNote: false
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceConflicts.stories.svelte -t WorkspaceConflictSurface
```
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 2
detail: "Initial editor integration: 9 passed/2 failed; workspace surface was
  inside inert Main. editors.log."
hasNote: false
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceConflicts.stories.svelte catalog/components/store-mutations/StoreConflictEditor.stories.svelte
```
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 1
detail: "Application mounting repair: 25 passed/1 failed; standalone fixture
  needed sibling surface mounting. root-after.log."
hasNote: false
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspace.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceConflicts.stories.svelte catalog/components/store-mutations/StoreConflictEditor.stories.svelte
```
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: pass
failures: 0
detail: "Sibling surface integration: 27 passed; editor-final.log."
hasNote: false
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspace.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceConflicts.stories.svelte catalog/components/store-mutations/StoreConflictEditor.stories.svelte
```
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 4
detail: Two native term-ID type errors in each lane; generic number/bigint
  picker preserves native identities. web-final.log/catalog-final.log.
  Subsequent web/catalog runs passed with zero diagnostics.
hasNote: false
noteEncoding: markdown

```

```shell
pnpm web:check; pnpm catalog:check
```
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 1
detail: "Before multi-sided support: unsupported reason, 22 filtered;
  before-multisided.log."
hasNote: false
noteEncoding: markdown

```

```shell
cargo test --locked -p jj-store-jj --test local_stacks multisided_text_conflicts_offer_in_app_resolution_and_preserve_other_paths -- --exact
```
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 1
detail: "22 passed/1 failed: test incorrectly assumed apply order was native
  merge-term order. Asserted the complete positive-version set and selected
  reviewed text without reordering descriptors; native-after.log."
hasNote: false
noteEncoding: markdown

```

```shell
cargo test --locked -p jj-store-jj --test local_stacks
```
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: pass
failures: 0
detail: 1 passed, 22 filtered; multisided-after.log.
hasNote: false
noteEncoding: markdown

```

```shell
cargo test --locked -p jj-store-jj --test local_stacks multisided_text_conflicts_offer_in_app_resolution_and_preserve_other_paths -- --exact
```
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: pass
failures: 0
detail: 24 passed including multi-sided resolution preserving other paths,
  membership, merge identity and WIP change identity, plus aggregate size/binary
  rejection and read-only operation proof. native-verified.log. Existing unused
  assignment warning at line 1978 is unchanged.
hasNote: false
noteEncoding: markdown

```

```shell
cargo test --locked -p jj-store-jj --test local_stacks
```
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: pass
failures: 0
detail: Owning provider/all-target check passed twice;
  jj-check.log/jj-check-verified.log; same pre-existing unused-assignment
  warning.
hasNote: false
noteEncoding: markdown

```

```shell
cargo check --locked -p jj-store-jj --all-targets
```
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: pass
failures: 0
detail: Built explicit local CLI/server binaries; backend-build.log. macOS
  linker reports existing large unwind-section warning; no installed executable
  replacement.
hasNote: false
noteEncoding: markdown

```

```shell
pnpm dev:backend:build
```
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: fail
failures: 1
detail: Safe process guard rejected the Vite relative launch path against its
  absolute runtime marker. Verified ps and lsof cwd/owner, corrected only
  ignored runtime marker, retained Vite PID 57203; backend-restart.log.
hasNote: false
noteEncoding: markdown

```

```shell
JJ_STORE_DEV_PORT=7476 JJ_STORE_DEV_BACKEND_PORT=7478 pnpm dev:backend:restart
```
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: pass
failures: 0
detail: Owned backend restarted with fixture/search preserved;
  backend-restart-verified.log. Vite remains running; no reset or unrelated
  service restart.
hasNote: false
noteEncoding: markdown

```

```shell
JJ_STORE_DEV_PORT=7476 JJ_STORE_DEV_BACKEND_PORT=7478 pnpm dev:backend:restart
```
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: pass
failures: 0
detail: 58 passed across 6 direct controller/adapter/read/lifecycle surfaces;
  context partitions retain IDs and native descriptors, no duplicated lines or
  foreign changes; native replacement retains the full patch;
  null/unsupported/stale/foreign/closed conflict reads remain guarded.
  units.log.
hasNote: false
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/stack-hunk-context.test.ts web/tests/vite/stack-workspace-reads.test.ts web/tests/vite/production-stack-adapter.test.ts web/tests/vite/local-stack-file-mutations.test.ts web/tests/vite/production-stacks-lifecycle.test.ts web/tests/vite/stack-workspace-lazy-controller.test.ts
```
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: pass
failures: 0
detail: "Cold Vitest browser process: 52 passed across 6 existing story files.
  Last path does not exist and was not counted; Op Log navigation is covered by
  StoreApplication. stories-cold.log."
hasNote: false
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspace.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceConflicts.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceApplication.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceLazy.stories.svelte catalog/components/store-mutations/StoreConflictEditor.stories.svelte catalog/components/store-application/StoreApplication.stories.svelte catalog/components/store-operations/StoreOperationLog.stories.svelte
```
<!-- /run -->

<!-- run -->
### Run 18

```yaml
result: pass
failures: 0
detail: 7 passed for direct repository/Op Log diff consumer; log-diffs.log.
hasNote: false
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/community-projects/CommunityProjectsRepositoryLogDiff.stories.svelte
```
<!-- /run -->

<!-- run -->
### Run 19

```yaml
result: pass
failures: 0
detail: 8 passed before story split, including deletion-version disablement and
  contextual unsupported reasons; conflicts-verified.log.
hasNote: false
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceConflicts.stories.svelte
```
<!-- /run -->

<!-- run -->
### Run 20

```yaml
result: pass
failures: 0
detail: 31 passed after surface-to-preview handoff, disabled workspace picker
  and preview focus assertion; preview-focus.log.
hasNote: false
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceConflicts.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspace.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceApplication.stories.svelte
```
<!-- /run -->
<!-- /evidence-field -->

