---
schema: 1
id: E-M6-003
---

# E-M6-003 details 2

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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-adapters --test delivery adapter_history_and_prior_inventory_stop_at_the_delivered_snapshot --locked
```

**Notes**

Before implementation: compile failure with four missing history/at method diagnostics.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle native_adapter_projection_reader_decodes_committed_trees_and_files --locked
```

**Notes**

Before implementation: missing ProjectionRead and inaccessible fixture; four compiler diagnostics. An intermediate filtered run after moving the fixture matched zero tests and is not acceptance evidence.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle native_adapter_projection_reader_decodes_committed_trees_and_files --locked
```

**Notes**

Named reader fixture passes after placement under content_acceptance; committed trees/files, virtual roots and authorization bridge.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle native_tantivy_extracts_committed_metadata_and_explicit_source_scopes --locked
```

**Notes**

Before implementation: native extractor module absent.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle native_tantivy_extracts_committed_metadata_and_explicit_source_scopes --locked
```

**Notes**

One runtime failure: native tree diff rejects inconsistent copy history in the codec fixture. Diagnostic rerun confirms the same failure. The shared codec fixture remains unchanged; search publishes a valid nested copy lineage.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle native_tantivy_extracts_committed_metadata_and_explicit_source_scopes --locked
```

**Notes**

Valid fixture helper initially failed compilation: nonexistent Tree::set and collection type mismatch; two diagnostics. Fixed sorted tree entry construction.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle native_tantivy_extracts_committed_metadata_and_explicit_source_scopes --locked
```

**Notes**

Real native metadata, invisible predecessor, actor and explicit source scopes pass with the valid search fixture.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle native_tantivy_diffs_replay_equivalently_and_keep_exact_scope_context --locked
```

**Notes**

Exact operation-ID query returned one file instead of two after a scope update: unchanged file terms retained stale context.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle native_tantivy_diffs_replay_equivalently_and_keep_exact_scope_context --locked
```

**Notes**

Schema 2 derives file context and exact commit/operation-ID scope matches from the pinned payload. The original two-file assertion passes; incremental hits equal rebuild.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle native_tantivy --locked
```

**Notes**

Two tests passed; multi-head fixture failed because its sibling publication provided an empty removed-head set. Service correctly returned native transition heads.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle native_tantivy_clears_ambiguous_scopes_and_respects_source_disable --locked
```

**Notes**

Named multi-head fixture passes after using the original parent operation as removed head; no service guard relaxation.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-adapters --test delivery --locked
```

**Notes**

All ten delivery fixtures pass.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-tantivy --test durability --locked
```

**Notes**

All ten real local Tantivy durability fixtures pass.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle native_adapter_projection_reader_decodes_committed_trees_and_files --locked
```

**Notes**

Original named native read bridge target passes after final changes.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-adapters --test delivery adapter_history_and_prior_inventory_stop_at_the_delivered_snapshot --locked
```

**Notes**

Original selected-history boundary target passes after final changes.
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle native_tantivy --locked
```

**Notes**

All three native extraction/replay/context/multi-head fixtures pass, zero failures.
<!-- /run -->

<!-- run -->
### Run 17

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

Normal checks pass, including all-target Clippy, rustfmt, package boundaries, ledger/spec, generated contracts and existing Svelte analysis.
<!-- /run -->

<!-- run -->
### Run 18

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

327 Rust tests pass, zero failures; six opt-in external tests ignored. Governance, contracts, storage harness and existing browser unit lanes pass.
<!-- /run -->
<!-- /evidence-field -->

