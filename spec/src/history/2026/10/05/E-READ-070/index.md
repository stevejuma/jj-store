---
schema: 1
id: E-READ-070
---

# E-READ-070

<!-- evidence-field:id markdown -->
## Id

E-READ-070
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

zspxprxy
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Bounded selected inventory operation and copy-edge projection primitive
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; jj-lib/jj-core and jj 0.45.1; macOS ARM64.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Additive WAL source interface only. No persisted catalog, descriptor, manifest, native body, DTO, RPC, cursor or receipt format changes.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Rust-only selected control-record reader. Shared CountedStore fixture justifies complete owning WAL library tests; selected snapshot API receives complete publication binary tests. Run owning WAL all-target check plus canonical and changed-document checks. Additive APIs have no current production consumers; unchanged consumer builds, UI, transport generation and root aggregates are out of scope. Full scale/RSS/latency/fault/cold-browser/RustFS/release qualification remains deferred.
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-PUB-002
- JST-PUB-006
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-wal/src/catalog/page.rs
- crates/jj-store-wal/src/publication/reader/projection.rs
- crates/jj-store-wal/src/publication/reader.rs
- crates/jj-store-wal/src/publication/reader_tests.rs
- crates/jj-store-wal/src/publication.rs
- crates/jj-store-wal/tests/publication.rs
- spec/src/publication.md
- spec/src/architecture.md
- docs/implementation-plan.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-wal/src/publication/reader_tests.rs#projection_pages_read_copy_parent_edges_once_under_unrelated_inventory_pressure
- crates/jj-store-wal/src/publication/reader_tests.rs#projection_edge_admission_continues_without_losing_a_copy_or_certifying_a_prefix
- crates/jj-store-wal/src/publication/reader_tests.rs#projection_additions_retain_copy_edges_without_reading_unchanged_descriptors
- crates/jj-store-wal/src/publication/reader_tests.rs#projection_pages_reject_invalid_shapes_limits_and_descriptor_identity
- crates/jj-store-wal/src/publication/reader_tests.rs#inventory_pages_continue_before_byte_admission_without_losing_descriptors
- crates/jj-store-wal/tests/publication.rs#inventory_projections_preserve_pins_copy_edges_and_manifest_only_visibility
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-READ-070 details 2](details-02.md)
- [E-READ-070 details 3](details-03.md)
