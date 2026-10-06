---
schema: 1
id: E-UI-045
---

# E-UI-045 details 3

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
node .scratch/review-activity-main-state.mjs after
```

**Notes**

Exact native WorkStore/code snapshots and fixture/server-config fingerprints unchanged after every owned restart.
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
JJ_STORE_REAL_BASE_URL=http://127.0.0.1:7460/ JJ_STORE_REAL_REPOSITORY=1bdb89c409c1b891ac7f180c01a265d5 JJ_STORE_REAL_RUNTIME=/Users/stevejuma/code/lapismd/jj-store/.local/dev-demo JJ_STORE_REAL_ROOT=/Users/stevejuma/code/lapismd/jj-store pnpm exec playwright test --config web/playwright.real.config.ts projects-activity.spec.ts
```

**Notes**

First cold read-only browser attempt omitted the repository code pin after a Projects-only bootstrap. Add the captured pin. Project/task Activity and reload/default checks already passed.
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
JJ_STORE_REAL_BASE_URL=http://127.0.0.1:7460/ JJ_STORE_REAL_REPOSITORY=1bdb89c409c1b891ac7f180c01a265d5 JJ_STORE_REAL_RUNTIME=/Users/stevejuma/code/lapismd/jj-store/.local/dev-demo JJ_STORE_REAL_ROOT=/Users/stevejuma/code/lapismd/jj-store pnpm exec playwright test --config web/playwright.real.config.ts projects-activity.spec.ts
```

**Notes**

Second attempt exposes the native external-route handler ignoring all repository routes without an active source, even with discovered access. Fix authorization/connection and add focused no-source regression; Activity commit links and Back now exercised.
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
JJ_STORE_REAL_BASE_URL=http://127.0.0.1:7460/ JJ_STORE_REAL_REPOSITORY=1bdb89c409c1b891ac7f180c01a265d5 JJ_STORE_REAL_RUNTIME=/Users/stevejuma/code/lapismd/jj-store/.local/dev-demo JJ_STORE_REAL_ROOT=/Users/stevejuma/code/lapismd/jj-store pnpm exec playwright test --config web/playwright.real.config.ts projects-activity.spec.ts
```

**Notes**

Third attempt passes navigation, native review files/commits and layout. Fixture incorrectly required an agent association for the seeded browser revision, which has none. Test actual associated commits plus an original anchored agent review; do not synthesize an association.
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
JJ_STORE_REAL_BASE_URL=http://127.0.0.1:7460/ JJ_STORE_REAL_REPOSITORY=1bdb89c409c1b891ac7f180c01a265d5 JJ_STORE_REAL_RUNTIME=/Users/stevejuma/code/lapismd/jj-store/.local/dev-demo JJ_STORE_REAL_ROOT=/Users/stevejuma/code/lapismd/jj-store pnpm exec playwright test --config web/playwright.real.config.ts projects-activity.spec.ts
```

**Notes**

A fourth fixture expectation assumed the seeded Activity feed had code associations. Live AgentStore confirms every captured session has zero code associations; the exact original anchored agent capture is available with three tools. Require no fabricated inline context for this seed, and verify the anchored agent review. Shared associated-commit context remains covered by native readers and Storybook.
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
JJ_STORE_REAL_BASE_URL=http://127.0.0.1:7460/ JJ_STORE_REAL_REPOSITORY=1bdb89c409c1b891ac7f180c01a265d5 JJ_STORE_REAL_RUNTIME=/Users/stevejuma/code/lapismd/jj-store/.local/dev-demo JJ_STORE_REAL_ROOT=/Users/stevejuma/code/lapismd/jj-store pnpm exec playwright test --config web/playwright.real.config.ts projects-activity.spec.ts
```

**Notes**

Read-only cold built-browser native acceptance passes 1/1 in 12.4 s: all five landing tabs, project/task feeds, exact native commit links from Projects without an active code source, Back, Description-only defaults after reload, actual review files/commits/workflow/discussion, original anchored agent activity and tool calls, review accessibility and mobile overflow. The seed intentionally records no code-agent associations; no inferred context is inserted. Final artifact screenshot scroll is reset to the review header and a final icon build is checked in the final same test rerun.
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
pnpm spec:validate
```

**Notes**

4 validators, 21 chapters, 131 requirements.
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
pnpm spec:first
```

**Notes**

56 protected files, 4 canonical chapters.
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
pnpm check:progress
```

**Notes**

103 tasks; existing parent evidence preserved.
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
pnpm exec prettier --write (changed TS/Svelte/CSS/Markdown paths from jj diff --name-only)
```

**Notes**

Production/catalog/spec files formatted in stable source intervals. No source formatting overlapped the final passing Storybook transform.
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
jj --no-pager diff --check
```

**Notes**

Optional CLI diff check flag is unsupported by installed JJ. No state changed. Changed-source Prettier checking and JJ status provide the format/conflict checks.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/native-activity.test.ts web/tests/vite/production-store-collaboration.test.ts web/tests/vite/projects-hub.test.ts web/tests/vite/projects-code-navigation.test.ts web/tests/vite/projects-repositories.test.ts web/tests/vite/production-store-bootstrap.test.ts web/tests/vite/store-route-history.test.ts
```

**Notes**

Final 54/54 includes exact file counts when the path index omits diff content. .scratch/review-activity-unit.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/store-route-history.test.ts
```

**Notes**

Before Activity history repair: 7/8 pass; ordinary commit link cannot return to Projects. .scratch/activity-history-before.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/native-activity.test.ts web/tests/vite/production-store-collaboration.test.ts web/tests/vite/store-route-history.test.ts
```

**Notes**

Deferred discovery/details, reader budget, continuation roots and Activity Back/Forward: 30/30. .scratch/activity-lazy-unit.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/NativeReviewDetail.stories.svelte catalog/components/store-work/ProjectsActivity.stories.svelte catalog/components/store-work/NativeActivity.stories.svelte
```

**Notes**

Initial lazy activity stories 4/4; .scratch/activity-lazy-stories.log.
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/NativeReviewDetail.stories.svelte catalog/components/store-work/ProjectsActivity.stories.svelte catalog/components/store-work/NativeActivity.stories.svelte catalog/components/store-work/ProjectsHub.stories.svelte catalog/components/store-work/ProjectsDirectory.stories.svelte catalog/components/store-work/StoreWork.stories.svelte catalog/components/store-application/StoreCollaboration.stories.svelte
```

**Notes**

Intermediate expansion encountered a transient virtual-CSS import and a fixture querying radio layout controls as buttons. Stable sequential rerun passes 30/30. The layout controls are subsequently removed by the user request; no dependency edits.
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
pnpm check:architecture
```

**Notes**

StoreApplication grew to 323 counted lines against its existing 320 limit. Extract unchanged workspace heading into StoreWorkspaceSectionHeader; retain the limit.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreWorkspaces.stories.svelte catalog/components/store-application/NativeReviewDetail.stories.svelte
```

**Notes**

Workspace header extraction plus review detail: 7/7. .scratch/activity-review-workspace-stories.log.
<!-- /run -->
<!-- /evidence-field -->

