---
schema: 1
id: E-UI-033
---

# E-UI-033

<!-- evidence-field:id markdown -->
## Id

E-UI-033
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M4.20
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-30
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

repository-right-rail-navigation-and-responsive-tabs
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

Enclosing Jujutsu change uqvnuppm.
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Repository view links occupied the left rail; body tabs were Activity, Files, Workspaces, Review and Commits inside a horizontal ScrollArea, with no Op Log tab. The new RightRailNavigation regression failed once before integration.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-application/RepositoryReview.stories.svelte
- catalog/components/store-application/StoreApplication.stories.svelte
- catalog/components/store-application/StoreCollaboration.stories.svelte
- catalog/components/store-application/StoreFileEditor.stories.svelte
- catalog/components/store-application/StoreRepositoryApplication.stories.svelte
- catalog/components/store-application/StoreRepositoryBranchContext.stories.svelte
- catalog/components/store-application/StoreRepositoryNavigation.stories.svelte
- spec/src/api.md
- spec/src/milestones.md
- spec/src/ui.md
- spec/src/verification.md
- web/src/lib/ui/store-application/StoreApplication.svelte
- web/src/lib/ui/store-application/StoreRepositoryApplication.svelte
- web/src/lib/ui/store-application/StoreRepositoryDetails.svelte
- web/src/lib/ui/store-application/StoreRepositoryNavigation.svelte
- web/src/lib/ui/store-application/StoreRepositoryNotices.svelte
- web/src/lib/ui/store-application/StoreRepositoryTabs.svelte
- web/src/lib/ui/store-application/repository-sections.ts
- web/src/lib/ui/store-application/store-application.css
- web/tests/browser/mutations.spec.ts
- web/tests/browser/navigation.spec.ts
- web/tests/browser/reads.spec.ts
- web/tests/browser/repository-navigation.spec.ts
- web/tests/browser/workspaces.spec.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/browser/repository-navigation.spec.ts#right rail destinations match
  repository tabs and preserve workspace snapshot
- web/tests/browser/repository-navigation.spec.ts#phone overflow supports
  keyboard selection, reload, resize and right rail navigation
- web/tests/browser/repository-navigation.spec.ts#request errors use
  notifications while the right rail stays minimal
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-UI-033 details 2](details-02.md)
- [E-UI-033 details 3](details-03.md)
- [E-UI-033 details 4](details-04.md)
