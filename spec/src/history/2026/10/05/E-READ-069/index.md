---
schema: 1
id: E-READ-069
---

# E-READ-069

<!-- evidence-field:id markdown -->
## Id

E-READ-069
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

zoxkwqvx
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Pinned browser and explicit preparation kind identity lookup
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; jj-lib/jj-core and jj 0.45.1; macOS ARM64.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Owned ReadAccess InventoryKind enumeration is a source-only interface addition.No persisted,DTO,schema,cursor,RPC,receipt,authoritative or installed format changes.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Rust-only read authority/provider slice.Escalate to complete native_rpc and native_lifecycle binaries plus owning query library tests:the shared upstream read authority also serves graph/content/command consumers,and the refactored complete-kind provider serves both factory and RPC reads.Run exact preparation boundary and affected JJ/service/RPC/CLI all-target package checks.Local-profile implementation,UI,contracts generation,root aggregates,complete scale/RSS/latency/fault/cold-browser/RustFS/release and Help qualification remain out of scope.
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-JJ-003
- JST-JJ-005
- JST-API-002
- JST-CLI-003
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-jj/src/query/mod.rs
- crates/jj-store-jj/src/query/access.rs
- crates/jj-store-jj/src/query/access/selection.rs
- crates/jj-store-jj/src/query/access/kinds_tests.rs
- crates/jj-store-service/src/native_inventory/complete.rs
- crates/jj-store-service/src/reads.rs
- crates/jj-store-service/src/reads/preparation.rs
- crates/jj-store-service/src/reads/preparation/tests.rs
- crates/jj-store-cli/tests/native_rpc/inventory.rs
- spec/src/jj-integration.md
- spec/src/mutations.md
- spec/src/cli.md
- spec/src/architecture.md
- docs/implementation-plan.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/native_rpc/inventory.rs#browser_operation_prefix_ignores_unrelated_inventory_pressure
- crates/jj-store-cli/tests/native_rpc/inventory.rs#browser_kind_lookup_retains_historical_roots_and_fresh_bounds
- crates/jj-store-jj/src/query/access/kinds_tests.rs#compatibility_kind_lookup_filters_with_exact_result_bounds
- crates/jj-store-jj/src/query/access/kinds_tests.rs#upstream_authorities_use_kind_lookup_and_recheck_query_budget
- crates/jj-store-jj/src/query/access/kinds_tests.rs#historical_operation_wrapper_preserves_outer_kind_capability
- crates/jj-store-jj/src/query/access/kinds_tests.rs#pending_kind_lookup_obeys_query_cancellation
- crates/jj-store-service/src/reads/preparation/tests.rs#prepared_kind_lookup_outlives_page_deadline_without_full_inventory
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-READ-069 details 2](details-02.md)
- [E-READ-069 details 3](details-03.md)
- [E-READ-069 details 4](details-04.md)
