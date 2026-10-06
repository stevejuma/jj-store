---
schema: 1
id: E-READ-060
---

# E-READ-060

<!-- evidence-field:id markdown -->
## Id

E-READ-060
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

enclosing Jujutsu change mpumsyxr
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Bounded manifest-pinned native RPC inventory scan pages and complete-result client adapter
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust1.98.1;jj-lib0.45.1;macOS ARM64.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Additive native V1 InventoryPage RPC and request/reply protobuf messages;legacy Inventory request tags and bytes unchanged.No authoritative object,manifest,catalog,WAL,receipt or HTTP contract change.Catalog seek skips earlier radix subtrees;window admits at most256 descriptors,1024 control reads and8MiB encoded records before decode.Sequential descriptor checks retain only identities.Pins retain checked manifest data and query binding,no credentials or request;at most64 live pins with five-minute nonrenewing TTL,jni1 process-bound HMAC cursors.Fresh current authorization and epoch before/after every scan.NativeClient.inventory collects matching IDs through pages under100000 result IDs,two-million inspected descriptors and whole-call Options.timeout;no truncated success.
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-JJ-001
- JST-JJ-005
- JST-API-001
- JST-API-002
- JST-API-004
- JST-PUB-002
- JST-OBJ-004
- JST-CLI-003
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Native inventory control-record reader,pinned service pager,generated protobuf boundary,NativeClient and direct remote CLI consumers.Escalate to complete WAL and RPC package suites because Records reads and tonic service/schema are shared infrastructure,and complete CLI native_rpc binary because its harness/public consumers changed.UI/HTTP generation,root aggregate,full service/CLI and complete scale/fault/cold-browser/RustFS/release lanes out of scope.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/tests/native_rpc.rs
- crates/jj-store-cli/tests/native_rpc/inventory.rs
- crates/jj-store-rpc/proto/native.proto
- crates/jj-store-rpc/src/client.rs
- crates/jj-store-rpc/src/client/inventory.rs
- crates/jj-store-rpc/src/inventory.rs
- crates/jj-store-rpc/src/lib.rs
- crates/jj-store-rpc/src/server.rs
- crates/jj-store-service/src/lib.rs
- crates/jj-store-service/src/native_inventory.rs
- crates/jj-store-service/src/native_inventory/cursors.rs
- crates/jj-store-wal/src/catalog.rs
- crates/jj-store-wal/src/catalog/page.rs
- crates/jj-store-wal/src/publication.rs
- crates/jj-store-wal/src/publication/reader.rs
- crates/jj-store-wal/src/publication/reader_tests.rs
- crates/jj-store-wal/src/records.rs
- docs/implementation-plan.md
- spec/src/api.md
- spec/src/architecture.md
- spec/src/cli.md
- spec/src/jj-integration.md
- spec/src/milestones.md
- spec/src/mutations.md
- spec/src/publication.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/native_rpc/inventory.rs#rpc_inventory_collects_only_requested_ids_when_other_objects_exceed_the_budget
- crates/jj-store-cli/tests/native_rpc/inventory.rs#rpc_inventory_pages_pin_selection_and_reauthorize_sparse_windows
- crates/jj-store-cli/tests/native_rpc/inventory.rs#rpc_inventory_returns_sorted_copy_ids_and_excludes_staged_objects
- crates/jj-store-cli/tests/native_rpc/inventory.rs#inventory_pin_capacity_expiry_and_repository_binding_are_explicit
- crates/jj-store-wal/src/publication/reader_tests.rs#inventory_pages_seek_deep_anchors_and_keep_old_roots
- crates/jj-store-rpc/src/inventory.rs#validates_scan_progress_and_bounded_canonical_ids
- crates/jj-store-rpc/src/inventory.rs#legacy_inventory_wire_fields_remain_byte_identical
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-READ-060 details 2](details-02.md)
- [E-READ-060 details 3](details-03.md)
