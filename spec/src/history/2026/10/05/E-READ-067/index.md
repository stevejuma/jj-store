---
schema: 1
id: E-READ-067
---

# E-READ-067

<!-- evidence-field:id markdown -->
## Id

E-READ-067
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

zpyxqsmp
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

complete-source-scoped-native-kind-projections
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; jj-lib/jj-core 0.45.1; macOS ARM64.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Authoritative object, catalog, manifest, WAL, DTO, cursor, durable receipt and legacy RPC wire fields unchanged. Retention is disposable process-local source/epoch/root/kind data. No migration, installed executable update or service restart.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Exact native and complete-RPC regressions first; dedicated copy/baseline-loss/corrupt-addition targets; focused cache and wire units. Full native_rpc (18) and native_lifecycle (231) binaries because shared native factory authorization/complete kind enumeration and RPC client/server behavior affect two independent consumers. Focused service/RPC/CLI all-target package and document/contract checks. Root aggregates, UI/browser/local-browser and full Rust workspace/release/scale/fault/RSS/latency/RustFS qualification out of scope.
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-JJ-003
- JST-JJ-005
- JST-API-002
- JST-API-004
- JST-CLI-003
- JST-MUT-005
- JST-PUB-002
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/tests/native_rpc/inventory.rs
- crates/jj-store-rpc/src/client/inventory.rs
- crates/jj-store-rpc/src/inventory.rs
- crates/jj-store-service/src/native.rs
- crates/jj-store-service/src/native_inventory/cache/memory.rs
- crates/jj-store-service/src/native_inventory/cache/tests.rs
- crates/jj-store-service/src/native_inventory/cache.rs
- crates/jj-store-service/src/native_inventory/complete.rs
- crates/jj-store-service/src/native_inventory.rs
- docs/implementation-plan.md
- spec/src/architecture.md
- spec/src/cli.md
- spec/src/jj-integration.md
- spec/src/mutations.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/native_rpc/inventory.rs#in_process_kind_inventory_reuses_exact_roots_and_advances_only_additions
- crates/jj-store-cli/tests/native_rpc/inventory.rs#rpc_complete_kind_inventory_reuses_exact_roots_and_advances_only_additions
- crates/jj-store-cli/tests/native_rpc/inventory.rs#rpc_inventory_returns_sorted_copy_ids_and_excludes_staged_objects
- crates/jj-store-cli/tests/native_rpc/inventory.rs#complete_kind_inventory_discards_nonretained_baselines_and_rejects_corrupt_additions
- crates/jj-store-cli/tests/native_rpc/inventory.rs#inventory_deadlines_reject_ready_complete_io_after_the_budget
- crates/jj-store-cli/tests/native_rpc/inventory.rs#rpc_inventory_pages_pin_selection_and_reauthorize_sparse_windows
- crates/jj-store-service/src/native_inventory/cache/tests.rs#expiry_eviction_and_active_owners_preserve_allocation_charges
- crates/jj-store-service/src/native_inventory/cache/tests.rs#exact_kind_data_is_isolated_by_source_epoch_and_kind
- crates/jj-store-service/src/native_inventory/cache/tests.rs#growth_exhaustion_falls_back_without_retaining_incomplete_data
- crates/jj-store-rpc/src/inventory.rs#complete_inventory_rejects_oversized_malformed_duplicate_and_unordered_results
- crates/jj-store-rpc/src/inventory.rs#legacy_inventory_wire_fields_remain_byte_identical
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-READ-067 details 2](details-02.md)
- [E-READ-067 details 3](details-03.md)
