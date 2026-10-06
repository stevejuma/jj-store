---
schema: 1
id: E-M14-008
---

# E-M14-008

<!-- evidence-field:id markdown -->
## Id

E-M14-008
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M14
- M14.6
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-01
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

local-sync-settings
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-application/StoreSyncSettings.stories.svelte
- catalog/controller/in-memory-store-controller.svelte.ts
- catalog/controller/in-memory-store-settings.svelte.ts
- catalog/fixtures/SyncSettingsFixture.svelte
- catalog/fixtures/sync-settings.ts
- crates/jj-store-cli/tests/native_lifecycle/http_local_management.rs
- crates/jj-store-cli/tests/native_lifecycle/http_sync_management.rs
- crates/jj-store-http/src/dto.rs
- crates/jj-store-http/src/lib.rs
- crates/jj-store-http/src/local_management.rs
- crates/jj-store-http/src/routes.rs
- crates/jj-store-http/src/sync_management.rs
- crates/jj-store-server/src/lib.rs
- crates/jj-store-server/src/process.rs
- crates/jj-store-server/src/sync_management.rs
- crates/jj-store-service/src/lib.rs
- crates/jj-store-service/src/replication_control.rs
- crates/jj-store-service/src/sync_management.rs
- schemas/openapi.json
- spec/src/api.md
- spec/src/cli.md
- spec/src/compatibility.md
- spec/src/mutations.md
- spec/src/replication.md
- spec/src/security.md
- spec/src/ui.md
- web/src/lib/api/generated.ts
- web/src/lib/controller/contracts.ts
- web/src/lib/controller/http-repository-transports.ts
- web/src/lib/controller/http-store-data-source.ts
- web/src/lib/controller/native-data-source.ts
- web/src/lib/controller/production-store-actions.ts
- web/src/lib/controller/production-store-index.ts
- web/src/lib/controller/production-store-state.svelte.ts
- web/src/lib/controller/production-store-view.ts
- web/src/lib/controller/production-sync-settings.ts
- web/src/lib/controller/sync-settings.ts
- web/src/lib/ui/store-settings/StoreSettings.svelte
- web/src/lib/ui/store-settings/StoreSettingsNavigation.svelte
- web/src/lib/ui/store-settings/StoreSyncSettings.svelte
- web/src/lib/ui/store-settings/model.ts
- web/tests/browser/local-sync.spec.ts
- web/tests/vite/production-sync-settings.test.ts
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/vite/production-sync-settings.test.ts#sync controls require
  advertised configured service and explicit refresh after failure
- web/tests/browser/local-sync.spec.ts#local sync controls and grouped progress
  preserve repository context
<!-- /evidence-field -->

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
cargo test --locked -p jj-store-cli --test native_lifecycle content_acceptance::http_acceptance::local_management::local_workspace_management_requires_local_admin_and_captures_contracts -- --exact
```

**Notes**

Local-only, pause/resume/retry, strict intent, LocalTrusted Admin, origin and bearer denial; actual response fixtures. Initial helper inherited wrong Result alias, corrected to std::result::Result.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/production-sync-settings.test.ts web/tests/vite/production-store-settings.test.ts web/tests/vite/production-store-workspaces.test.ts
```

**Notes**

17 tests; initial 2 failures were shared vi.fn mock aliasing, fixed independent implementations.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreSyncSettings.stories.svelte catalog/components/store-application/StoreApplication.stories.svelte
```

**Notes**

16 interaction/accessibility stories.
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
pnpm web:build; pnpm exec playwright test --config web/playwright.config.ts web/tests/browser/local-sync.spec.ts
```

**Notes**

Cold server and fresh bundle; pause/resume/retry, progress tabs, mobile overflow and axe. First attempt kept desktop back control hidden by mobile viewport; restored viewport before desktop return assertion.
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
pnpm contracts:generate; pnpm contracts:check
```

**Notes**

95 captured responses, 6 contract tests. Initial generation missed axum Json import; repaired.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
cargo clippy --locked -p jj-store-service -p jj-store-http -p jj-store-server --lib -- -D warnings
```
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
pnpm web:check; pnpm catalog:check; pnpm check:architecture; pnpm check:boundaries
```

**Notes**

Zero errors/warnings, 104 catalog modules. Fixed request-only SyncAction type, explicit fixture initial-prop capture and forbidden catalog API import. Extracted repository transports after source reached 303-line limit.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
cargo fmt -p jj-store-service -p jj-store-server -p jj-store-http -p jj-store-cli; pnpm exec prettier --write <changed TS/Svelte/Markdown>; pnpm spec:first; pnpm spec:validate; pnpm check:progress
```
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pass
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

[]
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Sync controls existed only through RPC/CLI; browser had no local Sync section.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

One configured-sync behavior and direct HTTP, server, controller, browser consumers; generated public contract lane included. Root aggregates and full unrelated suites out of scope.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Additive local_sync_management and V1 LocalSyncStatus/SyncCommand. No persistence or replication protocol change. Enclosing Jujutsu change is revision reference.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; JJ 0.45.1; pnpm 10.32.1; locked Svelte/Storybook/Vitest/Playwright.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Editable Search configuration and isolated worker lifecycle with Configuration/Health tabs.
<!-- /evidence-field -->

<!-- evidence-field:deferred list -->
## Deferred

- Finite-frontier wait UI and integrated persistent settings qualification
  remain pending.
- Visual baselines remain visual-pending.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


