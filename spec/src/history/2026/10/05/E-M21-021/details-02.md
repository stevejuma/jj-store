---
schema: 1
id: E-M21-021
---

# E-M21-021 details 2

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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-jj --test stack_source_recovery extras::
```

**Notes**

stack-source-extras-before.log: Absent explicit recovery API.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-jj --test stack_source_recovery extras::
```

**Notes**

stack-source-extras-after.log: Two exact cases pass; one unused import warning subsequently removed.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-jj --test stack_source_recovery extras
```

**Notes**

stack-source-extras-faults.log: Lifecycle assertion had a Debug bound; corrected without changing canceled behavior.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-jj --test stack_source_recovery extras
```

**Notes**

stack-source-extras-faults-after.log: Seven cases pass; subprocess helper invoked through parent.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-jj --test stack_source_recovery extras
```

**Notes**

stack-source-extras-limits.log: Nine cases pass; includes exact orphan conflict/retry and size boundary.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-jj --test stack_source_recovery --test stack_foreground --test stack_reword_preparation --test stack_operation_archive --test stack_preparation_retention
```

**Notes**

stack-source-extras-direct.log: 47 direct cases pass: 18 source recovery, eight foreground, eight reword, seven archive and six retention. Three ignored process helpers are invoked by passing parents.
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
cargo fmt --all
```

**Notes**

stack-source-extras-source.log: Formatted only Rust source/test implementation.
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

stack-source-extras-boundaries.log: 17 package boundaries pass.
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

stack-source-extras-architecture.log: 1090 presentation modules pass; no UI code change.
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
pnpm format:rust:check
```

**Notes**

stack-source-extras-rust-format.log: Owning/runtime Rust formatting passes.
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
pnpm exec prettier --write spec/src/architecture.md spec/src/documentation.md spec/src/local-jj.md spec/src/jj-integration.md spec/src/verification.md docs/implementation-plan.md docs/site/technical/local.md
```

**Notes**

stack-source-extras-format.log: Changed documents formatted; ledger excluded.
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
pnpm docs:review local "Reviewed explicit saved-archive GitBackend extra recovery, complete native-record preflight, bounded private serialization, create-only original-byte preservation and nonblocking lock lifetime. Manual separates cancellation/process-loss retry from derived index, operation publication, checkout and unavailable writable capabilities."
```

**Notes**

stack-source-extras-docs-review.log: Local source acknowledgement refreshed after manual review.
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
pnpm exec prettier --write docs/maintenance/coverage.json
```

**Notes**

stack-source-extras-coverage-format.log: Source-review fingerprint formatting; ledger excluded.
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
pnpm exec prettier --check spec/src/architecture.md spec/src/documentation.md spec/src/local-jj.md spec/src/jj-integration.md spec/src/verification.md docs/implementation-plan.md docs/site/technical/local.md docs/maintenance/coverage.json
```

**Notes**

stack-source-extras-doc-format-check.log: All changed canonical/manual documents pass.
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
CARGO_INCREMENTAL=0 pnpm docs:check
```

**Notes**

stack-source-extras-docs-check.log: 139-file closure, links and anchors pass; nine Mermaid diagrams rendered in both themes.
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
CARGO_INCREMENTAL=0 cargo check --locked -p jj-store-jj --all-targets
```

**Notes**

stack-source-extras-check.log: All owning package targets pass.
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
CARGO_INCREMENTAL=0 cargo clippy --locked -p jj-store-jj --all-targets --no-deps -- -D warnings
```

**Notes**

stack-source-extras-lint.log: Strict owning lint passes with no suppressed diagnostics.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-jj --test stack_source_recovery extras::
```

**Notes**

stack-source-extras-exact-final.log: Original exact regression target plus two added lock/link cases pass (four).
<!-- /run -->
<!-- /evidence-field -->

