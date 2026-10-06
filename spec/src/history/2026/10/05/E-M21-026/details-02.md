---
schema: 1
id: E-M21-026
---

# E-M21-026 details 2

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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-cli --test stack_registered_preparation apply::registered_first_apply_recovers_original_prepared_source_after_loss_and_gc -- --exact
```

**Notes**

stack-prepared-source-before.log: Provider(Unavailable) reproduces first-apply source-read ordering gap.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-cli --test stack_registered_preparation apply::registered_first_apply_recovers_original_prepared_source_after_loss_and_gc -- --exact
```

**Notes**

stack-prepared-source-exact.log: Exact new regression passes after original dependency recovery is ordered before source-backed context.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-cli --test stack_registered_preparation
```

**Notes**

stack-prepared-source-registered.log: Six pass, one fixture assumption fails on byte-restoration success. Physical fingerprint includes file identity/timestamps. Scenario split retains combination recovery success and strengthens editor/reverted-byte stale assertions; production guard unchanged.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-cli --test stack_registered_preparation apply::registered_recovered_first_apply -- --nocapture
```

**Notes**

stack-prepared-source-guards.log: Both focused complete combination and editor-guard cases pass.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-cli --test stack_registered_preparation
```

**Notes**

stack-prepared-source-registered-final.log: All eight actual registered-folder cases pass; fixture combination authority remains explicitly isolated. Original source loss/GC and journal reopen before first apply recover exact candidate. Terminal retry after later source loss performs no repair.
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

stack-prepared-source-server.log: 34 pass: nine execution, 13 journal, eight preparation and four combination metadata cases. Ignored child helpers run through passing process-loss parents; no fabricated current proof introduced.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-server --lib stack_
```

**Notes**

stack-prepared-source-units.log: Nine journal/combination units pass; two child helpers through parents.
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
CARGO_INCREMENTAL=0 cargo check --locked -p jj-store-server -p jj-store-cli --all-targets
```

**Notes**

stack-prepared-source-check.log: Both owning packages all-target checks pass.
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
CARGO_INCREMENTAL=0 cargo clippy --locked -p jj-store-server -p jj-store-cli --all-targets --no-deps -- -D warnings
```

**Notes**

stack-prepared-source-lint.log: Both owning packages strict all-target lint passes; no suppression.
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
pnpm check:boundaries
```

**Notes**

stack-prepared-source-boundaries.log: All 17 package boundaries pass.
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
pnpm check:architecture
```

**Notes**

stack-prepared-source-architecture.log: 1090 modules pass.
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
pnpm spec:validate
```

**Notes**

stack-prepared-source-spec-validation.log: Five validators, 23 chapters and 173 requirements pass.
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
node --test scripts/governance.test.mjs scripts/progress-mapping.test.mjs
```

**Notes**

stack-prepared-source-governance.log: All 33 governance/historical evidence cases pass.
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

stack-prepared-source-rust-format-final.log: Implementation and tests formatted; final pnpm format:rust:check passes owning/root/runtime format.
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

stack-prepared-source-rust-format.log: Initial owning/root/runtime formatting passes.
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
pnpm format:rust:check
```

**Notes**

stack-prepared-source-rust-format-final.log: Final owning/root/runtime formatting passes after stronger physical guard fixture.
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
pnpm exec prettier --write spec/src/architecture.md spec/src/cli.md spec/src/documentation.md spec/src/local-jj.md docs/implementation-plan.md docs/site/technical/local.md docs/site/reference/configuration.md docs/maintenance/coverage.json
```

**Notes**

stack-prepared-source-doc-format.log: Changed documents formatted; milestones excluded.
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
pnpm exec prettier --write spec/src/verification.md
```

**Notes**

stack-prepared-source-verification-format.log: Qualification mapping formatted; remains Planned.
<!-- /run -->
<!-- /evidence-field -->

