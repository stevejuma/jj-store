---
schema: 1
id: E-UI-059
---

# E-UI-059

<!-- evidence-field:id markdown -->
## Id

E-UI-059
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

review-empty-task-activity-settings-header
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change zmzkuouq
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Clean reused Appearance workspace zmzkuouq above opznqxqt/fd1f28bc. Unsupported local reviews are stored as an error and render retry. Task comments and repository activity are collapsed together in the right panel. Settings header has a return arrow and right-aligned title, while Appearance repeats the title. Owning chapters and milestones reviewed, Community inspected before Design Core Empty; native identities and snapshot policies unchanged.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-appearance/StoreAppearance.stories.svelte
- catalog/components/store-application/StoreApplication.stories.svelte
- catalog/components/store-application/settings-shell-play.ts
- catalog/components/store-work/ProjectsHub.stories.svelte
- catalog/components/store-work/StoreWork.stories.svelte
- catalog/fixtures/AppearanceFixture.svelte
- catalog/fixtures/ProjectsHubFixture.svelte
- catalog/fixtures/projects-hub-layout.ts
- spec/src/api.md
- spec/src/ui.md
- spec/src/verification.md
- web/src/lib/controller/projects-reviews.svelte.ts
- web/src/lib/ui/store-appearance/StoreAppearance.svelte
- web/src/lib/ui/store-appearance/appearance.css
- web/src/lib/ui/store-application/StoreApplication.svelte
- web/src/lib/ui/store-settings/StoreRepositorySettings.svelte
- web/src/lib/ui/store-settings/StoreSearchSettings.svelte
- web/src/lib/ui/store-settings/StoreSettings.css
- web/src/lib/ui/store-settings/StoreSettings.svelte
- web/src/lib/ui/store-settings/StoreSettingsPage.svelte
- web/src/lib/ui/store-settings/StoreSyncSettings.svelte
- web/src/lib/ui/store-work/StoreProjectsReviews.svelte
- web/src/lib/ui/store-work/StoreWorkComments.svelte
- web/src/lib/ui/store-work/StoreWorkTaskDetail.svelte
- web/tests/browser/appearance.spec.ts
- web/tests/browser/local-search-settings.spec.ts
- web/tests/browser/local-sync.spec.ts
- web/tests/browser/local-workspaces.spec.ts
- web/tests/real/local-browser.spec.ts
- web/tests/vite/projects-directories.test.ts
- spec/src/milestones.md
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
jj --no-pager st; jj describe -m "Refine reviews empty state, task activity and settings headers"; jj --no-pager diff --stat; scoped source/spec/reference review
```

**Notes**

Main unrelated work preserved; clean Appearance workspace reused. Existing Vite HMR and persistent dev fixture retained.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/projects-directories.test.ts
```

**Notes**

4 tests pass: unsupported local profiles perform no review reads, native failure can refresh successfully, continuations and permission removal retain their checks. /tmp/jj-ui-polish-unit.log.
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
pnpm web:check
```

**Notes**

0 errors and warnings. /tmp/jj-ui-polish-web-check.log.
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
pnpm catalog:check (initial)
```

**Notes**

New Storybook role queries incorrectly supplied Playwright-only exact option. Removed options; production check passes.
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
pnpm catalog:check (corrected)
```

**Notes**

0 errors and warnings; 129 catalog modules. /tmp/jj-ui-polish-catalog-check-final.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/ProjectsHub.stories.svelte catalog/components/store-work/StoreWork.stories.svelte catalog/components/store-application/StoreApplication.stories.svelte catalog/components/store-appearance/StoreAppearance.stories.svelte -t "UnsupportedReviewsEmptyState|ReviewLoadFailureRetainsRetry|ProjectsRepositoriesAndTaskNavigation|ReadOnlyTask|HierarchyAndDiscussion|UnknownOutcome|NativeThreadedDiscussion|^Settings$|SettingsCollapsed|SettingsMobile|AllControlsAndLocalLinks" (initial)
```

**Notes**

10 tests pass; new Empty icon assertion used empty-media as data-slot instead of the public data-ui-part. Corrected selector; no implementation workaround.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/ProjectsHub.stories.svelte catalog/components/store-work/StoreWork.stories.svelte catalog/components/store-application/StoreApplication.stories.svelte catalog/components/store-appearance/StoreAppearance.stories.svelte -t "UnsupportedReviewsEmptyState|ReviewLoadFailureRetainsRetry|ProjectsRepositoriesAndTaskNavigation|ReadOnlyTask|HierarchyAndDiscussion|UnknownOutcome|NativeThreadedDiscussion|^Settings$|SettingsCollapsed|SettingsMobile|AllControlsAndLocalLinks" --reporter verbose
```

**Notes**

11 exact stories pass with runtime/accessibility guard. Other21 stories intentionally skipped; not a full-suite claim. /tmp/jj-ui-polish-stories-final.log.
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
node /tmp/jj-ui-polish-live.mjs (initial and corrected); node /tmp/jj-ui-polish-contrast.mjs
```

**Notes**

Initial live icon selector probe failed for same incorrect data-slot; corrected to public data-ui-part. Isolated read-only Chromium verifies unsupported icon/no retry, Activity after Subtasks, comments/composer in discussion, single title-first Appearance header, description, tab placement and exact route/snapshot retention through desktop/mobile sidebar return. No runtime errors. Supplemental first task axe probe reported2 transient contrast nodes; independent probe and final settled rerun both report0 violations. /tmp/jj-ui-polish-live-final.log and contrast.log. Existing user browser and native data untouched.
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

21 chapters,147 requirements validated; no Community or catalog dependency added.
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
pnpm exec prettier --write (changed supported files); pnpm exec prettier --check (same); pnpm spec:first; pnpm spec:validate; pnpm check:progress
```

**Notes**

Canonical requirements, verification map and same-change ledger. Detailed /tmp/jj-ui-polish-* logs.
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

UI-only projects reviews/task discussion and Settings presentation. Exact units/stories, web/catalog type checks, ownership and read-only live Vite desktop/mobile acceptance are the complete impact surface. Browser fixtures updated for sidebar return; root aggregate lanes, Rust builds/tests, unrelated story families, fixture resets, native writes and production built-CSP qualification out of scope. M16 remains Partial with prior release/registry-only/composer-CSP/visual gates retained. No baseline approval.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Node26.9.0; pnpm10.32.1; Svelte5.57.1; Vite8.3.2; Vitest4.1.11; Storybook10.5.7; existing Design Core Empty and Collapsible. Dependencies unchanged.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No native/API/storage or preference format change. Owned review view distinguishes unsupported boolean from transport error. Section title is the only page heading. Thread drafts, native comment focus, repository activity scope and captured snapshots unchanged.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Continue the newly requested table-style alignment of Local service, Search and Sync Settings in a separate verified slice.
<!-- /evidence-field -->

<!-- evidence-field:artifacts markdown -->
## Artifacts

/tmp/jj-ui-polish-*.log/json/png and live/contrast probe scripts. Isolated browser contexts closed; persistent Vite/backend fixture retained.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


