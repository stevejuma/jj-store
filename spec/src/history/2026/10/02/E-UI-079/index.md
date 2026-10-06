---
schema: 1
id: E-UI-079
---

# E-UI-079

<!-- evidence-field:id markdown -->
## Id

E-UI-079
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M11.4
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-02
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Stable circular search controls and a single Activity filter
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

Enclosing Jujutsu change pynomxls
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-015
- JST-WORK-008
- JST-UI-001
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Visual baseline approval remains separate. App root aggregate lanes, Rust
  builds/tests, production built-CSP qualification and package publication are
  out of scope.
- Design Core broad qualification is tracked independently in its verification
  chapter; cold AI preview readiness and visual cache permissions required
  additional investigation. No release or visual approval is claimed.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Reused appearance workspace. Inspected Community Projects and Design Core InputGroup before editing. Leading Search/Close action remains mounted at the same position and 32px size across expansion, retaining its original outline and subtle background. Search changes to Close on hover/keyboard focus; the trailing 24px circular Clear exists only for nonempty input, clears both draft and submitted query, and keeps the input open/focused. Enter, Escape, route restoration and filter-popup draft retention remain. Activity combines scope and native category filters in one top-row popover. Source-first InputGroup size forwarding repair in sibling Design Core; consumed through the already governed local pnpm hook, with registry manifest/lockfile preserved. Restarted only owned Vite after package refresh; backend PID91193 and fixture data retained. No main landing, Rust restart, reseed or live mutation. Broader story execution exposed shared browser storage: a prior outer details rail made agent selection constrain and occlude the main region. ProjectsHubFixture now uses fresh per-instance memory storage by default and retains explicitly supplied storage. Original agent assertions are restored; no waits, assertions or production shell policy changed.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- web/src/lib/ui/store-work/StoreProjectsNavigation.svelte
- web/src/lib/ui/store-work/StoreProjectsFilterControls.svelte
- web/src/lib/ui/store-work/StoreActivityFilter.svelte
- catalog/components/store-work/ProjectsFilters.stories.svelte
- catalog/components/store-work/ProjectsActivityControls.stories.svelte
- catalog/fixtures/ProjectsHubFixture.svelte
- spec/src/ui.md
- spec/src/api.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

[]
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Svelte5.57.1;Vitest4.1.11;Storybook10.5.7;pnpm10.32.1;local Design Core0.1.2 sibling source;portable app dependency range remains^0.2.0.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Presentation-only. Existing bounded q and activity filters; no API/schema, authorization, native identity or persistence-format changes.
<!-- /evidence-field -->

<!-- evidence-field:artifacts markdown -->
## Artifacts

/tmp/jj-search-controls-*.log; /tmp/dc-input-group-*.log; /tmp/design-core-input-group-fix.tgz
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Retain visual-pending; follow source package qualification separately. Next requested slice: agent row spacing and Focus sheets.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-UI-079 details 2](details-02.md)
- [E-UI-079 details 3](details-03.md)
