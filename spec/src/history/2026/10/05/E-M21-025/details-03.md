---
schema: 1
id: E-M21-025
---

# E-M21-025 details 3

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
node --test scripts/governance.test.mjs scripts/progress-mapping.test.mjs
```

**Notes**

stack-local-execution-governance.log: 33 governance and historical evidence cases pass
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
cargo fmt --all
```

**Notes**

Source and direct test families formatted after each edit; unrelated files unchanged.
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
pnpm exec prettier --write spec/src/cli.md spec/src/jj-integration.md spec/src/local-jj.md spec/src/architecture.md spec/src/documentation.md spec/src/verification.md docs/implementation-plan.md docs/site/technical/local.md docs/site/reference/configuration.md docs/maintenance/coverage.json
```

**Notes**

stack-local-execution-doc-format.log and doc-format-final.log: changed canonical/manual documents and reviewed source fingerprints formatted; ledger excluded.
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
pnpm docs:review local 'Reviewed exact saved checkout linkage and checked retained commit/archive/candidate identity. Runtime compares original journal selection before head/pointer mutation; source/index repair and killed-process recovery remain separate from production routes, selected combinations and general checkout.'
```

**Notes**

stack-local-execution-docs-review-local-final.log: final source fingerprint review, following initial local review of saved witness accessors.
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
pnpm docs:review configuration 'Reviewed original-request publication/checkout composition with exact retained commit/archive/candidate comparison before mutation, version-3 saved witness durability, bounded reserved growth and typed provider receipts. Killed-process and real registered physical guards are separate from legacy metadata fixtures; selected combinations, general checkout and production routes remain required.'
```

**Notes**

stack-local-execution-docs-review-server-final.log: final server source review, following initial journal/apply review.
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
CARGO_INCREMENTAL=0 pnpm docs:check
```

**Notes**

stack-local-execution-docs-check.log: initial 139-file closure, links/anchors and nine offline diagrams pass. Final source review/identity run is pending below until completion.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-jj --test stack_unchanged_checkout --test stack_head_publication --test stack_source_recovery --test stack_foreground --test stack_operation_archive --test stack_reword_preparation
```

**Notes**

stack-local-execution-provider-final.log: all 78 provider cases pass after final retained identity capture under worker budget; six process helpers through parents.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-server --test stack_local_execution --test stack_command_journal --test stack_local_preparation --test stack_combinations
```

**Notes**

stack-local-execution-server-final.log: 34 pass, including nine execution cases, 13 journal, eight preparation and four combination metadata cases. Four killed checkpoints resume exact candidate/witness; source/index loss, immutable/corrupt witnesses, identity/proof/retention guards and terminal history pass.
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
CARGO_INCREMENTAL=0 cargo clippy --locked -p jj-store-server -p jj-store-jj -p jj-store-cli --all-targets --no-deps -- -D warnings
```

**Notes**

stack-local-execution-lint-complete.log: final three owning package strict all-target no-deps lint passes.
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
CARGO_INCREMENTAL=0 cargo check --locked -p jj-store-server -p jj-store-jj -p jj-store-cli --all-targets
```

**Notes**

stack-local-execution-check-complete.log: final three owning all-target source checks pass.
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
CARGO_INCREMENTAL=0 pnpm docs:check
```

**Notes**

stack-local-execution-docs-check-final.log: final 139-file closure, links/anchors and nine offline diagrams in both themes pass after final source identity and review updates.
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
pnpm exec prettier --check spec/src/cli.md spec/src/jj-integration.md spec/src/local-jj.md spec/src/architecture.md spec/src/documentation.md spec/src/verification.md docs/implementation-plan.md docs/site/technical/local.md docs/site/reference/configuration.md docs/maintenance/coverage.json
```

**Notes**

stack-local-execution-doc-format-check.log: final changed documents pass; ledger excluded.
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
pnpm format:rust:check
```

**Notes**

stack-local-execution-rust-format-final.log: final owning/runtime formatting passes.
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
pnpm spec:first
```

**Notes**

stack-local-execution-spec-first-final.log: final 18 protected files/six canonical chapters pass after identity fields and evidence append.
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
pnpm check:progress
```

**Notes**

stack-local-execution-progress.log: all 142 task/evidence checks pass; aggregate remains pending.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-server --lib stack_
```

**Notes**

stack-local-execution-units-final.log: final nine journal/combination units pass after retained identity capture; two subprocess helpers execute through passing parents. Existing linker warning remains unsuppressed.
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
python3 raw JSON body SHA-256 audit against stack-local-execution-ledger-before.json
```

**Notes**

stack-local-execution-ledger-audit.log: exactly 507 evidence objects; all 506 preceding raw bodies, including E-M21-024, remain byte-identical.
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
pnpm check:progress
```

**Notes**

stack-local-execution-progress-final.log: all 142 task/evidence checks pass after final units/raw-body audit.
<!-- /run -->
<!-- /evidence-field -->

