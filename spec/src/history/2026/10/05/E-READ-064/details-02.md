---
schema: 1
id: E-READ-064
---

# E-READ-064 details 2

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
cargo test -p jj-store-cli --test native_lifecycle --locked graph_acceptance::prepared_evolution_pages::prepared_evolution_continuations_resume_without_preceding_body_reads -- --exact --nocapture
```

**Notes**

native evolution anchor5: Stats { metadata_nodes: 695, predicate_file_bytes: 0, predicate_lines: 0, object_reads: 18, object_bytes: 15567, work: 1295 }; thread 'graph_acceptance::prepared_evolution_pages::prepared_evolution_continuations_resume_without_preceding_body_reads' (5849974) panicked at crates/jj-store-cli/tests/native_lifecycle/prepared_evolution_pages.rs:77:21:; Stats { metadata_nodes: 695, predicate_file_bytes: 0, predicate_lines: 0, object_reads: 18, object_bytes: 15567, work: 1295 }; test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 229 filtered out; finished in 3.91s; ; .local/bounded-reads/performance/prepared-evolution-native-before-064.log
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
cargo test -p jj-store-cli --test local_browser --locked prepared_evolution_pages::prepared_evolution_continuations_resume_without_preceding_body_reads -- --exact --nocapture
```

**Notes**

local evolution anchor5: Stats { metadata_nodes: 20, predicate_file_bytes: 0, predicate_lines: 0, object_reads: 0, object_bytes: 21140, work: 1418 }; thread 'prepared_evolution_pages::prepared_evolution_continuations_resume_without_preceding_body_reads' (5851171) panicked at crates/jj-store-cli/tests/local_browser/prepared_evolution_pages.rs:106:13:; Stats { metadata_nodes: 20, predicate_file_bytes: 0, predicate_lines: 0, object_reads: 0, object_bytes: 21140, work: 1418 }; test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 40 filtered out; finished in 10.32s; ; .local/bounded-reads/performance/prepared-evolution-local-before-064.log
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
cargo test -p jj-store-cli --test native_lifecycle --locked graph_acceptance::prepared_evolution_pages::prepared_evolution_continuations_resume_without_preceding_body_reads -- --exact --nocapture
```

**Notes**

native evolution anchor5: Stats { metadata_nodes: 289, predicate_file_bytes: 0, predicate_lines: 0, object_reads: 8, object_bytes: 6803, work: 1304 }; thread 'graph_acceptance::prepared_evolution_pages::prepared_evolution_continuations_resume_without_preceding_body_reads' (5857649) panicked at crates/jj-store-cli/tests/native_lifecycle/prepared_evolution_pages.rs:78:21:; Stats { metadata_nodes: 289, predicate_file_bytes: 0, predicate_lines: 0, object_reads: 8, object_bytes: 6803, work: 1304 }; test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 229 filtered out; finished in 3.81s; ; .local/bounded-reads/performance/prepared-evolution-native-after-064.log
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
cargo test -p jj-store-cli --test local_browser --locked prepared_evolution_pages::prepared_evolution_continuations_resume_without_preceding_body_reads -- --exact --nocapture
```

**Notes**

local evolution anchor5: Stats { metadata_nodes: 10, predicate_file_bytes: 0, predicate_lines: 0, object_reads: 0, object_bytes: 10386, work: 2077 }; thread 'prepared_evolution_pages::prepared_evolution_continuations_resume_without_preceding_body_reads' (5860585) panicked at crates/jj-store-cli/tests/local_browser/prepared_evolution_pages.rs:106:13:; Stats { metadata_nodes: 10, predicate_file_bytes: 0, predicate_lines: 0, object_reads: 0, object_bytes: 10386, work: 2077 }; test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 40 filtered out; finished in 10.69s; ; .local/bounded-reads/performance/prepared-evolution-local-after-064.log
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
cargo test -p jj-store-cli --test native_lifecycle --locked graph_acceptance::prepared_evolution_pages::prepared_evolution_continuations_resume_without_preceding_body_reads -- --exact --nocapture
```

**Notes**

    Finished `test` profile [unoptimized] target(s) in 40.82s; native evolution anchor5: Stats { metadata_nodes: 252, predicate_file_bytes: 0, predicate_lines: 0, object_reads: 7, object_bytes: 5881, work: 218 }; native evolution anchor5: Stats { metadata_nodes: 252, predicate_file_bytes: 0, predicate_lines: 0, object_reads: 7, object_bytes: 5881, work: 220 }; native evolution anchor19: Stats { metadata_nodes: 252, predicate_file_bytes: 0, predicate_lines: 0, object_reads: 7, object_bytes: 5881, work: 222 }; native evolution anchor19: Stats { metadata_nodes: 252, predicate_file_bytes: 0, predicate_lines: 0, object_reads: 7, object_bytes: 5881, work: 224 }; native evolution anchor32: Stats { metadata_nodes: 213, predicate_file_bytes: 0, predicate_lines: 0, object_reads: 6, object_bytes: 4687, work: 204 }; native evolution anchor32: Stats { metadata_nodes: 213, predicate_file_bytes: 0, predicate_lines: 0, object_reads: 6, object_bytes: 4687, work: 206 }; test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 229 filtered out; finished in 5.53s; 48.51s; .local/bounded-reads/performance/prepared-evolution-native-final-exact-064.log
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
cargo test -p jj-store-cli --test local_browser --locked prepared_evolution_pages::prepared_evolution_continuations_resume_without_preceding_body_reads -- --exact --nocapture
```

**Notes**

    Finished `test` profile [unoptimized] target(s) in 15.31s; local evolution anchor5: Stats { metadata_nodes: 9, predicate_file_bytes: 0, predicate_lines: 0, object_reads: 0, object_bytes: 9077, work: 875 }; Stats { metadata_nodes: 9, predicate_file_bytes: 0, predicate_lines: 0, object_reads: 0, object_bytes: 9077, work: 875 }; test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 40 filtered out; finished in 10.05s; 28.10s; .local/bounded-reads/performance/prepared-evolution-local-final-exact-064.log
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
cargo check -p jj-store-jj --locked
```

**Notes**

Initial shared walker compile check missing Repo trait import (E0599); corrected owning import; no test failure substitution.
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
cargo test -p jj-store-jj --lib --locked
```

**Notes**

64 pass,1 new unit fails:fixture expected reversed parallel branches; inspected pinned jj-core0.45.1 topo_order_forward_ok stack and reverse output,which retain supplied branch order. Corrected expected sequence to1,2,3,4; production follows pinned walker unchanged.
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
cargo test -p jj-store-jj --lib --locked (after unique-anchor proof, before new proof regression)
```

**Notes**

65 tests pass;prepared-evolution-jj-lib-proof-064.log.
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
cargo fmt -p jj-store-jj -p jj-store-cli
```

**Notes**

Formats changed Rust modules/fixtures before focused runs.
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
cargo test -p jj-store-cli --test native_lifecycle --locked graph_acceptance::prepared_evolution_pages::prepared_evolution_continuations_resume_without_preceding_body_reads -- --exact --nocapture
```

**Notes**

    Finished `test` profile [unoptimized] target(s) in 1m 51s; native evolution anchor5: Stats { metadata_nodes: 252, predicate_file_bytes: 0, predicate_lines: 0, object_reads: 7, object_bytes: 5881, work: 285 }; native evolution anchor5: Stats { metadata_nodes: 252, predicate_file_bytes: 0, predicate_lines: 0, object_reads: 7, object_bytes: 5881, work: 287 }; native evolution anchor19: Stats { metadata_nodes: 252, predicate_file_bytes: 0, predicate_lines: 0, object_reads: 7, object_bytes: 5881, work: 289 }; native evolution anchor19: Stats { metadata_nodes: 252, predicate_file_bytes: 0, predicate_lines: 0, object_reads: 7, object_bytes: 5881, work: 291 }; native evolution anchor32: Stats { metadata_nodes: 213, predicate_file_bytes: 0, predicate_lines: 0, object_reads: 6, object_bytes: 4687, work: 271 }; native evolution anchor32: Stats { metadata_nodes: 213, predicate_file_bytes: 0, predicate_lines: 0, object_reads: 6, object_bytes: 4687, work: 273 }; test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 229 filtered out; finished in 6.23s; 120.16s; .local/bounded-reads/performance/prepared-evolution-native-proof-exact-064.log
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
cargo test -p jj-store-cli --test local_browser --locked prepared_evolution_pages::prepared_evolution_continuations_resume_without_preceding_body_reads -- --exact --nocapture
```

**Notes**

    Finished `test` profile [unoptimized] target(s) in 21.41s; local evolution anchor5: Stats { metadata_nodes: 9, predicate_file_bytes: 0, predicate_lines: 0, object_reads: 0, object_bytes: 9029, work: 896 }; local evolution anchor5: Stats { metadata_nodes: 9, predicate_file_bytes: 0, predicate_lines: 0, object_reads: 0, object_bytes: 9029, work: 898 }; local evolution anchor19: Stats { metadata_nodes: 9, predicate_file_bytes: 0, predicate_lines: 0, object_reads: 0, object_bytes: 9221, work: 910 }; local evolution anchor19: Stats { metadata_nodes: 9, predicate_file_bytes: 0, predicate_lines: 0, object_reads: 0, object_bytes: 9221, work: 912 }; local evolution anchor31: Stats { metadata_nodes: 8, predicate_file_bytes: 0, predicate_lines: 0, object_reads: 0, object_bytes: 7387, work: 720 }; local evolution anchor31: Stats { metadata_nodes: 8, predicate_file_bytes: 0, predicate_lines: 0, object_reads: 0, object_bytes: 7387, work: 722 }; test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 40 filtered out; finished in 12.03s; 36.36s; .local/bounded-reads/performance/prepared-evolution-local-proof-exact-064.log
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
cargo test -p jj-store-jj --lib --locked
```

**Notes**

    Finished `test` profile [unoptimized] target(s) in 11.46s; test result: ok. 66 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 5.76s; 17.89s; .local/bounded-reads/performance/prepared-evolution-jj-lib-final-064.log
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
cargo test -p jj-store-cli --test native_lifecycle --locked -- --nocapture
```

**Notes**

    Finished `test` profile [unoptimized] target(s) in 17.99s; test result: ok. 230 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 77.26s; 97.24s; .local/bounded-reads/performance/prepared-evolution-native-lifecycle-final-064.log
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-cli --test local_browser --locked -- --nocapture
```

**Notes**

    Finished `test` profile [unoptimized] target(s) in 34.67s; test result: FAILED. 37 passed; 2 failed; 2 ignored; 0 measured; 0 filtered out; finished in 85.36s; 123.15s; .local/bounded-reads/performance/prepared-evolution-local-browser-final-064.log;Both failures at shared verifier old JSTOPG1 magic expectation;updated to strict JSTOPG2 and fixed64KiB bitmap framing,with full checksum and exact upstream index bytes preserved.
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
pnpm exec prettier --write docs/implementation-plan.md spec/src/architecture.md spec/src/cli.md spec/src/jj-integration.md spec/src/local-jj.md spec/src/milestones.md
```

**Notes**

docs/implementation-plan.md 1052ms
spec/src/architecture.md 614ms (unchanged)
spec/src/cli.md 279ms (unchanged)
spec/src/jj-integration.md 174ms
spec/src/local-jj.md 307ms
spec/src/milestones.md 3931ms
;13.37s;.local/bounded-reads/performance/prepared-evolution-docs-format-064.log
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
pnpm spec:first
```

**Notes**


&gt; @lapismd/jj-store@0.1.0 spec:first /Users/stevejuma/code/lapismd/jj-store-bounded-reads
&gt; spec-validator first

Spec-first gate passed: 15 protected file(s), 5 canonical chapter(s).
;3.39s;.local/bounded-reads/performance/prepared-evolution-spec-first-064.log
<!-- /run -->
<!-- /evidence-field -->

