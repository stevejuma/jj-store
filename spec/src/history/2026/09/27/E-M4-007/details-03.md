---
schema: 1
id: E-M4-007
---

# E-M4-007 details 3

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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle conflict_descriptor_
```

**Notes**

All 3 cases pass: native term/label parity, typed editor capabilities, 5-term merges, fingerprint binding, limits/revocation and verified content failure.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-jj conflict_fingerprint_v1_matches_independent_canonical_vector
```

**Notes**

1 golden case passes against independent Python canonical JSON/hashlib BLAKE2b digest.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-jj --test native_backend
```

**Notes**

All 12 backend cases pass after shared copy-query changes.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle content_read_
```

**Notes**

All 7 existing content-read cases pass after shared native helper extraction.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check
```

**Notes**

57 requirements, boundaries, ledger, rustfmt and all-target Clippy pass.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test
```

**Notes**

178 Rust tests pass / 6 external cases intentionally ignored. 15 governance and 3 storage-harness tests pass.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build
```

**Notes**

Locked workspace build passes.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check
```

**Notes**

All 9 lanes pass, including 178 Rust tests, rustfmt, mdBook and mapped spec-first checks.
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

- Production query CPU/memory/index/cache qualification, HTTP JSON/media/SSE,
  generated contracts and browser integration remain M4 work. Typed conflict
  resolution is M5.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.97.1; jj-cli/jj-lib 0.45.1 at 7c41cdeb16b6b321c64e789a966b6adf723816a5. Native production reads over disposable in-memory storage; no deployment or performance claim.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

V1 structural diff DTO retains raw target terms separately from complete source annotations and optional native line hunks. V1 conflict descriptor carries ordered labeled terms, explicit editor support and a domain-separated ConflictFingerprint. No native object, authority or RPC persistence format change.
<!-- /evidence-field -->

