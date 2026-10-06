---
schema: 1
id: E-M21-015
---

# E-M21-015 details 2

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
cargo test --locked -p jj-store-jj --test stack_reword_preparation
```

**Notes**

stack-reword-before.log: missing Reword export and two absent prepare_reword method diagnostics establish the missing real provider boundary.
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
cargo test --locked -p jj-store-jj --test stack_reword_preparation
```

**Notes**

stack-reword-first.log: implementation compilation reports one invalid conversion to borrowed WorkspaceName. Replaced it with the pinned WorkspaceName::new API; no assertion or contract weakened.
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
cargo test --locked -p jj-store-jj --test stack_reword_preparation
```

**Notes**

stack-reword-second.log: both initial real generation and invalid root/hidden/message cases pass.
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
cargo test --locked -p jj-store-jj --test stack_reword_preparation
```

**Notes**

stack-reword-complete1.log and stack-reword-closure-final.log: all six cases pass, including a real three-commit ancestor rewrite, exact bookmark terms and tree/author/change invariants, owned candidate-to-reserved-ref retention, and cancellation/authorization loss after actual operation candidate creation. Final validation additionally proves the complete visible descendant closure and exact target/descendant messages.
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
cargo test --locked -p jj-store-jj --test stack_reword_preparation --test stack_foreground --test stack_preparation_retention --test local_metadata --test local_evidence
```

**Notes**

stack-reword-direct.log and stack-reword-direct-final.log: final 40 passing tests across five complete direct binaries (6 reword, 8 foreground, 6 retention, 14 metadata, 6 evidence). Foreground process-loss helper is explicitly ignored in ordinary invocation but actually invoked by its passing parent.
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
cargo check --locked -p jj-store-jj --all-targets
```

**Notes**

stack-reword-check.log and stack-reword-check-final.log: owning all-target source check passes, including the final exact-closure validation.
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
cargo clippy --locked -p jj-store-jj --all-targets --no-deps -- -D warnings
```

**Notes**

stack-reword-lint.log and stack-reword-lint-final.log: strict owning integration lint passes without warnings suppressed; dependency/service lint remains outside this qualified lane.
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
cargo test --locked -p jj-store-jj --lib
```

**Notes**

stack-reword-unit.log: all 29 owning units pass. Existing native/current reads and archive/object helpers are reused unchanged.
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
cargo fmt --all
```

**Notes**

Pinned Rust formatting during implementation; no unrelated source changed.
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

stack-reword-boundaries.log: 17 packages pass; upstream transaction/repository objects remain inside the integration boundary.
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

stack-reword-architecture.log: 1090 modules pass; no production/catalog/reference-checkout ownership changes.
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
pnpm format:rust:check
```

**Notes**

stack-reword-format-rust.log: root and embedded-runtime formatting checks pass.
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
pnpm exec prettier --write spec/src/architecture.md spec/src/documentation.md spec/src/jj-integration.md spec/src/local-jj.md spec/src/verification.md docs/implementation-plan.md docs/site/technical/local.md
```

**Notes**

stack-reword-docs-format.log: explicit canonical/manual formatting only; milestones.md excluded.
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
pnpm docs:review local "Documented real unpublished local reword generation at the exact foreground source, unsigned resolved-tree bounds, stable author/change/tree and descendant/bookmark effects, unchanged checkout, candidate cancellation and authorization loss. Runtime policy/journal composition, ancestry recovery, publication and checkout remain unfinished."
```

**Notes**

stack-reword-docs-review.log: source review covers the initial real generation boundary.
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
pnpm docs:review local "Documented real unpublished reword at the exact foreground source; bounded visible membership and exact complete descendant closure, message/author/change/tree invariants, local bookmark terms, unchanged checkout and candidate cancellation/authorization loss. Runtime journal/policy composition, ancestry recovery, publication and checkout remain unfinished."
```

**Notes**

stack-reword-docs-review-final.log: source fingerprint refreshed after final exact-descendant closure and archive-limit classification changes.
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
pnpm exec prettier --write docs/maintenance/coverage.json
```

**Notes**

stack-reword-coverage-format.log and stack-reword-coverage-format-final.log: review JSON formatted independently from ledger.
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
pnpm exec prettier --check spec/src/architecture.md spec/src/documentation.md spec/src/jj-integration.md spec/src/local-jj.md spec/src/verification.md docs/implementation-plan.md docs/site/technical/local.md docs/maintenance/coverage.json
```

**Notes**

stack-reword-format-docs.log: all affected docs/coverage pass formatting.
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
pnpm spec:first
```

**Notes**

stack-reword-spec-first.log: eight protected files and five owning canonical chapters pass.
<!-- /run -->
<!-- /evidence-field -->

