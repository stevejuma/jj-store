---
schema: 1
id: E-M6-008
---

# E-M6-008

<!-- evidence-field:id markdown -->
## Id

E-M6-008
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M6
- M6.2
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-27
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

server-search-lifecycle
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- Cargo.lock
- crates/jj-store-adapters/src/runner.rs
- crates/jj-store-adapters/tests/delivery.rs
- crates/jj-store-cli/src/server_commands.rs
- crates/jj-store-cli/tests/native_lifecycle/adapter_projection.rs
- crates/jj-store-cli/tests/native_lifecycle/server_search.rs
- crates/jj-store-cli/tests/server_executable.rs
- crates/jj-store-server/Cargo.toml
- crates/jj-store-server/src/config.rs
- crates/jj-store-server/src/lib.rs
- crates/jj-store-server/src/process.rs
- crates/jj-store-server/src/search.rs
- crates/jj-store-server/tests/configuration.rs
- crates/jj-store-server/tests/search_diagnostics.rs
- crates/jj-store-tantivy/src/native.rs
- crates/jj-store-tantivy/src/paging.rs
- docs/cli/server-config.md
- docs/cli/server-doctor.md
- docs/evidence/m6-server-search.md
- scripts/check-boundaries.mjs
- scripts/governance.test.mjs
- spec/src/adapters.md
- spec/src/architecture.md
- spec/src/cli.md
- spec/src/compatibility.md
- spec/src/milestones.md
- spec/src/security.md
- spec/src/spec-governance.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/native_lifecycle/server_search.rs#server_search_runs_recovers_and_restores_after_cache_loss
- crates/jj-store-cli/tests/native_lifecycle/server_search.rs#server_search_revoked_worker_degrades_without_revoking_other_readers
- crates/jj-store-cli/tests/native_lifecycle/server_search.rs#server_search_diagnostics_use_authorized_live_status_and_handle_unavailability
- crates/jj-store-cli/tests/native_lifecycle/server_search.rs#server_search_drop_cancels_workers_and_releases_owned_state
- crates/jj-store-cli/tests/native_lifecycle/server_search.rs#server_search_rejects_state_symlink_escape_and_invalid_programmatic_limits
- crates/jj-store-server/tests/search_diagnostics.rs#search_diagnostics_bound_streams_and_reject_foreign_or_invalid_status
- crates/jj-store-server/tests/search_diagnostics.rs#search_diagnostics_do_not_forward_credentials_through_redirects
- crates/jj-store-cli/tests/server_executable.rs#cold_search_server_requires_injected_credentials_without_echoing_secrets
- crates/jj-store-cli/tests/server_executable.rs#cold_search_worker_polls_failed_storage_and_stops_with_foreground_server
- crates/jj-store-server/tests/configuration.rs#search_configuration_is_explicit_bounded_and_contains_no_credentials
- crates/jj-store-adapters/tests/delivery.rs#adapter_shutdown_waits_for_retained_worker_contexts
- scripts/governance.test.mjs#server_composition_cannot_depend_on_cli_or_native_upstream
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M6-008 details 2](details-02.md)
- [E-M6-008 details 3](details-03.md)
