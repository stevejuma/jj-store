---
schema: 1
id: E-WORKFLOW-HISTORY-001
---

# E-WORKFLOW-HISTORY-001

<!-- evidence-field:id markdown -->
## Id

E-WORKFLOW-HISTORY-001
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

- JST-WF-006
- JST-WF-008
- JST-UI-019
- JST-UI-015
- JST-GOV-002
- JST-GOV-008
- JST-DOC-002
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-workflows/StoreWorkflows.stories.svelte
- catalog/components/store-workflows/WorkflowHistory.stories.svelte
- catalog/fixtures/WorkflowsFixture.svelte
- catalog/fixtures/workflows.ts
- docs/maintenance/coverage.json
- docs/site/guides/browsing.md
- docs/ui-provenance.json
- spec/src/current/contracts/api/32-workflows.md
- spec/src/current/contracts/documentation/10-workflows.md
- spec/src/current/contracts/ui/41-workflows.md
- spec/src/current/governance/spec-governance.md
- spec/src/current/progress/M28/index.md
- spec/src/current/verification/workflows.md
- web/src/lib/controller/store-route-codec.ts
- web/src/lib/controller/store-route-types.ts
- web/src/lib/controller/store-route-writer.ts
- web/src/lib/controller/workflow-directory.svelte.ts
- web/src/lib/controller/workflows-route-codec.ts
- web/src/lib/controller/workflows.svelte.ts
- web/src/lib/ui/store-work/StoreWorkOwnershipGroups.svelte
- web/src/lib/ui/store-workflows/StoreWorkflows.svelte
- web/src/lib/ui/store-workflows/WorkflowEditor.svelte
- web/src/lib/ui/store-workflows/WorkflowFilters.svelte
- web/src/lib/ui/store-workflows/WorkflowHistory.svelte
- web/src/lib/ui/store-workflows/WorkflowHistoryRows.svelte
- web/src/lib/ui/store-workflows/WorkflowNavigation.svelte
- web/src/lib/ui/store-workflows/WorkflowOverview.svelte
- web/src/lib/ui/store-workflows/WorkflowPageSkeleton.svelte
- web/src/lib/ui/store-workflows/WorkflowRunDetail.svelte
- web/src/lib/ui/store-workflows/WorkflowRunEvidence.svelte
- web/src/lib/ui/store-workflows/WorkflowRunHeader.svelte
- web/src/lib/ui/store-workflows/WorkflowRunSections.svelte
- web/src/lib/ui/store-workflows/WorkflowRunStatus.svelte
- web/src/lib/ui/store-workflows/workflow-editor.css
- web/src/lib/ui/store-workflows/workflow-history.css
- web/src/lib/ui/store-workflows/workflow-history.svelte.ts
- web/src/lib/ui/store-workflows/workflow-home.ts
- web/tests/vite/workflow-history.test.ts
- web/tests/vite/workflows-route.test.ts
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

UI-only workflow navigation and historical inspection. Workflows/History tabs, filtered task-style Mine/Other runs with server profile avatars, exact URL-selected details, source identity controls, icon-only header actions, initially collapsed task sections and discovery-aware skeletons. Enclosing Jujutsu change zklzvpqw is the evidence revision.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Review visual presentation while retaining independent M28 live provider, Forgejo-wrapper and protocol qualification gaps.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-WF-008 UI and JST-DOC-002 guidance now describe tab navigation,
  bounded history discovery, exact run/attempt/publication URLs, task-style
  sections and loading. JST-WF-006 wire/storage/authorization and
  unknown-request recovery, JST-UI-019 shared toasts and
  JST-UI-015/JST-GOV-002/008 ownership remain unchanged. Catalog lifecycle and
  loading helpers are equivalent extractions to keep the orchestration component
  within its size limit. StoreWorkOwnershipGroups only adds a typed runs label;
  Projects/tasks runtime behaviour is unchanged.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M28 remains In progress. Focused unit and fresh isolated desktop/mobile
  Storybook/Axe and native run inspection do not promote live execution, Forgejo
  wrapper/ConnectRPC or approved visual baselines.

```
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64; JJ 0.45.1; Node 26.9.0; pnpm 10.32.1; Svelte 5.57.1; Design Core 0.3.1; Storybook 10.5.7; Vitest 4.1.11; configured Playwright Chromium.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No native WorkflowStore, manifest, WAL, durable receipt, identity or public wire format changes. Hash routes add validated optional workflow fields, including full uint64 publication pins as decimal text. Documentation checks consume managed help-reference compilation; no independent Rust check/test/build, full root aggregate, release, service restart or fixture reset was run. Provenance/evidence schema 1 retained.
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Approved visual baselines and M28 live agent/Forgejo-wrapper/remote runner
  protocol gates remain open.
- History search filters loaded rows; discovery is bounded to currently
  registered workflow definitions. Exact saved home/run links inspect retained
  removed workflows.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-WORKFLOW-HISTORY-001 details 2](details-02.md)
- [E-WORKFLOW-HISTORY-001 details 3](details-03.md)
