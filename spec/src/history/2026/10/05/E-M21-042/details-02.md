---
schema: 1
id: E-M21-042
---

# E-M21-042 details 2

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-jj --test local_stacks --locked
```

**Notes**

before-native.log: missing Update/Delete command variants.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceSettings.stories.svelte
```

**Notes**

before-settings-stories.log: absent Settings action.
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
cargo test -p jj-store-jj --test local_stacks --locked; cargo check -p jj-store-jj --locked
```

**Notes**

native-tests.log/native-tests-final.log: nine stock-JJ-backed tests, including dirty target rebase, stable name-only identities, active/inactive removal, retained references, malformed/stale intent and existing seven core regressions. native-check.log passes.
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

contracts-generate.log: new local command variants emitted in schemas/openapi.json and web/src/lib/api/generated.ts.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools JJ_STORE_LOCAL_BROWSER_CONTRACT_CAPTURE=.local/stacks-settings/http-captures.json cargo test -p jj-store-cli --test local_browser filesystem_runtime_registers_and_reads_without_native_storage --locked
```

**Notes**

http-tests.log: one passed, 29 filtered. Actual Update/Delete responses join 48 captures. Build succeeds with existing large __eh_frame linker warning.
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
node --input-type=module (inline responseValidator/schemaValidator over .local/stacks-settings/http-captures.json and Update/Delete requests); pnpm test:contracts
```

**Notes**

All 48 actual HTTP captures and new request shapes validated; contract-tools.log: seven tooling tests and TypeScript contracts pass. Full 267-capture lane not needed for this extension of the existing command.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/production-stack-adapter.test.ts
```

**Notes**

adapter-tests.log: 11 direct adapter tests.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/stack-workspace.test.ts web/tests/vite/stack-workspace-menus.test.ts web/tests/vite/stack-workspace-setup.test.ts web/tests/vite/production-stack-adapter.test.ts web/tests/vite/production-stacks-application.test.ts web/tests/vite/production-stacks-lifecycle.test.ts
```

**Notes**

controller-tests.log: six files, 71 tests.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 15
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:check; pnpm catalog:check
```

**Notes**

Initial web five errors/catalog ten errors: ungenerated variants, generic target string versus object collision and unsupported exact role options. Corrected with targetBranch and supported locators; no contract weakening.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceSettings.stories.svelte
```

**Notes**

settings-stories.log: target option description prevented exact name match; Delete matched both confirmation and Settings. Corrected regex and dialog scope.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceSettings.stories.svelte
```

**Notes**

settings-stories-precise.log/settings-stories-final.log: null visibility assertion and duplicate No stack workspace locator; corrected null and empty-title assertions.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceSettings.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceApplication.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceSetup.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspacePinning.stories.svelte catalog/components/store-threads/StoreThreadAlignment.stories.svelte
```

**Notes**

verified-stories.log: five directly affected families, 19 interaction/accessibility stories. Includes setup spacing, fixed columns, Settings edit/preview retention, validation/cancel, readonly, Delete confirmation and unchanged Threads.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceSettings.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceApplication.stories.svelte catalog/components/store-threads/StoreThreadAlignment.stories.svelte
```

**Notes**

final-stories.log: 15 pass after navigation guard and layout helper extraction.
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
pnpm check:architecture
```

**Notes**

architecture.log: Application story exceeded 450 counted lines.
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
pnpm check:architecture
```

**Notes**

architecture-corrected.log: first extraction still exceeded 403 counted lines; reusable canvas-layout helper resolves it.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceSettings.stories.svelte -t EditWorkspaceSettingsWithRetainedDraft
```

**Notes**

before-identity.log: live review found the injected workspace identity snippet was not rendered. Exact assertion rejects the retained Thread Hash icon.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceSettings.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceApplication.stories.svelte catalog/components/store-threads/StoreThreadAlignment.stories.svelte
```

**Notes**

identity-final-stories.log: final 15 pass after rendering the workspace icon, retaining the default Thread icon.
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
pnpm web:check; pnpm catalog:check; pnpm check:ui-ownership; pnpm check:architecture; pnpm check:boundaries
```

**Notes**

Final web/catalog zero errors/warnings, 229 catalog boundaries, ownership, 1096 web architecture modules and 17 package boundaries pass. Logs: *check-identity.log, ownership-final.log, architecture-identity.log, boundaries.log; earlier *-final.log passes retained.
<!-- /run -->
<!-- /evidence-field -->

