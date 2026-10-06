---
schema: 1
id: E-UI-100
---

# E-UI-100

<!-- evidence-field:id markdown -->
## Id

E-UI-100
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M4.5
- M4.18
- M4.31
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-002
- JST-UI-012
- JST-UI-019
- JST-API-005
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-04
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

repository-entry-and-settings-presentation
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change pkrqsmov
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Six focused Storybook failures reproduce separate repository entry buttons, absent header spacing, unbordered creation/conversion forms, unstyled conversion actions, missing folder-notice clearance and inline repository-summary errors. One controller test reproduces error-episode loss during retry, including queued reads. After the requested full repository labels, the corrected width regression observes two text lines instead of one.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64; Node 24.10.0; pnpm 10.32.1; Svelte 5.57.1; Vitest 4.1.11; Storybook 10.5.7; existing explicit target/debug/examples/browser_preview. Installed Design Core 0.1.2 is the existing ignored local-source substitution used for validation; released registry-only acceptance remains separate.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Presentation and transient read-state behavior only. No routes, schemas, generated contracts, credentials, persisted notification state, repository, storage, WAL, receipt, companion or server-configuration formats changed.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-application/RepositoryEntryControls.stories.svelte
- catalog/components/store-application/StoreLocalFolderDialog.stories.svelte
- catalog/components/store-application/StoreRepositoryConversion.stories.svelte
- catalog/components/store-work/ProjectsDirectory.stories.svelte
- catalog/fixtures/RepositoryEntryControlsFixture.svelte
- spec/src/api.md
- spec/src/spec-governance.md
- spec/src/ui.md
- web/src/lib/controller/projects-repositories.svelte.ts
- web/src/lib/notifications/notification-recovery.svelte.ts
- web/src/lib/ui/AGENTS.md
- web/src/lib/ui/community-app-shell/CommunityAppShell.svelte
- web/src/lib/ui/community-navigation/CommunityNavigationAction.svelte
- web/src/lib/ui/community-navigation/CommunityNavigationSectionPicker.svelte
- web/src/lib/ui/community-navigation/icons.ts
- web/src/lib/ui/community-navigation/model.ts
- web/src/lib/ui/store-application/StoreApplicationNavigation.svelte
- web/src/lib/ui/store-settings/StoreLocalFolderDialog.svelte
- web/src/lib/ui/store-settings/StoreNativeRepositoryCreate.svelte
- web/src/lib/ui/store-settings/StoreRepositoryConversion.svelte
- web/src/lib/ui/store-settings/StoreRepositoryConversionDialog.svelte
- web/src/lib/ui/store-work/StoreProjectsAddMenu.svelte
- web/src/lib/ui/store-work/StoreRepositoryDirectoryRow.svelte
- web/src/lib/ui/store-work/StoreWorkField.svelte
- web/src/lib/ui/store-work/community-work-directory.css
- web/src/lib/ui/store-work/community-work-forms.css
- web/src/lib/ui/store-work/store-work.css
- web/tests/browser/native-repository-creation.spec.ts
- web/tests/browser/sidebar-repository-actions.spec.ts
- web/tests/vite/projects-repositories.test.ts
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/vite/projects-repositories.test.ts#directory retries retain an error
  episode until successful recovery, including queued reads
- web/tests/browser/native-repository-creation.spec.ts#cold native creation
  recovers exact intent across entrypoints and reload
- web/tests/browser/native-repository-creation.spec.ts#cold server creation
  stays unavailable without installation capability
- web/tests/browser/native-repository-creation.spec.ts#cold native creation
  keeps name rejection inline and permits a corrected request
- web/tests/browser/repository-conversion.spec.ts#cold export preflight, pending
  reload, exact retry and verified report
- web/tests/browser/repository-conversion.spec.ts#cold unsupported conversion
  cannot dispatch
- web/tests/browser/repository-conversion.spec.ts#cold conversion status retries
  preserve a dismissed error episode
- web/tests/browser/sidebar-repository-actions.spec.ts#sidebar add actions open
  the matching existing dialogs without writes
- web/tests/browser/notifications.spec.ts#operational toasts deduplicate, retain
  dismissal and clear on recovery
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-UI-100 details 2](details-02.md)
- [E-UI-100 details 3](details-03.md)
- [E-UI-100 details 4](details-04.md)
