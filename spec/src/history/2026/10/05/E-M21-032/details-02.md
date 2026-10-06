---
schema: 1
id: E-M21-032
---

# E-M21-032 details 2

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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-cli --test stack_checkout_plans installation::retained_tree_files_install_with_durable_progress_and_unchanged_native_metadata -- --exact
```

**Notes**

tree-install-before.log: Exact new test fails with the missing install_tree_files method.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-cli --test stack_checkout_plans installation::retained_tree_files_install_with_durable_progress_and_unchanged_native_metadata -- --exact
```

**Notes**

tree-install-after.log: Initial exact real candidate/scanner installation passes, preserving native checkout/tree-state bytes.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-cli --test stack_checkout_plans installation::
```

**Notes**

tree-install-expanded.log: Five installation parents pass: replay, file forms, collision and ignored-file guards.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-cli --test stack_checkout_plans installation
```

**Notes**

tree-install-process.log: Six parents pass; isolated worker invoked at ten before/after physical and prepared/final progress boundaries.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-cli --test stack_checkout_plans
```

**Notes**

tree-install-all-plans.log: 30 parents pass including newly added replacement, backup, revocation and cancellation faults; four invoked worker helpers.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
CARGO_INCREMENTAL=0 cargo clippy --locked -p jj-store-jj -p jj-store-service -p jj-store-server -p jj-store-cli --all-targets --no-deps -- -D warnings
```

**Notes**

tree-install-lint.log: Three collapsible-if diagnostics in new provider source; repaired without weakening checks.
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
CARGO_INCREMENTAL=0 cargo clippy --locked -p jj-store-jj -p jj-store-service -p jj-store-server -p jj-store-cli --all-targets --no-deps -- -D warnings
```

**Notes**

tree-install-lint-final.log: One collapsible-if diagnostic in the test guard; repaired.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-cli --test stack_checkout_plans installation
```

**Notes**

tree-install-final-focus.log: Compilation reports platform RawMode u16 versus u32 cast in the staging chmod; use inferred Rustix RawMode to retain platform portability. No test result.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-cli --test stack_checkout_plans
```

**Notes**

tree-install-final-plans.log: 33 parents pass including exact initial-progress prefix, partial candidate and missing terminal progress; subsequent next-frame hardening is checked below.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-cli --test stack_checkout_plans
```

**Notes**

tree-install-final-expanded.log: 32 parents pass; existing inventory_process::process_loss_on_each_side_of_inventory_reference_retains_exact_original fails at journal save with Unavailable (line 75). No assertion or implementation is weakened. Exact rerun and final complete family below pass; cause remains unestablished.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-cli --test stack_checkout_plans inventory_process::process_loss_on_each_side_of_inventory_reference_retains_exact_original -- --exact
```

**Notes**

tree-install-inventory-loss-recheck.log: Exact unchanged inventory-journal process-loss parent passes after the parallel-run failure.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-cli --test stack_checkout_plans
```

**Notes**

tree-install-verified-plans.log: Final complete family passes all 34 parents, including the added partial move acknowledgement. All four helpers are invoked by parents; no changed assertion, concurrency setting or recovery contract.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-cli --test stack_checkout_plans installation_process::partial_move_acknowledgement_replays_only_exact_saved_next_prefix -- --exact
```

**Notes**

tree-install-partial-move-ack.log: Additional exact parent passes both valid and corrupt partial acknowledgement after an actual killed backup move. 34 unique CLI parents in total.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-jj --test stack_tree_checkout --test stack_unchanged_checkout --test stack_head_publication
```

**Notes**

tree-install-provider-regressions.log: 47 existing direct provider parents pass (18 preparation, 10 unchanged checkout, 19 head publication); subprocess helpers invoked by parents.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-service --test working_tree_inventory --test working_tree_checkout
```

**Notes**

tree-install-service-regressions.log: 13 service parents pass: original validation and optional fresh observation retain authorization, complete scope, cancellation and shared scanner admission.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-cli --test stack_registered_preparation
```

**Notes**

tree-install-runtime-regressions.log: 12 registered-folder/private runtime parents pass; synthetic tree-install test combinations do not qualify the combination engine.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-server --test stack_local_execution
```

**Notes**

tree-install-server-regressions.log: 9 existing private reword execution parents pass, including original journal receipts and interrupted subprocess boundaries.
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
CARGO_INCREMENTAL=0 cargo check --locked -p jj-store-jj -p jj-store-service -p jj-store-server -p jj-store-cli --all-targets
```

**Notes**

tree-install-check.log: Owning/direct-consumer all-target type check passes.
<!-- /run -->
<!-- /evidence-field -->

