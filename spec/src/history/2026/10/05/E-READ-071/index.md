---
schema: 1
id: E-READ-071
---

# E-READ-071

<!-- evidence-field:id markdown -->
## Id

E-READ-071
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

xvyloluq
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Main landing and focused combined verification through E-READ-070
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; JJ/jj-lib0.45.1; macOS ARM64.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No new format in landing. Preserve the E-READ-069 additive owned kind interface and E-READ-070 additive selected inventory projection interface.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

User-requested main landing and verification. Escalate to directly affected native query library, explicit preparation, complete native RPC/lifecycle binaries and WAL library/publication tests, plus native inventory cache/wire tests and WAL/JJ/service/RPC/CLI all-target checks. Compile actual main sources while sharing the temporary generated target directory. Local-profile implementation, UI, contract generation, root aggregates and complete scale/RSS/latency/fault/cold-browser/RustFS/release qualification stay out of scope.
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-JJ-003
- JST-JJ-005
- JST-PUB-002
- JST-PUB-006
- JST-API-002
- JST-CLI-003
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- spec/src/architecture.md
- docs/implementation-plan.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/native_rpc/inventory.rs#browser_operation_prefix_ignores_unrelated_inventory_pressure
- crates/jj-store-cli/tests/native_rpc/inventory.rs#browser_kind_lookup_retains_historical_roots_and_fresh_bounds
- crates/jj-store-service/src/reads/preparation/tests.rs#prepared_kind_lookup_outlives_page_deadline_without_full_inventory
- crates/jj-store-wal/tests/publication.rs#inventory_projections_preserve_pins_copy_edges_and_manifest_only_visibility
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-READ-071 details 2](details-02.md)
- [E-READ-071 details 3](details-03.md)
