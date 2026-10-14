---
schema: 1
id: E-CI-BROWSER-001
---

# E-CI-BROWSER-001 details 10

<!-- evidence-field:files list -->
## Files

- .changeset/ci-cold-browser-freshness.md
- .github/workflows/ci-heavy.yml
- .gitignore
- .prettierignore
- README.md
- catalog/components/store-appearance/AppearanceShell.stories.svelte
- catalog/components/store-application/AgentDefaults.stories.svelte
- catalog/components/store-application/AgentInboxTree.stories.svelte
- catalog/components/store-application/AgentPromptMarkup.stories.svelte
- catalog/components/store-application/NativeAgentMenus.stories.svelte
- catalog/components/store-application/NativeReviewDetail.stories.svelte
- catalog/components/store-application/RepositoryNotices.stories.svelte
- catalog/components/store-application/RepositoryReview.stories.svelte
- catalog/components/store-application/SettingsNavigation.stories.svelte
- catalog/components/store-application/StoreFileEditor.stories.svelte
- catalog/components/store-application/StoreMultiHarnessSettings.stories.svelte
- catalog/components/store-application/StoreRepositoryFeedback.stories.svelte
- catalog/components/store-application/WorkspaceFiles.stories.svelte
- catalog/components/store-application/agent-sidebar-play.ts
- catalog/components/store-mutations/StoreConflictEditor.stories.svelte
- catalog/components/store-threads/StoreThreadAppearance.stories.svelte
- catalog/components/store-work/ProjectsHomeSpacing.stories.svelte
- catalog/controller/in-memory-store-controller.svelte.ts
- catalog/controller/store-controller.test.ts
- catalog/fixtures/delta-chat/adapter.test.ts
- catalog/fixtures/delta-chat/captured-code.test.ts
- catalog/vitest.config.ts
- crates/jj-store-cli/tests/server_executable/git_bridge.rs
- crates/jj-store-jj/src/query/cache/managed.rs
- crates/jj-store-jj/src/query/cache.rs
- docs/maintenance/coverage.json
- docs/site/guides/settings.md
- package.json
- pnpm-lock.yaml
- scripts/automation-governance.test.mjs
- scripts/catalog-browser.test.mjs
- scripts/ci-browser.mjs
- scripts/ci.test.mjs
- scripts/profile-acceptance.mjs
- scripts/real-browser-acceptance.mjs
- spec/src/current/contracts/api/01-read-apis-and-browser.md
- spec/src/current/contracts/api/10-file-differences-and-conflicts.md
- spec/src/current/contracts/api/15-browser-read-client.md
- spec/src/current/contracts/api/35-scoped-application-freshness.md
- spec/src/current/contracts/api/41-interactive-agent-execution.md
- spec/src/current/contracts/architecture/23-public-distribution.md
- spec/src/current/contracts/cli/01-cli-and-server-commands.md
- spec/src/current/contracts/documentation/01-application-documentation.md
- spec/src/current/contracts/git-interop/01-explicit-git-interoperability.md
- spec/src/current/contracts/jj-integration/14-bounded-application-graphs.md
- spec/src/current/contracts/ui/02-repository-owned-community-presentation.md
- spec/src/current/contracts/ui/07-application-startup.md
- spec/src/current/contracts/ui/09-transient-store-settings.md
- spec/src/current/contracts/ui/11-shell-composition.md
- spec/src/current/contracts/ui/18-installation-projects-hub.md
- spec/src/current/contracts/ui/27-usable-capture-settings-lifecycle.md
- spec/src/current/contracts/ui/32-event-driven-freshness-migration.md
- spec/src/current/contracts/ui/36-commentable-chat-body-prototype.md
- spec/src/current/governance/06-verification-scope.md
- spec/src/current/progress/M30/index.md
- spec/src/history/2026/10/14/E-CI-BROWSER-001/SUMMARY.nav.md
- spec/src/history/2026/10/14/E-CI-BROWSER-001/details-02.md
- spec/src/history/2026/10/14/E-CI-BROWSER-001/details-03.md
- spec/src/history/2026/10/14/E-CI-BROWSER-001/details-04.md
- spec/src/history/2026/10/14/E-CI-BROWSER-001/details-05.md
- spec/src/history/2026/10/14/E-CI-BROWSER-001/details-06.md
- spec/src/history/2026/10/14/E-CI-BROWSER-001/details-07.md
- spec/src/history/2026/10/14/E-CI-BROWSER-001/details-08.md
- spec/src/history/2026/10/14/E-CI-BROWSER-001/details-09.md
- spec/src/history/2026/10/14/E-CI-BROWSER-001/details-10.md
- spec/src/history/2026/10/14/E-CI-BROWSER-001/index.md
- spec/src/history/2026/10/14/SUMMARY.nav.md
- spec/src/history/2026/10/14/index.md
- web/playwright.config.ts
- web/playwright.docs.config.ts
- web/playwright.real.config.ts
- web/src/lib/controller/capture-refresh.ts
- web/src/lib/controller/production-automatic-refresh.ts
- web/src/lib/controller/production-profile-route.ts
- web/src/lib/controller/production-projects-hub.svelte.ts
- web/src/lib/controller/production-read-services.ts
- web/src/lib/controller/production-route-restorer.ts
- web/src/lib/controller/production-store-actions.ts
- web/src/lib/controller/production-work.svelte.ts
- web/src/lib/controller/projects-dirtiness.ts
- web/src/lib/controller/runtime-reload.svelte.ts
- web/src/lib/controller/store-route-browser.ts
- web/src/lib/controller/store-route-history.svelte.ts
- web/src/lib/controller/work-types.ts
- web/src/lib/controller/working-copy-actions.ts
- web/src/lib/conversations/agent-adapter.ts
- web/src/lib/conversations/legacy-recorded.ts
- web/src/lib/ui/community-inbox/CommunityInboxSessions.svelte
- web/src/lib/ui/store-agent-chat-body/captured-code.ts
- web/src/lib/ui/store-agent-chat-body/markdown-blocks.ts
- web/src/lib/ui/store-agent-chat-body/text-reader.css
- web/src/lib/ui/store-application/StoreViewTabs.svelte
- web/src/lib/ui/store-collaboration/StoreRecordedConversation.svelte
- web/src/lib/ui/store-settings/StoreGlobalSettingsLoading.svelte
- web/src/lib/ui/store-settings/StoreRepositoryConversion.svelte
- web/src/lib/ui/store-threads/conversation-parent-scroll.ts
- web/src/lib/ui/store-work/StoreProjects.svelte
- web/src/lib/ui/store-work/projects-initial-loading.ts
- web/tests/browser/agent-chat-changes.spec.ts
- web/tests/browser/agent-fleet.spec.ts
- web/tests/browser/agent-prompt-markup.spec.ts
- web/tests/browser/agent-tool-rich.spec.ts
- web/tests/browser/appearance.spec.ts
- web/tests/browser/automatic-refresh.spec.ts
- web/tests/browser/conflicts.spec.ts
- web/tests/browser/editor-focus.spec.ts
- web/tests/browser/file-editor-navigation.spec.ts
- web/tests/browser/lazy-views.spec.ts
- web/tests/browser/local-bootstrap.spec.ts
- web/tests/browser/local-capture-loading.spec.ts
- web/tests/browser/local-capture-settings.spec.ts
- web/tests/browser/local-search-settings.spec.ts
- web/tests/browser/local-server-settings.spec.ts
- web/tests/browser/local-storage.spec.ts
- web/tests/browser/local-sync.spec.ts
- web/tests/browser/local-workspaces.spec.ts
- web/tests/browser/mutations.spec.ts
- web/tests/browser/native-browser-fixture.ts
- web/tests/browser/notifications.spec.ts
- web/tests/browser/reaction-metadata.spec.ts
- web/tests/browser/reads.spec.ts
- web/tests/browser/repository-conversion.spec.ts
- web/tests/browser/repository-navigation.spec.ts
- web/tests/browser/repository-settings-navigation.ts
- web/tests/browser/search.spec.ts
- web/tests/browser/sidebar-repository-actions.spec.ts
- web/tests/browser/thread-performance.spec.ts
- web/tests/browser/threads-alignment.spec.ts
- web/tests/real/event-freshness.spec.ts
- web/tests/real/git-bridge.spec.ts
- web/tests/real/local-mixed.spec.ts
- web/tests/real/native-repository.spec.ts
- web/tests/real/profiles-emoji.spec.ts
- web/tests/real/project-polish.spec.ts
- web/tests/real/project-repository-actions.spec.ts
- web/tests/real/projects-activity.spec.ts
- web/tests/real/repository-settings-presentation.spec.ts
- web/tests/real/repository-settings.spec.ts
- web/tests/real/settings-layout.spec.ts
- web/tests/real/work-seed.spec.ts
- web/tests/real/work-store.spec.ts
- web/tests/vite/capture-event-scope.test.ts
- web/tests/vite/conversation-projection.test.ts
- web/tests/vite/production-automatic-refresh.test.ts
- web/tests/vite/projects-code-navigation.test.ts
- web/tests/vite/projects-freshness.test.ts
- web/tests/vite/projects-initial-loading.test.ts
- web/tests/vite/repository-settings-restoration.test.ts
- web/tests/vite/runtime-reload.test.ts
- web/tests/vite/store-route-history.test.ts
- web/tests/vite/working-copy-actions.test.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/vite/runtime-reload.test.ts#idle reload owners make no periodic
  requests and explicit observation reads fresh status
- web/tests/vite/projects-freshness.test.ts#task reactions retain home-scoped
  dirtiness without dirtying code or other project domains
- web/tests/vite/projects-initial-loading.test.ts#captured project repository
  views survive summary invalidation while initial and task views still wait for
  their owners
- crates/jj-store-jj/src/query/cache/managed.rs#ordinary_reservations_evict_idle_code_namespaces_at_slot_limit
- crates/jj-store-jj/src/query/cache/managed.rs#ordinary_reservations_evict_only_same_repository_at_repository_limit
- catalog/fixtures/delta-chat/captured-code.test.ts#captured TypeScript provides
  its grammar without an editable file host
- web/tests/vite/capture-event-scope.test.ts#healthy capture reads only changed
  status, sources and configuration interests
- web/tests/browser/threads-alignment.spec.ts#cold LocalTrusted Threads shell
  settings references scoped search and reply focus
- web/tests/browser/thread-performance.spec.ts#cold Threads load conversation
  before directory and confirmed reaction refreshes one target
<!-- /evidence-field -->
