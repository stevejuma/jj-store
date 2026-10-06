---
schema: 1
id: E-UI-085
---

# E-UI-085

<!-- evidence-field:id markdown -->
## Id

E-UI-085
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M9.5
- M11.4
- M16
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-02
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Continuous Buzz chrome and compact navigation popup surfaces
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

Enclosing Jujutsu change osvzspxk
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

- Visual baselines remain visual-pending. No milestone promotion. Existing M16
  release/registry qualification remains separate. Rust builds/tests, root
  aggregates, production bundle builds, service rebuild/reset/restart,
  dependency changes, main landing and publication are out of scope under the
  targeted UI policy.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Reuse the appearance workspace and existing Vite HMR. Inspect Community shell/navigation first and released Design Core Popover/Command View public tokens/props second. Live light Buzz header repeated the whole yellow-to-blue gradient within40px; compact picker used235grey and its left edge touched the48px rail. Paint one gradient across the viewport-height app shell; transparent Buzz light/dark chrome reveals it while glass keeps its tint/opacity/blur. Other theme chrome and generated palette assets stay unchanged. Set both compact section-picker public background tokens to the resolved app background, white in light Buzz and opaque with glass enabled. Increase right sideOffset to16px to leave8px beyond the rail, retaining8px collision padding. Native navigation, search, focus restoration and authorization remain unchanged. One owned UI surface includes Projects/repository shell and compact-menu consumers; no foundation patches, sibling imports or new provenance source copies.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-appearance/AppearanceShell.stories.svelte
- catalog/fixtures/AppearanceShellFixture.svelte
- docs/reference/appearance-provenance.md
- spec/src/api.md
- spec/src/architecture.md
- spec/src/milestones.md
- spec/src/ui.md
- web/src/lib/appearance/appearance.css
- web/src/lib/ui/community-navigation/CommunityNavigation.css
- web/src/lib/ui/community-navigation/CommunityNavigation.svelte
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

[]
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Node26.9.0; pnpm10.32.1; Svelte5.57.1; Vitest4.1.11; Storybook10.5.7; Design Core ^0.2.0. Dependencies, generated Shiki4.0.2 palette bytes, stored preferences and owned Community source provenance unchanged.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No native/API/route or persistence format changes. jj-store.appearance.v1 fields and glass preference remain unchanged; presentation-only public token bindings and Popover spacing.
<!-- /evidence-field -->

<!-- evidence-field:artifacts markdown -->
## Artifacts

/tmp/jj-shell-chrome-*.log; /tmp/jj-shell-chrome-live.jpg
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Retain visual-pending and the committed appearance workspace for UI iteration.
<!-- /evidence-field -->

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
jj --no-pager st; inspect owning UI/API/architecture/governance and milestone chapters; inspect Community first and released Design Core second; read-only CUA shell/picker inspection
```

**Notes**

Clean osvzspxk above tlnzznzu/19446858. Narrow acceptance is continuous Buzz chrome plus themed opaque compact menus with8px rail clearance. Live before: header linear gradient; popup and Command View rgb235,235,235; popup left48px equals rail right48px, gap0; trigger gap8px.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --write catalog/fixtures/AppearanceShellFixture.svelte catalog/components/store-appearance/AppearanceShell.stories.svelte
```

**Notes**

Initial regression/fixture formatting. /tmp/jj-shell-chrome-format.log.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 6
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-appearance/AppearanceShell.stories.svelte (before)
```

**Notes**

2header-gradient assertions and2rail-clearance assertions fail at the requested behavior.2Projects cases also expose an expanded-fixture assumption, so their absent compact input is fixture setup evidence. /tmp/jj-shell-chrome-before.log.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm catalog:check (initial)
```

**Notes**

Testing Library ByRoleOptions does not accept the browser-style exact property. Removed it in favor of actual rendered compact triggers. /tmp/jj-shell-chrome-catalog.log.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --write (3 production files, UI chapter and appearance provenance)
```

**Notes**

App-owned public token, viewport gradient and16px Popover offset changes formatted. /tmp/jj-shell-chrome-format-after.log.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-appearance/AppearanceShell.stories.svelte (after production fix)
```

**Notes**

2header stories passed.4picker cases reached fixture assumptions: Projects starts expanded; repository fixtures omit Your projects. Helper now collapses the rail and verifies every rendered compact section trigger, with unchanged geometry and colour assertions. /tmp/jj-shell-chrome-after.log.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --write catalog/components/store-appearance/AppearanceShell.stories.svelte; pnpm test:storybook catalog/components/store-appearance/AppearanceShell.stories.svelte (qualified)
```

**Notes**

6/6 in fresh browser/Vite process: Buzz light/dark continuous gradient; all rendered compact pickers in Projects/repository scope; app/Command backgrounds,8px rail/viewport clearance and Escape focus under Buzz, dark glass, GitHub Light Default and Original. /tmp/jj-shell-chrome-format-story.log; /tmp/jj-shell-chrome-qualified.log.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:check; pnpm catalog:check; pnpm check:architecture; pnpm check:ui-ownership
```

**Notes**

Both type checks0errors0warnings;151catalog modules;796architecture modules with unchanged budgets; ownership1validator/21chapters/147requirements. /tmp/jj-shell-chrome-{web,catalog-qualified,architecture,ownership}.log.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-application/StoreApplication.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte catalog/components/store-application/StoreRepositoryNavigation.stories.svelte catalog/components/store-threads/StoreThreadNavigation.stories.svelte catalog/components/store-work/ProjectsHub.stories.svelte catalog/components/store-application/AppearanceDiscussions.stories.svelte catalog/components/store-appearance/StoreAppearance.stories.svelte
```

**Notes**

43/43 across7directly affected consumer files in a fresh browser/Vite process. Shell desktop/mobile, compact selection/search, Projects, repository/thread navigation and Appearance settings/discussion behavior. /tmp/jj-shell-chrome-consumers.log.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first (initial)
```

**Notes**

Missing mapped API and architecture chapter updates for appearance presentation CSS. Added the owned CSS/unchanged preference-contract boundary and compact-menu exception to the existing appearance chapters. /tmp/jj-shell-chrome-spec-first.log.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
Read-only CUA live HMR verification in temporary background Projects tab
```

**Notes**

Header backgroundImage none and backgroundColor transparent, full-height shell gradient retained. Popup left56px against48px rail, gap8px; Popover and Command View both rgb255,255,255. Saved and inspected full viewport screenshot, then closed temporary tab. User tab untouched; no data writes, preference changes or service restart.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --write spec/src/api.md spec/src/architecture.md spec/src/ui.md spec/src/milestones.md; pnpm spec:first; pnpm spec:validate; pnpm check:progress; pnpm exec prettier --check (10 changed files)
```

**Notes**

Final evidence checks:6protected files/4canonical chapters;5validators/21chapters/147requirements;120tasks;all10changed files formatted. /tmp/jj-shell-chrome-{ledger-format,spec-first-qualified,spec-validate,progress,format-check}.log. This evidence insertion is checked again before the standing JJ commit.
<!-- /run -->
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


