---
schema: 1
id: E-UI-035
---

# E-UI-035

<!-- evidence-field:id markdown -->
## Id

E-UI-035
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M4.22
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-30
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

selected-repository-rail-actions
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

Enclosing Jujutsu change mouksruo.
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

The right rail rendered only its heading/create control and Details navigation. The existing capability-aware Actions panel was disconnected. New selected-change and selected-operation stories each failed to find it; the two existing navigation stories passed.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- web/src/lib/ui/store-application/StoreApplication.svelte
- web/src/lib/ui/store-application/StoreRepositoryContextActions.svelte
- catalog/components/store-application/StoreRepositoryNavigation.stories.svelte
- catalog/components/store-application/StoreRepositoryBranchContext.stories.svelte
- web/tests/browser/context-actions.spec.ts
- spec/src/api.md
- spec/src/ui.md
- spec/src/milestones.md
- spec/src/verification.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/browser/context-actions.spec.ts#selected JJ Log change restores rail
  commands and clears them on navigation
- web/tests/browser/context-actions.spec.ts#selected change rail commands
  respect unavailable capabilities
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-UI-035 details 2](details-02.md)
- [E-UI-035 details 3](details-03.md)
