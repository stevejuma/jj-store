---
schema: 1
id: E-READ-070
---

# E-READ-070 details 2

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-wal --lib --locked projection_pages_read_copy_parent_edges_once_under_unrelated_inventory_pressure -- --nocapture
```

**Notes**

Baseline compilation fails with two missing-API errors; no functional test ran. projection-before-070.log.
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
cargo test -p jj-store-wal --lib --locked projection_pages_read_copy_parent_edges_once_under_unrelated_inventory_pressure -- --nocapture
```

**Notes**

One exact target passes; projection-after-070.log.
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
cargo test -p jj-store-wal --lib --locked projection_ -- --nocapture
```

**Notes**

Four projection units pass; projection-units-070.log.
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
cargo fmt --all
```

**Notes**

Formatting completed after source and regression edits.
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
cargo test -p jj-store-wal --test publication --locked inventory_projections_preserve_pins_copy_edges_and_manifest_only_visibility -- --nocapture
```

**Notes**

One fake-store control-plane integration target passes; projection-pins-070.log.
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
cargo test -p jj-store-wal --lib --locked inventory_pages_continue_before_byte_admission_without_losing_descriptors -- --nocapture
```

**Notes**

One extended byte-admission target passes, including three descriptors each retaining90000 dependencies; projection-bytes-070.log.
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
cargo test -p jj-store-wal --lib --locked -- --nocapture
```

**Notes**

16.64s; .local/bounded-reads/performance/projection-library-070.log
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
cargo test -p jj-store-wal --test publication --locked -- --nocapture
```

**Notes**

1.55s; .local/bounded-reads/performance/projection-publication-070.log
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
cargo check -p jj-store-wal --all-targets --locked
```

**Notes**

6.28s; .local/bounded-reads/performance/projection-package-070.log
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
cargo fmt --all -- --check
```

**Notes**

2.66s; .local/bounded-reads/performance/projection-rust-format-070.log
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

1.31s; .local/bounded-reads/performance/projection-boundaries-070.log
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

1.44s; .local/bounded-reads/performance/projection-architecture-070.log
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
pnpm spec:first
```

**Notes**

1.94s; .local/bounded-reads/performance/projection-spec-first-070.log
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
pnpm spec:validate
```

**Notes**

2.49s; .local/bounded-reads/performance/projection-spec-validation-070.log
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

3.57s; .local/bounded-reads/performance/projection-progress-070.log
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
pnpm exec prettier --check spec/src/publication.md spec/src/architecture.md spec/src/milestones.md docs/implementation-plan.md
```

**Notes**

2.68s; .local/bounded-reads/performance/projection-document-format-070.log
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
pnpm exec prettier --write spec/src/architecture.md spec/src/publication.md docs/implementation-plan.md
```

**Notes**

Changed document formatting passes; projection-document-write-070.log.
<!-- /run -->
<!-- /evidence-field -->

