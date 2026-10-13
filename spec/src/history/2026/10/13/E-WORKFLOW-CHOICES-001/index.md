---
schema: 1
id: E-WORKFLOW-CHOICES-001
---

# E-WORKFLOW-CHOICES-001

<!-- evidence-field:scope markdown -->
## Scope

Fix cold workflow author/thread discovery, hydrate installation names and avatars, display configured agents with explicit unavailable matching semantics, and compose the existing revset editor. Thread scope follows canonical metadata home rather than linked codebase aliases. Retain filters, source leases, unknown saved selections and toast dismissal. The enclosing Jujutsu change is the ledger revision. UI-only targeted policy: Rust, root aggregate build/test/check lanes and provider execution are out of scope. Focused interaction success does not constitute approved visual baselines.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Review visual parity and implement a separately governed agent-message author contract if requested; existing M28 live provider and Forgejo compatibility gates remain pending.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-WF-008 UI chapter now states server-backed trigger choices,
  canonical cross-home thread selection and existing highlighted query
  composition. JST-UI-019 toasts and JST-UI-015 local ownership remain enforced.
  Native workflow semantics and principal-versus-agent identity separation are
  unchanged.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M28 remains In progress. This slice qualifies UI data discovery and
  editor interaction only; no backend event format, remote Forgejo protocol or
  provider execution gate is promoted.

```
<!-- /evidence-field -->

<!-- evidence-field:id markdown -->
## Id

E-WORKFLOW-CHOICES-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M28
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-WF-008
- JST-UI-019
- JST-UI-015
- JST-DOC-002
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-13
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-workflows/WorkflowChoices.stories.svelte
- catalog/components/store-workflows/WorkflowTriggers.stories.svelte
- catalog/fixtures/WorkflowsFixture.svelte
- catalog/fixtures/workflow-collaboration.ts
- docs/maintenance/coverage.json
- docs/ui-provenance.json
- spec/src/current/contracts/api/32-workflows.md
- spec/src/current/contracts/documentation/10-workflows.md
- spec/src/current/contracts/ui/41-workflows.md
- spec/src/current/governance/spec-governance.md
- spec/src/current/progress/M28/index.md
- spec/src/current/verification/workflows.md
- spec/src/history/2026/10/13/E-WORKFLOW-CHOICES-001/SUMMARY.nav.md
- spec/src/history/2026/10/13/E-WORKFLOW-CHOICES-001/details-02.md
- spec/src/history/2026/10/13/E-WORKFLOW-CHOICES-001/index.md
- spec/src/history/2026/10/13/SUMMARY.nav.md
- spec/src/history/2026/10/13/index.md
- web/src/lib/controller/http-threads-data-source.ts
- web/src/lib/controller/repository-thread-navigation.svelte.ts
- web/src/lib/controller/thread-types.ts
- web/src/lib/ui/store-application/StoreRepositoryPicker.svelte
- web/src/lib/ui/store-application/repository-picker.css
- web/src/lib/ui/store-workflows/StoreWorkflows.svelte
- web/src/lib/ui/store-workflows/WorkflowAuthorPicker.svelte
- web/src/lib/ui/store-workflows/WorkflowConditions.svelte
- web/src/lib/ui/store-workflows/WorkflowEditor.svelte
- web/src/lib/ui/store-workflows/WorkflowForm.svelte
- web/src/lib/ui/store-workflows/WorkflowInspector.svelte
- web/src/lib/ui/store-workflows/WorkflowRevisionMatch.svelte
- web/src/lib/ui/store-workflows/WorkflowTextCondition.svelte
- web/src/lib/ui/store-workflows/WorkflowThreadPicker.svelte
- web/src/lib/ui/store-workflows/workflow-code-options.svelte.ts
- web/src/lib/ui/store-workflows/workflow-presentation.ts
- web/tests/vite/repository-thread-navigation.test.ts
- web/tests/vite/workflow-code-options.test.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/vite/workflow-code-options.test.ts#cold source discovery pages
  history at one explicit operation and query time
- web/tests/vite/workflow-code-options.test.ts#an old source reply cannot
  populate author or syntax choices for another repository
- web/tests/vite/workflow-code-options.test.ts#a mismatched history snapshot is
  refused and retains a retryable error
- web/tests/vite/repository-thread-navigation.test.ts#fresh shared thread list
  reads bypass cached empty pages without changing detail pins
- web/tests/vite/workflow-code-options.test.ts#native author discovery excludes
  independent local companion actors
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-WORKFLOW-CHOICES-001 details 2](details-02.md)
- [E-WORKFLOW-CHOICES-001 details 3](details-03.md)
