---
schema: 1
id: E-M21-029
---

# E-M21-029 details 2

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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-cli --test stack_checkout_plans
```

**Notes**

checkout-plan-store-before.log: Missing private server module/export; exact new target cannot compile before implementation.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-cli --test stack_checkout_plans
```

**Notes**

checkout-plan-store-after-initial.log: Initial missing-store open case passes after adding the API.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-cli --test stack_checkout_plans
```

**Notes**

checkout-plan-store-after.log: Nine real-native/input cases pass; ignored subprocess helper is invoked by its passing parent.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-server --lib stack_checkout_plans
```

**Notes**

checkout-plan-store-faults.log: Three metadata-only pressure/replacement/unknown-ack cases pass. Synthetic framing and pressure data are not engine evidence.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-server --test stack_command_journal --test stack_local_preparation --test stack_local_execution --test stack_combinations
```

**Notes**

checkout-plan-journal-regressions.log: Initial existing reword retention-mismatch case fails inside fixture preparation with Journal(Unavailable); 4 combinations, 13 journal and eight execution cases pass, one execution fails, remaining family not run. No assertion or source change made to resolve this transient run.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-server --test stack_local_execution journal_retention_mismatch_is_rejected_before_publication -- --exact
```

**Notes**

checkout-plan-retention-exact.log: Exact failed target passes unchanged. Later full focused family also passes.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-cli --test stack_checkout_plans
```

**Notes**

checkout-plan-store-final.log: Twelve new parents pass, two invoked subprocess helpers ignored in the main run.
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
CARGO_INCREMENTAL=0 cargo check --locked -p jj-store-jj -p jj-store-server -p jj-store-cli --all-targets
```

**Notes**

checkout-plan-check.log: Owning provider/server and direct CLI all-target check passes.
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
pnpm test:governance
```

**Notes**

checkout-plan-governance.log: 50 cases pass.
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
pnpm check:boundaries
```

**Notes**

checkout-plan-boundaries.log: 17-package boundaries pass; no new upstream JJ dependency in server.
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
pnpm check:architecture
```

**Notes**

checkout-plan-architecture.log: 1090 source modules pass.
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
cargo fmt --all
```

**Notes**

source edits: Repeated after Rust implementation and test additions; all invocations succeed and unrelated tracked files remain unchanged.
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
CARGO_INCREMENTAL=0 cargo clippy --locked -p jj-store-jj -p jj-store-server -p jj-store-cli --all-targets --no-deps -- -D warnings
```

**Notes**

checkout-plan-lint.log: Strict owning/direct-consumer all-target lint passes, including final test additions.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-server --test stack_command_journal --test stack_local_preparation --test stack_local_execution --test stack_combinations
```

**Notes**

checkout-plan-journal-regressions-final.log: All 34 focused existing cases pass: combinations 4, journal 13, reword execution 9 and preparation 8. Process helpers invoked by passing parents.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-cli --test stack_checkout_plans --test stack_registered_preparation
```

**Notes**

checkout-plan-cli-regressions-final.log: 13 new real-native/input parents and 12 existing actual registered/observer reword parents pass; the two new isolated workers are invoked by passing parents.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-jj --test stack_tree_checkout
```

**Notes**

checkout-plan-provider-final.log: All 18 existing exact changed-tree provider parents pass, including killed worker and cancellation lifetime.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-server --lib stack_
```

**Notes**

checkout-plan-unit-final.log: All 12 journal/combination/plan metadata unit parents pass; two isolated workers invoked by passing parents. Includes 3 new store tests.
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
pnpm format:rust:check
```

**Notes**

checkout-plan-rust-format.log: Rust formatting passes.
<!-- /run -->
<!-- /evidence-field -->

