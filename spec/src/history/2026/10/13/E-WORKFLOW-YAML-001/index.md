---
schema: 1
id: E-WORKFLOW-YAML-001
---

# E-WORKFLOW-YAML-001

<!-- evidence-field:id markdown -->
## Id

E-WORKFLOW-YAML-001
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-13
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M28
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-WF-008
- JST-UI-015
- JST-DOC-002
- JST-WF-006
- JST-GOV-002
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-workflows/WorkflowYaml.stories.svelte
- docs/maintenance/coverage.json
- docs/site/guides/browsing.md
- spec/src/current/contracts/api/32-workflows.md
- spec/src/current/contracts/documentation/10-workflows.md
- spec/src/current/contracts/ui/41-workflows.md
- spec/src/current/progress/M28/index.md
- spec/src/current/verification/workflows.md
- web/src/lib/ui/store-workflows/WorkflowEditor.svelte
- web/src/lib/ui/store-workflows/workflow-editor.css
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

[]
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Create/Edit YAML dialog hides the right inspector and stretches the actual code editor across the padded body. Form inspector visibility and existing draft/validation behaviour remain unchanged. Enclosing Jujutsu change zqulysou is the evidence revision.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Review the full-width presentation; retain independent M28 execution and approved visual gates.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-WF-008 and JST-DOC-002 now describe full-width YAML with no right
  inspector. Existing Form/YAML parsing, hidden Form inspector preference, draft
  custody, toast failures, pending commands, native workflow formats and
  authorization are unchanged. Layout belongs to the application; released
  Design Core/Mira implementations and ownership are unchanged. JST-WF-006 is
  explicitly confirmed unchanged in the API chapter to satisfy the mapped web
  spec-first gate under JST-GOV-002.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M28 remains In progress. Focused UI layout, interaction/Axe, responsive
  and live HMR inspection do not promote live agent/Forgejo execution,
  ConnectRPC compatibility or approved visual baselines.

```
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64; JJ 0.45.1; Node 26.9.0; pnpm 10.32.1; Svelte 5.57.1; Design Core 0.3.1; Storybook 10.5.7; Vitest 4.1.11; configured Playwright Chromium.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

WorkflowStore/native YAML, root/receipt, identity, authorization and wire formats unchanged. UI-only change; no independent Rust build/test/check, root aggregate, development restart, reset, publication or baseline update. Documentation checks consume existing managed help-reference generation. Evidence schema 1 retained.
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- M28 live agent/Forgejo-wrapper/ConnectRPC and approved visual baselines remain
  separate open gates.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-WORKFLOW-YAML-001 details 2](details-02.md)
- [E-WORKFLOW-YAML-001 details 3](details-03.md)
