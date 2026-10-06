---
schema: 1
id: E-M21-044
---

# E-M21-044 details 2

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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/store-stacks-navigation.test.ts web/tests/vite/store-route-history.test.ts web/tests/vite/production-stacks-application.test.ts web/tests/vite/production-stacks-lifecycle.test.ts
```

**Notes**

Initial focused runs culminated in 37 passing tests; new pending-read regression below extends the final count.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/production-stacks-lifecycle.test.ts -t "binding revalidation preserves"
```

**Notes**

pending-selection-before.log reproduces a second workspace read replacing an in-flight selection.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/store-stacks-navigation.test.ts web/tests/vite/store-route-history.test.ts web/tests/vite/production-stacks-application.test.ts web/tests/vite/production-stacks-lifecycle.test.ts web/tests/vite/projects-code-navigation.test.ts
```

**Notes**

units-final-complete.log: 45 tests in five files. Includes exact entry restoration, four views and route fields, independent history selections, browser history/reload, repository-wide graph scope, cold context startup, bounded canvas switches, cleared obsolete history feedback, pending selection preservation and binding/auth disposal. Intermediate four-file runs passed 38; direct navigation consumer seven pass.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceApplication.stories.svelte
```

**Notes**

app-first.log: eight adapted application stories pass.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/*.stories.svelte catalog/components/store-application/StoreRepositoryNavigation.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte catalog/components/store-application/StoreWorkspaces.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLog.stories.svelte
```

**Notes**

stories.log: 123 pass, two fail. NarrowLayout captured a replaced tab while responsive measurement ran; operation selection locator matched both graph and detail. Fixed waiting/scoping, preserving the visibility and selection assertions.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStacksNavigation.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspace.stories.svelte
```

**Notes**

rail-stories.log: 16 pass, two fail. Synthetic hover does not activate CSS :hover; feature/api occurs in both Stack and Branch rows. Further focused run rejects missing spaces in text assertion. Changed to native pointer, scoped/multiple values and whitespace-aware content checks.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/*.stories.svelte catalog/components/store-application/StoreRepositoryNavigation.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte catalog/components/store-application/StoreWorkspaces.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLog.stories.svelte
```

**Notes**

stories-verified.log: 125 pass in 27 directly affected story files. Shared tab composition justifies repository/navigation/history consumers. No unrelated whole-catalog lane.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStacksNavigation.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceProduction.stories.svelte
```

**Notes**

rail-production-last.log: seven pass, including the added dependent-branch/target-history rail story. Circular transparent-rest/native-hover Back, tab order/local eligibility/sidebar/chrome, narrow overflow/keyboard, retained actual canvas/diff nodes/title draft/history selection, workspace and selected file/branch/commit/operation metadata. These seven include six repeats plus one new story beyond the 125-story run.
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
pnpm web:check; pnpm catalog:check; pnpm check:architecture; pnpm check:ui-ownership
```

**Notes**

Last logs: zero web/catalog errors or warnings, 230 catalog boundaries, 1103 architecture modules and ownership pass. Repeated focused checks after presentation and lifecycle corrections.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:build; node scripts/stacks-browser-acceptance.mjs
```

**Notes**

First cold runs rejected late/obsolete graph preparation assumptions and shared index state. Second also caught unrelated Overview table keyboard accessibility because the new Stacks test scanned its final ordinary route; moved the unchanged Axe check to the focused canvas under test. Separate cold attempts exposed cancelled target/branch selection reads. Fixed product owner refresh and canvas navigation without weakening history/core assertions. One simultaneous filtered run also collided in the shared Playwright trace output; subsequent runs are sequential.
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
node scripts/stacks-browser-acceptance.mjs --grep cold_local_megamerges
```

**Notes**

cold-core-final.log: one cold real-folder test passes (18.1 seconds). Create/switch/apply/unapply, independent WIP, external branch updates, retained checkout files, reload, workspace Settings/update/delete, TOML locality, companion stability and accessibility verified using existing explicitly built development binaries; temporary server/folder cleaned.
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
JJ_STORE_STACKS_BROWSER_PORT=7490 node scripts/stacks-browser-acceptance.mjs --grep cold_focused_stacks
```

**Notes**

cold-navigation-verified.log: one cold test passes (5.2 seconds). Capability-gated Stacks entry, focused repository-wide histories, selection/canvas identity, exact Back, direct/reload fallback, browser Back/Forward, focused accessibility and no page errors. Separate disposable folder/index; no installed service changes.
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
pnpm docs:review application (specific focused navigation/details/lifetime explanation); pnpm web:build
```

**Notes**

Build includes documentation closure. An intermediate build failed the expected stale application review fingerprint after rail changes; updated Browsing guidance and refreshed the specific review. Final preceding successful build closure: nine light/dark Mermaid diagrams, 139 files, 13,953,873 bytes, valid links/anchors. Vite large-chunk warning remains unchanged.
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
pnpm spec:first; pnpm spec:validate
```

**Notes**

Initial spec-first rejected the new filtered runner without mapped spec-governance.md. Added the precise cold-isolation requirement. Follow-up first passed; five validators/23 chapters/173 requirements validated.
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
CUA read-only live HMR verification at http://127.0.0.1:7476/
```

**Notes**

Temporary verification tab confirms focused four-tab navigation, workspace picker/actions, unchanged Unstaged controls and relevant Workspace details (mains workspace, ID, main target, applied count, physical folder). Commits rail asks for a commit before selection. Returning from unprepared history exposed obsolete graph feedback; added tested canvas-return cleanup. User tabs preserved, temporary tab closed after verification.
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
```

**Notes**

progress-complete.log rejects path-only named-test references in this new entry. Corrected only E-M21-044 references to path#test-name and its API requirement to JST-API-005; historical evidence unchanged.
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
node scripts/stacks-browser-acceptance.mjs
```

**Notes**

cold-complete.log: focused navigation passes; post-apply physical file diff reports index rebuild required at the newly advanced operation. Corrected the browser scenario to explicitly prepare that pin and reselect the physical file, retaining command independence and the real diff/hunk assertions.
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
node scripts/stacks-browser-acceptance.mjs --grep cold_local_megamerges
```

**Notes**

cold-core-recovered.log: exact cold core regression passes in 13.2 seconds with real adjacent diff, command-independent preparation, Settings/deletion and retained named WIP.
<!-- /run -->
<!-- /evidence-field -->

