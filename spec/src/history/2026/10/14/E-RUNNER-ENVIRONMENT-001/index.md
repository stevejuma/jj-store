---
schema: 1
id: E-RUNNER-ENVIRONMENT-001
---

# E-RUNNER-ENVIRONMENT-001

<!-- evidence-field:id markdown -->
## Id

E-RUNNER-ENVIRONMENT-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M28
- M27
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

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

ontrkprr
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-application/WorkflowHostSettings.stories.svelte
- catalog/fixtures/ServerSettingsFixture.svelte
- catalog/fixtures/server-settings.ts
- catalog/fixtures/workflow-host-settings.ts
- crates/jj-store-cli/src/server_process.rs
- crates/jj-store-cli/src/workflow_host/discovery.rs
- crates/jj-store-cli/src/workflow_host/managed.rs
- crates/jj-store-cli/src/workflow_host.rs
- crates/jj-store-cli/tests/server_executable/workflows.rs
- crates/jj-store-server/src/config.rs
- crates/jj-store-server/src/process/preparation.rs
- crates/jj-store-server/src/process.rs
- crates/jj-store-server/tests/capture_management/server_settings.rs
- crates/jj-store-service/src/workflow_host/environment.rs
- crates/jj-store-service/src/workflow_host.rs
- docs/cli/workflows.md
- docs/maintenance/coverage.json
- docs/site/cli/jj-workflow.md
- docs/site/guides/settings.md
- docs/site/reference/api/installation.md
- docs/site/reference/api/types.md
- docs/site/reference/configuration.md
- docs/site/reference/transport.md
- docs/site/technical/architecture.md
- docs/ui-provenance.json
- schemas/openapi.json
- spec/src/current/contracts/api/30-installation-configuration-editing.md
- spec/src/current/contracts/api/32-workflows.md
- spec/src/current/contracts/architecture/21-workflow-host.md
- spec/src/current/contracts/cli/23-app-managed-operational-configuration.md
- spec/src/current/contracts/cli/33-workflows.md
- spec/src/current/contracts/compatibility/39-workflow-formats.md
- spec/src/current/contracts/documentation/10-workflows.md
- spec/src/current/contracts/jj-integration/19-workflow-transports.md
- spec/src/current/contracts/mutations/19-workflow-commands.md
- spec/src/current/contracts/ui/28-complete-supported-configuration-settings.md
- spec/src/current/contracts/ui/41-workflows.md
- spec/src/current/governance/spec-governance.md
- spec/src/current/progress/M27/index.md
- spec/src/current/progress/M28/index.md
- spec/src/history/2026/10/14/E-RUNNER-ENVIRONMENT-001/SUMMARY.nav.md
- spec/src/history/2026/10/14/E-RUNNER-ENVIRONMENT-001/details-02.md
- spec/src/history/2026/10/14/E-RUNNER-ENVIRONMENT-001/details-03.md
- spec/src/history/2026/10/14/E-RUNNER-ENVIRONMENT-001/index.md
- spec/src/history/2026/10/14/SUMMARY.nav.md
- spec/src/history/2026/10/14/index.md
- spec/src/history/2026/10/SUMMARY.nav.md
- spec/src/history/2026/10/index.md
- web/src/lib/api/generated.ts
- web/src/lib/controller/workflow-host-choices.svelte.ts
- web/src/lib/controller/workflow-host-configuration.ts
- web/src/lib/controller/workflow-runner-environment.ts
- web/src/lib/ui/store-settings/StoreWorkflowHostFields.svelte
- web/src/lib/ui/store-settings/StoreWorkflowHostSettings.svelte
- web/src/lib/ui/store-settings/WorkflowHostIdentityFields.svelte
- web/src/lib/ui/store-settings/WorkflowHostSources.svelte
- web/src/lib/ui/store-settings/WorkflowRunnerAccess.svelte
- web/src/lib/ui/store-settings/WorkflowRunnerAdvanced.svelte
- web/src/lib/ui/store-settings/WorkflowRunnerEnvironmentFields.svelte
- web/src/lib/ui/store-settings/global-settings-skeleton.ts
- web/tests/vite/workflow-host-choices.test.ts
- web/tests/vite/workflow-host-settings.test.ts
<!-- /evidence-field -->

<!-- evidence-field:removed_files list -->
## Removed files

- web/src/lib/controller/workflow-host-choices.svelte.ts
- web/src/lib/ui/store-settings/WorkflowHostIdentityFields.svelte
- web/src/lib/ui/store-settings/WorkflowHostSources.svelte
- web/tests/vite/workflow-host-choices.test.ts
<!-- /evidence-field -->

<!-- evidence-field:removed_tests list -->
## Removed tests

- web/tests/vite/workflow-host-choices.test.ts#attachment discovery coalesces
  reads and retains last choices after failure
- web/tests/vite/workflow-host-choices.test.ts#unavailable and destroyed readers
  never republish late attachment results
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- M28 live agents, Forgejo wrapper/ConnectRPC, additional platform qualification
  and approved visual baselines remain separate open gates.
- Inherited workflow-console.css exceeds the unchanged architecture size limit;
  no aggregate architecture acceptance claimed.
- Root aggregate check/test/build/spec:check and unrelated Rust/UI lanes are out
  of scope under targeted verification; focused passing checks are not a
  full-suite claim.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1, JJ 0.45.1 custom CLI, Node 26.9.0, pnpm 10.32.1, Svelte 5 and Vitest 4.1.11 on macOS ARM64; existing pinned embedded RustFS.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Additive managed configuration version 2 and ManagedWorkflowHostConfiguration union; version-1 standalone settings and journals retained. Optional derived principal in runtime status. Native WorkflowStore objects, manifests, commands, receipts and report formats unchanged.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Concise runner environment form with collapsed Advanced and separate explicit admission; server/CLI derive the connection and native identity and discover verified registered native workspace links. Recorded jobs still own source revisions. CLI host reuses existing pinned export, dispatch and reporting, with one retained journal per workflow home and an explicit legacy recovery-directory override. Avatar choices keep their image beside selection checkmarks; local host card has 16px clearance; polling uses numeric units and a bounded keyboard-accessible gauge. Removed the redundant UI repository attachment loader and fields, without creating a second installation directory.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Commit the verified runner-environment slice. Keep M28/M27 In progress; qualify live providers, Forgejo protocol, additional platforms and visual baselines independently.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: Updated owning
  architecture/API/CLI/compatibility/JJ/mutations/UI/documentation chapters with
  version-2 environment discovery and explicit authority/recovery boundaries. No
  native workflow/storage or server executor authority change; paired CLI
  remains the executor.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M28 and M27 remain In progress. Isolated three-step real-storage
  execution and settings save/reload qualify this runner simplification, not
  remote Forgejo registration/polling/ConnectRPC, live agent providers,
  Linux/desktop or approved visual parity.

```
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-RUNNER-ENVIRONMENT-001 details 2](details-02.md)
- [E-RUNNER-ENVIRONMENT-001 details 3](details-03.md)
