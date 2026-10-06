---
schema: 1
id: E-M21-019
---

# E-M21-019 details 2

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
cargo test --locked -p jj-store-cli --test stack_registered_preparation
```

**Notes**

stack-registered-preparation-before.log: one missing import diagnostic for three authority APIs; no new implementation existed.
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
cargo test --locked -p jj-store-cli --test stack_registered_preparation
```

**Notes**

stack-registered-preparation-after.log: two original actual registry/scanner cases pass.
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
cargo test --locked -p jj-store-cli --test stack_registered_preparation
```

**Notes**

stack-registered-preparation-full.log: all three cases pass, including interrupted pending archive recovery through real registration after op-store/index loss and Git GC.
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
cargo test --locked -p jj-store-jj --test stack_reword_preparation
```

**Notes**

stack-registered-preparation-policy.log: seven pass; added immutable-ancestry case names the hidden pre-snapshot parent. Corrected to the actual @- identity from the owning stock-JJ fixture without weakening policy.
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
cargo test --locked -p jj-store-jj --test stack_reword_preparation
```

**Notes**

stack-registered-preparation-policy-final.log: all eight cases pass, including protected ancestors, unrelated/duplicate heads and unchanged operation/view inventory.
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
cargo test --locked -p jj-store-jj --lib --test stack_reword_preparation --test stack_operation_archive --test stack_source_recovery
```

**Notes**

stack-registered-preparation-jj-direct.log: 53 unique direct source cases pass (29 units/24 integrations), with subprocess helper executed by its passing parent.
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
cargo test --locked -p jj-store-server --test stack_local_preparation --test stack_command_journal
```

**Notes**

stack-registered-preparation-server-direct.log: 13 journal and eight real preparation cases pass after trusted immutable-head input integration; ignored helper executes in parent subprocess cases.
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
cargo test --locked -p jj-store-server --lib stack_command_journal
```

**Notes**

stack-registered-preparation-journal-unit.log: all four journal units pass; process-loss helper is invoked by its passing parent.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
cargo test --locked -p jj-store-cli --test stack_registered_preparation --test local_browser
```

**Notes**

stack-registered-preparation-cli-direct.log: 28 ordinary local read/registration consumers pass and two external large-repository cases remain ignored. New Stacks suite has two passes/one stale-generation fixture assertion after removal; corrected the scenario placement before final rerun.
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
cargo check --locked -p jj-store-jj -p jj-store-server -p jj-store-cli --all-targets
```

**Notes**

stack-registered-preparation-check.log: disk capacity exhausted while writing native_lifecycle incremental cache; no product diagnostic. Package-scoped generated-cache cleanup and nonincremental rerun follow.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
cargo test --locked -p jj-store-cli --test stack_registered_preparation
```

**Notes**

stack-registered-preparation-final.log: no space writing test fingerprint; no test ran. Nonincremental exact rerun follows after cleanup.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
cargo clippy --locked -p jj-store-jj -p jj-store-server -p jj-store-cli --all-targets --no-deps -- -D warnings
```

**Notes**

stack-registered-preparation-lint.log: disk capacity prevents CLI fingerprint creation; no source lint diagnostic reached. Nonincremental rerun follows.
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

stack-registered-preparation-docs-check.log: owning generated CLI build fails with disk capacity, preventing manual qualification. Exact rerun follows after generated-cache cleanup.
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

stack-registered-preparation-boundaries.log: All 17 package boundaries pass; no upstream JJ import outside owning integration/CLI.
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

stack-registered-preparation-architecture.log: All 1090 presentation modules pass; no production UI change.
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

stack-registered-preparation-rust-format.log: Root/runtime Rust formatting passes.
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
pnpm exec prettier --write spec/src/architecture.md spec/src/cli.md spec/src/documentation.md spec/src/jj-integration.md spec/src/local-jj.md docs/implementation-plan.md docs/site/technical/local.md docs/site/technical/lifecycle.md
```

**Notes**

stack-registered-preparation-docs-format.log: Changed owning canonical/manual documents formatted; ledger excluded.
<!-- /run -->
<!-- /evidence-field -->

