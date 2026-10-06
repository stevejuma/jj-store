---
schema: 1
id: E-UI-064
---

# E-UI-064

<!-- evidence-field:id markdown -->
## Id

E-UI-064
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M11.4
- M15.3
- M15.5
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-02
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

cross-layer task reaction and menu acceptance
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

Enclosing Jujutsu change lwxwqzum; Design Core source change kupxxqyt (b3eb19e7)
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-WORK-008
- JST-COLLAB-010
- JST-COLLAB-013
- JST-COLLAB-014
- JST-COLLAB-016
- JST-API-006
- JST-UI-015
- JST-ARCH-002
- JST-GOV-007
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Functional native task reaction, UI and direct provider acceptance passed.
  Inherited broad Rust lint failures, released-registry Design Core
  qualification, local browser collaboration transport and visual baselines
  remain separate pending gates; canonical visual comparison blocked by
  published tool cache EACCES. Unsharded Storybook browser lifetime failures are
  recorded; all 53 families pass in the same suite split into three sequential
  shards.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Metadata-home task comment reactions, leading Copy submenu alignment and visible task Edit/Add subtask surfaces. Cross-layer escalation: additive jj-collab target, native/local pinned provider, read materialization, generated HTTP/RPC contract and browser consumers. Direct service, resolver, WAL, RPC and UI regressions; Design Core guide separately requires full source checks. No root aggregate app pnpm check/test/build/spec:check. Native storage formats and installed CLI unchanged. Local provider support does not qualify the still-gated local browser collaboration transport. Source validation uses the existing private dependency hook; tracked manifests/lock remain registry-only. Publication, main landing and visual baseline updates are out of scope. Shared hover-helper escalation: focus loss after removing a focused reaction chip must preserve a toolbar under the pointer; conversation, thread, review and agent consumers require the full Storybook lane. Boundary-policy escalation: admit only the owned jj-collab to jj-work model edge, with positive and reverse/publication/upstream negative tests; full owning governance test file and actual 16-package metadata check selected, not unrelated Rust suites.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust1.98.1; jj-lib0.45.1; Git2.55.0; Node26.9.0; pnpm10.32.1; BitsUI2.19.4; DesignCore source0.1.2, tracked registry range^0.2.0; Svelte5.57.1; Vite8.3.2; Vitest4.1.11; Playwright1.63.0. Source Design Core Storybook10.5.2 and Playwright1.61.1.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Additive work_comment target: task_id, comment_id, exact nonzero u64 version, scoped by repository request. Existing target canonical bytes preserved. Optional Work root in CollabReadSnapshotV1; absent roots omitted. Existing native manifest/WAL, companionv2, durable receipts, actor/slot preconditions and work versions unchanged. No synthetic Threads or new credentials. Original pending WorkComment target remains recoverable in the same comment metadata after version changes and tombstones; counts/pickers use only the current visible target. Background WorkStore read refresh never disables or silently drops a reaction selection; exact target validation remains server-owned.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Review the verified native task-comment slice in the current development workspace. Qualify a compatible Design Core registry release and visual comparison/baselines separately; local-browser collaboration transport and inherited broad lint remain outside this slice.
<!-- /evidence-field -->

<!-- evidence-field:artifacts markdown -->
## Artifacts

/tmp/jj-task-*.log; /tmp/jj-task-actions.jpg. Vite7460 PID88268; backend7462 PID91193, persistent main fixture preserved without reset. Cold test uses a separate disposable root/port7484.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- Cargo.lock
- catalog/components/store-work/StoreWork.stories.svelte
- catalog/fixtures/StoreWorkFixture.svelte
- crates/jj-collab/Cargo.toml
- crates/jj-collab/src/resolver.rs
- crates/jj-collab/src/targets.rs
- crates/jj-collab/tests/target_resolution.rs
- crates/jj-store-rpc/src/collab/codec_tests.rs
- crates/jj-store-service/src/collab_reads/materialization.rs
- crates/jj-store-service/src/collab_reads.rs
- crates/jj-store-service/src/local_repository/collab_commands.rs
- crates/jj-store-service/src/local_repository/collab_store.rs
- crates/jj-store-service/tests/local_collab.rs
- crates/jj-store-service/tests/native_collab.rs
- crates/jj-store-wal/src/publication/collab_commands.rs
- crates/jj-store-wal/src/publication/collab_targets.rs
- crates/jj-store-wal/src/publication/work.rs
- docs/ui-provenance.json
- schemas/openapi.json
- spec/src/api.md
- spec/src/architecture.md
- spec/src/collaboration.md
- spec/src/compatibility.md
- spec/src/jj-integration.md
- spec/src/local-jj.md
- spec/src/mutations.md
- spec/src/projects.md
- spec/src/publication.md
- spec/src/spec-governance.md
- spec/src/ui.md
- spec/src/verification.md
- web/src/lib/api/generated.ts
- web/src/lib/controller/http-work-data-source.ts
- web/src/lib/controller/production-work-integration.ts
- web/src/lib/controller/production-work.svelte.ts
- web/src/lib/controller/projects-work-registry.svelte.ts
- web/src/lib/controller/thread-types.ts
- web/src/lib/controller/work-reactions.svelte.ts
- web/src/lib/controller/work-types.ts
- web/src/lib/ui/store-collaboration/StoreMessageActions.svelte
- web/src/lib/ui/store-threads/StoreMessageReactions.svelte
- web/src/lib/ui/store-work/StoreWorkComment.svelte
- web/src/lib/ui/store-work/StoreWorkComments.svelte
- web/src/lib/ui/store-work/StoreWorkTaskDetail.svelte
- web/tests/real/task-reactions.spec.ts
- web/tests/vite/work-reactions.test.ts
- spec/src/milestones.md
- web/src/lib/ui/community-message-actions/message-action-hover.svelte.ts
- scripts/check-boundaries.mjs
- scripts/governance.test.mjs
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-collab/tests/target_resolution.rs#work_comment_requires_exact_visible_identity_task_and_version
- crates/jj-store-service/tests/native_collab.rs#native_work_comment_reactions_pin_versions_authorize_and_survive_cold_sessions
- crates/jj-store-service/tests/local_collab.rs#local_work_comment_reactions_share_companion_pin_and_survive_gc_and_reopen
- web/tests/vite/work-reactions.test.ts#Work comment reactions use each home's
  source, exact versions and observed slots
- web/tests/vite/work-reactions.test.ts#automatic invalidation preserves the
  original unknown reaction and confirmed retry
- web/tests/vite/work-reactions.test.ts#late reaction reads cannot repopulate a
  cleared home session
- web/tests/real/task-reactions.spec.ts#task reaction picker persists exact
  comments and refreshes a second client
- scripts/governance.test.mjs#collaboration_targets_use_owned_work_types_without_reverse_or_runtime_edges
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-UI-064 details 2](details-02.md)
- [E-UI-064 details 3](details-03.md)
- [E-UI-064 details 4](details-04.md)
- [E-UI-064 details 5](details-05.md)
- [E-UI-064 details 6](details-06.md)
