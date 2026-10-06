---
schema: 1
id: E-M15-004
---

# E-M15-004 details 3

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm exec prettier --write spec/src/architecture.md spec/src/collaboration.md spec/src/local-jj.md spec/src/mutations.md spec/src/spec-governance.md spec/src/milestones.md scripts/check-boundaries.mjs
```
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
pnpm exec prettier --check spec/src/architecture.md spec/src/collaboration.md spec/src/local-jj.md spec/src/mutations.md spec/src/spec-governance.md spec/src/milestones.md scripts/check-boundaries.mjs
```
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
pnpm spec:first
```

**Notes**

16 protected files; 6 canonical chapters.
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
pnpm check:progress
```

**Notes**

111 tasks.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Native typed command adapter and actor-filtered queries, HTTP/RPC/CLI
  commands, atomic optional capture binding and Thread-review work completion
  remain pending.
- Local native code anchors and stronger code attribution remain unsupported.
  Portable manual anchors require retained qualified source capsules; portable
  review anchors retain the M13 gate.
- Reducer 3, shared released UI, canonical-source timeline/search, real-RustFS
  and cold full-process local/native/runner acceptance remain pending.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Shared pure projector and Git-backed local provider surface. Focused local service/direct source consumers plus domain and retained native tests, strict crate Clippy and direct Rust consumer compilation. Root aggregate, UI/browser and real RustFS lanes out of scope.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Implement native typed Thread/Reaction commands and bounded authorized query services, then public HTTP/RPC/CLI contracts and optional capture binding.
<!-- /evidence-field -->

<!-- evidence-field:assets markdown -->
## Assets

Original encoded raster bytes and deterministic first-frame PNG previews remain exact selected Git blobs across definition updates/retirement, GC and reopen. No URL fetch or executable image formats.
<!-- /evidence-field -->

<!-- evidence-field:runner markdown -->
## Runner

No jj-acp runtime linkage in this slice. The optional ThreadBindingV1 contract targets the sibling jj-acp CLI/runner/control boundary after command and capture qualification.
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-service/tests/local_collab.rs#local_thread_reaction_reads_do_not_initialize_metadata_or_actor
- crates/jj-store-service/tests/local_collab.rs#selected_descriptor_cannot_read_unretained_staged_blobs
- crates/jj-store-service/tests/local_collab.rs#local_commands_keep_exact_history_preserve_siblings_and_replay_after_gc
- crates/jj-store-service/tests/local_collab.rs#local_custom_library_pins_assets_retains_old_versions_and_rejects_stale_sets
- crates/jj-store-service/tests/local_collab.rs#local_emoji_library_pages_are_bounded_and_keep_the_captured_store_pin
- crates/jj-store-service/tests/local_collab.rs#local_review_reactions_follow_current_visibility_and_keep_independent_thread_links
- crates/jj-store-service/tests/local_collab.rs#local_captured_sessions_remain_optional_and_tool_targets_keep_native_identity
- crates/jj-store-service/tests/local_collab.rs#local_unknown_outcome_retains_request_and_replays_after_restart
- crates/jj-collab/tests/domain.rs#shared_reaction_projector_retains_conflicts_until_exact_explicit_resolution
<!-- /evidence-field -->

