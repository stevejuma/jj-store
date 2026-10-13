---
schema: 1
id: E-CREATE-FORMS-001
---

# E-CREATE-FORMS-001 details 2

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
jj --no-pager st
```

**Notes**

Before: clean zzlznykq 8e292352, parent 885401f1. Existing development fixture and user-owned workflow tab preserved.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/RepositoryEntryControls.stories.svelte catalog/components/store-work/ProjectsHub.stories.svelte -t 'One repository menu opens server and local forms|Create project selects writable home and retains request'
```

**Notes**

Initial before command skipped all 13 stories because generated test names use PascalCase. This is not a passing baseline. Source before-state: legacy border groups, no Settings cards/icons; project create in body and Done below it.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/RepositoryEntryControls.stories.svelte catalog/components/store-work/ProjectsHub.stories.svelte catalog/components/store-work/StoreWork.stories.svelte
```

**Notes**

Intermediate UI verification: corrected an invalid nested Svelte snippet (1 web error), story line budget by extracting a narrow helper, and test selectors using field-label data-ui-part rather than data-slot. Subsequent iterations: 3 then 2 then 1 failed, with 19, 20 and 21 passing. Card outlines are the released Design Core 1px shadow, not CSS borders. The unrelated existing header story used a retired global Settings Repository button; its test now follows canonical body-header Repository settings and asserts the card outline/header divider without changing runtime navigation.
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
pnpm web:check
```

**Notes**

Final web check: zero errors/warnings. Initial 1 error was the named snippet supplied inside Field.Field; moved it outside. /tmp/jj-store-create-forms-web.log.
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
pnpm catalog:check
```

**Notes**

Final zero errors/warnings; 424 catalog modules. Initial 2 helper typing errors corrected with typed HTMLElement closest results. /tmp/jj-store-create-forms-catalog.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/RepositoryEntryControls.stories.svelte catalog/components/store-work/ProjectsHub.stories.svelte catalog/components/store-work/StoreWork.stories.svelte
```

**Notes**

22 tests across 3 files, including configured Axe checks. Fresh isolated Vitest/Vite browser process; live development services were not restarted. /tmp/jj-store-create-forms-focused.log.
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
pnpm exec vitest run --config /tmp/jj-store-creation-mobile.config.mts --project storybook catalog/components/store-application/RepositoryEntryControls.stories.svelte catalog/components/store-work/ProjectsHub.stories.svelte -t 'OneRepositoryMenuOpensServerAndLocalForms|CreateProjectSelectsWritableHomeAndRetainsRequest'
```

**Notes**

2 creation tests pass, 11 intentionally skipped, 390x844 viewport. Temporary config imports repository config and changes only the Chromium instance viewport; no tracked test/runtime configuration changed. /tmp/jj-store-create-forms-mobile.log.
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
pnpm check:architecture
```

**Notes**

1810 modules. Initial ProjectsHub story exceeded 400 counted lines; narrow assertions extracted into an owned catalog helper.
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
pnpm check:ui-ownership
```

**Notes**

Ownership validator passes. Community Settings and Design Core Field/DialogFrame inspected; no external copied source or runtime import introduced. New creation CSS and opted-in forms declared in provenance.
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
CUA: fresh local app tab; open Create project and Create server repository; inspect cards and close without submitting
```

**Notes**

Running Vite app shows tinted icon-bearing cards, trailing bounded controls, full-width Markdown row and footer actions. Screenshots: /tmp/jj-store-create-project-settings.png and /tmp/jj-store-create-repository-settings.png. Temporary tab closed; no repository/project data created and no user-tab navigation.
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
pnpm spec:first; pnpm docs:review
```

**Notes**

Initial expected governance checks required fresh evidence/ledger and application documentation review. Added this record, canonical UI/doc/ownership changes, milestone review and manual guidance.
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
pnpm docs:review application 'Repository and project creation now use the documented Settings cards, icon rows, responsive labels and controls, with project commands in the dialog footer. Writable homes, retained drafts and original request recovery remain unchanged.'
```

**Notes**

Updated task guidance and reviewed application source fingerprint.
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
pnpm check:progress; pnpm spec:first; pnpm spec:validate
```

**Notes**

Intermediate record validation rejected narrative test references and a pending run result. Replaced them with existing story files in files/runs and recorded the skipped command as exit-success with explicit zero executed tests. Scoped navigation created an unlinked day SUMMARY.md; removed it and updated the day index with the canonical navigation helper.
<!-- /run -->

<!-- run -->
### Run 14

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

Mapped API confirmation was required for protected browser form changes. Added an unchanged-wire/authority confirmation to WorkStore service and transports.
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
pnpm catalog:check; pnpm check:architecture; pnpm check:ui-ownership
```

**Notes**

Final catalog 0 errors/warnings, 424 modules; architecture 1810; owned UI boundary passes.
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
pnpm exec prettier --check catalog/components/store-application/RepositoryEntryControls.stories.svelte catalog/components/store-work/ProjectsHub.stories.svelte catalog/fixtures/project-creation-settings-play.ts docs/maintenance/coverage.json docs/site/guides/settings.md docs/ui-provenance.json spec/src/current/contracts/api/21-workstore-service-and-transports.md spec/src/current/contracts/documentation/01-application-documentation.md spec/src/current/contracts/ui/17-native-projects-and-tasks.md spec/src/current/contracts/ui/28-complete-supported-configuration-settings.md spec/src/current/governance/spec-governance.md spec/src/current/progress/M11/index.md spec/src/current/progress/M14/index.md spec/src/current/verification/ui-02.md spec/src/current/verification/work.md web/src/lib/ui/store-settings/StoreNativeRepositoryCreate.svelte web/src/lib/ui/store-settings/creation-forms.css web/src/lib/ui/store-work/StoreWorkDialog.svelte web/src/lib/ui/store-work/StoreWorkField.svelte web/src/lib/ui/store-work/StoreWorkProjectForm.svelte
```

**Notes**

All matched authored files formatted; generated evidence/history follows canonical helpers.
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
PATH=/Users/stevejuma/.local/bin:$PATH pnpm docs:check; pnpm spec:links
```

**Notes**

143-file documentation closure, 9 Mermaid diagrams light/dark and link/anchor validation. Existing generated book: 2679 HTML files, 101087 local links.
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
pnpm check:progress; pnpm spec:first; pnpm spec:validate
```

**Notes**

Final: 153 tasks, 780 evidence records; 11 protected files and 5 mapped canonical chapters; 7 specification validators, 3729 chapters, 196 requirements. No milestone status promoted.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

