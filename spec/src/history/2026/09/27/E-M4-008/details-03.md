---
schema: 1
id: E-M4-008
---

# E-M4-008 details 3

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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build
```

**Notes**

Locked workspace build passes.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check
```

**Notes**

All 9 lanes pass, including 185 Rust tests, rustfmt, mdBook and mapped spec-first checks.
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

- Production CPU/memory/transient-build resource qualification and long rebuild
  task/status, then HTTP JSON/media/SSE, generated contracts and browser
  integration.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.97.1; jj-cli/jj-lib 0.45.1 at 7c41cdeb16b6b321c64e789a966b6adf723816a5. Native queries over disposable in-memory stores; no deployment/performance claim.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Local query-cache-v1 slots replace unreleased per-operation paths under the pinned revision/profile. Receipts bind repository/epoch/operation and checksummed file inventories. Additive V1 graph cache diagnostics report reuse/build/rebuild, evictions and retained bytes/counts. No authority, native object or native RPC format change.
<!-- /evidence-field -->

