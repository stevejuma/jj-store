---
schema: 1
id: E-PROFILES-001
---

# E-PROFILES-001

<!-- evidence-field:id markdown -->
## Id

E-PROFILES-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M26
- M26.1
- M26.2
- M26.3
- M26.4
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-SEC-008
- JST-API-010
- JST-UI-021
- JST-OBJ-006
- JST-ARCH-001
- JST-ARCH-002
- JST-ARCH-004
- JST-GOV-008
- JST-COLLAB-013
- JST-LOCAL-001
- JST-LOCAL-002
- JST-COMPAT-001
- JST-JJ-002
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-12
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- Cargo.lock
- catalog/components/store-application/StoreProfileSettings.stories.svelte
- catalog/controller/in-memory-store-settings.svelte.ts
- catalog/fixtures/ProfileSettingsFixture.svelte
- crates/jj-collab/src/assets.rs
- crates/jj-collab/src/commands.rs
- crates/jj-collab/src/conversation_submission.rs
- crates/jj-collab/src/emoji_snapshot.rs
- crates/jj-collab/src/lib.rs
- crates/jj-collab/src/objects.rs
- crates/jj-collab/src/reaction_state.rs
- crates/jj-collab/src/threads.rs
- crates/jj-collab/tests/conversation_submission.rs
- crates/jj-collab/tests/domain.rs
- crates/jj-collab/tests/emoji_snapshots.rs
- crates/jj-collab/tests/replies.rs
- crates/jj-collab/tests/target_resolution.rs
- crates/jj-collab/tests/thread_codebase.rs
- crates/jj-store-cli/src/agent_commands/collab/intents.rs
- crates/jj-store-cli/src/identity_commands.rs
- crates/jj-store-cli/src/lib.rs
- crates/jj-store-cli/src/server_commands.rs
- crates/jj-store-cli/src/token_commands.rs
- crates/jj-store-cli/tests/agent_execution/comment_answers/history.rs
- crates/jj-store-cli/tests/agent_execution/comment_answers.rs
- crates/jj-store-cli/tests/agent_execution/workspace_comments.rs
- crates/jj-store-cli/tests/local_collab_cli.rs
- crates/jj-store-cli/tests/local_review_files.rs
- crates/jj-store-cli/tests/native_lifecycle/http_threads_reactions.rs
- crates/jj-store-http/Cargo.toml
- crates/jj-store-http/src/application_events/authorization.rs
- crates/jj-store-http/src/application_events/producers.rs
- crates/jj-store-http/src/assets.rs
- crates/jj-store-http/src/collab/assets.rs
- crates/jj-store-http/src/collab/commands.rs
- crates/jj-store-http/src/contract/tests.rs
- crates/jj-store-http/src/contract.rs
- crates/jj-store-http/src/directory.rs
- crates/jj-store-http/src/dto.rs
- crates/jj-store-http/src/lib.rs
- crates/jj-store-http/src/repositories.rs
- crates/jj-store-http/tests/assets.rs
- crates/jj-store-rpc/tests/collab.rs
- crates/jj-store-server/Cargo.toml
- crates/jj-store-server/src/agent_execution/avatar.rs
- crates/jj-store-server/src/directory/audit.rs
- crates/jj-store-server/src/directory/mod.rs
- crates/jj-store-server/src/directory/mutation.rs
- crates/jj-store-server/src/directory/persistence.rs
- crates/jj-store-server/src/directory/reads.rs
- crates/jj-store-server/src/directory/tests.rs
- crates/jj-store-server/src/directory/validation.rs
- crates/jj-store-server/src/lib.rs
- crates/jj-store-server/src/policy.rs
- crates/jj-store-server/src/presentation_avatar.rs
- crates/jj-store-server/src/process/preparation.rs
- crates/jj-store-server/tests/directory_api.rs
- crates/jj-store-server/tests/repository_conversion.rs
- crates/jj-store-service/src/agent_feedback/tests.rs
- crates/jj-store-service/src/auth.rs
- crates/jj-store-service/src/collab_reads/materialization.rs
- crates/jj-store-service/src/collab_reads.rs
- crates/jj-store-service/src/directory.rs
- crates/jj-store-service/src/freshness.rs
- crates/jj-store-service/src/lib.rs
- crates/jj-store-service/src/local_repository/browser_collab.rs
- crates/jj-store-service/src/registry.rs
- crates/jj-store-service/tests/collab_pagination.rs
- crates/jj-store-service/tests/content_search.rs
- crates/jj-store-service/tests/local_collab/agent_context.rs
- crates/jj-store-service/tests/local_collab/agent_discussion.rs
- crates/jj-store-service/tests/local_collab.rs
- crates/jj-store-service/tests/native_collab.rs
- crates/jj-store-wal/tests/collab_commands.rs
- crates/jj-store-wal/tests/collab_persistence.rs
- docs/cli/server-operator.md
- docs/maintenance/coverage.json
- docs/site/cli/jj-store.md
- docs/site/concepts/collaboration.md
- docs/site/guides/settings.md
- docs/site/reference/api/collaboration.md
- docs/site/reference/api/installation.md
- docs/site/reference/api/types.md
- docs/site/reference/configuration.md
- docs/site/reference/security.md
- docs/site/reference/transport.md
- docs/site/technical/architecture.md
- docs/site/technical/storage.md
- docs/ui-provenance.json
- schemas/openapi.json
- scripts/profile-acceptance.mjs
- spec/src/current/contracts/api/04-requirements.md
- spec/src/current/contracts/api/05-bootstrap-and-browser-admission.md
- spec/src/current/contracts/architecture/05-object-foundation.md
- spec/src/current/contracts/cli/02-cli-and-server-commands-continued-2.md
- spec/src/current/contracts/collaboration/16-persistent-threads-and-reactions-m15.md
- spec/src/current/contracts/compatibility/37-intentional-thread-formats.md
- spec/src/current/contracts/jj-integration/20-m15-companion-envelope-extension.md
- spec/src/current/contracts/local-jj/07-m15-additive-collaboration-publication.md
- spec/src/current/contracts/mutations/06-durable-command-service.md
- spec/src/current/contracts/object-store/01-object-storage.md
- spec/src/current/contracts/object-store/02-requirements.md
- spec/src/current/contracts/publication/05-typed-application-intent.md
- spec/src/current/contracts/security/02-requirements.md
- spec/src/current/contracts/security/11-installation-profiles-and-emoji.md
- spec/src/current/contracts/security/SUMMARY.nav.md
- spec/src/current/contracts/security/index.md
- spec/src/current/contracts/ui/03-requirements.md
- spec/src/current/contracts/ui/09-transient-store-settings.md
- spec/src/current/governance/spec-governance.md
- spec/src/current/progress/M26/SUMMARY.nav.md
- spec/src/current/progress/M26/index.md
- spec/src/current/progress/SUMMARY.nav.md
- spec/src/current/progress/index.md
- spec/src/current/verification/api.md
- spec/src/current/verification/obj.md
- spec/src/current/verification/sec.md
- spec/src/current/verification/ui-03.md
- spec/src/history/2026/10/12/E-PROFILES-001/SUMMARY.nav.md
- spec/src/history/2026/10/12/E-PROFILES-001/details-02.md
- spec/src/history/2026/10/12/E-PROFILES-001/details-03.md
- spec/src/history/2026/10/12/E-PROFILES-001/index.md
- spec/src/history/2026/10/12/SUMMARY.nav.md
- spec/src/history/2026/10/12/index.md
- web/src/lib/api/application-events.ts
- web/src/lib/api/directory-client.ts
- web/src/lib/api/generated.ts
- web/src/lib/collaboration/thread-presentation.ts
- web/src/lib/community/workspace-model.ts
- web/src/lib/controller/contracts.ts
- web/src/lib/controller/directory-actions.svelte.ts
- web/src/lib/controller/directory-media.ts
- web/src/lib/controller/directory-model.ts
- web/src/lib/controller/directory.svelte.ts
- web/src/lib/controller/global-settings-navigation.ts
- web/src/lib/controller/message-reactions.svelte.ts
- web/src/lib/controller/prepare-thread-emoji.ts
- web/src/lib/controller/production-local-configuration.ts
- web/src/lib/controller/production-store-connect.ts
- web/src/lib/controller/production-store-controller.svelte.ts
- web/src/lib/controller/production-store-lifecycle.svelte.ts
- web/src/lib/controller/production-store-state.svelte.ts
- web/src/lib/controller/production-store-view.ts
- web/src/lib/controller/production-threads-integration.ts
- web/src/lib/controller/production-threads.svelte.ts
- web/src/lib/controller/reaction-library.svelte.ts
- web/src/lib/controller/thread-controller-view.ts
- web/src/lib/controller/thread-media.svelte.ts
- web/src/lib/controller/thread-types.ts
- web/src/lib/controller/work-reactions.svelte.ts
- web/src/lib/conversations/thread-adapter.ts
- web/src/lib/conversations/thread-send.ts
- web/src/lib/profiles/context.svelte.ts
- web/src/lib/ui/community-chat-composer/composer-widgets.ts
- web/src/lib/ui/community-chat-composer/model.ts
- web/src/lib/ui/community-member-avatar/CommunityMemberAvatar.svelte
- web/src/lib/ui/community-member-hover-card/CommunityMemberHoverCard.svelte
- web/src/lib/ui/community-message/CommunityMessage.svelte
- web/src/lib/ui/community-message/message-deletion.svelte.ts
- web/src/lib/ui/store-agent-chat-body/ChatTextBlock.svelte
- web/src/lib/ui/store-agent-chat-body/StoreAgentChatBody.svelte
- web/src/lib/ui/store-agent-chat-body/emoji-decorations.ts
- web/src/lib/ui/store-agent-chat-body/model.ts
- web/src/lib/ui/store-agent-chat-body/text-editor.ts
- web/src/lib/ui/store-application/StoreApplication.svelte
- web/src/lib/ui/store-settings/StoreCustomEmojiSettings.svelte
- web/src/lib/ui/store-settings/StoreProfileSettings.svelte
- web/src/lib/ui/store-settings/StoreSettings.svelte
- web/src/lib/ui/store-settings/StoreSettingsNavigation.svelte
- web/src/lib/ui/store-settings/StoreSettingsPage.svelte
- web/src/lib/ui/store-settings/emoji-settings.css
- web/src/lib/ui/store-settings/global-settings-skeleton.ts
- web/src/lib/ui/store-settings/profile-settings.css
- web/src/lib/ui/store-threads/StoreMessageReactions.svelte
- web/tests/managed-agent.test.ts
- web/tests/real/profiles-emoji.spec.ts
- web/tests/vite/directory.test.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-server/src/directory/tests.rs#profiles_are_content_addressed_private_conditional_and_restart_safe
- crates/jj-store-server/src/directory/tests.rs#manifest_lost_acknowledgement_recovers_original_receipt_after_restart
- crates/jj-store-server/tests/directory_api.rs#authenticated_directory_is_private_independent_of_repo_grants_and_revocable
- crates/jj-collab/tests/emoji_snapshots.rs#messages_and_reactions_retain_independent_immutable_library_closures
- crates/jj-store-wal/tests/collab_commands.rs#library_snapshot_message_and_reaction_publish_complete_closures_and_replay
- crates/jj-store-service/tests/local_collab.rs#library_snapshots_survive_companion_restart_and_asset_reads
- crates/jj-store-service/tests/local_collab/agent_discussion.rs#saved_agent_replies_and_reactions_survive_restart_and_keep_exact_targets
- crates/jj-store-http/src/contract/tests.rs#contract_inventory_covers_security_methods_media_and_task_bodies
- crates/jj-store-server/src/process/tests/capture_factory/management.rs#cold_foreground_advertises_authenticated_disabled_capture_without_creating_policy
- web/tests/vite/directory.test.ts#profile drafts survive refresh and require
  explicit review of a changed revision
- web/tests/vite/directory.test.ts#lost save acknowledgement keeps the original
  request and recovers its receipt
- web/tests/real/profiles-emoji.spec.ts#cold local and enrolled profiles and
  emoji survive real object store restart
<!-- /evidence-field -->

<!-- generated-navigation -->
## Pages

- [E-PROFILES-001 details 2](details-02.md)
- [E-PROFILES-001 details 3](details-03.md)
- [E-PROFILES-001 details 4](details-04.md)
