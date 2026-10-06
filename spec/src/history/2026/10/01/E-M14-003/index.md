---
schema: 1
id: E-M14-003
---

# E-M14-003

<!-- evidence-field:id markdown -->
## Id

E-M14-003
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M14
- M14.2
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-01
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

implementation
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-application/StoreWorkspaceSettings.stories.svelte
- catalog/controller/in-memory-store-controller.svelte.ts
- catalog/controller/in-memory-store-settings.svelte.ts
- catalog/fixtures/WorkspaceSettingsFixture.svelte
- spec/src/api.md
- spec/src/milestones.md
- spec/src/ui.md
- web/src/lib/controller/contracts.ts
- web/src/lib/controller/http-store-data-source.ts
- web/src/lib/controller/http-workspace-links.ts
- web/src/lib/controller/native-data-source.ts
- web/src/lib/controller/production-store-actions.ts
- web/src/lib/controller/production-store-index.ts
- web/src/lib/controller/production-store-state.svelte.ts
- web/src/lib/controller/production-store-view.ts
- web/src/lib/controller/production-workspace-links.ts
- web/src/lib/controller/workspace-link-types.ts
- web/src/lib/ui/store-settings/StoreRepositorySettings.svelte
- web/src/lib/ui/store-settings/StoreSettings.css
- web/src/lib/ui/store-settings/StoreWorkspaceSettings.svelte
- web/tests/browser/local-workspaces.spec.ts
- web/tests/vite/http-workspace-links.test.ts
- web/tests/vite/production-store-workspaces.test.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/vite/production-store-workspaces.test.ts#local settings load and
  link physical workspaces without text search
- web/tests/vite/production-store-workspaces.test.ts#failed linking retains the
  draft and explicit refresh recovers revision
- web/tests/vite/production-store-workspaces.test.ts#workspace links ignore late
  responses after disconnect
- web/tests/vite/production-store-workspaces.test.ts#unsupported capability
  never reads or links device folders
- web/tests/vite/production-store-workspaces.test.ts#link submission is single
  flight and ignores completion after repository switch
- web/tests/vite/production-store-workspaces.test.ts#late folder refresh cannot
  replace the result of linking
- web/tests/vite/http-workspace-links.test.ts#workspace transport preserves
  revisions and uses protected local requests
- web/tests/browser/local-workspaces.spec.ts#local folder settings preserve
  drafts and link without changing repository context
<!-- /evidence-field -->

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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/production-store-workspaces.test.ts -t 'local settings load and link'
```

**Notes**

Before implementation: one failed, three skipped; no workspace-list call.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/production-store-settings.test.ts web/tests/vite/production-store-workspaces.test.ts
```

**Notes**

Initial run: nine passed, one failed. Corrected simulated generic Error to the typed ApiError accepted by the sanitizing production transport.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/production-store-workspaces.test.ts -t 'local settings load and link'
```

**Notes**

Exact regression after implementation: one passed, five skipped.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/production-store-settings.test.ts web/tests/vite/production-store-workspaces.test.ts web/tests/vite/http-workspace-links.test.ts
```

**Notes**

13 passed: six settings, six folder lifecycle, one lossless protected transport test.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreWorkspaceSettings.stories.svelte catalog/components/store-application/StoreApplication.stories.svelte -t 'Settings|draft|Unsupported'
```

**Notes**

Seven selected stories passed; six unrelated stories skipped. Interaction and accessibility passed; visual baselines remain unapproved.
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
pnpm catalog:check
```

**Notes**

One TypeScript diagnostic: Testing Library role query does not accept exact. Replaced it with an anchored name regex.
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
pnpm catalog:check
```

**Notes**

Zero errors/warnings; catalog boundaries passed for 94 modules.
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
pnpm web:check
```

**Notes**

Zero errors and warnings.
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
pnpm web:build
```

**Notes**

Fresh production bundle built. Existing large-chunk warning remains informational.
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
pnpm exec playwright test --config .scratch/settings-playwright.config.ts web/tests/browser/local-bootstrap.spec.ts web/tests/browser/reads.spec.ts web/tests/browser/local-workspaces.spec.ts --grep 'local trusted bootstrap|index_preparation_retries|local folder settings'
```

**Notes**

Three passed against a fresh preview on 127.0.0.1:7476: bootstrap, graph recovery and folder linking. Captured/intercepted API responses; validates explicit revision recovery, navigation/draft retention, unchanged repository hash, mobile overflow and zero axe violations. Not a live browser-to-native integration claim.
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

409 production modules passed before the final transport-only test was added.
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
pnpm spec:first
```

**Notes**

Mapped API chapter required; added concrete browser revision, draft and response-invalidation behavior.
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
pnpm exec prettier --write <all changed UI files and canonical chapters>
```

**Notes**

Formatted all 23 changed files returned by jj diff --name-only.
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
pnpm web:check; pnpm check:architecture; pnpm spec:validate; pnpm spec:first; pnpm check:progress
```

**Notes**

Final web check has zero errors/warnings; architecture accepts 409 modules; canonical validation accepts 132 requirements; spec-first accepts 20 protected files and three canonical chapters; progress accepts 111 tasks.
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
<!-- generated-navigation -->
## Pages

- [E-M14-003 details 2](details-02.md)
