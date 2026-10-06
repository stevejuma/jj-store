---
schema: 1
id: E-READ-057
---

# E-READ-057 details 2

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
cargo test -p jj-store-cli --test local_browser --locked prepared_operation_history::prepared_operation_history_skips_prior_bodies_for_filtered_and_historical_pages -- --exact --nocapture (unchanged E-READ-056 production with final five-visit fixture)
```

**Notes**

Exact before regression fails Limit at the first deep anchor: ordinary JJ body traversal replays earlier history. Earlier four-visit fixture also fails Limit; final five visits admit head, historical start, anchor, row and probe. operation-history-before-corrected-057.log and operation-history-before-057.log. Parent-production restoration completed before after verification.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-jj --lib --locked query::prepared::operations::history -- --nocapture (initial unit implementation)
```

**Notes**

Two compile attempts used the wrong pinned time-range path: time_util::TimeSpan, then backend::TimestampRange. Corrected to op_store::TimestampRange. operation-history-units-initial-057.log and operation-history-units-second-057.log.
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
cargo test -p jj-store-jj --lib --locked query::prepared::operations::history -- --nocapture (initial corrected units)
```

**Notes**

Two units passed; corrected private capture visibility after initial warning, then strengthened exact ordering to 32 divergent/skewed-time DAGs,37 nodes each,every historical start. Final exact verification runs the strengthened fixture. operation-history-units-third-057.log.
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
cargo test -p jj-store-cli --test local_browser --locked prepared_operation_history::prepared_operation_history_skips_prior_bodies_for_filtered_and_historical_pages -- --exact --nocapture (initial implementation)
```

**Notes**

Initial six-visit fixture passes with4–6 metadata records; final fixture tightened to five source visits. operation-history-local-after-057.log.
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
cargo test -p jj-store-cli --test native_lifecycle --locked graph_acceptance::operation_history_acceptance::prepared_native_operation_history_hydrates_only_deep_anchor_and_page_candidates -- --exact --nocapture (initial implementation)
```

**Notes**

Native deep plain/filtered/historical pages pass with2–5 object reads and70–170 decoded metadata nodes,denied views/commits/inventory,removed prepared topology artifact and invalid anchors. operation-history-native-after-057.log.
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
cargo test -p jj-store-jj --lib --locked query::prepared::operations::history -- --nocapture
```

**Notes**

Finished `test` profile [unoptimized] target(s) in 5.91s; test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 57 filtered out; finished in 0.35s; 6.74s; .local/bounded-reads/performance/operation-history-topology-exact-057.log
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
cargo test -p jj-store-cli --test local_browser --locked prepared_operation_history::prepared_operation_history_skips_prior_bodies_for_filtered_and_historical_pages -- --exact --nocapture
```

**Notes**

Finished `test` profile [unoptimized] target(s) in 1m 19s; test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 38 filtered out; finished in 13.65s; 96.03s; .local/bounded-reads/performance/operation-history-local-exact-057.log
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
cargo test -p jj-store-cli --test native_lifecycle --locked graph_acceptance::operation_history_acceptance::prepared_native_operation_history_hydrates_only_deep_anchor_and_page_candidates -- --exact --nocapture
```

**Notes**

Finished `test` profile [unoptimized] target(s) in 15.26s; test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 227 filtered out; finished in 3.75s; 20.95s; .local/bounded-reads/performance/operation-history-native-exact-057.log
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
cargo test -p jj-store-jj --lib --locked
```

**Notes**

Finished `test` profile [unoptimized] target(s) in 0.77s; test result: ok. 59 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 4.76s; 5.64s; .local/bounded-reads/performance/operation-history-owning-jj-057.log
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
cargo test -p jj-store-cli --test local_browser --locked -- --nocapture
```

**Notes**

Finished `test` profile [unoptimized] target(s) in 12.63s; test result: ok. 37 passed; 0 failed; 2 ignored; 0 measured; 0 filtered out; finished in 48.21s; 63.47s; .local/bounded-reads/performance/operation-history-local-readers-057.log
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
cargo test -p jj-store-cli --test native_lifecycle --locked graph_acceptance -- --nocapture
```

**Notes**

Finished `test` profile [unoptimized] target(s) in 14.63s; test result: ok. 44 passed; 0 failed; 0 ignored; 0 measured; 184 filtered out; finished in 16.16s; 32.82s; .local/bounded-reads/performance/operation-history-native-graphs-057.log
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
cargo test -p jj-store-cli --test native_lifecycle --locked reference -- --nocapture
```

**Notes**

Finished `test` profile [unoptimized] target(s) in 13.84s; test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 220 filtered out; finished in 51.33s; 67.09s; .local/bounded-reads/performance/operation-history-native-references-057.log
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
cargo check -p jj-store-jj -p jj-store-service -p jj-store-http -p jj-store-rpc -p jj-store-cli --locked
```

**Notes**

Finished `dev` profile [unoptimized] target(s) in 33.27s; 33.38s; .local/bounded-reads/performance/operation-history-packages-057.log
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
cargo fmt -p jj-store-jj -p jj-store-cli -- --check
```

**Notes**

; 1.0s; .local/bounded-reads/performance/operation-history-rust-format-057.log
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
pnpm check:boundaries
```

**Notes**

package boundaries ok (17 packages); 1.53s; .local/bounded-reads/performance/operation-history-boundaries-057.log
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
pnpm exec prettier --write docs/implementation-plan.md spec/src/api.md spec/src/architecture.md spec/src/cli.md spec/src/jj-integration.md spec/src/local-jj.md spec/src/milestones.md
```

**Notes**

Changed canonical and ledger documents formatted. operation-history-ledger-format-057.log.
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

19 protected files,6 canonical chapters. operation-history-spec-first-057.log.
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
pnpm spec:validate
```

**Notes**

5 validators,23 chapters,172 requirements. operation-history-spec-validation-057.log.
<!-- /run -->
<!-- /evidence-field -->

