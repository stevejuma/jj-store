---
schema: 1
id: E-M15-025
---

# E-M15-025

<!-- evidence-field:id markdown -->
## Id

E-M15-025
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M15
- M15.5
- M15.6
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-01
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

thread-forum-settings-metadata-replies-independent-sidebars
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change ultsumzo
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Existing isolated Threads workspace clean above latest main landing/restart. Browse directory lacks Community inline post composer; lifecycle actions live in settings, kebab precedes Refresh, actor dialogs require separate buttons, first reply hover actions clip and selected reply history can lack its root. Main PID 43304 remains unchanged.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

JJ 0.45.1; pnpm 10.32.1; Svelte 5.57.1; Community reference 741b8285e2081f1c9af58afe5a9b72f538fad781; inherited development Design Core 0.1.2 snapshot, not registry-only foundation qualification.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Owned Community forum composer/rows; native stable create/body requests and existing receipt semantics; explicit snapshot refresh preserves bounded loaded history and exact revision ancestors. Edited/footer feedback uses metadata area. No native formats or API/schema changes. Community standard bordered pane and named outer details rail use distinct persistent panel identities and may open simultaneously; existing shared value seeds the new rail only until its own value exists.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-threads/StoreCapturedReactions.stories.svelte
- catalog/components/store-threads/StoreThreadAlignment.stories.svelte
- catalog/components/store-threads/StoreThreads.stories.svelte
- catalog/components/store-work/ProjectsDirectory.stories.svelte
- docs/ui-provenance.json
- spec/src/api.md
- spec/src/milestones.md
- spec/src/spec-governance.md
- spec/src/ui.md
- web/src/lib/collaboration/thread-presentation.ts
- web/src/lib/community/workspace-model.ts
- web/src/lib/controller/collab-actions.svelte.ts
- web/src/lib/controller/production-threads.svelte.ts
- web/src/lib/controller/thread-history.ts
- web/src/lib/controller/thread-post.svelte.ts
- web/src/lib/ui/community-channel-settings/CommunityChannelSettings.svelte
- web/src/lib/ui/community-message/CommunityMessage.svelte
- web/src/lib/ui/community-message/CommunityMessageMeta.svelte
- web/src/lib/ui/community-reactions/CommunityReactionActors.svelte
- web/src/lib/ui/community-reactions/CommunityReactions.svelte
- web/src/lib/ui/community-thread-panel/thread-panel.css
- web/src/lib/ui/community-topic-composer/CommunityTopicComposer.svelte
- web/src/lib/ui/community-topic-composer/CommunityTopicComposerEditor.svelte
- web/src/lib/ui/forum-topic-list/ForumTopicList.svelte
- web/src/lib/ui/layout-storage.ts
- web/src/lib/ui/store-application/StoreApplication.svelte
- web/src/lib/ui/store-application/StoreApplicationRightPanels.svelte
- web/src/lib/ui/store-application/create-store-shell.ts
- web/src/lib/ui/store-threads/StoreMessageReactions.svelte
- web/src/lib/ui/store-threads/StoreThreadHeader.svelte
- web/src/lib/ui/store-threads/StoreThreadLifecycleActions.svelte
- web/src/lib/ui/store-threads/StoreThreadMenu.svelte
- web/src/lib/ui/store-threads/StoreThreadReplies.svelte
- web/src/lib/ui/store-threads/StoreThreadSettings.svelte
- web/src/lib/ui/store-threads/StoreThreads.svelte
- web/src/lib/ui/store-threads/store-threads.css
- web/src/lib/ui/store-work/StoreProjects.svelte
- web/src/lib/ui/store-work/community-work-layout.css
- web/tests/browser/threads-alignment.spec.ts
- web/tests/vite/production-threads.test.ts
- web/tests/vite/store-shell-layout.test.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/vite/production-threads.test.ts#focused replies load their missing
  ancestors before presentation
- web/tests/vite/production-threads.test.ts#inline forum posts retain content
  through unknown create and body receipts
- web/tests/vite/production-threads.test.ts#refresh follows a reply's historical
  parent revision after root edits
- web/tests/browser/threads-alignment.spec.ts#cold LocalTrusted Threads shell
  settings references scoped search and reply focus
- web/tests/vite/store-shell-layout.test.ts#standard Thread pane and outer
  details rail persist widths independently
- web/tests/vite/production-threads.test.ts#inline posts recover after transport
  recreation without blocking another repository
- web/tests/vite/production-threads.test.ts#rejected initial body keeps the
  directory draft and retries its created thread
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M15-025 details 2](details-02.md)
- [E-M15-025 details 3](details-03.md)
- [E-M15-025 details 4](details-04.md)
- [E-M15-025 details 5](details-05.md)
