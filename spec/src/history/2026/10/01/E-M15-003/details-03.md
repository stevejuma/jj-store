---
schema: 1
id: E-M15-003
---

# E-M15-003 details 3

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

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

16 packages.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm exec prettier --write spec/src/collaboration.md spec/src/publication.md spec/src/milestones.md
```
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm exec prettier --write spec/src/architecture.md spec/src/collaboration.md spec/src/publication.md spec/src/milestones.md
```
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
pnpm spec:first
```

**Notes**

8 protected files; 4 canonical chapters.
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
cargo test -p jj-store-wal --test collab_persistence --locked (after inaccessible lookup mapping)
```

**Notes**

13 tests.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
cargo clippy -p jj-collab -p jj-store-wal --all-targets --locked -- -D warnings (final)
```
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
pnpm check:progress (final)
```

**Notes**

111 tasks.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm exec prettier --check spec/src/architecture.md spec/src/collaboration.md spec/src/publication.md spec/src/milestones.md
```
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Local canonical graph provider and typed native/local command publication
  remain pending.
- Service/API/RPC/CLI actor filtering, capture binding, reducer 3, shared UI,
  search and cold real-RustFS/local/runner qualification remain pending.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Rust target-resolution surface and direct native persistence consumers. Focused domain/native target and crate Clippy; no full WAL suite escalation because publication/CAS infrastructure is unchanged. Root aggregate, UI and live RustFS lanes out of scope.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Implement local canonical graph carrier with complete retained blobs, then typed Thread/Reaction command providers.
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-collab/tests/target_resolution.rs#human_targets_require_selected_history_and_current_visibility
- crates/jj-collab/tests/target_resolution.rs#agent_message_checks_selected_batch_and_exact_event_content_witnesses
- crates/jj-collab/tests/target_resolution.rs#tool_output_is_exact_and_old_snapshot_requires_selected_tool_membership
- crates/jj-collab/tests/target_resolution.rs#review_current_hidden_tombstone_and_conflict_suppress_prior_reactions
- crates/jj-collab/tests/target_resolution.rs#resolved_review_conflict_retains_exact_alternatives_but_rejects_orphans
- crates/jj-collab/tests/target_resolution.rs#selected_review_replies_use_the_same_exact_revision_policy
- crates/jj-collab/tests/target_resolution.rs#checkpoint_versions_require_exact_native_or_portable_supersession
- crates/jj-collab/tests/target_resolution.rs#forged_hashes_and_cross_entity_selections_fail_closed
- crates/jj-collab/tests/target_resolution.rs#revision_traversal_is_bounded_and_absent_targets_are_unavailable
- crates/jj-store-wal/tests/collab_persistence.rs#pinned_target_reader_hides_staged_objects_and_current_tombstoned_history
- crates/jj-store-wal/tests/collab_persistence.rs#target_reader_closure_excludes_unselected_collab_and_agent_bytes
- crates/jj-store-wal/tests/collab_persistence.rs#target_resolver_uses_repository_bound_agent_ids_from_pinned_root
- crates/jj-store-wal/tests/collab_persistence.rs#target_resolver_uses_repository_bound_review_ids_and_selected_moderation
<!-- /evidence-field -->

