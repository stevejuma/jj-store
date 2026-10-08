---
schema: 1
id: E-COMPUTE-002
---

# E-COMPUTE-002

<!-- evidence-field:id markdown -->
## Id

E-COMPUTE-002
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-08
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M25
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-012
- JST-API-001
- JST-DOC-002
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

rsnonvyu
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-application/AgentFleet.stories.svelte
- catalog/components/store-application/LocalCompute.stories.svelte
- catalog/fixtures/LocalComputeFixture.svelte
- docs/maintenance/coverage.json
- docs/site/guides/settings.md
- spec/src/current/contracts/ui/35-compute-defaults-and-sent-context.md
- spec/src/current/progress/M25/history/M25/index.md
- spec/src/current/progress/M25/index.md
- spec/src/history/2026/10/08/SUMMARY.nav.md
- spec/src/history/2026/10/08/index.md
- spec/src/history/2026/10/SUMMARY.nav.md
- spec/src/history/2026/10/index.md
- web/src/lib/ui/store-agent-execution/StoreAgentAdvanced.svelte
- web/src/lib/ui/store-settings/StoreAgentDefaults.svelte
- web/src/lib/ui/store-settings/StoreComputeHardware.svelte
- web/src/lib/ui/store-settings/StoreComputeModels.svelte
- web/src/lib/ui/store-settings/StoreComputePeers.svelte
- web/src/lib/ui/store-settings/StoreComputeSettings.svelte
- web/src/lib/ui/store-settings/StoreSettingsDisclosure.svelte
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
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/LocalCompute.stories.svelte
```

**Notes**

Before: 2 failed / 2 passed. Compute lacks named compact settings sections and Advanced lacks its Lucide chevron. Source additionally shows naked headings, action borders nested inside bordered groups, horizontally scattered command buttons and no Compute tabs.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/LocalCompute.stories.svelte
```

**Notes**

First repair: 3 passed / 1 failed. The new assertion incorrectly prohibited the top divider between read-only observations and editable fields. Preserved that standard divider and changed the assertion to forbid redundant left/right/bottom group borders, retaining the outer border check.
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

Read-only row integration initially used unsupported code/copy props. Used the existing copyable identity contract; no primitive or upstream change.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/LocalCompute.stories.svelte catalog/components/store-application/AgentFleet.stories.svelte catalog/components/store-application/AgentExecution.stories.svelte catalog/components/store-application/AgentSessionSettings.stories.svelte catalog/components/store-application/AgentHarnessModels.stories.svelte
```

**Notes**

Intermediate selected closure: 32 passed / 1 failed. New global-defaults fixture selected settings before its agent/settings data was ready. Added explicit readiness and scoped Settings navigation; assertions retained.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:check && pnpm catalog:check
```

**Notes**

Web passed. New story role queries incorrectly used the text-only exact option; removed it since role names already match exactly. Explicit untrack documents initial-only paired fixture state and resolves three Svelte warnings.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/LocalCompute.stories.svelte catalog/components/store-application/AgentFleet.stories.svelte
```

**Notes**

After requested tabs: 13 passed / 1 failed. New tab draft test read Import GGUF before async compute status arrived. Awaited that labelled control; no production deadline or assertion changes.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/LocalCompute.stories.svelte catalog/components/store-application/AgentFleet.stories.svelte catalog/components/store-application/AgentExecution.stories.svelte catalog/components/store-application/AgentSessionSettings.stories.svelte catalog/components/store-application/AgentHarnessModels.stories.svelte
```

**Notes**

Final cold run: 5 families / 34 interaction and configured blocking accessibility tests passed in 24.41 seconds. Covers Models/Resources/Sharing visibility and keyboard navigation, retained import/memory/pairing/relay drafts, explicit resource save payload, one outer row-group border, 390px layout with long identities/invitations, equal-width stacked commands, Lucide Advanced and unsaved environment input in Add/Edit/global defaults, and existing model/session lifecycle regressions.
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
pnpm web:check && pnpm catalog:check && pnpm check:architecture && pnpm check:ui-ownership
```

**Notes**

Final web/catalog Svelte checks: 0 errors / 0 warnings. Catalog boundaries: 286 modules. Architecture: 1324 modules. UI ownership passed; no Community runtime/build/type imports, aliases, copied primitives or baseline changes.
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
pnpm docs:review application "Aligned Compute with standard compact Settings sections, divided rows, stacked actions and Models/Resources/Sharing tabs; Advanced uses Lucide while retaining drafts. Runtime and credential contracts unchanged."
```

**Notes**

Updated the application source fingerprint and reviewed Settings guide; source-derived CLI/API contracts remain unchanged.
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
pnpm docs:review && uv run --locked mkdocs build --strict && uv run --locked python scripts/docs/site.py
```

**Notes**

Source fingerprint review passed. Bare uv is outside the command PATH; use the existing explicit /Users/stevejuma/.local/bin/uv, without installing anything.
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
pnpm exec uv --version
```

**Notes**

Confirmed no pnpm-local uv shim. The existing user executable is /Users/stevejuma/.local/bin/uv.
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
/Users/stevejuma/.local/bin/uv --version && /Users/stevejuma/.local/bin/uv run --locked mkdocs build --strict && /Users/stevejuma/.local/bin/uv run --locked python scripts/docs/site.py
```

**Notes**

Focused changed-document build passed in 5.11 seconds. Closure: 141 files / 17,712,114 bytes; links and anchors valid. No Rust build or reference regeneration needed for unchanged CLI/API contracts.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first && pnpm spec:validate && pnpm check:progress
```

**Notes**

Spec-first rejected five plain Storybook file paths in the named-test metadata field, which requires path#fn/def/test declarations. Removed unsupported metadata references; the exact selected story files and names remain recorded in actual validation commands. No production test failure; later commands did not run.
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
pnpm spec:first && pnpm spec:validate && pnpm check:progress
```

**Notes**

Final source closure passed: 12 protected files / 1 canonical chapter, 7 validators / 3172 chapters / 181 requirements, and 146 tasks / 651 evidence records. M25 stays In progress.
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
pnpm exec prettier --check web/src/lib/ui/store-settings/StoreCompute*.svelte web/src/lib/ui/store-settings/StoreSettingsDisclosure.svelte web/src/lib/ui/store-settings/StoreAgentDefaults.svelte web/src/lib/ui/store-agent-execution/StoreAgentAdvanced.svelte catalog/components/store-application/LocalCompute.stories.svelte catalog/components/store-application/AgentFleet.stories.svelte catalog/fixtures/LocalComputeFixture.svelte spec/src/current/contracts/ui/35-compute-defaults-and-sent-context.md spec/src/current/progress/M25/index.md docs/site/guides/settings.md docs/maintenance/coverage.json
```

**Notes**

All affected authored presentation, fixture, guide, source-review and canonical milestone files pass formatting after history generation. Unrelated historical navigation-only rewrites restored.
<!-- /run -->
<!-- /evidence-field -->

