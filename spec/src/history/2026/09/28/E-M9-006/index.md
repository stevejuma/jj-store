---
schema: 1
id: E-M9-006
---

# E-M9-006

<!-- evidence-field:id markdown -->
## Id

E-M9-006
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
- crates/jj-agent/src/lib.rs
- crates/jj-agent/tests/delta.rs
- crates/jj-store-wal/src/publication.rs
- crates/jj-store-wal/src/publication/agent.rs
- crates/jj-store-wal/src/publication/agent_catalog.rs
- crates/jj-store-wal/src/publication/agent_delta.rs
- crates/jj-store-wal/src/publication/domains.rs
- crates/jj-store-wal/src/publication.rs
- crates/jj-store-wal/tests/agent_catalog.rs
- crates/jj-store-wal/tests/publication.rs
- spec/src/collaboration.md
- spec/src/publication.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-agent/tests/delta.rs#typed_deltas_reject_duplicate_keys_wrong_kinds_and_noops
- crates/jj-store-wal/tests/agent_catalog.rs#persistent_catalog_splits_and_keeps_old_root_pinned
- crates/jj-store-wal/tests/publication.rs#agent_delta_publications_rebase_disjoint_keys_and_pin_old_roots
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-agent --test delta --locked
```

**Notes**

The test-first compile proved that the typed delta catalog/edit contract did not exist.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-wal --test publication agent_delta_publications_rebase_disjoint_keys_and_pin_old_roots --locked -- --exact
```

**Notes**

The focused compile produced nine missing API errors for root-pinned reads and typed agent-delta publication.
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

All 11 AgentStore model tests and 34 WAL catalog/inventory/publication tests pass, including branch splitting, pinned old-root reads, closure membership, disjoint rebase, same-key staleness and receipt replay.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo clippy -p jj-agent -p jj-store-wal --all-targets --locked -- -D warnings; DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo fmt --all -- --check
```

**Notes**

The affected model/facade/publication code is warning-free and formatted. New production modules remain under 300 counted lines and the new test remains under 500.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --check spec/src/collaboration.md spec/src/publication.md spec/src/milestones.md
```

**Notes**

The focused formatter found mechanical prose wrapping and table alignment drift in the three protected specification files.
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
pnpm exec prettier --check spec/src/collaboration.md spec/src/publication.md spec/src/milestones.md; pnpm spec:validate; pnpm spec:first; pnpm check:progress; pnpm spec:build
```

**Notes**

The corrected specification formatting passes. All 84 requirements validate, all ten protected files map to three canonical chapters, all 60 progress tasks are consistent, and the canonical book builds.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Alias, relation, source and projection schemas plus bounded enumeration,
  transcript streaming and relation traversal
- Version-bound Codex, Cursor and OpenCode fixtures and normalizers
- Released registry acceptance for @lapismd/agent-redaction
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; AgentStore delta schema v1; 32 selections per persistent catalog leaf; manifest format 2.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

AgentDeltaV1 carries typed expected/replacement edits for session, checkpoint and association catalogs. AgentStoreSnapshot binds stable lookup and immutable-object reads to one audited root closure. AgentDeltaRequest uses a root-independent stable intent and the existing domain CAS/receipt path.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


