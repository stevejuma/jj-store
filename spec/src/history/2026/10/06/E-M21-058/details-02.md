---
schema: 1
id: E-M21-058
---

# E-M21-058 details 2

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 7
detail: "Before implementation: 7 failed/6 passed; five startup/failure stories
  and two chip-size regressions. /tmp/stack-startup-chips-before.log."
hasNote: false
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceLoading.stories.svelte catalog/components/community-projects/CommunityProjectsRevisionGraph.stories.svelte
```
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 1
detail: "Before readiness state: 1 failed/10 passed. Binding seed had no
  successful-content-read indicator. /tmp/stack-startup-unit-before.log."
hasNote: false
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/production-stacks-lifecycle.test.ts
```
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 4
detail: "First fix: 4 failed/9 passed. Two existing assertions addressed chip
  text rather than its Badge after adding a truncation span; two new history
  stories used nonexistent region labels. /tmp/stack-startup-chips-after.log."
hasNote: false
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook <loading and revision-graph stories>
```
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 1
detail: "After semantic chip/list queries: 1 failed/12 passed. New fixture
  supplied commits instead of the public log route value. Corrected fixture.
  /tmp/stack-startup-chips-fixed.log."
hasNote: false
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook <loading and revision-graph stories>
```
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 3
detail: "Three fixture type errors: remote:null instead of optional string, and
  commits instead of log view. Corrected without loosening production types.
  /tmp/stack-startup-catalog.log."
hasNote: false
noteEncoding: markdown

```

```shell
pnpm catalog:check
```
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: pass
failures: 0
detail: 13 passed across connecting, binding seed, held read, failure/retry and
  retained refresh content; bounded long JJ/Op Log chips.
  /tmp/stack-startup-chips-final.log.
hasNote: false
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook <loading and revision-graph stories>
```
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: pass
failures: 0
detail: 11 passed after successful workspace-read readiness.
  /tmp/stack-startup-unit-after.log.
hasNote: false
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/production-stacks-lifecycle.test.ts
```
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: pass
failures: 0
detail: "Initial directly affected controller family: 118 passed in 14 files.
  /tmp/stack-startup-units-final.log."
hasNote: false
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit 'web/tests/vite/stack-workspace' 'web/tests/vite/production-stacks'
```
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: pass
failures: 0
detail: 0 errors/0 warnings, 244 catalog modules before exposing the additional
  discovery state. /tmp/stack-startup-web.log and
  /tmp/stack-startup-catalog-final.log.
hasNote: false
noteEncoding: markdown

```

```shell
pnpm web:check; pnpm catalog:check
```
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 2
detail: "Two files crossed the 300 counted-line budget: controller 303,
  workspace 315. Extracted typed controller options and the shared startup/Empty
  state component; kept the boundary intact.
  /tmp/stack-startup-architecture.log."
hasNote: false
noteEncoding: markdown

```

```shell
pnpm check:architecture
```
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: pass
failures: 0
detail: Initial ownership check passes. /tmp/stack-startup-ownership.log.
hasNote: false
noteEncoding: markdown

```

```shell
pnpm check:ui-ownership
```
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: fail
failures: 1
detail: "50 passed/1 failed: the narrow More trigger was replaced during
  redundant width measurements. /tmp/stack-startup-consumers.log."
hasNote: false
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook <Application, Navigation, Setup, Production, Recovery, RepositoryLog and RepositoryLoading stories>
```
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: fail
failures: 1
detail: Live check exposed a connected-but-discovering gap absent from the first
  fixture; the neutral unavailable state still appeared during binding
  discovery. Added explicit reactive discovery state and a connected/ready
  fixture phase. Broad network latency missed the loading window and was
  cleared; scoped Fetch holding provided the precise reproduction. No repository
  mutations.
hasNote: false
noteEncoding: markdown

```

```shell
CUA cold Vite reload and tab-local delayed binding discovery
```
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: fail
failures: 1
detail: "Added discovery regression: 1 failed/11 passed before the independently
  exposed loading getter. /tmp/stack-discovery-unit-before.log."
hasNote: false
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/production-stacks-lifecycle.test.ts
```
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: pass
failures: 0
detail: 119 passed in 14 files after discovery ownership and readiness updates.
  /tmp/stack-discovery-units-final.log.
hasNote: false
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit 'web/tests/vite/stack-workspace' 'web/tests/vite/production-stacks'
```
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: fail
failures: 1
detail: 17 passed/1 failed; only narrow More remained.
  /tmp/stack-discovery-stories-final.log.
hasNote: false
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook <Loading, RevisionGraph and StacksNavigation stories>
```
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: fail
failures: 1
detail: Exact failure reproduced; waiting for loaded/enabled controls did not
  fix it. Diagnostic showed original More disconnected, replacement closed and
  no menu. Temporary diagnostic removed.
  /tmp/stack-startup-navigation-exact.log, navigation-ready.log and
  navigation-diagnostic.log.
hasNote: false
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStacksNavigation.stories.svelte -t FocusedTabsFitNarrowBodyAndSupportKeyboard
```
<!-- /run -->

<!-- run -->
### Run 18

```yaml
result: pass
failures: 0
detail: "Exact regression passes after suppressing equivalent width/section
  fits: trigger remains mounted; Op Log menu selection, Back and keyboard tab
  movement succeed. Four other stories skipped by the exact filter.
  /tmp/stack-startup-navigation-after.log."
hasNote: false
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStacksNavigation.stories.svelte -t FocusedTabsFitNarrowBodyAndSupportKeyboard
```
<!-- /run -->

<!-- run -->
### Run 19

```yaml
result: pass
failures: 0
detail: "After discovery extraction: both type checks 0 errors/0 warnings, 244
  catalog modules; architecture 1140 modules.
  /tmp/stack-discovery-web-final.log, catalog-final.log and
  architecture-final.log."
hasNote: false
noteEncoding: markdown

```

```shell
pnpm web:check; pnpm catalog:check; pnpm check:architecture
```
<!-- /run -->
<!-- /evidence-field -->

