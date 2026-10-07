---
schema: 1
id: E-UI-107
---

# E-UI-107 details 2

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-search/StoreRecentSearch.stories.svelte -t TypedSearchAndActionIcons
```

**Notes**

Before production change: typed action had no composite icon. Existing rows used generic Search/Plus.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-search/StoreRecentSearch.stories.svelte -t TypedSearchAndActionIcons
```

**Notes**

First production pass rendered correct glyphs but both SVGs were 22.390625px. Scoped the public Command View --ui-command-view-icon-size token independently to main and badge, preserving Community geometry rather than changing foundational components.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-search/StoreRecentSearch.stories.svelte -t TypedSearchAndActionIcons
```

**Notes**

Geometry passed; new story keyboard sequence assumed pointer selection wraps to the first row. Corrected the test to exercise keyboard before pointer callbacks.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-search/StoreRecentSearch.stories.svelte -t TypedSearchAndActionIcons
```

**Notes**

First keyboard row was already selected by Command View; ArrowDown selected a different row. Use explicit End/Home selection, assert aria-selected and then Enter. Production keyboard code unchanged.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-search/StoreRecentSearch.stories.svelte catalog/components/store-search/StoreSearchCommand.stories.svelte catalog/components/store-application/StoreRepositorySearch.stories.svelte catalog/components/store-appearance/AppearanceOverlay.stories.svelte
```

**Notes**

Initial direct-consumer run used the earlier keyboard story: eight pass, one fail. Final rerun required after the exact keyboard correction.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-search/StoreRecentSearch.stories.svelte -t TypedSearchAndActionIcons
```

**Notes**

Exact typed-icon story passes: main glyphs for every section/action, smaller bottom-right badges, hidden decoration, exact pointer callbacks, End/Home/Enter selection and refocused input.
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
pnpm web:check
pnpm catalog:check
```

**Notes**

Zero web/catalog diagnostics; catalog boundary covers 276 modules.
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
pnpm check:ui-ownership
pnpm check:architecture
```

**Notes**

Owned Community source passes; architecture covers 1273 modules. Existing pinned Apache-2.0 provenance already includes the reused browse-icon style.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-search/StoreRecentSearch.stories.svelte catalog/components/store-search/StoreSearchCommand.stories.svelte catalog/components/store-application/StoreRepositorySearch.stories.svelte catalog/components/store-appearance/AppearanceOverlay.stories.svelte
```

**Notes**

Final complete direct-consumer lane: nine stories in four families pass with blocking accessibility. Covers typed icons, geometry, scope keyboard/pointer activation, repository navigation and appearance surfaces across generated palettes.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
node .local/search-icons-browser.mjs
```

**Notes**

Read-only acceptance observer initially classified the existing POST local work/browse query as a mutation. Glyph geometry, scope focus and light axe had passed. Corrected only the observer to admit this exact read endpoint; no product mutation occurred. CUA tab lookup was stale and its follow-up timed out, so current-UI checks use isolated browser contexts.
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
node .local/search-icons-browser.mjs
```

**Notes**

Fresh isolated light/dark contexts against the existing HMR app pass every search/browse entity glyph and smaller lower-right badge, scoped-input focus, Escape, zero dialog axe violations and zero unexpected commands/page errors. Existing read-only work/browse POST remains admitted. Screenshots inspected; no fixture reset, backend restart, user-tab change or baseline approval.
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
pnpm exec prettier --write catalog/components/store-search/StoreRecentSearch.stories.svelte spec/src/current/contracts/api/03-captured-application-content-search.md spec/src/current/contracts/ui/02-repository-owned-community-presentation.md spec/src/current/progress/M15/history/M15.5/index.md spec/src/current/progress/M15/index.md spec/src/history/2026/10/07/E-UI-107/SUMMARY.nav.md spec/src/history/2026/10/07/E-UI-107/index.md spec/src/history/2026/10/07/SUMMARY.nav.md spec/src/history/2026/10/07/index.md web/src/lib/ui/community-search-command/CommunitySearchCommand.svelte web/src/lib/ui/community-search-command/model.ts web/src/lib/ui/community-search-command/search-command.css web/src/lib/ui/store-search/StoreSearchCommand.svelte web/src/lib/ui/store-search/StoreSearchOverlay.svelte
```

**Notes**

Scoped formatter passes for all changed source/document files; only owning milestone tables/navigation formatting changed.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
pnpm spec:first
pnpm spec:validate
```

**Notes**

144 tasks and 645 evidence records; six protected files and two canonical chapters; seven validators, 3122 chapters and 178 requirements pass before final result bookkeeping. No milestone status promoted.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
pnpm spec:first
pnpm spec:validate
pnpm exec prettier --check catalog/components/store-search/StoreRecentSearch.stories.svelte spec/src/current/contracts/api/03-captured-application-content-search.md spec/src/current/contracts/ui/02-repository-owned-community-presentation.md spec/src/current/progress/M15/history/M15.5/index.md spec/src/current/progress/M15/index.md spec/src/history/2026/10/07/E-UI-107/SUMMARY.nav.md spec/src/history/2026/10/07/E-UI-107/details-02.md spec/src/history/2026/10/07/E-UI-107/index.md spec/src/history/2026/10/07/SUMMARY.nav.md spec/src/history/2026/10/07/index.md web/src/lib/ui/community-search-command/CommunitySearchCommand.svelte web/src/lib/ui/community-search-command/model.ts web/src/lib/ui/community-search-command/search-command.css web/src/lib/ui/store-search/StoreSearchCommand.svelte web/src/lib/ui/store-search/StoreSearchOverlay.svelte
```

**Notes**

Completed-result bookkeeping passes: 144 tasks/645 records, six protected files/two canonical chapters, seven validators/3123 chapters/178 requirements, scoped formatting and all changed canonical/evidence physical budgets. Exact current file coverage retained.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pass
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

[]
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Keep current HMR UI available for review; maintain exact label/callback semantics and separate visual-baseline approval.
<!-- /evidence-field -->

