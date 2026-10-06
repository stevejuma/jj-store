---
schema: 1
id: E-M21-022
---

# E-M21-022 details 2

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 6
hasNote: true
noteEncoding: markdown

```

```shell
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-jj --test stack_source_recovery indexes::
```

**Notes**

stack-source-indexes-before.log: Missing explicit API and fixture API diagnostics.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-jj --test stack_source_recovery indexes::
```

**Notes**

stack-source-indexes-after.log: ObjectId trait import missing in archive Source; repaired.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-jj --test stack_source_recovery indexes::
```

**Notes**

stack-source-indexes-after2.log: Generic native Operation conversion inference needed explicit owning type.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-jj --test stack_source_recovery indexes::
```

**Notes**

stack-source-indexes-after3.log: No hex crate in owning dependencies; replaced with local bounded digest rendering.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-jj --test stack_source_recovery indexes::
```

**Notes**

stack-source-indexes-after4.log: One case passes; no-follow private scratch under macOS /var symlink failed; canonicalized the private directory.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-jj --test stack_source_recovery indexes::
```

**Notes**

stack-source-indexes-after5.log: Original two exact cases pass.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-jj --test stack_source_recovery indexes::
```

**Notes**

stack-source-indexes-semantic.log: Used user-expression helper instead of resolved enum; fixed to exact typed ancestry expression.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-jj --test stack_source_recovery indexes::
```

**Notes**

stack-source-indexes-existing.log: Same expression diagnostic repeated before correction.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-jj --test stack_source_recovery indexes::
```

**Notes**

stack-source-indexes-semantic-after.log: Four exact integrations pass, including valid existing CLI mapping preservation and substituted future graph rejection.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-jj --test stack_source_recovery indexes
```

**Notes**

stack-source-indexes-lifecycle.log: Six pass; isolated helper executed by parent.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-jj --test stack_source_recovery indexes
```

**Notes**

stack-source-indexes-adversarial.log: Eight new cases pass; added unsafe/replaced/oversized input and authorization loss.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-jj --test stack_source_recovery --test stack_foreground --test stack_operation_archive
```

**Notes**

stack-source-indexes-direct.log: 41 direct cases pass: 26 source recovery, eight foreground and seven archive; four subprocess helpers execute through passing parents.
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
CARGO_INCREMENTAL=0 cargo check --locked -p jj-store-jj --all-targets
```

**Notes**

stack-source-indexes-check.log: All owning package targets pass.
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
CARGO_INCREMENTAL=0 cargo clippy --locked -p jj-store-jj --all-targets --no-deps -- -D warnings
```

**Notes**

stack-source-indexes-lint.log: Strict owning lint passes without suppressions.
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

stack-source-indexes-boundaries.log: 17 package boundaries pass.
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

stack-source-indexes-architecture.log: 1090 presentation modules pass; no UI implementation change.
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

stack-source-indexes-rust-format.log: Owning/runtime Rust formatting passes.
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
pnpm exec prettier --write spec/src/architecture.md spec/src/documentation.md spec/src/local-jj.md spec/src/jj-integration.md spec/src/verification.md docs/implementation-plan.md docs/site/technical/local.md
```

**Notes**

stack-source-indexes-format.log: Changed documents formatted; ledger excluded.
<!-- /run -->
<!-- /evidence-field -->

