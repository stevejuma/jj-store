---
schema: 1
id: E-M21-030
---

# E-M21-030

<!-- evidence-field:id markdown -->
## Id

E-M21-030
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M21
- M21.2
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-05
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

zlpmntul
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Complete original physical inventory for interrupted-checkout guards
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; pinned JJ 0.45.1/Git SHA-1 profile; macOS ARM64. Isolated real-native and scanner fixtures; CARGO_INCREMENTAL=0. Existing development executable used for explicit folder tests; installed binaries/services unchanged.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Owned PhysicalInventoryV1 canonical JSON: 8 MiB and 4097 combined directory/file records. Directory stamps retain the eight-word dev/ino/len/mode/mtime/ctime scanner frame. Existing physical-tree/v1 fingerprint unchanged; inventory/v1 digest has a separate domain. No public wire, native, companion, WAL, plan-catalog or journal-format changes.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-jj/src/local_execution/mod.rs
- crates/jj-store-jj/src/local_execution/physical_inventory/validation.rs
- crates/jj-store-jj/src/local_execution/physical_inventory.rs
- crates/jj-store-jj/src/local_execution/recovery/checkout.rs
- crates/jj-store-service/src/working_tree/contracts.rs
- crates/jj-store-service/src/working_tree/guard.rs
- crates/jj-store-service/src/working_tree/inventory.rs
- crates/jj-store-service/src/working_tree/mod.rs
- crates/jj-store-service/src/working_tree/walk.rs
- crates/jj-store-service/tests/working_tree_inventory/adversarial.rs
- crates/jj-store-service/tests/working_tree_inventory/guard.rs
- crates/jj-store-service/tests/working_tree_inventory.rs
- docs/implementation-plan.md
- docs/maintenance/coverage.json
- docs/site/technical/architecture.md
- docs/site/technical/local.md
- spec/src/architecture.md
- spec/src/documentation.md
- spec/src/jj-integration.md
- spec/src/local-jj.md
- spec/src/mutations.md
- spec/src/verification.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-LOCAL-005
- JST-JJ-002
- JST-JJ-004
- JST-MUT-001
- JST-GOV-002
- JST-GOV-003
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

The exact target initially fails with two missing inventory API diagnostics. One strict lint diagnostic is fixed by checked frame conversion; the initially missing mapped architecture chapter is added. Failed development --version probe is recorded; all required final lanes pass without weakened assertions.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Original private input only: sorted complete file fingerprints and directory stamps reproduce the unchanged physical digest, including ignored/untracked paths, symlinks and empty directories. Optional registered PhysicalGuard capture shares existing bounded two-pass scan, authorization, deadline and admission through cancellation. Validation-only guards explicitly refuse capture. Canonical data conveys no normalization, physical write or completion authority.
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-service/tests/working_tree_inventory/adversarial.rs#noncanonical_tampered_incomplete_and_conflicting_inventory_cannot_decode
- crates/jj-store-service/tests/working_tree_inventory/adversarial.rs#physical_rows_and_scope_must_match_the_exact_original_digest
- crates/jj-store-service/tests/working_tree_inventory/adversarial.rs#record_byte_path_and_namespace_bounds_fail_without_a_partial_inventory
- crates/jj-store-service/tests/working_tree_inventory/guard.rs#registered_access_guard_captures_complete_inventory_and_preserves_validation
- crates/jj-store-service/tests/working_tree_inventory/guard.rs#revoked_and_replaced_binding_cannot_capture_or_initialize_inventory
- crates/jj-store-service/tests/working_tree_inventory/guard.rs#validation_only_guard_does_not_gain_inventory_capture
- crates/jj-store-service/tests/working_tree_inventory/guard.rs#inventory_capture_cancellation_retains_shared_admission_until_worker_exits
- crates/jj-store-service/tests/working_tree_inventory.rs#original_inventory_roundtrips_the_complete_scanner_fingerprint
- crates/jj-store-service/tests/working_tree_inventory.rs#inventory_retains_ignored_symlink_binary_and_unicode_rows_without_content
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M21-030 details 2](details-02.md)
- [E-M21-030 details 3](details-03.md)
- [E-M21-030 details 4](details-04.md)
