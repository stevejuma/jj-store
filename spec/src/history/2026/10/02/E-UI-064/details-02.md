---
schema: 1
id: E-UI-064
---

# E-UI-064 details 2

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
cargo check -p jj-store-service -p jj-collab -p jj-store-rpc
```

**Notes**

Initial affected Rust check; source/domain changes compile.
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
cargo test -p jj-collab --test target_resolution --locked
```

**Notes**

10/10, exact target versions/identity/task/tombstone and existing Thread JSON.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-service --test native_collab --test local_collab --test collab_pagination --locked
```

**Notes**

New test fixture used private error path and wrong receipt field; corrected to the public mutation Error and actor. No production workaround.
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
cargo test -p jj-store-service --test native_collab --test local_collab --test collab_pagination --locked
```

**Notes**

35/35: native10, local10, pagination15. Includes Work versions, visibility, pinning, home write admission, receipt replay, cold reopen/GitGC and clear without changing Work selection.
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
pnpm contracts:generate
```

**Notes**

OpenAPI and TypeScript generated; no handwritten type shim.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:check
```

**Notes**

Started before generated contract completed; work_comment type absent in old DTO.
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
pnpm web:check
```

**Notes**

After generated contracts: 0 errors/warnings.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/work-reactions.test.ts web/tests/vite/reaction-feedback.test.ts web/tests/vite/production-work.test.ts web/tests/vite/automatic-work-refresh.test.ts
```

**Notes**

15/15. Extended direct lifecycle consumer run with production-automatic-refresh.test.ts passed18/18 after lazy disconnected transport construction.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm catalog:check
```

**Notes**

New story used Testing Library unsupported exact option; replaced with exact name regex.
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
pnpm catalog:check
```

**Notes**

0 errors/warnings, catalog129 modules.
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
pnpm check:architecture
```

**Notes**

Task Activity exceeded300 counted lines; extracted native per-comment adapter, preserving callbacks/drafts.
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

737 modules after extraction.
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
pnpm web:check
```

**Notes**

Extracted leaf imported Model from work-types; fixed to public API type import.
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
pnpm dev:backend:build; JJ_STORE_DEV_ROOT=/Users/stevejuma/code/lapismd/jj-store/.local/dev-demo pnpm dev:backend:restart
```

**Notes**

Explicit development binaries; existing fixture preserved, Vite remains running.
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
pnpm install --no-frozen-lockfile; node /tmp/jj-auto-refresh-restart-ui.mjs
```

**Notes**

Existing temporary private source validation. Restored tracked registry lock exactly; UI oldPID52247/newPID88268, backend36217 initially preserved before authorized backend rebuild/restart.
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
pnpm web:build
```

**Notes**

Cold built bundle prepared; normal chunk-size advisory only.
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
cargo clippy -p jj-collab -p jj-store-wal -p jj-store-service -p jj-store-rpc --all-targets --locked -- -D warnings
```

**Notes**

Inherited redundant local in jj-store-jj/local_browser/operations.rs:202. No change there.
<!-- /run -->

<!-- run -->
### Run 18

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
cargo clippy -p jj-collab -p jj-store-wal -p jj-store-service -p jj-store-rpc --all-targets --no-deps --locked -- -D warnings
```

**Notes**

Inherited collapsible_if in unchanged materialization cache eviction and content_search/extract.rs, and items_after_test_module in content_search/cursors.rs. Production/new test compile and focused service tests pass; full lint lane remains unresolved.
<!-- /run -->
<!-- /evidence-field -->

