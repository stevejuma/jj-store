---
schema: 1
id: E-PROJECT-ROWS-001
---

# E-PROJECT-ROWS-001

<!-- evidence-field:id markdown -->
## Id

E-PROJECT-ROWS-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M11.4
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-015
- JST-API-001
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-11
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

rwkxtvyp
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-work/ProjectsHomeSpacing.stories.svelte
- catalog/fixtures/ProjectsHomeSpacingFixture.svelte
- web/src/lib/ui/store-work/community-work-collections.css
- spec/src/current/contracts/ui/18-installation-projects-hub.md
- spec/src/current/contracts/api/22-workstore-browse-summaries.md
- spec/src/current/progress/M11/index.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

[]
<!-- /evidence-field -->

<!-- evidence-field:story_files list -->
## Story files

- catalog/components/store-work/ProjectsHomeSpacing.stories.svelte
- catalog/components/store-work/ProjectsDirectory.stories.svelte
- catalog/components/store-work/ProjectsHub.stories.svelte
- catalog/components/store-work/ProjectsFilters.stories.svelte
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

UI-only main-workspace correction: adjacent project-home lists in one Mine/Other
collection reserve padding only at its outside edges. Independent home reads,
identity, snapshots, paging and selection remain. Generic/standalone row and
Tasks styling is unchanged.
<!-- /evidence-field -->

<!-- evidence-field:before_state markdown -->
## Before state

Live Projects page: Personal agent chats row height 44px, followed by a 31.195px
gap before Developer experience; later rows are contiguous. Different home lists
each had 24px bottom and 7.2px top padding. The source is owned
Community-derived collection CSS, not virtualization or a Design Core defect.
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
pnpm spec:search JST-UI-015
```

**Notes**

Read UI installation-hub and API browse topics, M11 ledger, owned
rows/collections and Community/Design Core references. Log
/tmp/jj-project-gap-requirement.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook --maxWorkers 1 catalog/components/store-work/ProjectsHomeSpacing.stories.svelte
```

**Notes**

Initial fixture lacked hub initialization and readiness wait; one missing-button
failure. The second readiness-only run also failed before initialization was
corrected. Logs /tmp/jj-project-gap-before.log and
/tmp/jj-project-gap-before-exact.log.
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
pnpm catalog:check
```

**Notes**

New fixture initially omitted required changed callback, summary counts/excerpt,
used clear rather than reset, and a Testing Library exact option. Fixed these
fixture errors without production changes. Log
/tmp/jj-project-gap-type-before.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook --maxWorkers 1 catalog/components/store-work/ProjectsHomeSpacing.stories.svelte
```

**Notes**

Qualified before state after fixture repair:
ProjectRowsRemainContiguousAcrossHomeRepositories fails geometry: expected
31.1875px gap to be below 1px. Log /tmp/jj-project-gap-before-ready.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook --maxWorkers 1 catalog/components/store-work/ProjectsHomeSpacing.stories.svelte
```

**Notes**

Exact regression passes with Axe: contiguous four rows from two same-owner homes
at 1440, 900 and 480 widths; collapse/expand retains rows, Grid layout has no
overflow. ResizeObserver loop diagnostic is emitted during viewport changes; no
functional/accessibility failure. Log /tmp/jj-project-gap-after.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook --maxWorkers 1 catalog/components/store-work/ProjectsDirectory.stories.svelte catalog/components/store-work/ProjectsHub.stories.svelte catalog/components/store-work/ProjectsFilters.stories.svelte
```

**Notes**

19 directly affected directory, hub and filter stories pass, including row
alignment, linked cards, tab/context navigation, failed-home retention,
paging/filter and loading cases. Log /tmp/jj-project-gap-consumers.log.
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
pnpm catalog:check
```

**Notes**

Zero Svelte errors/warnings; catalog boundaries pass. Log
/tmp/jj-project-gap-type-after.log.
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
pnpm check:ui-ownership
```

**Notes**

Owned Community-derived stylesheet retains original attribution and uses no
external source dependency. Log /tmp/jj-project-gap-ownership.log.
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
CUA background Projects tab: inspect rendered row geometry and save screenshot
```

**Notes**

All four actual live rows are 44px high, with 0px gap at the home boundary and
between subsequent rows. Screenshot /tmp/jj-store-project-spacing.jpg. No
backend rebuild/reset or fixture mutation.
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

Node 26.9.0; pnpm 10.32.1; JJ 0.45.1; Design Core 0.3.1; Storybook 10.5.7;
Vitest 4.1.11.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Owned CSS collection layout; Svelte 5 production-based fixtures; unchanged
WorkStore API schemas; Markdown evidence schema 1.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Retain M11.4 native/cold and visual gates. Inherited E-NPM-REGISTRY-001 evidence
must be corrected by its owner before repository-wide progress/spec-first can
pass.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-UI-015 installation Projects chapter clarifies continuous visual
  collections across homes. JST-API-001 unchanged confirmation in browse-summary
  chapter; no request, publication, authority or source ownership change.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M11.4 remains In progress. 20 focused interaction/accessibility stories
  qualify this collection padding correction, not native/cold browser or
  shared-package release gates. Visual baselines remain unapproved.

```
<!-- /evidence-field -->

<!-- evidence-field:excluded_scope markdown -->
## Excluded scope

Root aggregate/Rust/build/full-repository suites and baseline updates are out of
scope under the UI-only targeted policy. Existing package-adoption governance
failures and viewport ResizeObserver diagnostics remain separate.
<!-- /evidence-field -->

<!-- evidence-field:document_checks markdown -->
## Document checks

Final `pnpm exec spec-validator validate --only governance,summary,fileLimits`
and targeted `pnpm exec prettier --ignore-path /dev/null --check` for the
changed files pass. `pnpm check:progress` and `pnpm spec:first` report only the
same five inherited E-NPM-REGISTRY-001 diagnostics already reproduced on
tsptytzx; no fresh slice coverage/chapter error remains. Logs:
`/tmp/jj-project-gap-docs.log`, `/tmp/jj-project-gap-format.log`,
`/tmp/jj-project-gap-progress.log`, `/tmp/jj-project-gap-spec-first.log`.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


