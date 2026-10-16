---
schema: 1
id: E-UI-TAG-LIST-001
---

# E-UI-TAG-LIST-001 details 2

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
pnpm exec svelte-check --tsconfig web/tsconfig.json
```

**Notes**

Before edits: zero errors/warnings with Design Core 0.3.2.
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
pnpm test:storybook catalog/components/community-projects/CommunityProjectsRevisionGraph.stories.svelte catalog/components/store-mutations/StoreMutationCommitPicker.stories.svelte
```

**Notes**

Before edits: 10 stories passed. /tmp/jj-tags-baseline-stories.log.
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
pnpm test:storybook catalog/components/store-stack-workspace/StoreStackWorkspaceRefinements.stories.svelte catalog/components/store-threads/StoreThreads.stories.svelte catalog/components/store-application/StoreCaptureSources.stories.svelte
```

**Notes**

Before edits: 19 stories passed. /tmp/jj-tags-baseline-more.log.
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
pnpm add -D '@lapismd/design-core@^0.5.0'
```

**Notes**

Installed published registry 0.5.0; portable range and registry integrity retained in the lockfile. No package hook, override, source alias or archive substitution.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 11
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/community-projects/CommunityProjectsRevisionGraph.stories.svelte catalog/components/store-mutations/StoreMutationCommitPicker.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceRefinements.stories.svelte catalog/components/store-threads/StoreThreads.stories.svelte catalog/components/store-application/StoreCaptureSources.stories.svelte
```

**Notes**

Initial adoption run: 18 passed, 11 failed. Includes old DOM assertions, measurement/overlay transition races and missing consumer utility generation. No assertions or a11y rules were disabled. Next run: 26 passed, 3 failed, all in Threads; resolved by explicit consumer-owned Tailwind entry and transition completion checks.
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
pnpm test:storybook catalog/components/tag-list-adoption/TagListAdoption.stories.svelte catalog/components/store-threads/StoreThreads.stories.svelte catalog/components/store-application/WorkflowHostSettings.stories.svelte
```

**Notes**

After initial test updates: 18 passed, 4 failed. Two new adoption-fixture timing/size assumptions plus Thread action alignment and Workflow popover transition. After explicit utilities entry: existing 20 stories passed; two new fixture assertions remained.
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
pnpm test:storybook catalog/components/tag-list-adoption/TagListAdoption.stories.svelte catalog/components/store-mutations/StoreMutationCommitPicker.stories.svelte
```

**Notes**

Final focused picker/adoption run: all 4 passed, including nested popup interaction/accessibility. Listbox options retain text summaries; interactive TagList lives in a preview footer outside the listbox.
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
pnpm web:test
```

**Notes**

180 web unit tests passed.
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
pnpm catalog:test
```

**Notes**

184 files, 1165 tests passed and one existing skipped test. Includes direct UI/domain adapters. No Rust lane run.
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
pnpm exec vite build --config web/vite.config.ts
node scripts/web-manifest.mjs
```

**Notes**

Production UI bundle and native asset manifest checks passed. Docs staging, Rust, live backend and native distribution qualification remain outside this UI slice.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
CI=true pnpm test:storybook --shard=1/2
CI=true pnpm test:storybook --shard=2/2
```

**Notes**

Broader UI escalation because dependency/CSS entry affects web and catalog. Shards stopped early: AgentPromptMarkup closed-menu a11y transition and ProjectsHub initial Reviews tab startup. Exact current rerun: ProjectsHub passed, AgentPromptMarkup reproduced. Parent workspace with locked 0.3.2: both exact targets passed. AgentPromptMarkup now waits for keyboard menu closure before a11y, matching its existing pointer path. Full runs are repeated without bail.
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
pnpm test:storybook catalog/components/store-application/StoreCaptureSettings.stories.svelte catalog/components/store-stack-workspace/StoreStackTopology.stories.svelte
```

**Notes**

After replacing chip-specific assertions with visible TagList rows and waiting for measurement: all 26 directly affected stories passed. Initial extra-family run: 3 failed/23 passed; current result closes these failures. The inert measurement copies are never counted as displayed tags.
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
pnpm catalog:check
```

**Notes**

Final catalog and imported production types: zero errors/warnings, 461 modules satisfy catalog boundaries. Intermediate web check found one declaration-order TypeScript error in the new preview; moving selected before preview resolved it.
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
pnpm check:ui-ownership
```

**Notes**

Released package use and maintained source/license attribution validated.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentPromptMarkup.stories.svelte catalog/components/store-work/ProjectsHub.stories.svelte -t 'OpenCodeTypedTextMessageParts|UnsupportedReviewsEmptyState'
```

**Notes**

Untouched parent ae9e4ea0, locked Design Core 0.3.2 in a disposable JJ workspace: two exact tests passed. In the requested workspace the first reproduces a keyboard-menu exit/a11y timing failure; waiting for menu exit fixes that exact case. The Reviews case passes the exact requested-workspace rerun.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentPromptMarkup.stories.svelte
```

**Notes**

Parent 0.3.2 and current 0.5.0 both fail only MarkupAttributesRemainInertAndReferencesSpanSections, with the same existing 4.14:1 reference-label contrast. Other nine stories pass in each exact family run. This is separate from the tag adoption.
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
```

**Notes**

First run overlapped working-copy updates and rejected a changed revision. Second run correctly identified newly edited helper/history paths missing from the evidence list. The list is now refreshed; rerun after source freeze.
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
pnpm test:storybook catalog/components/community-projects/CommunityProjectsRevisionGraph.stories.svelte catalog/components/store-mutations/StoreMutationCommitPicker.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceRefinements.stories.svelte catalog/components/store-stack-workspace/StoreStackTopology.stories.svelte catalog/components/store-application/StoreCaptureSettings.stories.svelte catalog/components/store-application/StoreCaptureSources.stories.svelte catalog/components/store-application/WorkflowHostSettings.stories.svelte catalog/components/store-threads/StoreThreads.stories.svelte catalog/components/tag-list-adoption/TagListAdoption.stories.svelte
```

**Notes**

Fresh Vitest/Chromium process: nine directly affected files and all 69 interaction/accessibility stories passed. Includes resizing, live tags, full labels, independent commit preview, keyboard activation, Escape/focus restoration and row-selection isolation. Visual baselines remain unchanged.
<!-- /run -->
<!-- /evidence-field -->

