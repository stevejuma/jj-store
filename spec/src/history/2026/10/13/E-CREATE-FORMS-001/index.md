---
schema: 1
id: E-CREATE-FORMS-001
---

# E-CREATE-FORMS-001

<!-- evidence-field:id markdown -->
## Id

E-CREATE-FORMS-001
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-13
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M14.2
- M11.4
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-016
- JST-WORK-008
- JST-UI-015
- JST-GOV-002
- JST-GOV-008
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Apply documented Settings form presentation to server repository creation and project creation/editing. Reuse owned cards, headings, row icons and footer composition with released Design Core. WorkField icons are opt-in; unadorned consumers keep their existing markup. The hub project dialog associates its fixed footer with the existing form and preserves empty-title, loading and pending guards. UI-only targeted scope; no Rust, server restart, root aggregate suites, provider execution or visual-baseline promotion. The enclosing Jujutsu change is the evidence revision.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Review visual styling; retain existing M11/M14 storage, platform and runtime qualification gates.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-UI-016 and JST-WORK-008 UI chapters explicitly describe Settings
  cards, responsive icon rows and project footer submission. WorkStore commands,
  version guards, draft owners, writable-home policy, exact pending identities,
  native creation authority and all API/storage formats are unchanged.
  JST-UI-015 and JST-GOV-002/008 ownership is retained.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M14.2 remains Done, M11.4 remains In progress. Focused UI
  interaction/Axe and responsive evidence does not promote independent
  native/cold persistence, package, distribution or visual-baseline gates.

```
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-application/RepositoryEntryControls.stories.svelte
- catalog/components/store-work/ProjectsHub.stories.svelte
- catalog/fixtures/project-creation-settings-play.ts
- docs/maintenance/coverage.json
- docs/site/guides/settings.md
- docs/ui-provenance.json
- spec/src/current/contracts/documentation/01-application-documentation.md
- spec/src/current/contracts/ui/17-native-projects-and-tasks.md
- spec/src/current/contracts/ui/28-complete-supported-configuration-settings.md
- spec/src/current/governance/spec-governance.md
- spec/src/current/progress/M11/index.md
- spec/src/current/progress/M14/index.md
- spec/src/current/verification/ui-02.md
- spec/src/current/verification/work.md
- web/src/lib/ui/store-settings/StoreNativeRepositoryCreate.svelte
- web/src/lib/ui/store-settings/creation-forms.css
- web/src/lib/ui/store-work/StoreWorkDialog.svelte
- web/src/lib/ui/store-work/StoreWorkField.svelte
- web/src/lib/ui/store-work/StoreWorkProjectForm.svelte
- spec/src/current/contracts/api/21-workstore-service-and-transports.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

[]
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-CREATE-FORMS-001 details 2](details-02.md)
- [E-CREATE-FORMS-001 details 3](details-03.md)
