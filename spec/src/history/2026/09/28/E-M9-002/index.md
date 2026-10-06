---
schema: 1
id: E-M9-002
---

# E-M9-002

<!-- evidence-field:id markdown -->
## Id

E-M9-002
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

- Cargo.lock
- Cargo.toml
- crates/jj-agent/Cargo.toml
- crates/jj-agent/src/codec.rs
- crates/jj-agent/src/ids.rs
- crates/jj-agent/src/lib.rs
- crates/jj-agent/src/objects.rs
- crates/jj-agent/tests/model.rs
- crates/jj-store-wal/Cargo.toml
- crates/jj-store-wal/src/publication.rs
- crates/jj-store-wal/src/publication/agent.rs
- crates/jj-store-wal/src/publication/collaboration.rs
- crates/jj-store-wal/src/publication/domains.rs
- crates/jj-store-wal/src/records.rs
- crates/jj-store-wal/tests/publication.rs
- scripts/check-boundaries.mjs
- scripts/governance.test.mjs
- spec/src/architecture.md
- spec/src/collaboration.md
- spec/src/milestones.md
- spec/src/publication.md
- spec/src/spec-governance.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-agent/tests/model.rs#stable_ids_reject_noncanonical_or_cross_type_values
- crates/jj-agent/tests/model.rs#agent_root_and_index_pages_validate_strong_dependencies
- crates/jj-agent/tests/model.rs#canonical_codec_rejects_unknown_fields_noncanonical_bytes_and_wrong_ids
- crates/jj-store-wal/tests/publication.rs#agent_roots_require_complete_typed_dependency_closures
- crates/jj-store-wal/tests/publication.rs#domain_publications_preserve_roots_without_staling_code
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-agent --test model --offline
```

**Notes**

The test-first compile proved that no owned AgentStore ID, root, catalog or canonical codec model existed.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-wal --test publication agent_roots_require_complete_typed_dependency_closures --locked -- --exact
```

**Notes**

The WAL regression proved that publication still lacked an owned jj-agent dependency and typed object/root staging methods.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-agent --test model --locked
```

**Notes**

All three model tests pass for stable typed IDs, canonical empty roots, bounded radix pages, strong references and fail-closed decoding.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-wal --test publication agent_roots_require_complete_typed_dependency_closures --locked -- --exact
```

**Notes**

Missing agent objects and semantic-kind substitutions fail closed before a typed root can be staged.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-wal --test publication domain_publications_preserve_roots_without_staling_code --locked -- --exact
```

**Notes**

Agent publication now selects a typed catalog root while preserving the sibling review root and native code-state precondition.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-wal --locked
```

**Notes**

All 31 WAL inventory/publication tests pass with typed empty roots used by initialization, legacy enablement, publication, checkpoint and cold recovery.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo clippy -p jj-agent -p jj-store-wal --all-targets --locked -- -D warnings
```

**Notes**

The new model crate and affected publication layer are warning-free across all targets.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first
```

**Notes**

The mapped gate required the package-boundary checker change to document jj-agent layering in the canonical governance chapter.
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
pnpm test:governance; pnpm spec:validate; pnpm spec:first; pnpm check:progress; pnpm check:boundaries
```

**Notes**

All 25 governance tests, 84 requirements, five mapped chapters, 60 task records and all 12 owned package boundaries pass.
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
pnpm spec:build; DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm format:rust:check; pnpm exec prettier --check <changed prose and scripts>
```

**Notes**

The canonical book builds and all affected Rust, Markdown and JavaScript files match tracked formatting policy.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Immutable session, tool, evidence, transcript, checkpoint and association
  object schemas
- AgentStore root-pinned read/staging facade and typed publication deltas
- Packaged sanitizer plus Codex, Cursor and OpenCode fixtures and normalizers
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; AgentStore schema v1 foundation; manifest format 2.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

AgentRootV1 canonically selects seven optional radix catalogs. Stable entity IDs are 128-bit typed values; repository-bound content IDs and catalog keys are 256-bit typed values. Agent roots/pages use existing immutable record envelopes and complete bounded dependency audits.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


