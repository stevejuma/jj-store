---
schema: 1
id: E-M3-002
---

# E-M3-002

<!-- evidence-field:id markdown -->
## Id

E-M3-002
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M3
- M3.2
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-27
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

native-operation-traits
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-jj/src/lib.rs
- crates/jj-store-jj/src/operation_stores.rs
- crates/jj-store-jj/tests/native_operations.rs
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-jj/tests/native_operations.rs#native_operation_traits_preserve_identity_fields_and_virtual_roots
- crates/jj-store-jj/tests/native_operations.rs#staged_prefixes_are_session_local_and_reads_publish_nothing
- crates/jj-store-jj/tests/native_operations.rs#concurrent_native_head_updates_keep_siblings_and_receipt_retries
- crates/jj-store-jj/tests/native_operations.rs#failed_upload_and_revocation_never_report_native_success
- crates/jj-store-jj/tests/native_operations.rs#prefix_queries_fail_on_budget_or_deadline_instead_of_partial_matches
- crates/jj-store-jj/tests/native_operations.rs#invalid_heads_and_invalid_id_lengths_fail_before_publication
- crates/jj-store-jj/tests/native_operations.rs#staged_operation_limit_preserves_identical_write_retry
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-jj --test native_operations
```

**Notes**

Before implementation: native_operations target absent.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-jj --test native_operations
```

**Notes**

6 tests over real pinned OpStore/OpHeadsStore traits, checked native objects and the manifest coordinator using a deterministic test authority.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-jj --test native_operations staged_operation_limit_preserves_identical_write_retry
```

**Notes**

Review regression: a full staging budget rejected a repeated identical operation upload; fixed to preserve idempotency without admitting a new distinct operation.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-jj --test native_operations
```

**Notes**

7 native operation trait tests, including identical retry at the staging limit.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check
```
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check
```
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test
```

**Notes**

59 Rust tests, 14 governance tests and 3 storage-harness self-tests. 2 external RustFS tests remain explicitly ignored in the normal lane; this slice does not claim live transport integration.
<!-- /run -->

<!-- run -->
### Run 8

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
### Run 9

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

- Production Backend, StoreFactories, authenticated authority implementation and
  native RPC are still required.
- Independent physical workspaces and full CLI lifecycle acceptance remain
  unimplemented.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

jj-lib 0.45.1 / 7c41cdeb16b6b321c64e789a966b6adf723816a5; Rust 1.97.1
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No storage format changes. Owned operation-authority interface requires current authorization and guarded durable publication; bounded session-local staging enumeration is not committed authority.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


