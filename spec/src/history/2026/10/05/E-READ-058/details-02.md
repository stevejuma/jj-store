---
schema: 1
id: E-READ-058
---

# E-READ-058 details 2

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
cargo test -p jj-store-jj --lib --locked query::prepared::operations::history::tests::selected_operation_continuation_seeks_under_a_fixed_work_budget -- --exact --nocapture (before)
```

**Notes**

Unchanged E-READ-057 production fails Limit before the deep anchor in a64-node operation chain under16 work units. operation-seek-before-058.log.
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
cargo test -p jj-store-jj --lib --locked query::prepared::operations::history::tests::selected_operation_continuation_seeks_under_a_fixed_work_budget -- --exact --nocapture (initial implementation)
```

**Notes**

Compile found two private implementation mistakes:derive_selected needed pub(super);the mutable neighbor callback needed a mutable binding. Both corrected. operation-seek-after-058.log.
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
cargo test -p jj-store-jj --lib --locked query::prepared::operations::history::tests::selected_operation_continuation_seeks_under_a_fixed_work_budget -- --exact --nocapture (corrected implementation)
```

**Notes**

Exact regression passes twice for the same deep anchor under unchanged16-unit budget. operation-seek-after-corrected-058.log.
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
python3 .local/bounded-reads/performance/verify-058.py (initial run)
```

**Notes**

Strengthened3 exact topology units passed,then local endpoint failed the provisional700-unit complete-request fixture bound. Fresh canonical metadata admission consumes712 units for this page;selected traversal itself remains bounded independently at16 in the unit regression. Corrected complete local fixture allowance to800,below every unchanged-parent selected request at940–942;historical-start requests retain ordinary independent limits. Initial results and failure log retained as operation-seek-results-initial-058.json and operation-seek-local-exact-initial-058.log.
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
cargo test -p jj-store-cli --test local_browser --locked prepared_operation_history::prepared_operation_history_skips_prior_bodies_for_filtered_and_historical_pages -- --exact --nocapture (diagnostic complete-request bound900)
```

**Notes**

Selected plain/filtered pages consume524–712 work units;historical-start pages remain992–1088. Final complete local bound tightened to800. operation-seek-local-corrected-058.log.
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

Finished `test` profile [unoptimized] target(s) in 0.73s; test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 57 filtered out; finished in 0.42s; 1.22s; .local/bounded-reads/performance/operation-seek-topology-exact-058.log
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

Finished `test` profile [unoptimized] target(s) in 15.16s; test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 38 filtered out; finished in 14.17s; 32.09s; .local/bounded-reads/performance/operation-seek-local-exact-058.log
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

Finished `test` profile [unoptimized] target(s) in 22.06s; test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 227 filtered out; finished in 4.03s; 28.21s; .local/bounded-reads/performance/operation-seek-native-exact-058.log
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

Finished `test` profile [unoptimized] target(s) in 0.90s; test result: ok. 60 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 4.98s; 5.99s; .local/bounded-reads/performance/operation-seek-owning-jj-058.log
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

Finished `test` profile [unoptimized] target(s) in 16.24s; test result: ok. 37 passed; 0 failed; 2 ignored; 0 measured; 0 filtered out; finished in 49.55s; 68.49s; .local/bounded-reads/performance/operation-seek-local-readers-058.log
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

Finished `test` profile [unoptimized] target(s) in 14.90s; test result: ok. 44 passed; 0 failed; 0 ignored; 0 measured; 184 filtered out; finished in 17.07s; 33.94s; .local/bounded-reads/performance/operation-seek-native-graphs-058.log
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

Finished `test` profile [unoptimized] target(s) in 15.61s; test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 220 filtered out; finished in 53.31s; 71.11s; .local/bounded-reads/performance/operation-seek-native-references-058.log
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

Finished `dev` profile [unoptimized] target(s) in 16.70s; 16.8s; .local/bounded-reads/performance/operation-seek-packages-058.log
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

; 1.39s; .local/bounded-reads/performance/operation-seek-rust-format-058.log
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

package boundaries ok (17 packages); 1.84s; .local/bounded-reads/performance/operation-seek-boundaries-058.log
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
pnpm check:architecture
```

**Notes**

988 modules;operation-seek-architecture-058.log.
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
pnpm exec prettier --write docs/implementation-plan.md spec/src/api.md spec/src/architecture.md spec/src/cli.md spec/src/jj-integration.md spec/src/local-jj.md
```

**Notes**

Changed canonical documents formatted;operation-seek-docs-write-058.log.
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
pnpm exec prettier --write spec/src/milestones.md
```

**Notes**

Ledger formatted;operation-seek-ledger-format-058.log.
<!-- /run -->
<!-- /evidence-field -->

