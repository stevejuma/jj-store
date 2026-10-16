---
schema: 1
id: E-FORGE-BUTTONS-001
---

# E-FORGE-BUTTONS-001 details 2

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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-forge/StoreForgeWorkspace.stories.svelte
```

**Notes**

Before title-row/button styling: 2 failed, 2 passed. New light/dark regressions measured the PR button center 62.609375px below its title. Log .local/forge-buttons-before.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-forge/StoreForgeWorkspace.stories.svelte
```

**Notes**

First rerun: 2 failed, 2 passed. The test queried a native header but the released Card.Header is a div. Corrected to the semantic card-header selector without weakening geometry/color assertions. Log .local/forge-buttons-after.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-forge/StoreForgeWorkspace.stories.svelte
```

**Notes**

Second rerun: 2 failed, 2 passed. Native pointer sampling found the former column-header Submit button hover differed by only 11 RGB steps. Its public header-action token was set for that placement. Log .local/forge-buttons-after2.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-forge/StoreForgeWorkspace.stories.svelte
```

**Notes**

Before the additional Submit placement request: 4 passed. Exact title alignment, opaque background, visible border, native hovered-parent/button contrast and PR selection without selecting the containing branch pass in both themes. Log .local/forge-buttons-after3.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceRefinements.stories.svelte -t 'BranchChipsShare|BranchAndCommitChips'
```

**Notes**

Initial direct no-forge consumers: 2 passed, 8 intentionally skipped. Log .local/forge-buttons-consumers.log.
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
pnpm catalog:check
```

**Notes**

Initial and intermediate scoped checks: zero Svelte errors/warnings; catalog boundaries pass (447 modules). Logs .local/forge-buttons-types.log and .local/forge-buttons-types-final.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-forge/StoreForgeWorkspace.stories.svelte -t 'SubmitStackFromColumnMenu'
```

**Notes**

Before relocation/menu implementation: 1 failed, 4 intentionally skipped. No Submit stack button exists in the first bookmark action row. Log .local/forge-submit-placement-before.log.
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
node scripts/web-architecture.mjs
```

**Notes**

Two interim checks found BookmarkCard at 301 counted lines against the unchanged 300-line boundary. Reused existing head/immutability facts and removed a repeated lookup; no boundary exception or limit change.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-forge/StoreForgeWorkspace.stories.svelte
```

**Notes**

First placement/menu run: 4 passed, 1 failed. Submit and PR hover/alignment and the submission button flow passed, but the new menu item accessible name included its labelled forge icon. Log .local/forge-submit-placement-after.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-forge/StoreForgeWorkspace.stories.svelte -t 'SubmitStackFromColumnMenu'
```

**Notes**

Diagnostic rerun: 1 failed, 4 intentionally skipped. Added an aria-expanded assertion to prove the column menu opens; its item needed an explicit Submit stack accessible name. Corrected the production menu label. Log .local/forge-submit-menu-debug.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-forge/StoreForgeWorkspace.stories.svelte
```

**Notes**

Placement/menu correction: 5 passed. Log .local/forge-submit-placement-final.log. Removed the obsolete column-header hover token after relocating Submit; card/title and action-row contrast remain covered.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-forge/StoreForgeWorkspace.stories.svelte
```

**Notes**

Final fresh Vitest/Chromium run: 5 passed, zero interaction/accessibility failures. Covers rendered details, top-card Submit immediately after Push, absence from lower cards/header, menu opening and Escape/focus restoration, identical menu preview, retained Markdown submission editing, title alignment and native-pointer hover/border/background checks in light/dark. The fixture assertion ensures multiple bookmark action rows exist. Log .local/forge-submit-final.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceRefinements.stories.svelte -t 'BranchChipsShare|BranchAndCommitChips|BranchActionsDiffer|KebabMenusHave'
```

**Notes**

Final direct unchanged/no-forge consumers: 5 passed, 5 intentionally skipped. Covers metadata wrapping, branch/commit chip contrast, action-row shading and light/dark column/branch/commit kebab alignment and hover. Log .local/forge-buttons-consumers-final.log.
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
pnpm catalog:check
```

**Notes**

Final zero Svelte errors/warnings, catalog boundaries pass (447 modules). Log .local/forge-buttons-types-final2.log.
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
node scripts/web-architecture.mjs
```

**Notes**

Maintained module boundaries pass (1941 modules), including both changed stack components at or below 300 counted lines.
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
pnpm check:ui-ownership
```

**Notes**

Owned production composition remains independent of catalog and Community checkouts. No new component, copied source, dependency, provenance or ownership exception.
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
pnpm check:progress
```

**Notes**

Fresh same-change evidence and explicit M31 review pass.
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
pnpm spec:first
```

**Notes**

Owning UI/forge chapters and API unchanged-contract confirmation cover this presentation slice.
<!-- /run -->
<!-- /evidence-field -->

