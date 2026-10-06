---
schema: 1
id: E-UI-045
---

# E-UI-045 details 4

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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/NativeReviewDetail.stories.svelte
```

**Notes**

Back control vertically centered, including wrapped heading: 2/2. .scratch/activity-review-centered-stories.log.
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

Activity Back/Forward and scroll, task defaults, native review files/commits, axe and mobile checks pass. Column selector lookup after mobile resize times out because the rail is closed. The user then replaces column modes with list/detail; remove the superseded mode assertions. .scratch/activity-review-native-browser.log.
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
pnpm web:check
```

**Notes**

Final list estimate passed nullable review excerpt into a string-only estimator. Normalize null to undefined. .scratch/activity-review-list-web-check.log.
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
pnpm catalog:check
```

**Notes**

Same nullable excerpt diagnostic; .scratch/activity-review-list-catalog-check.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/NativeReviewDetail.stories.svelte catalog/components/store-work/ProjectsActivity.stories.svelte catalog/components/store-work/NativeActivity.stories.svelte catalog/components/store-work/ProjectsHub.stories.svelte catalog/components/store-work/ProjectsDirectory.stories.svelte catalog/components/store-work/StoreWork.stories.svelte catalog/components/store-application/StoreCollaboration.stories.svelte
```

**Notes**

29/30 pass; full-width list regression detects consumer ScrollArea host shrinking to 17.6px. Set consumer width/min-width/flex and import existing collection styling; keep the &gt;600px assertion. .scratch/activity-review-list-stories.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/NativeReviewDetail.stories.svelte
```

**Notes**

Exact repaired Community review list/detail cases 2/2. .scratch/activity-review-list-exact-stories.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/native-activity.test.ts web/tests/vite/production-store-collaboration.test.ts web/tests/vite/projects-hub.test.ts web/tests/vite/projects-code-navigation.test.ts web/tests/vite/projects-repositories.test.ts web/tests/vite/production-store-bootstrap.test.ts web/tests/vite/store-route-history.test.ts
```

**Notes**

Final seven direct controller consumers 59/59, including retired column hash normalization and exact Activity Back/Forward. .scratch/activity-review-list-unit.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/NativeReviewDetail.stories.svelte catalog/components/store-work/ProjectsActivity.stories.svelte catalog/components/store-work/NativeActivity.stories.svelte catalog/components/store-work/ProjectsHub.stories.svelte catalog/components/store-work/ProjectsDirectory.stories.svelte catalog/components/store-work/StoreWork.stories.svelte catalog/components/store-application/StoreCollaboration.stories.svelte
```

**Notes**

Final seven direct interaction/accessibility files 30/30 in a fresh runner. Bounded 120-row activity rendering, lazy transcript reads, measured expansion, memoized remount and cursor-edge continuation to 160 rows inside shadcn ScrollArea. Reviews list/detail, centered Back and absent sidebar controls verified. .scratch/activity-review-list-final-stories.log.
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

Final 0 errors/0 warnings. .scratch/activity-review-list-final-web-check.log.
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
pnpm catalog:check
```

**Notes**

Final 0 errors/0 warnings and 92 catalog transport-boundary modules. .scratch/activity-review-list-final-catalog-check.log.
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
pnpm check:architecture
```

**Notes**

Final 402 modules pass existing boundaries and line budgets. .scratch/activity-review-list-final-architecture.log.
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
pnpm exec prettier --write web/src/lib/ui/store-collaboration/StoreReviewThreads.svelte
```

**Notes**

Final consumer width/nullable estimate fix formatted before test transforms.
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
node .scratch/review-activity-main-state.mjs before
```

**Notes**

Final rebuild capture includes three project records, exact task snapshots, code snapshot and fixture/config hashes.
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
JJ_STORE_DEV_RUSTFS_RUNTIME=/Users/stevejuma/code/lapismd/jj-store/runtime/embedded-rustfs/target/debug/jj-store-rustfs pnpm dev:restart
```

**Notes**

Final list/detail source rebuilt and owned loopback demo restarted. .scratch/activity-review-list-restart.log. No reset or reseed.
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
node .scratch/review-activity-main-state.mjs after
```

**Notes**

Final persisted WorkStore/code captures and config hashes exactly unchanged.
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
JJ_STORE_REAL_BASE_URL=http://127.0.0.1:7460/ JJ_STORE_REAL_REPOSITORY=1bdb89c409c1b891ac7f180c01a265d5 JJ_STORE_REAL_RUNTIME=/Users/stevejuma/code/lapismd/jj-store/.local/dev-demo JJ_STORE_REAL_ROOT=/Users/stevejuma/code/lapismd/jj-store pnpm exec playwright test --config web/playwright.real.config.ts projects-activity.spec.ts
```

**Notes**

Final cold built-browser 1/1 passes in 13.2s. App Back/Forward restore exact Activity route and scroll. Task defaults/reload, list-to-review navigation, no workspace picker or detail tabs, Files/Commits/Activity, native diff stats, axe, mobile overflow, retired column link, absent column canvas/sidebar controls and original anchored agent tools pass. Screenshots manually inspected. .scratch/activity-review-list-native.log.
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
pnpm spec:validate
```

**Notes**

Final 4 validators, 21 chapters, 131 requirements; .scratch/activity-review-list-spec.log.
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
pnpm spec:first
```

**Notes**

Final 64 protected paths, 4 canonical chapters; .scratch/activity-review-list-spec-first.log.
<!-- /run -->
<!-- /evidence-field -->

