---
schema: 1
id: E-M18-005
---

# E-M18-005

<!-- evidence-field:id markdown -->
## Id

E-M18-005
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M18.2
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-COLLAB-017
- JST-UI-012
- JST-API-006
- JST-GOV-003
- JST-GOV-009
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-04
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Editable installation-local capture configuration
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change qnyktnvp
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Main clean at qnyktnvp/54a71872, parent69d0db17. Agents settings expose resolved roots and pause/rescan/history commands but no capture configuration inputs. The disabled view requires manual TOML editing. Existing HMR restart regenerates ui-server.toml from the base fixture and loses app-saved settings.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

JJ0.45.1; Node24.10.0; pnpm10.32.1; Rust1.98.1; locked Svelte5, Storybook10.5.7, Vitest4.1.11 and Playwright1.63.0. Qualified Codex CLI0.156.1 and Desktop0.159.2 only. Inherited Design Core source-validation/release boundary unchanged.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Configuration format2 and capture journal/source/spool/import/native AgentStore formats unchanged. Additive actor-free version1 GET/POST /v1/local/capture/settings; SHA256 exact file revisions; saved/startup-effective/default-root reporting; shared settings-file lock and bounded synchronized atomic TOML replacement. Unrelated TOML values retained; formatting/comments normalized as in the existing search editor. Credentials and arbitrary TOML are excluded.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-application/StoreCaptureSettings.stories.svelte
- catalog/fixtures/CaptureSettingsFixture.svelte
- catalog/fixtures/capture-management.ts
- crates/jj-store-http/src/capture_management.rs
- crates/jj-store-http/src/capture_management/settings.rs
- crates/jj-store-http/src/contract/capture_tests.rs
- crates/jj-store-server/src/capture_management.rs
- crates/jj-store-server/src/capture_management/authority.rs
- crates/jj-store-server/src/capture_management/settings.rs
- crates/jj-store-server/src/capture_management/tests.rs
- crates/jj-store-server/tests/capture_management.rs
- crates/jj-store-server/tests/capture_management/http.rs
- crates/jj-store-server/tests/capture_management/settings.rs
- crates/jj-store-service/src/capture_management.rs
- crates/jj-store-service/src/capture_management/settings.rs
- schemas/openapi.json
- scripts/dev-ui.mjs
- scripts/dev-ui.test.mjs
- spec/src/api.md
- spec/src/cli.md
- spec/src/collaboration.md
- spec/src/compatibility.md
- spec/src/milestones.md
- spec/src/mutations.md
- spec/src/spec-governance.md
- spec/src/ui.md
- web/src/lib/api/generated.ts
- web/src/lib/controller/capture-configuration.ts
- web/src/lib/controller/capture-http.ts
- web/src/lib/controller/capture-management.svelte.ts
- web/src/lib/controller/capture-observations.ts
- web/src/lib/controller/capture-settings-management.ts
- web/src/lib/controller/capture-types.ts
- web/src/lib/ui/store-settings/StoreAgentSettings.svelte
- web/src/lib/ui/store-settings/StoreCaptureConfiguration.svelte
- web/src/lib/ui/store-settings/StoreCaptureSettings.svelte
- web/src/lib/ui/store-settings/StoreCaptureSources.svelte
- web/tests/browser/local-capture-settings.spec.ts
- web/tests/vite/capture-settings.test.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-server/tests/capture_management/settings.rs#configuration_saves_are_conditional_private_and_leave_capture_state_untouched
- crates/jj-store-server/tests/capture_management/settings.rs#configuration_revalidates_admin_identity_lock_and_regular_file
- crates/jj-store-server/src/capture_management/tests.rs#configuration_remains_editable_while_capture_inspection_holds_the_runtime_gate
- crates/jj-store-server/tests/capture_management/http.rs#local_capture_http_derives_actor_revalidates_authority_and_captures_contracts
- crates/jj-store-http/src/contract/capture_tests.rs#capture_management_contract_is_installation_local_and_actor_free
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M18-005 details 2](details-02.md)
- [E-M18-005 details 3](details-03.md)
