---
schema: 1
id: E-REUSABLE-AGENTS-003
---

# E-REUSABLE-AGENTS-003 details 2

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
node --test scripts/capture-features.test.mjs
```

**Notes**

Initial tool setup attempted unsupported managed Cargo tree. Corrected read-only graph inspection to Cargo directly; no build/runtime command bypass.
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
CARGO_INCREMENTAL=0 pnpm dev:cargo -- check -p jj-store-capture --no-default-features --locked
```

**Notes**

Headless compilation succeeds. Initial unused feature-specific helper imports were subsequently gated and strict lint verifies closure.
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
node --test scripts/capture-features.test.mjs
```

**Notes**

Headless normal graph excludes WAL/object/native/service/RPC/upstream JJ/runtime/Tauri.
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
CARGO_INCREMENTAL=0 pnpm dev:cargo -- test -p jj-store-capture --no-default-features --lib --locked
```

**Notes**

11 private source/lease/identity cases pass.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
CARGO_INCREMENTAL=0 pnpm dev:cargo -- test -p jj-store-capture --no-default-features --test headless_capture --locked
```

**Notes**

New fixture initially used32bytes for the existing16byte CaptureBatchId. Corrected the fixture; production identities unchanged.
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
CARGO_INCREMENTAL=0 pnpm dev:cargo -- test -p jj-store-capture --no-default-features --locked
```

**Notes**

Existing OpenCode recovery fixture assumed transitive serde_json preserve_order. Changed expected bytes to the actual original persisted catalog, strengthening exact-byte restoration independently of JSON Value map iteration.
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
CARGO_INCREMENTAL=0 pnpm dev:cargo -- test -p jj-store-capture --no-default-features --test opencode_recovery_index malformed_or_unbound_index_withholds_registry_and_legacy_empty_index_is_readable --locked
```

**Notes**

Exact formerly failing headless recovery regression passes.
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
CARGO_INCREMENTAL=0 pnpm dev:cargo -- test -p jj-store-capture --no-default-features --locked
```

**Notes**

Complete headless profile:67printed passing executions including one fresh-process child; one parent-invoked child ignored in discovery.
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
CARGO_INCREMENTAL=0 pnpm dev:cargo -- test -p jj-store-capture --locked
```

**Notes**

Complete original default application boundary:187printed passing executions including one child; three parent-invoked children ignored. New headless case additionally runs under default in focused consumer verification.
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
CARGO_INCREMENTAL=0 pnpm dev:cargo -- clippy -p jj-store-capture --all-targets --locked -- -D warnings
```

**Notes**

Strict default application lint passes.
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
CARGO_INCREMENTAL=0 pnpm dev:cargo -- clippy -p jj-store-capture --no-default-features --all-targets --locked -- -D warnings
```

**Notes**

Strict complete headless target lint passes.
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
pnpm test:governance
```

**Notes**

75 normal-governance cases pass, including the new acquisition and feature-closure checks.
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
node scripts/package-check.mjs; node scripts/check-boundaries.mjs
```

**Notes**

24 Cargo package metadata/input and23 dependency boundaries pass.
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
CARGO_INCREMENTAL=0 pnpm dev:cargo -- fmt -p jj-store-capture -- --check
```

**Notes**

Affected Rust formatting passes.
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
pnpm dev:disk
```

**Notes**

43.1GiBfree; no cleanup, fixture reset or service restart required.
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
CARGO_INCREMENTAL=0 pnpm dev:cargo -- test -p jj-store --test agent_capture_spool --locked
```

**Notes**

All9CLI durable spool and prepared-binding cases pass.
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
CARGO_INCREMENTAL=0 pnpm dev:cargo -- check -p jj-store-server --lib --locked
```

**Notes**

Direct server consumer compiles with default application feature and unchanged adapter APIs.
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
CARGO_INCREMENTAL=0 pnpm dev:cargo -- test -p jj-store-capture --test headless_capture --test opencode_recovery_index --locked
```

**Notes**

New headless reader case and3OpenCode index cases pass under default feature; recovery checks exact original bytes.
<!-- /run -->
<!-- /evidence-field -->

