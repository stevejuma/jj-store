---
schema: 1
id: E-READ-080
---

# E-READ-080

<!-- evidence-field:id markdown -->
## Id

E-READ-080
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M13.4
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-05
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

xnpzyzls
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Managed standard JJ core browsing through existing events
<!-- /evidence-field -->

<!-- evidence-field:versions list -->
## Versions

- JJ/jj-lib 0.45.1
- Rust 1.98.1
- Node 26.9.0
- pnpm 10.32.1
- Svelte 5.57.1
- Vitest 4.1.11
- macOS ARM64
<!-- /evidence-field -->

<!-- evidence-field:formats list -->
## Formats

- Standard JJ default index format 6
- Private derived reservation version 2 adds profile plus epoch/generation;
  version 1 bytes unchanged
- Additive automatic_code_index capability and code-index/tasks routes; existing
  advanced task contracts unchanged
- No authoritative object, manifest, WAL, receipt or identity change
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Approved stages two and three: one persistent bounded native/local owner, first-open/restart activation through existing monitors, automatic exact-operation browser recovery. Shared Rust/HTTP/UI admission and lifecycle justify complete local/native consumer binaries, actual registered contract captures and focused UI/component checks. Advanced and complete release qualification stay deferred.
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-JJ-005
- JST-LOCAL-004
- JST-API-001
- JST-UI-005
- JST-GOV-002
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-application/StoreKindPreparation.stories.svelte
- catalog/fixtures/KindPreparationFixture.svelte
- crates/jj-store-cli/tests/local_browser.rs
- crates/jj-store-cli/tests/local_browser/managed_http.rs
- crates/jj-store-cli/tests/local_browser/managed_lifecycle.rs
- crates/jj-store-cli/tests/native_lifecycle/http_reads.rs
- crates/jj-store-cli/tests/native_lifecycle/managed_core_http.rs
- crates/jj-store-http/src/application_events.rs
- crates/jj-store-http/src/application_events/code_indexes.rs
- crates/jj-store-http/src/application_events/monitors.rs
- crates/jj-store-http/src/application_events/producers.rs
- crates/jj-store-http/src/dto.rs
- crates/jj-store-http/src/error.rs
- crates/jj-store-http/src/lib.rs
- crates/jj-store-http/src/local_browser.rs
- crates/jj-store-http/src/local_browser/events.rs
- crates/jj-store-http/src/local_browser/index_tasks.rs
- crates/jj-store-http/src/routes.rs
- crates/jj-store-jj/src/local_browser/code_index.rs
- crates/jj-store-jj/src/local_browser/mod.rs
- crates/jj-store-jj/src/query/cache.rs
- crates/jj-store-jj/src/query/cache/managed.rs
- crates/jj-store-jj/src/query/cache/reservation.rs
- crates/jj-store-jj/src/query/code_index.rs
- crates/jj-store-jj/src/query/code_index/native.rs
- crates/jj-store-jj/src/query/workers.rs
- crates/jj-store-service/src/code_indexes.rs
- crates/jj-store-service/src/code_indexes/jobs.rs
- crates/jj-store-service/src/code_indexes/tasks.rs
- crates/jj-store-service/src/lib.rs
- crates/jj-store-service/src/pagination.rs
- crates/jj-store-service/src/reads.rs
- docs/implementation-plan.md
- schemas/openapi.json
- scripts/contracts.mjs
- spec/src/api.md
- spec/src/architecture.md
- spec/src/cli.md
- spec/src/compatibility.md
- spec/src/jj-integration.md
- spec/src/local-jj.md
- spec/src/milestones.md
- spec/src/mutations.md
- spec/src/spec-governance.md
- spec/src/ui.md
- web/src/lib/api/generated.ts
- web/src/lib/controller/http-local-index-data-source.ts
- web/src/lib/controller/http-store-data-source.ts
- web/src/lib/controller/http-store-write-data-source.ts
- web/src/lib/controller/managed-index-recovery.svelte.ts
- web/src/lib/controller/native-data-source.ts
- web/src/lib/controller/production-index-preparation.ts
- web/src/lib/controller/production-read-services.ts
- web/src/lib/controller/production-store-index.ts
- web/src/lib/ui/store-settings/StoreGraphIndexSettings.svelte
- web/tests/vite/managed-code-index.test.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-jj/src/query/cache/managed.rs#persistent_namespaces_share_quota_resume_and_reuse_one_repository_graph
- crates/jj-store-jj/src/query/workers.rs#background_observation_preserves_foreground_admission
- crates/jj-store-cli/tests/local_browser/managed_lifecycle.rs#managed_owner_coalesces_resumes_and_keeps_old_selection_during_updates
- crates/jj-store-cli/tests/local_browser/managed_lifecycle.rs#managed_background_updates_coalesce_but_preserve_foreground_requests
- crates/jj-store-cli/tests/local_browser/managed_lifecycle.rs#managed_cancel_and_server_teardown_release_pending_authorization
- crates/jj-store-cli/tests/local_browser/managed_lifecycle.rs#managed_reads_and_controls_reject_revoked_or_changed_registration
- crates/jj-store-cli/tests/local_browser/managed_http.rs#managed_local_http_activates_without_subscribers_resumes_and_captures_tasks
- crates/jj-store-cli/tests/native_lifecycle/managed_core_http.rs#managed_native_http_preserves_light_reads_restarts_and_captures_tasks
- crates/jj-store-cli/tests/local_browser/prepared_points_http.rs#local_point_http_uses_ready_membership_and_only_falls_back_when_cold
- crates/jj-store-cli/tests/local_browser.rs#filesystem_runtime_registers_and_reads_without_native_storage
- web/tests/vite/managed-code-index.test.ts#managed history recovery preserves
  the exact selection and does not prepare advanced inventories
- web/tests/vite/managed-code-index.test.ts#failed managed recovery requires a
  new episode or explicit retry
- web/tests/vite/managed-code-index.test.ts#automatic maintenance defaults off
  and advanced operation views stay explicit
- web/tests/vite/managed-code-index.test.ts#navigation aborts obsolete readiness
  without restoring another view
- web/tests/vite/managed-code-index.test.ts#explicit advanced preparation
  retains its existing task route
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-READ-080 details 2](details-02.md)
- [E-READ-080 details 3](details-03.md)
- [E-READ-080 details 4](details-04.md)
