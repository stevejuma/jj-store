---
schema: 1
id: E-M15-019
---

# E-M15-019

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

E-M15-019
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M15
- M15.5
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

owned-community-presentation-governance
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

175 local presentation copies with 177 pinned source references and 12 explicit native ownership declarations. Community package imports, substitution hook and Vite Turso helper removed. Provenance manifest V1 and ui-ownership spec-validator plugin added. No persistence format change.
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Working change msszrzpn starts at parent tpkpymol / 4dee949b in original isolated threads-reactions workspace. Thread conversation/composer/replies/Markdown still depend on the Community package and local checkout substitution; ownership regression check is absent.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- AGENTS.md
- docs/ui-provenance.json
- package.json
- pnpm-lock.yaml
- scripts/catalog-browser.test.mjs
- scripts/check-ui-ownership.mjs
- scripts/local-package-pnpmfile.cjs
- scripts/local-package-pnpmfile.test.mjs
- scripts/ui-ownership.test.mjs
- spec-validator.config.mjs
- spec/src/architecture.md
- spec/src/milestones.md
- spec/src/spec-governance.md
- spec/src/ui.md
- spec/src/verification.md
- web/src/lib/community/avatar-content.ts
- web/src/lib/community/community-link-normalize.ts
- web/src/lib/community/community-links.ts
- web/src/lib/community/composer-media.ts
- web/src/lib/community/composer-model.ts
- web/src/lib/community/content-mentions.ts
- web/src/lib/community/contracts.ts
- web/src/lib/community/conversations.ts
- web/src/lib/community/send-contracts.ts
- web/src/lib/community/workspace-model.ts
- web/src/lib/projects/repository-log-canvas.ts
- web/src/lib/projects/repository-log-collapse.ts
- web/src/lib/projects/repository-log-files.ts
- web/src/lib/projects/repository-log-graph.ts
- web/src/lib/projects/repository-log-message.ts
- web/src/lib/projects/repository-log.ts
- web/src/lib/ui/community-app-shell/CommunityAppShell.svelte
- web/src/lib/ui/community-app-shell/CommunitySidebarToggleIcon.svelte
- web/src/lib/ui/community-app-shell/right-sidebar.css
- web/src/lib/ui/community-app-shell/shell-header-geometry.ts
- web/src/lib/ui/community-application/CommunityApplicationMobileNavigation.svelte
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
- web/src/lib/ui/community-chip/CommunityChip.svelte
- web/src/lib/ui/community-chip/model.ts
- web/src/lib/ui/community-chip/props.ts
- web/src/lib/ui/community-commit-diff/CommunityCommitDiff.svelte
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
- web/src/lib/ui/community-diff-controls/CommunityDiffControls.svelte
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
- web/src/lib/ui/community-member-avatar-group/CommunityMemberAvatarGroup.svelte
- web/src/lib/ui/community-member-avatar/CommunityMemberAvatar.svelte
- web/src/lib/ui/community-member-avatar/community-member-avatar.css
- web/src/lib/ui/community-member-avatar/handle.ts
- web/src/lib/ui/community-member-avatar/model.ts
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
- web/src/lib/ui/community-navigation/CommunityNavigation.css
- web/src/lib/ui/community-navigation/CommunityNavigation.svelte
- web/src/lib/ui/community-navigation/CommunityNavigationSearch.svelte
- web/src/lib/ui/community-navigation/CommunityNavigationTree.css
- web/src/lib/ui/community-navigation/model.ts
- web/src/lib/ui/community-overlays.css
- web/src/lib/ui/community-profile-menu/CommunityProfileMenu.svelte
- web/src/lib/ui/community-project-channel-icon/CommunityProjectChannelIcon.svelte
- web/src/lib/ui/community-projects/CommunityProjectsGitAuthorAvatar.svelte
- web/src/lib/ui/community-projects/CommunityProjectsMarkdownPreview.svelte
- web/src/lib/ui/community-projects/CommunityProjectsRepositoryBranchSelector.svelte
- web/src/lib/ui/community-projects/CommunityProjectsRepositoryBreadcrumb.svelte
- web/src/lib/ui/community-projects/CommunityProjectsRepositoryDirectory.svelte
- web/src/lib/ui/community-projects/CommunityProjectsRepositoryDirectoryRows.svelte
- web/src/lib/ui/community-projects/CommunityProjectsRepositoryDocuments.svelte
- web/src/lib/ui/community-projects/CommunityProjectsRepositoryFilePanel.svelte
- web/src/lib/ui/community-projects/CommunityProjectsRepositoryFilePanelHeader.svelte
- web/src/lib/ui/community-projects/CommunityProjectsRepositoryFileTree.svelte
- web/src/lib/ui/community-projects/CommunityProjectsRepositoryFiles.svelte
- web/src/lib/ui/community-projects/CommunityProjectsRepositoryLog.svelte
- web/src/lib/ui/community-projects/CommunityProjectsRepositoryLogDiff.svelte
- web/src/lib/ui/community-projects/CommunityProjectsRepositoryLogEmpty.svelte
- web/src/lib/ui/community-projects/CommunityProjectsRepositoryLogFiles.svelte
- web/src/lib/ui/community-projects/CommunityProjectsRepositoryLogHeader.svelte
- web/src/lib/ui/community-projects/CommunityProjectsRepositoryLogRail.svelte
- web/src/lib/ui/community-projects/CommunityProjectsRepositorySkeleton.svelte
- web/src/lib/ui/community-projects/CommunityProjectsRevisionGraph.svelte
- web/src/lib/ui/community-projects/CommunityProjectsRevisionGraphCopy.svelte
- web/src/lib/ui/community-projects/CommunityProjectsRevisionGraphIds.svelte
- web/src/lib/ui/community-projects/CommunityProjectsRevisionGraphMessage.svelte
- web/src/lib/ui/community-projects/CommunityProjectsVirtualRows.svelte
- web/src/lib/ui/community-projects/repository-browser.ts
- web/src/lib/ui/community-projects/repository-directory.css
- web/src/lib/ui/community-projects/repository-documents.css
- web/src/lib/ui/community-projects/repository-file-tree-resize.css
- web/src/lib/ui/community-projects/repository-files.css
- web/src/lib/ui/community-projects/repository-log-diff.css
- web/src/lib/ui/community-projects/repository-log.css
- web/src/lib/ui/community-projects/repository-skeleton.css
- web/src/lib/ui/community-projects/revision-graph-ids.css
- web/src/lib/ui/community-projects/virtual-project-row-estimates.ts
- web/src/lib/ui/community-reactions/CommunityReactions.svelte
- web/src/lib/ui/community-relative-time/CommunityRelativeTime.svelte
- web/src/lib/ui/community-relative-time/model.ts
- web/src/lib/ui/community-reply-context/CommunityReplyContext.svelte
- web/src/lib/ui/community-reply-preview/CommunityReplyPreview.svelte
- web/src/lib/ui/community-report-dialog/CommunityReportDialog.svelte
- web/src/lib/ui/community-repository-branch-command/CommunityRepositoryBranchCommand.svelte
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
- web/src/lib/ui/community-typography.css
- web/src/lib/ui/community-workspace/CommunityWorkspaceSurface.svelte
- web/src/lib/ui/community-workspace/model.ts
- web/src/lib/ui/forum-topic-list/ForumTopicList.svelte
- web/src/lib/ui/forum-topic-list/VirtualForumTopicRows.svelte
- web/src/lib/ui/forum-topic-list/forum-topic-list.css
- web/src/lib/ui/forum-topic-list/topic-row-estimate.ts
- web/src/lib/ui/store-application/StoreApplication.svelte
- web/src/lib/ui/store-threads/StoreThreadHeader.svelte
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- scripts/ui-ownership.test.mjs#rejects runtime, type, reexport and lazy
  Community module dependencies
- scripts/ui-ownership.test.mjs#rejects CSS, package aliases, lock resolutions
  and checkout configuration
- scripts/ui-ownership.test.mjs#rejects external production symlinks and invalid
  or missing provenance
- scripts/ui-ownership.test.mjs#requires complete recognized and explicitly
  marked copy coverage
- scripts/ui-ownership.test.mjs#rejects Community CSS imports inside Svelte
  styles
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M15-019 details 2](details-02.md)
