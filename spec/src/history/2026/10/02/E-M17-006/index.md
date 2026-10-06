---
schema: 1
id: E-M17-006
---

# E-M17-006

<!-- evidence-field:id markdown -->
## Id

E-M17-006
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

native-capture-directory-admission
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/src/workspace_management.rs
- crates/jj-store-cli/tests/workspace_management.rs
- crates/jj-store-server/src/capture_admission.rs
- crates/jj-store-server/src/lib.rs
- crates/jj-store-server/tests/capture_admission.rs
- crates/jj-store-service/src/local_management.rs
- spec/src/cli.md
- spec/src/collaboration.md
- spec/src/milestones.md
- spec/src/mutations.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/workspace_management.rs#capture_discovery_resolves_nested_and_linked_workspaces_without_mutation
- crates/jj-store-cli/tests/workspace_management.rs#capture_discovery_stops_at_inner_boundaries_metadata_and_depth_limit
- crates/jj-store-cli/tests/workspace_management.rs#capture_binding_changes_on_connection_or_physical_replacement
- crates/jj-store-cli/tests/workspace_management.rs#server_capture_admits_only_published_registry_bindings_and_revalidates_them
- crates/jj-store-server/tests/capture_admission.rs#root_only_verifiers_cannot_accidentally_admit_capture
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M17-006 details 2](details-02.md)
- [E-M17-006 details 3](details-03.md)
