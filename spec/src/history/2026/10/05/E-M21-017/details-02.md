---
schema: 1
id: E-M21-017
---

# E-M21-017 details 2

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 7
hasNote: true
noteEncoding: markdown

```

```shell
cargo test --locked -p jj-store-jj --test stack_source_recovery
```

**Notes**

stack-source-recovery-before.log: missing witness/capture/source-restoration APIs establish the absent exact source recovery boundary.
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
cargo test --locked -p jj-store-jj --test stack_source_recovery
```

**Notes**

stack-source-recovery-after.log: one private archive-constant visibility compile diagnostic, fixed by a crate-private reexport; no source behavior changed.
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
cargo test --locked -p jj-store-jj --test stack_source_recovery
```

**Notes**

stack-source-recovery-after1.log: two pass; restart case rejects archive labels because native resolved empty labels and checkout label storage have different representations. Recovery now compares through upstream ConflictLabels, preserving exact archive and checkout formats.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
cargo test --locked -p jj-store-jj --test stack_source_recovery
```

**Notes**

stack-source-recovery-after2.log: two pass; restart case cannot open the private temporary materialization through macOS /var symlink. Canonicalized only the owning disposable cache; source path no-follow checks remain strict.
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
cargo test --locked -p jj-store-jj --test stack_source_recovery
```

**Notes**

stack-source-recovery-after3.log: all three original exact regressions pass, including total op-store/index loss, immutable conflict preflight and stale request/checkout rejection.
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
cargo test --locked -p jj-store-jj --test stack_source_recovery
```

**Notes**

stack-source-recovery-full.log: added lifecycle fixture used unwrap_err with a non-Debug owned lease. Fixed the test to match canceled JoinError without changing production API or weakening the assertion.
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
cargo test --locked -p jj-store-jj --test stack_source_recovery
```

**Notes**

stack-source-recovery-full1.log: nine tests pass; ignored subprocess helper actually executes through its passing parent. Real first-view process-loss retry, cancellation admission, complete key/codec rejection, changed policy/heads/binding, link rejection and exclusive recovered lease all pass.
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
cargo test --locked -p jj-store-jj --lib --test stack_source_recovery --test stack_operation_archive --test stack_reword_preparation --test stack_foreground --test stack_preparation_retention --test local_metadata --test local_evidence
```

**Notes**

stack-source-recovery-direct.log: 86 passing cases: 29 units and 57 directly affected integrations. Foreground and source-recovery ignored helpers are executed in isolated subprocesses by their passing parents.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
cargo check --locked -p jj-store-jj --all-targets
```

**Notes**

stack-source-recovery-check.log: reproduced the same lifecycle fixture Debug-bound compile diagnostic before its repair; final check is recorded separately after it finishes.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
cargo clippy --locked -p jj-store-jj --all-targets --no-deps -- -D warnings
```

**Notes**

stack-source-recovery-lint.log: two new collapsible-if diagnostics in profile preflight; repaired directly without suppression.
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

stack-source-recovery-lint-final.log: owning source/test strict lint passes; inherited service/dependency lint remains unqualified.
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

Pinned source formatting during implementation; no unrelated Rust changes.
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
pnpm check:boundaries
```

**Notes**

stack-source-recovery-boundaries.log: all 17 owning package boundaries pass; upstream imports remain in JJ integration.
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
pnpm check:architecture
```

**Notes**

stack-source-recovery-architecture.log: all 1090 presentation modules pass; no production UI, column or controller behavior changed.
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

stack-source-recovery-rust-format.log: root/runtime Rust formatting passes.
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
pnpm exec prettier --write spec/src/architecture.md spec/src/documentation.md spec/src/jj-integration.md spec/src/local-jj.md spec/src/verification.md docs/implementation-plan.md docs/site/technical/local.md
```

**Notes**

stack-source-recovery-docs-format.log: owning canonical/manual documents only; ledger never formatted.
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
pnpm docs:review local "Documented exact source operation/view restoration after operation/index loss, using the foreground lease, bounded saved witness, original reserved retention key, conflict/link preflight and synced no-replace records. Real cancellation and subprocess loss retry retain the original identity and leave heads, index, extras, companion and physical checkout unchanged. Runtime authenticated journal composition, extra/index repair, publication and guarded checkout remain required."
```

**Notes**

Exact review text and command retained in stack-source-recovery-docs-review.log and stack-source-recovery-docs-review-final.log. Final acknowledgement covers the completed owning provider and records remaining runtime publication/checkout gates.
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
pnpm exec prettier --write docs/maintenance/coverage.json
```

**Notes**

stack-source-recovery-coverage-format.log: updated owning local source review acknowledgement only.
<!-- /run -->
<!-- /evidence-field -->

