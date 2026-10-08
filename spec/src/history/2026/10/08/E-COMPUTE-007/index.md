---
schema: 1
id: E-COMPUTE-007
---

# E-COMPUTE-007

<!-- evidence-field:id markdown -->
## Id

E-COMPUTE-007
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M25
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-COLLAB-034
- JST-COLLAB-035
- JST-COLLAB-036
- JST-UI-012
- JST-API-001
- JST-CLI-004
- JST-COMPAT-001
- JST-DOC-001
- JST-DOC-002
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-08
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Provider-aware jj-agent configuration and Settings rows
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change tnyoxvlv above b50c5b02
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

The local runner forced OpenAI-compatible launches; provider and model controls were split between an unbordered model picker and Advanced. Two focused new provider regressions failed (Anthropic validation and foreign model inheritance); five existing catalog/settings unit tests and two harness-catalog tests passed. Before the follow-up, the local harness used the jj-agent label, harness options lacked icons and collapsed Advanced had no enclosing rounded group.
<!-- /evidence-field -->

<!-- evidence-field:findings markdown -->
## Findings

Shared bordered Harness/provider/model/advertised-thinking/mode rows now drive Add/Edit and defaults. Service-owned metadata exposes eight providers. Legacy openai keeps compatible endpoint meaning and chat default. Draft POST preview is bounded, credential-scoped and ephemeral. Shared compute uses the selected trusted host; no public pool or automatic fallback is introduced. Existing nonlocal live/remembered catalogs remain available. Launch-option edits and already-pending launches cannot apply foreign live preferences. No native repository identity/publication contract changes. JJ Agent is the display label, with owned icons in all harness options and the selected value. Add/Edit Advanced has one rounded Settings boundary while collapsed or expanded, preserving focus and unsaved drafts. User review exposed the square Button focus treatment clipped by the rounded group. The owned shared disclosure now rounds the trigger corners according to expanded state and keeps a visible inset keyboard outline, eliminating the clipped outer ring.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Cross-layer service/HTTP/supervisor/helper plus shared agent editor/defaults and direct story families. Public Runner/Catalog schema changes justify generated-contract checks. No root aggregate check/test/build/spec:check, unrelated full Rust suite, release qualification or visual baseline update. Shared SettingsDisclosure presentation adjustment additionally covers its Add/Edit, Agent defaults and Compute consumers; direct consumer stories are the appropriate wider UI lane.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-application/AgentDefaults.stories.svelte
- catalog/components/store-application/AgentExecution.stories.svelte
- catalog/components/store-application/AgentFleet.stories.svelte
- catalog/components/store-application/AgentHarnessModels.stories.svelte
- catalog/components/store-application/AgentProviderConfiguration.stories.svelte
- catalog/components/store-application/AgentRuntimes.stories.svelte
- catalog/fixtures/AgentFleetApplicationFixture.svelte
- catalog/fixtures/AgentProviderConfigurationFixture.svelte
- catalog/fixtures/AgentRuntimesFixture.svelte
- catalog/fixtures/agent-execution-session.ts
- crates/jj-store-cli/tests/agent_execution/providers.rs
- crates/jj-store-cli/tests/agent_execution.rs
- crates/jj-store-http/src/agent_execution/tests.rs
- crates/jj-store-http/src/agent_execution.rs
- crates/jj-store-server/src/agent_execution/catalog.rs
- crates/jj-store-server/src/agent_execution/commands.rs
- crates/jj-store-server/src/agent_execution/configuration/tests.rs
- crates/jj-store-server/src/agent_execution/configuration.rs
- crates/jj-store-server/src/agent_execution/local_agent.rs
- crates/jj-store-server/src/agent_execution/mod.rs
- crates/jj-store-server/src/agent_execution/provider_catalog.rs
- crates/jj-store-server/src/agent_execution/provider_launch.rs
- crates/jj-store-server/src/agent_execution/tooling/mod.rs
- crates/jj-store-service/src/agent_execution/catalog.rs
- crates/jj-store-service/src/agent_execution/providers.rs
- crates/jj-store-service/src/agent_execution.rs
- docs/local-compute.md
- docs/maintenance/coverage.json
- docs/site/guides/agent-execution.md
- docs/site/guides/settings.md
- docs/site/reference/api/installation.md
- docs/site/reference/api/types.md
- docs/ui-provenance.json
- runtime/local-agent/Cargo.lock
- runtime/local-agent/Cargo.toml
- runtime/local-agent/UPSTREAM.md
- runtime/local-agent/src/main.rs
- runtime/local-agent/src/provider_catalog.rs
- runtime/local-agent/tests/provider_catalog.rs
- schemas/openapi.json
- spec/src/current/contracts/api/42-compute-and-agent-settings.md
- spec/src/current/contracts/architecture/21-local-compute-resources.md
- spec/src/current/contracts/cli/35-interactive-agent-execution.md
- spec/src/current/contracts/collaboration/40-local-compute-and-agent-configuration.md
- spec/src/current/contracts/compatibility/14-local-inference-runtime.md
- spec/src/current/contracts/documentation/08-interactive-agent-execution.md
- spec/src/current/contracts/mutations/23-installation-agent-configuration.md
- spec/src/current/contracts/ui/35-compute-defaults-and-sent-context.md
- spec/src/current/governance/spec-governance.md
- spec/src/current/progress/M25/history/M25/index.md
- spec/src/current/progress/M25/index.md
- web/src/lib/api/generated.ts
- web/src/lib/controller/agent-configuration.ts
- web/src/lib/controller/agent-model-catalog.svelte.ts
- web/src/lib/ui/community-agent-editor/CommunityAgentEditorDialog.svelte
- web/src/lib/ui/store-agent-execution/StoreAgentAdvanced.svelte
- web/src/lib/ui/store-agent-execution/StoreAgentConfiguration.svelte
- web/src/lib/ui/store-agent-execution/StoreAgentHarnessSelect.svelte
- web/src/lib/ui/store-agent-execution/StoreAgentModelSelect.svelte
- web/src/lib/ui/store-agent-execution/StoreAgentSetupEditor.svelte
- web/src/lib/ui/store-settings/StoreAgentDefaults.svelte
- web/src/lib/ui/store-settings/StoreSettingsDisclosure.svelte
- web/tests/vite/agent-provider-configuration.test.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-server/src/agent_execution/configuration/tests.rs#provider_catalog_accepts_buzz_providers_and_preserves_legacy_compatible_endpoint
- crates/jj-store-server/src/agent_execution/configuration/tests.rs#provider_changes_do_not_inherit_foreign_models_endpoints_or_effort
- crates/jj-store-cli/tests/agent_execution/providers.rs#provider_preview_is_ephemeral_credential_scoped_and_reaped
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-COMPUTE-007 details 2](details-02.md)
- [E-COMPUTE-007 details 3](details-03.md)
- [E-COMPUTE-007 details 4](details-04.md)
