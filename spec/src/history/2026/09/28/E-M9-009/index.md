---
schema: 1
id: E-M9-009
---

# E-M9-009

<!-- evidence-field:id markdown -->
## Id

E-M9-009
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

- crates/jj-agent/src/activity.rs
- crates/jj-agent/src/capture.rs
- crates/jj-agent/src/normalize.rs
- crates/jj-agent/tests/activity.rs
- crates/jj-store-wal/src/publication.rs
- crates/jj-store-wal/src/publication/agent_capture.rs
- crates/jj-store-wal/src/publication/agent_capture_projection.rs
- crates/jj-store-wal/src/publication/agent_capture_records.rs
- crates/jj-store-wal/tests/agent_catalog.rs
- spec/src/collaboration.md
- spec/src/publication.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-agent/tests/activity.rs#normalized_events_and_tool_snapshots_keep_typed_content_edges
- crates/jj-store-wal/tests/agent_catalog.rs#normalized_capture_stages_one_root_pinned_publication_delta
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-wal --test agent_catalog normalized_capture_stages_one_root_pinned_publication_delta --locked -- --exact
```

**Notes**

The test-first compile proved that no normalized capture-to-AgentStore staging boundary existed.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-wal --test agent_catalog normalized_capture_stages_one_root_pinned_publication_delta --locked -- --exact
```

**Notes**

The first implementation reached publication and was denied because the existing fixture guard intentionally lacked agent-domain authorization; the fixture now grants only that owned domain path.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-wal --test agent_catalog normalized_capture_stages_one_root_pinned_publication_delta --locked -- --exact
```

**Notes**

The exact regression stages one Codex evidence/event/tool/session/source/projection closure, selects it through one AgentDelta publication and replays the original receipt.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-agent --locked; DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-wal --locked
```

**Notes**

All 20 AgentStore model/normalizer tests and all 36 WAL catalog, inventory and publication tests pass, including explicit permission/file event encoding and the complete capture projection.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo clippy -p jj-agent -p jj-store-wal --all-targets --locked -- -D warnings; DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo fmt --all -- --check
```

**Notes**

Both affected crates are warning-free and formatted. New production modules contain at most 283 physical lines and the extended agent-catalog test contains 498 lines.
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
pnpm check:boundaries; pnpm test:governance; pnpm spec:validate; pnpm spec:first; pnpm check:progress; pnpm spec:build; pnpm exec prettier --check spec/src/collaboration.md spec/src/publication.md spec/src/milestones.md
```

**Notes**

Package boundaries, governance, canonical requirement and progress checks pass; the canonical book builds and changed specification prose is formatted.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Owned integration discovery, durable pending spool and setup/status commands
- Released registry acceptance for @lapismd/agent-redaction
- Pinned real-harness capability matrix and smoke tests in M9.6
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; AgentStore capture projection schema v1; manifest format 2; exact normalizer profiles from E-M9-008.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

AgentEventV1 adds permission and file observations. stage_agent_capture produces repository-bound immutable evidence/content/event/tool/session/alias/source/projection objects and one typed AgentDeltaV1 against an explicit pinned root; publication remains the existing receipt-bearing domain CAS.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


