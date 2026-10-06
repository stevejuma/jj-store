---
schema: 1
id: E-UI-058
---

# E-UI-058

<!-- evidence-field:id markdown -->
## Id

E-UI-058
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M16
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-02
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

repository-card-theme-alignment-and-menu-width
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change opznqxqt
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Appearance working copy is clean at opznqxqt/f7dbb1fb above Vite workflow nmxuylws/7d1d8ade. User identifies narrow header Repository actions and mismatched/sticky bookmark card surfaces on Branches. Read-only Chromium baseline on Buzz at 1351x988 measures128px menu, bookmark root rgb235/235/235 from popover versus white log column, and command-active neutral hover background persisting after pointer exit. Three observed presentation defects; no browser runtime errors. Community is inspected before Design Core; Command View intentionally retains a keyboard cursor, so app card interaction is adapted through its public disablePointerSelection option.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- web/src/lib/ui/community-projects/repository-row-tokens.css
- web/src/lib/ui/community-projects/CommunityProjectsRevisionGraph.svelte
- web/src/lib/ui/community-repository-branch-command/CommunityRepositoryBranchCommand.svelte
- web/src/lib/ui/community-repository-branch-command/CommunityWorkingCopyCard.svelte
- web/src/lib/ui/store-mutations/StoreRepositoryActions.svelte
- catalog/components/community-repository-branch-command/CommunityRepositoryBranchCommand.stories.svelte
- catalog/components/store-mutations/StoreRepositoryActions.stories.svelte
- docs/ui-provenance.json
- spec/src/ui.md
- spec/src/verification.md
- spec/src/milestones.md
- spec/src/api.md
- spec/src/spec-governance.md
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
jj --no-pager st; jj describe -m "Align Branches cards with repository log themes and widen actions menu"; jj --no-pager diff --stat; jj --no-pager diff (scoped source review)
```

**Notes**

Reused clean Appearance workspace; main unrelated work is untouched. Owning UI requirements and M16 state reviewed; Community source reference inspected first and Design Core public command/menu/column APIs second.
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
node /tmp/jj-branch-styles-probe.mjs (before and after)
```

**Notes**

Fresh isolated browser context with Buzz preference; read-only pinned native route. Before:128px popup, popover grey list versus white column, retained data-selected shading outside hover. After:256px popup, white matching list/cards, hover returns to white on pointer exit. No runtime errors. Before/after logs/JSON and after.png under /tmp/jj-branch-styles-*; no visual baseline approval.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/community-repository-branch-command/CommunityRepositoryBranchCommand.stories.svelte catalog/components/store-mutations/StoreRepositoryActions.stories.svelte (initial)
```

**Notes**

Branch3/3 passes; actions suite fails import after Vite newly optimizes arrow-up and reloads the test. Exact retry clears import failure but exposes separate existing mutation-dialog ScrollArea teardown diagnostics. No optimizer/diagnostic suppression.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/community-repository-branch-command/CommunityRepositoryBranchCommand.stories.svelte catalog/components/store-mutations/StoreRepositoryActions.stories.svelte (retry)
```

**Notes**

All7 assertions pass; two blocking derived_inert runtime diagnostics originate in Bits UI ScrollAreaScrollbarYState.onResize after DestructiveConfirmation/UnknownOutcomeRecovery teardown. These dialogs and scrollbar ownership are outside the changed menu/card styling surface. /tmp/jj-branch-styles-stories-final.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/RepositoryReview.stories.svelte -t "Shell and graph alignment"
```

**Notes**

Incorrect human-readable filter skipped3 stories; not claimed as acceptance. Final target uses the generated CamelCase test name and executes the graph alignment story.
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
jj workspace add /private/tmp/jj-store-branch-card-baseline --name branch-card-baseline -r @- -m "Disposable repository menu diagnostic baseline"; ln -s /Users/stevejuma/code/lapismd/jj-store-appearance/node_modules /private/tmp/jj-store-branch-card-baseline/node_modules; pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-mutations/StoreRepositoryActions.stories.svelte (baseline and retry)
```

**Notes**

Unchanged parent source baseline. Both attempts fail before tests because the symlinked addon setup import is outside the temporary Vite filesystem allow roots. Temporary baseline-only config explicitly allows the existing dependency directory; no tracked build configuration or API admission is changed.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config .scratch/baseline.vitest.config.ts --project storybook catalog/components/store-mutations/StoreRepositoryActions.stories.svelte (unchanged parent baseline)
```

**Notes**

Parent source nmxuylws/7d1d8ade executes all4 assertions successfully and reproduces3 derived_inert ScrollArea teardown diagnostics, including the same DestructiveConfirmation/UnknownOutcomeRecovery stacks. Confirms the wider dialog failure predates these styles. Existing pinned Bits UI2.18.1/runed0.35.1 behavior remains a separate upstream qualification gap. /tmp/jj-branch-styles-parent-qualified.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/community-repository-branch-command/CommunityRepositoryBranchCommand.stories.svelte catalog/components/store-mutations/StoreRepositoryActions.stories.svelte catalog/components/store-application/RepositoryReview.stories.svelte -t "BookmarkCards|EmptyBookmarks|EmptyFilterStates|MenuLayout|ShellAndGraphAlignment"
```

**Notes**

Exact complete affected style/interaction surface5/5 passes,6 unrelated stories skipped; blocking interaction/accessibility and runtime guard retained. Synthetic pointer events cannot set CSS hover, so branch story verifies unchanged command cursor and no retained shading; real pointer hover is covered by native-browser probe. Menu labels/256px width/Escape close, keyboard exact selection, filter/search/empty states and shared graph/header geometry all pass. /tmp/jj-branch-styles-focused-stories.log.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
node /tmp/jj-branch-theme-acceptance.mjs (development attempts)
```

**Notes**

Harness first compared equivalent oklch/oklab foreground serialization; normalized rendered foreground to RGBA. Next compared while shell theme transitions were still running; isolated test disables transition/animation to measure settled token values. Third passes all64 themes and keyboard/desktop menu but wrongly expects desktop Repository actions at390px, where the existing mobile shell omits that control. Narrowed viewport assertion to the requested desktop menu; no product assertion or security policy weakened. /tmp/jj-branch-styles-qualified{,-final,-settled}.log.
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
node /tmp/jj-branch-theme-acceptance.mjs (qualified desktop)
```

**Notes**

Fresh isolated Chromium pages compare actual native Branches and selected Op Log on all64 themes, including Original light/dark. Column/card/foreground/header/group-heading parity; identical hover/selected surfaces; bookmark and working-copy pointer exit; unchanged keyboard cursor on hover; selected bookmark restores selected shade after hover. Real search ArrowDown/Enter selects exact bookmark;256px menu remains within1351px viewport. Browser runtime errors0. No native writes or changes to the user browser preferences. /tmp/jj-branch-styles-qualified-desktop.log; qualified.json/png.
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
pnpm web:check; pnpm catalog:check; pnpm check:ui-ownership
```

**Notes**

Web and catalog0 errors/warnings; catalog129-module boundary and owned Community provenance pass. New shared tokens record their existing Apache-2.0 Community graph source pin. Production imports stay independent of catalog and Community packages. /tmp/jj-branch-styles-{web-check,catalog-check,ownership}.log.
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
pnpm spec:first (first canonical validation)
```

**Notes**

Source/provenance change map additionally requires api.md and spec-governance.md. Added exact native identity/capability presentation and owned shared-token provenance/state requirements to those owning chapters; final gate passes without map exceptions. /tmp/jj-branch-styles-spec-first-initial.log.
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
pnpm exec prettier --write (changed supported files); pnpm exec prettier --check (same files); pnpm spec:first; pnpm spec:validate; pnpm check:progress
```

**Notes**

Same-change canonical UI, verification mapping and ledger validation. Focused formatting; no milestone completion status or visual baseline change. /tmp/jj-branch-styles-{format,format-check,spec-first,spec-validate,progress}.log.
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
jj workspace forget branch-card-baseline; trash -v /private/tmp/jj-store-branch-card-baseline
```

**Notes**

Disposable parent diagnostic workspace forgotten before recoverable directory cleanup. Owned interactive Vite/backend remain running; no backend restart or build.
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

<!-- evidence-field:scope markdown -->
## Scope

UI-only repository card/menu presentation and its direct browser/Storybook consumers. Shared row token extraction preserves existing log color recipes and does not alter navigation/controller/native contracts. Exact focused stories, web/catalog type checks, ownership and all-theme native read-only browser comparison are the acceptance surface. Broader dialog stories were tried only for diagnostics and reproduce an upstream ScrollArea lifecycle warning on unchanged parent; they are not a full-suite pass. M16 remains Partial with prior foundation release, registry-only, composer-CSP and visual gates retained. Rust builds/tests, root aggregates, unrelated story suites and production built-CSP qualification are out of scope; Vite HMR applies the UI changes without stack rebuild.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Node26.9.0; pnpm10.32.1; Vite8.3.2; Svelte5.57.1; Vitest4.1.11; Storybook10.5.7; existing Bits UI2.18.1/runed0.35.1;64 owned theme styles. Foundational dependencies and lockfiles unchanged.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No native/API/preference/storage formats change. Cards use the column card surface, foreground and shared foreground7% hover/12% selected recipes. Headers keep jj-store-row-background. Keyboard command cursor remains distinct from exact route selection and pointer hover; public disablePointerSelection prevents hover from changing the cursor. Menu layout uses16rem with viewport maximum.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-UI-058 details 2](details-02.md)
