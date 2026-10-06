---
schema: 1
id: E-READ-066
---

# E-READ-066

<!-- evidence-field:id markdown -->
## Id

E-READ-066
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M13.4
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-05
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

tmrztksn
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

immutable-inventory-addition-pages
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; macOS ARM64.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Authoritative catalog, control record, manifest, WAL, native object, DTO, cursor and receipt formats unchanged. No migration, service restart or installed executable change.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Exact changed-path regression first, catalog shape/anchor/admission tests, complete WAL library and selected-snapshot publication regression; affected WAL all-target package check. Shared ordinary/addition descriptor hydration requires all library reader tests. UI, RPC, full workspace/publication suites, cold/browser/RustFS/scale/RSS/latency/fault/release qualification and generated Help/reference refresh deferred or out of scope.
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-PUB-002
- JST-PUB-006
- JST-API-002
- JST-API-004
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-wal/src/catalog/additions.rs
- crates/jj-store-wal/src/catalog.rs
- crates/jj-store-wal/src/publication/reader.rs
- crates/jj-store-wal/src/publication/reader_tests.rs
- crates/jj-store-wal/src/publication.rs
- crates/jj-store-wal/tests/publication.rs
- docs/implementation-plan.md
- spec/src/architecture.md
- spec/src/publication.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-wal/src/publication/reader_tests.rs#inventory_additions_seek_changed_paths_and_skip_unchanged_descriptors
- crates/jj-store-wal/src/publication/reader_tests.rs#inventory_pages_continue_before_byte_admission_without_losing_descriptors
- crates/jj-store-wal/src/catalog/additions.rs#additions_keep_shape_transitions_and_exclusive_missing_anchors_exact
- crates/jj-store-wal/src/catalog/additions.rs#additions_reject_replacements_missing_branches_and_invalid_references
- crates/jj-store-wal/src/catalog/additions.rs#additions_admit_reads_and_bytes_and_resume_after_nonempty_prefix
- crates/jj-store-wal/tests/publication.rs#inventory_additions_bind_checked_selections_and_exclude_staged_candidates
<!-- /evidence-field -->

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-wal --lib --locked publication::reader::tests::inventory_additions_seek_changed_paths_and_skip_unchanged_descriptors -- --exact --nocapture
```

**Notes**

32.72s; .local/bounded-reads/performance/inventory-additions-before-066.log
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
cargo test -p jj-store-wal --lib --locked publication::reader::tests::inventory_additions_seek_changed_paths_and_skip_unchanged_descriptors -- --exact --nocapture
```

**Notes**

8.01s; .local/bounded-reads/performance/inventory-additions-after-066.log
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
cargo test -p jj-store-wal --lib --locked catalog::additions::tests -- --nocapture
```

**Notes**

4.11s; .local/bounded-reads/performance/inventory-additions-catalog-066.log
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
cargo test -p jj-store-wal --test publication --locked inventory_additions_bind_checked_selections_and_exclude_staged_candidates -- --exact --nocapture
```

**Notes**

10.07s; .local/bounded-reads/performance/inventory-additions-scope-066.log
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
cargo test -p jj-store-wal --lib --locked -- --nocapture
```

**Notes**

14.46s; .local/bounded-reads/performance/inventory-additions-reader-066.log
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
cargo check -p jj-store-wal --all-targets --locked
```

**Notes**

12.04s; .local/bounded-reads/performance/inventory-additions-package-066.log
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
cargo fmt --all -- --check
```

**Notes**

2.72s; .local/bounded-reads/performance/inventory-additions-rust-format-066.log
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

1.34s; .local/bounded-reads/performance/inventory-additions-boundaries-066.log
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

1.47s; .local/bounded-reads/performance/inventory-additions-architecture-066.log
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
pnpm spec:first
```

**Notes**

1.84s; .local/bounded-reads/performance/inventory-additions-spec-first-066.log
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
pnpm spec:validate
```

**Notes**

2.62s; .local/bounded-reads/performance/inventory-additions-spec-validation-066.log
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
pnpm check:progress
```

**Notes**

3.79s; .local/bounded-reads/performance/inventory-additions-progress-066.log
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
pnpm exec prettier --check spec/src/publication.md spec/src/architecture.md spec/src/milestones.md docs/implementation-plan.md
```

**Notes**

2.80s; .local/bounded-reads/performance/inventory-additions-document-format-066.log
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:notes list -->
## Notes

- "Before: 4,421 reads and 4,099 descriptors for three additions among 4,096
  prior objects. Exact regression fails original fixed <=128 read bound. After:
  27 reads and exactly three descriptors across two pages, same bound."
- Immutable paired radix traversal skips equal references, admits <=1,024
  frontier entries, retains exact baseline selections and partitions <=32-entry
  leaves without full catalog clones. Addition descriptors share existing
  1..256/1,024-read/32MiB admission with independent 768-read/8MiB paths.
- Complete traversal from None is required for retention certification.
  Partial/anchored pages grant no complete baseline proof. Missing/replaced
  baseline values fail closed; no selected/staged or publication boundary
  changed.
- "11 distinct Rust tests pass: complete WAL library (10) plus checked selection
  publication target (1). Exact catalog tests and main regression also rerun."
- Main verified E-READ-062 landing and temporary workspace preserved. Linux,
  fixtures, services, installed executables and unrelated workspaces unchanged.
- All 506 historical evidence entries remain JSON-identical. Format, boundaries,
  architecture, spec-first, specification validation and progress checks pass.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-READ-066 details 2](details-02.md)
