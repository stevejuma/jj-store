---
schema: 1
id: E-M14-014
---

# E-M14-014 details 2

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 10
hasNote: true
noteEncoding: markdown

```

```shell
cargo test --locked -p jj-store-wal --test publication maintenance_reuses_captured_target_and_reports_covering_checkpoint (before)
```

**Notes**

Missing captured maintenance APIs; /tmp/jj-local-management-storage-before.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/production-storage-settings.test.ts (before)
```

**Notes**

Missing controller module; /tmp/jj-local-management-storage-ui-before.log.
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
cargo test --locked -p jj-store-wal --test publication maintenance_reuses_captured_target_and_reports_covering_checkpoint
```

**Notes**

1/1; target survives serialization, newer writes, lost manifest acknowledgement, coordinator reconstruction and covering checkpoint; bounded incomplete audit and digest corruption rejected.
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
cargo test --locked -p jj-store-wal
```

**Notes**

Full relevant WAL suite passes. Escalated because shared checkpoint/recovery primitives affect publication consumers. /tmp/jj-local-management-storage-wal-suite.log.
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
cargo check --locked -p jj-store-service
```

**Notes**

Pass.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
cargo check --locked -p jj-store-service -p jj-store-http -p jj-store-server (first)
```

**Notes**

Composition initially referenced the configuration enum principal instead of the resolved HTTP local identity; corrected.
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
cargo test --locked -p jj-store-cli --test native_lifecycle local_workspace_management_requires_local_admin_and_captures_contracts (initial Storage fixture)
```

**Notes**

Ambiguous Error import corrected; then GET fixture retained POST Content-Type and correctly received 400. Removed only the invalid header; no admission change.
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
cargo test --locked -p jj-store-cli --test native_lifecycle local_workspace_management_requires_local_admin_and_captures_contracts (final)
```

**Notes**

1/1; real native coordinator over deterministic storage. GET creates no journal, prepare changes no manifest, restart retains target, selected checkpoint preserves heads/position/receipts, integrity completes, budget exhaustion reports incomplete, interrupted check and cancellation are retained. Remote/read-only denied; arbitrary paths rejected. /tmp/jj-local-management-storage-http-final.log.
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
pnpm contracts:generate
```

**Notes**

Generated from registered endpoints.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 8
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:check (initial)
```

**Notes**

Request-only StorageCommand schema is inline; corrected frontend type to derive from generated endpoint request, rather than Model components.
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
pnpm web:check; pnpm catalog:check
```

**Notes**

0 diagnostics; 160 catalog modules pass.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/production-storage-settings.test.ts web/tests/vite/production-store-settings.test.ts web/tests/vite/production-sync-settings.test.ts
```

**Notes**

Initial 13/13; final added definite rejection and incomplete report assertions recorded below.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-application/StoreStorageSettings.stories.svelte (first)
```

**Notes**

Vite optimized the new shield-check icon during the run, reloaded tests and failed virtual CSS imports. Fresh process with completed dependency optimization passes.
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
pnpm test:storybook catalog/components/store-application/StoreStorageSettings.stories.svelte (final)
```

**Notes**

5/5 keyboard/interaction/accessibility stories pass; visual-pending.
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
pnpm check:architecture (initial)
```

**Notes**

301 line local-profile source exceeded 300; moved shared unavailable-native-management capability values to owning profile model.
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
pnpm check:architecture; pnpm check:ui-ownership; pnpm check:boundaries; pnpm spec:validate; pnpm spec:first
```

**Notes**

834 web modules; 16 package boundaries; canonical checks pass.
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
rustfmt --edition 2024 changed Rust files; pnpm exec prettier --write changed frontend/spec files
```

**Notes**

Targeted formatting.
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
cargo test --locked -p jj-store-wal (final timeout boundary)
```

**Notes**

90/90 pass after retaining end-to-end time bounds around initial reads and restored capture validation. /tmp/jj-local-management-storage-wal-final.log.
<!-- /run -->
<!-- /evidence-field -->

