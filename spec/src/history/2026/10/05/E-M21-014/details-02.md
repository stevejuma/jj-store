---
schema: 1
id: E-M21-014
---

# E-M21-014 details 2

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
cargo test --locked -p jj-store-jj --test stack_foreground
```

**Notes**

stack-foreground-before.log: one absent local_execution export diagnostic reproduces the missing foreground coordinator boundary.
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
cargo test --locked -p jj-store-jj --test stack_foreground
```

**Notes**

stack-foreground-first.log: four exact foreground regressions pass against real stock JJ. Busy locks, unsafe symlink/hardlink entries and selected-lock replacement preserve checkout state.
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
cargo test --locked -p jj-store-jj --test stack_foreground
```

**Notes**

stack-foreground-lifecycle1.log: seven pass, one explicitly ignored subprocess helper is actually invoked by its passing process-loss parent. Cancellation, external heads, replaced WC directories and abrupt process exit preserve checkout bytes and release foreground admission.
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
cargo test --locked -p jj-store-jj --test stack_foreground --test stack_preparation_retention --test local_metadata --test local_evidence
```

**Notes**

stack-foreground-direct.log: 33 pass (7 foreground, 6 retention, 14 metadata, 6 evidence); the foreground subprocess helper remains explicitly ignored for ordinary invocation but runs through its parent.
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
cargo check --locked -p jj-store-jj --all-targets
```

**Notes**

stack-foreground-check.log: owning integration all-target type/check lane passes.
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
cargo clippy --locked -p jj-store-jj --all-targets --no-deps -- -D warnings
```

**Notes**

stack-foreground-lint.log: strict owning integration lint passes, without warnings suppressed or dependency/service lint qualification.
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
cargo test --locked -p jj-store-jj --lib
```

**Notes**

stack-foreground-unit.log: all 29 owning unit tests pass. Private descriptor helper visibility changes preserve original read implementations.
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
pnpm check:boundaries
```

**Notes**

stack-foreground-boundaries.log: 17 packages pass; owned witnesses and access cross the coordinator boundary, no upstream working-copy type escapes.
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
pnpm check:architecture
```

**Notes**

stack-foreground-architecture.log: 1090 modules pass; no presentation/source-checkout dependency change.
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
cargo fmt --all
```

**Notes**

Pinned Rust formatter applied to the affected source; unrelated files remain unchanged.
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
pnpm format:rust:check
```

**Notes**

stack-foreground-rust-format.log and stack-foreground-rust-format-final.log: root and embedded-runtime formatting checks pass.
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
pnpm exec prettier --write spec/src/local-jj.md spec/src/jj-integration.md spec/src/architecture.md spec/src/verification.md docs/implementation-plan.md docs/site/technical/local.md
```

**Notes**

stack-foreground-docs-format.log: changed canonical/manual documents only; historical ledger excluded.
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
pnpm docs:review local "Qualified nonblocking foreground JJ working-copy leases with exact authorized metadata/head validation, bounded admission and cancellation, real CLI contention/process loss, unsafe lock rejection and release that never unlinks. No snapshot/publication/checkout capability; physical editor bytes remain separately guarded."
```

**Notes**

stack-foreground-docs-review.log: local source review acknowledges the exact primitive and its incomplete execution boundaries.
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
pnpm exec prettier --check spec/src/architecture.md spec/src/jj-integration.md spec/src/local-jj.md spec/src/verification.md docs/implementation-plan.md docs/site/technical/local.md docs/maintenance/coverage.json
```

**Notes**

stack-foreground-docs-format-check.log: one generated review JSON formatting diagnostic; formatted coverage explicitly, without touching milestones.md.
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
pnpm exec prettier --write docs/maintenance/coverage.json spec/src/documentation.md
```

**Notes**

stack-foreground-coverage-format.log: formatted review JSON and owning documentation chapter.
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
pnpm exec prettier --check spec/src/architecture.md spec/src/jj-integration.md spec/src/local-jj.md spec/src/verification.md spec/src/documentation.md docs/implementation-plan.md docs/site/technical/local.md docs/maintenance/coverage.json
```

**Notes**

stack-foreground-format-final.log: all changed documents pass formatting; ledger never formatted.
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
cargo test --locked -p jj-store-jj --test stack_foreground
```

**Notes**

stack-foreground-final.log: final complete foreground binary has eight passing regressions and one ignored subprocess helper invoked by its passing parent. Includes noninitialization of missing checkout state and explicit separation of external editor bytes from metadata proof.
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
pnpm docs:check
```

**Notes**

stack-foreground-docs-check.log: nine offline Mermaid diagrams, strict manual build and 139-file link/anchor closure pass.
<!-- /run -->
<!-- /evidence-field -->

