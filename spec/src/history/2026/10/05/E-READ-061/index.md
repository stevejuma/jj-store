---
schema: 1
id: E-READ-061
---

# E-READ-061

<!-- evidence-field:id markdown -->
## Id

E-READ-061
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

nksqwmtt
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

bounded-native-kind-inventory-and-adaptive-descriptor-windows
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust1.98.1;jj-lib0.45.1;macOS ARM64.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No authoritative catalog,record,WAL,native identity or RPC wire format changes;additive jni1 page cursor from E-READ-060 unchanged.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Escalate to complete WAL/RPC suites and CLI native_rpc/native_lifecycle binaries:adaptive catalog reads affect paged RPC and shared NativeSession authority feeds legacy transport and in-process factories. Rerun RPC/CLI consumers after deadline fix;unchanged WAL passing suite retained. Root aggregate/UI/HTTP generation and complete scale/fault/cold-browser/RustFS/release lanes out of scope.
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-JJ-001
- JST-JJ-005
- JST-API-002
- JST-API-004
- JST-PUB-002
- JST-OBJ-004
- JST-CLI-003
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/tests/native_rpc/inventory.rs
- crates/jj-store-rpc/src/client/inventory.rs
- crates/jj-store-service/src/native.rs
- crates/jj-store-service/src/native_inventory.rs
- crates/jj-store-service/src/native_inventory/complete.rs
- crates/jj-store-service/src/transport.rs
- crates/jj-store-wal/src/catalog/page.rs
- crates/jj-store-wal/src/publication/reader.rs
- crates/jj-store-wal/src/publication/reader_tests.rs
- docs/implementation-plan.md
- spec/src/architecture.md
- spec/src/cli.md
- spec/src/jj-integration.md
- spec/src/milestones.md
- spec/src/mutations.md
- spec/src/publication.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/native_rpc/inventory.rs#rpc_legacy_inventory_ignores_unrelated_objects
- crates/jj-store-cli/tests/native_rpc/inventory.rs#native_kind_inventory_ignores_unrelated_objects
- crates/jj-store-cli/tests/native_rpc/inventory.rs#inventory_deadlines_reject_ready_page_io_after_the_budget
- crates/jj-store-cli/tests/native_rpc/inventory.rs#inventory_deadlines_reject_ready_complete_io_after_the_budget
- crates/jj-store-wal/src/publication/reader_tests.rs#inventory_pages_continue_before_byte_admission_without_losing_descriptors
- crates/jj-store-wal/src/publication/reader_tests.rs#catalog_work_admission_continues_after_a_verified_nonempty_prefix
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-READ-061 details 2](details-02.md)
- [E-READ-061 details 3](details-03.md)
