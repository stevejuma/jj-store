---
schema: 1
id: E-M21-013
---

# E-M21-013 details 2

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
cargo test --locked -p jj-store-jj --test stack_preparation_retention retained_unpublished_stack_operation_survives_gc_and_source_state_loss -- --exact
```

**Notes**

stack-retention-before.log: missing retention exports/methods plus one fixture await on synchronous get_commit. Added the provider primitive and corrected the fixture API use.
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
cargo test --locked -p jj-store-jj --test stack_preparation_retention retained_unpublished_stack_operation_survives_gc_and_source_state_loss -- --exact
```

**Notes**

stack-retention-first.log and stack-retention-exact-final.log: exact real unpublished reword fixture passes. Archive, operation/view and code closure survive source op-store/index deletion and real Git gc --prune=now; original heads, companion absence and dirty file bytes are retained.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
cargo test --locked -p jj-store-jj --test stack_preparation_retention
```

**Notes**

stack-retention-integration1.log: four pass, two fixture errors. A discarded TempDir deleted one test repository; the missing-tree test selected the special empty tree, which Git can synthesize. Retained the fixture owner and removed a nonempty physical tree instead; no assertion weakened.
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
cargo test --locked -p jj-store-jj --test stack_preparation_retention
```

**Notes**

stack-retention-integration2.log: all six regressions pass, including immutable scope/intent collisions, invalid reserved/symbolic refs, physical closure loss, unknown ref lock selection and exact candidate replay, byte/work limits, noninitializing reads and unchanged user bookmarks/checkout metadata.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
cargo clippy --locked -p jj-store-jj --lib --no-deps -- -D warnings
```

**Notes**

stack-retention-clippy-lib1.log reproduces the four inherited JJ diagnostics recorded in E-M21-010: range pattern, map entry and two argument groups. This required owning source lane cannot pass by waiving them.
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
cargo clippy --locked -p jj-store-jj --lib --no-deps -- -D warnings
```

**Notes**

stack-retention-clippy-lib2.log: all four repaired in owning private helpers without suppression. Inclusive field range and vacant entry preserve decode/hydration behavior; named loader/insertion records retain every original budget/access/cache field.
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
cargo test --locked -p jj-store-jj --test stack_preparation_retention --test local_metadata --test local_evidence
```

**Notes**

stack-retention-direct-tests.log: 26 pass across three complete binaries (6 evidence, 14 existing metadata, 6 retention). No companion metadata codec or shared object primitive changed.
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

stack-retention-jj-lib-tests.log: all 29 unit tests pass, including exact extra-data wire validation, cache reservation/memory and read identity consumers.
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
cargo test --locked -p jj-store-cli --test local_browser --test native_lifecycle
```

**Notes**

stack-retention-read-consumers.log: complete direct local/native readers exercised because strict-lint repair changes their shared private cache/index/hydration helpers. Local browser 28 pass, two explicit fixtures ignored in ordinary lane; native lifecycle 205 pass. This complete direct-consumer escalation was selected before running it; no root aggregate or real-RustFS qualification. Existing macOS linker unwind-size warnings retained.
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
cargo check --locked -p jj-store-jj --all-targets
```

**Notes**

stack-retention-check.log: owning JJ all-target package check passes. No dependency additions.
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
cargo clippy --locked -p jj-store-jj --all-targets --no-deps -- -D warnings
```

**Notes**

stack-retention-clippy-all1.log: all owning JJ production/tests/examples strict lint pass. The four previously recorded JJ failures are resolved; two unchanged service diagnostics remain unwaived and are not qualified by this no-deps command.
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
pnpm docs:review local "Documented immutable reserved-ref Stacks code retention, exact scope and retry, complete Git reachability, unknown ref selection and loose/packed replay. Archival retention does not qualify production command generation, source operation ancestry, head publication or guarded checkout. Four JJ read lint repairs preserve admission and preparation behavior."
```

**Notes**

stack-retention-docs-review.log: reviewed the owning JJ/local source topic and manual; formatted its portable coverage acknowledgement explicitly.
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
pnpm exec prettier --write spec/src/architecture.md spec/src/jj-integration.md spec/src/local-jj.md spec/src/documentation.md spec/src/verification.md docs/implementation-plan.md docs/site/technical/local.md docs/maintenance/coverage.json
```

**Notes**

stack-retention-docs-format.log: explicit changed docs only; never formats milestones.md.
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

stack-retention-docs-check.log: nine offline Mermaid diagrams, strict manual build and 139-file link/anchor closure pass. No UI screenshot comparison or baseline update.
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

stack-retention-boundaries.log: 17 packages pass; upstream JJ imports remain only in the owning integration/CLI boundary.
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

stack-retention-architecture.log: 1090 modules pass; production UI and reference checkout dependencies unchanged.
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

stack-retention-rust-format.log: root/runtime formatting passes. cargo fmt --all used during implementation; no unrelated source edited.
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
pnpm exec prettier --check spec/src/architecture.md spec/src/jj-integration.md spec/src/local-jj.md spec/src/documentation.md spec/src/verification.md docs/implementation-plan.md docs/site/technical/local.md docs/maintenance/coverage.json
```

**Notes**

stack-retention-format-check.log: all changed canonical/manual/coverage documents formatted; historical ledger excluded.
<!-- /run -->
<!-- /evidence-field -->

