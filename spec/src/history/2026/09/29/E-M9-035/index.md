---
schema: 1
id: E-M9-035
---

# E-M9-035

<!-- evidence-field:id markdown -->
## Id

E-M9-035
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M9
- M9.4
- M9.5
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-29
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

collaboration-context-presentation-alignment
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- .storybook/vite.config.ts
- catalog/components/store-application/RepositoryReview.stories.svelte
- catalog/components/store-application/StoreCollaboration.stories.svelte
- catalog/controller/in-memory-store-collaboration.svelte.ts
- catalog/controller/in-memory-inline-review.ts
- catalog/controller/in-memory-store-controller.svelte.ts
- catalog/controller/in-memory-store-route.ts
- catalog/fixtures/collaboration.ts
- crates/jj-agent/tests/fixtures/opencode-1.18.30-plugin.json
- crates/jj-store-cli/src/agent_hook.rs
- crates/jj-store-cli/tests/agent_hook.rs
- crates/jj-store-cli/tests/native_lifecycle/http_collaboration.rs
- crates/jj-store-http/src/contract/tests.rs
- crates/jj-store-service/src/collaboration.rs
- crates/jj-store-service/src/collaboration/agent_details.rs
- crates/jj-store-service/src/collaboration/agent_relations.rs
- crates/jj-store-service/src/collaboration/agent_session_timing.rs
- crates/jj-store-service/src/collaboration/models.rs
- crates/jj-store-service/src/collaboration/review_models.rs
- crates/jj-store-service/src/collaboration/review_threads.rs
- crates/jj-store-service/tests/agent_relations.rs
- crates/jj-store-service/tests/agent_sessions.rs
- crates/jj-store-service/tests/review_threads.rs
- crates/jj-store-wal/src/publication.rs
- schemas/openapi.json
- scripts/catalog-browser.test.mjs
- spec/src/api.md
- spec/src/cli.md
- spec/src/collaboration.md
- spec/src/compatibility.md
- spec/src/milestones.md
- spec/src/mutations.md
- spec/src/publication.md
- spec/src/spec-governance.md
- spec/src/ui.md
- web/src/lib/api/generated.ts
- web/src/lib/collaboration/presentation.ts
- web/src/lib/controller/contracts.ts
- web/src/lib/controller/http-collaboration-data-source.ts
- web/src/lib/controller/http-store-data-source.ts
- web/src/lib/controller/native-data-source.ts
- web/src/lib/controller/native-presentation.ts
- web/src/lib/controller/production-collaboration-context.ts
- web/src/lib/controller/production-file-reader.svelte.ts
- web/src/lib/controller/production-route-restorer.ts
- web/src/lib/controller/production-store-actions.ts
- web/src/lib/controller/production-store-collaboration.svelte.ts
- web/src/lib/controller/production-store-lifecycle.svelte.ts
- web/src/lib/controller/production-store-loaders.ts
- web/src/lib/controller/production-store-state.svelte.ts
- web/src/lib/controller/production-store-view.ts
- web/src/lib/controller/store-route-codec.ts
- web/src/lib/projects/repository-log-canvas.ts
- web/src/lib/ui/community-projects/CommunityProjectsRepositoryFiles.svelte
- web/src/lib/ui/community-projects/CommunityProjectsRepositoryLog.svelte
- web/src/lib/ui/store-application/StoreRepositoryApplication.svelte
- web/src/lib/ui/store-collaboration/StoreAgentContext.svelte
- web/src/lib/ui/store-collaboration/StoreAgentSessions.svelte
- web/src/lib/ui/store-collaboration/StoreAgentToolDetail.svelte
- web/src/lib/ui/store-collaboration/StoreAgentTranscript.svelte
- web/src/lib/ui/store-collaboration/StoreIdentityCopy.svelte
- web/src/lib/ui/store-collaboration/StoreReviewAnchorIdentity.svelte
- web/src/lib/ui/store-collaboration/StoreReviewDiscussion.svelte
- web/src/lib/ui/store-collaboration/StoreReviewThreads.svelte
- web/src/lib/ui/store-collaboration/store-collaboration.css
- web/tests/collaboration-presentation.test.ts
- web/tests/store-route-codec.test.ts
- web/tests/vite/production-file-reader.test.ts
- web/tests/vite/production-store-collaboration.test.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/agent_hook.rs#hook_ingestion_survives_restart_orders_sequences_and_never_persists_raw_input
- crates/jj-store-service/tests/agent_relations.rs#relation_pages_retain_root_and_reject_subject_cursor_reuse
- crates/jj-store-service/tests/agent_sessions.rs#session_detail_marks_activity_beyond_the_event_bound_truncated
- crates/jj-store-service/tests/agent_sessions.rs#session_detail_resolves_bounded_messages_and_final_tool_snapshots
- crates/jj-store-service/tests/agent_sessions.rs#session_detail_resolves_current_stable_identity_and_exact_links
- crates/jj-store-service/tests/review_threads.rs#review_threads_are_root_pinned_and_details_resolve_selected_objects
- web/tests/collaboration-presentation.test.ts#agent session durations stay
  compact
- web/tests/collaboration-presentation.test.ts#agent session timing uses
  captured first and last activity
- web/tests/vite/production-file-reader.test.ts#files load the selected commit
  metadata independently of graph history
- web/tests/vite/production-store-collaboration.test.ts#path context continues
  review pages to the exact code anchor
- web/tests/vite/production-store-collaboration.test.ts#revision context follows
  paged exact agent and review associations
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M9-035 details 2](details-02.md)
- [E-M9-035 details 3](details-03.md)
