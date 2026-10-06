---
schema: 1
id: E-M21-020
---

# E-M21-020 details 2

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-server --test stack_combinations
```

**Notes**

stack-combinations-before.log: Absent API plus module path/native-ID-width fixture diagnostics; no state owner existed.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-server --test stack_combinations
```

**Notes**

stack-combinations-after.log: One remaining native-ID-width fixture diagnostic; fixed width to the actual 64-byte native identity.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-server --test stack_combinations
```

**Notes**

stack-combinations-after2.log: Two original owner integrations pass.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-server --test stack_combinations --lib stack_combinations
```

**Notes**

stack-combinations-faults.log: Two fault units pass; integration functions did not match this filter and zero integration cases ran. Exact unfiltered integration command follows.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-server --lib stack_combinations
```

**Notes**

stack-combinations-units.log: Three pass; stale preview fixture failed prepared identity validation before staleness. Corrected its prepared digest; two unused fixture warnings also repaired by actual physical/target assertions.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-server --lib stack_combinations
```

**Notes**

stack-combinations-units2.log: Four pass; repeated same stale fixture diagnostic before repair.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-server --lib stack_combinations
```

**Notes**

stack-combinations-units3.log: Misplaced physical-state assertion names absent local variable; relocated without weakening unchanged-source assertion.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-server --lib stack_combinations
```

**Notes**

stack-combinations-units4.log: Five units pass; ignored process-loss helper is invoked by its passing parent.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-server --test stack_combinations
```

**Notes**

stack-combinations-final.log: Two exact owner integrations pass after generation/source-tree checks.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-server --test stack_combinations
```

**Notes**

stack-combinations-final2.log: Four exact integrations pass, including rejected replay and actual reserved code through source operation/index loss and Git GC.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-server --lib stack_combinations
```

**Notes**

stack-combinations-units-final.log: Five final units pass after authority module extraction; child helper exits at exact device-state selection.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-cli --test stack_registered_preparation
```

**Notes**

stack-combinations-registered.log: Three actual registered-folder cases pass, adding exact durable combination operation/WIP mismatch rejection before candidates.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-server --test stack_local_preparation --test stack_command_journal
```

**Notes**

stack-combinations-direct.log: 13 existing journal and eight real preparation cases pass; subprocess helper exercised through parent.
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
CARGO_INCREMENTAL=0 cargo check --locked -p jj-store-server -p jj-store-cli --all-targets
```

**Notes**

stack-combinations-check.log: Affected owning package targets pass.
<!-- /run -->

<!-- run -->
### Run 15

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

stack-combinations-lint.log: Two unused_unit diagnostics in new selection matcher, removed without suppression.
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

stack-combinations-boundaries.log: 17 package boundaries pass.
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

stack-combinations-architecture.log: 1090 presentation modules pass; no production UI change.
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

stack-combinations-rust-format.log: Owning/runtime Rust formatting passes.
<!-- /run -->
<!-- /evidence-field -->

