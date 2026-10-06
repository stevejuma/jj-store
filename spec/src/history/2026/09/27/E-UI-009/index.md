---
schema: 1
id: E-UI-009
---

# E-UI-009

<!-- evidence-field:id markdown -->
## Id

E-UI-009
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M4.8
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-27
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

implementation
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/community-projects/CommunityProjectsRevisionGraphIds.stories.svelte
- catalog/components/community-projects/revision-id.test.ts
- scripts/catalog-browser.test.mjs
- scripts/catalog-split-copy.mjs
- spec/src/api.md
- spec/src/milestones.md
- spec/src/spec-governance.md
- spec/src/ui.md
- web/src/lib/ui/community-projects/CommunityProjectsRevisionGraph.svelte
- web/src/lib/ui/community-projects/CommunityProjectsRevisionGraphIds.svelte
- web/src/lib/ui/community-projects/revision-graph-ids.css
- web/src/lib/ui/community-projects/revision-id.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- catalog/components/community-projects/revision-id.test.ts#abbreviates native
  identifiers with a prefix and final two characters
- scripts/catalog-browser.test.mjs#cold_catalog_contract
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/community-projects/CommunityProjectsRevisionGraphIds.stories.svelte
```

**Notes**

Before change: the two existing identity stories passed but did not cover the requested split-control behavior.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 7
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/community-projects/CommunityProjectsRevisionGraphIds.stories.svelte
```

**Notes**

New acceptance scenarios fail against the original component.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/community-projects/CommunityProjectsRevisionGraphIds.stories.svelte
```

**Notes**

Synthetic Storybook hover did not activate CSS hover. Keyboard assertions remain in the stories; real pointer checks were added to cold browser acceptance.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/community-projects/CommunityProjectsRevisionGraphIds.stories.svelte
```

**Notes**

Nine focused interaction/accessibility stories pass, including full copy values, independent feedback, failure retry, keyboard input and optional identifiers.
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
pnpm catalog:test
```

**Notes**

All 23 catalog unit tests pass, including three abbreviation regressions.
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
pnpm web:check; pnpm catalog:check; pnpm check:architecture
```

**Notes**

No Svelte errors or warnings; catalog and 109-module architecture checks pass.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm catalog:test:browser
```

**Notes**

Cold build succeeds; cold_catalog_contract finds row 2 bookmark overlap with the wider identifiers before reaching the new native pointer/touch checks.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Fix graph bookmark overlap and complete native pointer/touch/reduced-motion
  acceptance, full check/test/build/spec and formatting lanes. Stories remain
  visual-pending.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Existing registry Design Core ^0.2.0, Svelte 5.57.1, Storybook 10.5.7, Vitest 4.1.11. Enclosing Jujutsu change is the revision reference.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Presentation-only prefix/suffix abbreviation and fixed-width split-copy interaction. Complete underlying clipboard identities retained. No service, schema, storage or credential format changes. User requested a checkpoint commit before continuing validation.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


