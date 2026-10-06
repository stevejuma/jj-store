---
schema: 1
id: E-M21-034
---

# E-M21-034 details 2

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-server --test stack_combinations
```

**Notes**

tool baseline output: Four legacy metadata-owner cases pass before edits; no native-completion/combination join existed.
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
CARGO_INCREMENTAL=0 cargo check --locked -p jj-store-server --all-targets
```

**Notes**

tool initial check output: Owning all-target type check passes during initial implementation.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-server --test stack_combinations original_combination_binding_is_durable_but_never_a_success_receipt -- --exact
```

**Notes**

tool exact binding output: First original binding/reopen/pending-state target passes.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-cli --test stack_checkout_plans coordination::native_completion_remains_pending_until_exact_combination_selection -- --exact
```

**Notes**

tool exact native join output: First actual native/scanner proof remains pending until original combination selection; exact replay passes.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-cli --test stack_checkout_plans coordination -- --nocapture
```

**Notes**

combination-join-native.log: Four real native/scanner coordination parents pass, including an isolated process-loss parent at native/evidence/final acknowledgement boundaries.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-server --test stack_combinations
```

**Notes**

combination-join-store.log: Seven metadata-owner parents pass before the additional bound rejection regression.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-server --lib stack_command_journal::combination::tests
```

**Notes**

combination-join-protocol.log: Three new metadata parents initially fail during fixture retention because the changed preview reused an already occupied request slot. Corrected the fixture request identity; no guard or assertion was relaxed.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-server --lib stack_command_journal::combination::tests
```

**Notes**

combination-join-protocol-repaired.log: All three exact metadata protocol parents pass after the fixture identity repair, including process loss after combination selection and unknown final receipt sync.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-cli --test stack_checkout_plans coordination_unchanged::actual_reword_checkout_requires_combination_before_final_acknowledgement -- --exact
```

**Notes**

combination-join-unchanged.log: Actual retained reword and native unchanged pointer proof cannot use legacy direct acknowledgement when bound; ordered selection, exact replay and stock-JJ consumption pass. Historical metadata bootstrap is not setup-engine evidence.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-server --test stack_combinations bound_unpublished_rejection_discards_only_the_matching_pending_update -- --exact
```

**Notes**

combination-join-bound-rejection.log: Exact cancellation target passes: a bound unpublished rejected update releases only its matching pending combination.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
CARGO_INCREMENTAL=0 cargo clippy --locked -p jj-store-server -p jj-store-cli --all-targets -- -D warnings
```

**Notes**

combination-join-lint.log: Four diagnostics: two unneeded unit expressions and two duplicate test-module loads. Fixed the expressions and reused the existing test modules without suppressions.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
CARGO_INCREMENTAL=0 cargo clippy --locked -p jj-store-server -p jj-store-cli --all-targets --no-deps -- -D warnings
```

**Notes**

combination-join-lint-repaired.log: Two now-unused test imports after module reuse; removed without behavioral changes.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm docs:check
```

**Notes**

combination-join-docs-check.log: Final shared unit fixture edits changed the configuration source fingerprint after its first review; refreshed the review against final source.
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
pnpm docs:review configuration 'Reviewed final original combination binding and provider-proof acknowledgement code, shared unit fixtures and exact private-state loss boundaries. Documented journal version 6, unchanged historical formats, pending native evidence, journal-first locking, final combination sync before success, and the separate setup/merge/WIP and production transport gates.'
```

**Notes**

combination-join-docs-review-final.log: Final source and matching documentation reviewed after lint repairs.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-cli --test stack_checkout_plans
```

**Notes**

combination-join-final-plans.log: 48 passing parents. Complete directly affected plan/inventory/installation/native-completion and coordination family; isolated helpers invoked by passing parents.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-server --test stack_combinations --test stack_command_journal --test stack_local_preparation --test stack_local_execution
```

**Notes**

combination-join-server-regressions.log: 38 passing parents. Complete directly affected journal, combination, preparation and private reword runtime integration families.
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

combination-join-final-unit.log: 16 passing parents. All private journal/combination/plan metadata unit parents, including selected-before-ack process loss and unknown sync. Explicit synthetic completion is metadata evidence only.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-cli --test stack_registered_preparation
```

**Notes**

combination-join-registered-regressions.log: 12 passing parents. Existing actual registered-folder preparation/execution parents; real registry checks remain distinct from fixture combination engines.
<!-- /run -->
<!-- /evidence-field -->

