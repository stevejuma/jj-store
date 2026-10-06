---
schema: 1
id: E-M21-031
---

# E-M21-031 details 2

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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-cli --test stack_checkout_plans inventory::saved_complete_inventory_is_required_for_changed_tree_publication -- --exact
```

**Notes**

saved-inventory-before.log: Exact new regression fails before implementation with two missing save/load API diagnostics.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-cli --test stack_checkout_plans inventory::saved_complete_inventory_is_required_for_changed_tree_publication -- --exact
```

**Notes**

saved-inventory-after-initial.log: First implementation fails with one unresolved public MAX_INVENTORY_BYTES import; fixed by exporting the owning constant instead of duplicating the bound.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-cli --test stack_checkout_plans
```

**Notes**

saved-inventory-expanded.log: The queued expanded attempt also reports the same unresolved byte-bound export before the repair; no runtime test result.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-cli --test stack_checkout_plans
```

**Notes**

saved-inventory-after.log: First complete implementation run passes 19 parents and three ignored invoked workers, including inventory/reference subprocess boundaries.
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

saved-inventory-final.log: Repeated complete 19-parent pass after process coverage and store fault additions. Three subprocess helpers invoked by passing parents.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-server --lib stack_checkout_plans
```

**Notes**

saved-inventory-store-faults.log: Four metadata-only store cases pass, including new unknown inventory acknowledgement and existing replacement/aggregate-capacity coverage.
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

saved-inventory-final-expanded.log: 20 final CLI parents pass after the additional inventory-contention regression; three subprocess workers invoked by parents. Seven new parents plus 13 existing plan cases.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-server --test stack_command_journal --test stack_local_preparation --test stack_local_execution --test stack_combinations
```

**Notes**

saved-inventory-server-regressions.log: 34 focused existing server parents pass; isolated workers invoked by passing parents.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-cli --test stack_registered_preparation
```

**Notes**

saved-inventory-registered-regressions.log: 12 actual registered-folder/private reword parents pass; no production changed-tree execution is inferred.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-server --lib stack_
```

**Notes**

saved-inventory-unit-regressions.log: 13 journal/combination/plan metadata parents pass, including the four store cases; two isolated workers invoked by parents.
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
CARGO_INCREMENTAL=0 cargo check --locked -p jj-store-jj -p jj-store-service -p jj-store-server -p jj-store-cli --all-targets
```

**Notes**

saved-inventory-check.log: Owning/direct-consumer all-target type check passes.
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
CARGO_INCREMENTAL=0 cargo clippy --locked -p jj-store-jj -p jj-store-service -p jj-store-server -p jj-store-cli --all-targets --no-deps -- -D warnings
```

**Notes**

saved-inventory-lint.log: Strict owning/direct-consumer all-target lint passes.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-service --test working_tree_inventory --test working_tree_checkout
```

**Notes**

saved-inventory-service-regressions.log: 12 complete inventory/registered guard parents pass.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-jj --test stack_tree_checkout
```

**Notes**

saved-inventory-provider-regressions.log: 18 exact retained-tree preparation parents pass; killed-worker helper invoked by its parent.
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
pnpm test:governance
```

**Notes**

saved-inventory-governance.log: 50 cases pass.
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

saved-inventory-boundaries.log: 17-package boundaries pass.
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

saved-inventory-architecture.log: 1090 web/source ownership modules pass.
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
cargo fmt --all
```

**Notes**

source formatting: Repeated after source/test additions; unrelated tracked files unchanged.
<!-- /run -->
<!-- /evidence-field -->

