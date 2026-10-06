---
schema: 1
id: E-READ-054
---

# E-READ-054 details 2

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
cargo test -p jj-store-cli --test native_lifecycle --locked graph_acceptance::prepared_native_child_selectors_hydrate_only_selected_operation_metadata -- --exact --nocapture (initial test and first implementation)
```

**Notes**

Both initial runs failed Limit because max_nodes8 counts decoded fields, not operation bodies. Corrected the new regression to512 decoded nodes plus an independent8 object-read bound, then rechecked unchanged parent implementation. native-child-before-054.log/native-child-after-054.log. These initial failures do not establish the intended history-scan baseline.
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
python3 .local/bounded-reads/performance/native-before-054.py (unchanged parent production plus corrected exact regression)
```

**Notes**

0 passed,1 failed IndexBudget: old native operation-prefix selection required complete inventory enumeration, deliberately disabled by the point-only capability. New regression allows512 decoded metadata nodes and caps object reads at8. Script restored every in-progress production file in finally. native-child-baseline-corrected-054.log.
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
cargo test -p jj-store-cli --test native_lifecycle --locked graph_acceptance::prepared_native_child_selectors_hydrate_only_selected_operation_metadata -- --exact --nocapture (corrected test)
```

**Notes**

1 passed,224 filtered; four selectors read4/5/5/6 objects and136/170/170/204 decoded metadata nodes over40-operation history. Later-child and later-operation exclusion pass. native-child-corrected-054.log.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-cli --test native_lifecycle --locked graph_acceptance::prepared_native_child_selectors -- --nocapture (first extended test)
```

**Notes**

New revocation fixture used the wrong TokenRequest fields and missed the borrowed argument: E0559/E0063/E0308. Corrected to expected manifest snapshot plus token ID and borrowed request; switched queried capability to independent reader token. native-child-extended-054.log.
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
cargo test -p jj-store-cli --test native_lifecycle --locked prepared_native_ -- --nocapture
```

**Notes**

3 passed,224 filtered: cold current-relative path, warm mixed selectors without operation files, no views/commits/inventory; retained branch ambiguity, sibling/later exclusion, fresh reader-token revocation and older-generation upgrade preserving exact filter bytes and manifest. native-child-complete-054.log.
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
cargo test -p jj-store-cli --test native_lifecycle --locked prepared_native_ -- --nocapture (no-copy resolver first compile)
```

**Notes**

E0599: pinned HexPrefix constructor is try_from_hex, not new. Corrected constructor; full final exact native family and graph consumers rerun. native-child-selector-final-compile-054.log; first no-copy compile failure preserved.
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
cargo test -p jj-store-cli --test native_lifecycle --locked prepared_native_ -- --nocapture
```

**Notes**

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 224 filtered out; finished in 7.31s .local/bounded-reads/performance/native-child-final-054.log; wall time 55.68s. Results also retained in native-child-results-initial-054.json.
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
cargo test -p jj-store-jj --lib --locked
```

**Notes**

test result: ok. 57 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 7.38s .local/bounded-reads/performance/native-child-owning-jj-054.log; wall time 22.87s. Results also retained in native-child-results-initial-054.json.
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
cargo test -p jj-store-cli --test native_lifecycle --locked graph_acceptance -- --nocapture
```

**Notes**

test result: ok. 43 passed; 0 failed; 0 ignored; 0 measured; 184 filtered out; finished in 15.74s .local/bounded-reads/performance/native-child-graph-consumers-054.log; wall time 40.59s. Results also retained in native-child-results-initial-054.json.
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
cargo test -p jj-store-cli --test native_lifecycle --locked reference -- --nocapture
```

**Notes**

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 219 filtered out; finished in 62.25s .local/bounded-reads/performance/native-child-reference-consumers-054.log; wall time 81.34s. Results also retained in native-child-results-initial-054.json.
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

test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 26 filtered out; finished in 25.63s .local/bounded-reads/performance/native-child-local-consumers-054.log; wall time 45.78s. Results also retained in native-child-results-initial-054.json.
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
cargo check -p jj-store-jj -p jj-store-service -p jj-store-http -p jj-store-rpc -p jj-store-cli --locked
```

**Notes**

Finished `dev` profile [unoptimized] target(s) in 24.01s .local/bounded-reads/performance/native-child-packages-054.log; wall time 24.14s. Results also retained in native-child-results-initial-054.json.
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
cargo fmt -p jj-store-jj -p jj-store-cli -- --check
```

**Notes**

Exit0. .local/bounded-reads/performance/native-child-rust-format-054.log; wall time 1.35s. Results also retained in native-child-results-initial-054.json.
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
pnpm check:boundaries
```

**Notes**

package boundaries ok (17 packages) .local/bounded-reads/performance/native-child-boundaries-054.log; wall time 1.57s. Results also retained in native-child-results-initial-054.json.
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
pnpm spec:first
```

**Notes**

Spec-first gate passed: 13 protected file(s), 5 canonical chapter(s). .local/bounded-reads/performance/native-child-spec-first-054.log; wall time 1.9s. Results also retained in native-child-results-initial-054.json.
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
cargo test -p jj-store-cli --test native_lifecycle --locked prepared_native_ -- --nocapture
```

**Notes**

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 224 filtered out; finished in 3.42s .local/bounded-reads/performance/native-child-selector-final-054.log; wall time 40.43s. Results also retained in native-child-results-resolver-054.json.
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
cargo test -p jj-store-cli --test native_lifecycle --locked graph_acceptance -- --nocapture
```

**Notes**

test result: ok. 43 passed; 0 failed; 0 ignored; 0 measured; 184 filtered out; finished in 16.29s .local/bounded-reads/performance/native-child-graphs-final-054.log; wall time 31.19s. Results also retained in native-child-results-resolver-054.json.
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
cargo check -p jj-store-jj -p jj-store-service -p jj-store-http -p jj-store-rpc -p jj-store-cli --locked
```

**Notes**

Finished `dev` profile [unoptimized] target(s) in 14.47s .local/bounded-reads/performance/native-child-packages-final-054.log; wall time 14.61s. Results also retained in native-child-results-resolver-054.json.
<!-- /run -->
<!-- /evidence-field -->

