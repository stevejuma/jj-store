---
schema: 1
id: E-M17-076
---

# E-M17-076

<!-- evidence-field:id markdown -->
## Id

E-M17-076
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M17
- M17.2
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-03
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

opencode-directory-aware-native-workspace-discovery
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change torvylwl
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-server/src/capture_supervisor/opencode/discovery.rs
- crates/jj-store-server/src/capture_supervisor/opencode.rs
- crates/jj-store-server/src/opencode_read_api/discovery.rs
- crates/jj-store-server/src/opencode_read_api.rs
- crates/jj-store-server/tests/opencode_discovery.rs
- crates/jj-store-server/tests/opencode_source/fixture.rs
- spec/src/cli.md
- spec/src/collaboration.md
- spec/src/verification.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-server/tests/opencode_discovery.rs#runtime_discovers_registered_native_directories_when_default_status_is_empty
- crates/jj-store-server/tests/opencode_discovery.rs#directory_candidates_without_native_write_never_receive_activity_requests
- crates/jj-store-server/tests/opencode_discovery.rs#bounded_inventories_withhold_malformed_overflow_and_unsafe_directory_requests
- crates/jj-store-server/tests/opencode_discovery.rs#admitted_sessions_recover_offline_tails_despite_unavailable_project_inventory
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M17-076 details 2](details-02.md)
- [E-M17-076 details 3](details-03.md)
