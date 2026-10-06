---
schema: 1
id: E-M21-024
---

# E-M21-024 details 2

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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-jj --test stack_unchanged_checkout
```

**Notes**

stack-unchanged-checkout-before.log: missing PhysicalGuard/CheckoutWitnessV1 imports.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-service --test working_tree_checkout
```

**Notes**

stack-unchanged-checkout-guard-before.log: missing Observer::physical_guard.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
CARGO_INCREMENTAL=0 cargo clippy --locked -p jj-store-service --all-targets --no-deps -- -D warnings
```

**Notes**

stack-unchanged-checkout-service-lint.log: inherited large_enum_variant in reads and manual_is_multiple_of in stack selection; repaired internal snapshot allocation and parity expression without public-contract changes or suppressions.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-jj --test stack_unchanged_checkout
```

**Notes**

stack-unchanged-checkout-first.log: initial pointer/reopen case passes. Expanded log: nine pass, helper through parent. Staging log: final 10 pass, empty/partial/complete prefix recovery and unsafe staging rejection. Earlier wrong-path patch applied nothing; corrected before these additions.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-service --test working_tree_checkout
```

**Notes**

stack-unchanged-checkout-guard-first.log: real full scanner/provider integration passes. Guard-expanded log: three pass, binding/auth/mode/link/ignored/limit and cancellation/shared admission included.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-jj --test stack_unchanged_checkout --test stack_head_publication --test stack_source_recovery --test stack_foreground --test stack_operation_archive --test stack_reword_preparation
```

**Notes**

stack-unchanged-checkout-direct.log: 78 unique direct cases pass (10 checkout,19 publication,26 recovery,8 foreground,7 archive,8 reword). Six process helpers execute through parents.
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
JJ_STORE_WORKING_TREE_TEST_JJ=/opt/homebrew/bin/jj CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-service --test working_tree_checkout --test working_tree_scan --test working_tree_projection --test working_tree_local --test working_tree_observer --test stack_workspace_contracts -- --include-ignored
```

**Notes**

stack-unchanged-checkout-service-direct.log: 27 pass (3 checkout,6 scan,4 projection,1 local,4 observer,9 contracts). Real observer fixtures explicitly executed with pinned JJ.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-cli --test native_lifecycle read_session_
```

**Notes**

stack-unchanged-checkout-read-consumer.log: five pinned-read consumers pass: membership point reads, historical/divergent pins, staged exclusion/revocation, budgets and replaced epoch. Existing linker unwind-size warning unsuppressed.
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
CARGO_INCREMENTAL=0 cargo clippy --locked -p jj-store-service -p jj-store-jj --all-targets --no-deps -- -D warnings
```

**Notes**

stack-unchanged-checkout-lint-after.log: both owning all-target strict lints pass, inherited diagnostics repaired.
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
CARGO_INCREMENTAL=0 cargo check --locked -p jj-store-jj -p jj-store-service --all-targets
```

**Notes**

stack-unchanged-checkout-check.log: owning all-target type checks pass.
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
pnpm check:boundaries
```

**Notes**

stack-unchanged-checkout-boundaries.log: 17 packages pass
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
pnpm check:architecture
```

**Notes**

stack-unchanged-checkout-architecture.log: 1090 modules pass
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
pnpm spec:first
```

**Notes**

stack-unchanged-checkout-spec-first.log: 20 protected files/five canonical chapters pass
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
pnpm spec:validate
```

**Notes**

stack-unchanged-checkout-spec-validation.log: five validators/23 chapters/173 requirements pass
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
pnpm format:rust:check
```

**Notes**

stack-unchanged-checkout-rust-format.log: owning/runtime formatting pass
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
node --test scripts/governance.test.mjs scripts/progress-mapping.test.mjs
```

**Notes**

stack-unchanged-checkout-governance.log: 33 pass
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
CARGO_INCREMENTAL=0 pnpm docs:check
```

**Notes**

stack-unchanged-checkout-docs-check.log: 139-file closure, links/anchors and nine offline diagrams in both themes pass
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

Owning source/tests formatted; no unrelated edits.
<!-- /run -->
<!-- /evidence-field -->

