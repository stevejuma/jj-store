---
schema: 1
id: E-UI-022
---

# E-UI-022

<!-- evidence-field:id markdown -->
## Id

E-UI-022
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M4
- M4.8
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-28
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

implementation
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/community-projects/CommunityProjectsRepositoryDirectory.stories.svelte
- catalog/components/community-projects/CommunityProjectsRepositoryLogDiff.stories.svelte
- catalog/components/community-repository-branch-command/CommunityRepositoryBranchCommand.stories.svelte
- catalog/fixtures/DiffFixture.svelte
- scripts/dev-demo-repository.mjs
- spec/src/api.md
- spec/src/milestones.md
- spec/src/spec-governance.md
- spec/src/ui.md
- web/src/lib/controller/native-presentation.ts
- web/src/lib/ui/community-projects/CommunityProjectsRepositoryDirectory.svelte
- web/src/lib/ui/community-projects/CommunityProjectsRepositoryFiles.svelte
- web/src/lib/ui/community-projects/CommunityProjectsRepositoryLogDiff.svelte
- web/src/lib/ui/community-projects/repository-directory.css
- web/src/lib/ui/community-repository-branch-command/CommunityRepositoryBranchCommand.svelte
- web/src/lib/ui/community-repository-branch-command/CommunityRepositoryBranchEmpty.svelte
- web/tests/production-store-loaders.test.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/production-store-loaders.test.ts#empty working copies use the native
  no-description label
<!-- /evidence-field -->

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
pnpm exec tsx --test web/tests/production-store-loaders.test.ts
```

**Notes**

The focused loader regression returned No description for an empty native working-copy description instead of the required explicit label.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/community-repository-branch-command/CommunityRepositoryBranchCommand.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryDirectory.stories.svelte
```

**Notes**

The focused stories reproduced the inherited branch-card radius and LICENSE-first repository-document order; one existing interaction passed.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/community-repository-branch-command/CommunityRepositoryBranchCommand.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLogDiff.stories.svelte
```

**Notes**

The two new empty-state stories reproduced plain bookmark-empty text and an empty standalone diff body; four existing interactions passed.
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
pnpm exec tsx --test web/tests/production-store-loaders.test.ts
```

**Notes**

All eight focused presentation and paging tests pass, including the exact empty working-copy label.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/community-repository-branch-command/CommunityRepositoryBranchCommand.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryDirectory.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLogDiff.stories.svelte
```

**Notes**

All seven focused interaction and accessibility stories pass for square branch cards, matched title sizing, shadcn icon Empty states, README-first document tabs, selected styling, hover interaction and diff fallback. Visual baselines were skipped by user direction.
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
pnpm web:check; pnpm catalog:check; pnpm check:architecture; pnpm web:build
```

**Notes**

Production and catalog diagnostics report no warnings, architecture budgets pass across 196 modules and the production bundle builds with only the existing Design Core global-selector and bundle-size warnings.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm dev:restart
```

**Notes**

The LocalTrusted demo restarted with repository 96435f1d43aed5050d6edd20d84acb63, no credentials, persistent embedded RustFS, 11546 ms cold startup and 410 ms restart.
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
live in-app browser verification at http://127.0.0.1:7460/
```

**Notes**

The live working-copy fallback is exact; working-copy and branch titles both compute to 14px; the final bookmark card has a 1px divider and zero lower radii; README precedes LICENSE with a selected theme background, 6px radius and visible 2px underline.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:first && pnpm check:progress && DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm format:check
```

**Notes**

Same-change governance maps every protected file to the four updated canonical chapters, validates evidence bookkeeping and confirms Rust plus Prettier formatting without running unrelated aggregate suites.
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

<!-- evidence-field:versions markdown -->
## Versions

@lapismd/design-core 0.2.0; Svelte 5.57.1; Storybook 10.5.7; Vitest 4.1.11.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Empty native working-copy descriptions use an explicit presentation fallback. Repository-document order and branch/diff empty states remain client presentation. The demo seed now ends on a descriptionless working copy. Native IDs, OpenAPI, object, manifest, WAL, receipt, route and browser-layout formats are unchanged. Stories remain visual-pending and visual baselines were skipped by user direction.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


