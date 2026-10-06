---
schema: 1
id: E-M4-014
---

# E-M4-014

<!-- evidence-field:id markdown -->
## Id

E-M4-014
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M4
- M4.1
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-27
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

authenticated-http-reads-and-shared-tls-listener
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- Cargo.lock
- Cargo.toml
- crates/jj-store-cli/Cargo.toml
- crates/jj-store-cli/tests/native_lifecycle/content.rs
- crates/jj-store-cli/tests/native_lifecycle/http_reads.rs
- crates/jj-store-cli/tests/server_executable.rs
- crates/jj-store-http/Cargo.toml
- crates/jj-store-http/src/body.rs
- crates/jj-store-http/src/error.rs
- crates/jj-store-http/src/lib.rs
- crates/jj-store-http/src/routes.rs
- crates/jj-store-jj/src/query/content.rs
- crates/jj-store-server/Cargo.toml
- crates/jj-store-server/src/lib.rs
- crates/jj-store-server/src/listener.rs
- crates/jj-store-server/src/process.rs
- crates/jj-store-service/src/reads.rs
- crates/jj-store-service/src/transport.rs
- docs/evidence/m4-http-read-gate.md
- docs/evidence/m4-http-rustfs.json
- scripts/check-boundaries.mjs
- scripts/governance.test.mjs
- spec/src/api.md
- spec/src/architecture.md
- spec/src/cli.md
- spec/src/compatibility.md
- spec/src/jj-integration.md
- spec/src/milestones.md
- spec/src/mutations.md
- spec/src/security.md
- spec/src/spec-governance.md
- spec/src/verification.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/native_lifecycle/http_reads.rs#http_reads_match_native_snapshots_objects_graphs_and_differences_without_publication
- crates/jj-store-cli/tests/native_lifecycle/http_reads.rs#http_reads_authentication_queries_and_binary_ranges_fail_closed
- crates/jj-store-cli/tests/native_lifecycle/http_reads.rs#http_reads_revocation_stops_inflight_media_and_credentials_never_become_query_data
- crates/jj-store-cli/tests/native_lifecycle/http_reads.rs#http_reads_unpolled_response_expiry_releases_transport_and_native_slots
- crates/jj-store-cli/tests/native_lifecycle/http_reads.rs#http_reads_index_task_and_signed_cursor_preserve_snapshot_across_publication
- crates/jj-store-cli/tests/native_lifecycle/http_reads.rs#http_reads_range_selection_cannot_reset_consumed_or_preselected_files
- crates/jj-store-cli/tests/native_lifecycle/http_reads.rs#http_reads_reject_unframed_request_content
- crates/jj-store-server/src/listener.rs#listener_idle_deadline_releases_owned_connection_slot
- crates/jj-store-server/src/listener.rs#listener_connection_capacity_is_retained_until_drop
- crates/jj-store-cli/tests/server_executable.rs#cold_tls_http_reads_support_http1_and_http2
- crates/jj-store-cli/tests/server_executable.rs#real_rustfs_server_cli_restart_and_revocation
- scripts/governance.test.mjs#http_consumes_owned_service_contracts_without_storage_or_upstream
- scripts/governance.test.mjs#server_composition_cannot_depend_on_cli_or_native_upstream
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M4-014 details 2](details-02.md)
- [E-M4-014 details 3](details-03.md)
