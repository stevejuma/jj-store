---
schema: 1
id: E-M21-060
---

# E-M21-060 details 2

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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceConflicts.stories.svelte -t <display name>
```

**Notes**

Initial filter matched no tests: all six skipped. This was not regression evidence; reran the entire exact conflict family.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceConflicts.stories.svelte
```

**Notes**

Before implementation: new four-tab scope assertion failed on Branches, with five other stories passing. /tmp/stacks-banner-scope-before-family.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook <conflict and standalone workspace stories>
```

**Notes**

First after: 19 passed/2 failed. Conflict story still requested the old global banner from Branches; NarrowLayout expected Workspace to remain a direct tab despite overflow. /tmp/stacks-banner-scope-after.log.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook <same two families>
```

**Notes**

Standalone conflict navigation initially used an ambiguous Workspace tab query; scoped it to Local repository views. NarrowLayout remained. /tmp/stacks-banner-scope-final-stories.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook <same two families>
```

**Notes**

20 passed; only NarrowLayout failed. Four-tab banner/retained editor and standalone conflict flows pass. /tmp/stacks-banner-scope-complete-stories.log.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
Restore only the unchanged parent banner component for a controlled baseline; pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspace.stories.svelte -t NarrowLayout; restore scoped component in finally
```

**Notes**

Pre-existing narrow assertion reproduced on the parent component: one failed/14 skipped. No shared helper or production layout change was needed. /tmp/stacks-banner-narrow-parent.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspace.stories.svelte -t NarrowLayout
```

**Notes**

One passed/14 skipped after checking the visible selected More trigger, selected Workspace menu item, Escape focus restoration, Workspace content and unchanged 390px width limit. /tmp/stacks-banner-narrow-after.log.
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
pnpm web:check; pnpm catalog:check; pnpm check:ui-ownership; pnpm check:architecture
```

**Notes**

Zero type errors/warnings; 244 catalog modules and 1141 production modules; ownership passes. /tmp/stacks-banner-web.log, catalog.log, ownership.log and architecture.log.
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
pnpm docs:review application <Workspace-only banner review>; pnpm docs:check
```

**Notes**

Browsing guide and source fingerprint updated. 139-file closure, links/anchors and nine light/dark offline diagrams pass. /tmp/stacks-banner-docs.log.
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
Owned Vite root transfer to Main and runUiDevelopment(Main, retained fixture environment, backendOnly=true)
```

**Notes**

Existing merged native executable reused to match Main additive reads; no Rust rebuild/test. Main Vite 91701 on 7476 and backend 92095 on 7478 reuse the source .local/dev-demo. Saved UI backend configuration is byte-identical; fixture, registration and app settings preserved. /tmp/stacks-banner-live-main.log.
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
CUA cold reload and Branches/Commits/Op Log/Workspace navigation on the existing live user tab
```

**Notes**

Real conflicted workspace: banner counts 0,0,0,1 respectively. Returned to the original Workspace route; two conflicts and thirteen unstaged files retained. Rail/toast resolution actions remain available; no resolution or graph/membership mutation requested; ordinary startup refresh retains normal JJ snapshot policy.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook <conflict and standalone workspace stories>
```

**Notes**

21 passed in two files after the narrow overflow repair. /tmp/stacks-banner-all-stories.log.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm catalog:check
```

**Notes**

Final leaf-story edits used Playwright-only exact options on three Testing Library getByRole/findByRole calls. Removed those unsupported options; string role names already match exactly. /tmp/stacks-banner-catalog-final.log.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first
```

**Notes**

Protected mapping requires API and documentation chapter updates alongside UI. Added explicit unchanged-transport boundary and manual/overflow verification scope; no contract fields added. /tmp/stacks-banner-spec-first.log.
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
pnpm catalog:check; pnpm spec:first; pnpm spec:validate
```

**Notes**

Final types zero errors/warnings, 244 catalog modules; spec-first five protected files/four chapters; validate five validators/23 chapters/173 requirements. /tmp/stacks-banner-catalog-complete2.log, spec-first-final2.log and spec-validate-final2.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceConflicts.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspace.stories.svelte
```

**Notes**

Final after typed query correction: 21 passed in two files in a fresh browser process. /tmp/stacks-banner-verified-stories.log.
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
pnpm exec prettier --check <eight changed UI/story/manual/canonical files>
```

**Notes**

All changed non-ledger files formatted. Ledger prefix and only the newest evidence entry are formatted separately, preserving historical payloads. /tmp/stacks-banner-format-final.log.
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

Continue UI work from Main; existing Main Vite/backend remains available on 7476/7478 with the preserved fixture. Other gates and visual baselines remain separate.
<!-- /evidence-field -->

<!-- evidence-field:verification_scope markdown -->
## Verification scope

UI-only presentation scope. Exact four-tab regression, directly affected conflict/standalone stories, web/catalog types, UI ownership/architecture, documentation/specification/progress and cold live browser. The standalone narrow assertion was reproduced as pre-existing and repaired in its own story without changing shared infrastructure or production layout. Unrelated story families, root aggregate lanes, Rust builds/tests, package publication, screenshot comparisons and visual baselines remain out of scope; no milestone promotion.
<!-- /evidence-field -->

