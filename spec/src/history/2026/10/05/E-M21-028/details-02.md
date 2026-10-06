---
schema: 1
id: E-M21-028
---

# E-M21-028 details 2

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
CARGO_INCREMENTAL=0 cargo test -p jj-store-jj --test stack_unchanged_checkout --locked
```

**Notes**

tree-checkout-before.log: unchanged-tree baseline 10 passing parents and one ignored subprocess helper.
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
CARGO_INCREMENTAL=0 cargo test -p jj-store-jj --test stack_tree_checkout --locked
```

**Notes**

tree-checkout-unavailable.log: The new exact target cannot compile before implementation: missing prepare_tree_checkout API; no existing physical checkout plan is available.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
CARGO_INCREMENTAL=0 cargo test -p jj-store-jj --test stack_tree_checkout --locked
```

**Notes**

tree-checkout-first.log: Initial implementation compile errors: duplicate existing git_path accessor, &amp;&amp;str descriptor argument and moving gix Object data. Fixed without dependency/API workarounds.
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
CARGO_INCREMENTAL=0 cargo test -p jj-store-jj --test stack_tree_checkout --locked
```

**Notes**

tree-checkout-second.log: Native candidate fixture transaction rejected pending rewrites; fixture now rebases descendants before leaving unpublished.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
CARGO_INCREMENTAL=0 cargo test -p jj-store-jj --test stack_tree_checkout --locked
```

**Notes**

tree-checkout-third.log: Provider incorrectly expected empty archived conflict labels for a resolved commit. Pinned native profile represents the resolved label as one empty string; corrected that exact check.
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
CARGO_INCREMENTAL=0 cargo test -p jj-store-jj --test stack_tree_checkout --locked
```

**Notes**

tree-checkout-fourth.log: One original exact positive passes without changing physical or checkout state.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
CARGO_INCREMENTAL=0 cargo test -p jj-store-jj --test stack_tree_checkout --locked
```

**Notes**

tree-checkout-complete-first.log: Cancellation fixture assertion accidentally required RecoveredHistory Debug; replaced unwrap_err with pattern matching.
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
CARGO_INCREMENTAL=0 cargo test -p jj-store-jj --test stack_tree_checkout --locked
```

**Notes**

tree-checkout-complete-second.log: 13 provider cases pass.
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
CARGO_INCREMENTAL=0 cargo test -p jj-store-jj --test stack_tree_checkout --locked
```

**Notes**

tree-checkout-complete-third.log: 15 provider cases pass, including symlink candidate rejection and a recomputed tampered request digest.
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
CARGO_INCREMENTAL=0 cargo test -p jj-store-jj --test stack_tree_checkout --locked
```

**Notes**

tree-checkout-complete-final.log: 18 final provider parents pass; one isolated worker is invoked and killed by its passing parent. Includes real stock-JJ sparse checkout rejection, missing-parent invalidation and exact saved-plan reopening. This log also supersedes an earlier 16-parent pass before the two final cases were added.
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
cargo fmt --all
```

**Notes**

Changed Rust source/test formatting.
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
CARGO_INCREMENTAL=0 cargo test -p jj-store-jj --test stack_foreground --test stack_head_publication --test stack_operation_archive --test stack_preparation_retention --test stack_reword_preparation --test stack_source_recovery --test stack_unchanged_checkout --test working_tree_baseline --test working_tree_current --locked
```

**Notes**

tree-checkout-provider-regressions.log: 98 passing direct provider cases across nine binaries. Six ignored worker helpers are invoked through their passing parents; no ordinary reader behavior is changed.
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
CARGO_INCREMENTAL=0 cargo check --locked -p jj-store-jj -p jj-store-server -p jj-store-cli --all-targets
```

**Notes**

tree-checkout-check.log: owning provider and direct consumer type/target checks; no public transport or runtime capability changed.
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
pnpm check:boundaries
```

**Notes**

tree-checkout-boundaries.log: 17 packages.
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
pnpm check:architecture
```

**Notes**

tree-checkout-architecture.log: 1090 modules.
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
pnpm test:governance
```

**Notes**

tree-checkout-governance.log: 50 source-governance cases.
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
pnpm exec prettier --write docs/implementation-plan.md docs/site/technical/local.md spec/src/local-jj.md spec/src/architecture.md spec/src/jj-integration.md spec/src/documentation.md spec/src/verification.md
```

**Notes**

tree-checkout-format-docs.log: changed-document formatting; milestones.md excluded.
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
pnpm docs:review local 'Reviewed exact retained tree-changing plan capture and revalidation, raw object identity/size bounds, pinned native metadata bytes, physical file/directory stamps, unsafe/dirty obstacle rejection and original foreground ownership through cancellation. The initial resolved-file/full-sparse preparation profile and killed-worker reopening do not qualify durable plan/journal storage, per-path installation, native tree-state completion or production commands.'
```

**Notes**

tree-checkout-docs-review.log: owning JJ source/manual review acknowledgement.
<!-- /run -->
<!-- /evidence-field -->

