---
schema: 1
id: E-M21-028
---

# E-M21-028 details 3

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
pnpm spec:validate
```

**Notes**

tree-checkout-spec-validation.log: five validators, 23 chapters, 173 requirements. Final ledger-aware validation follows.
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
CARGO_INCREMENTAL=0 cargo clippy --locked -p jj-store-jj -p jj-store-server -p jj-store-cli --all-targets --no-deps -- -D warnings
```

**Notes**

tree-checkout-lint.log: one comparison-to-empty lint in the root-directory witness check; replaced with is_empty without weakening diagnostics.
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
CARGO_INCREMENTAL=0 cargo clippy --locked -p jj-store-jj -p jj-store-server -p jj-store-cli --all-targets --no-deps -- -D warnings
```

**Notes**

tree-checkout-lint-final.log: owning provider and direct server/CLI consumers pass strict all-target no-deps clippy.
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
CARGO_INCREMENTAL=0 cargo test -p jj-store-jj --test stack_tree_checkout --locked
```

**Notes**

tree-checkout-after-lint.log: all 18 new provider parents pass after the final source lint fix; one worker is intentionally isolated and killed by its passing parent.
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
pnpm docs:check
```

**Notes**

tree-checkout-docs-check.log: 139-file documentation closure, valid links/anchors and nine offline Mermaid diagrams in both themes.
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
pnpm check:progress
```

**Notes**

tree-checkout-progress.log: 142 tasks; milestone completion unchanged.
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
pnpm spec:first
```

**Notes**

tree-checkout-spec-first.log: 16 protected files/five canonical chapters.
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
pnpm format:rust:check
```

**Notes**

tree-checkout-rust-format.log: Rust formatting passes.
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
pnpm docs:review local 'Reviewed exact retained tree-changing plan capture and revalidation, raw object identity/size bounds, pinned native metadata bytes, physical file/directory stamps, unsafe/dirty obstacle rejection and original foreground ownership through cancellation. The initial resolved-file/full-sparse preparation profile and killed-worker reopening do not qualify durable plan/journal storage, per-path installation, native tree-state completion or production commands.'
```

**Notes**

tree-checkout-docs-review-final.log: final owning source fingerprint acknowledgement after the source lint fix.
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
pnpm exec prettier --check docs/implementation-plan.md docs/site/technical/local.md docs/maintenance/coverage.json spec/src/local-jj.md spec/src/architecture.md spec/src/jj-integration.md spec/src/documentation.md spec/src/verification.md
```

**Notes**

tree-checkout-format-docs-final.log: docs:review rewrites coverage.json with JSON indentation differing from repository Prettier layout; one formatting failure.
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
pnpm exec prettier --write docs/maintenance/coverage.json
```

**Notes**

tree-checkout-coverage-format.log: format only the changed review acknowledgement; ledger excluded.
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
pnpm exec prettier --check docs/implementation-plan.md docs/site/technical/local.md docs/maintenance/coverage.json spec/src/local-jj.md spec/src/architecture.md spec/src/jj-integration.md spec/src/documentation.md spec/src/verification.md
```

**Notes**

tree-checkout-format-docs-pass.log: all changed docs/review acknowledgement pass; ledger remains excluded.
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

tree-checkout-progress-final.log: final 142-task progress/evidence pass.
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
pnpm spec:first
```

**Notes**

tree-checkout-spec-first-final.log: final 16 protected files/five canonical chapters pass.
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
pnpm spec:validate
```

**Notes**

tree-checkout-spec-validation-final.log: final five validators/23 chapters/173 requirements pass.
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
pnpm docs:check
```

**Notes**

tree-checkout-docs-check-final.log: final 139-file closure, links/anchors and nine offline diagrams in both themes pass.
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
python3 .local/stacks-backend-evidence/tree-checkout-ledger-audit.py
```

**Notes**

tree-checkout-ledger-audit.log: 510 objects; every one of the 509 original raw evidence bodies is byte-identical to the enclosing parent.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

