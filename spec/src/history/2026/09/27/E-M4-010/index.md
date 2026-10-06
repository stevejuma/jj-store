---
schema: 1
id: E-M4-010
---

# E-M4-010

<!-- evidence-field:id markdown -->
## Id

E-M4-010
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M4
- M4.1
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-27
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

native-query-metadata-shape-admission
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/tests/native_lifecycle/content.rs
- crates/jj-store-cli/tests/native_lifecycle/content_admission.rs
- crates/jj-store-jj/src/query/admission.rs
- crates/jj-store-jj/src/query/access.rs
- crates/jj-store-jj/src/query/content.rs
- crates/jj-store-jj/src/query/mod.rs
- crates/jj-store-jj/examples/query_qualification.rs
- crates/jj-store-jj/examples/content_qualification.rs
- scripts/qualify-query-resources.py
- docs/evidence/m4-query-metadata-budget.json
- docs/evidence/m4-content-shape-budget.json
- docs/evidence/m4-query-admission-audit.md
- spec/src/api.md
- spec/src/cli.md
- spec/src/compatibility.md
- spec/src/jj-integration.md
- spec/src/spec-governance.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/native_lifecycle/content_admission.rs#query_shape_rejects_commit_merge_amplification_without_narrowing_persistence
- crates/jj-store-cli/tests/native_lifecycle/content_admission.rs#query_shape_rejects_reference_merge_amplification
- crates/jj-store-cli/tests/native_lifecycle/content_admission.rs#query_shape_rejects_dense_trees_before_native_expansion
- crates/jj-store-cli/tests/native_lifecycle/content_admission.rs#query_shape_node_admission_is_request_local_and_preserves_file_streaming
- crates/jj-store-jj/src/query/admission.rs#query_shape_bounds_every_persisted_reference_and_edge_collection
- crates/jj-store-jj/src/query/admission.rs#query_shape_node_limit_is_cumulative_and_counts_keys_nulls_and_empty_values
- crates/jj-store-jj/src/query/admission.rs#query_shape_rejects_corruption_depth_and_cancellation_without_collections
- crates/jj-store-jj/src/query/admission.rs#query_shape_recognizes_escaped_fields_and_rejects_trailing_input
- crates/jj-store-jj/examples/content_qualification.rs#main
- crates/jj-store-jj/examples/query_qualification.rs#main
<!-- /evidence-field -->

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle query_shape_
```

**Notes**

Before: all 3 regressions fail because oversized merges, reference conflicts and dense trees are accepted.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle query_shape_
```

**Notes**

After admission: original 3 regressions pass; published native objects still round-trip losslessly.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-jj query_shape_
```

**Notes**

One unit-fixture compile error: ObjectKey::from_parts needs the kind byte, not ObjectKind. Fixed explicit conversion.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-jj query_shape_
```

**Notes**

All 4 scanner cases pass.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo build --release --locked -p jj-store-jj --example query_qualification
```

**Notes**

Release graph fixture builds with explicit metadata limits/counters.
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
python3 scripts/qualify-query-resources.py --binary target/release/examples/query_qualification --commits 50000 --output .scratch/m4-shape-graph-budget.json
```

**Notes**

8 queries plus 4 negative admission cases; 5.88 seconds elapsed, 5.16 CPU seconds, 278675456 peak RSS bytes; at most 2221020 metadata nodes.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo build --release --locked -p jj-store-jj --example content_qualification
```

**Notes**

Release content fixture builds.
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
python3 scripts/qualify-query-resources.py --scenario content --binary target/release/examples/content_qualification --output .scratch/m4-shape-content-budget.json
```

**Notes**

8 content boundary cases pass; 7.70 seconds elapsed, 7.36 CPU seconds, 111181824 peak RSS bytes. Dense/merge pages, 256 MiB complete verification and bounded streams/ranges, negative shape/blob budgets.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle
```

**Notes**

60 native lifecycle cases passed before adding the request-local node/file-streaming case.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle query_shape_
```

**Notes**

All 4 final shape acceptance cases pass; original 3 regressions plus request-local node accounting/file streaming.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check
```

**Notes**

57 requirements, package boundaries, progress ledger, rustfmt and all-target Clippy pass.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test
```

**Notes**

198 Rust tests pass / 6 external cases intentionally ignored; includes all 61 native lifecycle cases. 15 governance and 3 storage-harness tests pass. No unresolved failures.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build
```

**Notes**

Locked workspace build passes.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check
```

**Notes**

All 9 lanes pass, including 198 Rust tests, rustfmt, mdBook and mapped spec-first checks.
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
<!-- generated-navigation -->
## Pages

- [E-M4-010 details 2](details-02.md)
