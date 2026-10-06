---
schema: 1
id: E-M6-007
---

# E-M6-007

<!-- evidence-field:id markdown -->
## Id

E-M6-007
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M6
- M6.2
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-27
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

search-config-recovery
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-adapters/src/runner.rs
- crates/jj-store-cli/tests/native_lifecycle/adapter_projection.rs
- crates/jj-store-cli/tests/native_lifecycle/search_identity.rs
- crates/jj-store-tantivy/src/native.rs
- crates/jj-store-tantivy/src/projection.rs
- docs/evidence/m6-search-identity.md
- spec/src/adapters.md
- spec/src/cli.md
- spec/src/compatibility.md
- spec/src/milestones.md
- spec/src/security.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/native_lifecycle/search_identity.rs#native_search_rejects_mislabeled_extractor_configuration_before_recovery
- crates/jj-store-cli/tests/native_lifecycle/search_identity.rs#native_search_config_change_hides_incompatible_recovery_until_rebuild
<!-- /evidence-field -->

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle search_identity --locked
```

**Notes**

Both regressions fail: mislabeled native extractor accepted and incompatible recovered generation served after failed rebuild.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle search_identity --locked
```

**Notes**

Both original regressions pass after early identity validation and recovery filtering.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle search_identity --locked
```

**Notes**

Both final regressions pass; mismatched config, implementation, schema and name are rejected before ownership.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle adapter_projection --locked
```

**Notes**

All 22 native projection/search/HTTP/recovery fixtures pass.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-adapters --test delivery --locked
```

**Notes**

All ten adapter delivery/recovery fixtures pass.
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

All ten local Tantivy durability fixtures pass.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check
```

**Notes**

Normal checks pass, including rustfmt, all-target Clippy, contract drift/capture, boundaries, progress and existing Svelte checks.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test
```

**Notes**

346 Rust tests passed, zero failures, six external tests ignored; governance, harness, contract and existing web unit lanes pass.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build
```

**Notes**

Rust workspace and existing browser bundle build successfully.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check
```

**Notes**

All 13 specification lanes pass, including existing cold browser interaction/accessibility, Rust tests, mdBook, rustfmt and spec-first.
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

- Configure server search runners, credentials, lifecycle and diagnostics.
- Implement immutable remote index snapshots/restore and measured cold
  deployment evidence. Real RustFS/TLS qualification remains blocked externally;
  UI stays deferred.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.97.1 with rustfmt; jj-cli/jj-lib 0.45.1 at 7c41cdeb16b6b321c64e789a966b6adf723816a5; jj-native-0.45.1-v1; Tantivy 0.26.2 at 72d1ef9a6468aa68bbc69dcc80cdf60aaf64364d.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Projection/Extractor owned interfaces add identity validation. No native, WAL, receipt, adapter checkpoint, local index schema 2 or HTTP format changes. Enclosing Jujutsu change is the revision reference.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


