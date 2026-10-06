---
schema: 1
id: E-M14-016
---

# E-M14-016 details 2

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
jj --no-pager st; read tracked/parent AGENTS, jj-jujutsu skill, owning API/UI/CLI/replication/mutations/desktop requirements and M14/M18 ledger; inspect Community then Design Core
```

**Notes**

Clean before state; existing Search/Capture forms reused. No relevant memory facts used.
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
cargo check -p jj-store-server -p jj-store-http --locked (first)
```

**Notes**

Missing axum::Json import in new HTTP module; corrected.
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
cargo fmt --all; cargo check -p jj-store-server -p jj-store-http --locked (final)
```

**Notes**

Affected server/HTTP and direct service/RPC/search dependencies compile.
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

Generated OpenAPI and TypeScript; additive settings routes and Search limits.
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
cargo test -p jj-store-server --test capture_management server_settings:: --locked
```

**Notes**

Three persistence/auth/actual-HTTP regressions pass. Saving does not initialize capture state, contact peers or change authority; re-created manager selects desired values at startup.
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
cargo test -p jj-store-cli --test native_lifecycle local_search_save_apply_rebuild_disable_and_restart_preserve_authority_and_pins --locked
```

**Notes**

One exact Search lifecycle regression now changes all seven advanced fields; validated save, Apply, rebuild/retry and restart while preserving manifest and retained read pins. macOS linker emitted existing compact-unwind size diagnostic.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:check (first)
```

**Notes**

Missing equal import and Svelte state variable shadowing; corrected.
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
pnpm web:check; pnpm catalog:check (final)
```

**Notes**

No errors or warnings; catalog boundaries175 modules.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/server-settings.test.ts web/tests/vite/production-search-settings.test.ts
```

**Notes**

Initial14 tests across2 files.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/server-settings.test.ts web/tests/vite/production-search-settings.test.ts web/tests/vite/production-capture-management.test.ts
```

**Notes**

18 tests across3 files including seven advanced Search bounds, installation availability, section preservation, dirty base revisions, unknown outcomes and obsolete GET rejection.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreServerSettings.stories.svelte catalog/components/store-application/StoreSearchSettings.stories.svelte (first)
```

**Notes**

Four new stories caught role=status on a dd; move status to nested span. Advanced Search had duplicate fields/IDs from insertion; remove duplicate.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreServerSettings.stories.svelte catalog/components/store-application/StoreSearchSettings.stories.svelte catalog/components/store-application/StoreStorageSettings.stories.svelte catalog/components/store-application/StoreSyncSettings.stories.svelte (final)
```

**Notes**

20 direct interaction/accessibility stories pass; visual-pending.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:architecture (first and intermediate)
```

**Notes**

Main controller grew beyond300 counted lines (308 then304). Extract installation configuration factory and shared teardown; do not change limits.
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
pnpm check:architecture; pnpm check:boundaries; pnpm check:ui-ownership
```

**Notes**

905 production modules,17 package boundaries and ownership validator pass.
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
pnpm web:build; pnpm exec playwright test --config .local/agent-prompt-markup/playwright.config.ts local-server-settings.spec.ts local-capture-settings.spec.ts (first)
```

**Notes**

Seven pass; final mobile fixture tried to click hidden desktop navigation at390px. Navigate before reducing viewport; rerun. New controls scoped Axe check itself passes.
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
pnpm dev:backend:build
```

**Notes**

Development binaries built; installed tools untouched. Existing macOS compact-unwind linker diagnostic retained.
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
pnpm contracts:check
```

**Notes**

Full registered lane required because settings routes/Search schema and capture registration share the public inventory. 237 actual HTTP/SSE responses,7 contract-tool tests and generated TypeScript pass. This is a focused contract surface, not a full repository suite.
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
pnpm web:build; pnpm exec playwright test --config .local/agent-prompt-markup/playwright.config.ts local-server-settings.spec.ts (final)
```

**Notes**

Three cold built-bundle editor/advanced-Search/scoped-Axe/mobile checks pass. Previous five Capture regressions passed in the first combined run. Viewport set back to desktop before hidden-sidebar navigation in test; scoped controls qualified at390px.
<!-- /run -->
<!-- /evidence-field -->

