---
schema: 1
id: E-READ-056
---

# E-READ-056 details 2

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
jj --no-pager st; jj --no-pager log -n5; jj rebase -r xkmpzpso -o oqwzuzwn; jj describe -m "Land and verify prepared native and local operation metadata reads"; jj --no-pager diff --from oqwzuzwn --to @ --stat
```

**Notes**

Main was clean at E-READ-052; empty change advanced to E-READ-055. Zero source-tree differences. E-READ-053/054/055 retained as committed ancestors; no merge conflicts.
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
python3 preserve bounded-reads performance evidence artifacts for E-READ-053/054/055 in main
```

**Notes**

89 files copied byte-identically, existing files checked before writes. Temporary evidence and workspace preserved. All496 parent evidence records captured in parent-evidence-056.json.
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
cargo test -p jj-store-cli --test local_browser --locked prepared_operation_admission::prepared_operation_pages_admit_fresh_metadata_without_any_view_or_git_reads -- --exact --nocapture
```

**Notes**

Finished `test` profile [unoptimized] target(s) in 59.73s; test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 37 filtered out; finished in 11.42s; 73.91s; .local/bounded-reads/performance/main-operation-local-exact-056.log
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
cargo test -p jj-store-cli --test native_lifecycle --locked prepared_native_ -- --nocapture
```

**Notes**

Finished `test` profile [unoptimized] target(s) in 14.53s; test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 224 filtered out; finished in 4.99s; 21.52s; .local/bounded-reads/performance/main-operation-native-exact-056.log
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
cargo test -p jj-store-cli --test local_browser --locked -- --nocapture
```

**Notes**

Finished `test` profile [unoptimized] target(s) in 1.48s; test result: ok. 36 passed; 0 failed; 2 ignored; 0 measured; 0 filtered out; finished in 50.52s; 52.31s; .local/bounded-reads/performance/main-operation-local-readers-056.log
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
cargo test -p jj-store-cli --test native_lifecycle --locked graph_acceptance -- --nocapture
```

**Notes**

Finished `test` profile [unoptimized] target(s) in 2.82s; test result: ok. 43 passed; 0 failed; 0 ignored; 0 measured; 184 filtered out; finished in 14.35s; 17.41s; .local/bounded-reads/performance/main-operation-native-graphs-056.log
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
cargo test -p jj-store-cli --test native_lifecycle --locked reference -- --nocapture
```

**Notes**

Finished `test` profile [unoptimized] target(s) in 0.53s; test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 219 filtered out; finished in 50.53s; 51.14s; .local/bounded-reads/performance/main-operation-native-references-056.log
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

Finished `test` profile [unoptimized] target(s) in 9.72s; test result: ok. 57 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 4.87s; 15.07s; .local/bounded-reads/performance/main-operation-owning-jj-056.log
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
cargo test -p jj-store-service --lib --locked
```

**Notes**

Finished `test` profile [unoptimized] target(s) in 16.53s; test result: ok. 35 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.81s; 18.76s; .local/bounded-reads/performance/main-operation-owning-service-056.log
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
cargo test -p jj-store-http --lib --locked
```

**Notes**

Finished `test` profile [unoptimized] target(s) in 38.58s; test result: ok. 21 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.38s; 40.57s; .local/bounded-reads/performance/main-operation-owning-http-056.log
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
cargo check -p jj-store-jj -p jj-store-service -p jj-store-http -p jj-store-rpc -p jj-store-cli --locked
```

**Notes**

Finished `dev` profile [unoptimized] target(s) in 22.95s; 23.23s; .local/bounded-reads/performance/main-operation-packages-056.log
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
cargo fmt -p jj-store-jj -p jj-store-cli -- --check
```

**Notes**

; 1.13s; .local/bounded-reads/performance/main-operation-rust-format-056.log
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
pnpm check:boundaries
```

**Notes**

package boundaries ok (17 packages); 0.45s; .local/bounded-reads/performance/main-operation-boundaries-056.log
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
pnpm exec prettier --write docs/implementation-plan.md spec/src/architecture.md spec/src/milestones.md
```

**Notes**

Changed integration documents formatted; main-operation-docs-write-056.log.
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
pnpm check:progress
```

**Notes**

progress ok (139 tasks; evidence bookkeeping only); .local/bounded-reads/performance/main-operation-progress-initial-056.log
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
pnpm spec:first
```

**Notes**

Spec-first gate passed: 1 protected file(s), 2 canonical chapter(s).; .local/bounded-reads/performance/main-operation-spec-first-initial-056.log
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
pnpm spec:validate
```

**Notes**

Specification validated: 5 validators, 23 chapters, 172 requirements.; .local/bounded-reads/performance/main-operation-spec-validation-initial-056.log
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
python3 compare current ledger with parent-evidence-056.json
```

**Notes**

All496 parent records remain JSON-identical; E-READ-056 sole new record,497 total.
<!-- /run -->
<!-- /evidence-field -->

