---
schema: 1
id: E-M4-006
---

# E-M4-006

<!-- evidence-field:id markdown -->
## Id

E-M4-006
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

native-operation-view-and-visibility-differences
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-jj/src/query/content/operations.rs
- crates/jj-store-jj/src/query/content.rs
- crates/jj-store-service/src/reads.rs
- crates/jj-store-cli/tests/native_lifecycle/operation_diff.rs
- crates/jj-store-cli/tests/native_lifecycle/content.rs
- spec/src/api.md
- spec/src/jj-integration.md
- spec/src/cli.md
- spec/src/mutations.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/native_lifecycle/operation_diff.rs#operation_diff_preserves_every_view_field_and_excludes_hidden_predecessors
- crates/jj-store-cli/tests/native_lifecycle/operation_diff.rs#operation_diff_matches_native_revsets_after_rewrite_and_abandon
- crates/jj-store-cli/tests/native_lifecycle/operation_diff.rs#operation_diff_merge_parent_selection_and_root_are_explicit
- crates/jj-store-cli/tests/native_lifecycle/operation_diff.rs#operation_diff_rejects_limits_staged_later_and_revoked_reads
- crates/jj-store-cli/tests/native_lifecycle/operation_diff.rs#operation_diff_rejects_missing_and_corrupt_visible_history
<!-- /evidence-field -->

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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle operation_diff_
```

**Notes**

Before implementation: zero matching cases, not acceptance evidence.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo check -p jj-store-service
```

**Notes**

Compile diagnostic: upstream ObjectId trait needed for CommitId::as_bytes. Added the trait import at the JJ integration boundary.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo check -p jj-store-service
```
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle operation_diff_
```

**Notes**

Fixture compile diagnostic: pinned Revset exposes an async stream, not iter(). The upstream parity fixture now collects that native stream.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle operation_diff_
```

**Notes**

Initial 5 cases pass: full-field deltas, native revset parity, parent selection, budgets, membership, revocation, missing/corrupt commit history.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle operation_diff_
```

**Notes**

Expanded 5 cases pass: native child creation distinguishes head changes from visibility, parent abandonment rebases descendants and bookmark/workspace mappings, and a merge yields distinct differences for its two parents.
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

57 requirements, boundaries, progress, rustfmt and all-target Clippy pass.
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

168 Rust tests pass / 6 external cases intentionally ignored. 15 governance and 3 storage-harness tests pass.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build
```
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

All 9 lanes pass, including 168 Rust tests, rustfmt, mdBook and mapped spec-first checks.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm format:check
```
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Structured content diffs/conflict descriptors, query resource qualification,
  HTTP JSON/media/SSE, generated contracts and browser integration remain M4
  work.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.97.1; jj-cli/jj-lib 0.45.1. Native transactions and production reads over disposable in-memory stores; no deployment/performance claim.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Version 1 operation-diff DTO distinguishes root/parent-selection/difference, complete view-field deltas, deterministic native-ID visibility deltas and absent-versus-empty predecessor maps. No authority, persistence or native RPC format change.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


