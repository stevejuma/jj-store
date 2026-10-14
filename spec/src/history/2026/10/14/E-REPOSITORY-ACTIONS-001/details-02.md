---
schema: 1
id: E-REPOSITORY-ACTIONS-001
---

# E-REPOSITORY-ACTIONS-001 details 2

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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/repository-actions.test.ts web/tests/vite/workflows-route.test.ts web/tests/vite/store-route-history.test.ts web/tests/vite/store-stacks-navigation.test.ts web/tests/vite/repository-settings-restoration.test.ts
```

**Notes**

43 tests passed. Fixed source scope, uint64 pins, malformed routes, tab transitions and restoration without unrelated code reads.
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
pnpm exec tsx --test web/tests/store-route-codec.test.ts
```

**Notes**

13 route codec tests passed.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-workflows/WorkflowSidebarActions.stories.svelte catalog/components/store-workflows/RepositoryActions.stories.svelte
```

**Notes**

8 interaction/Axe stories passed. Edit/Run/disabled duplicate draft, enable/disable/removal, permission and uncertain-request guards, toast, page-only divider, scoped list and full-body pinned details. Initial four failures included premature layout inspection, ambiguous menu queries and cascading pointer suppression; subsequent two failures exposed mutation state preventing automatic menu closure. Explicit controlled menu closure fixes that regression.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-workflows/*.stories.svelte catalog/components/store-application/StoreRepositoryNavigation.stories.svelte catalog/components/store-application/StoreApplication.stories.svelte catalog/components/store-application/RepositoryLoading.stories.svelte
```

**Notes**

Expanded to the full directly affected workflow surface because cards, root/fixture and route navigation are shared. 89/91 tests passed; two stale expectations were updated for five menu items and repository-tab overflow after adding Actions.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-workflows/WorkflowRun.stories.svelte catalog/components/store-application/StoreRepositoryNavigation.stories.svelte
```

**Notes**

Final exact regression rerun: 13 interaction/Axe tests passed, process exit 0. Browser emitted a ResizeObserver loop warning; no test failed. The wider unrelated repository/Rust aggregate suites were not run.
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
pnpm web:check
```

**Notes**

0 errors and 0 warnings.
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

0 errors and 0 warnings; 440-module catalog boundary passed. An intermediate check found two unsupported Testing Library exact options, corrected to anchored role names.
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
pnpm check:architecture
```

**Notes**

1878 modules passed. Earlier production/root/stylesheet and fixture line budget failures were resolved with owned shared sidebar header/menu/paging, navigation helper and tree stylesheet extraction; budgets unchanged.
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
pnpm check:ui-ownership
```

**Notes**

Passed; production remains independent of catalog and external Community source.
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
node scripts/docs.mjs review application "Reviewed native repository Actions scope, shared workflow sidebar action menus, Actions-only body divider, full-body pinned details and navigation guards; updated the browsing guide."
```

**Notes**

Application coverage fingerprint refreshed.
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
pnpm docs:check
```

**Notes**

Documentation closure, links and anchors passed.
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
pnpm exec prettier --write changed UI/catalog/docs/spec files
```

**Notes**

Only named changed paths formatted. An initial coverage path was incorrect; used docs/maintenance/coverage.json.
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
pnpm check:progress
```

**Notes**

Ledger validation passed.
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
pnpm spec:validate
```

**Notes**

Applicable canonical document validators passed.
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
CUA: preserved live service, repository Actions menu/edit/full-body completed run
```

**Notes**

Live page showed seven loaded runs and three source-scoped workflows. Menu has Edit, Run, Duplicate, Disable and Remove; Edit opens the existing draft and Cancel leaves saved data intact. Page border is 1px, shared layout border 0px. Completed a035c8a4 opens publication 400 without workflow sidebar or repository tabs. Proof: .local/repository-actions.png. Initial extra tabs stalled; closing agent-owned temporary tabs and reopening one fresh tab resolved loading, without restarting or changing the service. A plain curl bootstrap read returned 403 and was not used to bypass browser admission. User tab was preserved.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Visual baseline approval remains separate; M28 live providers, remote Forgejo
  protocol and additional-platform gates remain open.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64; JJ 0.45.1; Node 24.10.0; pnpm 10.32.1; Svelte 5.57.1; Design Core 0.3.1; Vitest 4.1.11; Storybook 10.5.7.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Repository-local frontend Actions routes share validated WorkflowSelection filters and exact pins. No native WorkflowStore, CLI, storage, execution or public HTTP API changes.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Native repository Actions tab reuses repository-scoped two-pane History, source/run guard, filters, representative loading and full-body details. Workflow sidebar and overview share icon-bearing guarded actions. Full-width top divider belongs only to the repository Actions page. Before: repository tabs omitted Actions, sidebar workflow rows had no menu and page container had no Actions divider.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Keep M28 In progress; separately qualify visual baselines, remote protocol/platforms and providers. Remove duplicate Workflow host rows from Launch as a separate requested presentation slice.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: Updated JST-WF-008/UI chapter 41, routing presentation under JST-UI-012,
  provenance and browsing guide. Existing toast, authorization, pending intent
  and native execution authority are unchanged.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M28 remains In progress; focused UI and route acceptance do not promote
  provider, protocol, platform or visual parity gates.

```
<!-- /evidence-field -->

