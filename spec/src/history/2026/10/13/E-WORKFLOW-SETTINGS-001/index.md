---
schema: 1
id: E-WORKFLOW-SETTINGS-001
---

# E-WORKFLOW-SETTINGS-001

<!-- evidence-field:id markdown -->
## Id

E-WORKFLOW-SETTINGS-001
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

2026-10-13
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

cross-layer-verification
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

zlssvywu
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- Cargo.lock
- catalog/components/store-application/RuntimeReload.stories.svelte
- catalog/components/store-application/WorkflowHostSettings.stories.svelte
- catalog/fixtures/RuntimeReloadFixture.svelte
- catalog/fixtures/ServerSettingsFixture.svelte
- catalog/fixtures/server-settings.ts
- catalog/fixtures/workflow-host-settings.ts
- crates/jj-store-cli/src/server_process.rs
- crates/jj-store-cli/src/workflow_host/config.rs
- crates/jj-store-cli/src/workflow_host/managed.rs
- crates/jj-store-cli/src/workflow_host.rs
- crates/jj-store-cli/tests/server_executable/workflows.rs
- crates/jj-store-http/src/lib.rs
- crates/jj-store-http/src/workflow.rs
- crates/jj-store-rpc/src/server.rs
- crates/jj-store-server/src/config/types.rs
- crates/jj-store-server/src/config.rs
- crates/jj-store-server/src/installation.rs
- crates/jj-store-server/src/process/authority.rs
- crates/jj-store-server/src/process/preparation.rs
- crates/jj-store-server/src/process/reload.rs
- crates/jj-store-server/src/process.rs
- crates/jj-store-server/src/runtime_reload/mod.rs
- crates/jj-store-server/src/runtime_reload/tests.rs
- crates/jj-store-server/src/server_settings.rs
- crates/jj-store-server/tests/capture_management/server_settings.rs
- crates/jj-store-service/Cargo.toml
- crates/jj-store-service/src/lib.rs
- crates/jj-store-service/src/runtime_reload.rs
- crates/jj-store-service/src/server_settings.rs
- crates/jj-store-service/src/workflow/commands.rs
- crates/jj-store-service/src/workflow.rs
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
- spec/src/history/2026/10/13/E-WORKFLOW-SETTINGS-001/SUMMARY.nav.md
- spec/src/history/2026/10/13/E-WORKFLOW-SETTINGS-001/details-02.md
- spec/src/history/2026/10/13/E-WORKFLOW-SETTINGS-001/details-03.md
- spec/src/history/2026/10/13/E-WORKFLOW-SETTINGS-001/index.md
- spec/src/history/2026/10/13/SUMMARY.nav.md
- spec/src/history/2026/10/13/index.md
- web/src/lib/api/generated.ts
- web/src/lib/controller/global-settings-navigation.ts
- web/src/lib/controller/runtime-reload-validation.ts
- web/src/lib/controller/runtime-reload.svelte.ts
- web/src/lib/controller/server-configuration.ts
- web/src/lib/controller/server-settings.svelte.ts
- web/src/lib/controller/workflow-host-choices.svelte.ts
- web/src/lib/controller/workflow-host-configuration.ts
- web/src/lib/ui/store-settings/StoreConfigurationEditor.svelte
- web/src/lib/ui/store-settings/StoreRuntimeReload.svelte
- web/src/lib/ui/store-settings/StoreServiceSettings.svelte
- web/src/lib/ui/store-settings/StoreSettingsPage.svelte
- web/src/lib/ui/store-settings/StoreWorkflowHostFields.svelte
- web/src/lib/ui/store-settings/StoreWorkflowHostSettings.svelte
- web/src/lib/ui/store-settings/WorkflowHostConfigEditor.svelte
- web/src/lib/ui/store-settings/WorkflowHostIdentityFields.svelte
- web/src/lib/ui/store-settings/WorkflowHostPrincipalPicker.svelte
- web/src/lib/ui/store-settings/WorkflowHostSources.svelte
- web/src/lib/ui/store-settings/global-settings-skeleton.ts
- web/tests/vite/workflow-host-choices.test.ts
- web/tests/vite/workflow-host-settings.test.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-server/src/runtime_reload/tests.rs#workflow_hosts_participate_in_saved_reload
- crates/jj-store-service/src/workflow.rs#replacement_fences_live_publication_and_updates_every_clone
- crates/jj-store-server/tests/capture_management/server_settings.rs#workflow_settings_save_admission_and_local_config_atomically_without_execution
- crates/jj-store-cli/src/workflow_host/managed.rs#idle_disabled_runtime_is_reloadable_and_shutdown_drains_poll
- crates/jj-store-cli/tests/server_executable/workflows.rs#two_hosts_execute_one_pinned_run_and_preserve_history_after_restart
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- M28 live agents, Forgejo wrapper/ConnectRPC, additional platform qualification
  and approved visual baselines remain separate open gates.
- The inherited unchanged workflow-console.css exceeds the architecture size
  limit (411 counted lines versus 300). Parent matches byte-for-byte; no
  aggregate architecture pass claimed.
- Root aggregate check/test/build/spec:check and unrelated Rust/UI suites were
  not run under the repository targeted policy.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1, pnpm 10.32.1, Svelte 5.57.1, released Design Core 0.3.1; macOS ARM64 and existing pinned embedded RustFS runtime.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Manifest format 6, WorkflowStore version 1, original host journal version 1 and standalone flat operator JSON version 1 unchanged. Existing version-1 local settings/reload API gains owned workflow configuration, runtime status and workflows domain.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

LocalTrusted Admin conditional saves for shared execution admission and optional CLI-owned local host. Save does not start execution. Explicit reload retains listeners/storage and waits for host polling/job custody. Owned Settings rows, chip labels, numeric polling units and bounded advanced JSON retain drafts and toast feedback. Command View principal/profile/avatar, native attachment, agent setup and target selectors retain configured unknown values. A right-aligned header Form/JSON toggle shows one editor, shares drafts and validates raw edits before switching or Save. Canonical set ordering and defaults match server acknowledgements.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Retain M27/M28 In progress. Review visual baselines and qualify deferred provider/Forgejo protocol and platform gates independently.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: Updated API, architecture, CLI, UI, transport, compatibility, mutations,
  documentation and ownership topics for optional managed hosting and reloadable
  admission. Native workflow objects/commands/CAS/receipts, standalone JSON
  shape and Forgejo protocol deferral are unchanged.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M28/M27 retain In progress. This slice adds operational configuration;
  it does not promote provider, Forgejo protocol, additional platforms or visual
  baselines.

```
<!-- /evidence-field -->

