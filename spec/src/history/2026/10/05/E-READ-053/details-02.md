---
schema: 1
id: E-READ-053
---

# E-READ-053 details 2

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
cargo test -p jj-store-cli --test local_browser --locked prepared_operations::prepared_operation_prefixes_resolve_with_only_selected_metadata -- --exact --nocapture (before implementation)
```

**Notes**

Prepared historical child selector exceeded4 operation visits and returned Limit after Git objects/index segments were made unavailable; 0 passed,1 failed. child-before-053.log.
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
cargo test -p jj-store-cli --test local_browser --locked prepared_operations::prepared_operation_prefixes_resolve_with_only_selected_metadata -- --exact --nocapture (first implementation)
```

**Notes**

E0308: resolve_op_at requires borrowed Arc&lt;OpStore&gt;; fixed &amp;head.op_store(). child-after-053.log.
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
cargo test -p jj-store-cli --test local_browser --locked prepared_operations::prepared_operation_prefixes_resolve_with_only_selected_metadata -- --exact --nocapture (first mixed-suffix cases)
```

**Notes**

New double-child selectors hydrate one more required operation than the original single-step8-metadata assertion; initial mixed reversal also reread prior metadata. Removed immediate inverse-step reads with one request-local immutable prior handle. Original single-step bound remains8; new double-child cases admit one record per extra child, with unchanged4 distinct-operation budget. child-fixed-053.log and child-window-final-053.log.
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
cargo test -p jj-store-jj --lib --locked query::prepared::operations::children::tests -- --nocapture
```

**Notes**

2 passed,55 filtered out. Flat unique/empty/ambiguous/missing-artifact outcomes; source selection/membership, order, duplicate/self edges, checksum and work/byte/row limits; retained lookup survives file removal. child-lookup-tests-053.log.
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
cargo test -p jj-store-cli --test local_browser --locked prepared_operations::prepared_operation_prefixes_resolve_with_only_selected_metadata -- --exact --nocapture
```

**Notes**

1 passed,35 filtered out;10.94s;9 mixed/prefix selectors plus historical child selection with no Git bodies/source index and deleted disposable child file. child-selected-window-053.log.
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
cargo test -p jj-store-cli --test local_browser --locked prepared_operations -- --nocapture
```

**Notes**

2 passed,35 filtered out;10.71s. Includes retained branch ambiguity, ambiguous merge parents, later children outside prepared snapshot, denied access and current-operation child/parent cold admission distinction. child-selectors-complete-053.log.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-cli --test local_browser --locked prepared -- --nocapture (initial direct-consumer run)
```

**Notes**

10 passed,1 failed: shared prepared_index_files helper initially compared the new owned local-operation-children artifact against native index files. Added explicit owned framing/selection/checksum/order/membership validation before excluding only that projection; preserved all native byte equality assertions. child-local-prepared-consumers-053.log; child-results-initial-053.json.
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
cargo test -p jj-store-cli --test local_browser --locked prepared_operations -- --nocapture
```

**Notes**

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 35 filtered out; finished in 11.37s .local/bounded-reads/performance/child-final-selectors-final-053.log; wall time 44.76s.
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
cargo test -p jj-store-cli --test local_browser --locked prepared_points_http::local_point_http_uses_ready_membership_and_only_falls_back_when_cold -- --exact --nocapture
```

**Notes**

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 36 filtered out; finished in 8.41s .local/bounded-reads/performance/child-http-final-053.log; wall time 24.72s.
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
cargo test -p jj-store-jj --lib --locked
```

**Notes**

test result: ok. 57 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 5.25s .local/bounded-reads/performance/child-owning-jj-final-053.log; wall time 10.89s.
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
cargo test -p jj-store-cli --test local_browser --locked prepared -- --nocapture
```

**Notes**

test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 26 filtered out; finished in 24.96s .local/bounded-reads/performance/child-local-prepared-consumers-final-053.log; wall time 40.53s.
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
cargo test -p jj-store-cli --test local_browser --locked preparation_acceptance -- --nocapture
```

**Notes**

test result: ok. 3 passed; 0 failed; 1 ignored; 0 measured; 33 filtered out; finished in 5.68s .local/bounded-reads/performance/child-preparation-consumers-final-053.log; wall time 21.82s.
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
cargo test -p jj-store-cli --test local_browser --locked index_tasks_acceptance -- --nocapture
```

**Notes**

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 32 filtered out; finished in 5.32s .local/bounded-reads/performance/child-local-task-consumers-final-053.log; wall time 20.33s.
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
cargo test -p jj-store-cli --test native_lifecycle --locked graph_acceptance::disk_preparation_matches_pinned_upstream_index_bytes -- --exact --nocapture
```

**Notes**

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 223 filtered out; finished in 0.74s .local/bounded-reads/performance/child-native-index-bytes-final-053.log; wall time 23.53s.
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
cargo test -p jj-store-cli --test native_lifecycle --locked graph_acceptance::disk_preparation_quota_failure_discards_stage_and_allows_retry -- --exact --nocapture
```

**Notes**

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 223 filtered out; finished in 0.51s .local/bounded-reads/performance/child-native-quota-final-053.log; wall time 15.54s.
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
cargo check -p jj-store-jj -p jj-store-service -p jj-store-http -p jj-store-rpc -p jj-store-cli --locked
```

**Notes**

Finished `dev` profile [unoptimized] target(s) in 14.34s .local/bounded-reads/performance/child-packages-final-053.log; wall time 14.46s.
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
cargo fmt -p jj-store-jj -p jj-store-cli -- --check
```

**Notes**

Exit0. .local/bounded-reads/performance/child-rust-format-final-053.log; wall time 0.92s.
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
pnpm check:boundaries
```

**Notes**

package boundaries ok (17 packages) .local/bounded-reads/performance/child-boundaries-final-053.log; wall time 1.44s.
<!-- /run -->
<!-- /evidence-field -->

