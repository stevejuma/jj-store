---
schema: 1
id: E-WORKFLOW-CONSOLE-001
---

# E-WORKFLOW-CONSOLE-001

<!-- evidence-field:id markdown -->
## Id

E-WORKFLOW-CONSOLE-001
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

- JST-GOV-002
- JST-GOV-008
- JST-API-006
- JST-WF-008
- JST-WF-006
- JST-UI-019
- JST-DOC-001
- JST-DOC-002
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-workflows/StoreWorkflows.stories.svelte
- catalog/components/store-workflows/WorkflowConsole.stories.svelte
- catalog/components/store-workflows/WorkflowHistory.stories.svelte
- catalog/fixtures/WorkflowsFixture.svelte
- catalog/fixtures/workflow-console.ts
- docs/maintenance/coverage.json
- docs/site/guides/browsing.md
- docs/ui-provenance.json
- package.json
- pnpm-lock.yaml
- spec/src/current/contracts/api/32-workflows.md
- spec/src/current/contracts/architecture/21-workflow-host.md
- spec/src/current/contracts/documentation/10-workflows.md
- spec/src/current/contracts/ui/41-workflows.md
- spec/src/current/governance/spec-governance.md
- spec/src/current/progress/M28/index.md
- spec/src/current/verification/workflows.md
- spec/src/history/2026/10/13/E-WORKFLOW-CONSOLE-001/SUMMARY.nav.md
- spec/src/history/2026/10/13/E-WORKFLOW-CONSOLE-001/details-02.md
- spec/src/history/2026/10/13/E-WORKFLOW-CONSOLE-001/details-03.md
- spec/src/history/2026/10/13/E-WORKFLOW-CONSOLE-001/index.md
- spec/src/history/2026/10/13/SUMMARY.nav.md
- spec/src/history/2026/10/13/index.md
- web/src/lib/ui/store-workflows/WorkflowConsoleLogs.svelte
- web/src/lib/ui/store-workflows/WorkflowConsoleSettings.svelte
- web/src/lib/ui/store-workflows/WorkflowConsoleStep.svelte
- web/src/lib/ui/store-workflows/WorkflowPageSkeleton.svelte
- web/src/lib/ui/store-workflows/WorkflowRunDetail.svelte
- web/src/lib/ui/store-workflows/WorkflowRunEvidence.svelte
- web/src/lib/ui/store-workflows/WorkflowRunHeader.svelte
- web/src/lib/ui/store-workflows/WorkflowRunSections.svelte
- web/src/lib/ui/store-workflows/WorkflowRunStatus.svelte
- web/src/lib/ui/store-workflows/workflow-console-reads.ts
- web/src/lib/ui/store-workflows/workflow-console.css
- web/src/lib/ui/store-workflows/workflow-console.ts
- web/src/lib/ui/store-workflows/workflow-history.css
- web/src/lib/ui/store-workflows/workflow-log-lines.ts
- web/tests/vite/workflow-console.test.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

[]
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pass
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

[]
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Owned Svelte Forgejo-style workflow run console, latest/historical attempts, pinned lazy step logs, ANSI/chunk assembly and nested groups, console settings/fullscreen/downloads, metadata/avatar/header retry and representative console skeleton. Includes an explicitly requested grouped second native demo attempt, with temporary previously authorized host admission restored. Enclosing Jujutsu change kuwuxqyw; parent wtltlprp/91421d00. No unrelated work existed at start.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Keep the grouped native demo available. Qualify external Forgejo/provider execution and obtain visual-baseline approval separately.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64; JJ 0.45.1; Node 26.9.0; pnpm 10.32.1; Vitest 4.1.11; Storybook 10.5.7; Svelte 5.57.1; ansi_up direct exact dependency 6.0.6. Source reference Forgejo v15.0.7.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Manifest/WorkflowStore format 6 and public request/report/evidence version 1 unchanged. Frontend evidence projection only; no Rust, CLI, API/schema or storage-format implementation changes. No installed executable replacement, reset or publication. Private operator scripts, admission backup, retry receipt and host journal stay outside checkout.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-WF-008 and UI/41 now specify the independent Forgejo-style console,
  preserved metadata, internal attempts/settings, lazy pinned log reads, nested
  groups, fullscreen layering, downloads and guarded latest-attempt Re-run all
  jobs. JST-WF-001 through 007 authority/formats remain unchanged; retry still
  submits the latest recorded expected attempt, and the server retains
  CAS/status guards. Operational errors use JST-UI-019 shared retry toasts.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M28 remains In progress. Focused unit, desktop/mobile interaction/Axe,
  native grouped attempt and exact post-restart history/download evidence
  qualify this UI slice. Actual Forgejo wrapper/registration/polling/ConnectRPC,
  live agent providers and approved visual baselines remain separate open gates.

```
<!-- /evidence-field -->

<!-- evidence-field:out_of_scope list -->
## Out of scope

- External Forgejo runner protocol, ConnectRPC, provider execution and remote
  CI/CD compatibility remain deferred/unqualified; this demo uses existing
  admitted native named /bin/sh commands.
- Visual baseline approval is separate; no baseline changed or approved.
- Root aggregate check/test/build/spec:check and full Rust/UI/browser suites are
  not run under the targeted UI verification policy. The documentation checker
  invokes its existing managed CLI reference generator; this is not Rust
  execution qualification.
- Multi-tab refresh latency and general workflow lifecycle qualification are not
  promoted by these focused checks.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-WORKFLOW-CONSOLE-001 details 2](details-02.md)
- [E-WORKFLOW-CONSOLE-001 details 3](details-03.md)
