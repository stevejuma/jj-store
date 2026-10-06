---
schema: 1
id: E-UI-063
---

# E-UI-063 details 2

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
jj --no-pager st; jj describe -m "Refresh application data automatically after saves and live updates"; scoped requirements/source/UI reference inspection; jj --no-pager diff --summary/stat/git
```

**Notes**

Reuse existing Appearance workspace; preserve main and fixture. No Community dependency/provenance changes. Exploratory missing source/spec/config paths and a no-write git status attempt were corrected to scoped inventories/Jujutsu; one guarded helper extraction substring mismatch was repaired before validation.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit (automatic refresh, WorkStore, Projects, events, sync/search, Threads, collaboration, repository directory and native/local consumers; units1, units2, units3, units-final and units-complete)
```

**Notes**

Initial 24/24, two78/78,97/97 and98/98 pass. Focused queue coalescing/races, keyed pauses, session cancellation, periodic/reconnect checks, own save/status reload, draft/pending identities and current-head/historical guards. /tmp/jj-auto-refresh-units*.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/automatic-refresh.test.ts web/tests/vite/production-automatic-refresh.test.ts web/tests/vite/automatic-work-refresh.test.ts web/tests/vite/automatic-store-refresh.test.ts web/tests/vite/production-work.test.ts web/tests/vite/projects-hub.test.ts web/tests/vite/production-store-events.test.ts web/tests/vite/production-sync-settings.test.ts web/tests/vite/production-search-settings.test.ts web/tests/vite/production-threads.test.ts web/tests/vite/production-store-collaboration.test.ts web/tests/vite/projects-repositories.test.ts web/tests/vite/local-browser.test.ts web/tests/vite/local-browser-merge.test.ts web/tests/vite/store-route-history.test.ts web/tests/vite/projects-code-navigation.test.ts web/tests/vite/production-store-data-source.test.ts web/tests/vite/production-store-settings.test.ts web/tests/vite/production-store-evolution.test.ts web/tests/vite/production-store-workspaces.test.ts
```

**Notes**

Final151/151 in20 direct controller test files. Adds fresh-cursor sidebar replay, canonical browser navigation and dirty Search draft revision retention/cancel adoption to existing request/session/authorization protections. /tmp/jj-auto-refresh-units-final-qualified.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/StoreWork.stories.svelte catalog/components/store-work/ProjectsHub.stories.svelte -t 'HierarchyAndDiscussion|UnknownOutcome|NativeThreadedDiscussion|ProjectsRepositoriesAndTaskNavigation|ReadOnlyTask|TaskEditAndRetainedUnknownRequest|MobileTaskDetail|RepositoryLinks'
```

**Notes**

Two runs:8 directly affected stories pass,8 unrelated skipped. Actual interaction/accessibility/runtime checks retained. /tmp/jj-auto-refresh-stories1.log and stories-complete.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreApplication.stories.svelte catalog/components/store-application/StoreSearchSettings.stories.svelte catalog/components/store-application/StoreSyncSettings.stories.svelte catalog/components/store-application/StoreWorkspaceSettings.stories.svelte -t 'Settings|Default|Native|Local'; same four files without title filter
```

**Notes**

First9/9 selected pass with17 skipped; complete directly affected Settings families26/26 pass. Optional recovery controls, dirty drafts, unsupported/failed status, mobile/collapsed navigation remain qualified. /tmp/jj-auto-refresh-settings-stories.log and settings-verified.log.
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
JJ_STORE_REAL_BASE_URL=http://127.0.0.1:7460 JJ_STORE_REAL_REPOSITORY=1bdb89c409c1b891ac7f180c01a265d5 JJ_STORE_REAL_PROJECT=38d1c167c245c42fd7d1818086a89ed1 JJ_STORE_REAL_TASK=c8a8573c0204be10644dcd77d16df2b8 pnpm exec playwright test --config web/playwright.real.config.ts web/tests/real/automatic-updates.spec.ts
```

**Notes**

Initial own/external update test1/1 and final2/2 pass against real native backend and HMR UI. Two clients observe committed comments without refresh; observer draft survives; stale banners absent; whole-page WCAG axe0 violations and0 runtime errors. Test comments are tombstoned in finally; historical receipts remain. Second case only intercepts GET metadata/version advances and refuses captured POST with stale_work: proves task/project forms retain original versions/drafts without real metadata writes. No fixture reset/backend rebuild. /tmp/jj-auto-refresh-live1.log and live-final.log.
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
pnpm exec playwright test --config /tmp/jj-auto-refresh-browser.config.mjs web/tests/browser/automatic-refresh.spec.ts
```

**Notes**

First cold run cannot import non-exported referencesPage; no product test executes. Exported existing fixture helper and reran. /tmp/jj-auto-refresh-cold1.log.
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
pnpm exec playwright test --config /tmp/jj-auto-refresh-browser.config.mjs web/tests/browser/automatic-refresh.spec.ts
```

**Notes**

2/2 built-UI/mock boundary cases pass: current head advances automatically, historical operation remains pinned while fresh reads occur. Fresh isolated preview7458; no Rust fixture/build. /tmp/jj-auto-refresh-cold2.log.
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
pnpm exec playwright test --config /tmp/jj-auto-refresh-browser.config.mjs web/tests/browser/automatic-refresh.spec.ts web/tests/browser/navigation.spec.ts web/tests/browser/reads.spec.ts -g 'new repository data|hash route restores|multiple_heads_require_choice|event_reconnect_resumes'
```

**Notes**

First broadened5-case run has4 pass/1 fail: repository Back fails to recognize internal optional route fields in its URL-restored destination, disables Forward. Added exact unit before changing canonical route matching. /tmp/jj-auto-refresh-cold-final.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/store-route-history.test.ts -t 'refreshed repository routes retain Forward' (before and after)
```

**Notes**

Before1 failure/10 skipped: canForward=false after browser Back. After canonical URL comparison1 pass/10 skipped. Full11 route-history cases pass in direct/final lanes. /tmp/jj-auto-refresh-navigation-{before,after}.log.
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
pnpm exec playwright test --config /tmp/jj-auto-refresh-browser.config.mjs web/tests/browser/navigation.spec.ts -g 'hash route restores'; pnpm exec playwright test --config /tmp/jj-auto-refresh-browser.config.mjs web/tests/browser/automatic-refresh.spec.ts web/tests/browser/navigation.spec.ts web/tests/browser/reads.spec.ts -g 'new repository data|hash route restores|multiple_heads_require_choice|event_reconnect_resumes'
```

**Notes**

Exact repaired navigation1/1 passes, then fresh built UI5/5 pass: new current/history updates, hash/back/forward/reload, multiple-head choice and reconnect cursor/reset automatic recapture. /tmp/jj-auto-refresh-navigation-cold-after.log and cold-complete.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/automatic-work-refresh.test.ts -t 'sidebar refresh retains' (before and after)
```

**Notes**

Before1 failure/4 skipped: refresh dropped second loaded project page. After reloadProjectHomeWindow reuse1 pass/4 skipped, fresh-root cursor and retained project window verified. Final unit lane passes all. /tmp/jj-auto-refresh-sidebar-{before,after}.log.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/production-search-settings.test.ts -t 'automatic status reads retain' (before); pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/production-search-settings.test.ts web/tests/vite/production-store-settings.test.ts web/tests/vite/production-store-workspaces.test.ts (after)
```

**Notes**

Before1 failure/4 skipped: new status revision b replaces dirty draft base a. Private draftRevision retains original precondition until accepted Save/Cancel. After20/20 direct settings/workspace units pass; final151-unit lane includes them. /tmp/jj-auto-refresh-search-draft-{before,after}.log.
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
pnpm exec tsx --test web/tests/store-route-codec.test.ts web/tests/production-store-loaders.test.ts; pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/store-route-history.test.ts web/tests/vite/store-route-codec.test.ts web/tests/vite/projects-route.test.ts web/tests/vite/automatic-store-refresh.test.ts web/tests/vite/production-automatic-refresh.test.ts; pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/projects-code-navigation.test.ts web/tests/vite/production-store-data-source.test.ts web/tests/vite/production-store-settings.test.ts web/tests/vite/production-store-evolution.test.ts
```

**Notes**

20 route/loaders,17 actual matching unit cases and31 direct code navigation/data/settings/evolution cases pass. Two nonexistent extra Vitest filter paths matched no files; three real selected files ran17 tests. Final lane uses actual files only. /tmp/jj-auto-refresh-route-codec.log, navigation-consumers.log and route-direct-consumers.log.
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
pnpm web:check; pnpm catalog:check; pnpm check:ui-ownership (initial and final source rounds)
```

**Notes**

All actual web/catalog checks report0 errors/warnings and129 catalog modules. Ownership passes1 validator,21 chapters/147 requirements. Final source web/catalog logs /tmp/jj-auto-refresh-{web,catalog}-final-qualified.log; ownership-{,complete}.log.
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:architecture (initial unredirected check, architecture.log, architecture2.log and architecture4.log)
```

**Notes**

Module limits caught root/Work/Hub growth and inherited401-line story, then311-line collaboration module. Extracted automatic refresh, Work page/cache helpers, home-window replay and exact collaboration matching; moved existing read-only story play helper. No limit or assertion relaxation. Three recorded failed logs plus first unredirected attempt.
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
pnpm check:architecture (architecture3, architecture5, final, complete, verified and final-qualified); pnpm web:build (build, build-final, build-final2, build-complete, build-verified and build-final-qualified)
```

**Notes**

Architecture passes final735 modules. Fresh Vite assets and existing manifest generated successfully on each build; existing &gt;500k chunk advisory remains. Latest final-qualified build6.08s; no Rust build/restart. /tmp/jj-auto-refresh-{architecture*,build*}.log.
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
pnpm exec prettier --write (changed supported source/docs files, initial through format-final)
```

**Notes**

Changed-file formatting only. Final source paths are from jj diff --name-only. Canonical ledger/map gates are recorded after this evidence insertion. /tmp/jj-auto-refresh-format*.log.
<!-- /run -->
<!-- /evidence-field -->

