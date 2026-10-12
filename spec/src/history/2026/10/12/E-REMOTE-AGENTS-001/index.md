---
schema: 1
id: E-REMOTE-AGENTS-001
---

# E-REMOTE-AGENTS-001

<!-- evidence-field:id markdown -->
## Id

E-REMOTE-AGENTS-001
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-12
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M27
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-SEC-009
- JST-API-011
- JST-UI-022
- JST-GOV-002
- JST-GOV-008
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

implementation-verification
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

mulvtstp
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- Cargo.lock
- catalog/components/store-application/ServerConnectionOwners.stories.svelte
- catalog/components/store-application/ServerConnections.stories.svelte
- catalog/components/store-application/SettingsNavigation.stories.svelte
- catalog/controller/in-memory-store-controller.svelte.ts
- catalog/controller/in-memory-store-options.ts
- catalog/controller/in-memory-store-settings.svelte.ts
- catalog/fixtures/ServerConnectionsFixture.svelte
- catalog/fixtures/StoreApplicationFixture.svelte
- catalog/fixtures/server-connections/owner.ts
- crates/jj-store-http/Cargo.toml
- crates/jj-store-http/src/agent_execution.rs
- crates/jj-store-http/src/application_events/producers.rs
- crates/jj-store-http/src/connections.rs
- crates/jj-store-http/src/contract/remote_agents.rs
- crates/jj-store-http/src/contract/tests.rs
- crates/jj-store-http/src/contract.rs
- crates/jj-store-http/src/dto.rs
- crates/jj-store-http/src/lib.rs
- crates/jj-store-http/src/remote_agents/tests.rs
- crates/jj-store-http/src/remote_agents.rs
- crates/jj-store-rpc/src/server.rs
- crates/jj-store-rpc/tests/delegated.rs
- crates/jj-store-server/Cargo.toml
- crates/jj-store-server/src/agent_execution/conversation_home.rs
- crates/jj-store-server/src/agent_execution/conversation_links.rs
- crates/jj-store-server/src/agent_execution/conversations.rs
- crates/jj-store-server/src/agent_execution/feedback.rs
- crates/jj-store-server/src/agent_execution/fleet.rs
- crates/jj-store-server/src/agent_execution/mod.rs
- crates/jj-store-server/src/agent_execution/publication.rs
- crates/jj-store-server/src/agent_execution/targets.rs
- crates/jj-store-server/src/agent_execution/workspace_inspection/source.rs
- crates/jj-store-server/src/config/types.rs
- crates/jj-store-server/src/config.rs
- crates/jj-store-server/src/connections/events.rs
- crates/jj-store-server/src/connections/mod.rs
- crates/jj-store-server/src/connections/tests.rs
- crates/jj-store-server/src/connections/transport.rs
- crates/jj-store-server/src/installation.rs
- crates/jj-store-server/src/lib.rs
- crates/jj-store-server/src/process/preparation.rs
- crates/jj-store-server/src/process/running.rs
- crates/jj-store-server/src/process.rs
- crates/jj-store-server/src/remote_agents/caller.rs
- crates/jj-store-server/src/remote_agents/config.rs
- crates/jj-store-server/src/remote_agents/execution_tests.rs
- crates/jj-store-server/src/remote_agents/lease.rs
- crates/jj-store-server/src/remote_agents/mod.rs
- crates/jj-store-server/src/remote_agents/tests.rs
- crates/jj-store-server/tests/remote_agent_configuration.rs
- crates/jj-store-service/src/access.rs
- crates/jj-store-service/src/connections.rs
- crates/jj-store-service/src/lib.rs
- crates/jj-store-service/src/native.rs
- crates/jj-store-service/src/remote_agents.rs
- crates/jj-store-service/src/transport.rs
- crates/jj-store-service/tests/authorization.rs
- docs/agent-execution.md
- docs/cli/server-config.md
- docs/maintenance/coverage.json
- docs/site/cli/jj-store.md
- docs/site/guides/agent-execution.md
- docs/site/reference/api/installation.md
- docs/site/reference/api/types.md
- docs/site/reference/configuration.md
- docs/site/reference/transport.md
- docs/ui-provenance.json
- schemas/openapi.json
- scripts/remote-connection-browser.mjs
- spec/src/current/contracts/api/45-remote-connections-and-agents.md
- spec/src/current/contracts/api/SUMMARY.nav.md
- spec/src/current/contracts/api/index.md
- spec/src/current/contracts/architecture/14-remote-connection-ownership.md
- spec/src/current/contracts/architecture/SUMMARY.nav.md
- spec/src/current/contracts/architecture/index.md
- spec/src/current/contracts/cli/38-remote-agent-configuration.md
- spec/src/current/contracts/cli/SUMMARY.nav.md
- spec/src/current/contracts/cli/index.md
- spec/src/current/contracts/collaboration/44-authenticated-agent-admission.md
- spec/src/current/contracts/collaboration/SUMMARY.nav.md
- spec/src/current/contracts/collaboration/index.md
- spec/src/current/contracts/compatibility/38-remote-connections.md
- spec/src/current/contracts/compatibility/SUMMARY.nav.md
- spec/src/current/contracts/compatibility/index.md
- spec/src/current/contracts/documentation/09-remote-agent-connections.md
- spec/src/current/contracts/documentation/SUMMARY.nav.md
- spec/src/current/contracts/documentation/index.md
- spec/src/current/contracts/jj-integration/33-delegated-agent-workspaces.md
- spec/src/current/contracts/jj-integration/SUMMARY.nav.md
- spec/src/current/contracts/jj-integration/index.md
- spec/src/current/contracts/mutations/24-remote-agent-authority.md
- spec/src/current/contracts/mutations/SUMMARY.nav.md
- spec/src/current/contracts/mutations/index.md
- spec/src/current/contracts/security/12-remote-connections-and-agents.md
- spec/src/current/contracts/security/SUMMARY.nav.md
- spec/src/current/contracts/security/index.md
- spec/src/current/contracts/ui/40-server-switcher.md
- spec/src/current/contracts/ui/SUMMARY.nav.md
- spec/src/current/contracts/ui/index.md
- spec/src/current/governance/spec-governance.md
- spec/src/current/progress/M27/SUMMARY.nav.md
- spec/src/current/progress/M27/history/M27/SUMMARY.nav.md
- spec/src/current/progress/M27/history/M27/index.md
- spec/src/current/progress/M27/history/SUMMARY.nav.md
- spec/src/current/progress/M27/history/index.md
- spec/src/current/progress/M27/index.md
- spec/src/current/progress/SUMMARY.nav.md
- spec/src/current/progress/index.md
- spec/src/current/verification/SUMMARY.nav.md
- spec/src/current/verification/api.md
- spec/src/current/verification/index.md
- spec/src/current/verification/sec.md
- spec/src/current/verification/ui-04.md
- web/src/App.svelte
- web/src/lib/api/application-events-websocket.ts
- web/src/lib/api/generated.ts
- web/src/lib/connections/agent-transport.ts
- web/src/lib/connections/browser.ts
- web/src/lib/connections/context.ts
- web/src/lib/connections/management.ts
- web/src/lib/connections/owner.svelte.ts
- web/src/lib/connections/preferences.ts
- web/src/lib/connections/startup.ts
- web/src/lib/connections/transport.ts
- web/src/lib/connections/workspaces.svelte.ts
- web/src/lib/controller/agent-execution-refresh.ts
- web/src/lib/controller/agent-execution.svelte.ts
- web/src/lib/controller/agent-selection-history.ts
- web/src/lib/controller/application-freshness.ts
- web/src/lib/controller/contracts.ts
- web/src/lib/controller/controller-actions.ts
- web/src/lib/controller/production-connection-activity.ts
- web/src/lib/controller/production-connection-views.ts
- web/src/lib/controller/production-local-configuration.ts
- web/src/lib/controller/production-session-events.ts
- web/src/lib/controller/production-store-controller.svelte.ts
- web/src/lib/controller/production-store-lifecycle.svelte.ts
- web/src/lib/controller/store-route-browser.ts
- web/src/lib/ui/community-agent-profile/CommunityAgentProfile.svelte
- web/src/lib/ui/community-agent-profile/CommunityAgentRuntimeFields.svelte
- web/src/lib/ui/community-agents/CommunityAgents.svelte
- web/src/lib/ui/store-agent-chat/StoreAgentChatRepositories.svelte
- web/src/lib/ui/store-agent-execution/StoreAgentDetailsSidebar.svelte
- web/src/lib/ui/store-agent-execution/StoreAgentExecutionHost.svelte
- web/src/lib/ui/store-agent-execution/StoreAgentRuntimeSettings.svelte
- web/src/lib/ui/store-application/StoreApplication.svelte
- web/src/lib/ui/store-application/StoreApplicationHeaderActions.svelte
- web/src/lib/ui/store-connections/StoreConnectionDialog.svelte
- web/src/lib/ui/store-connections/StoreConnectionFooter.svelte
- web/src/lib/ui/store-connections/StoreServerSwitcher.svelte
- web/src/lib/ui/store-connections/navigation.svelte.ts
- web/src/lib/ui/store-connections/presentation.ts
- web/src/lib/ui/store-settings/StoreSettingsNavigation.svelte
- web/src/lib/ui/store-threads/StoreConversationMessageActions.svelte
- web/src/lib/ui/store-work/StoreProjectsRail.svelte
- web/tests/server-connections.test.ts
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Implement device-owned remote server connections and operator-enabled authenticated agents in the isolated remote-agents Jujutsu workspace. Parent qwxwqrxp (8210b56e) has installation profiles/avatars but no connection switcher, gateway or public authenticated execution. This slice spans the owned service/RPC, HTTP/server and production application boundaries; targeted checks cover their direct consumers. The original workspace, published dependency resolution and existing development services/data remain unchanged.
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-server/src/connections/tests.rs#origins_require_tls_and_reject_credentials_and_paths
- crates/jj-store-server/src/connections/tests.rs#registry_restart_retains_hints_without_credentials
- crates/jj-store-server/src/connections/tests.rs#gateway_binds_sessions_handles_binary_and_refuses_identity_replacement
- crates/jj-store-server/src/connections/transport.rs#gateway_paths_are_fixed_public_apis
- crates/jj-store-server/src/remote_agents/tests.rs#remote_admission_filters_definitions_and_denies_ungranted_home_and_revoked_identity
- crates/jj-store-server/src/remote_agents/execution_tests.rs#native_remote_execution_retains_receipts_and_never_replays_after_restart
- crates/jj-store-http/src/remote_agents/tests.rs#authenticated_execution_requires_caller_admission_and_rechecks_stream_continuation
- crates/jj-store-http/src/contract/tests.rs#contract_inventory_covers_security_methods_media_and_task_bodies
- crates/jj-store-service/tests/authorization.rs#delegated_access_rechecks_identity_scope_revocation_and_repository_grants
- crates/jj-store-rpc/tests/delegated.rs#private_delegated_rpc_checks_uid_scope_no_bearer_admin_and_revocation
- crates/jj-store-server/tests/remote_agent_configuration.rs#documented_remote_agents_require_explicit_authenticated_allowlists
- web/tests/server-connections.test.ts#old connection failures cannot invalidate
  a replacement sign-in
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64; Jujutsu 0.45.1; Rust/Cargo 1.98.1; Node 26.9.0; pnpm manifest 10.32.1; released Design Core 0.3.1 and Mira 0.0.3; Vitest 4.1.11; Storybook 10.5.7; Linux source fixtures used rust:1.98.1-bookworm.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Server strict format-2 TOML adds optional remote_agents; no format migration. Private saved connection hints and execution scopes use versioned JSON outside checkouts. Gateway sessions/tokens stay memory-only. Client-only server query and bounded public device preferences preserve exact per-connection destinations. Native objects, WAL, manifest CAS, AgentStore and WorkStore authority formats are unchanged. Public execution SSE uses integer after sequences, distinct from repository event cursors.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-REMOTE-AGENTS-001 details 2](details-02.md)
- [E-REMOTE-AGENTS-001 details 3](details-03.md)
- [E-REMOTE-AGENTS-001 details 4](details-04.md)
- [E-REMOTE-AGENTS-001 details 5](details-05.md)
