---
schema: 1
id: E-M15-021
---

# E-M15-021

<!-- evidence-field:date markdown -->
## Date

2026-10-01
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change msszrzpn
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

JJ 0.45.1; Rust 1.98.1; Svelte 5.57.1; Vitest 4.1.11; Storybook 10.5.7; Vite 8.3.1; CARGO_INCREMENTAL=0 for Rust commands. Community source revisions 741b8285e2081f1c9af58afe5a9b72f538fad781 and d859f33a484a9238bb11f974f4ead8e9be729ec6 under Apache-2.0. Mira 0.0.2 from registry; exact registry bits-ui 2.18.1. Inherited Design Core local hook resolves 0.1.2 despite declared ^0.2.0; this does not qualify registry-only foundations.
<!-- /evidence-field -->

<!-- evidence-field:id markdown -->
## Id

E-M15-021
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M15
- M15.5
- M15.6
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

community-thread-forum-and-workspace-search-alignment
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Owned Community channel toolbar with native heading, forum directory, shell-owned far-right settings/contributors/replies, native composer categories repositories/projects/tasks/commits/logs/threads, safe rendered native entity links and titled removable chips. Threads generate thread= URLs and read legacy channel=; Review thread= remains separate. Global search fans out over authorized discovered repositories/homes with two readers, four recent rows, server scopes, independent content/code cursors, explicit source errors and capability-gated create/browse actions.
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Thread details retain repository tabs and an inset custom panel; directory lacks forum metadata, composer slash categories are inherited Community placeholders, native relative Markdown is treated as a Mira file path, and global recent/scoped search/actions are absent.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- .storybook/vite.config.ts
- catalog/components/community-projects/CommunityProjectsRepositoryDirectory.stories.svelte
- catalog/components/store-application/StoreRepositorySearch.stories.svelte
- catalog/components/store-search/StoreRecentSearch.stories.svelte
- catalog/components/store-search/StoreSearchCommand.stories.svelte
- catalog/components/store-threads/StoreThreadAlignment.stories.svelte
- catalog/components/store-threads/StoreThreads.stories.svelte
- catalog/fixtures/StoreThreadsFixture.svelte
- catalog/fixtures/thread-transport.ts
- docs/catalog.md
- scripts/catalog-browser.test.mjs
- spec/src/api.md
- spec/src/architecture.md
- spec/src/milestones.md
- spec/src/ui.md
- spec/src/verification.md
- web/src/lib/api/generated.ts
- web/src/lib/collaboration/presentation.ts
- web/src/lib/collaboration/thread-presentation.ts
- web/src/lib/community/community-link-normalize.ts
- web/src/lib/community/community-links.ts
- web/src/lib/community/composer-media.ts
- web/src/lib/community/composer-model.ts
- web/src/lib/community/content-mentions.ts
- web/src/lib/community/contracts.ts
- web/src/lib/community/conversations.ts
- web/src/lib/community/send-contracts.ts
- web/src/lib/community/workspace-model.ts
- web/src/lib/controller/contracts.ts
- web/src/lib/controller/global-search-presentation.ts
- web/src/lib/controller/global-search-types.ts
- web/src/lib/controller/global-search.svelte.ts
- web/src/lib/controller/production-entity-search.ts
- web/src/lib/controller/production-repository-connect.ts
- web/src/lib/controller/production-route-restore.ts
- web/src/lib/controller/production-store-controller.svelte.ts
- web/src/lib/controller/production-threads.svelte.ts
- web/src/lib/controller/production-work.svelte.ts
- web/src/lib/controller/projects-route-codec.ts
- web/src/lib/controller/store-route-codec.ts
- web/src/lib/controller/work-comment-focus.ts
- web/src/lib/controller/work-types.ts
- web/src/lib/ui/community-channel-settings/CommunityChannelSettings.css
- web/src/lib/ui/community-channel-settings/CommunityChannelSettings.svelte
- web/src/lib/ui/community-chat-composer/CommunityChatComposer.svelte
- web/src/lib/ui/community-chat-composer/CommunityComposerAutocompleteLayer.svelte
- web/src/lib/ui/community-chat-composer/CommunityComposerChannelChip.svelte
- web/src/lib/ui/community-chat-composer/CommunityComposerChipDismiss.svelte
- web/src/lib/ui/community-chat-composer/CommunityComposerEditContext.svelte
- web/src/lib/ui/community-chat-composer/CommunityComposerEntityChip.svelte
- web/src/lib/ui/community-chat-composer/CommunityComposerMemberChip.svelte
- web/src/lib/ui/community-chat-composer/CommunityComposerMemberReference.svelte
- web/src/lib/ui/community-chat-composer/CommunityComposerRemovableChip.svelte
- web/src/lib/ui/community-chat-composer/CommunityComposerToolbar.svelte
- web/src/lib/ui/community-chat-composer/autocomplete-rank.ts
- web/src/lib/ui/community-chat-composer/autocomplete.ts
- web/src/lib/ui/community-chat-composer/composer-0.css
- web/src/lib/ui/community-chat-composer/composer-1.css
- web/src/lib/ui/community-chat-composer/composer-autocomplete-apply.ts
- web/src/lib/ui/community-chat-composer/composer-autocomplete.svelte.ts
- web/src/lib/ui/community-chat-composer/composer-chip-remove.ts
- web/src/lib/ui/community-chat-composer/composer-chip-widgets.ts
- web/src/lib/ui/community-chat-composer/composer-decorations.ts
- web/src/lib/ui/community-chat-composer/composer-entity-links.ts
- web/src/lib/ui/community-chat-composer/composer-extension.ts
- web/src/lib/ui/community-chat-composer/composer-mention-insert.ts
- web/src/lib/ui/community-chat-composer/composer-mention-token.ts
- web/src/lib/ui/community-chat-composer/composer-reference-decorations.ts
- web/src/lib/ui/community-chat-composer/composer-room-tokens.ts
- web/src/lib/ui/community-chat-composer/composer-slash-catalog.ts
- web/src/lib/ui/community-chat-composer/composer-slash-context.ts
- web/src/lib/ui/community-chat-composer/composer-slash-insert.ts
- web/src/lib/ui/community-chat-composer/composer-slash-session.ts
- web/src/lib/ui/community-chat-composer/composer-slash.ts
- web/src/lib/ui/community-chat-composer/composer-token-references.ts
- web/src/lib/ui/community-chat-composer/composer-widgets.ts
- web/src/lib/ui/community-chat-composer/model.ts
- web/src/lib/ui/community-chat-composer/props.ts
- web/src/lib/ui/community-composer-mention-picker/CommunityComposerMentionPicker.css
- web/src/lib/ui/community-composer-mention-picker/CommunityComposerMentionPicker.svelte
- web/src/lib/ui/community-composer-room-icon/CommunityComposerRoomIcon.svelte
- web/src/lib/ui/community-composer-slash-picker/CommunityComposerSlashPicker.svelte
- web/src/lib/ui/community-composer-slash-picker/slash-picker-rows.ts
- web/src/lib/ui/community-composer-status.css
- web/src/lib/ui/community-conversation/CommunityConversation.svelte
- web/src/lib/ui/community-conversation/conversation-day.ts
- web/src/lib/ui/community-conversation/conversation.css
- web/src/lib/ui/community-details/CommunityDetails.svelte
- web/src/lib/ui/community-emoji-picker/CommunityEmojiPicker.svelte
- web/src/lib/ui/community-entity-chip/CommunityEntityChip.svelte
- web/src/lib/ui/community-entity-chip/CommunityEntityChipFrame.svelte
- web/src/lib/ui/community-entity-chip/CommunityEntityChipIcon.svelte
- web/src/lib/ui/community-entity-chip/community-entity-link.css
- web/src/lib/ui/community-entity-chip/navigation.ts
- web/src/lib/ui/community-markdown-content/CommunityChannelReference.svelte
- web/src/lib/ui/community-markdown-content/CommunityEmojiReference.svelte
- web/src/lib/ui/community-markdown-content/CommunityEntityReference.svelte
- web/src/lib/ui/community-markdown-content/CommunityMarkdownContent.svelte
- web/src/lib/ui/community-markdown-content/CommunityMemberReference.svelte
- web/src/lib/ui/community-markdown-content/CommunitySpoiler.svelte
- web/src/lib/ui/community-markdown-content/context.ts
- web/src/lib/ui/community-markdown-content/preview-mentions.ts
- web/src/lib/ui/community-markdown-content/syntax-entity.ts
- web/src/lib/ui/community-markdown-content/syntax-hast-links.ts
- web/src/lib/ui/community-markdown-content/syntax-hast.ts
- web/src/lib/ui/community-markdown-content/syntax-tokens.ts
- web/src/lib/ui/community-markdown-content/syntax.ts
- web/src/lib/ui/community-member-avatar/handle.ts
- web/src/lib/ui/community-member-hover-card/CommunityMemberHoverCard.svelte
- web/src/lib/ui/community-message-actions/CommunityMessageActions.svelte
- web/src/lib/ui/community-message-actions/message-action-hover.svelte.ts
- web/src/lib/ui/community-message/CommunityMessage.svelte
- web/src/lib/ui/community-message/CommunityMessageMeta.svelte
- web/src/lib/ui/community-message/CommunityMessageReplySummary.svelte
- web/src/lib/ui/community-message/community-message.css
- web/src/lib/ui/community-message/grouping.ts
- web/src/lib/ui/community-message/implicit-mentions.ts
- web/src/lib/ui/community-message/message-activity.ts
- web/src/lib/ui/community-message/message-local.css
- web/src/lib/ui/community-message/props.ts
- web/src/lib/ui/community-project-channel-icon/CommunityProjectChannelIcon.svelte
- web/src/lib/ui/community-projects/CommunityProjectsMarkdownPreview.svelte
- web/src/lib/ui/community-projects/StoreFileEditor.svelte
- web/src/lib/ui/community-reactions/CommunityReactions.svelte
- web/src/lib/ui/community-reply-context/CommunityReplyContext.svelte
- web/src/lib/ui/community-reply-preview/CommunityReplyPreview.svelte
- web/src/lib/ui/community-report-dialog/CommunityReportDialog.svelte
- web/src/lib/ui/community-search-command/CommunitySearchCommand.svelte
- web/src/lib/ui/community-search-command/model.ts
- web/src/lib/ui/community-search-command/search-command.css
- web/src/lib/ui/community-skeleton/CommunitySkeleton.svelte
- web/src/lib/ui/community-thread-panel/CommunityThreadMessage.svelte
- web/src/lib/ui/community-thread-panel/CommunityThreadPanel.svelte
- web/src/lib/ui/community-thread-panel/CommunityThreadTreeRow.svelte
- web/src/lib/ui/community-thread-panel/props.ts
- web/src/lib/ui/community-thread-panel/thread-panel.css
- web/src/lib/ui/community-thread-panel/thread-tree-layout.ts
- web/src/lib/ui/community-workspace/CommunityWorkspaceSurface.svelte
- web/src/lib/ui/community-workspace/model.ts
- web/src/lib/ui/focus-message.ts
- web/src/lib/ui/forum-topic-list/ForumTopicList.svelte
- web/src/lib/ui/forum-topic-list/VirtualForumTopicRows.svelte
- web/src/lib/ui/forum-topic-list/forum-topic-list.css
- web/src/lib/ui/forum-topic-list/topic-row-estimate.ts
- web/src/lib/ui/store-application/StoreApplication.svelte
- web/src/lib/ui/store-application/StoreApplicationHeaderActions.svelte
- web/src/lib/ui/store-application/create-store-shell.ts
- web/src/lib/ui/store-application/entity-context.ts
- web/src/lib/ui/store-collaboration/StoreAgentContext.svelte
- web/src/lib/ui/store-collaboration/StoreAgentToolReview.svelte
- web/src/lib/ui/store-collaboration/StoreAgentTranscript.svelte
- web/src/lib/ui/store-collaboration/StoreCommentableCommitDiff.svelte
- web/src/lib/ui/store-collaboration/StoreRepositoryActivity.svelte
- web/src/lib/ui/store-collaboration/StoreReviewComposer.svelte
- web/src/lib/ui/store-collaboration/StoreReviewConversation.svelte
- web/src/lib/ui/store-collaboration/StoreReviewDetail.svelte
- web/src/lib/ui/store-mutations/StoreMutationFields.svelte
- web/src/lib/ui/store-search/StoreSearchCommand.svelte
- web/src/lib/ui/store-search/StoreSearchOverlay.svelte
- web/src/lib/ui/store-search/search-context.ts
- web/src/lib/ui/store-threads/StoreMessageReactions.svelte
- web/src/lib/ui/store-threads/StoreThreadCreate.svelte
- web/src/lib/ui/store-threads/StoreThreadHeader.svelte
- web/src/lib/ui/store-threads/StoreThreadReplies.svelte
- web/src/lib/ui/store-threads/StoreThreadSettings.svelte
- web/src/lib/ui/store-threads/StoreThreadSidebar.svelte
- web/src/lib/ui/store-threads/StoreThreads.svelte
- web/src/lib/ui/store-threads/store-threads.css
- web/src/lib/ui/store-threads/thread-context.ts
- web/src/lib/ui/store-threads/thread-panel.svelte.ts
- web/src/lib/ui/store-threads/thread-view.ts
- web/src/lib/ui/store-work/StoreProjectsActivity.svelte
- web/src/lib/ui/store-work/StoreProjectsDirectory.svelte
- web/src/lib/ui/store-work/StoreProjectsRepositories.svelte
- web/src/lib/ui/store-work/StoreProjectsReviews.svelte
- web/src/lib/ui/store-work/StoreRepositoryDirectoryRow.svelte
- web/src/lib/ui/store-work/StoreWorkComments.svelte
- web/src/lib/ui/store-work/StoreWorkProjectForm.svelte
- web/src/lib/ui/store-work/StoreWorkProjectRows.svelte
- web/src/lib/ui/store-work/StoreWorkReviewRow.svelte
- web/src/lib/ui/store-work/StoreWorkTaskDetail.svelte
- web/src/lib/ui/store-work/StoreWorkTaskForm.svelte
- web/src/lib/ui/store-work/StoreWorkTaskRows.svelte
- web/src/lib/ui/store-work/repository-directory-presentation.ts
- web/src/lib/ui/store-work/work-presentation.ts
- web/tests/browser/threads-alignment.spec.ts
- web/tests/native-entity-links.test.ts
- web/tests/vite/global-search.test.ts
- web/tests/vite/production-threads.test.ts
- web/vite.config.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/browser/threads-alignment.spec.ts#cold LocalTrusted Threads shell
  settings references scoped search and reply focus
- web/tests/native-entity-links.test.ts#Thread references normalize legacy
  channels while Review thread identity remains distinct
- web/tests/native-entity-links.test.ts#slash picker inserts useful native
  titles and keeps complete IDs in Markdown routes
- web/tests/vite/global-search.test.ts#fans out over authorized sources with two
  readers and preserves exact native result routes
- web/tests/vite/global-search.test.ts#revocation removes cached source results
  and continuations and stops code discovery
- web/tests/vite/global-search.test.ts#sources removed while a query is in
  flight cannot leave results or continuations
- scripts/catalog-browser.test.mjs#cold_catalog_contract
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M15-021 details 2](details-02.md)
- [E-M15-021 details 3](details-03.md)
