---
schema: 1
id: E-WORKFLOW-GITHUB-001
---

# E-WORKFLOW-GITHUB-001

<!-- evidence-field:id markdown -->
## Id

E-WORKFLOW-GITHUB-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M28
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-WF-001
- JST-WF-002
- JST-WF-003
- JST-WF-004
- JST-WF-005
- JST-WF-006
- JST-WF-007
- JST-WF-008
- JST-API-001
- JST-UI-016
- JST-UI-019
- JST-CLI-005
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

cross-layer-verification
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

mpprswqs
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- Cargo.lock
- catalog/components/store-application/WorkflowHostSettings.stories.svelte
- catalog/components/store-workflows/WorkflowConsole.stories.svelte
- catalog/components/store-workflows/WorkflowFilePicker.stories.svelte
- catalog/fixtures/WorkflowFilePickerFixture.svelte
- crates/jj-store-cli/src/workflow_host/command.rs
- crates/jj-store-cli/src/workflow_host/config.rs
- crates/jj-store-cli/src/workflow_host/forgejo.rs
- crates/jj-store-cli/src/workflow_host/worker.rs
- crates/jj-store-cli/src/workflow_host.rs
- crates/jj-store-cli/src/workflow_source.rs
- crates/jj-store-cli/tests/server_executable/workflows.rs
- crates/jj-store-service/src/workflow/reads.rs
- crates/jj-store-service/src/workflow/source.rs
- crates/jj-store-service/src/workflow_host/environment.rs
- crates/jj-store-service/src/workflow_host.rs
- crates/jj-store-service/tests/workflow_store.rs
- crates/jj-workflow/Cargo.toml
- crates/jj-workflow/src/github.rs
- crates/jj-workflow/src/lib.rs
- crates/jj-workflow/src/model.rs
- crates/jj-workflow/src/state.rs
- crates/jj-workflow/tests/coordination.rs
- crates/jj-workflow/tests/github.rs
- docs/cli/workflows.md
- docs/maintenance/coverage.json
- docs/site/cli/jj-workflow.md
- docs/site/concepts/collaboration.md
- docs/site/guides/settings.md
- docs/site/reference/api/installation.md
- docs/site/reference/api/types.md
- docs/site/reference/api/workflows.md
- docs/site/technical/architecture.md
- docs/ui-provenance.json
- schemas/openapi.json
- spec/src/current/contracts/api/32-workflows.md
- spec/src/current/contracts/architecture/21-workflow-host.md
- spec/src/current/contracts/cli/33-workflows.md
- spec/src/current/contracts/compatibility/39-workflow-formats.md
- spec/src/current/contracts/ui/41-workflows.md
- spec/src/current/governance/spec-governance.md
- spec/src/current/progress/M28/index.md
- spec/src/history/2026/10/14/E-WORKFLOW-GITHUB-001/SUMMARY.nav.md
- spec/src/history/2026/10/14/E-WORKFLOW-GITHUB-001/details-02.md
- spec/src/history/2026/10/14/E-WORKFLOW-GITHUB-001/details-03.md
- spec/src/history/2026/10/14/E-WORKFLOW-GITHUB-001/details-04.md
- spec/src/history/2026/10/14/E-WORKFLOW-GITHUB-001/index.md
- spec/src/history/2026/10/14/SUMMARY.nav.md
- spec/src/history/2026/10/14/index.md
- web/src/lib/api/generated.ts
- web/src/lib/controller/workflow-forgejo-configuration.ts
- web/src/lib/controller/workflow-host-configuration.ts
- web/src/lib/controller/workflow-runner-environment.ts
- web/src/lib/ui/store-settings/StoreModelFilePicker.svelte
- web/src/lib/ui/store-settings/WorkflowForgejoFields.svelte
- web/src/lib/ui/store-settings/WorkflowRunnerEnvironmentFields.svelte
- web/src/lib/ui/store-workflows/WorkflowConsoleStep.svelte
- web/src/lib/ui/store-workflows/WorkflowExecutorFields.svelte
- web/src/lib/ui/store-workflows/WorkflowFilePicker.svelte
- web/src/lib/ui/store-workflows/WorkflowHistoryRows.svelte
- web/src/lib/ui/store-workflows/WorkflowInspector.svelte
- web/src/lib/ui/store-workflows/WorkflowRunDetail.svelte
- web/src/lib/ui/store-workflows/WorkflowRunSections.svelte
- web/src/lib/ui/store-workflows/WorkflowRunStatus.svelte
- web/src/lib/ui/store-workflows/workflow-card-presentation.ts
- web/src/lib/ui/store-workflows/workflow-console-layout.css
- web/src/lib/ui/store-workflows/workflow-console-reads.ts
- web/src/lib/ui/store-workflows/workflow-console.css
- web/src/lib/ui/store-workflows/workflow-console.ts
- web/src/lib/ui/store-workflows/workflow-editor.css
- web/src/lib/ui/store-workflows/workflow-files.ts
- web/src/lib/ui/store-workflows/workflow-log-lines.ts
- web/src/lib/ui/store-workflows/workflow-presentation.ts
- web/tests/vite/workflow-console.test.ts
- web/tests/vite/workflow-files.test.ts
- web/tests/vite/workflow-host-settings.test.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/server_executable/workflows.rs#github_yaml_executor_matches_pinned_source_and_retains_retry_history
- crates/jj-store-cli/tests/server_executable/workflows.rs#executor_parent_loss_stops_the_owned_process_group
- crates/jj-store-cli/src/workflow_host/forgejo.rs#structured_logs_separate_jobs_steps_and_partial_records
- crates/jj-store-cli/src/workflow_host/command.rs#unicode_pipe_boundaries_and_redaction_windows_preserve_text
- crates/jj-store-service/tests/workflow_store.rs#workflow_grants_receipts_and_pinned_history_share_one_service
- web/tests/vite/workflow-console.test.ts#YAML executor envelopes retain startup
  output without duplicating actual steps
- web/tests/vite/workflow-console.test.ts#scoped YAML step output after a
  terminal report remains available across pages
- web/tests/vite/workflow-files.test.ts#foreign snapshots are rejected and
  discovery does not change the saved path
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-WORKFLOW-GITHUB-001 details 2](details-02.md)
- [E-WORKFLOW-GITHUB-001 details 3](details-03.md)
- [E-WORKFLOW-GITHUB-001 details 4](details-04.md)
