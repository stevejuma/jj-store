---
schema: 1
id: E-M9-011
---

# E-M9-011

<!-- evidence-field:id markdown -->
## Id

E-M9-011
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

- crates/jj-review/src/catalog.rs
- crates/jj-review/src/codec.rs
- crates/jj-review/src/comments.rs
- crates/jj-review/src/ids.rs
- crates/jj-review/src/lib.rs
- crates/jj-review/src/objects.rs
- crates/jj-review/src/threads.rs
- crates/jj-review/tests/objects.rs
- spec/src/collaboration.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-review/tests/objects.rs#review_root_and_index_pages_are_bounded_typed_and_canonical
- crates/jj-review/tests/objects.rs#thread_snapshot_closure_preserves_anchor_comment_event_and_versions
- crates/jj-review/tests/objects.rs#review_objects_reject_kind_confusion_unsafe_bodies_and_incoherent_history
- crates/jj-review/tests/objects.rs#anchor_objects_forward_native_and_agent_dependencies
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-review --test objects --locked
```

**Notes**

The test-first compile reported the missing root, index, body, comment revision, event page, event, snapshot, object-reference and canonical codec APIs.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-review --test objects --locked
```

**Notes**

All four immutable-object regressions pass for canonical roots/pages, complete thread closure, version separation, unsafe-state rejection and native/AgentStore dependency forwarding.
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

All eight ReviewStore tests pass, the crate is warning-free and all Rust is formatted. Production modules contain at most 247 physical lines and the new regression contains 263 lines.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --check spec/src/collaboration.md spec/src/milestones.md
```

**Notes**

The focused formatter reported mechanical prose wrapping and milestone-table alignment in both changed canonical specification files.
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
pnpm exec prettier --check spec/src/collaboration.md spec/src/milestones.md; pnpm test:governance; pnpm spec:validate; pnpm spec:first; pnpm check:progress; pnpm spec:build
```

**Notes**

Both changed specification files are formatted; all 26 governance tests, 84 requirements, eight protected files across two canonical chapters and 60 progress tasks pass, and the canonical book builds.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Root-pinned ReviewStore reads and typed COW deltas with cross-object
  thread/event agreement
- Stale-safe create, reply, edit, tombstone, hide, resolve and reopen commands
  through shared publication
- Conservative explicit re-anchoring after original-context reads
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; ReviewStore object/root schema v1; 32 selections per catalog leaf; 256 events per event page; one MiB body and root ceilings.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

ReviewRootV1 selects nine independent typed COW catalogs. ReviewObjectV1 registers index, anchor, body, revision, thread-event, event-page and thread-snapshot objects with canonical codecs, safe body state, append-only revision edges and separate thread/content/workflow versions.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


