---
schema: 1
id: E-UI-088
---

# E-UI-088

<!-- evidence-field:id markdown -->
## Id

E-UI-088
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M9.5
- M11.4
- M16
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-03
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Align shell header, collapsed search and open menu paint with sidebar hover
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

Enclosing Jujutsu change kltumumn
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-002
- JST-UI-018
- JST-GOV-002
- JST-GOV-003
- JST-GOV-005
- JST-GOV-007
- JST-GOV-008
- JST-GOV-009
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Stories retain visual-pending and existing milestone statuses remain
  unchanged; baseline approval remains separate. Further Design Core visual
  tests are excluded by the user.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

User requests matching left-sidebar hover for header details, snapshot refresh, repository actions, sidebar toggle, history controls and collapsed search; subsequent request includes the open kebab state. Inspect Community shell/navigation source first, then public Design Core Button styles/tokens. This is an owned presentation adaptation: replace the inherited opaque sidebar-accent header mix with the existing sidebar-item 7% foreground tint. Apply the same theme-relative paint to keyboard focus, collapsed search and open header menu triggers; preserve disabled controls, neutral resting toggle states, size, icons, routes and focus rings. No foundation source fix, palette rewrite or consumer dependency refresh is needed. Add native-pointer and open-menu interaction evidence in the existing Appearance shell stories and calibrate the private resting-state helper to blur and assert no hover/focus before checking transparency. Wider jj-store root aggregates, Rust, native writes, backend restart/reset, publication and baseline updates are out of scope under the targeted policy.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- web/src/lib/ui/community-app-shell/CommunityAppShell.svelte
- web/src/lib/ui/community-navigation/CommunityNavigation.css
- catalog/components/store-appearance/AppearanceShell.stories.svelte
- catalog/components/store-work/DisclosurePersistence.stories.svelte
- catalog/fixtures/AppearanceShellFixture.svelte
- spec/src/ui.md
- spec/src/milestones.md
- spec/src/api.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

[]
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Node26.9.0; pnpm10.32.1; consumer Svelte5.57.1, Storybook10.5.7 and Vitest4.1.11. Existing installed foundation source and portable registry manifests/lockfiles unchanged.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No native/API/route/preference/public-token format changes. CSS only scopes app chrome; existing jj-store.appearance.v1 and persistent fixture retained.
<!-- /evidence-field -->

<!-- evidence-field:artifacts markdown -->
## Artifacts

/tmp/jj-chrome-hover-*.log; /tmp/jj-chrome-hover-live-final.png; /tmp/jj-chrome-menu-open-live-final.png
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Retain focused interaction and accessibility evidence; no milestone promotion or additional Design Core visual test run.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-UI-088 details 2](details-02.md)
