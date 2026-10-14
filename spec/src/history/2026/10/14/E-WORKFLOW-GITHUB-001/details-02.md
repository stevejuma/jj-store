---
schema: 1
id: E-WORKFLOW-GITHUB-001
---

# E-WORKFLOW-GITHUB-001 details 2

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
pnpm dev:cargo -- test -p jj-workflow --locked
```

**Notes**

Before: 8 existing tests passed. After: 15 domain tests passed, including 7 YAML tests.
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
pnpm dev:cargo -- test -p jj-store-cli --lib workflow_host --locked
```

**Notes**

11 host tests passed, including split JSON, secret redaction and Unicode pipe windows.
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
pnpm dev:cargo -- test -p jj-store-service --test workflow_store --locked
```

**Notes**

2 tests passed; source/event admission and step summaries independent of log pagination and historical selection.
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
pnpm dev:cargo -- test -p jj-store-cli --test server_executable workflows::executor_parent_loss_stops_the_owned_process_group --locked -- --exact
```

**Notes**

1 process-lifetime regression passed.
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
JJ_STORE_DOCKER_HOST=unix:///Users/stevejuma/.docker/run/docker.sock JJ_STORE_FORGEJO_RUNNER=$PWD/.local/tools/forgejo-runner-13.2.0/forgejo-runner JJ_STORE_DOCKER=/usr/local/bin/docker JJ_STORE_RUSTFS_RUNTIME=$PWD/runtime/embedded-rustfs/target/debug/jj-store-rustfs pnpm dev:cargo -- test -p jj-store-cli --test server_executable workflows::github_yaml_executor_matches_pinned_source_and_retains_retry_history --locked -- --ignored --exact
```

**Notes**

Final real-storage test passed twice: 326.71s and 426.28s. Three named YAML steps, scoped/grouped logs, original-source retry after a newer revision, path nonmatch, same-revision YAML edits and history after server restart. Earlier runs exposed missing HOME and checkout-copy behavior, fixed with a private runtime home and pinned-source checkout. Test-only errors used an unsupported --limit flag and compared a moving page root; fixed by using actual CLI arguments and immutable run/definition/evidence/event comparison.
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
go build -ldflags '-X code.forgejo.org/forgejo/runner/v13/internal/pkg/ver.version=v13.2.0' -o ../forgejo-runner .
```

**Notes**

Built official Forgejo Runner v13.2.0 source on macOS ARM64 with Go 1.27.1. Binary --version verified. Source tag commit df6b843fb929bb04b933b09c6bf208774d42480b. No runner source vendored into production.
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
.local/tools/forgejo-runner-13.2.0/forgejo-runner --version
```

**Notes**

Native binary reports v13.2.0. Docker 29.8.0 and alpine:3.23 digest sha256:85fe1e81d6758c208f3e1eed4338a1997e19d4be002d4dd32d3100c9a8c010a0; raw three-step probe passed.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/workflow-console.test.ts web/tests/vite/workflow-files.test.ts web/tests/vite/workflow-host-settings.test.ts web/tests/vite/workflow-card-presentation.test.ts
```

**Notes**

30 unit tests passed. An earlier file-discovery assertion omitted the new path description (1 failed); corrected expected complete rows, including foreign-source rejection. Scoped terminal logs remain across pagination; raw nested groups and envelope suppression covered.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-workflows/WorkflowConsole.stories.svelte catalog/components/store-workflows/WorkflowEditor.stories.svelte catalog/components/store-workflows/WorkflowFilePicker.stories.svelte
```

**Notes**

16 focused interactions/Axe passed, including running step/attempt Refresh spinners. Earlier picker expectation omitted the accessible path description; corrected after inspecting actual option label. A combined cold suite had a transient dynamic-import failure; isolated cold console rerun passed 11 stories.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-workflows/WorkflowConsole.stories.svelte catalog/components/store-application/WorkflowHostSettings.stories.svelte
```

**Notes**

23 focused interactions/Axe passed. Settings capability, invalid JSON and toast paths retain drafts.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-workflows/WorkflowFilePicker.stories.svelte catalog/components/store-workflows/WorkflowConsole.stories.svelte catalog/components/store-application/LocalCompute.stories.svelte
```

**Notes**

25 interactions/Axe passed. Existing model Browse consumer defaults preserved; YAML path/Browse opens pinned source choices and failures toast without replacing the selection.
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

0 errors/warnings, including final browse integration.
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

0 errors/warnings; catalog inventory passes.
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
pnpm check:architecture
```

**Notes**

1864 modules passed. Existing 411-line console stylesheet split into layout import with identical cascade; new helpers respect owned module budgets.
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
pnpm check:boundaries
```

**Notes**

18 packages passed; no runner or controller runtime added to model/storage.
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
pnpm check:ui-ownership
```

**Notes**

UI ownership passes. Owned source and released Design Core only; provenance updated.
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
pnpm dev:cargo -- clippy -p jj-workflow -p jj-store-service -p jj-store-cli --lib --locked -- -D warnings
```

**Notes**

Stopped on unchanged jj-collab diagnostics (three collapsible-if cases and large ThreadLinkTargetV1 enum). No strict full dependency lint acceptance claimed.
<!-- /run -->

<!-- run -->
### Run 18

```yaml
result: fail
failures: 7
hasNote: true
noteEncoding: markdown

```

```shell
pnpm dev:cargo -- clippy -p jj-workflow -p jj-store-service -p jj-store-cli --lib --no-deps --locked -- -D warnings
```

**Notes**

Six existing service diagnostics plus one new validation collapsible-if. New slice diagnostics corrected; final default-warning review completes with only the six unrelated service warnings. ServerSettingsUpdate already contains the large workflow variant; capability increases its size. Strict lint remains a separate gap.
<!-- /run -->
<!-- /evidence-field -->

