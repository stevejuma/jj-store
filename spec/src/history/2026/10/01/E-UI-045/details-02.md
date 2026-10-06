---
schema: 1
id: E-UI-045
---

# E-UI-045 details 2

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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/NativeReviewDetail.stories.svelte
```

**Notes**

Before implementation: missing Review details; .scratch/review-detail-before.log.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/ProjectsHub.stories.svelte
```

**Notes**

Before acceptance updates: missing Activity tab and old visible-Subtasks expectation; 8 other cases pass. .scratch/projects-activity-before.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/NativeReviewDetail.stories.svelte
```

**Notes**

Intermediate suite-import failure from virtual CSS/Tailwind parsing while source formatting overlapped transforms; no tests executed. Stable cold reruns import and pass. .scratch/review-detail-after.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/NativeReviewDetail.stories.svelte catalog/components/store-work/ProjectsActivity.stories.svelte catalog/components/store-work/ProjectsHub.stories.svelte
```

**Notes**

Intermediate 10 pass; two fixtures incorrectly queried the agent message list as list instead of its public log role. Corrected role; .scratch/review-activity-focused-stories.log.
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

Initial new fixture imported API client types across the catalog transport boundary. Replace with production presentation-state types; the boundary remains enforced.
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
pnpm check:architecture
```

**Notes**

Initial hub exceeded the 300 counted-line production bound. Extract project route actions; no limit changes.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/NativeReviewDetail.stories.svelte catalog/components/store-work/ProjectsActivity.stories.svelte catalog/components/store-work/ProjectsHub.stories.svelte catalog/components/store-work/ProjectsDirectory.stories.svelte catalog/components/store-work/StoreWork.stories.svelte catalog/components/store-application/StoreCollaboration.stories.svelte
```

**Notes**

Intermediate 25 pass. Timeline article had an invalid listitem role; use a valid div. Scope the description/comment duplicate to Review comments. Final same six files pass 28/28; .scratch/review-activity-stories.log is the passing rerun.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/native-activity.test.ts web/tests/vite/production-store-collaboration.test.ts web/tests/vite/projects-hub.test.ts web/tests/vite/projects-code-navigation.test.ts web/tests/vite/projects-repositories.test.ts
```

**Notes**

Intermediate 41 pass. Continuation drift fixture changed an informational code version instead of selected manifest identity; use manifest_revision drift. Final focused unit cases pass.
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
pnpm web:check
```

**Notes**

Final 0 errors/warnings; .scratch/review-activity-web-check.log.
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
pnpm catalog:check
```

**Notes**

Expanded search story supplied Playwright exact in a Testing Library role query; use anchored regex. Final catalog check passes 0 errors/warnings and all 90 boundary modules.
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
pnpm catalog:check
```

**Notes**

Final type/boundary pass; .scratch/review-activity-catalog-check.log.
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
pnpm check:architecture
```

**Notes**

399 production/catalog/script modules satisfy existing boundaries and file-size rules.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/native-activity.test.ts web/tests/vite/production-store-collaboration.test.ts web/tests/vite/projects-hub.test.ts web/tests/vite/projects-code-navigation.test.ts web/tests/vite/projects-repositories.test.ts web/tests/vite/production-store-bootstrap.test.ts web/tests/vite/store-route-history.test.ts
```

**Notes**

54/54 across seven direct controller consumers, including no-source native-link navigation; .scratch/review-activity-unit.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/NativeReviewDetail.stories.svelte catalog/components/store-work/ProjectsActivity.stories.svelte catalog/components/store-work/ProjectsHub.stories.svelte catalog/components/store-work/ProjectsDirectory.stories.svelte catalog/components/store-work/StoreWork.stories.svelte catalog/components/store-application/StoreCollaboration.stories.svelte
```

**Notes**

28/28 Chromium interaction/accessibility tests; .scratch/review-activity-stories.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/NativeReviewDetail.stories.svelte catalog/components/store-work/ProjectsActivity.stories.svelte
```

**Notes**

Final expanded direct acceptance 3/3, including exact copied file-sidebar toggle and icon empty state after Activity search. .scratch/review-activity-final-stories.log.
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
pnpm web:build
```

**Notes**

Built browser manifest/assets; existing large-chunk warning retained. .scratch/review-activity-build.log. Owned restart rebuilds final source once more.
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
node .scratch/review-activity-main-state.mjs before
```

**Notes**

Captures exact WorkStore records for all three sample projects, code snapshot and fixture/server-config SHA-256 before owned main restart.
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
JJ_STORE_DEV_RUSTFS_RUNTIME=/Users/stevejuma/code/lapismd/jj-store/runtime/embedded-rustfs/target/debug/jj-store-rustfs pnpm dev:restart
```

**Notes**

Owned-demo rebuild/restarts reuse the same installation and RustFS volume. Latest restart 2583 ms, 172576 KiB resident; .scratch/review-activity-final-restart.log. No reset/reseed or unrelated service restart.
<!-- /run -->
<!-- /evidence-field -->

