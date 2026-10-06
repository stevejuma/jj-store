---
schema: 1
id: E-M6-008
---

# E-M6-008 details 3

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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-adapters --test delivery --locked
```

**Notes**

All eleven delivery/recovery/shutdown fixtures pass.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-tantivy --test durability --locked
```

**Notes**

All ten local Tantivy durability fixtures pass.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check
```

**Notes**

All normal checks pass, including rustfmt, Clippy, boundaries, progress, generated contracts and existing Svelte checks.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test
```

**Notes**

All normal tests pass: 357 Rust tests, zero failures, six external fixtures ignored; governance, storage harness, contracts and existing client unit tests pass.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build
```

**Notes**

Workspace binaries and existing browser assets build successfully.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check
```

**Notes**

All 13 specification lanes pass, including existing cold browser interactions/accessibility, contracts, rustfmt, governance, native tests, mdBook and mapped spec-first.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm format:check
```

**Notes**

Rustfmt and all matched Prettier files pass.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Implement immutable remote index snapshots/restore and measured
  indexing/rebuild performance.
- Qualify real RustFS/TLS server search and SSE. The original Docker
  volume-create harness is still live and stalled; UI remains deferred.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.97.1 with rustfmt; jj-cli/jj-lib 0.45.1 at 7c41cdeb16b6b321c64e789a966b6adf723816a5; jj-native-0.45.1-v1; Tantivy 0.26.2 at 72d1ef9a6468aa68bbc69dcc80cdf60aaf64364d.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

TOML format 1 adds explicit search entries; doctor JSON adds structured live search diagnosis. Owned adapter API adds idle drain; search status treats closed runner channels as degraded. No native/WAL/receipt/adapter cursor/local index schema 2 or registered HTTP format changes. Enclosing Jujutsu change is the revision reference.
<!-- /evidence-field -->

