---
schema: 1
id: E-M6-004
---

# E-M6-004

<!-- evidence-field:id markdown -->
## Id

E-M6-004
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

native-search-visibility
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/tests/native_lifecycle/adapter_projection.rs
- crates/jj-store-cli/tests/native_lifecycle/search_visibility.rs
- crates/jj-store-jj/src/query/content.rs
- crates/jj-store-jj/src/query/content/operations.rs
- crates/jj-store-service/src/reads.rs
- crates/jj-store-tantivy/src/lib.rs
- crates/jj-store-tantivy/src/search.rs
- crates/jj-store-tantivy/tests/durability.rs
- docs/evidence/m6-search-visibility.md
- spec/src/adapters.md
- spec/src/cli.md
- spec/src/compatibility.md
- spec/src/jj-integration.md
- spec/src/milestones.md
- spec/src/mutations.md
- spec/src/security.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/native_lifecycle/search_visibility.rs#native_tantivy_selected_view_filters_hidden_commits_before_pagination
- crates/jj-store-cli/tests/native_lifecycle/search_visibility.rs#native_search_visibility_excludes_predecessors_and_enforces_limits
- crates/jj-store-cli/tests/native_lifecycle/search_visibility.rs#native_tantivy_rejects_foreign_view_and_changed_epoch_filters
<!-- /evidence-field -->

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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle native_tantivy_selected_view_filters_hidden_commits_before_pagination --locked
```

**Notes**

Initial named target cannot compile: 26 diagnostics for absent visibility API and explicit query request types.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle native_tantivy_selected_view_filters_hidden_commits_before_pagination --locked
```

**Notes**

One fixture compiler error: ambiguous glob import of Limits. Explicit Tantivy imports fix it.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle search_visibility --locked
```

**Notes**

Initial two fixtures pass.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle search_visibility --locked
```

**Notes**

Expanded three fixtures pass with native all() oracle, actual token revocation, foreign repository/epoch filters and limit checks.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle native_tantivy_selected_view_filters_hidden_commits_before_pagination --locked
```

**Notes**

Exact original acceptance target passes after final changes, with native graph oracle and revocation.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle adapter_projection --locked
```

**Notes**

All seven native projection and visibility fixtures pass, zero failures.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-tantivy --test durability --locked
```

**Notes**

All ten real local Tantivy durability fixtures pass with explicit historical search mode.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo clippy -p jj-store-tantivy -p jj-store-jj -p jj-store-service --all-targets --locked -- -D warnings
```

**Notes**

Focused all-target Clippy passes with no suppressed warnings.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check
```

**Notes**

Normal checks pass, including rustfmt, all-target Clippy, ledger/spec, package boundaries, generated contracts and existing Svelte analysis.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test
```

**Notes**

330 Rust tests pass, zero failures; six opt-in external tests ignored. Governance, contracts, storage harness and existing browser unit lanes pass.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build
```

**Notes**

Locked Rust workspace and existing browser assets build successfully.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check
```

**Notes**

All 13 specification lanes pass, including existing cold browser interaction/accessibility, Rust tests, rustfmt, mdBook and spec-first.
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

- Implement server search lifecycle/configuration, HTTP generation cursors and
  freshness/minimum-position waits with generated contracts.
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

Core search requests explicitly select historical metadata or native-view commit visibility; results include the selected read snapshot. Local Tantivy schema remains 2. Native/WAL/receipt/adapter cursor and HTTP formats unchanged. Enclosing Jujutsu change is the revision reference; no UI changes.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


