---
schema: 1
id: E-M9-012
---

# E-M9-012

<!-- evidence-field:id markdown -->
## Id

E-M9-012
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M9
- M9.3
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-28
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

implementation
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- Cargo.lock
- crates/jj-review/src/delta.rs
- crates/jj-review/src/lib.rs
- crates/jj-review/tests/delta.rs
- crates/jj-store-wal/Cargo.toml
- crates/jj-store-wal/src/publication.rs
- crates/jj-store-wal/src/publication/collaboration.rs
- crates/jj-store-wal/src/publication/domains.rs
- crates/jj-store-wal/src/publication/review.rs
- crates/jj-store-wal/src/publication/review_audit.rs
- crates/jj-store-wal/src/publication/review_catalog.rs
- crates/jj-store-wal/src/publication/review_reads.rs
- crates/jj-store-wal/src/records.rs
- crates/jj-store-wal/tests/publication.rs
- crates/jj-store-wal/tests/review_catalog.rs
- scripts/check-boundaries.mjs
- spec/src/architecture.md
- spec/src/collaboration.md
- spec/src/publication.md
- spec/src/spec-governance.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-review/tests/delta.rs#review_delta_accepts_typed_expected_replacement_edits
- crates/jj-review/tests/delta.rs#review_delta_rejects_noops_duplicates_and_semantic_kind_substitution
- crates/jj-store-wal/tests/review_catalog.rs#persistent_review_catalog_splits_pages_and_keeps_old_root_pinned
- crates/jj-store-wal/tests/review_catalog.rs#selecting_a_thread_rejects_cross_object_thread_identity_mismatch
- crates/jj-store-wal/tests/review_catalog.rs#review_root_rejects_catalog_values_with_the_wrong_semantic_kind
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-review --test delta --locked
```

**Notes**

The test-first compile reported the missing typed review catalog kind, expected/replacement edit and delta APIs.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-review --test delta --locked
```

**Notes**

Both focused delta tests pass for typed selection changes and rejection of no-op, duplicate or semantic-kind-substituting edits.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-wal --test review_catalog --locked
```

**Notes**

The test-first compile produced twelve missing staging, pinned-store and delta methods before the typed WAL integration existed.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-wal --test review_catalog --locked
```

**Notes**

Three focused WAL regressions pass for 40-thread COW pagination and old-root pinning, selected-object isolation, cross-object thread/event agreement and semantic catalog value kinds.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-review --locked; DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-wal --locked; DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo clippy -p jj-review -p jj-store-wal --all-targets --locked -- -D warnings; DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo fmt --all -- --check
```

**Notes**

All 10 ReviewStore model tests and 39 WAL inventory, publication, recovery and catalog tests pass; both crates are warning-free and formatted. Production modules contain at most 274 physical lines and the new WAL regression has 338 lines.
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
pnpm check:boundaries
```

**Notes**

All 13 workspace boundaries pass with the exact jj-store-wal to jj-review validation dependency admitted and reverse or higher-layer edges still rejected.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --check spec/src/architecture.md spec/src/collaboration.md spec/src/publication.md spec/src/spec-governance.md spec/src/milestones.md
```

**Notes**

The focused formatter reported mechanical wrapping and milestone-table alignment in four changed canonical specification files.
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
pnpm exec prettier --check spec/src/architecture.md spec/src/collaboration.md spec/src/publication.md spec/src/spec-governance.md spec/src/milestones.md; pnpm test:governance; pnpm spec:validate; pnpm spec:first; pnpm check:progress; pnpm spec:build
```

**Notes**

Changed canonical files are formatted; all 26 governance tests, 84 requirements, 16 protected files across five canonical chapters and 60 progress tasks pass, and the canonical book builds.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Stale-safe create thread, append reply, edit, tombstone, hide, resolve and
  reopen commands with receipt replay
- Typed relation/status catalogs and conservative explicit re-anchoring
- LocalTrusted CLI/HTTP contracts and shared Community/Design Core presentation
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; ReviewDeltaV1 and ReviewStore root/object schema v1; 32 selections per persistent catalog leaf; 256-row bounded catalog reads.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Typed review objects use repository-bound ReviewObject records. ReviewStoreSnapshot pins one audited root closure; ReviewDeltaV1 copy-on-write updates thread, comment and thread-event catalogs while preserving old roots and rejecting stale or semantically mismatched selections.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


