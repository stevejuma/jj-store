---
schema: 1
id: E-M9-014
---

# E-M9-014

<!-- evidence-field:id markdown -->
## Id

E-M9-014
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

- crates/jj-review/src/anchors.rs
- crates/jj-review/src/delta.rs
- crates/jj-review/src/ids.rs
- crates/jj-review/src/lib.rs
- crates/jj-review/src/objects.rs
- crates/jj-review/src/relations.rs
- crates/jj-review/src/resolver.rs
- crates/jj-review/src/resolver_agent.rs
- crates/jj-review/src/resolver_code.rs
- crates/jj-review/src/resolver_text.rs
- crates/jj-review/tests/objects.rs
- crates/jj-review/tests/resolver.rs
- crates/jj-store-wal/src/publication.rs
- crates/jj-store-wal/src/publication/review_audit.rs
- crates/jj-store-wal/src/publication/review_audit_relations.rs
- crates/jj-store-wal/src/publication/review_catalog.rs
- crates/jj-store-wal/src/publication/review_command_apply.rs
- crates/jj-store-wal/src/publication/review_command_commit.rs
- crates/jj-store-wal/src/publication/review_command_history.rs
- crates/jj-store-wal/src/publication/review_command_relations.rs
- crates/jj-store-wal/src/publication/review_command_stage.rs
- crates/jj-store-wal/src/publication/review_command_update.rs
- crates/jj-store-wal/src/publication/review_resolver.rs
- crates/jj-store-wal/tests/review_catalog.rs
- crates/jj-store-wal/tests/review_commands.rs
- crates/jj-store-wal/tests/review_relations.rs
- crates/jj-store-wal/tests/support/review.rs
- spec/src/collaboration.md
- spec/src/publication.md
- spec/src/verification.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-review/tests/resolver.rs#exact_identity_and_explicit_insertion_map_without_retargeting_the_anchor
- crates/jj-review/tests/resolver.rs#changed_deleted_duplicate_and_missing_lineage_are_categorical
- crates/jj-review/tests/resolver.rs#budgets_divergence_and_unavailable_content_fail_closed
- crates/jj-review/tests/resolver.rs#agent_output_maps_only_through_retained_unchanged_prefix_lineage
- crates/jj-review/tests/objects.rs#relation_and_status_pages_are_typed_bounded_and_canonical
- crates/jj-store-wal/tests/review_relations.rs#typed_relation_and_status_catalogs_validate_selected_threads
- crates/jj-store-wal/tests/review_catalog.rs#root_pinned_resolver_does_not_publish_repository_state
- crates/jj-store-wal/tests/review_commands.rs#create_thread_is_atomic_replayable_and_preserves_sibling_roots
- crates/jj-store-wal/tests/review_commands.rs#reply_losing_a_race_with_resolve_is_rejected
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-review --test resolver --locked
```

**Notes**

The test-first compile reported the missing typed resolver request, target, lineage, budget, candidate and categorical placement contracts.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-review --test objects relation_and_status_pages_are_typed_bounded_and_canonical --locked -- --exact
```

**Notes**

The model regression first failed because typed relation/status pages, object kinds, keys and dependencies did not exist.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 14
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-wal --test review_catalog typed_relation_and_status_catalogs_validate_selected_threads --locked -- --exact
```

**Notes**

The root-pinned regression first failed because relation/status catalog variants, staging, lookup and semantic audit were absent.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-wal --test review_commands create_thread_is_atomic_replayable_and_preserves_sibling_roots --locked -- --exact
```

**Notes**

The first runtime regression found that normal review commands did not yet populate direct anchor, artifact and status indexes.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-review --test resolver --test objects --locked
```

**Notes**

All nine focused ReviewStore model and resolver regressions pass.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-wal --test review_commands --test review_catalog --test review_relations --locked
```

**Notes**

All thirteen affected WAL command, direct-index, root-audit and read-only resolver regressions pass.
<!-- /run -->

<!-- run -->
### Run 7

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

The focused Clippy run required deriving the resolver wrapper's equivalent Default implementation.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
```

**Notes**

The ledger correctly rejected a passing evidence record with remaining M9 gaps and a Done M9.3 task whose M9.2 dependency remains in progress.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo clippy -p jj-review -p jj-store-wal --all-targets --locked -- -D warnings
```

**Notes**

Both affected packages and every directly affected test target are warning-free.
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
pnpm check:progress; pnpm spec:build; pnpm spec:validate; pnpm spec:first
```

**Notes**

All 60 progress tasks are consistent, the canonical book builds, all 85 requirements validate and 27 protected files map to three canonical chapters.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- LocalTrusted CLI/HTTP contracts and shared Community/Design Core presentation
- Cold three-root recovery and native copy qualification
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; ReviewStore object/root schema v1; AnchorResolverV1 algorithm v1; 256 entries per relation/status page; eight MiB default resolver byte budget; eight default bounded candidates.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

ArtifactId hashes one validated exact target independently from anchor creation context. Relation/status pages select current thread snapshots under typed subject keys. Placement results preserve the original anchor and return exact, mapped_unchanged, candidate_modified, deleted, ambiguous, unresolved or unavailable with categorical evidence.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


