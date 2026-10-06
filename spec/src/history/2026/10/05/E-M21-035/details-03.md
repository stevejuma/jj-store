---
schema: 1
id: E-M21-035
---

# E-M21-035 details 3

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
target/debug/deps/stack_setup_preparation-2ac29e3a079557b3 --exact actual_setup_retains_original_history_refs_and_dirty_bytes_with_unpublished_merge_wip
```

**Notes**

setup-provider-exact-final.log: Final exact real setup generation/retention target passes against current compiled source.
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
target/debug/deps/stack_registered_preparation-9418350f0da02d0a --exact setup::registered_setup_creates_real_merge_wip_and_selects_combination_only_after_native_checkout
```

**Notes**

setup-registered-exact-final.log: Final exact registered setup/publication/pointer/selection target passes.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-jj --test stack_setup_preparation --test stack_operation_archive --test stack_reword_preparation --test stack_source_recovery --test stack_unchanged_checkout --test stack_head_publication --test stack_preparation_retention --test stack_tree_checkout --test stack_foreground
```

**Notes**

setup-provider-regressions.log: 113 passing parents. Owning setup, archive, retention, source recovery, publication, foreground and unchanged/tree-checkout consumers. Isolated helpers are invoked by passing parents.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-server --test stack_command_journal --test stack_combinations --test stack_local_preparation --test stack_local_execution -- --test-threads=1
```

**Notes**

setup-server-isolated-final.log: 38 passing parents. Complete affected private metadata/preparation/execution family passes with serial harness scheduling; tests retain their own concurrency, cancellation and abrupt process-loss scenarios. This does not claim a default-parallel pass.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-cli --test stack_registered_preparation
```

**Notes**

setup-registered-final-retry.log: 19 passing parents. Complete actual registered preparation/execution family, including seven setup parents; no fixture combination is consulted for setup.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-server --lib stack_
```

**Notes**

setup-metadata-unit.log: 16 passing parents. Private metadata fault, bound completion and retained-plan unit parents. Synthetic metadata evidence stays separate from actual setup execution.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-cli --test stack_checkout_plans
```

**Notes**

setup-plan-regressions.log: 48 passing parents. Direct journal/checkout-proof consumers, real native completion and original combination coordination; isolated process-loss helpers run through their parents.
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
CARGO_INCREMENTAL=0 cargo clippy --locked -p jj-store-jj -p jj-store-server -p jj-store-cli --all-targets --no-deps -- -D warnings
```

**Notes**

setup-lint.log: Final owning provider/server and direct CLI-consumer all-target strict type/lint passes.
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

tool formatting outputs: Rust changes formatted after source/test edits.
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

setup-rust-format.log: Pinned Rust format lane passes.
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
pnpm check:boundaries
```

**Notes**

setup-boundaries.log: 17-package ownership/upstream boundaries pass.
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
pnpm check:architecture
```

**Notes**

setup-architecture.log: Application architecture passes.
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
pnpm test:governance
```

**Notes**

setup-governance.log: 50 governance cases pass.
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
pnpm docs:review configuration 'Reviewed actual explicit setup from exact retained target/merge/WIP archives, registered authorization and complete physical guards. Initial combination binding precedes publication, native pointer completion precedes selected state and final acknowledgement; original interruption replay keeps its candidate and identity. Unsnapshotted edits remain on disk, while dirty capture, membership, transport and writable UI remain unqualified.'
```

**Notes**

setup-docs-review.log: Server private protocol reviewed against the final source.
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
pnpm docs:review local 'Reviewed explicit retained initial merge/WIP generation, exact source and known bookmark resolution, preservation of original refs/workspaces, and registered bound completion. The WIP retains the prior baseline tree while unsnapshotted edits remain guarded in place; setup does not qualify dirty capture, membership/switching, command transport or writable UI.'
```

**Notes**

setup-local-docs-review.log: Provider and local guide reviewed against final source.
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
pnpm exec prettier --write docs/implementation-plan.md docs/site/technical/local.md spec/src/local-jj.md spec/src/jj-integration.md spec/src/cli.md spec/src/verification.md
```

**Notes**

setup-docs-format.log: Changed-document formatting; ledger excluded.
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
pnpm exec prettier --write docs/implementation-plan.md spec/src/architecture.md
```

**Notes**

setup-docs-format-final.log: Updated implementation table and architecture formatted.
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
pnpm exec prettier --write spec/src/documentation.md
```

**Notes**

setup-documentation-format.log: Owning documentation requirements formatted.
<!-- /run -->
<!-- /evidence-field -->

