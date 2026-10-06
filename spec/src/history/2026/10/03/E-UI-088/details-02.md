---
schema: 1
id: E-UI-088
---

# E-UI-088 details 2

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
jj --no-pager st; inspect CommunityAppShell.svelte and CommunityNavigation.css in Community; inspect Design Core button.svelte/button.tokens.ts; inspect JST-UI-002/JST-UI-018 and existing Appearance shell fixtures
```

**Notes**

Clean Appearance working copy after separately committing tooltip source quoystmy/bb40d39d and consumer evidence wovknnpw/e61af022. Existing copied-source provenance remains unchanged; header/compact search styling is app-owned.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --write catalog/components/store-appearance/AppearanceShell.stories.svelte; pnpm test:storybook catalog/components/store-appearance/AppearanceShell.stories.svelte (initial before)
```

**Notes**

6existing stories pass. New4fail:3repository fixtures have no inactive compact group and1hover mismatch. Calibrate reference/sample selection and wait for theme paint rather than reading an in-flight transition. /tmp/jj-chrome-hover-before.log.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-appearance/AppearanceShell.stories.svelte (settled reference and fallback iterations)
```

**Notes**

Each attempt6pass/4fail;3missing-reference errors and1actual mismatch. Logs before-settled/before-final. No production changes yet.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --write catalog/fixtures/AppearanceShellFixture.svelte catalog/components/store-appearance/AppearanceShell.stories.svelte; pnpm test:storybook catalog/components/store-appearance/AppearanceShell.stories.svelte (qualified before)
```

**Notes**

6pass/4real computed-style failures after adding a theme-derived sample when sidebar groups are already active. Header opaque oklab differs from sidebar foreground/0.07 in light and dark. /tmp/jj-chrome-hover-before-qualified.log.
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
pnpm exec prettier --write web/src/lib/ui/community-app-shell/CommunityAppShell.svelte web/src/lib/ui/community-navigation/CommunityNavigation.css catalog/components/store-appearance/AppearanceShell.stories.svelte catalog/fixtures/AppearanceShellFixture.svelte spec/src/ui.md; pnpm test:storybook catalog/components/store-appearance/AppearanceShell.stories.svelte (after)
```

**Notes**

10/10 in a fresh browser: repository/project Buzz, Buzz dark and neutral hover/focus plus existing gradient/picker coverage. Logs format/after.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:check; pnpm catalog:check; pnpm check:architecture; pnpm check:ui-ownership (initial impact checks)
```

**Notes**

Web0errors0warnings; architecture798modules and ownership pass. Catalog2type errors: Testing Library does not accept Playwright exact option. Replace with anchored accessible-name regex; runtime assertions remain identical. Logs web/catalog/architecture/ownership.
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
pnpm exec prettier --write catalog/components/store-appearance/AppearanceShell.stories.svelte; pnpm catalog:check; pnpm test:storybook catalog/components/store-appearance/AppearanceShell.stories.svelte (after story typing fix)
```

**Notes**

Catalog0errors0warnings/153modules; exact10/10 again. Concurrent test process selected a free fallback port; neither unrelated service was stopped. Logs story-format-final/catalog-final/after-final.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-work/DisclosurePersistence.stories.svelte catalog/components/store-application/StoreApplication.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte catalog/components/store-application/StoreRepositoryNavigation.stories.svelte catalog/components/store-threads/StoreThreadNavigation.stories.svelte catalog/components/store-work/ProjectsHub.stories.svelte
```

**Notes**

43pass/1failure across6files. The resting-toggle helper retained keyboard focus after a synthetic click, so received the new focus tint. Preserve transparency assertion, clear native hover/focus and assert true resting state; separate new stories prove keyboard focus tint. Five other files pass all38tests. /tmp/jj-chrome-hover-consumers.log.
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
Read-only CUA temporary tab34: native-pointer hover for inactive Projects reference and all marked controls, navigate Branches/Back to enable both history buttons, compare live colors, inspect open menu trigger
```

**Notes**

All7marked/header/history/search controls equal sidebar color(srgb0.1413580.1610.180642/0.07), foreground rgb36,41,46. Initial compact-selector lookup had no inactive groups; used the actual inactive Projects button. Menu-open before fix: rgb240,240,240 with pointer in menu; after fix: identical0.07tint while not hovered. Screenshots saved. Temporary tab closed; user tab, preferences and native data untouched; no restart.
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
pnpm exec prettier --write web/src/lib/ui/community-app-shell/CommunityAppShell.svelte catalog/components/store-appearance/AppearanceShell.stories.svelte catalog/components/store-work/DisclosurePersistence.stories.svelte spec/src/ui.md; pnpm test:storybook catalog/components/store-appearance/AppearanceShell.stories.svelte catalog/components/store-work/DisclosurePersistence.stories.svelte (final menu/resting checks)
```

**Notes**

16/16 across2files, including light/dark open-menu paint after native pointer leaves the trigger, Escape clearing and transparent toggles in both expansion states. Together with38passing direct-consumer stories:54qualified stories across7files, not a full-suite claim. /tmp/jj-chrome-hover-final-stories.log.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:check; pnpm catalog:check; pnpm check:architecture; pnpm check:ui-ownership (after menu-state addition)
```

**Notes**

Web/catalog0errors0warnings,153catalog modules; ownership passes. Architecture reports shell301counted lines against300limit. Consolidate equivalent open/focus selectors with :is rather than changing the limit. Logs web-final/catalog-qualified/architecture-final/ownership-final.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --write spec/src/milestones.md; pnpm spec:first; pnpm spec:validate; pnpm check:progress; pnpm exec prettier --check web/src/lib/ui/community-app-shell/CommunityAppShell.svelte web/src/lib/ui/community-navigation/CommunityNavigation.css catalog/components/store-appearance/AppearanceShell.stories.svelte catalog/components/store-work/DisclosurePersistence.stories.svelte catalog/fixtures/AppearanceShellFixture.svelte spec/src/ui.md spec/src/milestones.md (initial final gates)
```

**Notes**

Formatting,5validators/21chapters/147requirements and120task bookkeeping pass. Spec-first requires api.md under the existing broad web path map. Add controller-contract preservation note in that canonical chapter. Logs ledger-format/spec-first/spec-validate/progress/format-final.
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
pnpm exec prettier --write web/src/lib/ui/community-app-shell/CommunityAppShell.svelte spec/src/api.md; pnpm test:storybook catalog/components/store-appearance/AppearanceShell.stories.svelte catalog/components/store-work/DisclosurePersistence.stories.svelte (qualified final)
```

**Notes**

16/16 across2files after consolidating the equivalent open/focus selector with :is. Fresh browser152.84s; no assertion, timeout or baseline weakening. Full direct impact remains54stories across7files including the earlier38passing consumer tests. Logs gates-format/qualified-stories.
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
pnpm exec prettier --write spec/src/milestones.md; pnpm check:architecture; pnpm web:check; pnpm spec:first; pnpm spec:validate; pnpm check:progress; pnpm exec prettier --check web/src/lib/ui/community-app-shell/CommunityAppShell.svelte web/src/lib/ui/community-navigation/CommunityNavigation.css catalog/components/store-appearance/AppearanceShell.stories.svelte catalog/components/store-work/DisclosurePersistence.stories.svelte catalog/fixtures/AppearanceShellFixture.svelte spec/src/api.md spec/src/ui.md spec/src/milestones.md (qualified final)
```

**Notes**

Architecture798modules with unchanged300line limit; web0errors0warnings. Existing final catalog check0errors0warnings/153modules and ownership pass. Spec-first5protected files/3canonical chapters;5validators/21chapters/147requirements;120tasks; all8changed files formatted. Logs ledger-format-final/architecture-qualified/web-qualified/spec-first-final/spec-validate-final/progress-final/format-qualified. Final evidence insertion is formatted and ledger gates repeated before commit. Wider root aggregates/Rust and further Design Core visual tests remain out of scope.
<!-- /run -->
<!-- /evidence-field -->

