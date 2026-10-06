---
schema: 1
id: E-M21-030
---

# E-M21-030 details 2

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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-service --test working_tree_inventory
```

**Notes**

physical-inventory-before.log: Exact initial target fails with two missing API diagnostics, before implementation.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-service --test working_tree_inventory
```

**Notes**

physical-inventory-after-initial.log: Initial roundtrip case passes.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-service --test working_tree_inventory
```

**Notes**

physical-inventory-after.log: Expanded eight-case target passes.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-service --test working_tree_inventory --test working_tree_checkout --test working_tree_scan --test working_tree_projection --test working_tree_local --test working_tree_observer
```

**Notes**

physical-inventory-service-regressions.log: 22 pass: inventory 9, checkout guards 3, scanner 6 and projection 4. Five explicitly gated local/observer cases were ignored here and executed separately below.
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
pnpm test:governance
```

**Notes**

physical-inventory-governance.log: 50 cases pass.
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
pnpm check:boundaries
```

**Notes**

physical-inventory-boundaries.log: 17-package boundaries pass; integration data stays owned.
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
pnpm check:architecture
```

**Notes**

physical-inventory-architecture.log: 1090 source modules pass.
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
CARGO_INCREMENTAL=0 cargo clippy --locked -p jj-store-jj -p jj-store-service -p jj-store-server -p jj-store-cli --all-targets --no-deps -- -D warnings
```

**Notes**

physical-inventory-lint.log: One chunks_exact_to_as_chunks diagnostic; repaired with checked as_chunks framing, without suppressing lint.
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
CARGO_INCREMENTAL=0 cargo clippy --locked -p jj-store-jj -p jj-store-service -p jj-store-server -p jj-store-cli --all-targets --no-deps -- -D warnings
```

**Notes**

physical-inventory-lint-final.log: Final strict owning/direct-consumer all-target lint passes.
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
CARGO_INCREMENTAL=0 JJ_STORE_WORKING_TREE_TEST_JJ=/Users/stevejuma/code/lapismd/jj-store-stack-workspace-ui/target/debug/jj cargo test --locked -p jj-store-service --test working_tree_local --test working_tree_observer -- --ignored
```

**Notes**

physical-inventory-real-folder.log: All five explicit real-folder fixtures pass against the existing isolated development executable. No installed binary or repository is replaced.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-jj --test stack_tree_checkout --test stack_unchanged_checkout
```

**Notes**

physical-inventory-provider.log: 28 provider parents pass: 18 tree-plan and 10 unchanged-checkout. Two isolated process helpers are invoked by their passing parents.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-service --lib working_tree
```

**Notes**

physical-inventory-contracts.log: One exact service contract case passes; 15 unrelated unit cases filtered.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-server --test stack_local_execution --test stack_local_preparation
```

**Notes**

physical-inventory-server.log: 17 private server execution/preparation parents pass: 9 execution and 8 preparation. Isolated workers are invoked by passing parents.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-cli --test stack_checkout_plans --test stack_registered_preparation
```

**Notes**

physical-inventory-cli.log: 25 direct CLI parents pass: 13 saved-plan and 12 registered-folder cases. Two saved-plan process workers are invoked by passing parents.
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
CARGO_INCREMENTAL=0 cargo check --locked -p jj-store-jj -p jj-store-service -p jj-store-server -p jj-store-cli --all-targets
```

**Notes**

physical-inventory-check.log: Owning provider/service/server and direct CLI all-target check passes.
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
cargo fmt --all
```

**Notes**

source edits: Rust formatting during implementation succeeds; unrelated tracked files unchanged.
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
pnpm format:rust:check
```

**Notes**

physical-inventory-rust-format.log: Rust formatting passes.
<!-- /run -->

<!-- run -->
### Run 18

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
target/debug/jj --version
```

**Notes**

version probe: This development CLI does not support the flag; failed probe did not change files or installed binaries.
<!-- /run -->
<!-- /evidence-field -->

