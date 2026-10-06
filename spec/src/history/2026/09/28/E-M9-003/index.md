---
schema: 1
id: E-M9-003
---

# E-M9-003

<!-- evidence-field:id markdown -->
## Id

E-M9-003
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
- crates/jj-agent/src/catalog.rs
- crates/jj-agent/src/lib.rs
- crates/jj-agent/src/objects.rs
- crates/jj-agent/src/transcript.rs
- crates/jj-agent/tests/activity.rs
- spec/src/collaboration.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-agent/tests/activity.rs#session_snapshots_separate_evidence_events_tools_and_transcripts
- crates/jj-agent/tests/activity.rs#sanitized_evidence_requires_canonical_payloads_and_source_order
- crates/jj-agent/tests/activity.rs#normalized_events_and_tool_snapshots_keep_typed_content_edges
- crates/jj-agent/tests/activity.rs#transcript_manifests_reconstruct_from_their_own_chunk_page_closure
- crates/jj-store-wal/tests/publication.rs#agent_roots_require_complete_typed_dependency_closures
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-agent --test activity --locked
```

**Notes**

The test-first compile proved that the typed root had no immutable activity, evidence, session, tool or transcript object families.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-agent --locked
```

**Notes**

All seven AgentStore model tests pass, including source ordering, canonical sanitized JSON, typed content edges and self-contained transcript closures.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-wal --test publication agent_roots_require_complete_typed_dependency_closures --locked -- --exact
```

**Notes**

The existing publication staging regression accepts the expanded typed object enum while retaining missing-object and kind-substitution rejection.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo clippy -p jj-agent -p jj-store-wal --all-targets --locked -- -D warnings
```

**Notes**

The expanded object model and its publication consumer are warning-free across all targets.
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
pnpm spec:validate; pnpm spec:first; pnpm check:progress; DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm format:rust:check
```

**Notes**

All 84 requirements, two mapped canonical chapters, 60 task records and Rust formatting pass for the activity-object slice.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Checkpoint and code-association schemas with native dependency retention
- AgentStore root-pinned read/staging facade and typed publication deltas
- Packaged sanitizer plus Codex, Cursor and OpenCode fixtures and normalizers
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; AgentStore schema v1; initial harness targets Codex 0.156.1, Cursor 3.22.7 and OpenCode 1.18.30.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

SanitizedObservationBatchV1, AgentEventBatchV1, AgentSessionSnapshotV1, ToolInvocationSnapshotV1 and TranscriptManifest/Page/ChunkV1 are separate canonical immutable objects. Transcript manifests root complete chunk-page closures and session snapshots do not reference checkpoints.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


