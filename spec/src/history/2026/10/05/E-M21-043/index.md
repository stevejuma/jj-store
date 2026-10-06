---
schema: 1
id: E-M21-043
---

# E-M21-043

<!-- evidence-field:id markdown -->
## Id

E-M21-043
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M21
- M21.1
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-05
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

sqztzrsrmmltlmtqqrmkpyxxrtwyzxpw
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Workspace Settings single Details frame, matching row padding and inline actions
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Existing JJ/jj-lib 0.45.1 and foundational package ranges unchanged; Vitest 4.1.11, Node 26.9.0, pnpm 10.32.1.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Presentation only. Local TOML/command version 1, native state, OpenAPI/generated types and layout formats unchanged.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-stack-workspace/StoreStackWorkspaceSettings.stories.svelte
- docs/maintenance/coverage.json
- docs/site/guides/settings.md
- spec/src/api.md
- spec/src/documentation.md
- spec/src/milestones.md
- spec/src/ui.md
- web/src/lib/ui/community-channel-settings/CommunityChannelSettings.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceSettings.svelte
- web/tests/real/stacks-browser.spec.ts
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-020
- JST-DOC-002
- JST-GOV-002
- JST-GOV-003
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Workspace Settings nested a bordered field group and action row inside the Threads Details frame. Workspace ID followed the form, Save settings/Cancel stacked beside explanatory text. Its 8.8px/8px padding differed from editable rows' 12px/16px.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Reuse existing owned Threads Settings composition, read-only copy row, Settings field group, Button, Separator and Lucide icons. A custom details snippet supplies one rounded frame, while the default Threads frame stays unchanged. Workspace ID is first with scoped matching 12px block/16px inline padding. Name, target and the icon Cancel/Save action row remain inside one frame with an internal separator and no action explanation. Draft/validation/preview/confirmation behavior stays unchanged. The existing cold browser locator is renamed to Save.
<!-- /evidence-field -->

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceSettings.stories.svelte
```

**Notes**

before.log: all four Settings stories reject missing first Workspace ID row.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceSettings.stories.svelte catalog/components/store-threads/StoreThreadAlignment.stories.svelte
```

**Notes**

stories.log: initial seven pass; subsequent user correction keeps buttons inside the shared frame and requests equal ID padding.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceSettings.stories.svelte -t EditWorkspaceSettingsWithRetainedDraft
```

**Notes**

before-contained-actions.log: one expected contained-action failure, three skipped.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceSettings.stories.svelte catalog/components/store-threads/StoreThreadAlignment.stories.svelte
```

**Notes**

contained-stories.log: four Settings failures measure 8.8px versus 12px ID block padding; three Threads pass. Svelte removed the selector because the wrapper is a child component. Changed to a global selector scoped by the workspace-specific class.
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
pnpm web:check; pnpm catalog:check
```

**Notes**

web-check-final.log: zero errors, one unused CSS selector warning. catalog-check-final.log rejects that warning. Corrected scoped descendant selector without weakening geometry assertions.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceSettings.stories.svelte catalog/components/store-threads/StoreThreadAlignment.stories.svelte
```

**Notes**

verified-stories.log: final seven stories pass. Retained edit/preview/cancel/readonly/Delete behavior and unchanged Threads interaction/accessibility included.
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
pnpm web:check; pnpm catalog:check; pnpm check:ui-ownership; pnpm check:architecture
```

**Notes**

Initial web/catalog/ownership/architecture logs pass; final web-check-verified.log/catalog-check-verified.log have zero errors/warnings and 229 catalog boundaries. Architecture 1096 modules, ownership 23 chapters/173 requirements pass.
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
pnpm spec:first
```

**Notes**

spec-first.log: docs guide and coverage update lacked mapped documentation.md. Added the governing Settings documentation requirement.
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
pnpm spec:validate
```

**Notes**

spec-validate.log: five validators, 23 chapters, 173 requirements.
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
pnpm exec prettier --write (changed supported source/spec/guide and coverage files); pnpm exec prettier --check (same paths, historical ledger excluded)
```

**Notes**

format.log, docs-format.log, documentation-format.log, contained-format.log, css-format.log, coverage-format.log, final-coverage-format.log, verified-coverage-format.log, format-check.log and format-verified.log.
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
pnpm docs:review application (specific Details ordering/actions review); pnpm docs:check
```

**Notes**

docs-review.log/docs-review-final.log/docs-review-verified.log acknowledge only application source changes after reviewing the Settings guide. docs-check.log/docs-check-final.log: nine diagrams, 139 files / 13,952,428 bytes, valid links/anchors. No UI/Rust build required.
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
CUA read-only live demo Settings review at http://127.0.0.1:7476/
```

**Notes**

Initial view confirmed single frame/row order/icons; final geometry confirms 1px frame, no outer nested frame, Save/Cancel both inside, and ID/name paddings of 12px 16px 12px 16px. Live saved workspace values untouched; no baseline update.
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
pnpm check:progress; pnpm spec:first; pnpm spec:validate; pnpm check:architecture; pnpm check:ui-ownership
```

**Notes**

Final verified logs: 142 tasks, six protected files/four canonical chapters, five validators/23 chapters/173 requirements, 1096 architecture modules and ownership pass. Historical 524 evidence objects and raw JSON prefix preserved.
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

<!-- evidence-field:tests list -->
## Tests

[]
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Commit this verified UI refinement and preserve live Vite 7476/backend 7478.
<!-- /evidence-field -->

<!-- evidence-field:summary markdown -->
## Summary

Seven directly affected Settings/Threads interaction/accessibility stories pass, with first-row, one-frame, equal four-sided padding, contained actions, separator, icons and aligned button geometry assertions. Type/ownership/architecture/docs checks and live HMR measurements pass. No milestone promotion.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M21-043 details 2](details-02.md)
