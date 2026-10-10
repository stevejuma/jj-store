---
schema: 1
id: E-CHATGPT-001
---

# E-CHATGPT-001

<!-- evidence-field:id markdown -->
## Id

E-CHATGPT-001
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
- JST-SEC-002
- JST-API-001
- JST-UI-012
- JST-DOC-001
- JST-DOC-002
- JST-ARCH-001
- JST-MUT-001
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-10
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Direct ChatGPT subscription provider source and controlled acceptance
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

Enclosing Jujutsu change szzqmzpx in chatgpt-plan, based on 2cd2903b.
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

The original workspace was clean. Created the requested isolated JJ workspace at /Users/stevejuma/code/lapismd/jj-store-chatgpt-plan from the original parent. The named chatgpt_provider regression failed because the provider was missing.
<!-- /evidence-field -->

<!-- evidence-field:findings markdown -->
## Findings

Implemented dynamic registration, signed identity validation, stable host/issued-client identities, Keychain bundles, serialized rotation, durable Connect receipts, bounded callbacks, account-bound worker leases, local sign-out and revocation reporting. Account/model inheritance is provider/account scoped. Public streaming Responses transport has no Codex CLI dependency, fixes its endpoint, replays opaque output in memory, permits one pre-stream 401 renewal and refuses uncertain/late/incomplete streams. Settings owns explicit sign-in and account selection; metadata remains independent. Test-only OAuth/vault/browser/HTTP substitutes cannot be enabled through production configuration. Final sign-out review reproduced and fixed disconnected-account reauthorization bypass: cancellation and generation fencing now apply regardless of cached connection status.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Affected service/server/HTTP, capture privacy, bundled runtime and direct UI consumers. Full bundled runtime suite is justified by shared configuration, transport, history and summary entrypoints. Full catalog unit suite is justified by shared settings freshness and teardown. Cold Storybook interaction/axe covers Accounts, Defaults, Session Settings, Provider Configuration and Conversation Metadata. Root aggregate pnpm check/test/build/spec:check, unrelated Rust/storage/RustFS suites, main fixture restart/reset, installed CLI replacement, packages and releases are out of scope. The full public contract lane is required for the additive provider/account schemas and validates generated artifacts against 300 captured HTTP/SSE responses. Original validation completed; a redundant second run was stopped with all of its owned child processes after the full pass, without touching the original application.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- Cargo.lock
- catalog/components/store-application/AgentChatGpt.stories.svelte
- catalog/components/store-application/AgentDefaults.stories.svelte
- catalog/fixtures/AgentChatGptFixture.svelte
- catalog/fixtures/agent-settings-fixture.svelte.ts
- crates/jj-store-capture/src/transcript_sanitization.rs
- crates/jj-store-capture/tests/transcript_sanitization.rs
- crates/jj-store-http/src/agent_execution/tests.rs
- crates/jj-store-http/src/agent_execution.rs
- crates/jj-store-server/Cargo.toml
- crates/jj-store-server/src/agent_execution/catalog.rs
- crates/jj-store-server/src/agent_execution/chatgpt/accounts.rs
- crates/jj-store-server/src/agent_execution/chatgpt/attempts.rs
- crates/jj-store-server/src/agent_execution/chatgpt/authorization.rs
- crates/jj-store-server/src/agent_execution/chatgpt/broker.rs
- crates/jj-store-server/src/agent_execution/chatgpt/callback.rs
- crates/jj-store-server/src/agent_execution/chatgpt/fixtures/synthetic-jwks.json
- crates/jj-store-server/src/agent_execution/chatgpt/fixtures/synthetic-rsa.fixture
- crates/jj-store-server/src/agent_execution/chatgpt/io.rs
- crates/jj-store-server/src/agent_execution/chatgpt/mod.rs
- crates/jj-store-server/src/agent_execution/chatgpt/oauth.rs
- crates/jj-store-server/src/agent_execution/chatgpt/tests/harness.rs
- crates/jj-store-server/src/agent_execution/chatgpt/tests.rs
- crates/jj-store-server/src/agent_execution/chatgpt/vault.rs
- crates/jj-store-server/src/agent_execution/commands.rs
- crates/jj-store-server/src/agent_execution/configuration/tests.rs
- crates/jj-store-server/src/agent_execution/configuration.rs
- crates/jj-store-server/src/agent_execution/fleet.rs
- crates/jj-store-server/src/agent_execution/freshness.rs
- crates/jj-store-server/src/agent_execution/journal.rs
- crates/jj-store-server/src/agent_execution/metadata_runtime.rs
- crates/jj-store-server/src/agent_execution/mod.rs
- crates/jj-store-server/src/agent_execution/provider_catalog.rs
- crates/jj-store-server/src/agent_execution/provider_launch.rs
- crates/jj-store-server/src/agent_execution/runtime.rs
- crates/jj-store-service/src/agent_execution/chatgpt.rs
- crates/jj-store-service/src/agent_execution/providers.rs
- crates/jj-store-service/src/agent_execution/settings.rs
- crates/jj-store-service/src/agent_execution.rs
- crates/jj-store-service/tests/chatgpt_provider.rs
- docs/agent-execution.md
- docs/maintenance/coverage.json
- docs/site/guides/agent-execution.md
- docs/site/guides/settings.md
- docs/site/reference/api/installation.md
- docs/site/reference/api/types.md
- docs/site/reference/configuration.md
- docs/site/reference/security.md
- docs/site/reference/transport.md
- docs/site/technical/architecture.md
- docs/site/technical/capture.md
- runtime/local-agent/Cargo.lock
- runtime/local-agent/Cargo.toml
- runtime/local-agent/UPSTREAM.md
- runtime/local-agent/src/chatgpt_token.rs
- runtime/local-agent/src/main.rs
- runtime/local-agent/src/metadata.rs
- runtime/local-agent/src/provider_catalog.rs
- runtime/local-agent/vendor/buzz-agent/src/config.rs
- runtime/local-agent/vendor/buzz-agent/src/lib.rs
- runtime/local-agent/vendor/buzz-agent/src/llm/chatgpt_plan/tests.rs
- runtime/local-agent/vendor/buzz-agent/src/llm/chatgpt_plan.rs
- runtime/local-agent/vendor/buzz-agent/src/llm.rs
- schemas/openapi.json
- spec/src/current/contracts/api/41-interactive-agent-execution.md
- spec/src/current/contracts/architecture/20-interactive-agent-execution.md
- spec/src/current/contracts/cli/36-installation-agent-configuration.md
- spec/src/current/contracts/collaboration/40-local-compute-and-agent-configuration.md
- spec/src/current/contracts/collaboration/41-chatgpt-plan-provider.md
- spec/src/current/contracts/collaboration/SUMMARY.nav.md
- spec/src/current/contracts/collaboration/index.md
- spec/src/current/contracts/compatibility/14-local-inference-runtime.md
- spec/src/current/contracts/documentation/08-interactive-agent-execution.md
- spec/src/current/contracts/mutations/23-installation-agent-configuration.md
- spec/src/current/contracts/security/02-requirements.md
- spec/src/current/contracts/ui/29-editable-agents-settings.md
- spec/src/current/progress/M25/history/M25/index.md
- spec/src/current/progress/M25/index.md
- spec/src/current/verification/api.md
- spec/src/current/verification/collab-03.md
- spec/src/current/verification/sec.md
- web/src/lib/api/generated.ts
- web/src/lib/controller/agent-chatgpt.svelte.ts
- web/src/lib/controller/agent-configuration.ts
- web/src/lib/controller/agent-execution-refresh.ts
- web/src/lib/controller/agent-execution.svelte.ts
- web/src/lib/controller/agent-resource-disposal.ts
- web/src/lib/ui/store-agent-execution/StoreAgentChatGptAccount.svelte
- web/src/lib/ui/store-agent-execution/StoreAgentConfiguration.svelte
- web/src/lib/ui/store-agent-execution/StoreAgentExecutionMode.svelte
- web/src/lib/ui/store-agent-execution/StoreAgentSetupEditor.svelte
- web/src/lib/ui/store-agent-execution/agent-configuration-status.ts
- web/src/lib/ui/store-settings/StoreAgentChatGptAccounts.svelte
- web/src/lib/ui/store-settings/StoreAgentDefaults.svelte
- web/src/lib/ui/store-settings/StoreConversationMetadataSettings.svelte
- web/tests/vite/agent-chat.test.ts
- web/tests/vite/agent-chatgpt.test.ts
- web/tests/vite/agent-freshness.test.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-service/tests/chatgpt_provider.rs#chatgpt_plan_has_distinct_oauth_identity_and_legacy_options_load
- crates/jj-store-server/src/agent_execution/chatgpt/tests.rs#dynamic_registration_returns_issued_identity_and_reauthorization_is_bound
- crates/jj-store-server/src/agent_execution/chatgpt/tests.rs#signed_identity_requires_issuer_client_expiry_nonce_and_valid_signature
- crates/jj-store-server/src/agent_execution/chatgpt/tests.rs#cold_authorization_receipts_serialized_rotation_and_account_bound_broker
- crates/jj-store-http/src/agent_execution/tests.rs#chatgpt_account_routes_require_local_admin_and_reject_credential_input
- crates/jj-store-capture/tests/transcript_sanitization.rs#oauth_jwts_are_redacted_at_their_own_bound_without_expanding_ordinary_secrets
- runtime/local-agent/vendor/buzz-agent/src/llm/chatgpt_plan/tests.rs#pre_stream_401_renews_once_tool_rounds_replay_and_uncertain_calls_never_retry
- runtime/local-agent/vendor/buzz-agent/src/llm/chatgpt_plan/tests.rs#late_failure_and_incomplete_never_release_tool_calls
- web/tests/vite/agent-chatgpt.test.ts#lost Connect replies retain the exact
  request identity and never send credentials
- web/tests/vite/agent-chatgpt.test.ts#proven command refusal releases the
  draft; polling recovers after an outage
- web/tests/vite/agent-chatgpt.test.ts#malformed account responses retain the
  last valid view without an unhandled poll
- crates/jj-store-server/src/agent_execution/chatgpt/tests.rs#sign_out_cancels_reauthorization_of_disconnected_accounts
<!-- /evidence-field -->

