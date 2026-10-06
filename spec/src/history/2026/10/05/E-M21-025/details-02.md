---
schema: 1
id: E-M21-025
---

# E-M21-025 details 2

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-server --test stack_local_execution
```

**Notes**

stack-local-execution-before.log: two missing apply_reword calls and one absent saved checkout field. Establishes the missing runtime/journal join.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-server --test stack_local_execution
```

**Notes**

stack-local-execution-first.log: one actual retained execution/reopen case passes. Expanded log: three cases pass. Lifecycle log: six pass including four killed-process boundaries. Final log: seven pass with immutable/corrupt saved-witness cases. Binding log: nine pass after source/index repair and exact retained commit/archive/candidate comparison.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-server --test stack_command_journal --test stack_local_preparation --test stack_combinations
```

**Notes**

stack-local-execution-direct.log: two combination metadata fixture failures at proof-free execution transitions. Production proof gate retained; fixtures now explicitly load historical V2 outcome state, preserving the existing metadata-owner assertions without claiming Setup/checkout execution.
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
CARGO_INCREMENTAL=0 cargo clippy --locked -p jj-store-server -p jj-store-jj --all-targets --no-deps -- -D warnings
```

**Notes**

stack-local-execution-lint.log: one unique collapsible_if diagnostic in saved checkout receipt validation. Collapsed the guard, retained legacy frozen completed receipts and added exact malformed/immutable witness coverage; no suppression.
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
CARGO_INCREMENTAL=0 cargo clippy --locked -p jj-store-server -p jj-store-jj --all-targets --no-deps -- -D warnings
```

**Notes**

stack-local-execution-lint-after.log: owning strict lint passes after validation repair; final larger owning lane includes final identity guard and CLI consumer.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-server --test stack_local_execution --test stack_command_journal --test stack_local_preparation --test stack_combinations
```

**Notes**

stack-local-execution-direct-after.log: 30 pass (5 execution/13 journal/8 preparation/4 combinations), helper through parent. Complete log: 33 pass after process-loss and source-loss additions. Final server lane recorded separately after exact retention guard.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-server --test stack_local_execution saved_pending_checkout_repairs_lost_source_dependencies_without_repreparing -- --exact
```

**Notes**

stack-local-execution-source-loss.log: saved canonical witness and original preview persist through real op_store/index removal, explicit reconstruction and pointer completion without re-preparation.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-server --test stack_local_execution journal_retention_mismatch_is_rejected_before_publication -- --exact
```

**Notes**

stack-local-execution-retention.log: altered journal retention commit or archive digest rejects before head/pointer publication and retains Prepared. Added following source review of the key-only provider/journal linkage; no earlier behavioral failure result claimed.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-server --lib stack_
```

**Notes**

stack-local-execution-units.log: nine complete direct journal/combination units pass; two ignored process helpers execute through parents. Existing linker __eh_frame warning unsuppressed.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-jj --test stack_unchanged_checkout
```

**Notes**

stack-local-execution-provider.log: 10 provider cases pass after witness linkage accessors; final broader recovery family reruns after retained identity capture.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-cli --test stack_registered_preparation
```

**Notes**

stack-local-execution-registered.log and registered-final.log: four actual registered-folder cases pass. New retained execution uses real device registration, CLI folder provider, Admin policy and full physical guard; combination authority remains an isolated fixture. Current/terminal retry preserves later editor bytes.
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
CARGO_INCREMENTAL=0 cargo clippy --locked -p jj-store-server -p jj-store-jj -p jj-store-cli --all-targets --no-deps -- -D warnings
```

**Notes**

stack-local-execution-lint-final.log: all three owning package strict all-target lints pass before final retained identity capture; final source rerun remains to record.
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
CARGO_INCREMENTAL=0 cargo check --locked -p jj-store-server -p jj-store-jj -p jj-store-cli --all-targets
```

**Notes**

stack-local-execution-check.log: all three owning package all-target checks pass before final retained identity capture; final source rerun remains to record.
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

stack-local-execution-boundaries.log: 17 packages pass
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

stack-local-execution-architecture.log: 1090 modules pass
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
pnpm spec:first
```

**Notes**

stack-local-execution-spec-first.log: 16 protected files/five chapters pass before final identity fields and append
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
pnpm spec:validate
```

**Notes**

stack-local-execution-spec-validation.log: five validators/23 chapters/173 requirements pass
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

stack-local-execution-rust-format.log: owning/runtime formatting pass
<!-- /run -->
<!-- /evidence-field -->

