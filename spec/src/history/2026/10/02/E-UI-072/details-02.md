---
schema: 1
id: E-UI-072
---

# E-UI-072 details 2

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
pnpm check:progress
```

**Notes**

Ledger pass conflicts with explicitly deferred visual approval. Evidence remains pending with passing functional runs; no milestone status changed.
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
pnpm exec prettier --write changed files; pnpm web:check; pnpm catalog:check; pnpm spec:first; pnpm spec:validate; pnpm check:architecture; pnpm check:progress
```

**Notes**

Final collection inset: web/catalog zero errors/warnings;25 protected files/6 chapters;5 validators/21 chapters/147 requirements;756 architecture modules. Corrected ledger then repeated its formatter/checker before commit.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/ProjectsAgents.stories.svelte
```

**Notes**

User follow-up before state: row separator and full-width row fail collection geometry assertion;2/3 stories pass.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/ProjectsAgents.stories.svelte catalog/components/store-application/StoreCollaboration.stories.svelte catalog/components/store-application/NativeAgentMenus.stories.svelte
```

**Notes**

12/12 stories including exact no-separator and heading-aligned row geometry; collection presentation uses heading inset while repository rows retain their composition.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/projects-agents.test.ts
```

**Notes**

Before:3/3 regressions fail because Agents collection and scope are absent. Initial implementation1 failure corrected by invalidating the fixture project view before injecting its new links.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/projects-agents.test.ts web/tests/vite/projects-code-navigation.test.ts web/tests/vite/projects-hub.test.ts web/tests/vite/projects-directories.test.ts web/tests/vite/projects-repositories.test.ts
```

**Notes**

Historical intermediate redirect implementation, superseded by inline response follow-up. New mismatch test expected raw code instead of readable API message. Collection helper awaited an unrelated empty dispatch and delayed browse start; narrowed dispatch to relevant tabs. Source behavior preserved.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/projects-agents.test.ts web/tests/vite/projects-code-navigation.test.ts web/tests/vite/projects-hub.test.ts web/tests/vite/projects-directories.test.ts web/tests/vite/projects-repositories.test.ts
```

**Notes**

Historical intermediate redirect implementation, superseded by inline response follow-up. 31/31 exact/direct controller tests after corrections; repeated after final extraction/navigation changes. Includes scope pagination, snapshot mismatch, denial, refresh window, history and late connection.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/ProjectsAgents.stories.svelte
```

**Notes**

3/3 new landing, selected-project and loading-skeleton stories with blocking accessibility.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work catalog/components/store-application/NativeAgentMenus.stories.svelte catalog/components/store-application/StoreCollaboration.stories.svelte catalog/components/store-application/AppearanceDiscussions.stories.svelte catalog/components/store-application/RepositoryProjects.stories.svelte
```

**Notes**

62/63 pass; existing landing-tab assertion still expected5 tabs. Updated exact expected list to include Agents; rerun14/14 focused Hub/Agents passes. Final direct lane63/63 passed.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/ProjectsHub.stories.svelte catalog/components/store-work/ProjectsAgents.stories.svelte
```

**Notes**

14/14 final Hub and Agents stories pass with blocking accessibility.
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
pnpm web:check; pnpm catalog:check; pnpm check:architecture; pnpm spec:first; pnpm check:ui-ownership
```

**Notes**

Final web/catalog0 errors/warnings;756 production modules;134 catalog modules;24 protected files/5 chapters;UI ownership passes. Initial nullable association and hub line limit repaired via typed nullable prop and owned helper extraction.
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
CUA live read-only Agents landing/project/navigation check on HMR7460
```

**Notes**

Historical intermediate redirect implementation, superseded by inline response follow-up. Captured landing/project scopes with actual native sessions and local empty state. Initial session action connection retained the hub; fixed route-first connection and verified exact native Agents URL/transcript. Stable live Back restores the originating Projects Agents route. No fixture writes or service changes.
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
pnpm catalog:test web/tests/vite/projects-agent-response.test.ts
```

**Notes**

New current-source regression: 1 failed, 2 passed before accessor implementation.
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
pnpm catalog:test web/tests/vite/projects-agent-response.test.ts web/tests/vite/projects-agents.test.ts
```

**Notes**

8/8 source lifecycle, late selection, denial and scoped directory tests.
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
pnpm catalog:test web/tests/vite/projects-agent-response.test.ts web/tests/vite/projects-agents.test.ts web/tests/vite/projects-code-navigation.test.ts web/tests/vite/projects-hub.test.ts web/tests/vite/projects-directories.test.ts web/tests/vite/projects-repositories.test.ts web/tests/vite/production-store-collaboration.test.ts web/tests/vite/store-route-history.test.ts
```

**Notes**

61/61 directly affected controller/history/collaboration tests; root aggregates remain out of scope.
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
pnpm exec prettier --write changed files using rg-filtered command substitution
```

**Notes**

Local rg configuration added numeric prefixes to filenames. Repaired command uses Python subprocess paths directly; no source formatting occurred in failed invocation.
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
Python subprocess pnpm exec prettier --write all changed ts/svelte/css/md files
```

**Notes**

Changed files formatted without shell interpolation.
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
pnpm web:check; pnpm check:architecture
```

**Notes**

Web zero errors/warnings; 760 production modules.
<!-- /run -->
<!-- /evidence-field -->

