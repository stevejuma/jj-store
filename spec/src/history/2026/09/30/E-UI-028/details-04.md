---
schema: 1
id: E-UI-028
---

# E-UI-028 details 4

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
PATH=<scratch-cmake> CMAKE=<scratch-cmake> BOOST_ROOT=<scratch-boost> CC=<xcode-wrapper> CXX=<xcode-wrapper> pnpm spec:check
```

**Notes**

All 18 specification lanes pass after the mutation-union repair, including the cold catalog browser, generated HTTP/SSE fixtures, 181-test native lifecycle binary, complete Rust workspace, documentation and spec-first validation.
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
PATH=<scratch-cmake> CMAKE=<scratch-cmake> BOOST_ROOT=<scratch-boost> CC=<xcode-wrapper> CXX=<xcode-wrapper> pnpm check
```

**Notes**

The required check lane passes specification validation, boundaries, architecture, progress, formatting, embedded RustFS, 70 generated HTTP/SSE captures, both Svelte projects and workspace clippy.
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
PATH=<scratch-cmake> CMAKE=<scratch-cmake> BOOST_ROOT=<scratch-boost> CC=<xcode-wrapper> CXX=<xcode-wrapper> pnpm test
```

**Notes**

The superseded aggregate policy run was stopped with SIGINT at the user's request while rebuilding the cold RustFS executable. Its governance stage passed all 29 tests, including the five-document seed regression; no test had failed. The new targeted verification policy makes this redundant aggregate out of scope because exact native, contract, UI, cold browser and one prior full spec lane already cover the changed boundaries.
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
pnpm exec prettier --check AGENTS.md spec/src/milestones.md && pnpm spec:first && pnpm check:progress
```

**Notes**

The targeted formatter found only that the expanded final evidence entry needed Prettier wrapping; the downstream governance checks did not run in this chained command.
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
pnpm exec prettier --write AGENTS.md spec/src/milestones.md && pnpm spec:first && pnpm check:progress
```

**Notes**

Prettier and the spec-first map passed for 51 protected files and nine canonical chapters, then progress validation correctly rejected a pass result while the visual-baseline gap remains explicit.
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
pnpm exec prettier --write AGENTS.md spec/src/milestones.md && pnpm spec:first && pnpm check:progress
```

**Notes**

The requested repository guidance and final ledger are formatted; the spec-first map and progress evidence remain valid with visual approval explicitly pending under the targeted verification policy.
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
PATH=<scratch-cmake> CMAKE=<scratch-cmake> BOOST_ROOT=<scratch-boost> CC=<xcode-wrapper> CXX=<xcode-wrapper> pnpm dev:reset; live browser verification of the main bookmark
```

**Notes**

The rebuilt demo reset and seeded all five root documents, but the live Files view reported file_unavailable: five parallel landing-document reads exceeded native admission and returned busy for three previews.
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
pnpm exec tsx --test --test-name-pattern='repository landing documents include community files in stable tab order' web/tests/production-store-loaders.test.ts
```

**Notes**

The focused loader regression proves stable five-tab selection, friendly labels and ordering while limiting landing-document content loading to one active native read.
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
pnpm web:check && pnpm web:build
```

**Notes**

The affected Svelte application type check and production bundle pass; only the existing Lightning CSS :global and bundle-size warnings remain.
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
PATH=<scratch-cmake> CMAKE=<scratch-cmake> BOOST_ROOT=<scratch-boost> CC=<xcode-wrapper> CXX=<xcode-wrapper> pnpm dev:restart; live browser verification of repository 1bdb89c409c1b891ac7f180c01a265d5
```

**Notes**

The warmed demo restart reused the deterministic seed and became ready with search. The live main bookmark shows README, CODE_OF_CONDUCT, CONTRIBUTING, LICENSE and SECURITY tabs, six root files, the far-left tree toggle, icon-only Edit file action and a single breadcrumb filename input in edit mode.
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
pnpm exec prettier --write web/src/lib/controller/production-landing-documents.ts web/tests/production-store-loaders.test.ts spec/src/ui.md spec/src/milestones.md && pnpm spec:first && pnpm check:progress
```

**Notes**

Formatting passed, then spec-first required the API chapter as the second canonical owner for the production landing-document loader; progress validation did not run.
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
pnpm exec prettier --write web/src/lib/controller/production-landing-documents.ts web/tests/production-store-loaders.test.ts spec/src/api.md spec/src/ui.md spec/src/milestones.md && pnpm spec:first && pnpm check:progress
```

**Notes**

The focused source, regression and owning API/UI requirements are formatted; spec-first and progress validation pass for the live admission repair.
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
pnpm web:check && pnpm catalog:check
```

**Notes**

Both affected Svelte projects and catalog boundary checks pass for the friendly landing-document tabs, responsive More menu and body-only file editor.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit catalog/components/community-projects/repository-browser.test.ts
```

**Notes**

All 6 focused repository-browser unit tests pass, including the five filename-to-friendly-label mappings.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/community-projects/CommunityProjectsRepositoryDirectory.stories.svelte catalog/components/store-application/StoreFileEditor.stories.svelte
```

**Notes**

The first focused Chromium run exposed an undefined tabs-list ref during measured overflow; null-initialized bound refs fixed the component, and Vite completed one-time optimization for the added Lucide icons.
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreFileEditor.stories.svelte
```

**Notes**

The editor interaction reached committed rename but retained Edit as the selected display mode; ending the edit session now restores Markdown Preview and source mode for other text files. Icon whitespace assertions were also normalized to visible button text.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/community-projects/CommunityProjectsRepositoryDirectory.stories.svelte catalog/components/store-application/StoreFileEditor.stories.svelte
```

**Notes**

Both focused Chromium interaction/accessibility stories pass. Coverage includes friendly icon tabs, a constrained-width More dropdown, selected-document edit, persistent Preview/Source/Edit, retained drafts, breadcrumb rename, Save/Cancel, description dialog and preview/apply publication. Stories remain visual-pending.
<!-- /run -->

<!-- run -->
### Run 18

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first
```

**Notes**

The focused presentation and loader expectation changes required the mapped API chapter to state that friendly labels, overflow and selected-document edit routing remain client presentation rules.
<!-- /run -->
<!-- /evidence-field -->

