---
schema: 1
id: E-UI-063
---

# E-UI-063 details 4

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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook --shard 3/3
```

**Notes**

Repaired final shard65/65 in17 files79.52s. Combined three completed shards account for all53 files/196 stories, all passing interaction/accessibility/runtime acceptance. No provider error or skipped tests in shard inventory. Existing Design Core source-validation setup used; registry-only qualification remains separate. /tmp/jj-auto-refresh-stories-shard-3-after.log.
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
pnpm exec prettier --write catalog/components/store-application/appearance-tool-discussion-play.ts catalog/components/store-threads/StoreThreads.stories.svelte catalog/components/store-application/StoreFileEditor.stories.svelte spec/src/milestones.md; pnpm exec prettier --check (71 changed files; lockfile ignored by existing policy); pnpm catalog:check; pnpm check:ui-ownership; pnpm spec:validate; pnpm check:progress
```

**Notes**

Formatting passes; catalog0 errors/warnings129 modules; ownership1 validator and canonical5 validators/21 chapters/147 requirements; progress120 tasks. Logs format-suite-final, format-all-final-check, catalog-all-final, ownership-final-complete, spec-validate-all-final and progress-all-final. Registry lock audit passes after narrowing an overbroad substring probe that matched vfile/link package names; history evidence bytes unchanged. Read-only source/installed dependency inspection confirms clean Design Core source pin and its actual runtime Bits UI2.19.4; first two introspection scripts assumed unsupported package.json export/location and were corrected, not product failures.
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
pnpm spec:first
```

**Notes**

Shared package/lockfile change requires mapped spec/src/architecture.md, which was missing from this slice. Added released overlay version/teardown requirement and separate source-validation versus registry-release handling; no gate relaxation. /tmp/jj-auto-refresh-spec-first-all-final.log.
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
pnpm exec prettier --write spec/src/architecture.md
```

**Notes**

Owning dependency chapter formatted; final spec-first/ledger/formatter checks follow. /tmp/jj-auto-refresh-format-architecture-final.log.
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
pnpm spec:first; pnpm spec:validate; pnpm check:progress; pnpm exec prettier --check (72 changed supported files, existing lockfile ignore)
```

**Notes**

Final mapped canonical chapters, evidence bookkeeping and formatting pass after dependency architecture entry; see spec-first-complete-final, spec-validate-complete-final, progress-complete-final and format-complete-final-check logs. Historical evidence bytes retained. Browser functional/a11y suites use explicitly recorded source-validation setup; registry-only gate remains pending. Ledger-only final result insertion rechecks progress and formatting before commit.
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

UI read scheduling across WorkStore, Projects, repository, Threads, Agents, Reviews and Settings. Shared Bits UI dependency update escalates to complete UI unit/web test and Storybook interaction/accessibility suites; sequential fresh-browser shards complete the full story inventory. Cold built-UI/mock acceptance is independent from two-client live native/HMR acceptance. Task/project stale-edit browser case simulates read-boundary changes and refuses writes; it is not native mutation concurrency qualification. Existing private Design Core source validation is explicitly temporary: registry-only web check and UI story failures reproduce existing missing public APIs, and M16 source/release qualification remains pending. Tracked manifests/lock remain registry-only. Root aggregate pnpm spec:check/check/test/build, Rust builds/tests, full native/local/Git-bridge suites, production CSP/release qualification, main landing/publication and visual baselines remain out of scope. Maintained native/Git bridge specs are not newly executed qualifications. Retired manual-only test names map to automatic-refresh regressions; historical evidence stays unchanged.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Node26.9.0; pnpm10.32.1; Svelte5.57.1; Vitest4.1.11; Vite8.3.2; Playwright1.63.0; Storybook10.5.7. Released Bits UI2.19.4 replaces direct2.18.1/nested2.19.3. Design Core tracked range remains registry ^0.2.0; clean released0.2.0 qualification fails existing APIs. Temporary private validation uses clean Design Core source package0.1.2, immutable59f25e0792875f99ba57d60c7da14cf5070166ce. Mira dependency unchanged.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No native, HTTP/schema, storage, asset-manifest, URL or preference formats changed. Optional StoreController.actions.setRefreshPaused is UI-only. Serial150ms coalescing and5s visible-page checks use bounded directory/home reads with at most2 concurrent probes. Fresh opaque cursors replay retained bounded windows. Current-head code may advance; explicit historical operations/commits remain selected while independent collaboration roots update. Drafts, exact unknown requests/preconditions, task/project form opening versions and dirty Search configuration revisions remain memory-only. Browser reads cannot publish or reconcile native state.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Use running Appearance Vite app for UI iteration. M16 registry-only Design Core/source-release qualification remains pending; source owner has existing API additions not present in released0.2.0. Preserve backend/fixture and main unrelated work; main landing, dependency publication and visual baseline approval remain separate.
<!-- /evidence-field -->

<!-- evidence-field:artifacts markdown -->
## Artifacts

Committed unit/web, story and browser regressions; /tmp/jj-auto-refresh-*.log. Complete Storybook inventory196/196 in53 files; catalog units238/238 and web tests70/70; cold built UI5/5 and live native/HMR2/2. Temporary preview7458 stopped and test browser contexts closed. Existing Vite7460 PID52247/backend7462 PID36217 and persistent fixture retained.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- AGENTS.md
- catalog/components/store-application/StoreFileEditor.stories.svelte
- catalog/components/store-application/StoreSyncSettings.stories.svelte
- catalog/components/store-application/appearance-tool-discussion-play.ts
- catalog/components/store-threads/StoreThreads.stories.svelte
- catalog/components/store-work/ProjectsHub.stories.svelte
- catalog/components/store-work/StoreWork.stories.svelte
- catalog/fixtures/StoreWorkFixture.svelte
- catalog/fixtures/SyncSettingsFixture.svelte
- catalog/fixtures/projects-hub-layout.ts
- package.json
- pnpm-lock.yaml
- spec/src/api.md
- spec/src/architecture.md
- spec/src/local-jj.md
- spec/src/milestones.md
- spec/src/projects.md
- spec/src/spec-governance.md
- spec/src/ui.md
- spec/src/verification.md
- web/src/lib/controller/automatic-refresh.ts
- web/src/lib/controller/collaboration-refresh.ts
- web/src/lib/controller/contracts.ts
- web/src/lib/controller/production-automatic-refresh.ts
- web/src/lib/controller/production-projects-hub.svelte.ts
- web/src/lib/controller/production-projects-integration.ts
- web/src/lib/controller/production-search-settings.ts
- web/src/lib/controller/production-store-collaboration.svelte.ts
- web/src/lib/controller/production-store-controller.svelte.ts
- web/src/lib/controller/production-store-events.ts
- web/src/lib/controller/production-store-refresh.ts
- web/src/lib/controller/production-store-view.ts
- web/src/lib/controller/production-sync-settings.ts
- web/src/lib/controller/production-work-integration.ts
- web/src/lib/controller/production-work.svelte.ts
- web/src/lib/controller/production-workspace-links.ts
- web/src/lib/controller/projects-activity.svelte.ts
- web/src/lib/controller/projects-home-read.ts
- web/src/lib/controller/projects-navigation.svelte.ts
- web/src/lib/controller/projects-reviews.svelte.ts
- web/src/lib/controller/projects-work-registry.svelte.ts
- web/src/lib/controller/store-refresh-pages.ts
- web/src/lib/controller/store-route-history.svelte.ts
- web/src/lib/controller/work-refresh-pages.ts
- web/src/lib/controller/work-types.ts
- web/src/lib/ui/store-application/StoreApplication.svelte
- web/src/lib/ui/store-application/StoreApplicationHeaderActions.svelte
- web/src/lib/ui/store-mutations/StoreRepositoryActions.svelte
- web/src/lib/ui/store-settings/StoreLocalFolders.svelte
- web/src/lib/ui/store-settings/StoreSearchHealth.svelte
- web/src/lib/ui/store-settings/StoreServiceSettings.svelte
- web/src/lib/ui/store-settings/StoreSettings.svelte
- web/src/lib/ui/store-settings/StoreSyncSettings.svelte
- web/src/lib/ui/store-work/StoreProjects.svelte
- web/src/lib/ui/store-work/StoreWorkProjectForm.svelte
- web/src/lib/ui/store-work/StoreWorkTaskForm.svelte
- web/tests/browser/automatic-refresh.spec.ts
- web/tests/browser/native-browser-fixture.ts
- web/tests/browser/navigation.spec.ts
- web/tests/browser/reads.spec.ts
- web/tests/real/automatic-updates.spec.ts
- web/tests/real/git-bridge.spec.ts
- web/tests/real/native-repository.spec.ts
- web/tests/real/work-store.spec.ts
- web/tests/vite/automatic-refresh.test.ts
- web/tests/vite/automatic-store-refresh.test.ts
- web/tests/vite/automatic-work-refresh.test.ts
- web/tests/vite/production-automatic-refresh.test.ts
- web/tests/vite/production-search-settings.test.ts
- web/tests/vite/production-store-events.test.ts
- web/tests/vite/production-sync-settings.test.ts
- web/tests/vite/store-route-history.test.ts
<!-- /evidence-field -->

<!-- evidence-field:removed_tests list -->
## Removed tests

- web/tests/browser/reads.spec.ts#event_reconnect_resumes_and_reset_requires_explicit_refresh
- web/tests/real/native-repository.spec.ts#external publication waits for
  explicit refresh and preserves selection
- web/tests/vite/production-store-events.test.ts#collaboration publication
  events announce freshness without mixing snapshots
- web/tests/real/work-store.spec.ts#native projects tasks and discussion persist
  on RustFS and refresh explicitly
- web/tests/real/git-bridge.spec.ts#Git import waits for refresh
<!-- /evidence-field -->

