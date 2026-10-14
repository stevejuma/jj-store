---
schema: 1
id: E-WORKFLOW-CARDS-001
---

# E-WORKFLOW-CARDS-001

<!-- evidence-field:id markdown -->
## Id

E-WORKFLOW-CARDS-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M28
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-WF-008
- JST-UI-015
- JST-UI-016
- JST-UI-019
- JST-GOV-002
- JST-GOV-008
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

nkopykom
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-application/WorkflowHostSettings.stories.svelte
- catalog/components/store-workflows/StoreWorkflows.stories.svelte
- catalog/components/store-workflows/WorkflowCards.stories.svelte
- catalog/components/store-workflows/WorkflowConsole.stories.svelte
- catalog/components/store-workflows/WorkflowEditor.stories.svelte
- catalog/components/store-workflows/WorkflowTriggers.stories.svelte
- catalog/components/store-workflows/WorkflowYaml.stories.svelte
- catalog/fixtures/WorkflowsFixture.svelte
- catalog/fixtures/workflows.ts
- docs/cli/workflows.md
- docs/maintenance/coverage.json
- docs/site/cli/jj-workflow.md
- docs/ui-provenance.json
- spec/src/current/contracts/api/32-workflows.md
- spec/src/current/contracts/cli/33-workflows.md
- spec/src/current/contracts/documentation/10-workflows.md
- spec/src/current/contracts/ui/28-complete-supported-configuration-settings.md
- spec/src/current/contracts/ui/41-workflows.md
- spec/src/current/governance/spec-governance.md
- spec/src/current/progress/M28/index.md
- spec/src/history/2026/10/14/E-WORKFLOW-CARDS-001/SUMMARY.nav.md
- spec/src/history/2026/10/14/E-WORKFLOW-CARDS-001/details-02.md
- spec/src/history/2026/10/14/E-WORKFLOW-CARDS-001/index.md
- spec/src/history/2026/10/14/SUMMARY.nav.md
- spec/src/history/2026/10/14/index.md
- web/src/lib/ui/store-settings/StoreWorkflowHostFields.svelte
- web/src/lib/ui/store-workflows/WorkflowCard.svelte
- web/src/lib/ui/store-workflows/WorkflowDescription.svelte
- web/src/lib/ui/store-workflows/WorkflowInspector.svelte
- web/src/lib/ui/store-workflows/WorkflowSkeleton.svelte
- web/src/lib/ui/store-workflows/workflow-card-presentation.ts
- web/src/lib/ui/store-workflows/workflow-card.css
- web/src/lib/ui/store-workflows/workflows.css
- web/tests/vite/workflow-card-presentation.test.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/vite/workflow-card-presentation.test.ts#cards summarize the recorded
  first action and remaining ordered steps
- web/tests/vite/workflow-card-presentation.test.ts#card wording includes native
  trigger limits and agent executor names
- web/tests/vite/workflow-card-presentation.test.ts#long user content is
  normalized for display without mutating its definition
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- M28 live provider, Forgejo wrapper/ConnectRPC, additional platform and
  approved visual baseline gates remain separate.
- Root aggregate checks/tests/builds, Rust and public contract regeneration are
  out of scope for this UI-only slice.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Buzz ef2aa1ae38fadcc0bc22b8bf6ed96b35933146be (local source clean for inspected files, pinned public reference); Node 26.9.0, pnpm 10.32.1, Svelte 5.57.1, Storybook 10.5.7 and Vitest 4.1.11 on macOS ARM64.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Native WorkflowDefinition.description reused; YAML, command/request identities, WorkflowStore formats, API schemas and runner configuration unchanged. Only owned frontend presentation and catalog fixture/test setup change.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Three UI corrections: the collapsed Advanced runner disclosure has a full-width top separator; Create/Edit Form exposes the existing workflow description; overview cards mirror the inspected Buzz stack and semantic action heading. Cards, create control and skeletons are 15rem high. At most three ordered action tiles overlap with the first in front. Summary uses actual native trigger, text/revset/reaction conditions, first named command or agent and remaining-step count; footer shows repository, workflow name and version date. Long summary/name content is bounded and content-width container queries choose columns. Existing menu/switch/drafts/permissions/toasts remain. Description stays with trigger-level workflow controls, retained through selection, mode changes and save. Added a deterministic clock only for an inherited date-sensitive console story.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Commit the verified UI slice; keep M28 In progress and qualify provider, protocol, platform and visual approval gates independently.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: Updated JST-WF-008/JST-UI-016 owning UI chapters, local provenance and
  Change map for card/description/separator behavior. JST-UI-019 toast and
  native storage/API/execution contracts unchanged; standalone owned components
  have no Buzz/Community/catalog runtime dependency.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M28 stays In progress. Source inspection, cold/live responsive review
  and focused interaction/Axe qualify only this presentation slice. No visual
  baseline is accepted and no execution/provider/protocol/platform gate is
  promoted.

```
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-WORKFLOW-CARDS-001 details 2](details-02.md)
