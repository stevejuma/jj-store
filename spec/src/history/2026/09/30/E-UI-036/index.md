---
schema: 1
id: E-UI-036
---

# E-UI-036

<!-- evidence-field:id markdown -->
## Id

E-UI-036
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M4.23
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-30
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

commit-selector-alignment-and-navigation-spacing
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

Enclosing Jujutsu change opokxnpq.
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

New Parent commit alignment scenario failed with a 5.5px avatar/body midpoint difference; existing navigation story failed with 20px section top margin. Three remaining navigation scenarios passed. Identifier chips used inline square shapes before the requested smaller oval styling.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- web/src/lib/ui/community-navigation/CommunityNavigation.css
- web/src/lib/ui/store-mutations/StoreMutationCommitTarget.svelte
- catalog/fixtures/StoreMutationCommitPickerFixture.svelte
- catalog/components/store-mutations/StoreMutationCommitPicker.stories.svelte
- catalog/components/store-application/StoreRepositoryNavigation.stories.svelte
- spec/src/api.md
- spec/src/ui.md
- spec/src/milestones.md
- spec/src/verification.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

[]
<!-- /evidence-field -->

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-mutations/StoreMutationCommitPicker.stories.svelte catalog/components/store-application/StoreRepositoryNavigation.stories.svelte
```

**Notes**

Two requested style regressions fail before production changes; three existing navigation cases pass. Ignored .scratch/commit-picker-before.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-mutations/StoreMutationCommitPicker.stories.svelte catalog/components/store-application/StoreRepositoryNavigation.stories.svelte catalog/components/store-mutations/StoreRepositoryActions.stories.svelte
```

**Notes**

9 focused Chromium interaction/accessibility checks pass. Exact before regressions pass: empty-description avatar/body/chevron alignment, compact oval chips in trigger and rich options, navigation margin. Existing mutation preview/apply, edit context, confirmation and unknown-outcome interactions remain passing. Stories retain visual-pending.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:check
```

**Notes**

Zero errors and warnings.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm catalog:check
```

**Notes**

Zero errors/warnings; catalog boundaries pass across 75 modules.
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
pnpm check:architecture
```

**Notes**

288 modules pass ownership and no-growth checks.
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
pnpm web:build
```

**Notes**

Built UI passes; existing nonfatal Design Core minification and chunk warnings remain. No Rust build.
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
pnpm exec prettier --write web/src/lib/ui/community-navigation/CommunityNavigation.css web/src/lib/ui/store-mutations/StoreMutationCommitTarget.svelte catalog/fixtures/StoreMutationCommitPickerFixture.svelte catalog/components/store-mutations/StoreMutationCommitPicker.stories.svelte catalog/components/store-application/StoreRepositoryNavigation.stories.svelte spec/src/api.md spec/src/ui.md
```

**Notes**

Changed source, regression and canonical chapter files formatted in focused subsets.
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
node .scratch/restart-workspace-ui.mjs
```

**Notes**

Owned demo restart loads rebuilt assets with existing explicit binaries and preserved repository data. PID 15182 ready at 127.0.0.1:7460. No reset/reseed or unrelated service restart.
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
CUA live built-UI verification
```

**Notes**

Separate hidden preview preserves the user dialog. Live navigation margin is 0px; descriptionless commit avatar/body/chevron midpoints differ by under 0.01px; selected GIT/JJ chips are 16px high with 26px radius. Options reuse the same chips. Saved ignored .scratch/commit-picker-styling.png; no mutation preview or apply invoked.
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
pnpm spec:validate
```

**Notes**

4 validators, 18 chapters, 107 requirements pass.
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
pnpm spec:first
```

**Notes**

5 protected files, 3 canonical chapters pass same-change governance.
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
pnpm spec:build
```

**Notes**

Canonical mdBook HTML builds.
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
```

**Notes**

76 tasks pass evidence bookkeeping only.
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
pnpm exec prettier --write spec/src/milestones.md spec/src/verification.md
```

**Notes**

Canonical evidence files formatted.
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
pnpm exec prettier --check web/src/lib/ui/community-navigation/CommunityNavigation.css web/src/lib/ui/store-mutations/StoreMutationCommitTarget.svelte catalog/fixtures/StoreMutationCommitPickerFixture.svelte catalog/components/store-mutations/StoreMutationCommitPicker.stories.svelte catalog/components/store-application/StoreRepositoryNavigation.stories.svelte spec/src/api.md spec/src/ui.md spec/src/milestones.md spec/src/verification.md
```

**Notes**

All changed files pass formatting.
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
<!-- generated-navigation -->
## Pages

- [E-UI-036 details 2](details-02.md)
