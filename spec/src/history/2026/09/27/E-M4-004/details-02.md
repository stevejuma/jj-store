---
schema: 1
id: E-M4-004
---

# E-M4-004 details 2

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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle content_read_
```

**Notes**

Before implementation: zero matching cases, not acceptance evidence.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo check -p jj-store-cli
```

**Notes**

Compile error: ConflictLabels does not implement From&lt;Merge&lt;String&gt;&gt;. Used the pinned Commit::tree() facade; removed one obsolete Deserialize import warning.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo check -p jj-store-cli
```

**Notes**

Compile error: native merged value borrowed the current tree across reassignment. Dropped the inspected value before loading the next directory.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle content_read_
```

**Notes**

Fixture compilation produced 2 type-inference diagnostics for redundant Into on a generic path constructor; no cases ran. Imports reserved for expanded fixtures initially emitted 3 warnings.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle content_read_
```

**Notes**

All 3 fixtures were denied publication because they named protected root as a legacy predecessor. The fixture now stages a real invisible predecessor; production root protection is unchanged.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle content_read_
```

**Notes**

2 cases pass / 1 path-validation failure: upstream internal RepoPath permits parent components as object names. Application traversal now explicitly rejects dot, parent and NUL components without normalization.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle content_read_
```

**Notes**

Expanded 6 cases pass: all native metadata and roots, binary ranges and permits, native five-term merge/deletion/directory semantics, staged/later-publication rejection, limits, deadlines, current revocation, corruption beyond the requested range and missing committed history.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle content_read_
```

**Notes**

All 7 cases pass including cancellation after chunk I/O but before final authorization. Reusing a canceled stream fails explicitly rather than skipping bytes.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-object --test contract private_spool_
```

**Notes**

Before range fixture: zero matching cases, not acceptance evidence.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-object --test contract private_spool_
```

**Notes**

1 range fixture passes, covering first/last bytes, 64 KiB crossings, whole file, exact EOF and invalid ranges.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-jj --test native_codec
```

**Notes**

All 8 codec fixtures pass, including frozen V1 envelopes and upstream identities.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check
```

**Notes**

57 requirements, boundaries, progress, rustfmt and all-target Clippy pass.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test
```

**Notes**

158 Rust tests pass; 6 external acceptance cases intentionally ignored by the default lane. 15 governance and 3 storage-harness tests pass.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build
```
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check
```

**Notes**

All 9 lanes pass, including rustfmt, repeated Rust tests, mdBook and mapped spec-first checks.
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm format:check
```
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- M4 retains signed directory continuations, structured diffs/conflict
  descriptors and operation diffs, HTTP JSON/media/SSE, generated contracts and
  browser integration.
- Production query CPU/memory/index/cache qualification and cold rebuild
  task/status remain open. Current content bounds are library acceptance
  evidence, not a deployment performance claim.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.97.1; jj-cli/jj-lib 0.45.1 / 7c41cdeb16b6b321c64e789a966b6adf723816a5; production native/service code over disposable in-memory storage, not RustFS conformance.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Owned native metadata V1 schemas exported without changing persisted field order, payload bytes or native identity. Object wrappers carry version/profile/key; directory pages expose native terms and internal name continuations. Files use complete native/envelope verification into bounded private spools and authorized 64 KiB chunks; ranges seek within verified spools. No authority or native RPC format change.
<!-- /evidence-field -->

