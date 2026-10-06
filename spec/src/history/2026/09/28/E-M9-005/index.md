---
schema: 1
id: E-M9-005
---

# E-M9-005

<!-- evidence-field:id markdown -->
## Id

E-M9-005
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

- crates/jj-agent/src/lib.rs
- crates/jj-agent/src/objects.rs
- crates/jj-agent/src/provenance.rs
- crates/jj-agent/tests/provenance.rs
- crates/jj-store-wal/src/publication.rs
- crates/jj-store-wal/src/publication/agent.rs
- crates/jj-store-wal/src/publication/domains.rs
- crates/jj-store-wal/tests/publication.rs
- spec/src/collaboration.md
- spec/src/publication.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-agent/tests/provenance.rs#associations_separate_agent_edges_from_native_retention_roots
- crates/jj-agent/tests/provenance.rs#checkpoints_pin_exact_agent_versions_and_native_anchors
- crates/jj-agent/tests/provenance.rs#invalid_causality_and_paths_fail_closed
- crates/jj-store-wal/tests/publication.rs#agent_publication_retains_exact_native_association_dependencies
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-agent --test provenance --locked
```

**Notes**

The test-first compile proved that checkpoint/code-association schemas and native retention roots did not exist.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-wal --test publication agent_publication_retains_exact_native_association_dependencies --locked -- --exact
```

**Notes**

The focused publication test proved that agent-only publication did not add a selected association's native operation and commit closures to inventory.
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

All ten AgentStore model tests and all 32 WAL inventory/publication tests pass, including exact native retention, missing-source rejection and cold recovery.
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

Both affected crates are warning-free and match the pinned Rust formatting policy. Provenance and domain-publication modules remain below 300 counted lines.
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

The focused formatter required mechanical prose reflow in collaboration.md and publication.md.
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
pnpm check:boundaries; pnpm test:governance; pnpm spec:validate; pnpm spec:first; pnpm check:progress; pnpm spec:build; pnpm exec prettier --check <changed prose>; DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo fmt --all -- --check
```

**Notes**

All 12 package boundaries, 26 governance tests, 84 requirements, eight mapped protected files and 60 task records pass, and the canonical book builds. Prettier and Rust formatting pass; provenance.rs is 279 and domains.rs is 297 counted lines.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- AgentStore root-pinned read/staging facade and typed publication deltas
- Version-bound Codex, Cursor and OpenCode fixtures and normalizers
- Released registry acceptance for @lapismd/agent-redaction
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; AgentStore checkpoint/code-association schema v1; manifest format 2.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

AgentCheckpointVersionV1 and CodeAssociationV1 separate immutable AgentStore edges, stable lookup identities and exact native operation/commit retention roots. Agent domain publication extends the existing verified native inventory without changing code state.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


