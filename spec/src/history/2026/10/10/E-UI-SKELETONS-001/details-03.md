---
schema: 1
id: E-UI-SKELETONS-001
---

# Verification follow-ups

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
pnpm exec tsx --test web/tests/lazy-view.test.ts
```

**Notes**

2/2 pass: shared demand/completion and explicit retry after rejected imports. The initial command also supplied a nonexistent projects-initial-loading.test.ts path; tsx ran only these same 2 tests. The final command names only the real test file. Logs: .local/skeleton-units.log and .local/skeleton-lazy-unit.log.
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
node --test scripts/web-manifest.test.mjs
```

**Notes**

3/3 pass. Browser asset closure and manifests still retain dynamically loaded views. Log: .local/skeleton-assets.log.
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
pnpm web:build
```

**Notes**

Initial Vite/manifest build succeeded, but documentation staging rejected an outdated application review fingerprint. Updated the browsing guide and ran pnpm docs:review application with a specific skeleton review. Full rerun including documentation staging passed. Logs: .local/skeleton-build.log, .local/skeleton-build-rerun.log and .local/skeleton-build-final.log.
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

Initial: 0 errors and one unused grid CSS selector warning. Corrected the selector, then reused the actual owned row/grid classes. Subsequent checks: 0 errors, 0 warnings. Logs: .local/skeleton-web-check.log, .local/skeleton-web-check-final.log and .local/skeleton-web-type.log.
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
pnpm catalog:check
```

**Notes**

A new test supplied Playwright exact:true to Testing Library, which rejected it during type checking. Replaced it with an anchored name regex. Final catalog check: 0 errors/warnings, catalog boundaries pass (377 modules). Logs: .local/skeleton-catalog-check-final.log and .local/skeleton-catalog-type.log.
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
pnpm check:ui-ownership
```

**Notes**

Owned composition/provenance boundary passes. No Community runtime, build, test or type dependency was introduced. Logs: .local/skeleton-ownership.log and .local/skeleton-ownership-final.log.
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
pnpm test:storybook catalog/components/store-work/ProjectsActivity.stories.svelte catalog/components/store-work/ProjectsAgents.stories.svelte catalog/components/store-application/RepositoryLoading.stories.svelte catalog/components/store-application/AgentSessions.stories.svelte
```

**Notes**

Final settled source: 41/41 interaction/accessibility stories pass in 4 families, including the new physical grid assertions and persistent gallery fixture. After the final history-selection fallback change, the directly affected RepositoryLoading family passes 18/18 in a fresh process. Logs: .local/skeleton-stories-verified.log and .local/skeleton-history-stories.log. Visual baselines remain unapproved.
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
pnpm exec playwright test --config web/playwright.config.ts web/tests/browser/lazy-views.spec.ts
```

**Notes**

Final cold built run: 28/28 pass, including selected history diff panes. Fresh contexts validate module/data phases, retries, obsolete imports, stored geometry and mobile regions. No command dispatch was performed. Log: .local/skeleton-history-browser.log.
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
pnpm web:build
```

**Notes**

The final history UI build and manifest succeeded, but concurrent docs:check cleaned the shared generated documentation while staging validated it. The closure check saw a transient empty directory and rejected it with the shared 512-file guard. The concurrent docs:check completed successfully (141 files). Run documentation/build mutations sequentially; final full build rerun is recorded separately. Logs: .local/skeleton-history-build.log and .local/skeleton-docs-check.log.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 7
hasNote: true
noteEncoding: markdown
```

```shell
pnpm check:progress
```

**Notes**

Initial evidence rejected 7 invented or unsupported test-reference names; a second pass rejected 5 Story names because the checker recognizes fn/def/test definitions. Removed invalid references and retained the exact real browser, unit and asset tests; Story names and family results remain in the actual run records. No checker was weakened.
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
pnpm web:build
```

**Notes**

Final sequential build passes Vite, asset manifest and documentation staging: 141 documentation files, 22,487,135 bytes, valid links/anchors. Existing large-chunk and mixed Tauri import warnings remain; no build error. Log: .local/skeleton-build-verified.log.
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
pnpm web:check
pnpm catalog:check
pnpm check:ui-ownership
pnpm docs:check
pnpm check:progress
pnpm spec:validate
pnpm spec:first
```

**Notes**

Final web/catalog checks have zero errors and warnings; catalog boundaries and UI ownership pass. Documentation closure passes. Progress validates 146 tasks and 742 records; bookkeeping is distinct from product qualification. Specification validates 7 validators, 3,527 chapters and 181 requirements. Initial spec:first failed 3 missing mapped chapter confirmations; the fresh unchanged-contract record now includes API, documentation and governance IDs and the gate passes without modifying its checker. Logs: .local/skeleton-final-web.log, .local/skeleton-final-catalog.log, .local/skeleton-final-ownership.log, .local/skeleton-docs-check.log, .local/skeleton-progress-final.log, .local/skeleton-spec-validate.log and .local/skeleton-spec-first-final.log.
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
pnpm exec prettier --check catalog/components/store-application/AgentSessions.stories.svelte catalog/components/store-application/RepositoryLoading.stories.svelte catalog/components/store-work/ProjectsActivity.stories.svelte docs/maintenance/coverage.json docs/site/guides/browsing.md docs/ui-provenance.json spec/src/current/contracts/ui/07-application-startup.md spec/src/current/progress/M24/index.md spec/src/current/progress/M24/view-loading-review.md spec/src/history/2026/10/10/E-UI-SKELETONS-001/SUMMARY.nav.md spec/src/history/2026/10/10/E-UI-SKELETONS-001/details-02.md spec/src/history/2026/10/10/E-UI-SKELETONS-001/details-03.md spec/src/history/2026/10/10/E-UI-SKELETONS-001/index.md spec/src/history/2026/10/10/SUMMARY.nav.md spec/src/history/2026/10/10/index.md web/src/lib/ui/community-agents/CommunityAgents.svelte web/src/lib/ui/community-inbox/CommunityInboxSkeleton.svelte web/src/lib/ui/community-projects/CommunityProjectsRepositoryFiles.svelte web/src/lib/ui/community-skeleton/CommunitySkeleton.svelte web/src/lib/ui/store-application/StoreApplicationAgents.svelte web/src/lib/ui/store-application/StoreApplicationMain.svelte web/src/lib/ui/store-application/StoreLazyView.svelte web/src/lib/ui/store-application/StorePageSkeleton.svelte web/src/lib/ui/store-application/StoreRepositoryApplication.svelte web/src/lib/ui/store-work/StoreProjects.svelte web/src/lib/ui/store-work/StoreProjectsDirectory.svelte web/src/lib/ui/store-work/StoreProjectsLoading.svelte web/src/lib/ui/store-work/StoreProjectsRepositories.svelte web/src/lib/ui/store-work/StoreProjectsReviews.svelte web/src/lib/ui/store-work/StoreWorkRowsSkeleton.svelte web/src/lib/ui/store-work/StoreWorkTaskSkeleton.svelte web/tests/browser/lazy-views.spec.ts
```

**Notes**

The command receives the explicit path list from jj diff --name-only, without a repository-wide formatter run. Authored evidence stays within 400 physical lines and 32,768 bytes per page. Discovery initially rejected pnpm spec:search -- JST-UI-005; pnpm spec:search JST-UI-005 succeeded and the returned startup/requirements sources were read.

Live temporary browser tab 24 held the Threads and agent-session module requests, showing both list and conversation regions. Threads retained list width 314.4375px and body width 907.5625px, both 662px high, before and after module completion. Screenshots: .local/thread-loading-reviewed.png and .local/agent-loading-reviewed.png. All intercepts and cache overrides were removed and the temporary tab closed. Native fixture and user tab remain retained. Runtime metadata identifies the default workspace in Vite mode on port 7460.
<!-- /run -->

<!-- /evidence-field -->
