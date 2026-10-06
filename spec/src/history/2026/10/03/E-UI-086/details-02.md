---
schema: 1
id: E-UI-086
---

# E-UI-086 details 2

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
jj --no-pager st; inspect owning chapters, milestone ledger, Community first and released Design Core second; read-only live CUA inspection
```

**Notes**

Shared popover role and absent scope-action class explain both requested behaviors; existing workspace/backend/fixture retained.
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
pnpm exec prettier --write catalog/fixtures/AppearanceOverlayFixture.svelte catalog/components/store-appearance/AppearanceOverlay.stories.svelte (initial)
```

**Notes**

Initial overlay regression formatting. /tmp/jj-overlay-token-format-before.log.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm catalog:test web/tests/vite/appearance.test.ts (before)
```

**Notes**

3 new white/GitHub dark/Dracula app-surface assertions fail;8existing tests pass. /tmp/jj-overlay-token-unit-before.log.
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
pnpm test:storybook catalog/components/store-appearance/AppearanceOverlay.stories.svelte (before shared-token fix)
```

**Notes**

Expected white app surface, received grey rgb235 popup. /tmp/jj-overlay-token-story-before.log.
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
pnpm exec tsx scripts/appearance-palettes.mts
```

**Notes**

Regenerated62Buzz-derived palette CSS files after shared adapter change. /tmp/jj-overlay-token-generate.log.
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
pnpm exec prettier --write (adapter, regression, provenance and canonical chapter files)
```

**Notes**

Shared-token implementation formatted. /tmp/jj-overlay-token-format-after.log.
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
pnpm catalog:test web/tests/vite/appearance.test.ts (after)
```

**Notes**

11/11 exact appearance tests. /tmp/jj-overlay-token-unit-after.log.
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
pnpm test:storybook catalog/components/store-appearance/AppearanceOverlay.stories.svelte (after shared-token fix)
```

**Notes**

1/1 fresh browser story checks Dialog/Command/Menu equality for all62generated palettes, retaining query and result. /tmp/jj-overlay-token-story-after.log.
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
pnpm exec tsx scripts/appearance-palettes.mts --check
```

**Notes**

Exact source-derived generated asset comparison. /tmp/jj-overlay-token-palette-check.log.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:check; pnpm catalog:check; pnpm check:architecture; pnpm check:ui-ownership (shared-token fix)
```

**Notes**

Web/catalog0errors0warnings;153catalog modules;798architecture modules;ownership1validator/21chapters/147requirements. Logs web/catalog/architecture/ownership.
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
pnpm exec prettier --write catalog/fixtures/AppearanceOverlayFixture.svelte catalog/components/store-appearance/AppearanceOverlay.stories.svelte (scope regression)
```

**Notes**

Light/dark native-pointer and keyboard regressions formatted. /tmp/jj-overlay-token-scope-format-before.log.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-appearance/AppearanceOverlay.stories.svelte (before scope-row fix)
```

**Notes**

2/2 new scope rows are transparent at rest;1all-palette overlay story passes. /tmp/jj-overlay-token-scope-before.log.
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
pnpm exec prettier --write web/src/lib/ui/community-search-command/CommunitySearchCommand.svelte web/src/lib/ui/community-search-command/search-command.css spec/src/ui.md
```

**Notes**

Owned class, theme-derived resting/selected token and supported selected selectors formatted. /tmp/jj-overlay-token-scope-format-after.log.
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
pnpm test:storybook catalog/components/store-appearance/AppearanceOverlay.stories.svelte (after scope-row fix)
```

**Notes**

3/3 in fresh browser: light/dark softer resting, real CSS hover, keyboard selection/reset; all62generated popup palettes. /tmp/jj-overlay-token-scope-after.log.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:check; pnpm catalog:check; pnpm check:architecture; pnpm check:ui-ownership (final production)
```

**Notes**

Web/catalog0errors0warnings;153catalog modules;798architecture modules with unchanged budgets;ownership1validator/21chapters/147requirements. /tmp/jj-overlay-token-{web,catalog,architecture,ownership}-final.log.
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
Read-only CUA live HMR workspace search verification and screenshot
```

**Notes**

White Dialog/Command in Buzz. Scope action selected rgb246.7 equivalent, resting rgb250.5 equivalent: color(srgb0.96755...) vs color(srgb0.9823...). Saved final full-viewport screenshot with resting row; closed temporary tab32. User tab untouched; no preferences/data writes or restart.
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-search catalog/components/store-appearance catalog/components/store-application/StoreRepositorySearch.stories.svelte catalog/components/store-application/StoreRepositoryPicker.stories.svelte catalog/components/store-application/StoreComparisonParentPicker.stories.svelte catalog/components/store-mutations/StoreMutationCommitPicker.stories.svelte catalog/components/store-threads/StoreThreadSearch.stories.svelte catalog/components/store-threads/StoreCapturedReactions.stories.svelte
```

**Notes**

35/35 across13direct overlay/search/picker/reaction consumer files in a fresh browser/Vite process. /tmp/jj-overlay-token-consumers.log.
<!-- /run -->

<!-- run -->
### Run 18

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
python3 inline milestone evidence update; pnpm exec prettier --write spec/src/milestones.md; pnpm spec:first; pnpm spec:validate; pnpm check:progress
```

**Notes**

E-UI-086 preserves milestone statuses. 69protected files/4canonical chapters;5validators/21chapters/147requirements;120tasks;73changed files formatted. Final evidence insertion and changed-file formatting checked again before commit. /tmp/jj-overlay-token-{spec-first,spec-validate,progress,format-final,progress-final}.log.
<!-- /run -->
<!-- /evidence-field -->

