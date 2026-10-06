---
schema: 1
id: E-M6-002
---

# E-M6-002 details 2

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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-tantivy --test durability tantivy_recovers_committed_payload_without_repeating_effects --locked
```

**Notes**

Named initial target cannot run: Tantivy crate does not exist.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo check -p jj-store-tantivy
```

**Notes**

Initial product compile target: four diagnostics for Tantivy 0.26 collector API and extractor Sync bounds.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-tantivy --test durability tantivy_recovers_committed_payload_without_repeating_effects --locked
```

**Notes**

One remaining product compile diagnostic: TopDocs requires order_by_score in Tantivy 0.26.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-tantivy --test durability tantivy_recovers_committed_payload_without_repeating_effects --locked
```

**Notes**

Fixture compile target: ambiguous Checkpoint import, three Arc inference errors and async closure borrow; compiler reports six diagnostics.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-tantivy --test durability tantivy_recovers_committed_payload_without_repeating_effects --locked
```

**Notes**

Original durable-pointer/reload/restart acceptance passes with real Tantivy files and recovered commit payload.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-tantivy --test durability --locked
```

**Notes**

Initial six durability, loss/config rebuild, scope, reader, authorization and limit tests pass.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo clippy -p jj-store-tantivy --all-targets --locked -- -D warnings
```

**Notes**

One too-many-arguments lint in private commit helper; grouped commit inputs without suppressing the lint.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-tantivy --test durability --locked
```

**Notes**

Expanded ten fixtures pass, including before/after remote cursor write, restored older local generation, retained-generation budget and post-query revocation.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo clippy -p jj-store-tantivy --all-targets --locked -- -D warnings
```

**Notes**

Focused all-target Clippy passes after input grouping and literal bounded query implementation.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-tantivy --test durability --locked
```

**Notes**

All ten final fixtures pass after streamed file hashing/copy, complete output bounds and retained-generation limits.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo clippy -p jj-store-tantivy --all-targets --locked -- -D warnings
```

**Notes**

Final focused all-target Clippy passes without suppressions.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-tantivy --test durability tantivy_recovers_committed_payload_without_repeating_effects --locked
```

**Notes**

Original target rerun with the exact locked command passes.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check
```

**Notes**

Normal checks pass, including all-target Clippy, rustfmt, eleven package boundaries, ledger/spec, generated contracts and existing Svelte analysis.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test
```

**Notes**

322 Rust tests pass, zero failures; six external deployment tests remain explicitly ignored. Governance, storage harness, contracts and existing browser unit checks pass.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build
```

**Notes**

Locked Rust workspace and existing browser assets build successfully.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check
```

**Notes**

All 13 specification lanes pass, including existing cold browser interaction/accessibility, full Rust tests, rustfmt, mdBook and spec-first.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm format:check
```

**Notes**

Rustfmt and all repository Prettier checks pass.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Implement native extraction and configured source scopes, native view
  visibility filtering, server/HTTP search and freshness/minimum-position waits,
  generated contracts and immutable remote index snapshots.
- Run cold process/deployment and measured search acceptance; UI remains
  deferred and real RustFS/TLS qualification remains externally blocked.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.97.1 with rustfmt; jj-cli/jj-lib 0.45.1 at 7c41cdeb16b6b321c64e789a966b6adf723816a5; Tantivy 0.26.2 with mmap/LZ4 locked from crates.io. Tantivy source revision from released .cargo_vcs_info.json: 72d1ef9a6468aa68bbc69dcc80cdf60aaf64364d.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

New disposable local Tantivy schema V1, canonical commit payload and checksummed generation descriptor plus atomic local active pointer. Uses existing adapter checkpoint V1; native/WAL/RPC/HTTP formats unchanged. Enclosing Jujutsu change is the revision reference; no UI changes.
<!-- /evidence-field -->

