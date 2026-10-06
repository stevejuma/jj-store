---
schema: 1
id: E-UI-096
---

# E-UI-096

<!-- evidence-field:id markdown -->
## Id

E-UI-096
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M4.31
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-04
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

session-notification-source-and-consumer-qualification
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

kkoupnws (enclosing change)
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Operational feedback was rendered in individual pages/dialogs without shared identity, dismissal or lifetime ownership. Field validation and durable unknown-outcome recovery already owned contextual state.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64; pnpm10.32.1, Svelte5, Vitest4.1.11, consumer Playwright1.63.0; Design Core source0.1.2 in enclosing source change lqkpxqtl. Released dependency acceptance remains distinct.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No storage, manifest, WAL, API wire, CLI, credential or persisted notification formats changed. Notification event subscriptions carry plain data; callbacks remain session-local. OS delivery and restart history are excluded.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- AGENTS.md
- catalog/components/store-application/StoreApplication.stories.svelte
- catalog/components/store-application/StoreNotifications.stories.svelte
- catalog/components/store-application/StoreSyncSettings.stories.svelte
- catalog/components/store-application/StoreWorkspaceSettings.stories.svelte
- catalog/components/store-mutations/StoreConflictEditor.stories.svelte
- catalog/fixtures/NotificationsFixture.svelte
- catalog/fixtures/WorkspaceSettingsFixture.svelte
- spec/src/api.md
- spec/src/spec-governance.md
- spec/src/ui.md
- spec/src/verification.md
- web/src/App.svelte
- web/src/lib/controller/production-search-settings.ts
- web/src/lib/controller/production-store-view.ts
- web/src/lib/controller/production-workspace-links.ts
- web/src/lib/controller/search-settings.ts
- web/src/lib/controller/workspace-link-types.ts
- web/src/lib/notifications/notification-context.svelte.ts
- web/src/lib/notifications/notification-recovery.svelte.ts
- web/src/lib/notifications/notification-service.svelte.ts
- web/src/lib/ui/community-message/CommunityMessage.svelte
- web/src/lib/ui/community-projects/StoreFileEditor.svelte
- web/src/lib/ui/store-application/StoreApplication.svelte
- web/src/lib/ui/store-application/StoreApplicationPlaceholder.svelte
- web/src/lib/ui/store-application/StoreConnection.svelte
- web/src/lib/ui/store-application/StoreRepositoryNotices.svelte
- web/src/lib/ui/store-collaboration/StoreRepositoryReviewDialog.svelte
- web/src/lib/ui/store-conflicts/StoreConflictEditor.svelte
- web/src/lib/ui/store-mutations/StoreMutationDialog.svelte
- web/src/lib/ui/store-notifications/StoreNotice.svelte
- web/src/lib/ui/store-notifications/StoreNotificationHost.svelte
- web/src/lib/ui/store-settings/StoreGraphIndexSettings.svelte
- web/src/lib/ui/store-settings/StoreSearchConfiguration.svelte
- web/src/lib/ui/store-settings/StoreSearchHealth.svelte
- web/src/lib/ui/store-settings/StoreSettingsRepository.svelte
- web/src/lib/ui/store-settings/StoreStorageSettings.svelte
- web/src/lib/ui/store-settings/StoreSyncReconciliation.svelte
- web/src/lib/ui/store-settings/StoreSyncSettings.svelte
- web/src/lib/ui/store-settings/StoreSyncWait.svelte
- web/src/lib/ui/store-settings/StoreWorkspaceSettings.svelte
- web/src/lib/ui/store-threads/StoreThreadCreate.svelte
- web/src/lib/ui/store-threads/StoreThreadLifecycleActions.svelte
- web/src/lib/ui/store-threads/StoreThreadSettings.svelte
- web/src/lib/ui/store-threads/StoreThreads.svelte
- web/src/lib/ui/store-work/StoreProjects.svelte
- web/src/lib/ui/store-work/StoreRepositoryProjects.svelte
- web/src/lib/ui/store-work/StoreWorkRemoveComment.svelte
- web/src/lib/ui/store-work/StoreWorkStatus.svelte
- web/src/lib/ui/store-working-copy/StoreWorkingFileEditor.svelte
- web/tests/browser/notifications.spec.ts
- web/tests/vite/notification-service.test.ts
- web/tests/vite/production-search-settings.test.ts
- web/tests/vite/production-store-workspaces.test.ts
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/browser/notifications.spec.ts#operational toasts deduplicate, retain
  dismissal and clear on recovery
- web/tests/vite/production-store-workspaces.test.ts#invalid folder binding
  stays field validation and retains the path
- web/tests/vite/production-search-settings.test.ts#invalid search drafts stay
  inline across refresh and clear on cancellation
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-UI-096 details 2](details-02.md)
