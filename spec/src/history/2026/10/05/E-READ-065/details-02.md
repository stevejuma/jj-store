---
schema: 1
id: E-READ-065
---

# E-READ-065 details 2

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
cargo test -p jj-store-cli --test native_lifecycle --locked graph_acceptance::prepared_evolution_pages::prepared_evolution_seeks_predecessors_across_unrelated_bookmark_operations -- --exact --nocapture
```

**Notes**

Finished `test` profile [unoptimized] target(s) in 18.65s; native sparse predecessor lookup: Stats { metadata_nodes: 1910, predicate_file_bytes: 0, predicate_lines: 0, object_reads: 52, object_bytes: 41832, work: 606 }; thread 'graph_acceptance::prepared_evolution_pages::prepared_evolution_seeks_predecessors_across_unrelated_bookmark_operations' (5971853) panicked at crates/jj-store-cli/tests/native_lifecycle/prepared_evolution_pages.rs:42:17:; test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 230 filtered out; finished in 4.21s; 24.91s; .local/bounded-reads/performance/predecessor-native-before-065.log
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
cargo test -p jj-store-cli --test local_browser --locked prepared_evolution_pages::prepared_evolution_seeks_predecessors_across_unrelated_bookmark_operations -- --exact --nocapture
```

**Notes**

Finished `test` profile [unoptimized] target(s) in 13.62s; local sparse predecessor lookup: Stats { metadata_nodes: 54, predicate_file_bytes: 0, predicate_lines: 0, object_reads: 0, object_bytes: 102591, work: 9481 }; thread 'prepared_evolution_pages::prepared_evolution_seeks_predecessors_across_unrelated_bookmark_operations' (5972886) panicked at crates/jj-store-cli/tests/local_browser/prepared_evolution_pages.rs:73:9:; test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 41 filtered out; finished in 13.78s; 30.16s; .local/bounded-reads/performance/predecessor-local-before-065.log
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
cargo test -p jj-store-jj --lib --locked query::prepared::operations::history::tests::predecessor_lookup -- --nocapture
```

**Notes**

Finished `test` profile [unoptimized] target(s) in 7.87s; test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 66 filtered out; finished in 0.05s; 8.47s; .local/bounded-reads/performance/predecessor-unit-after-065.log
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
cargo test -p jj-store-cli --test native_lifecycle --locked graph_acceptance::prepared_evolution_pages::prepared_evolution_seeks_predecessors_across_unrelated_bookmark_operations -- --exact --nocapture
```

**Notes**

Finished `test` profile [unoptimized] target(s) in 57.30s; native sparse predecessor lookup: Stats { metadata_nodes: 312, predicate_file_bytes: 0, predicate_lines: 0, object_reads: 5, object_bytes: 10765, work: 326 }; native sparse predecessor lookup: Stats { metadata_nodes: 312, predicate_file_bytes: 0, predicate_lines: 0, object_reads: 5, object_bytes: 10765, work: 328 }; test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 230 filtered out; finished in 4.19s; 63.76s; .local/bounded-reads/performance/predecessor-native-after-065.log
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
cargo test -p jj-store-cli --test local_browser --locked prepared_evolution_pages::prepared_evolution_seeks_predecessors_across_unrelated_bookmark_operations -- --exact --nocapture
```

**Notes**

Finished `test` profile [unoptimized] target(s) in 14.21s; local sparse predecessor lookup: Stats { metadata_nodes: 7, predicate_file_bytes: 0, predicate_lines: 0, object_reads: 0, object_bytes: 43665, work: 2902 }; local sparse predecessor lookup: Stats { metadata_nodes: 7, predicate_file_bytes: 0, predicate_lines: 0, object_reads: 0, object_bytes: 43665, work: 2904 }; test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 41 filtered out; finished in 13.50s; 30.51s; .local/bounded-reads/performance/predecessor-local-after-065.log
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
cargo check -p jj-store-jj --locked
```

**Notes**

Finished dev profile in 9.28s; .local/bounded-reads/performance/predecessor-compile-065.log
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
cargo test -p jj-store-jj --lib --locked query::prepared::operations::history::tests::predecessor_lookup -- --nocapture
```

**Notes**

Finished `test` profile [unoptimized] target(s) in 7.37s; test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 66 filtered out; finished in 0.06s; 7.89s; .local/bounded-reads/performance/predecessor-unit-compact-065.log
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
cargo test -p jj-store-cli --test native_lifecycle --locked graph_acceptance::prepared_evolution_pages::prepared_evolution_seeks_predecessors_across_unrelated_bookmark_operations -- --exact --nocapture
```

**Notes**

Finished `test` profile [unoptimized] target(s) in 57.12s; native sparse predecessor lookup: Stats { metadata_nodes: 312, predicate_file_bytes: 0, predicate_lines: 0, object_reads: 5, object_bytes: 10765, work: 326 }; native sparse predecessor lookup: Stats { metadata_nodes: 312, predicate_file_bytes: 0, predicate_lines: 0, object_reads: 5, object_bytes: 10765, work: 328 }; test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 230 filtered out; finished in 4.13s; 63.25s; .local/bounded-reads/performance/predecessor-native-compact-065.log
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
cargo test -p jj-store-cli --test local_browser --locked prepared_evolution_pages::prepared_evolution_seeks_predecessors_across_unrelated_bookmark_operations -- --exact --nocapture
```

**Notes**

Finished `test` profile [unoptimized] target(s) in 17.59s; local sparse predecessor lookup: Stats { metadata_nodes: 7, predicate_file_bytes: 0, predicate_lines: 0, object_reads: 0, object_bytes: 43693, work: 2903 }; local sparse predecessor lookup: Stats { metadata_nodes: 7, predicate_file_bytes: 0, predicate_lines: 0, object_reads: 0, object_bytes: 43693, work: 2905 }; test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 41 filtered out; finished in 14.21s; 34.8s; .local/bounded-reads/performance/predecessor-local-compact-065.log
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
cargo test -p jj-store-jj --lib --locked query::prepared::operations::history::tests::predecessor_occurrences_do_not_multiply_topology_scratch -- --exact --nocapture
```

**Notes**

Finished `test` profile [unoptimized] target(s) in 6.76s; test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 68 filtered out; finished in 0.03s; 7.33s; .local/bounded-reads/performance/predecessor-memory-exact-verification-065.log
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
cargo test -p jj-store-jj --lib --locked -- --nocapture
```

**Notes**

Finished `test` profile [unoptimized] target(s) in 0.34s; test result: ok. 69 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 4.79s; 5.2s; .local/bounded-reads/performance/predecessor-jj-lib-verification-065.log
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
cargo test -p jj-store-cli --test native_lifecycle --locked -- --nocapture
```

**Notes**

Finished `test` profile [unoptimized] target(s) in 14.00s; native sparse predecessor lookup: Stats { metadata_nodes: 312, predicate_file_bytes: 0, predicate_lines: 0, object_reads: 5, object_bytes: 10765, work: 326 }; native sparse predecessor lookup: Stats { metadata_nodes: 312, predicate_file_bytes: 0, predicate_lines: 0, object_reads: 5, object_bytes: 10765, work: 328 }; test result: ok. 231 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 70.46s; 86.36s; .local/bounded-reads/performance/predecessor-native-lifecycle-verification-065.log
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
cargo test -p jj-store-cli --test local_browser --locked -- --nocapture
```

**Notes**

Finished `test` profile [unoptimized] target(s) in 14.85s; local sparse predecessor lookup: Stats { metadata_nodes: 7, predicate_file_bytes: 0, predicate_lines: 0, object_reads: 0, object_bytes: 43665, work: 2902 }; local sparse predecessor lookup: Stats { metadata_nodes: 7, predicate_file_bytes: 0, predicate_lines: 0, object_reads: 0, object_bytes: 43665, work: 2904 }; test result: ok. 40 passed; 0 failed; 2 ignored; 0 measured; 0 filtered out; finished in 98.30s; 115.84s; .local/bounded-reads/performance/predecessor-local-browser-verification-065.log
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
cargo check -p jj-store-jj -p jj-store-cli --locked
```

**Notes**

Finished `dev` profile [unoptimized] target(s) in 19.34s; 19.43s; .local/bounded-reads/performance/predecessor-packages-verification-065.log
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
cargo fmt -p jj-store-jj -p jj-store-cli -- --check
```

**Notes**

; 1.06s; .local/bounded-reads/performance/predecessor-rust-format-verification-065.log
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
pnpm check:boundaries
```

**Notes**

package boundaries ok (17 packages); 1.33s; .local/bounded-reads/performance/predecessor-boundaries-verification-065.log
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
pnpm check:architecture
```

**Notes**

web architecture ok (988 modules); 1.41s; .local/bounded-reads/performance/predecessor-architecture-verification-065.log
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
pnpm spec:first
```

**Notes**

Spec-first gate passed: 10 protected file(s), 5 canonical chapter(s).; 1.69s; .local/bounded-reads/performance/predecessor-spec-first-verification-065.log
<!-- /run -->
<!-- /evidence-field -->

