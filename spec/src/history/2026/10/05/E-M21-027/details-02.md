---
schema: 1
id: E-M21-027
---

# E-M21-027 details 2

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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-cli --test stack_registered_preparation apply::newly_immutable_commit_rejects_original_prepared_apply_without_publication -- --exact
```

**Notes**

stack-apply-policy-before.log: New current immutable policy negative fails before implementation.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-cli --test stack_registered_preparation apply::newly_immutable_commit_rejects_original_prepared_apply_without_publication -- --exact
```

**Notes**

stack-apply-policy-exact.log: Original immutable-policy regression passes after retained-source policy validation.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-jj --test stack_source_recovery policy::
```

**Notes**

stack-apply-policy-provider.log: Three provider cases pass: protected/hidden/invalid policy, protected descendants, informative status and acknowledgement-only no-reannouncement.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-cli --test stack_registered_preparation policy::
```

**Notes**

stack-apply-policy-unknown.log: Two actual registered cases pass: unpublished Unknown requires current policy; already-published original can complete after policy absence and source loss.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-jj --test stack_source_recovery policy_lifecycle::
```

**Notes**

stack-apply-policy-lifecycle.log: One fixture compile diagnostic: unwrap_err required Debug on owned recovered lease. Pattern matching retains exact cancellation assertion without changing production/API.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-jj --test stack_source_recovery policy_lifecycle::
```

**Notes**

stack-apply-policy-lifecycle-after.log: Both authorization-loss and cancellation/admission/lock cases pass.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-cli --test stack_registered_preparation apply::newly_immutable_commit_rejects_original_prepared_apply_without_publication -- --exact
```

**Notes**

stack-apply-policy-exact-final.log: Exact original target passes after final bounded head/parent-edge policy charging.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-jj --test stack_unchanged_checkout --test stack_head_publication --test stack_source_recovery --test stack_foreground --test stack_operation_archive --test stack_reword_preparation
```

**Notes**

stack-apply-policy-provider-final.log: All 83 directly affected provider cases pass; six subprocess helpers run through passing parents. Provider source unchanged by subsequent server-only Published receipt guard.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-server --test stack_local_execution --test stack_command_journal --test stack_local_preparation --test stack_combinations
```

**Notes**

stack-apply-policy-server-final.log: 34 pass before final typed Published guard; final source rerun recorded below.
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

stack-apply-policy-units.log: Nine direct journal/combination units pass, two helpers through parents. Unit owners unchanged by final server-only guard.
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

stack-apply-policy-registered-final.log: 11 pass before newly discovered typed Published no-witness fault case.
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
CARGO_INCREMENTAL=0 cargo check --locked -p jj-store-jj -p jj-store-server -p jj-store-cli --all-targets
```

**Notes**

stack-apply-policy-check.log: Initial three owning package all-target lane passes before final typed Published guard; final source rerun below.
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

stack-apply-policy-lint.log: Initial three owning package all-target lane passes before final typed Published guard; final source rerun below.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-cli --test stack_registered_preparation policy::typed_published_receipt_cannot_reannounce_lost_selection_before_witness_save -- --exact
```

**Notes**

stack-apply-policy-known-before.log: Actual typed Published/Required receipt has no saved witness; raw source-head restoration caused candidate reannouncement and returned Verified. Exact negative exposes wrong historical execution behavior.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-cli --test stack_registered_preparation policy::typed_published_receipt_cannot_reannounce_lost_selection_before_witness_save -- --exact
```

**Notes**

stack-apply-policy-known-after.log: Exact new no-reannouncement target passes. Original receipt/head/pointer retained after loss; explicit fixture restoration allows original acknowledgement and completion with no current combination.
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

stack-apply-policy-registered-complete.log: Final all 12 actual registered cases pass, including both regression targets, unknown/published policy transitions and first-apply source loss/GC. Combination state remains fixture-supplied.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-server --test stack_local_execution --test stack_command_journal --test stack_local_preparation --test stack_combinations
```

**Notes**

stack-apply-policy-server-complete.log: Final 34 integrations pass after known Published guard: nine execution, 13 journal, eight preparation and four combination metadata cases. Killed subprocesses at four boundaries recover original candidate/witness; two helpers through passing parents.
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
CARGO_INCREMENTAL=0 cargo check --locked -p jj-store-jj -p jj-store-server -p jj-store-cli --all-targets
```

**Notes**

stack-apply-policy-check-complete.log: Final three owning package all-target source checks pass.
<!-- /run -->
<!-- /evidence-field -->

