---
schema: 1
id: E-M21-006
---

# E-M21-006 details 2

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
pnpm catalog:test web/tests/vite/production-stack-adapter.test.ts
```

**Notes**

Missing adapter module before implementation: production-stacks-before.log, no tests collected.
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
pnpm catalog:test web/tests/vite/production-stack-adapter.test.ts
```

**Notes**

Initial four adapter regressions pass in production-stacks-first.log and production-stacks-second.log.
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
pnpm catalog:test web/tests/vite/production-stack-adapter.test.ts
```

**Notes**

Exact stack history regression failed before the adapter method existed: production-stacks-history-before.log, four passed.
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
pnpm catalog:test web/tests/vite/production-stack-adapter.test.ts
```

**Notes**

Five adapter tests pass in production-stacks-history-after.log.
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
pnpm catalog:test web/tests/vite/production-stack-adapter.test.ts web/tests/vite/stack-workspace-stack-history.test.ts
```

**Notes**

Nine tests/two files pass in production-stacks-controller-first.log and production-stacks-controller-second.log.
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
pnpm catalog:test web/tests/vite/production-stack-adapter.test.ts
```

**Notes**

Pagination/range retention: production-stacks-pagination-before.log, five passed; missing readBranches and lost ancestors.
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
pnpm catalog:test web/tests/vite/production-stack-adapter.test.ts
```

**Notes**

Seven tests pass in production-stacks-pagination-after.log.
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
pnpm catalog:test web/tests/vite/production-stacks-lifecycle.test.ts web/tests/vite/production-stack-adapter.test.ts web/tests/vite/stack-workspace-stack-history.test.ts
```

**Notes**

Lifecycle collection initially imported the wrong route module: production-stacks-lifecycle-first.log. Fixed to initialStoreRoute.
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
pnpm catalog:test web/tests/vite/production-stacks-lifecycle.test.ts web/tests/vite/production-stack-adapter.test.ts web/tests/vite/stack-workspace-stack-history.test.ts
```

**Notes**

19 tests/three files pass: production-stacks-lifecycle-second.log.
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
pnpm catalog:test web/tests/vite/production-stacks-application.test.ts
```

**Notes**

Two production bootstrap/index-preparation tests pass: production-stacks-application-first.log.
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
pnpm catalog:test web/tests/vite/production-stacks-application.test.ts web/tests/vite/production-stack-adapter.test.ts web/tests/vite/stack-workspace-stack-history.test.ts
```

**Notes**

14 tests/three files pass in production-stacks-injection-units-first.log.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: fail
failures: 8
hasNote: true
noteEncoding: markdown

```

```shell
pnpm catalog:test web/tests/vite/production-stack-adapter.test.ts web/tests/vite/production-stacks-application.test.ts web/tests/vite/stack-workspace-target-history.test.ts
```

**Notes**

production-stacks-history-retention-first.log: eight pass; absent range matched an undefined head and was dereferenced. Added an explicit range guard.
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
pnpm catalog:test web/tests/vite/production-stack-adapter.test.ts web/tests/vite/production-stacks-application.test.ts web/tests/vite/stack-workspace-target-history.test.ts
```

**Notes**

16 tests/three files pass in production-stacks-history-retention-second.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit <20 affected stack/bootstrap/events/refresh/settings/workspace/local-browser files>
```

**Notes**

161 tests initially, then 162 tests after the operation/target retention regression: production-stacks-units-impact-first.log and production-stacks-units-impact-final.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/*stack*.test.ts web/tests/vite/production-automatic-refresh.test.ts web/tests/vite/production-store-bootstrap.test.ts web/tests/vite/production-store-events.test.ts web/tests/vite/production-store-settings.test.ts web/tests/vite/production-store-workspaces.test.ts web/tests/vite/local-browser.test.ts web/tests/vite/local-browser-merge.test.ts
```

**Notes**

173 tests/23 files pass after the final connection-owned teardown extraction: production-stacks-units-impact-current.log. One shell invocation with nonexistent production-local glob exited before Vitest; a quoted glob matched only three files/28 tests in production-stacks-units-current.log and was replaced by this complete invocation.
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
pnpm test:storybook catalog/components/store-stack-workspace/StoreStackWorkspaceProduction.stories.svelte
```

**Notes**

production-stacks-stories-first.log: one passed; the index preparation toast was searched inside the canvas rather than its document portal.
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
pnpm test:storybook catalog/components/store-stack-workspace/StoreStackWorkspaceProduction.stories.svelte
```

**Notes**

Two production-bootstrap interaction/axe stories pass in production-stacks-stories-second.log.
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
pnpm test:storybook <all stack-workspace stories; StoreRepositoryPicker, RepositoryLoading, StoreRepositoryApplication, StoreApplication, StoreWorkspaces; RepositoryLog, RepositoryLogDiff, RepositoryLogFiles>
```

**Notes**

150 interactions/29 story files pass in a fresh headless Vite/Chromium run: production-stacks-stories-impact-first.log. The exact expanded argv is retained in its command header.
<!-- /run -->
<!-- /evidence-field -->

