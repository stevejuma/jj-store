---
schema: 1
id: E-M21-023
---

# E-M21-023 details 2

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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-jj --test stack_head_publication
```

**Notes**

stack-head-publication-before.log: Missing resume_publication API diagnostics at the two initial exact regressions.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-jj --test stack_head_publication
```

**Notes**

stack-head-publication-first.log: Private constant/record-destination visibility diagnostics; repaired without changing dependencies.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-jj --test stack_head_publication
```

**Notes**

stack-head-publication-second.log: Both initial exact real-CLI publication/sibling cases pass.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-jj --test stack_head_publication
```

**Notes**

stack-head-publication-expanded.log: 13 pass/three new fixture failures. Two snapshots did not account for allowed empty native lock creation; fixture now starts with that coordination file. Policy fixture initially changed physical .gitignore rather than saved Git exclude metadata; corrected to .git/info/exclude. Contracts and assertions preserved.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-jj --test stack_head_publication
```

**Notes**

stack-head-publication-expanded-after.log: 16 pass; isolated subprocess helper runs through parent.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-jj --test stack_head_publication
```

**Notes**

stack-head-publication-full.log: 18 pass after adding interrupted acknowledgement and bounded marker/record cases.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-jj --test stack_head_publication --test stack_source_recovery --test stack_foreground --test stack_operation_archive --test stack_reword_preparation
```

**Notes**

stack-head-publication-direct.log: 68 unique direct cases pass: 19 publication, 26 source recovery, eight foreground, seven archive and eight reword. Five process-loss helpers execute through passing parents. Includes real 4096-descendant operation-limit/fanout cases.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-jj --test stack_head_publication
```

**Notes**

stack-head-publication-proof.log: All 19 publication cases pass after binding the provider-only proof to original retained key and witness digest. Same-proof/reopen assertions retain that scope.
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
CARGO_INCREMENTAL=0 cargo check --locked -p jj-store-jj --all-targets
```

**Notes**

stack-head-publication-check.log: All owning targets pass before the final private proof-binding fields; final strict all-target lint also type-checks the final source.
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
CARGO_INCREMENTAL=0 cargo clippy --locked -p jj-store-jj --all-targets --no-deps -- -D warnings
```

**Notes**

stack-head-publication-lint.log: One unique manual_non_exhaustive diagnostic (lib and lib-test). Replaced unit guard with semantically required private preparation key/witness fields; no allowance or suppression.
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
CARGO_INCREMENTAL=0 cargo clippy --locked -p jj-store-jj --all-targets --no-deps -- -D warnings
```

**Notes**

stack-head-publication-lint-after.log: Final owning all-target strict lint passes; no aggregate service qualification claimed.
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
cargo fmt --all
```

**Notes**

stack-head-publication-rust-format.log, stack-head-publication-rust-format-final.log, stack-head-publication-proof-format.log: Formatted owning Rust source and regression modules.
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

stack-head-publication-rust-format-check.log: Owning/runtime Rust formatting passes; final Rust field addition also formatted with cargo fmt.
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

stack-head-publication-boundaries.log: Package boundary audit passes.
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

stack-head-publication-architecture.log: Presentation architecture passes; no UI source change.
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
pnpm exec prettier --write spec/src/architecture.md spec/src/documentation.md spec/src/local-jj.md spec/src/jj-integration.md spec/src/verification.md docs/implementation-plan.md docs/site/technical/local.md
```

**Notes**

stack-head-publication-doc-format.log: Changed documents formatted; historical ledger excluded.
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
pnpm exec prettier --write docs/maintenance/coverage.json spec/src/local-jj.md
```

**Notes**

stack-head-publication-coverage-format.log: Source-review acknowledgement and final canonical proof contract formatted.
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
pnpm exec prettier --check spec/src/architecture.md spec/src/documentation.md spec/src/local-jj.md spec/src/jj-integration.md spec/src/verification.md docs/implementation-plan.md docs/site/technical/local.md docs/maintenance/coverage.json
```

**Notes**

stack-head-publication-doc-format-check.log: Changed canonical/manual documents pass.
<!-- /run -->
<!-- /evidence-field -->

