---
schema: 1
id: E-M9-010
---

# E-M9-010

<!-- evidence-field:id markdown -->
## Id

E-M9-010
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
- Cargo.toml
- crates/jj-review/Cargo.toml
- crates/jj-review/src/anchors.rs
- crates/jj-review/src/codec.rs
- crates/jj-review/src/dependencies.rs
- crates/jj-review/src/ids.rs
- crates/jj-review/src/lib.rs
- crates/jj-review/src/selectors.rs
- crates/jj-review/src/validation.rs
- crates/jj-review/tests/anchors.rs
- scripts/check-boundaries.mjs
- spec/src/architecture.md
- spec/src/collaboration.md
- spec/src/spec-governance.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-review/tests/anchors.rs#code_text_anchor_pins_exact_native_objects_and_utf8_bytes
- crates/jj-review/tests/anchors.rs#agent_text_anchor_keeps_owner_attempt_block_and_content_versions_distinct
- crates/jj-review/tests/anchors.rs#anchors_reject_paths_ranges_and_diff_sides_that_lose_exact_identity
- crates/jj-review/tests/anchors.rs#whole_artifact_anchor_requires_a_matching_native_object_kind
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-review --test anchors --locked
```

**Notes**

The test-first compile produced the missing exact-anchor public APIs. The first implementation compile then reported nine concrete Serde result-alias and SHA-256 trait errors before the focused repair.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-review --test anchors --locked
```

**Notes**

All four exact-anchor regressions pass for native and AgentStore identities, explicit diff sides, UTF-8 byte/digest validation, canonical round trips and object-kind matching.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-review --locked; DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo clippy -p jj-review --all-targets --locked -- -D warnings; DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo fmt --all -- --check
```

**Notes**

The complete new crate passes its four tests, clippy is warning-free and all Rust is formatted. Production modules contain at most 184 physical lines and the anchor regression has 231 lines.
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
pnpm check:boundaries
```

**Notes**

All 13 workspace package boundaries pass with jj-review limited to jj-agent and jj-store-model dependencies.
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
pnpm exec prettier --check spec/src/architecture.md spec/src/collaboration.md spec/src/spec-governance.md spec/src/milestones.md
```

**Notes**

The focused formatter reported mechanical wrapping and milestone-table alignment in three changed canonical specification files.
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
pnpm exec prettier --check spec/src/architecture.md spec/src/collaboration.md spec/src/spec-governance.md spec/src/milestones.md; pnpm test:governance; pnpm spec:validate; pnpm spec:first; pnpm check:progress; pnpm spec:build
```

**Notes**

Changed specification files are formatted; all 26 governance tests, 84 requirements, 12 protected files across four canonical chapters and 60 progress tasks pass, and the canonical book builds.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Typed immutable ReviewStore root, index, event, body, comment-revision,
  thread-event and thread-snapshot objects
- Root-pinned ReviewStore reads, typed deltas and stale-safe thread/comment
  commands
- Conservative explicit re-anchoring after original-context reads
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; ReviewStore anchor schema v1; canonical decimal-string UTF-8 byte offsets; SHA-256 anchor and selected-byte digests.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

ArtifactAnchorV1 distinguishes exact code, diff, agent, revision, operation and repository targets. Strong dependencies enumerate full native operation/commit/file objects or immutable AgentStore snapshot/content versions; external gitlink identities fail closed in v1.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


