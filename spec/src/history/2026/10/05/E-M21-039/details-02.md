---
schema: 1
id: E-M21-039
---

# E-M21-039 details 2

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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-jj --test working_tree_current
```

**Notes**

physical-identity-before.log: All five existing current-reader parents pass before the change.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 9
hasNote: true
noteEncoding: markdown

```

```shell
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-jj --test working_tree_current identity::
```

**Notes**

physical-identity-red.log: New four-parent identity target fails compilation with nine E0599 missing-method diagnostics.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-jj --test working_tree_current identity::
```

**Notes**

physical-identity-green.log: Four exact identity parents pass; five existing parents are filtered.
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
cargo fmt --all
```

**Notes**

tool output: Owning Rust source and test formatting applied; no unrelated Rust file changed.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-jj --test working_tree_current
```

**Notes**

physical-identity-regression.log: Complete directly affected current-reader binary: nine parents pass, including all five prior regressions.
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
CARGO_INCREMENTAL=0 cargo clippy --locked -p jj-store-jj --all-targets --no-deps -- -D warnings
```

**Notes**

physical-identity-clippy.log: Owning package all-target check and strict lint pass without suppression.
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
pnpm exec prettier --write docs/site/technical/local.md docs/implementation-plan.md spec/src/local-jj.md spec/src/jj-integration.md spec/src/architecture.md spec/src/documentation.md
```

**Notes**

physical-identity-document-format.log: Changed documentation formatted; historical milestone ledger excluded.
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
pnpm docs:review local 'Reviewed the provider-owned stable physical identity read for historical stack admission: unchanged observer digest, bounded no-follow directory checks across authorization, refusal on races/revocation, and independence from current code/checkout/policy/editor reads. Runtime registration, original journal validation and production wiring remain separate gates.'
```

**Notes**

physical-identity-docs-review.log: Owning local source fingerprint reviewed against the updated technical guide.
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
pnpm test:governance
```

**Notes**

physical-identity-governance.log: All 50 governance tests pass.
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

physical-identity-boundaries.log: All 17 package boundaries pass.
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

physical-identity-architecture.log: 1090-module source architecture passes.
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
pnpm docs:check
```

**Notes**

physical-identity-docs-check.log: Parser-reference generation, source review and strict documentation build pass; nine diagrams rendered locally and 139-file closure has valid links/anchors. This actual invocation used the inherited Cargo incremental setting; all direct Rust regression and lint commands explicitly disable incremental compilation.
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
pnpm spec:first
```

**Notes**

physical-identity-spec-first.log: Initial mapped gate passes six protected files and four canonical chapters.
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
pnpm spec:validate
```

**Notes**

physical-identity-spec-validation.log: Five validators, 23 chapters and 173 requirements pass.
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
pnpm exec prettier --write docs/maintenance/coverage.json spec/src/verification.md
```

**Notes**

physical-identity-coverage-format.log: Coverage and verification formatting restored; only the local source review changes in coverage.
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
cargo fmt --all -- --check
```

**Notes**

physical-identity-rust-format.log: All owning-workspace Rust formatting passes.
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
pnpm exec prettier --check docs/site/technical/local.md docs/implementation-plan.md docs/maintenance/coverage.json spec/src/local-jj.md spec/src/jj-integration.md spec/src/architecture.md spec/src/documentation.md spec/src/verification.md
```

**Notes**

physical-identity-format-check.log: All eight changed non-ledger documents pass formatting.
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
pnpm check:ui-ownership
```

**Notes**

physical-identity-ui-ownership.log: Ownership validator passes; no presentation or dependency ownership changes.
<!-- /run -->
<!-- /evidence-field -->

