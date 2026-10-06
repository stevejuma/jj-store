---
schema: 1
id: E-READ-073
---

# E-READ-073

<!-- evidence-field:id markdown -->
## Id

E-READ-073
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

kxupxzzm
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Pinned selected native copy-family projection, matching body delivery and acknowledged staged relation snapshots
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust1.98.1; JJ/jj-lib0.45.1; macOS ARM64.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Additive source-only owned Rust capability and immutable data-only relation projection. No DTO, schema, native object, RPC, signed cursor, persisted index, manifest, WAL or receipt format change.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Rust-only native source/query/backend/preparation behavior with direct service/RPC/CLI consumers. Start with the exact five-node family regression, then focused graph/cache/authority/preparation targets and complete query, native_backend, native_rpc and native_lifecycle consumer surfaces because the source capability, staged helper and shared inventory fixture affect them. Affected JJ/service/RPC/CLI all-target checks and canonical checks are required. UI, root aggregates, contract generation and unchanged WAL/storage suites are out of scope. Full scale/RSS/latency/fault/cold-browser/RustFS/release/Help qualification remains explicitly deferred.
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-JJ-003
- JST-JJ-005
- JST-MUT-001
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/tests/native_rpc/inventory.rs
- crates/jj-store-jj/src/backend/staged_copies.rs
- crates/jj-store-jj/src/backend.rs
- crates/jj-store-jj/src/commands/access.rs
- crates/jj-store-jj/src/copy_query.rs
- crates/jj-store-jj/src/query/access/kinds_tests.rs
- crates/jj-store-jj/src/query/access/selection.rs
- crates/jj-store-jj/src/query/access.rs
- crates/jj-store-jj/src/query/copy_index.rs
- crates/jj-store-jj/src/query/mod.rs
- crates/jj-store-jj/tests/copy_index.rs
- crates/jj-store-jj/tests/native_backend/families.rs
- crates/jj-store-jj/tests/native_backend.rs
- crates/jj-store-service/src/native.rs
- crates/jj-store-service/src/native_inventory/cache/memory.rs
- crates/jj-store-service/src/native_inventory/cache.rs
- crates/jj-store-service/src/native_inventory/complete.rs
- crates/jj-store-service/src/native_inventory/families/cache/tests.rs
- crates/jj-store-service/src/native_inventory/families/cache.rs
- crates/jj-store-service/src/native_inventory/families.rs
- crates/jj-store-service/src/native_inventory.rs
- crates/jj-store-service/src/reads/preparation/tests.rs
- crates/jj-store-service/src/reads/preparation.rs
- crates/jj-store-service/src/reads.rs
- docs/implementation-plan.md
- spec/src/architecture.md
- spec/src/cli.md
- spec/src/jj-integration.md
- spec/src/mutations.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/native_rpc/inventory.rs#native_copy_family_ignores_unrelated_inventory_and_body_pressure
- crates/jj-store-cli/tests/native_rpc/inventory.rs#pinned_copy_families_reuse_additions_without_loading_bodies
- crates/jj-store-jj/tests/copy_index.rs#copy_provider_admits_before_growth_and_baseline_clone
- crates/jj-store-jj/tests/native_backend/families.rs#projected_family_merges_committed_and_acknowledged_relations_only
- crates/jj-store-jj/tests/native_backend/families.rs#projected_family_captures_staging_before_provider_await
- crates/jj-store-jj/tests/native_backend/families.rs#staged_copy_edges_and_bytes_are_admitted_before_upload
- crates/jj-store-jj/src/query/access/kinds_tests.rs#copy_family_authority_preserves_pins_typed_failures_and_fresh_bounds
- crates/jj-store-jj/src/query/access/kinds_tests.rs#pending_copy_family_obeys_cancellation_and_whole_query_deadline
- crates/jj-store-service/src/native_inventory/families/cache/tests.rs#copy_slots_coalesce_by_source_epoch_and_keep_busy_gates
- crates/jj-store-service/src/native_inventory/families/cache/tests.rs#copy_growth_and_eviction_keep_live_allocation_charged
- crates/jj-store-service/src/native_inventory/families/cache/tests.rs#copy_slot_expiry_is_fixed_and_does_not_replace_a_busy_gate
- crates/jj-store-service/src/reads/preparation/tests.rs#prepared_kind_lookup_outlives_page_deadline_without_full_inventory
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-READ-073 details 2](details-02.md)
- [E-READ-073 details 3](details-03.md)
