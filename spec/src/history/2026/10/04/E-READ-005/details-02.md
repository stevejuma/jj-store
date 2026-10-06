---
schema: 1
id: E-READ-005
---

# E-READ-005 details 2

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
cargo test -p jj-store-cli --test local_browser --locked local_metadata_and_bookmark_pages_do_not_load_history_or_index -- --exact bounded_reads::local_metadata_and_bookmark_pages_do_not_load_history_or_index
```

**Notes**

Before: workspaces request rejected as unknown enum variant. Exact rerun after implementation passes with 10,000 bookmarks, corrupt derived index, one history visit and exact bookmark-9999 lookup.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/store-workspaces.test.ts -t 'workspace pages do not drain'
```

**Notes**

Before: expected one request, observed two due to draining reference windows. Owning file rerun passes after explicit workspace-window contract.
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
cargo test -p jj-store-cli --test local_browser --locked local_metadata_and_bookmark_pages_do_not_load_history_or_index -- --exact bounded_reads::local_metadata_and_bookmark_pages_do_not_load_history_or_index
```

**Notes**

Initial implementation compiler diagnostics (borrowed closure item/lifetime inference) corrected with owned bounded page selections. Final exact regression: 1 passed.
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
pnpm contracts:generate
```

**Notes**

Regenerated additive native/local Workspaces and exact-selector schemas and TypeScript contracts.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit --maxWorkers=2 web/tests/vite/production-reference-reader.test.ts web/tests/vite/store-workspaces.test.ts web/tests/vite/projects-code-navigation.test.ts
```

**Notes**

23 passed: independent startup, selected name outside first window, no drain, stale continuation rejection, coalescing, optional failure, exact missing-name rejection, retained workspace and Projects routes.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit --maxWorkers=2 web/tests/vite/production-store-workspaces.test.ts web/tests/vite/production-store-bootstrap.test.ts web/tests/vite/production-store-data-source.test.ts web/tests/vite/local-browser.test.ts web/tests/vite/local-browser-merge.test.ts web/tests/vite/production-store-events.test.ts
```

**Notes**

47 direct consumer tests passed.
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
cargo test -p jj-store-cli --test native_lifecycle --locked browsing_acceptance:: -- --test-threads=2
```

**Notes**

Initial test fixture used wrong publication method/old-new head order. Corrected fixture then corrected metric: native metadata_nodes counts JSON shape nodes; object_reads proves three hydrated records. No production admission or assertion threshold was weakened.
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
cargo test -p jj-store-cli --test native_lifecycle --locked browsing_acceptance:: -- --test-threads=2
```

**Notes**

5 passed including native HTTP: forward/backward filtered workspace windows, exact/missing names, shared-target hydration and cursor mismatch by kind/exact selector.
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
cargo test -p jj-store-cli --test local_browser --locked -- --test-threads=2
```

**Notes**

9 passed; 1 opt-in Linux smoke not rerun because unchanged scoped file algorithms were qualified in E-READ-004. Includes updated real local HTTP contract capture.
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
pnpm web:check; pnpm catalog:check
```

**Notes**

0 errors/warnings; catalog boundaries 182 modules. Temporary source qualification, not registry release acceptance.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreWorkspaces.stories.svelte
```

**Notes**

Fresh Vite process: 5 interaction/accessibility tests passed. Visual baselines unchanged.
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

929 production modules pass architecture limits.
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

Fresh built UI; existing chunk-size advisory retained.
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
pnpm exec playwright test --config web/playwright.config.ts web/tests/browser/workspaces.spec.ts web/tests/browser/repository-navigation.spec.ts web/tests/browser/reads.spec.ts web/tests/browser/file-editor-navigation.spec.ts
```

**Notes**

Port 7458 already occupied. Preserved existing listener and used temporary copy of the production preview host on 17548 with otherwise identical isolated test configuration.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec playwright test --config .local/bounded-reads/playwright.config.ts web/tests/browser/workspaces.spec.ts web/tests/browser/repository-navigation.spec.ts web/tests/browser/reads.spec.ts web/tests/browser/file-editor-navigation.spec.ts
```

**Notes**

First isolated cold run: 18 passed, 1 stale error-location assertion failed. Parent already uses StoreNotice and actual browser showed Notifications toast; updated test to assert that existing contract and retain right-rail exclusion.
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
cargo check -p jj-store-jj -p jj-store-service -p jj-store-http -p jj-store-cli --all-targets --locked
```

**Notes**

Affected Rust production and test targets checked.
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
JJ_STORE_LOCAL_BROWSER_CONTRACT_CAPTURE=$PWD/.local/bounded-reads/references-local-capture.json target/debug/deps/local_browser-893d8c4057fcb950 --exact filesystem_runtime_registers_and_reads_without_native_storage; JJ_STORE_BROWSING_CONTRACT_CAPTURE=$PWD/.local/bounded-reads/references-native-capture.json target/debug/deps/native_lifecycle-e6652c34f7ed2d93 --exact http_browsing_acceptance::http_browsing_windows_are_authorized_pinned_cursor_bound_and_contract_captured; node --input-type=module (responseValidator on both captures)
```

**Notes**

Both actual capture tests passed; all 43 native/local HTTP/SSE responses conform, including workspace windows and exact selectors.
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
pnpm test:contracts
```

**Notes**

7 schema tests plus generated TypeScript passed.
<!-- /run -->
<!-- /evidence-field -->

