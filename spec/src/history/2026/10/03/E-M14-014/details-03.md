---
schema: 1
id: E-M14-014
---

# E-M14-014 details 3

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
cargo check --locked -p jj-store-service -p jj-store-http -p jj-store-server (final)
```

**Notes**

Affected boundary compilation passes.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/production-storage-settings.test.ts web/tests/vite/production-store-settings.test.ts web/tests/vite/production-sync-settings.test.ts (final)
```

**Notes**

14/14 pass.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/local-browser.test.ts web/tests/vite/history-query-transport.test.ts
```

**Notes**

19/19 including explicit native-maintenance denial for the filesystem-only profile.
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
cargo test --locked -p jj-store-wal --test publication maintenance_reuses_captured_target_and_reports_covering_checkpoint (lost-write extension)
```

**Notes**

1/1; before-write unknown remains not covered; reconstructed coordinator retries the same target. After-write acknowledgement loss resolves exact selection.
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
cargo test --locked -p jj-store-cli --test native_lifecycle local_workspace_management_requires_local_admin_and_captures_contracts (service worker bound)
```

**Notes**

1/1 after bounding service-owned maintenance to one running worker across repositories.
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
pnpm contracts:check
```

**Notes**

159 actual HTTP/SSE responses conform, generated files match, 7 contract tests and contract type check pass.
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
pnpm web:build; pnpm exec playwright test --config web/playwright.config.ts web/tests/browser/local-storage.spec.ts (first)
```

**Notes**

Built UI succeeded with existing chunk-size warning. Preview server startup hit the 30-second Cargo/build-lock deadline; no browser assertions ran.
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
cargo build --locked -p jj-store-http --example browser_preview; pnpm exec playwright test --config .scratch/storage.playwright.config.ts web/tests/browser/local-storage.spec.ts
```

**Notes**

1/1 fresh production bundle/server; equivalent ignored config launches the prebuilt preview directly. Lost reply recovers exact target after newer overview position, Settings navigation retains intent, mobile no overflow and axe pass.
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
pnpm exec prettier --check changed Storage frontend/test files; pnpm check:progress
```

**Notes**

Formatting and 125-task progress bookkeeping pass.
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
JJ_STORE_DEV_PORT=7570 JJ_STORE_DEV_RUSTFS_RUNTIME=/Users/stevejuma/code/lapismd/jj-store/runtime/embedded-rustfs/target/debug/jj-store-rustfs JJ_STORE_REAL_TEST=web/tests/real/storage-maintenance.spec.ts pnpm web:test:real
```

**Notes**

Full demo checkpoint target survived real service/RustFS restart, selected correctly and preserved heads/position. Integrity correctly reported incomplete at the configured 30-second bound for position 99; the positive-path test incorrectly expected this larger fixture to finish. Dedicated bounded fixture retains the strict verified-data assertion. Cold 1254 ms, restart 5207 ms, 177840 KiB RSS; fixture cleaned up. /tmp/jj-local-management-storage-real.log.
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
JJ_STORE_DEV_PORT=7570 JJ_STORE_DEV_RUSTFS_RUNTIME=/Users/stevejuma/code/lapismd/jj-store/runtime/embedded-rustfs/target/debug/jj-store-rustfs node scripts/storage-browser-acceptance.mjs (initial and diagnostic retry)
```

**Notes**

Both failed before browser assertions: macOS temporary-directory socket exceeded SUN_LEN. Added bounded server-log diagnostics before cleanup, then shortened the isolated temporary path. /tmp/jj-local-management-storage-focused-real.log and /tmp/jj-local-management-storage-focused-real-diagnostic.log.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_DEV_PORT=7570 JJ_STORE_DEV_RUSTFS_RUNTIME=/Users/stevejuma/code/lapismd/jj-store/runtime/embedded-rustfs/target/debug/jj-store-rustfs node scripts/storage-browser-acceptance.mjs (short-path retry)
```

**Notes**

Server started and native repository initialized. Repository registration correctly rejected the running service owner lock; harness now registers after stopping its owned service. /tmp/jj-local-management-storage-focused-real-final.log.
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
pnpm check:architecture (repository-context guard and first extraction)
```

**Notes**

Central controller reached 303 then 301 counted lines. Moved route/context derivation to the owning Settings factory without relaxing the 300-line budget.
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
pnpm exec prettier --write web/src/lib/controller/production-store-index.ts web/src/lib/controller/production-store-controller.svelte.ts scripts/storage-browser-acceptance.mjs web/tests/real/storage-maintenance.spec.ts; pnpm check:architecture; pnpm web:check
```

**Notes**

Final route extraction: 835 modules; 0 Svelte errors/warnings. Storage unavailable at Projects hub; active native repository context required.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/production-storage-settings.test.ts web/tests/vite/production-store-settings.test.ts web/tests/vite/production-sync-settings.test.ts (context guard)
```

**Notes**

15/15 before route factory extraction; /tmp/jj-local-management-storage-context-unit.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/production-storage-settings.test.ts web/tests/vite/production-store-settings.test.ts web/tests/vite/production-sync-settings.test.ts web/tests/vite/production-store-workspaces.test.ts
```

**Notes**

23/23 with Settings route factory extraction and repository-context regression. /tmp/jj-local-management-storage-unit-context-final.log.
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
pnpm exec playwright test --config .scratch/storage.playwright.config.ts web/tests/browser/local-storage.spec.ts (context guard)
```

**Notes**

1/1 cold built UI/preview; lost reply recovery, captured target, mobile/axe and Projects-hub unavailable assertion. /tmp/jj-local-management-storage-browser-context-final.log.
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
pnpm catalog:check; pnpm check:ui-ownership
```

**Notes**

0 diagnostics; 160 catalog modules; UI ownership validator passes.
<!-- /run -->
<!-- /evidence-field -->

