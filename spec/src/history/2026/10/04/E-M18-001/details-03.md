---
schema: 1
id: E-M18-001
---

# E-M18-001 details 3

<!-- evidence-field:files list -->
## Files

- crates/jj-store-service/src/local_repository/agent_descriptor.rs
- crates/jj-store-service/src/local_repository/agent_store.rs
- crates/jj-store-service/src/local_repository/agent_thread_binding.rs
- crates/jj-store-service/src/local_repository/collab_commands.rs
- crates/jj-store-service/tests/agent_sessions.rs
- crates/jj-store-service/tests/local_agent/native_results.rs
- crates/jj-store-service/tests/local_agent/opencode_aliases.rs
- crates/jj-store-service/tests/local_agent.rs
- crates/jj-store-service/tests/local_agent_binding.rs
- crates/jj-store-wal/src/publication/agent.rs
- crates/jj-store-wal/src/publication/agent_capture.rs
- crates/jj-store-wal/src/publication/agent_capture_aliases/catalog.rs
- crates/jj-store-wal/src/publication/agent_capture_aliases/catalog_tests.rs
- crates/jj-store-wal/src/publication/agent_capture_aliases/remap.rs
- crates/jj-store-wal/src/publication/agent_capture_aliases.rs
- crates/jj-store-wal/src/publication/agent_capture_binding.rs
- crates/jj-store-wal/src/publication/agent_capture_projection.rs
- crates/jj-store-wal/src/publication/agent_capture_records.rs
- crates/jj-store-wal/src/publication/agent_capture_results.rs
- crates/jj-store-wal/src/publication/agent_capture_tools.rs
- crates/jj-store-wal/src/publication/agent_capture_transcript.rs
- crates/jj-store-wal/src/publication/agent_catalog.rs
- crates/jj-store-wal/src/publication/agent_io.rs
- crates/jj-store-wal/src/publication/agent_read_engine.rs
- crates/jj-store-wal/src/publication/agent_thread_binding.rs
- crates/jj-store-wal/src/publication/agent_transcript_audit.rs
- crates/jj-store-wal/src/publication/agent_transcript_read.rs
- crates/jj-store-wal/src/publication/agent_transcript_tail.rs
- crates/jj-store-wal/src/publication/collab_commands.rs
- crates/jj-store-wal/src/publication.rs
- crates/jj-store-wal/tests/agent_catalog/common_aliases.rs
- crates/jj-store-wal/tests/agent_catalog/native_results.rs
- crates/jj-store-wal/tests/agent_catalog/opencode_aliases.rs
- crates/jj-store-wal/tests/agent_catalog/selected_tail.rs
- crates/jj-store-wal/tests/agent_catalog.rs
- crates/jj-store-wal/tests/bound_capture.rs
- crates/jj-store-wal/tests/native_copy.rs
- docs/capture-preview.md
- docs/implementation-plan.md
- package.json
- schemas/openapi.json
- scripts/check-boundaries.mjs
- scripts/check-progress.mjs
- scripts/contracts.mjs
- scripts/dev-capture.mjs
- scripts/governance.test.mjs
- scripts/progress-mapping.mjs
- scripts/progress-mapping.test.mjs
- spec/src/adapters.md
- spec/src/api.md
- spec/src/architecture.md
- spec/src/cli.md
- spec/src/collaboration.md
- spec/src/compatibility.md
- spec/src/jj-integration.md
- spec/src/local-jj.md
- spec/src/milestones.md
- spec/src/mutations.md
- spec/src/publication.md
- spec/src/security.md
- spec/src/spec-governance.md
- spec/src/ui.md
- spec/src/verification.md
- spec-validator.config.mjs
- web/src/lib/api/generated.ts
- web/src/lib/controller/capture-errors.ts
- web/src/lib/controller/capture-http.ts
- web/src/lib/controller/capture-management.svelte.ts
- web/src/lib/controller/capture-observations.ts
- web/src/lib/controller/capture-presentation.ts
- web/src/lib/controller/capture-refresh.ts
- web/src/lib/controller/capture-response-validation.ts
- web/src/lib/controller/capture-snapshot-validation.ts
- web/src/lib/controller/capture-types.ts
- web/src/lib/controller/capture-validation.ts
- web/src/lib/controller/contracts.ts
- web/src/lib/controller/production-capture-management.ts
- web/src/lib/controller/production-store-controller.svelte.ts
- web/src/lib/controller/production-store-lifecycle.svelte.ts
- web/src/lib/controller/production-store-state.svelte.ts
- web/src/lib/controller/production-store-view.ts
- web/src/lib/ui/store-settings/StoreCaptureDiagnostics.svelte
- web/src/lib/ui/store-settings/StoreCaptureImport.svelte
- web/src/lib/ui/store-settings/StoreCaptureSettings.svelte
- web/src/lib/ui/store-settings/StoreCaptureSources.svelte
- web/src/lib/ui/store-settings/StoreServiceSettings.svelte
- web/src/lib/ui/store-threads/thread-context.ts
- web/tests/browser/local-capture-settings.spec.ts
- web/tests/vite/capture-freshness.test.ts
- web/tests/vite/capture-management.test.ts
- web/tests/vite/production-capture-management.test.ts
- web/tests/vite/thread-context.test.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- scripts/progress-mapping.test.mjs#historical_capture_tasks_map_without_rewriting_evidence
- crates/jj-store-wal/tests/agent_catalog/selected_tail.rs#selected_cached_agent_reader_retains_audited_append_witnesses
- web/tests/vite/capture-freshness.test.ts#visible capture loads polls coalesces
  and stops on closure
- web/tests/vite/capture-freshness.test.ts#background observations retain exact
  uncertain control intent
- web/tests/vite/capture-freshness.test.ts#background refresh retains every
  loaded source page
- crates/jj-store-capture/tests/codex_transcript.rs#codex_cli_exec_and_inherited_desktop_originator_keep_cli_surface_checks
- crates/jj-store-server/tests/capture_management/imports.rs#import_acceptance_progress_and_replay_are_actual_http_contracts
- web/tests/vite/capture-management.test.ts#cumulative unavailable observations
  do not disable capture controls
<!-- /evidence-field -->

