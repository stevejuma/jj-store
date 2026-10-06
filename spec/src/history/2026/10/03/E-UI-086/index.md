---
schema: 1
id: E-UI-086
---

# E-UI-086

<!-- evidence-field:id markdown -->
## Id

E-UI-086
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

Shared palette popup surfaces and softer workspace search scope action
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

Enclosing Jujutsu change nnrtvymn
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-002
- JST-UI-018
- JST-ARCH-004
- JST-GOV-002
- JST-GOV-003
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

- Visual baselines remain visual-pending. M11.4 remains In progress and M16
  Partial; existing release and registry qualification stays separate. No
  milestone promotion.
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Reuse the Appearance workspace nnrtvymn above osvzspxk/211f4205 and existing Vite7460/backend7462. Live Buzz Dialog was white while its inner Command View used the generated grey popover role, rgb235,235,235. Section-search markup omitted its existing scope-action class; unselected row was transparent. Community inspected first and released Design Core Command View public tokens/selectors second. Exact acceptance: generated popup/app surfaces match across62 palettes; scope action has a softer resting tint and stronger native hover and keyboard selection in light and dark.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

One UI appearance/search presentation surface with direct popup/search consumers. The owned adapter maps generated popover background to app background for all62 Buzz-derived palettes; regenerate62CSS files and retain historical Original light/dark snapshots. Restore owned section-action class, use30% muted over background at rest and55% for hover/keyboard through the public selected-background token; match foundation data-selected and aria-selected states. Source foreground, hover, border, focus, accent, syntax and other palette roles remain unchanged. Search behavior, native authority, APIs, routes, credentials and preferences remain unchanged. No new source copies or sibling imports. Rust checks/builds, root aggregates, backend or UI restart/reset, native writes, production built-CSP qualification, dependency publication, main landing and visual approval are out of scope under the targeted UI policy.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-appearance/AppearanceOverlay.stories.svelte
- catalog/fixtures/AppearanceOverlayFixture.svelte
- docs/reference/appearance-provenance.md
- spec/src/api.md
- spec/src/architecture.md
- spec/src/milestones.md
- spec/src/ui.md
- web/src/lib/appearance/adaptive-theme.ts
- web/src/lib/appearance/palettes/andromeeda.css
- web/src/lib/appearance/palettes/aurora-x.css
- web/src/lib/appearance/palettes/ayu-dark.css
- web/src/lib/appearance/palettes/buzz-dark.css
- web/src/lib/appearance/palettes/buzz.css
- web/src/lib/appearance/palettes/catppuccin-frappe.css
- web/src/lib/appearance/palettes/catppuccin-latte.css
- web/src/lib/appearance/palettes/catppuccin-macchiato.css
- web/src/lib/appearance/palettes/catppuccin-mocha.css
- web/src/lib/appearance/palettes/dark-plus.css
- web/src/lib/appearance/palettes/dracula-soft.css
- web/src/lib/appearance/palettes/dracula.css
- web/src/lib/appearance/palettes/everforest-dark.css
- web/src/lib/appearance/palettes/everforest-light.css
- web/src/lib/appearance/palettes/github-dark-default.css
- web/src/lib/appearance/palettes/github-dark-dimmed.css
- web/src/lib/appearance/palettes/github-dark-high-contrast.css
- web/src/lib/appearance/palettes/github-dark.css
- web/src/lib/appearance/palettes/github-light-default.css
- web/src/lib/appearance/palettes/github-light-high-contrast.css
- web/src/lib/appearance/palettes/github-light.css
- web/src/lib/appearance/palettes/gruvbox-dark-hard.css
- web/src/lib/appearance/palettes/gruvbox-dark-medium.css
- web/src/lib/appearance/palettes/gruvbox-dark-soft.css
- web/src/lib/appearance/palettes/gruvbox-light-hard.css
- web/src/lib/appearance/palettes/gruvbox-light-medium.css
- web/src/lib/appearance/palettes/gruvbox-light-soft.css
- web/src/lib/appearance/palettes/houston.css
- web/src/lib/appearance/palettes/kanagawa-dragon.css
- web/src/lib/appearance/palettes/kanagawa-lotus.css
- web/src/lib/appearance/palettes/kanagawa-wave.css
- web/src/lib/appearance/palettes/laserwave.css
- web/src/lib/appearance/palettes/light-plus.css
- web/src/lib/appearance/palettes/material-theme-darker.css
- web/src/lib/appearance/palettes/material-theme-lighter.css
- web/src/lib/appearance/palettes/material-theme-ocean.css
- web/src/lib/appearance/palettes/material-theme-palenight.css
- web/src/lib/appearance/palettes/material-theme.css
- web/src/lib/appearance/palettes/min-dark.css
- web/src/lib/appearance/palettes/min-light.css
- web/src/lib/appearance/palettes/monokai.css
- web/src/lib/appearance/palettes/night-owl.css
- web/src/lib/appearance/palettes/nord.css
- web/src/lib/appearance/palettes/one-dark-pro.css
- web/src/lib/appearance/palettes/one-light.css
- web/src/lib/appearance/palettes/plastic.css
- web/src/lib/appearance/palettes/poimandres.css
- web/src/lib/appearance/palettes/red.css
- web/src/lib/appearance/palettes/rose-pine-dawn.css
- web/src/lib/appearance/palettes/rose-pine-moon.css
- web/src/lib/appearance/palettes/rose-pine.css
- web/src/lib/appearance/palettes/slack-dark.css
- web/src/lib/appearance/palettes/slack-ochin.css
- web/src/lib/appearance/palettes/snazzy-light.css
- web/src/lib/appearance/palettes/solarized-dark.css
- web/src/lib/appearance/palettes/solarized-light.css
- web/src/lib/appearance/palettes/synthwave-84.css
- web/src/lib/appearance/palettes/tokyo-night.css
- web/src/lib/appearance/palettes/vesper.css
- web/src/lib/appearance/palettes/vitesse-black.css
- web/src/lib/appearance/palettes/vitesse-dark.css
- web/src/lib/appearance/palettes/vitesse-light.css
- web/src/lib/ui/community-search-command/CommunitySearchCommand.svelte
- web/src/lib/ui/community-search-command/search-command.css
- web/tests/vite/appearance.test.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

[]
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Node26.9.0; pnpm10.32.1; Svelte5.57.1; Vitest4.1.11; Storybook10.5.7; Shiki4.0.2; Design Core ^0.2.0. Dependency manifests and lockfile unchanged.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No native, API, route or stored-preference format changes. jj-store.appearance.v1 unchanged; generated semantic CSS bytes change only popover backgrounds.
<!-- /evidence-field -->

<!-- evidence-field:artifacts markdown -->
## Artifacts

/tmp/jj-overlay-token-*.log; /tmp/jj-overlay-token-live-final.jpg
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Retain focused regressions and visual-pending in the committed Appearance workspace.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-UI-086 details 2](details-02.md)
