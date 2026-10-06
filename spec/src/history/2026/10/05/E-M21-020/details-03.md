---
schema: 1
id: E-M21-020
---

# E-M21-020 details 3

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
pnpm exec prettier --write spec/src/architecture.md spec/src/cli.md spec/src/documentation.md spec/src/local-jj.md spec/src/verification.md docs/implementation-plan.md docs/site/technical/lifecycle.md docs/site/technical/local.md
```

**Notes**

stack-combinations-format.log: Changed canonical/manual pages formatted; ledger excluded.
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
pnpm exec prettier --write docs/maintenance/coverage.json
```

**Notes**

stack-combinations-coverage-format.log: Configuration source-review acknowledgement formatted.
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
pnpm exec prettier --check spec/src/architecture.md spec/src/cli.md spec/src/documentation.md spec/src/local-jj.md spec/src/verification.md docs/implementation-plan.md docs/site/technical/lifecycle.md docs/site/technical/local.md docs/maintenance/coverage.json
```

**Notes**

stack-combinations-doc-format-check.log: Changed canonical/manual documents pass; ledger excluded.
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
CARGO_INCREMENTAL=0 pnpm docs:check
```

**Notes**

stack-combinations-docs-check.log: Owning manual check passes; exact closure recorded in log.
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
pnpm docs:review configuration 'Documented installation-private combination metadata, exact retained merge/WIP anchors, pending/selected separation, journal-bound verified checkout selection, unchanged historical retry and scoped registered operation/WIP mismatch. Private state and real code retention fixtures remain distinct from simulated command publication/checkout receipts and unfinished setup, merge, membership, transport and guarded physical execution.'
```

**Notes**

stack-combinations-docs-review.log: metadata acceptance explicitly separated from actual execution.
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
pnpm docs:review configuration 'Reviewed the private combination metadata owner after final lint repairs. Scope, pending/selected separation, exact retained merge/WIP anchors, journal checkout proof, historical retry and actual registered operation/WIP staleness remain documented. Metadata fixture publication/checkout phases do not qualify setup, merge generation, membership, runtime transport or guarded checkout.'
```

**Notes**

stack-combinations-docs-review-final.log: metadata acceptance explicitly separated from actual execution.
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

Formatted implementation/test changes and removed unnecessary unit expressions without changing assertions.
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
pnpm exec prettier --write docs/maintenance/coverage.json
```

**Notes**

stack-combinations-coverage-format-final.log: source review refreshed after lint repair; ledger excluded.
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
CARGO_INCREMENTAL=0 cargo clippy --locked -p jj-store-server -p jj-store-cli --all-targets --no-deps -- -D warnings
```

**Notes**

stack-combinations-lint-final.log: All owning targets pass strict lint after two unused-unit repairs; no diagnostic suppressed.
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
CARGO_INCREMENTAL=0 pnpm docs:check
```

**Notes**

stack-combinations-docs-check-final.log: Final current-source manual check passes with valid links/anchors and both theme Mermaid rendering.
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
pnpm spec:first
```

**Notes**

stack-combinations-spec-first.log: 15 protected files/five canonical chapters pass.
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
pnpm spec:validate
```

**Notes**

stack-combinations-spec-validation.log: Five validators/23 chapters/173 requirements pass.
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
pnpm check:progress
```

**Notes**

stack-combinations-progress.log: 142 tasks pass; no completion promotion.
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
pnpm format:rust:check
```

**Notes**

stack-combinations-rust-format-final.log: Final owning/runtime formatting passes after lint fixes.
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

stack-combinations-progress-final.log: Final ledger passes 142 tasks; all 501 preceding evidence bodies byte-identical.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Combination metadata is durable and code archives/GC are real;
  Setup/CreateCombination effects and Published/Verified journal receipts in
  metadata tests are explicit isolated fixtures. They do not qualify actual
  setup, merge/WIP preservation, membership, switching or physical checkout.
- Stage 3 still requires exact lost-extra/derived-index recovery, operation-head
  publication preserving concurrent siblings, guarded checkout/durable fault
  recovery, and prepare/apply/status/discovery production composition. Writable
  capabilities remain unavailable.
- All seven approved stages remain required. Stages 4-7 need real combination
  engines, exact file then issued-hunk transfer/commit, all advertised graph
  commands/deletion policy/guarded undo, linked-native real-storage recovery and
  full cold production UI flows.
- Two inherited service strict-lint diagnostics remain unresolved/unwaived.
  No-deps owning server/CLI lint does not qualify aggregate dependency/service
  lint. Native linker emits __eh_frame size warning on the large server test
  binary; no product assertion or lint is weakened.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Implement exact source extra/index recovery and safe original operation publication, then guarded checkout and production command composition; continue remaining approved engines/UI wiring.
<!-- /evidence-field -->

<!-- evidence-field:out_of_scope markdown -->
## Out of scope

No production UI/startup/Storybook or generated transport changes, fresh visual review, screenshot comparisons/baselines, package publication, installed binary/service replacement, existing repository reset/migration, reserved-ref cleanup, linked-native/real-RustFS execution or unrelated root aggregate lanes.
<!-- /evidence-field -->

<!-- evidence-field:summary markdown -->
## Summary

33 unique direct cases pass: nine private combination cases (four integrations/five units), three registered-folder preparation cases, 13 journal and eight actual preparation integrations. Process-loss child executes through passing parent. Real retained archives survive metadata loss/GC; metadata Setup/CreateCombination and Published/Verified receipt fixtures explicitly do not qualify execution. Owning checks, strict no-deps lint, documentation and governance pass after recorded fixture/lint repairs. All 501 prior ledger bodies remain byte-identical. Source extra/index recovery, safe publication, guarded checkout, runtime command transport and Stages 4-7 remain incomplete.
<!-- /evidence-field -->

