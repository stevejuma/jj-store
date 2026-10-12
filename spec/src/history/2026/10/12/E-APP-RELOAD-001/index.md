---
schema: 1
id: E-APP-RELOAD-001
---

# E-APP-RELOAD-001

<!-- evidence-field:id markdown -->
## Id

E-APP-RELOAD-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M27
- M24
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-API-001
- JST-SEC-009
- JST-MUT-001
- JST-UI-016
- JST-LOCAL-003
- JST-LOCAL-004
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-12
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

cross-layer-verification
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

lknwuvly
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- Cargo.lock
- catalog/components/store-application/RuntimeReload.stories.svelte
- catalog/components/store-application/StoreCaptureSettings.stories.svelte
- catalog/components/store-application/StoreNumericSetting.stories.svelte
- catalog/components/store-application/StoreServerSettings.stories.svelte
- catalog/components/store-application/capture-configuration-play.ts
- catalog/fixtures/NumericSettingFixture.svelte
- catalog/fixtures/RuntimeReloadFixture.svelte
- crates/jj-store-cli/src/server_commands.rs
- crates/jj-store-cli/src/token_commands.rs
- crates/jj-store-cli/tests/native_lifecycle/http_reads.rs
- crates/jj-store-http/Cargo.toml
- crates/jj-store-http/src/application_events/code_indexes.rs
- crates/jj-store-http/src/application_events/monitors.rs
- crates/jj-store-http/src/application_events/producers.rs
- crates/jj-store-http/src/contract/tests.rs
- crates/jj-store-http/src/contract.rs
- crates/jj-store-http/src/lib.rs
- crates/jj-store-http/src/runtime_reload.rs
- crates/jj-store-http/src/sync_management.rs
- crates/jj-store-http/src/uploads.rs
- crates/jj-store-jj/src/local_browser/code_index.rs
- crates/jj-store-jj/src/local_browser/mod.rs
- crates/jj-store-jj/src/query/content.rs
- crates/jj-store-jj/src/query/mod.rs
- crates/jj-store-jj/src/query/workers.rs
- crates/jj-store-object/src/s3.rs
- crates/jj-store-rpc/src/server.rs
- crates/jj-store-server/src/agent_administration.rs
- crates/jj-store-server/src/agent_execution/compute/mod.rs
- crates/jj-store-server/src/agent_execution/compute/process.rs
- crates/jj-store-server/src/agent_execution/configuration.rs
- crates/jj-store-server/src/capture_management.rs
- crates/jj-store-server/src/config.rs
- crates/jj-store-server/src/connections/transport.rs
- crates/jj-store-server/src/lib.rs
- crates/jj-store-server/src/process/authority.rs
- crates/jj-store-server/src/process/capture.rs
- crates/jj-store-server/src/process/preparation.rs
- crates/jj-store-server/src/process/reload.rs
- crates/jj-store-server/src/process/reload_capture.rs
- crates/jj-store-server/src/process/running.rs
- crates/jj-store-server/src/process.rs
- crates/jj-store-server/src/remote_agents/caller.rs
- crates/jj-store-server/src/remote_agents/lease.rs
- crates/jj-store-server/src/remote_agents/mod.rs
- crates/jj-store-server/src/replication/freshness.rs
- crates/jj-store-server/src/replication.rs
- crates/jj-store-server/src/runtime_reload/journal.rs
- crates/jj-store-server/src/runtime_reload/mod.rs
- crates/jj-store-server/src/runtime_reload/tests.rs
- crates/jj-store-server/src/search_management/file.rs
- crates/jj-store-server/src/search_management/mod.rs
- crates/jj-store-server/src/server_settings.rs
- crates/jj-store-service/src/administration.rs
- crates/jj-store-service/src/auth.rs
- crates/jj-store-service/src/guard.rs
- crates/jj-store-service/src/lib.rs
- crates/jj-store-service/src/native.rs
- crates/jj-store-service/src/registry.rs
- crates/jj-store-service/src/runtime_policy.rs
- crates/jj-store-service/src/runtime_reload.rs
- crates/jj-store-service/src/transport.rs
- crates/jj-store-service/src/uploads.rs
- crates/jj-store-service/tests/authorization.rs
- crates/jj-store-wal/src/publication/collaboration.rs
- crates/jj-store-wal/src/publication/domains.rs
- crates/jj-store-wal/src/publication/replication_import.rs
- crates/jj-store-wal/src/publication/replication_seed.rs
- crates/jj-store-wal/src/publication.rs
- crates/jj-store-wal/tests/publication.rs
- docs/cli/server-config.md
- docs/cli/server-operator.md
- docs/cli/sync.md
- docs/maintenance/coverage.json
- docs/site/cli/jj-store.md
- docs/site/cli/jj-sync.md
- docs/site/guides/settings.md
- docs/site/reference/api/installation.md
- docs/site/reference/api/types.md
- docs/site/reference/configuration.md
- docs/site/reference/transport.md
- docs/site/technical/lifecycle.md
- docs/site/technical/publication.md
- docs/ui-provenance.json
- schemas/openapi.json
- scripts/remote-agent-administration-acceptance.py
- scripts/remote-agent-administration-browser.mjs
- scripts/runtime-reload-acceptance.py
- scripts/runtime-reload-browser.mjs
- spec/src/current/contracts/api/30-installation-configuration-editing.md
- spec/src/current/contracts/api/31-local-capture-configuration-editing.md
- spec/src/current/contracts/architecture/21-local-compute-resources.md
- spec/src/current/contracts/cli/23-app-managed-operational-configuration.md
- spec/src/current/contracts/compatibility/11-installation-operational-settings-contracts.md
- spec/src/current/contracts/documentation/09-remote-agent-connections.md
- spec/src/current/contracts/jj-integration/17-bounded-browser-reads.md
- spec/src/current/contracts/local-jj/12-bounded-local-code-reads.md
- spec/src/current/contracts/mutations/24-remote-agent-authority.md
- spec/src/current/contracts/object-store/04-streamed-transport-contract.md
- spec/src/current/contracts/publication/05-typed-application-intent.md
- spec/src/current/contracts/security/12-remote-connections-and-agents.md
- spec/src/current/contracts/ui/28-complete-supported-configuration-settings.md
- spec/src/current/governance/spec-governance.md
- spec/src/current/progress/M24/index.md
- spec/src/current/progress/M27/SUMMARY.nav.md
- spec/src/current/progress/M27/index.md
- spec/src/current/progress/M27/runtime-reload-review.md
- spec/src/current/verification/api.md
- spec/src/current/verification/mut.md
- spec/src/current/verification/sec.md
- spec/src/current/verification/ui-02.md
- web/src/lib/api/generated.ts
- web/src/lib/api/server-administration-client.ts
- web/src/lib/connections/management.ts
- web/src/lib/controller/capture-settings-management.ts
- web/src/lib/controller/contracts.ts
- web/src/lib/controller/production-local-configuration.ts
- web/src/lib/controller/production-store-view.ts
- web/src/lib/controller/runtime-reload-validation.ts
- web/src/lib/controller/runtime-reload.svelte.ts
- web/src/lib/controller/server-agent-administration.svelte.ts
- web/src/lib/controller/server-settings.svelte.ts
- web/src/lib/ui/AGENTS.md
- web/src/lib/ui/community-app-shell/CommunityAppShell.svelte
- web/src/lib/ui/store-agent-execution/StoreServerAccessOnboarding.svelte
- web/src/lib/ui/store-agent-execution/StoreServerAgentConfiguration.svelte
- web/src/lib/ui/store-agent-execution/StoreServerAgentDefinition.svelte
- web/src/lib/ui/store-application/StoreApplication.svelte
- web/src/lib/ui/store-connections/StoreConnectionFooter.svelte
- web/src/lib/ui/store-settings/StoreCaptureConfiguration.svelte
- web/src/lib/ui/store-settings/StoreCaptureSettings.svelte
- web/src/lib/ui/store-settings/StoreConfigurationEditor.svelte
- web/src/lib/ui/store-settings/StoreNumericSettingRow.svelte
- web/src/lib/ui/store-settings/StoreRuntimeReload.svelte
- web/src/lib/ui/store-settings/StoreSearchConfiguration.svelte
- web/src/lib/ui/store-settings/StoreSettingsNavigation.css
- web/src/lib/ui/store-settings/StoreSettingsNavigation.svelte
- web/src/lib/ui/store-settings/StoreSettingsSkeleton.svelte
- web/src/lib/ui/store-settings/StoreSyncConfiguration.svelte
- web/src/lib/ui/store-settings/global-settings-skeleton.ts
- web/src/lib/ui/store-settings/numeric-setting.css
- web/src/lib/ui/store-settings/settings-skeleton.css
- web/src/lib/ui/store-settings/settings-skeleton.ts
- web/tests/vite/runtime-reload.test.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-server/src/runtime_reload/tests.rs#saved_reload_retains_exact_intent_and_effective_values_without_replay
- crates/jj-store-server/src/runtime_reload/tests.rs#independent_operator_roles_are_fresh_and_agent_management_cannot_reload_service
- crates/jj-store-wal/tests/publication.rs#configuration_replacement_waits_for_the_original_manifest_cas
- crates/jj-store-jj/src/query/workers.rs#retained_query_clones_capture_updated_limits_for_new_jobs
- crates/jj-store-service/tests/authorization.rs#configuration_management_is_an_independent_operator_capability
- web/tests/vite/runtime-reload.test.ts#lost reload acknowledgement checks original intent and never reposts
- web/tests/vite/runtime-reload.test.ts#stale owners cannot replace the selected connection reload state
- scripts/runtime-reload-acceptance.py#qualify
- scripts/remote-agent-administration-acceptance.py#qualify
<!-- /evidence-field -->

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown
```

```shell
JJ_STORE_AGENT_PRIVACY_CACHE="$(cat .local/app-reload-test-cache-path)" JJ_STORE_TRANSCRIPT_CACHE="$(cat .local/app-reload-test-cache-path)" pnpm contracts:check
```

**Notes**

Generated contracts match and 300 captured HTTP/SSE responses conform; 8 contract
tests and contract type checking pass. Linked validation pages record the actual
Rust, ownership/history, interaction/Axe, build/document, cold real-storage and
installed-Codex commands. Final normalized-label cold and installed-provider
runs pass against the final executables and bundle, with disposable cleanup.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pass
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

[]
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Node 26.9.0; pnpm 10.32.1; Rust 1.98.1; JJ 0.45.1; Svelte 5.57.1; Vitest 4.1.11. Released Design Core ^0.3.1 and Mira ~0.0.3 manifests and lockfile remain unchanged. macOS ARM64 qualification only.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Additive V1 configuration reload status/command/outcome API and private receipt journal. Existing TOML, native object/WAL/manifest, repository/session identities and configuration formats remain unchanged. The legacy restart_required field continues to represent saved/loaded mismatch. ManageConfiguration is independent from repository grants and ManageAgents.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Explicit saved-runtime activation with stable principal-bound receipts, per-domain outcomes, fresh operator authority, publication fencing and retained process/storage/listener owners. Current bounds apply to new jobs; existing pins and budgets are retained. Capture/search/sync replacements retain durable state. Agent launch and Compute changes may defer to worker/helper start. Top-header vertical kebab opens the shared reload dialog; Settings version follows Help. Capture sliders use bounded choices without snapping exact typed drafts. Unit selection changes both compact endpoint labels without editing canonical values or bounds; tiny minima may display as zero. Routes, drafts and pending commands remain retained.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Continue the separate M24/M27 platform, provider and deferred-feature gates. This verified slice is committed with Jujutsu in enclosing change lknwuvly; no publication, migration, reset or visual-baseline update.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: Add independent configuration activation and typed reload endpoints with current authorization, receipt recovery and publication fencing. Owning API, security, mutations, publication, service/configuration, UI and lifecycle chapters are updated; existing native formats, manifest authority, historical pins and prompt identities remain unchanged.
```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M24 and M27 remain In progress. E-APP-RELOAD-001 reviews runtime activation and retained owner boundaries without completing independent provider, platform, backup, sandbox or visual gates. See current/progress/M27/runtime-reload-review.md.
```
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [Implementation and diagnostics](details-02.md)
- [Validation commands](details-03.md)
- [Runtime and browser qualification](details-04.md)
