---
schema: 1
id: E-READ-063
---

# E-READ-063 details 2

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
cargo test -p jj-store-cli --test native_lifecycle --locked graph_acceptance::prepared_log_pages::prepared_all_log_pages_seek_deep_anchors_under_fixed_work -- --exact --nocapture (initial before, max_work160)
```

**Notes**

Fixture initially reused protected root change ID; Denied. Corrected fixture to distinct nonroot IDs; prepared-log-native-before-063.log.
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
cargo test -p jj-store-cli --test native_lifecycle --locked graph_acceptance::prepared_log_pages::prepared_all_log_pages_seek_deep_anchors_under_fixed_work -- --exact --nocapture (corrected traversal before, max_work160)
```

**Notes**

Deep traversal fails Limit; prepared-log-native-corrected-before-063.log.
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
cargo test -p jj-store-cli --test native_lifecycle --locked graph_acceptance::prepared_log_pages::prepared_all_log_pages_seek_deep_anchors_under_fixed_work -- --exact --nocapture (initial implementation, max_work160)
```

**Notes**

Required fresh request admission exceeds initial fixture allowance; prepared-log-native-after-063.log.
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
cargo test -p jj-store-cli --test native_lifecycle --locked graph_acceptance::prepared_log_pages::prepared_all_log_pages_seek_deep_anchors_under_fixed_work -- --exact --nocapture (diagnostic allowance5000, assertion160 retained)
```

**Notes**

Diagnostic exposes 219 work/95 metadata nodes/3 object reads; retained160 assertion fails; prepared-log-native-diagnostic-063.log.
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
cargo test -p jj-store-cli --test native_lifecycle --locked graph_acceptance::prepared_log_pages::prepared_all_log_pages_seek_deep_anchors_under_fixed_work -- --exact --nocapture (controlled parent traversal before, max_work300)
```

**Notes**

Restored only old traversal from parent while keeping identical projection/admission; deep page fails Limit at calibrated fixture allowance. Implemented traversal restored byte-identically in finally block; prepared-log-native-calibrated-before-063.log.
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
cargo test -p jj-store-cli --test native_lifecycle --locked graph_acceptance::prepared_log_pages::prepared_all_log_pages_seek_deep_anchors_under_fixed_work -- --exact --nocapture (final exact, calibrated fixture allowance)
```

**Notes**

1 exact regression passes; 224-commit fixture compares independent upstream all() &amp; all(), first/deep/repeated/terminal pages, ordered edges/cursors and excluded anchor. Native work181–219, at most3 object reads under300. prepared-log-native-final-exact-063.log.
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
cargo test -p jj-store-cli --test local_browser --locked prepared_log_pages::prepared_all_log_pages_seek_deep_anchors_under_fixed_work -- --exact --nocapture (corrected traversal before, max_work160)
```

**Notes**

Deep traversal fails Limit; prepared-log-local-before-063.log.
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
cargo test -p jj-store-cli --test local_browser --locked prepared_log_pages::prepared_all_log_pages_seek_deep_anchors_under_fixed_work -- --exact --nocapture (initial implementation, max_work160)
```

**Notes**

Required fresh request admission exceeds initial fixture allowance; prepared-log-local-after-063.log.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-cli --test local_browser --locked prepared_log_pages::prepared_all_log_pages_seek_deep_anchors_under_fixed_work -- --exact --nocapture (diagnostic allowance5000, assertion160 retained)
```

**Notes**

Diagnostic exposes 430 work/5 metadata nodes; retained160 assertion fails; prepared-log-local-diagnostic-063.log.
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
cargo test -p jj-store-cli --test local_browser --locked prepared_log_pages::prepared_all_log_pages_seek_deep_anchors_under_fixed_work -- --exact --nocapture (controlled parent traversal before, max_work512)
```

**Notes**

Restored only old traversal from parent while keeping identical projection/admission; deep page fails Limit at calibrated fixture allowance. Implemented traversal restored byte-identically in finally block; prepared-log-local-calibrated-before-063.log.
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
cargo test -p jj-store-cli --test local_browser --locked prepared_log_pages::prepared_all_log_pages_seek_deep_anchors_under_fixed_work -- --exact --nocapture (final exact, calibrated fixture allowance)
```

**Notes**

1 exact regression passes; 224-commit fixture compares independent upstream all() &amp; all(), first/deep/repeated/terminal pages, ordered edges/cursors and excluded anchor. Local work376–440, at most5 metadata nodes under512. prepared-log-local-final-exact-063.log.
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
cargo test -p jj-store-jj --lib --locked (initial implementation)
```

**Notes**

61 existing tests pass before two new visibility units were added; prepared-log-jj-lib-initial-063.log.
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
cargo fmt -p jj-store-jj -p jj-store-cli
```

**Notes**

Formats changed Rust packages before final exact runs.
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
cargo test -p jj-store-jj --lib --locked
```

**Notes**

    Finished `test` profile [unoptimized] target(s) in 5.77s; test result: ok. 63 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 4.69s; 11.02s; .local/bounded-reads/performance/prepared-log-jj-lib-063.log
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
cargo test -p jj-store-cli --test native_lifecycle --locked -- --nocapture
```

**Notes**

    Finished `test` profile [unoptimized] target(s) in 14.04s; test result: ok. 229 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 83.69s; 99.74s; .local/bounded-reads/performance/prepared-log-native-lifecycle-063.log
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
cargo test -p jj-store-cli --test local_browser --locked -- --nocapture
```

**Notes**

    Finished `test` profile [unoptimized] target(s) in 17.81s; test result: ok. 38 passed; 0 failed; 2 ignored; 0 measured; 0 filtered out; finished in 81.72s; 102.37s; .local/bounded-reads/performance/prepared-log-local-browser-063.log
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
cargo check -p jj-store-jj -p jj-store-cli --locked
```

**Notes**

    Finished `dev` profile [unoptimized] target(s) in 16.60s; 16.71s; .local/bounded-reads/performance/prepared-log-packages-063.log
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
cargo fmt -p jj-store-jj -p jj-store-cli -- --check
```

**Notes**

; 0.96s; .local/bounded-reads/performance/prepared-log-rust-format-063.log
<!-- /run -->
<!-- /evidence-field -->

