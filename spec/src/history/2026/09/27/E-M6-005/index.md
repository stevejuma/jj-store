---
schema: 1
id: E-M6-005
---

# E-M6-005

<!-- evidence-field:id markdown -->
## Id

E-M6-005
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

native-search-serving
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- Cargo.lock
- crates/jj-store-cli/tests/native_lifecycle/adapter_projection.rs
- crates/jj-store-cli/tests/native_lifecycle/search_paging.rs
- crates/jj-store-tantivy/Cargo.toml
- crates/jj-store-tantivy/src/document.rs
- crates/jj-store-tantivy/src/lib.rs
- crates/jj-store-tantivy/src/paging.rs
- crates/jj-store-tantivy/src/paging/cursors.rs
- crates/jj-store-tantivy/src/search.rs
- docs/evidence/m6-search-sessions.md
- spec/src/adapters.md
- spec/src/architecture.md
- spec/src/cli.md
- spec/src/compatibility.md
- spec/src/milestones.md
- spec/src/security.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/native_lifecycle/search_paging.rs#native_search_pages_pin_generation_view_and_query_across_writes
- crates/jj-store-cli/tests/native_lifecycle/search_paging.rs#native_search_freshness_waits_catch_up_and_reauthorize_under_admission
- crates/jj-store-cli/tests/native_lifecycle/search_paging.rs#native_search_cursor_expiry_releases_idle_generations_and_restart_resets
- crates/jj-store-cli/tests/native_lifecycle/search_paging.rs#native_search_scope_filters_truncation_and_view_retention_limits
- crates/jj-store-cli/tests/native_lifecycle/search_paging.rs#native_search_cursor_epoch_and_request_changes_fail_closed
- crates/jj-store-cli/tests/native_lifecycle/search_paging.rs#native_search_reports_served_position_during_degraded_reader_recovery
- crates/jj-store-cli/tests/native_lifecycle/search_paging.rs#native_search_requires_head_selection_and_keeps_historical_mode_explicit
- crates/jj-store-cli/tests/native_lifecycle/search_paging.rs#native_search_page_output_failure_does_not_exhaust_cursor_pins
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M6-005 details 2](details-02.md)
- [E-M6-005 details 3](details-03.md)
