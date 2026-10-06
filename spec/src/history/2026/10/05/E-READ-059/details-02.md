---
schema: 1
id: E-READ-059
---

# E-READ-059 details 2

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
cargo test -p jj-store-jj --lib --locked query::prepared::operations::history::tests::historical_operation_continuations_resume_without_replaying_prior_edges -- --exact --nocapture (before)
```

**Notes**

Unchanged E-READ-058 production repeats earlier compact edges from historical node62 toward deep anchor3 and fails the96-unit continuation budget. operation-checkpoint-before-059.log.
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
cargo test -p jj-store-jj --lib --locked query::prepared::operations::history::tests::historical_operation_continuations_resume_without_replaying_prior_edges -- --exact --nocapture (initial implementation)
```

**Notes**

Initial checkpoint implementation still fails96-unit target because it pessimistically charges32 cache comparisons when only one checkpoint exists. Corrected to charge the bounded actual entry count and gave each saved checkpoint an independent pre-growth metadata lease. operation-checkpoint-after-059.log.
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
cargo test -p jj-store-jj --lib --locked query::prepared::operations::history::tests::historical_operation_continuations_resume_without_replaying_prior_edges -- --exact --nocapture (corrected implementation)
```

**Notes**

Same fixed96-unit regression passes twice for the saved historical continuation. Metadata-filter mismatch and changed anchor are rejected. operation-checkpoint-after-corrected-059.log.
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
cargo test -p jj-store-jj --lib --locked query::prepared::operations::history -- --nocapture (strengthened units)
```

**Notes**

4 units pass;32 DAGs with37 nodes,every historical start and all selected anchors now compare paged chunk/checkpoint output against pinned ordinary JJ. Retention never exceeds32 checkpoints;missing checkpoint falls back to identical full output. Final fixture checks altered source anchor while the checkpoint is still warm. operation-checkpoint-units-059.log.
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
cargo test -p jj-store-jj --lib --locked query::prepared::operations::history -- --nocapture
```

**Notes**

Finished `test` profile [unoptimized] target(s) in 6.45s; test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 57 filtered out; finished in 0.68s; 7.67s; .local/bounded-reads/performance/operation-checkpoint-topology-exact-059.log
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
cargo test -p jj-store-cli --test local_browser --locked prepared_operation_history::prepared_operation_history_skips_prior_bodies_for_filtered_and_historical_pages -- --exact --nocapture
```

**Notes**

Finished `test` profile [unoptimized] target(s) in 1m 25s; test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 38 filtered out; finished in 17.44s; 105.77s; .local/bounded-reads/performance/operation-checkpoint-local-exact-059.log
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
cargo test -p jj-store-cli --test native_lifecycle --locked graph_acceptance::operation_history_acceptance::prepared_native_operation_history_hydrates_only_deep_anchor_and_page_candidates -- --exact --nocapture
```

**Notes**

Finished `test` profile [unoptimized] target(s) in 33.01s; test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 227 filtered out; finished in 4.94s; 40.05s; .local/bounded-reads/performance/operation-checkpoint-native-exact-059.log
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

Finished `test` profile [unoptimized] target(s) in 0.70s; test result: ok. 61 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 6.40s; 7.19s; .local/bounded-reads/performance/operation-checkpoint-owning-jj-059.log
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
cargo test -p jj-store-cli --test local_browser --locked -- --nocapture
```

**Notes**

Finished `test` profile [unoptimized] target(s) in 17.86s; test result: ok. 37 passed; 0 failed; 2 ignored; 0 measured; 0 filtered out; finished in 49.66s; 70.49s; .local/bounded-reads/performance/operation-checkpoint-local-readers-059.log
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
cargo test -p jj-store-cli --test native_lifecycle --locked graph_acceptance -- --nocapture
```

**Notes**

Finished `test` profile [unoptimized] target(s) in 15.08s; test result: ok. 44 passed; 0 failed; 0 ignored; 0 measured; 184 filtered out; finished in 16.87s; 33.93s; .local/bounded-reads/performance/operation-checkpoint-native-graphs-059.log
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
cargo test -p jj-store-cli --test native_lifecycle --locked reference -- --nocapture
```

**Notes**

Finished `test` profile [unoptimized] target(s) in 16.22s; test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 220 filtered out; finished in 53.84s; 72.03s; .local/bounded-reads/performance/operation-checkpoint-native-references-059.log
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

Finished `dev` profile [unoptimized] target(s) in 25.41s; 25.54s; .local/bounded-reads/performance/operation-checkpoint-packages-059.log
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

; 1.04s; .local/bounded-reads/performance/operation-checkpoint-rust-format-059.log
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

package boundaries ok (17 packages); 1.33s; .local/bounded-reads/performance/operation-checkpoint-boundaries-059.log
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
pnpm check:architecture
```

**Notes**

988 modules;operation-checkpoint-architecture-059.log.
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

Changed canonical and ledger documents formatted;operation-checkpoint-ledger-format-059.log.
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

8 protected files,6 canonical chapters;operation-checkpoint-spec-first-059.log.
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

5 validators,23 chapters,172 requirements;operation-checkpoint-spec-validation-059.log.
<!-- /run -->
<!-- /evidence-field -->

