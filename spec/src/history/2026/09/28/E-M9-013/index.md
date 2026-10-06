---
schema: 1
id: E-M9-013
---

# E-M9-013

<!-- evidence-field:id markdown -->
## Id

E-M9-013
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

- crates/jj-review/src/codec.rs
- crates/jj-review/src/commands.rs
- crates/jj-review/src/lib.rs
- crates/jj-review/src/threads.rs
- crates/jj-store-wal/src/publication.rs
- crates/jj-store-wal/src/publication/review_audit.rs
- crates/jj-store-wal/src/publication/review_audit_events.rs
- crates/jj-store-wal/src/publication/review_command_apply.rs
- crates/jj-store-wal/src/publication/review_command_history.rs
- crates/jj-store-wal/src/publication/review_command_update.rs
- docs/cli/review.md
- crates/jj-store-wal/src/publication/review_commands.rs
- crates/jj-store-wal/tests/review_catalog.rs
- crates/jj-store-wal/tests/review_commands.rs
- spec/src/collaboration.md
- spec/src/publication.md
- spec/src/verification.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-wal/tests/review_commands.rs#create_thread_is_atomic_replayable_and_preserves_sibling_roots
- crates/jj-store-wal/tests/review_commands.rs#replies_with_the_same_workflow_version_both_survive
- crates/jj-store-wal/tests/review_commands.rs#edit_requires_the_exact_selected_comment_revision
- crates/jj-store-wal/tests/review_commands.rs#tombstone_and_moderated_hide_keep_append_only_history
- crates/jj-store-wal/tests/review_commands.rs#reply_losing_a_race_with_resolve_is_rejected
- crates/jj-store-wal/tests/review_commands.rs#resolve_rejects_an_unseen_reply
- crates/jj-store-wal/tests/review_commands.rs#reopen_must_commit_before_another_reply
- crates/jj-store-wal/tests/review_catalog.rs#review_root_rejects_snapshot_state_not_reconstructed_by_events
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-wal --test review_commands --locked
```

**Notes**

The test-first compile reported the missing typed review commands, receipt-backed publication entry point, command result codec and thread lookup APIs.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-wal --test review_commands --locked
```

**Notes**

All seven focused command regressions pass for atomic create and late receipt replay, independent replies, exact edit conflicts, tombstone/hide history, resolve/reply races, reopen sequencing and sibling-root preservation.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-wal --test review_catalog --locked; DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-review --test objects --locked
```

**Notes**

All three root-pinned catalog regressions and four immutable ReviewStore object regressions pass after event-stream state reconstruction was added.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo clippy -p jj-review -p jj-store-wal --all-targets --locked -- -D warnings
```

**Notes**

The first focused Clippy run identified the new append-reply helper as intentionally exceeding the default argument-count heuristic; its reviewed command-boundary signature now carries an explicit narrow allowance.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:governance; pnpm spec:validate
```

**Notes**

All 26 governance tests passed, then the canonical validator rejected JST-PUB-009 until its concrete review-command verification row was added.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-review --locked; DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-wal --locked; DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo clippy -p jj-review -p jj-store-wal --all-targets --locked -- -D warnings; DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo fmt --all -- --check
```

**Notes**

All 10 ReviewStore model tests and 46 WAL inventory, publication, recovery, catalog and command tests pass; both crates are warning-free and formatted. Production modules contain at most 294 physical lines and the command regression contains 435 lines.
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
pnpm check:boundaries
```

**Notes**

All 13 workspace package boundaries pass; the command and publication implementation remains within the existing jj-review to model/agent and jj-store-wal to jj-review ownership edges.
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
pnpm test:governance; pnpm spec:validate; pnpm spec:first; pnpm check:progress; pnpm spec:build; pnpm exec prettier --check spec/src/collaboration.md spec/src/publication.md spec/src/verification.md spec/src/milestones.md
```

**Notes**

All 26 governance tests and 85 canonical requirements pass; spec-first maps twelve protected implementation files to three canonical chapters, all 60 progress tasks are consistent, the canonical book builds and changed prose is formatted.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo fmt --all -- --check; DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-wal --test review_commands --locked; DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo clippy -p jj-store-wal --lib --locked -- -D warnings
```

**Notes**

The final resolved-thread event-audit guard is formatted, all seven command regressions still pass and the WAL library remains warning-free.
<!-- /run -->

<!-- run -->
### Run 10

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

The new malformed-snapshot regression first compiled against an intentionally private raw-object reader; the fixture now retains its staged snapshot value instead of widening the production API.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-wal --test review_catalog --locked
```

**Notes**

All four root-pinned ReviewStore catalog tests pass, including rejection when a model-valid snapshot claims resolved state that its selected event stream reconstructs as open.
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
pnpm exec prettier --write spec/src/milestones.md; pnpm spec:validate; pnpm spec:first; pnpm check:progress; pnpm exec prettier --check spec/src/collaboration.md spec/src/publication.md spec/src/verification.md spec/src/milestones.md
```

**Notes**

All 85 canonical requirements validate; spec-first maps thirteen protected implementation files to three canonical chapters, all 60 progress tasks are consistent and changed prose remains formatted.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-wal --test review_commands --test review_catalog --locked; DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo clippy -p jj-review -p jj-store-wal --all-targets --locked -- -D warnings; DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo fmt --all -- --check
```

**Notes**

All eleven focused command/catalog regressions pass after replacing logically infallible production key derivation assertions with typed fail-closed errors; both crates remain warning-free and formatted.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Typed relation/status catalogs and conservative explicit re-anchoring
- LocalTrusted CLI/HTTP contracts and shared Community/Design Core presentation
- Cold three-root recovery and native copy qualification
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; ReviewCommandV1 and ReviewCommandResultV1 schema v1; ReviewStore object/root schema v1.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Review command intent binds repository, principal and stable request/publication identity plus the typed semantic command. Stable thread/comment/action IDs are deterministic per request; committed receipts return exact event, revision and snapshot references. Replies bind workflow version, comment revisions bind exact selected revision IDs, and workflow transitions bind exact thread snapshot IDs.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


