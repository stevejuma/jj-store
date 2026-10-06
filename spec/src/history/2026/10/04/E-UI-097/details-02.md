---
schema: 1
id: E-UI-097
---

# E-UI-097 details 2

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
jj --no-pager st; jj workspace list; source: jj rebase -r @ -o lqkpxqtl; consumer: jj new @- kkoupnws -m "Integrate notification toasts into main"
```

**Notes**

Clean workspaces confirmed. Design Core main tree is byte-identical to its qualified feature revision. Eight jj-store conflicts were resolved by retaining both notification behavior and newer main presentation; no unrelated workspaces or bookmarks moved.
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
Initial Python conflict-resolution script; jj restore spec/src/api.md web/src/lib/ui/store-application/StoreApplication.svelte web/src/lib/ui/store-settings/StoreSearchHealth.svelte web/src/lib/ui/store-settings/StoreSettingsRepository.svelte web/src/lib/ui/store-settings/StoreStorageSettings.svelte web/src/lib/ui/store-settings/StoreSyncSettings.svelte; corrected conflict resolution
```

**Notes**

Initial marker pattern was too broad and the script stopped at the ledger. Restored only the six files it edited from the automatic parent merge, then used line-bounded markers. Final review, clean conflict state and functional checks below confirm the corrected result; no tests ran against the incomplete edit.
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
Python JSON comparison of merged ledger against uxoswqxv and kkoupnws
```

**Notes**

All313 original/merged evidence entries unique; every main evidence object preserved verbatim, and feature entry retained verbatim except its necessary E-UI-096 identifier.
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
pnpm install --offline --ignore-scripts --lockfile=false
```

**Notes**

Refreshed existing file:../design-core validation dependency from main; manifests/lockfile unchanged. No runtime dependency points to either retired workspace. Install log copied into .local/notifications-integration/install.log.
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
pnpm exec prettier --write spec/src/api.md spec/src/milestones.md web/src/lib/ui/store-application/StoreApplication.svelte web/src/lib/ui/store-settings/StoreSearchHealth.svelte web/src/lib/ui/store-settings/StoreSettingsRepository.svelte web/src/lib/ui/store-settings/StoreStorageSettings.svelte web/src/lib/ui/store-settings/StoreSyncSettings.svelte web/src/lib/ui/store-threads/StoreThreadSettings.svelte
```

**Notes**

.local/notifications-integration/format.log.
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
pnpm web:check; pnpm catalog:check
```

**Notes**

Both type checks0errors/0warnings;166catalog modules. .local/notifications-integration/types.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/notification-service.test.ts web/tests/vite/production-search-settings.test.ts web/tests/vite/production-store-workspaces.test.ts
```

**Notes**

22/22tests in three files; .local/notifications-integration/unit.log.
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
pnpm test:storybook catalog/components/store-application/StoreNotifications.stories.svelte catalog/components/store-application/StoreApplication.stories.svelte catalog/components/store-application/StoreSearchSettings.stories.svelte catalog/components/store-application/StoreStorageSettings.stories.svelte catalog/components/store-application/StoreSyncSettings.stories.svelte catalog/components/store-application/StoreWorkspaceSettings.stories.svelte catalog/components/store-application/StoreSettingsIdentifier.stories.svelte catalog/components/store-threads/StoreThreadAlignment.stories.svelte
```

**Notes**

41/41interaction/accessibility stories across eight directly merged surfaces. .local/notifications-integration/stories.log.
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
pnpm check:architecture; pnpm check:ui-ownership; pnpm web:build
```

**Notes**

859architecture modules with unchanged budgets; ownership and fresh production bundle pass. .local/notifications-integration/boundaries.log and build.log.
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
pnpm exec playwright test --config .local/notifications-playwright.config.ts notifications.spec.ts local-search-settings.spec.ts local-storage.spec.ts local-sync.spec.ts local-workspaces.spec.ts threads-alignment.spec.ts sidebar-repository-actions.spec.ts history-query.spec.ts
```

**Notes**

11/11cold built-UI tests under exact production CSP on fresh isolated127.0.0.1:7568server; mocked captured API responses. Covers dismissal/recovery, Settings drafts/copy actions, unknown storage/sync recovery, thread rename/reply, sidebar creation and both history grammars. .local/notifications-integration/browser.log. This is UI evidence, not native/storage qualification.
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
pnpm spec:first
```

**Notes**

Tool default @- is ambiguous on a merge. .local/notifications-integration/spec-first.log. No spec requirement weakened.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first --base uxoswqxv --head @
```

**Notes**

Explicit range uses Git read-only diff and requires commit hashes rather than JJ change IDs. .local/notifications-integration/spec-first-main.log.
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
pnpm spec:first --base d4a70441 --head 2132b4b67ba3996c9df31dc80b5afdad904be195; pnpm spec:first --base 2c33e89d --head 2132b4b67ba3996c9df31dc80b5afdad904be195; pnpm spec:validate; pnpm check:progress
```

**Notes**

Both parent ranges qualify:50protected files/4chapters from main and53/3from feature;5validators/22chapters/153requirements;126-task bookkeeping. .local/notifications-integration/spec-first-{main,feature}-sha.log, spec-validate.log and progress.log.
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
node .local/notifications-integration/restart-ui.mjs; node .local/notifications-integration/preview-smoke.mjs
```

**Notes**

Owned main Vite restart succeeded but first read-only shell smoke timed out: cached optimized Mira imports referred to prior pnpm paths. .local/notifications-integration/restart-ui.log and preview-smoke.log.
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
node .local/notifications-integration/restart-ui.mjs --refresh-cache; node .local/notifications-integration/preview-smoke.mjs
```

**Notes**

Moved ignored generated .vite cache aside and restarted only owned main UI.127.0.0.1:7460shell loads with zero page errors. Backend PID18306, native desktop and other preview services unchanged. .local/notifications-integration/restart-ui-fresh.log and preview-smoke-fresh.log.
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
lsof -d cwd -Fn; verified SIGTERM of temporary static servers74475/80809/81899/81928; jj workspace forget notifications in both repositories; remove exactly jj-store-notifications and design-core-notifications
```

**Notes**

Found and stopped four task-owned leftover Python Storybook static hosts before retirement. Both source revisions remain ancestors of main. Verified clean temporary workspaces and copied42consumer plus26source ignored artifacts byte-for-byte into each main .local/notifications-integration/source-evidence directory, including source package tarball. Other workspaces retained.
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
pnpm exec prettier --check merged files; pnpm check:progress; pnpm spec:validate; pnpm spec:first --base d4a70441 --head 70d42bcb; pnpm spec:first --base 2c33e89d --head 70d42bcb
```

**Notes**

Final merge record formatting,126-task bookkeeping,5validators/22chapters/153requirements and both parent spec-first ranges pass. Both temporary paths are absent and their workspace registrations removed; other workspaces remain. .local/notifications-integration/{format-final,progress-final,spec-final,spec-first-final-main,spec-first-final-feature}.log.
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

User-authorized main integration and temporary-workspace retirement. Existing source qualification is inherited unchanged; only merged consumer surfaces and their direct regressions rerun under targeted UI policy. Root aggregates, Rust/native/storage qualification, screenshot comparisons and visual baselines are out of scope. M4.31 remains in progress for released Design Core registry consumption; no milestone promoted and no package published.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Retain main notification regressions. Release and registry-only consumer acceptance remain the separately governed M4.31 follow-up; OS delivery and persistent history remain deferred.
<!-- /evidence-field -->

