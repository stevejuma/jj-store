---
schema: 1
id: E-EXEC-016
---

# E-EXEC-016

<!-- evidence-field:id markdown -->
## Id

E-EXEC-016
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-07
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M23
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-appearance/AppearanceComposer.stories.svelte
- catalog/fixtures/AppearanceComposerFixture.svelte
- crates/jj-store-http/src/agent_execution/tests.rs
- crates/jj-store-http/src/agent_execution.rs
- crates/jj-store-server/src/agent_execution/commands.rs
- crates/jj-store-server/src/agent_execution/delivery/tests.rs
- crates/jj-store-server/src/agent_execution/delivery.rs
- crates/jj-store-server/src/agent_execution/feedback.rs
- crates/jj-store-server/src/agent_execution/journal.rs
- crates/jj-store-server/src/agent_execution/launch.rs
- crates/jj-store-server/src/agent_execution/mod.rs
- crates/jj-store-service/src/agent_execution/context.rs
- crates/jj-store-service/src/agent_execution.rs
- crates/jj-store-service/src/collab_reads/agent_feedback.rs
- crates/jj-store-service/src/local_repository/agent_feedback.rs
- crates/jj-store-service/tests/local_collab/agent_context.rs
- crates/jj-store-service/tests/local_collab/agent_discussion.rs
- crates/jj-store-service/tests/local_collab.rs
- docs/maintenance/coverage.json
- docs/site/guides/agent-execution.md
- scripts/agent-execution-acceptance.py
- scripts/agent-execution-discussion-api.mjs
- scripts/agent-execution-discussion.mjs
- spec/src/current/contracts/api/41-interactive-agent-execution.md
- spec/src/current/contracts/collaboration/36-interactive-agent-execution.md
- spec/src/current/contracts/ui/34-interactive-agent-sessions.md
- spec/src/current/progress/M23/history/M23/index.md
- spec/src/current/progress/M23/index.md
- spec/src/history/2026/10/07/E-EXEC-016/SUMMARY.nav.md
- spec/src/history/2026/10/07/E-EXEC-016/details-02.md
- spec/src/history/2026/10/07/E-EXEC-016/details-03.md
- spec/src/history/2026/10/07/E-EXEC-016/details-04.md
- spec/src/history/2026/10/07/E-EXEC-016/index.md
- spec/src/history/2026/10/07/E-EXEC-017/SUMMARY.nav.md
- spec/src/history/2026/10/07/E-EXEC-017/details-02.md
- spec/src/history/2026/10/07/E-EXEC-017/details-03.md
- spec/src/history/2026/10/07/E-EXEC-017/details-04.md
- spec/src/history/2026/10/07/E-EXEC-017/index.md
- spec/src/history/2026/10/07/SUMMARY.nav.md
- spec/src/history/2026/10/07/index.md
- web/src/lib/controller/agent-execution-errors.ts
- web/src/lib/controller/agent-execution.svelte.ts
- web/src/lib/ui/community-chat-composer/CommunityChatComposer.svelte
- web/tests/vite/agent-execution-controller.test.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-service/tests/local_collab/agent_context.rs#incremental_context_tracks_primary_thread_edits_nested_replies_and_snapshot_races
- crates/jj-store-service/tests/local_collab/agent_context.rs#overlapping_discussion_paths_deduplicate_and_limits_apply_only_to_new_revisions
- crates/jj-store-server/src/agent_execution/delivery/tests.rs#queued_replies_reserve_release_and_deliver_only_on_exact_started
- crates/jj-store-server/src/agent_execution/delivery/tests.rs#restart_retains_batch_intent_reservations_and_legacy_defaults
- crates/jj-store-server/src/agent_execution/delivery/tests.rs#continuation_tracks_only_structured_feedback_in_whole_retained_records
- crates/jj-store-server/src/agent_execution/delivery/tests.rs#fresh_adapter_keeps_unknown_intent_but_only_reserves_its_own_context_generation
- crates/jj-store-http/src/agent_execution/tests.rs#feedback_preparation_refusals_have_specific_transport_codes
- crates/jj-store-server/src/agent_execution/delivery/tests.rs#cancelled_initial_prompt_does_not_lose_continuation_context_to_a_queued_turn
<!-- /evidence-field -->


<!-- generated-navigation -->
## Pages

- [E-EXEC-016 details](details-02.md)
- [E-EXEC-016 details](details-03.md)
- [E-EXEC-016 details](details-04.md)
