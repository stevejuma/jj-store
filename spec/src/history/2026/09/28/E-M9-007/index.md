---
schema: 1
id: E-M9-007
---

# E-M9-007

<!-- evidence-field:id markdown -->
## Id

E-M9-007
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M9
- M9.2
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

- crates/jj-agent/src/delta.rs
- crates/jj-agent/src/indexes.rs
- crates/jj-agent/src/lib.rs
- crates/jj-agent/src/objects.rs
- crates/jj-agent/src/transcript.rs
- crates/jj-agent/tests/activity.rs
- crates/jj-agent/tests/indexes.rs
- crates/jj-store-wal/src/publication.rs
- crates/jj-store-wal/src/publication/agent_catalog.rs
- crates/jj-store-wal/src/publication/agent_reads.rs
- crates/jj-store-wal/tests/agent_catalog.rs
- spec/src/collaboration.md
- spec/src/publication.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-agent/tests/indexes.rs#auxiliary_catalog_objects_validate_keys_and_strong_edges
- crates/jj-agent/tests/indexes.rs#auxiliary_catalog_objects_reject_wrong_kinds_and_receipt_order
- crates/jj-store-wal/tests/agent_catalog.rs#persistent_catalog_splits_and_keeps_old_root_pinned
- crates/jj-store-wal/tests/agent_catalog.rs#pinned_reads_page_catalog_relations_and_transcript_ranges
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-agent --test indexes --locked
```

**Notes**

The test-first compile produced fourteen missing type and variant errors because alias, relation, source and projection schemas did not exist.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-wal --test agent_catalog pinned_reads_page_catalog_relations_and_transcript_ranges --locked -- --exact
```

**Notes**

The focused compile produced five missing field or method errors for linked transcript pages and bounded root-pinned reads.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-agent --locked; DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-wal --locked
```

**Notes**

All 13 AgentStore model tests and 35 WAL catalog, inventory and publication tests pass, including auxiliary catalog validation, radix continuation, relation traversal and byte-bounded transcript continuation.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo fmt --all -- --check
```

**Notes**

The focused formatter reported mechanical wrapping in five affected Rust source and test files.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --check spec/src/collaboration.md spec/src/publication.md spec/src/milestones.md
```

**Notes**

The focused formatter required prose wrapping and milestone table alignment in two protected specification files.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo clippy -p jj-agent -p jj-store-wal --all-targets --locked -- -D warnings; DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo fmt --all -- --check
```

**Notes**

Both affected crates are warning-free and formatted. The new indexes and read modules contain 279 and 276 physical lines, below the 300 counted-line production limit, and the extended test contains 368 lines.
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
pnpm exec prettier --check spec/src/collaboration.md spec/src/publication.md spec/src/milestones.md; pnpm check:boundaries; pnpm test:governance; pnpm spec:validate; pnpm spec:first; pnpm check:progress; pnpm spec:build
```

**Notes**

Formatting, all 12 package boundaries, 26 governance tests, 84 requirements, eleven protected files across three canonical chapters and 60 progress tasks pass, and the canonical book builds.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Version-bound Codex, Cursor and OpenCode fixtures and deterministic normalizers
- Released registry acceptance for @lapismd/agent-redaction
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; AgentStore schema v1; Codex CLI 0.156.1; Cursor 3.22.7; OpenCode 1.18.30.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

AliasV1, RelationPageV1, CaptureSourceStateV1 and ProjectionCatalogV1 populate the four remaining typed catalogs. Root-pinned reads page radix catalogs by stable key, relations by page/offset and transcript chunks by source generation and sequence under explicit count and byte ceilings.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


