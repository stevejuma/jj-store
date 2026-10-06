---
schema: 1
id: E-M4-011
---

# E-M4-011 details 3

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
python3 scripts/rustfs-contract.py --run --rust-client --server --output .scratch/m4-native-reader-rustfs.json
```

**Notes**

All 11 real RustFS harness cases pass after the native returned-reader runtime fix, including native transport, cold publication recovery and TLS server/CLI restart/revocation.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check
```

**Notes**

57 requirements, package boundaries, progress, rustfmt and all-target Clippy pass.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test
```

**Notes**

206 Rust tests pass / 6 external cases intentionally ignored. All 68 native lifecycle cases, 15 governance tests and 3 storage-harness self-tests pass; no unresolved failures.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build
```

**Notes**

Locked workspace build passes.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check
```

**Notes**

All 9 lanes pass, including 206 Rust tests, rustfmt, mdBook and mapped spec-first checks.
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
pnpm format:check
```

**Notes**

Rustfmt and Prettier pass.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Shared worker composition and controlled long cold-index tasks/status, then
  HTTP JSON/media/SSE, generated contracts and browser integration. Whole-server
  operational qualification remains M8a.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.97.1; jj-cli/jj-lib 0.45.1 at 7c41cdeb16b6b321c64e789a966b6adf723816a5. Predicate ancestry accounting mirrors the pinned recursive merge ordering without constructing merges; admitted evaluation remains upstream.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Additive owned graph predicate limits and file-byte/line statistics. Optional native authority tree-path admission defaults to existing session permission. No native identity, envelope, publication or RPC format changes.
<!-- /evidence-field -->

