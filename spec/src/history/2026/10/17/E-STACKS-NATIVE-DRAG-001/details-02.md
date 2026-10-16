---
schema: 1
id: E-STACKS-NATIVE-DRAG-001
---

# E-STACKS-NATIVE-DRAG-001 details 2

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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/stack-workspace-lazy-controller.test.ts -t 'resolved native branch diffs'
```

**Notes**

Exact new regression before implementation: native branch file row was not draggable.
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
jj --no-pager st; node --version; pnpm --version; jj --version
```

**Notes**

Initial working copy clean; actual versions above. Preserved existing running Vite/backend and fixture.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/stack-workspace-lazy-controller.test.ts -t 'resolved native branch diffs'
```

**Notes**

Before implementation: 1 failed, 16 skipped; after: 1 passed, 16 skipped. Exact missing branch drag acceptance.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/stack-branch-file-owners.test.ts web/tests/vite/production-stack-adapter.test.ts web/tests/vite/stack-workspace-lazy-controller.test.ts
```

**Notes**

Initial source/adapter family: 41 passed before adding the concrete native preview integration case.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/production-stack-adapter.test.ts -t 'production branch drops'
```

**Notes**

1 passed, 18 skipped. Verifies native source/path/token plus exact operation, binding generation and configuration revision reach preview.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/stack-branch-file-owners.test.ts web/tests/vite/production-stack-adapter.test.ts web/tests/vite/stack-workspace-lazy-controller.test.ts web/tests/vite/local-stack-file-mutations.test.ts web/tests/vite/stack-workspace-selection.test.ts web/tests/vite/stack-workspace-header-drops.test.ts web/tests/vite/stack-workspace-sorting.test.ts web/tests/vite/stack-workspace-spacer.test.ts web/tests/vite/stack-workspace.test.ts
```

**Notes**

Final production/selection/direct-consumer family: 99 tests passed in 9 files after the bounded read helper extraction. Earlier same-family pass was also 99.
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
pnpm test:storybook catalog/components/store-stack-workspace/StoreStackWorkspaceNativeDrag.stories.svelte
```

**Notes**

Initial new-story run failed once on an assertion racing dialog visibility; waitFor corrected the test, then passed. Expanded three-story run had one failure from expecting the file-local toast title instead of the existing central Repository request failed title; corrected assertion, no feedback policy change.
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
pnpm test:storybook catalog/components/store-stack-workspace/StoreStackWorkspaceNativeDrag.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceDrag.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceHeaderDrops.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceProduction.stories.svelte
```

**Notes**

Final 16 interaction/accessibility stories passed in 4 files after extraction. Includes real adapter/application shell, staging files, exact native hunk transfers to commits and Unstaged, retained source before confirmation, ambiguous read-only sources, operational-error toast, existing folder/header/move/sort/cancel behavior. Fresh headless process; visual baselines untouched.
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

Initial run found two implicit-any errors in the new helper, corrected with explicit StackCommit[]. Final run: 0 errors, 0 warnings; catalog boundary check passed across 472 modules.
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
pnpm check:ui-ownership
```

**Notes**

Ownership validator passed. No external Community/catalog production imports or local dependency substitutions introduced.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:architecture
```

**Notes**

Initial audit found two newly oversized touched modules plus three pre-existing violations. Extracted owned lazy file reads and fixture command/diff helpers; touched modules now fit. Remaining unchanged parent/current counts: StoreStackWorkspaceApplication.stories.svelte 401/400, StoreApplicationMain.svelte 313/300, StoreStackWorkspace.svelte 310/300. Verified parent counts using jj file show -r @- and exported countedLines. These unrelated files were not changed or allowlisted.
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
pnpm exec prettier --write <explicit changed TypeScript, Svelte and canonical Markdown paths>
```

**Notes**

Formatted each touched source/document; final exact-file check recorded separately.
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
curl --silent --output /dev/null --write-out 'UI HTTP %{http_code}\n' http://127.0.0.1:7460/
```

**Notes**

UI HTTP 200; Vite HMR remains available.
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
curl --silent --output /dev/null --write-out 'Proxy API HTTP %{http_code}\n' -H 'Origin: http://127.0.0.1:7460' -H 'Sec-Fetch-Site: same-origin' -H 'Sec-Fetch-Mode: cors' -H 'Sec-Fetch-Dest: empty' -H 'x-jj-store-protocol: 1' http://127.0.0.1:7460/v1/bootstrap
```

**Notes**

API HTTP 200 with complete normal browser admission headers. Earlier incomplete curl header probes correctly received 403; no admission bypass or server change was made.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/production-stacks-application.test.ts web/tests/vite/production-stacks-lifecycle.test.ts web/tests/vite/stack-workspace-reads.test.ts
```

**Notes**

27 directly affected application/fixture, lifecycle and read-ownership tests passed in 3 files after extraction. Combined with the 99-test selection family: 126 distinct focused tests.
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
pnpm test:storybook catalog/components/store-stack-workspace/StoreStackWorkspaceCommands.stories.svelte
```

**Notes**

All 6 directly affected file/hunk command stories passed, including original draggable-hunk/card and inline staging behavior. Combined with 16 other affected stories: 22 distinct interaction/accessibility cases; visual baselines untouched.
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

Seven specification validators passed after canonical and evidence updates.
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
pnpm check:progress
```

**Notes**

Initial new evidence record omitted numeric run failures metadata; validator rejected the draft. Added explicit failure counts, retained the initial red regression and reran successfully.
<!-- /run -->
<!-- /evidence-field -->

