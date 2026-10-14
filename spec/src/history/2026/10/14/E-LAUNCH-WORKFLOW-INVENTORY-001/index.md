---
schema: 1
id: E-LAUNCH-WORKFLOW-INVENTORY-001
---

# E-LAUNCH-WORKFLOW-INVENTORY-001

<!-- evidence-field:schema yaml -->
## Schema

```yaml
1

```
<!-- /evidence-field -->

<!-- evidence-field:id markdown -->
## Id

E-LAUNCH-WORKFLOW-INVENTORY-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M14.2
- M28
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-016
- JST-WF-008
- JST-UI-019
- JST-DOC-001
- JST-DOC-002
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-14
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

ui-verification
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

rlknnuyq
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-application/StoreServerSettings.stories.svelte
- catalog/fixtures/server-settings.ts
- docs/maintenance/coverage.json
- docs/site/guides/settings.md
- spec/src/current/contracts/ui/28-complete-supported-configuration-settings.md
- spec/src/current/progress/M14/index.md
- spec/src/current/progress/M28/index.md
- web/src/lib/ui/store-settings/StoreDeploymentConfiguration.svelte
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

[]
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreServerSettings.stories.svelte -t LaunchInventoryOmitsWorkflowHostRows
```

**Notes**

Before production edit: representative root, nested, Forgejo and indexed admission inventory produced four unwanted Launch labels; 15 unrelated stories skipped. Live Launch also exposed all flattened host/environment fields.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreServerSettings.stories.svelte -t LaunchInventoryOmitsWorkflowHostRows
```

**Notes**

After: exact regression passed, 15 unrelated stories skipped. Other server/network, installation and access groups retained.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreServerSettings.stories.svelte catalog/components/store-application/WorkflowHostSettings.stories.svelte
```

**Notes**

28 directly affected Settings interaction/Axe stories passed, including storage grouping, exact copy, workflow configuration and existing command/toast flows.
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
pnpm web:check
```

**Notes**

0 errors, 0 warnings.
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
pnpm catalog:check
```

**Notes**

0 errors, 0 warnings; catalog ownership boundaries passed.
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
pnpm check:architecture
```

**Notes**

1878 modules passed without budget changes.
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
pnpm check:ui-ownership
```

**Notes**

Owned presentation and released primitives retained.
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
node scripts/docs.mjs review application "Reviewed Local service Launch excluding duplicate workflow host and admission inventory rows; dedicated Workflows configuration and underlying saved/effective inventory remain unchanged."
```

**Notes**

Application fingerprint refreshed after production edit; Settings guide updated.
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
pnpm docs:check
```

**Notes**

Documentation closure, links and anchors passed.
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
pnpm exec prettier --write changed source/catalog/docs/spec paths
```

**Notes**

Named paths only; no aggregate formatting.
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
pnpm check:progress
```

**Notes**

Fresh evidence and explicit unchanged M14.2/M28 reviews passed.
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
pnpm spec:validate
```

**Notes**

Applicable canonical document validators passed.
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
CUA: inspect preserved development Launch and Workflows tabs
```

**Notes**

Launch retained 14 service/installation/access labels and no Workflow host or indexed admission labels. Workflows still showed enabled live-demo-only, runner labels, allowed agents and Forgejo executable/image/Docker settings. No Save, Reload or restart performed. Screenshot: .local/launch-without-workflow-hosts.png. Temporary agent tab closed; user tab preserved.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Visual baseline approval remains separate. M14 lifecycle and M28
  provider/remote-protocol/platform gates remain unchanged.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64; JJ 0.45.1; Node 24.10.0; pnpm 10.32.1; Svelte 5.57.1; Design Core 0.3.1; Vitest 4.1.11; Storybook 10.5.7.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Presentation-only inventory filtering for workflow_host/workflow_hosts root, nested and indexed names. Backend inventory, saved/effective configuration, native WorkflowStore, CLI, runtime activation and public APIs are unchanged.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Remove duplicate workflow host/admission rows from Local service Launch. Keep editing and runtime details in the existing Workflows tab; preserve other Launch and Storage rows and copy behavior. UI-only; no Rust, root aggregate, built-bundle or lifecycle lanes required.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Retain separate visual approval and existing lifecycle/provider/protocol gates. No configuration reload or server restart is needed for this UI slice.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-LAUNCH-WORKFLOW-INVENTORY-001 details 2](details-02.md)
