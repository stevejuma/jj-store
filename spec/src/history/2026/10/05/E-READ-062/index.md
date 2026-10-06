---
schema: 1
id: E-READ-062
---

# E-READ-062

<!-- evidence-field:id markdown -->
## Id

E-READ-062
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

lqvkpvpm
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

main-landing-prepared-operation-and-native-inventory-delivery
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust1.98.1;jj-lib0.45.1;macOS ARM64.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Original implementation,authority and cursor formats/evidence from E-READ-057 through061 retained unchanged;no format migration.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

User explicitly requests landing and verification before further delivery.Complete relevant query library,WAL,RPC,service library and CLI native_rpc/native_lifecycle/local_browser consumers combine native/local operation-history and native inventory slices.Root aggregate/UI/full contracts/cold-browser/real-RustFS/scale/RSS/fault/release qualification remains out of scope.
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-JJ-003
- JST-JJ-005
- JST-LOCAL-004
- JST-API-002
- JST-API-004
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- docs/implementation-plan.md
- spec/src/architecture.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/local_browser/prepared_operation_history.rs#prepared_operation_history_skips_prior_bodies_for_filtered_and_historical_pages
- crates/jj-store-cli/tests/native_lifecycle/operation_history_pages.rs#prepared_native_operation_history_hydrates_only_deep_anchor_and_page_candidates
- crates/jj-store-jj/src/query/prepared/operations/history/tests.rs#historical_operation_continuations_resume_without_replaying_prior_edges
- crates/jj-store-cli/tests/native_rpc/inventory.rs#rpc_legacy_inventory_ignores_unrelated_objects
- crates/jj-store-cli/tests/native_rpc/inventory.rs#native_kind_inventory_ignores_unrelated_objects
- crates/jj-store-cli/tests/native_rpc/inventory.rs#inventory_deadlines_reject_ready_complete_io_after_the_budget
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-READ-062 details 2](details-02.md)
- [E-READ-062 details 3](details-03.md)
