---
schema: 1
id: E-M17-011
---

# E-M17-011

<!-- evidence-field:id markdown -->
## Id

E-M17-011
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M17
- M17.2
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-02
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

approved-codex-location-discovery
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-capture/src/codex_discovery/scanner.rs
- crates/jj-store-capture/src/codex_discovery.rs
- crates/jj-store-capture/src/lib.rs
- crates/jj-store-capture/tests/codex_discovery.rs
- crates/jj-store-http/src/local_browser.rs
- crates/jj-store-server/src/capture_config.rs
- crates/jj-store-server/tests/configuration.rs
- spec/src/api.md
- spec/src/cli.md
- spec/src/collaboration.md
- spec/src/compatibility.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-capture/tests/codex_discovery.rs#codex_locations_honor_explicit_roots_then_home_override_without_creating_paths
- crates/jj-store-capture/tests/codex_discovery.rs#incremental_discovery_finishes_large_directories_without_reading_transcript_bodies
- crates/jj-store-capture/tests/codex_discovery.rs#discovery_withholds_rebound_roots_symlink_children_and_changed_candidates
- crates/jj-store-capture/tests/codex_discovery.rs#interrupted_scan_withholds_rebound_child_and_reports_depth_and_budget_limits
- crates/jj-store-server/tests/configuration.rs#configured_codex_locations_use_qualified_versions_and_explicit_override_precedence
- crates/jj-store-server/tests/configuration.rs#automatic_capture_configuration_is_explicit_local_writable_and_bounded
- crates/jj-store-server/tests/configuration.rs#enabled_capture_startup_fails_before_state_creation_until_workers_are_connected
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M17-011 details 2](details-02.md)
- [E-M17-011 details 3](details-03.md)
