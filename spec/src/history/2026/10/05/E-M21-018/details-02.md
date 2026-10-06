---
schema: 1
id: E-M21-018
---

# E-M21-018 details 2

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
cargo test --locked -p jj-store-server --test stack_local_preparation
```

**Notes**

stack-local-preparation-before.log: missing composition import establishes the absent boundary.
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
cargo test --locked -p jj-store-server --test stack_local_preparation
```

**Notes**

stack-local-preparation-after.log: original two-case preparation/replay target passes.
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
cargo test --locked -p jj-store-server --test stack_local_preparation
```

**Notes**

stack-local-preparation-full.log: expanded target has five passes, one admission assertion failure and one subprocess helper. No error captured by initial matches assertion; later diagnostic assertion preserves the contract.
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
cargo test --locked -p jj-store-server --test stack_local_preparation admission::authorization_stale_physical_selection_and_conflicting_identity_do_not_prepare -- --exact
```

**Notes**

stack-local-preparation-admission.log: exact diagnostic rerun passes, before trusted-combination validation was added.
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
cargo test --locked -p jj-store-server --test stack_local_preparation
```

**Notes**

stack-local-preparation-full1.log: all six original/expanded cases pass; ignored helper executes through its subprocess parents.
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
cargo test --locked -p jj-store-server --test stack_local_preparation
```

**Notes**

stack-local-preparation-full2.log: eight cases pass with cancellation and persisted-evidence rejection.
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
cargo test --locked -p jj-store-server --test stack_local_preparation
```

**Notes**

stack-local-preparation-final.log: all eight cases pass after adding trusted combination precondition and stale-combination rejection; helper executes through subprocess parents.
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
cargo test --locked -p jj-store-server --test stack_local_preparation --test stack_command_journal --lib stack
```

**Notes**

stack-local-preparation-server-direct.log: filter selects four journal units but zero integrations; not integration qualification. Correct unfiltered integration command follows.
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
cargo test --locked -p jj-store-server --test stack_local_preparation --test stack_command_journal
```

**Notes**

stack-local-preparation-server-integrations.log: all 13 existing journal integrations and eight preparation integrations pass.
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
cargo test --locked -p jj-store-server --lib stack_command_journal
```

**Notes**

stack-local-preparation-journal-unit.log: four journal units pass, including real atomic-selection subprocess loss; ignored helper executes through its parent.
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
cargo test --locked -p jj-store-jj --lib --test stack_source_recovery --test stack_operation_archive --test stack_reword_preparation --test stack_foreground --test stack_preparation_retention --test local_metadata --test local_evidence
```

**Notes**

stack-local-preparation-jj-direct.log: 86 unique direct JJ cases pass (29 units/57 integrations), with subprocess helpers invoked by their passing parents.
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
cargo check --locked -p jj-store-server -p jj-store-jj --all-targets
```

**Notes**

stack-local-preparation-check.log: owning source and tests compile; not runtime authority qualification.
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
cargo clippy --locked -p jj-store-server -p jj-store-jj --all-targets --no-deps -- -D warnings
```

**Notes**

stack-local-preparation-lint.log: owning strict lint passes without suppression. Inherited service/dependency aggregate lint remains unqualified.
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
cargo fmt --all
```

**Notes**

Pinned formatting during implementation; no unrelated source changes.
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
pnpm check:boundaries
```

**Notes**

stack-local-preparation-boundaries.log: 17 owning packages pass; upstream imports remain in JJ integration.
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
pnpm check:architecture
```

**Notes**

stack-local-preparation-architecture.log: 1090 presentation modules pass; no UI ownership/lifecycle change.
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

stack-local-preparation-rust-format.log: Root/runtime Rust formatting passes.
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
pnpm exec prettier --write spec/src/architecture.md spec/src/documentation.md spec/src/jj-integration.md spec/src/local-jj.md spec/src/verification.md docs/implementation-plan.md docs/site/technical/local.md
```

**Notes**

stack-local-preparation-docs-format.log: Changed canonical/manual documents only; historical ledger excluded.
<!-- /run -->
<!-- /evidence-field -->

