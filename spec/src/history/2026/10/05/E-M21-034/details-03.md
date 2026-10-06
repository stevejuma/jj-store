---
schema: 1
id: E-M21-034
---

# E-M21-034 details 3

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
CARGO_INCREMENTAL=0 cargo clippy --locked -p jj-store-server -p jj-store-cli --all-targets --no-deps -- -D warnings
```

**Notes**

combination-join-lint-final.log: Final changed-owner and direct CLI-consumer all-target strict type/lint passes.
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

tool formatting outputs: Run after Rust source/test edits; only affected source changes remain.
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
pnpm format:rust:check
```

**Notes**

combination-join-rust-format.log: Pinned Rust format lane passes.
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
pnpm check:boundaries
```

**Notes**

combination-join-boundaries.log: 17-package upstream/source boundaries pass.
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
pnpm check:architecture
```

**Notes**

combination-join-architecture.log: Application architecture ownership check passes.
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
pnpm test:governance
```

**Notes**

combination-join-governance.log: 50 governance cases pass.
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
pnpm docs:review configuration 'Reviewed original staged combination binding, version-6 optional journal references, preserved legacy receipt reads, pending native evidence and combination sync before final success under journal-first coordination. Real native proofs remain distinct from metadata fault fixtures and unqualified setup/merge/WIP engines and transport.'
```

**Notes**

combination-join-docs-review.log: Final server source fingerprint reviewed against the private protocol.
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
pnpm exec prettier --write docs/implementation-plan.md docs/site/technical/local.md docs/maintenance/coverage.json spec/src/local-jj.md spec/src/cli.md spec/src/architecture.md spec/src/documentation.md spec/src/verification.md
```

**Notes**

combination-join-docs-format.log: Changed documents formatted; ledger excluded.
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
pnpm exec prettier --write spec/src/verification.md
```

**Notes**

combination-join-verification-format.log: Evidence moved into a prose section to retain the original verification table bytes.
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
pnpm exec prettier --check docs/implementation-plan.md docs/site/technical/local.md docs/maintenance/coverage.json spec/src/local-jj.md spec/src/cli.md spec/src/architecture.md spec/src/documentation.md spec/src/verification.md
```

**Notes**

combination-join-format-final.log: Changed-document format check passes; ledger excluded.
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
pnpm docs:check
```

**Notes**

combination-join-docs-check-final.log: Documentation closure, links/anchors and offline diagrams pass.
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
pnpm spec:first
```

**Notes**

combination-join-spec-first.log: Protected source changes have their mapped owning canonical chapters.
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

combination-join-coverage-format.log: Final reviewed coverage record formatted; source fingerprint unchanged.
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
python3 .local/stacks-backend-evidence/combination-join-ledger.py
```

**Notes**

combination-join-ledger.log: Appended only E-M21-034 and current M21 table references.
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
python3 .local/stacks-backend-evidence/combination-join-ledger-audit.py
```

**Notes**

combination-join-ledger-audit.log: 516 objects; all 515 preceding raw evidence bodies byte-identical.
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
pnpm check:progress
```

**Notes**

combination-join-progress.log: 142 task bookkeeping checks pass; M21.2 remains In progress.
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
pnpm spec:validate
```

**Notes**

combination-join-spec-validation.log: Five validators, 23 chapters and 173 requirements pass.
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
pnpm exec prettier --check docs/implementation-plan.md docs/site/technical/local.md docs/maintenance/coverage.json spec/src/local-jj.md spec/src/cli.md spec/src/architecture.md spec/src/documentation.md spec/src/verification.md
```

**Notes**

combination-join-format-final-rechecked.log: Final changed document format check passes; milestones excluded.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

