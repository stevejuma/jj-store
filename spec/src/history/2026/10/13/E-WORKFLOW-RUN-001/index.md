---
schema: 1
id: E-WORKFLOW-RUN-001
---

# E-WORKFLOW-RUN-001

<!-- evidence-field:id markdown -->
## Id

E-WORKFLOW-RUN-001
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

- catalog/components/store-workflows/WorkflowRun.stories.svelte
- catalog/fixtures/WorkflowRunFixture.svelte
- catalog/fixtures/workflow-run-source.ts
- docs/cli/workflows.md
- docs/maintenance/coverage.json
- docs/site/cli/jj-workflow.md
- docs/site/guides/browsing.md
- docs/ui-provenance.json
- spec/src/current/contracts/api/32-workflows.md
- spec/src/current/contracts/documentation/10-workflows.md
- spec/src/current/contracts/ui/41-workflows.md
- spec/src/current/governance/spec-governance.md
- spec/src/current/progress/M28/index.md
- spec/src/current/verification/workflows.md
- web/src/lib/ui/store-workflows/StoreWorkflows.svelte
- web/src/lib/ui/store-workflows/WorkflowCard.svelte
- web/src/lib/ui/store-workflows/WorkflowRunDialog.svelte
- web/src/lib/ui/store-workflows/workflow-run-source.svelte.ts
- web/tests/vite/workflow-run-source.test.ts
- spec/src/current/contracts/cli/33-workflows.md
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

Settings-style manual Run form with native operation/commit/bookmark CommandView choices and icon-bearing, wider overview menu. UI-only source-read and presentation slice; unchanged execution authority and wire/storage formats. Enclosing Jujutsu change vypkkkqw is the evidence revision.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Review visual presentation; retain M28 live provider, Forgejo-wrapper and protocol qualification gaps.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-WF-008 UI presentation and JST-DOC-002 guidance now describe source
  selectors and overview actions. JST-WF-006 source/command wire contracts,
  JST-UI-019 shared feedback, JST-UI-015 and JST-GOV-002/008 ownership remain
  unchanged. The orchestration component reuses a source-name helper and
  equivalent optional-remove return to retain its size bound.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M28 remains In progress. Focused unit, cold isolated Storybook/Axe and
  responsive checks do not promote live agent execution, Forgejo
  wrapper/ConnectRPC or approved visual baselines.

```
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64; JJ 0.45.1; Node 26.9.0; pnpm 10.32.1; Svelte 5.57.1; Design Core 0.3.1; Storybook 10.5.7; Vitest 4.1.11; configured Playwright Chromium.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No WorkflowStore, native identity, manifest-6, WAL, receipt, authorization or public API format change. Provenance/evidence schema 1 retained. docs:generate/docs:check use the managed Rust help-reference compiler as a documentation consumer; no independent Rust check/test/build lane or root aggregate suite was run.
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Approved visual baselines and existing M28 live provider, Forgejo-wrapper and
  remote protocol gates remain separate.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-WORKFLOW-RUN-001 details 2](details-02.md)
- [E-WORKFLOW-RUN-001 details 3](details-03.md)
