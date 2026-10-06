---
schema: 1
id: E-M6-005
---

# E-M6-005 details 3

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
pnpm check:progress && pnpm spec:validate && pnpm spec:build && pnpm spec:first
```

**Notes**

Final evidence checks pass after retaining pending milestone scope; 58 requirements and 13-lane product validation recorded above.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Register HTTP search/status routes and generated contracts; integrate
  configured server runners and lifecycle/diagnostics.
- Implement immutable remote index snapshots/restore and measured cold
  process/deployment indexing/rebuild acceptance. RustFS/TLS qualification
  remains externally blocked; UI remains deferred.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.97.1 with rustfmt; jj-cli/jj-lib 0.45.1 at 7c41cdeb16b6b321c64e789a966b6adf723816a5; jj-native-0.45.1-v1; Tantivy 0.26.2 at 72d1ef9a6468aa68bbc69dcc80cdf60aaf64364d.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

New in-memory search serving request/status/page contracts and ephemeral jss1 HMAC cursor profile. Canonical native/WAL/receipt/adapter cursor, local Tantivy schema 2 and registered HTTP formats remain unchanged. Enclosing Jujutsu change is the revision reference; no UI changes.
<!-- /evidence-field -->

