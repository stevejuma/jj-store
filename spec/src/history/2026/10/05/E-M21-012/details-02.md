---
schema: 1
id: E-M21-012
---

# E-M21-012 details 2

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
cargo test --locked -p jj-store-server --test stack_command_journal retained_preparation_survives_reopen_and_conflicting_request_reuse -- --exact
```

**Notes**

stack-journal-before.log: missing stack_command_journal module prevents the named retention regression from compiling.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 7
hasNote: true
noteEncoding: markdown

```

```shell
cargo test --locked -p jj-store-server --test stack_command_journal retained_preparation_survives_reopen_and_conflicting_request_reuse -- --exact
```

**Notes**

stack-journal-after-exact.log: integration fixture path was implicit; one missing-module and six missing-helper diagnostics. Fixed the explicit local fixture path; no product assertion weakened.
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
cargo test --locked -p jj-store-server --test stack_command_journal retained_preparation_survives_reopen_and_conflicting_request_reuse -- --exact
```

**Notes**

stack-journal-after-exact2.log and stack-journal-exact-final.log: exact regression passes after implementation and in the final source state.
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
cargo test --locked -p jj-store-server --test stack_command_journal
```

**Notes**

stack-journal-integration1.log: 12 pass. stack-journal-integration-final.log: 13 pass, including additional receipt-growth reservation at the byte limit. Scope, principal/intent/preparation binding, monotonic states, competing pending commands, reopen, terminal replay, corruption, private modes/paths, missing state and capacity are qualified only for journal metadata.
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
cargo test --locked -p jj-store-server --lib stack_command_journal::tests
```

**Notes**

stack-journal-unit1.log: four pass and one ignored subprocess fixture executed by the parent test. Actual lock loss after atomic replacement returns unknown; abrupt child exit after replacement before sync/ack recovers exact unresolved receipt through reopen and replay. Existing macOS linker unwind-size warning retained.
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
cargo check --locked -p jj-store-server --all-targets
```

**Notes**

stack-journal-check.log: affected server all-target package check passes; no new dependency or upstream JJ import.
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
cargo clippy --locked -p jj-store-server --all-targets --no-deps -- -D warnings
```

**Notes**

stack-journal-clippy.log: strict owning server source/tests pass without dependency lint. This does not qualify the six previously recorded JJ/service diagnostics or the aggregate Rust lane.
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
pnpm docs:review configuration "Documented bounded private stack command journal scope, exact retry and restart recovery, capacity reservation and monotonic receipts. Provider object retention, operation publication and physical checkout remain unimplemented; no runtime or installed command capability changes."
```

**Notes**

stack-journal-docs-review.log: server source coverage and lifecycle manual reviewed; coverage formatted explicitly.
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
pnpm exec prettier --write spec/src/cli.md spec/src/local-jj.md spec/src/verification.md docs/site/technical/lifecycle.md docs/maintenance/coverage.json
```

**Notes**

stack-journal-docs-format.log; subsequent exact verification and architecture/plan formatting in stack-journal-verification-format.log and stack-journal-plan-format.log. Never formats milestones.md. An earlier generic verification-row replacement assertion found two matching rows; corrected the edit to JST-LOCAL-005 only.
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

stack-journal-boundaries.log: 17 owned crate boundaries pass.
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

stack-journal-architecture.log: 1090 modules pass; no UI or reference-source changes.
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
cargo test --locked -p jj-store-server --lib stack_command_journal::tests
```

**Notes**

stack-journal-unit-final.log: final focused private journal lane passes four tests; the ignored abrupt-exit helper is exercised as a real subprocess by its passing parent test. Existing linker unwind-size notice remains.
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

stack-journal-rust-format-check.log: repository/runtime Rust formatting passes; cargo fmt --all was used during implementation and changed no unrelated files.
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
pnpm docs:check
```

**Notes**

stack-journal-docs-check.log: nine Mermaid diagrams, strict MkDocs build and 139-file self-contained link/anchor closure pass.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first
```

**Notes**

stack-journal-spec-first.log: lifecycle manual/coverage edit lacked its mapped documentation.md chapter. Added the owning requirement that journal metadata cannot imply provider retention/publication/checkout; no rule bypass.
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
pnpm exec prettier --write spec/src/documentation.md
```

**Notes**

stack-journal-documentation-format.log: explicit owning chapter format; milestones.md never formatted.
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
pnpm spec:first
```

**Notes**

stack-journal-spec-first-final.log: all protected source/plan/manual changes have their owning canonical chapters after documentation repair.
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
pnpm spec:validate
```

**Notes**

stack-journal-spec-validate.log: five validators, 23 chapters and 173 requirements validate.
<!-- /run -->
<!-- /evidence-field -->

