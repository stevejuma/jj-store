---
schema: 1
id: E-M15-016
---

# E-M15-016

<!-- evidence-field:id markdown -->
## Id

E-M15-016
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M15
- M15.1
- M15.2
- M15.3
- M15.5
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-01
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

community-thread-channel-ui
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change xrkpynxm
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

JJ 0.45.1 domain baseline; installed local file snapshots @lapismd/lapis-community 0.1.0 and @lapismd/design-core 0.1.2 through the existing development hook and unchanged parent lockfile; Svelte 5.57.1, Vitest 4.1.11, Storybook 10.5.7. Development Cargo runs use CARGO_INCREMENTAL=0. Released-package qualification is not established.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Optional reply_to is omitted from legacy human revisions and commands when absent. Parent revisions become exact immutable closure dependencies and edits preserve parent identity. Native manifest 5/local companion 2 remain unchanged. HTTP contracts add authorized human timeline bodies, custom emoji definition refs and captured Agent message reaction witnesses. Image reads remain credential-bound and become revocable memory blob URLs.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-threads/StoreCapturedReactions.stories.svelte
- catalog/components/store-threads/StoreThreads.stories.svelte
- catalog/fixtures/StoreCapturedReactionsFixture.svelte
- catalog/fixtures/StoreThreadsFixture.svelte
- catalog/fixtures/thread-transport.ts
- crates/jj-collab/src/commands.rs
- crates/jj-collab/src/graph.rs
- crates/jj-collab/src/objects.rs
- crates/jj-collab/src/threads.rs
- crates/jj-collab/tests/domain.rs
- crates/jj-collab/tests/replies.rs
- crates/jj-collab/tests/target_resolution.rs
- crates/jj-store-cli/tests/local_collab_cli.rs
- crates/jj-store-cli/tests/native_lifecycle/http_threads_reactions.rs
- crates/jj-store-rpc/tests/collab.rs
- crates/jj-store-service/src/collab_pages.rs
- crates/jj-store-service/src/collab_reads.rs
- crates/jj-store-service/src/collaboration/agent_details.rs
- crates/jj-store-service/src/collaboration/models.rs
- crates/jj-store-service/tests/agent_sessions.rs
- crates/jj-store-service/tests/local_collab.rs
- crates/jj-store-service/tests/native_collab.rs
- crates/jj-store-wal/tests/collab_commands.rs
- crates/jj-store-wal/tests/collab_persistence.rs
- schemas/openapi.json
- spec/src/api.md
- spec/src/cli.md
- spec/src/collaboration.md
- spec/src/compatibility.md
- spec/src/jj-integration.md
- spec/src/local-jj.md
- spec/src/milestones.md
- spec/src/mutations.md
- spec/src/publication.md
- spec/src/ui.md
- web/src/lib/api/client.ts
- web/src/lib/api/generated.ts
- web/src/lib/api/reaction-asset.ts
- web/src/lib/collaboration/thread-presentation.ts
- web/src/lib/controller/collab-actions.svelte.ts
- web/src/lib/controller/contracts.ts
- web/src/lib/controller/http-store-data-source.ts
- web/src/lib/controller/http-threads-data-source.ts
- web/src/lib/controller/message-reactions.svelte.ts
- web/src/lib/controller/native-data-source.ts
- web/src/lib/controller/production-store-controller.svelte.ts
- web/src/lib/controller/production-store-refresh.ts
- web/src/lib/controller/production-threads-integration.ts
- web/src/lib/controller/production-threads.svelte.ts
- web/src/lib/controller/store-route-codec.ts
- web/src/lib/controller/store-route-history.svelte.ts
- web/src/lib/controller/thread-types.ts
- web/src/lib/ui/community-navigation/icons.ts
- web/src/lib/ui/community-navigation/model.ts
- web/src/lib/ui/store-application/StoreApplication.svelte
- web/src/lib/ui/store-application/StoreRepositoryApplication.svelte
- web/src/lib/ui/store-application/store-navigation.ts
- web/src/lib/ui/store-collaboration/StoreAgentTranscript.svelte
- web/src/lib/ui/store-collaboration/StoreReviewConversation.svelte
- web/src/lib/ui/store-threads/StoreMessageReactions.svelte
- web/src/lib/ui/store-threads/StoreThreadCreate.svelte
- web/src/lib/ui/store-threads/StoreThreadReplies.svelte
- web/src/lib/ui/store-threads/StoreThreads.svelte
- web/src/lib/ui/store-threads/store-threads.css
- web/src/lib/ui/store-threads/thread-navigation.ts
- web/tests/client.test.ts
- web/tests/vite/production-threads.test.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-collab/tests/replies.rs#legacy_messages_keep_canonical_bytes_without_reply_field
- crates/jj-collab/tests/replies.rs#replies_retain_exact_parent_and_reject_cross_thread_or_self_links
- crates/jj-store-service/tests/local_collab.rs#local_reply_parent_and_receipt_survive_gc_and_cold_reopen
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M15-016 details 2](details-02.md)
- [E-M15-016 details 3](details-03.md)
- [E-M15-016 details 4](details-04.md)
- [E-M15-016 details 5](details-05.md)
