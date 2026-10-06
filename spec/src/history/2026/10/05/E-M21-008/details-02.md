---
schema: 1
id: E-M21-008
---

# E-M21-008 details 2

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 12
hasNote: true
noteEncoding: markdown

```

```shell
cargo test --locked -p jj-store-jj --test working_tree_baseline
```

**Notes**

working-baseline-before.log: missing baseline module/API plus new fixture API errors; no baseline implementation existed.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 6
hasNote: true
noteEncoding: markdown

```

```shell
cargo test --locked -p jj-store-jj --test working_tree_baseline
```

**Notes**

working-baseline-before-clean.log: remaining missing module/API and fixture init hash argument. Fixture errors were repaired, not suppressed.
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
cargo test --locked -p jj-store-jj --test working_tree_baseline
```

**Notes**

working-baseline-first.log: new fixture used workspace name instead of Git hash kind.
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
cargo test --locked -p jj-store-jj --test working_tree_baseline
```

**Notes**

working-baseline-second.log repeats the same fixture init error before repair.
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
cargo test --locked -p jj-store-jj --test working_tree_baseline
```

**Notes**

working-baseline-third.log: fixture component constructor required removing ambiguous into().
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
cargo test --locked -p jj-store-jj --test working_tree_baseline
```

**Notes**

working-baseline-fourth.log: initial two baseline/ignore regressions pass.
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
cargo test --locked -p jj-store-service --test working_tree_projection --test working_tree_scan
```

**Notes**

working-projection-first.log: nine tests pass; tracked ignored edits, untracked obstacles, content vs timestamp, stale binding/rules, cancellation and existing scan regressions.
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
cargo test --locked -p jj-store-jj --test working_tree_baseline
```

**Notes**

working-baseline-regressions.log: three pass, one new historical fixture tried editing the immutable root commit. Replaced the fixture root selection with a real child commit; contract assertion retained.
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
cargo test --locked -p jj-store-jj --test working_tree_baseline
```

**Notes**

working-baseline-fixture-repair.log: four tests pass. Exact historical WC operation/view, raw binary/executable/link digests, unchanged heads, denied/missing selections, work/blob limits, submodule rejection and captured-rule precedence/cancellation.
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
cargo test --locked -p jj-store-service --test working_tree_projection --test working_tree_scan --test local_working_copy --test stack_workspace_contracts
```

**Notes**

working-projection-final.log: 22 tests pass across physical scan/projection, existing physical Save and stacks contracts.
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
JJ_STORE_WORKING_TREE_TEST_JJ=/Users/stevejuma/code/lapismd/jj-store-stack-workspace-ui/target/debug/jj cargo test --locked -p jj-store-service --test working_tree_local real_folder_comparison_preserves_operations_metadata_and_unsnapshotted_files -- --ignored --exact
```

**Notes**

working-baseline-cold-local.log: one fresh real colocated JJ folder retains all code/metadata/physical file bytes and operation heads; finds unsnapshotted changes and ignored obstacles. This is not registered HTTP/browser or executor acceptance.
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
JJ_STORE_WORKING_TREE_TEST_JJ=/Users/stevejuma/code/lapismd/jj-store-stack-workspace-ui/target/debug/jj cargo test --locked -p jj-store-service --test working_tree_local real_folder_comparison_preserves_operations_metadata_and_unsnapshotted_files -- --ignored --exact
```

**Notes**

working-baseline-cold-local-final.log: exact rerun after explicitly setting the controlled fixture repository-exclude policy empty; one test passes and disposable TempDir cleans up.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
cargo clippy --locked -p jj-store-jj --lib -- -D warnings
```

**Notes**

working-baseline-lint.log: the same four existing diagnostics in git_read/extras.rs, references/prepared.rs, query/cache/resident.rs and query/index.rs. All owning files byte-identical to e687b039. No new source lint exception or waiver.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
cargo clippy --locked -p jj-store-service --no-deps --lib -- -D warnings
```

**Notes**

working-projection-lint.log: the same two existing reads.rs large enum and stack_workspace/selection.rs manual multiple diagnostics, both byte-identical to e687b039. Strict lint remains incomplete; this inspection is not a replacement passing lane.
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
cargo check --locked -p jj-store-jj -p jj-store-service --lib
```

**Notes**

working-baseline-package-check.log: both affected packages compile; no Rust workspace aggregate run.
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

working-baseline-boundaries.log: 17 package boundaries pass. No upstream JJ import in service policy.
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
cargo fmt --all
```

**Notes**

Formatted changed Rust implementation/tests; status confirms no unrelated Rust edits.
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
cargo fmt --all -- --check
```

**Notes**

working-baseline-format-rust.log passes.
<!-- /run -->
<!-- /evidence-field -->

