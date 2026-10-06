---
schema: 1
id: E-M4-014
---

# E-M4-014 details 3

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
pnpm format:check
```

**Notes**

Rustfmt and Prettier pass.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build
```

**Notes**

Locked workspace build passes after generated-cache cleanup.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check
```

**Notes**

All 9 lanes pass, including 226 Rust tests / 6 external ignored, governance, rustfmt, mdBook and mapped spec-first checks. M4.1 read gate is complete; M4.2 SSE/generated contracts/browser, M5, M6 and M8a remain separately unfinished.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pass
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

[]
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.97.1; jj-cli/jj-lib 0.45.1 at 7c41cdeb16b6b321c64e789a966b6adf723816a5; Axum 0.8.9, tonic 0.14.6, tokio-rustls 0.26.5; locked serde_path_to_error 0.1.20.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Additive HTTP V1 authenticated JSON/media/index-task routes and stable error envelopes, one-time verified spool range selection; dual HTTP/1.1 and h2 ALPN. Native persistence, publication and RPC wire formats unchanged. SSE, mutations, search and UI are not advertised.
<!-- /evidence-field -->

