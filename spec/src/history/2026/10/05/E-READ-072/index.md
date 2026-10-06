---
schema: 1
id: E-READ-072
---

# E-READ-072

<!-- evidence-field:id markdown -->
## Id

E-READ-072
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

ysoumkvm
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Immutable bounded copy relation index and exact family lookup primitive
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust1.98.1; JJ/jj-lib0.45.1; macOS ARM64.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Additive owned Rust interface only, using canonical CopyId values and distinct parent/child sets. No serialization, DTO, cursor, RPC, persisted projection, authoritative native or receipt format changes.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Rust-only unused read-index primitive. Exact new acceptance target, complete new copy_index test binary and owning JJ all-target check. No existing production consumer invokes this additive interface, so existing query/native/RPC/UI binaries, contract generation and root aggregates stay out of scope. Provider and backend wiring are a following slice; complete scale/RSS/latency/fault/cold-browser/RustFS/release qualification remains deferred.
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-JJ-003
- JST-JJ-005
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-jj/src/query/mod.rs
- crates/jj-store-jj/src/query/copy_index.rs
- crates/jj-store-jj/tests/copy_index.rs
- spec/src/jj-integration.md
- spec/src/architecture.md
- docs/implementation-plan.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-jj/tests/copy_index.rs#copy_family_includes_ancestor_descendants_without_unrelated_parent_branches
- crates/jj-store-jj/tests/copy_index.rs#copy_family_handles_reverse_leaf_and_multiple_seed_requests
- crates/jj-store-jj/tests/copy_index.rs#copy_index_seals_only_complete_acyclic_unchanged_relations
- crates/jj-store-jj/tests/copy_index.rs#copy_index_admits_growth_before_mutation_and_bounds_sealing_work
- crates/jj-store-jj/tests/copy_index.rs#copy_family_applies_fresh_work_bytes_deadline_and_cancellation_bounds
- crates/jj-store-jj/tests/copy_index.rs#copy_index_cpu_walks_yield_for_pending_cancellation
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-READ-072 details 2](details-02.md)
- [E-READ-072 details 3](details-03.md)
