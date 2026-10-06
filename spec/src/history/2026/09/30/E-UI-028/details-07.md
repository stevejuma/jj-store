---
schema: 1
id: E-UI-028
---

# E-UI-028 details 7

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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/community-projects/CommunityProjectsRepositoryDirectory.stories.svelte catalog/components/store-application/StoreFileEditor.stories.svelte
```

**Notes**

The responsive commit-details story passed; the editor story reached the fit-content filename input and exposed a brittle assertion that expected its breadcrumb item to be wider even though both correctly shrink to the same content width.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/community-projects/CommunityProjectsRepositoryDirectory.stories.svelte catalog/components/store-application/StoreFileEditor.stories.svelte
```

**Notes**

Both focused Chromium interaction/accessibility stories pass. Coverage includes commit-details routing, narrow-width message and revision hiding without overflow, a padded content-sized filename input and a focused CodeMirror surface with no outline or box shadow; stories remain visual-pending.
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
pnpm web:build
```

**Notes**

The production UI bundle and manifest rebuild pass with the existing Lightning CSS :global and bundle-size warnings; no Rust build ran.
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
shell-background existing-binary demo restart and immediate live reload
```

**Notes**

The first direct server process became ready, then inherited cleanup from the completed tool shell and exited before browser reload. No state or fixture was replaced.
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
detached existing-binary demo restart and live 900px/480px repository and editor verification
```

**Notes**

The preserved demo runs from the rebuilt web bundle and existing native binaries. Desktop shows the independent details button and truncated-safe summary. At 480px the 436px summary has equal client and scroll widths, hides the message and revision ID, and retains author, relative time, commit details and snapshot selection. The live README editor reports field-sizing content, 10.4px filename padding, an 87.6px filename width and a focused CodeMirror surface with outline none and box-shadow none.
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
pnpm exec prettier --write <changed UI, story and canonical files> && pnpm spec:first && pnpm check:progress
```

**Notes**

All changed files formatted successfully; spec-first then required the mapped API chapter to state that responsive commit metadata and editor sizing/focus remain client presentation, so progress validation did not run in the chained command.
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
pnpm exec prettier --write spec/src/api.md spec/src/ui.md spec/src/milestones.md && pnpm spec:first && pnpm check:progress
```

**Notes**

Formatting and spec-first passed for six protected files and three canonical chapters. The progress checker then exceeded Node's default 1 MiB child-process buffer while reading the 1,054,179-byte parent ledger, before ledger validation ran.
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
node --test scripts/governance.test.mjs; pnpm spec:first; pnpm check:progress
```

**Notes**

All 21 focused governance tests and the repaired progress checker passed. Spec-first required the mapped governance chapter to record the new bounded large-ledger child-process behavior.
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
node --test scripts/governance.test.mjs && pnpm exec prettier --write <changed tooling, UI, story and canonical files> && pnpm spec:first && pnpm check:progress
```

**Notes**

All 21 focused governance tests pass, all changed files are formatted, spec-first maps seven protected files to four canonical chapters and the 61-task evidence ledger validates with the bounded parent-ledger read.
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
live Open commit details interaction on the preserved repository Files page
```

**Notes**

The new details action updated the selected revision while the asynchronous page change was still pending, so the router remained on Files instead of presenting the selected commit in Commits.
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
pnpm web:check; pnpm catalog:check; pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/community-projects/CommunityProjectsRepositoryDirectory.stories.svelte catalog/components/store-application/StoreFileEditor.stories.svelte; pnpm web:build
```

**Notes**

Awaiting Files-to-Commits page selection before selecting the revision removes the live routing race. Both Svelte checks, both focused Chromium interaction/accessibility stories and the production UI bundle pass; the application story asserts Commits is selected before returning to Files to exercise editing. No Rust build or Rust test ran, and the stories remain visual-pending.
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
pnpm check:architecture; node --test scripts/governance.test.mjs; pnpm exec prettier --write <all changed files>; pnpm spec:first; pnpm check:progress
```

**Notes**

The final targeted gate passes all 260 architecture modules, all 21 governance tests, formatting for every changed file, the eight-file/four-chapter spec-first mapping and the 61-task progress ledger.
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
detached existing-binary demo restart and fresh-browser Open commit details verification
```

**Notes**

The preserved LocalTrusted demo serves the rebuilt web bundle at 127.0.0.1:7460 from the existing server and embedded RustFS binaries. The landing page exposes Open commit details; activating it selects Commits and loads the exact 912c8df1 revision with its 5d0e9e4c change identity. No Rust build or Rust test ran.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Visual baselines remain unapproved.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; Node.js 26.9.0; pnpm 10.32.1; Svelte 5.57.1; Storybook 10.5.7; Design Core ^0.2.0; Community ^0.1.0.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

The V1 typed mutation command union and capability list add edit_file with an exact bookmark target, canonical source/destination paths, bounded UTF-8 contents and description. Existing mutation request, prepared result, durable receipt, WAL, manifest and native object formats remain unchanged. Friendly landing-document labels/icons, measured More overflow, responsive commit details, body tabs/breadcrumb, persistent file display controls, the content-sized filename field, focus-ring-free line-numbered filename-highlighted Mira edit body and deferred search routing are browser presentation only; credentials remain memory-only. Changed stories remain visual-pending.
<!-- /evidence-field -->

