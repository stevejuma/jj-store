---
schema: 1
id: E-M6-003
---

# E-M6-003 details 3

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

Locked workspace and existing browser assets build successfully.
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

All 13 lanes pass, including existing cold browser interaction/accessibility, Rust tests, rustfmt, mdBook and spec-first.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm format:check
```

**Notes**

Rustfmt and repository Prettier checks pass.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Implement current-visible versus historical search, server lifecycle and
  configuration, HTTP signed generation cursors, freshness/minimum-position
  waits and generated API contracts.
- Implement immutable remote snapshots and restore; qualify cold
  process/deployment and measured indexing/rebuild performance. Real RustFS/TLS
  remains externally blocked; UI stays deferred.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.97.1 with rustfmt; jj-cli/jj-lib 0.45.1 at 7c41cdeb16b6b321c64e789a966b6adf723816a5; jj-native-0.45.1-v1; Tantivy 0.26.2 at 72d1ef9a6468aa68bbc69dcc80cdf60aaf64364d.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Disposable local Tantivy schema advances to 2: unchanged file context is bound to scope payload and exact-ID scope terms. Schema-1 generations rebuild. Native extraction config identity is domain-separated native-extraction-v1. Native/WAL/receipt/adapter cursor and HTTP formats unchanged. Enclosing Jujutsu change is the revision reference; no UI changes.
<!-- /evidence-field -->

