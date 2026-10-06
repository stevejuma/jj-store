---
schema: 1
id: E-UI-031
---

# E-UI-031

<!-- evidence-field:id markdown -->
## Id

E-UI-031
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M4.18
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-30
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

compact-store-settings-navigation-regression
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Collapsing the desktop left sidebar reduced the shell rail to 48 pixels, but StoreSettingsNavigation did not receive the shell compact state. Its Back, section heading and three section labels remained full width and were clipped instead of rendering the expected icon-only navigation.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-application/StoreApplication.stories.svelte
- catalog/components/store-application/StoreRepositorySearch.stories.svelte
- spec/src/api.md
- spec/src/milestones.md
- spec/src/ui.md
- web/src/lib/ui/store-application/StoreApplication.svelte
- web/src/lib/ui/store-settings/StoreSettingsNavigation.css
- web/src/lib/ui/store-settings/StoreSettingsNavigation.svelte
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

[]
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-UI-031 details 2](details-02.md)
- [E-UI-031 details 3](details-03.md)
