---
schema: 1
id: E-READ-077
---

# E-READ-077

<!-- evidence-field:id markdown -->
## Id

E-READ-077
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

mzmylqwx
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Bounded selected copy families for the remote native CLI authority
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust1.98.1; JJ/jj-lib0.45.1; macOS ARM64.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Additive CopyFamily native RPC V1 method with sorted typed seed flags, tightening allowances, complete sorted identity results and charged traversal work. Existing Inventory/InventoryPage framing, HTTP/OpenAPI/client contracts, native objects, manifest, WAL, receipts and cache formats are unchanged.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Focused owned service facade, RPC framing/client/server and real remote native factory consumers. Complete native_rpc binary, RPC lib and native_inventory module plus affected service/RPC/CLI all-target checks and canonical bookkeeping. Unchanged HTTP response capture and web/type lanes are out of scope. Complete scale/RSS/latency/fault/cold-browser/RustFS/installed-runtime/release and Help qualification stays deferred; no milestone promotion.
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-JJ-001
- JST-JJ-003
- JST-JJ-005
- JST-CLI-003
- JST-MUT-001
- JST-GOV-002
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/src/remote.rs
- crates/jj-store-cli/tests/native_rpc/inventory.rs
- crates/jj-store-cli/tests/native_rpc/inventory/copy_families.rs
- crates/jj-store-rpc/proto/native.proto
- crates/jj-store-rpc/src/client/copy_families.rs
- crates/jj-store-rpc/src/client.rs
- crates/jj-store-rpc/src/copy_families.rs
- crates/jj-store-rpc/src/lib.rs
- crates/jj-store-rpc/src/server.rs
- crates/jj-store-service/src/transport/copy_families.rs
- crates/jj-store-service/src/transport.rs
- spec/src/jj-integration.md
- spec/src/cli.md
- spec/src/mutations.md
- spec/src/architecture.md
- spec/src/compatibility.md
- spec/src/spec-governance.md
- docs/implementation-plan.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/native_rpc/inventory/copy_families.rs#remote_copy_family_ignores_unrelated_inventory_and_body_pressure
- crates/jj-store-cli/tests/native_rpc/inventory/copy_families.rs#rpc_copy_family_preserves_limits_selection_and_fresh_authorization
- crates/jj-store-cli/tests/native_rpc/inventory/copy_families.rs#rpc_copy_family_rejects_invalid_requests_before_storage
- crates/jj-store-cli/tests/native_rpc/inventory/copy_families.rs#remote_copy_family_merges_only_acknowledged_client_staging
- crates/jj-store-cli/tests/native_rpc/inventory/copy_families.rs#remote_copy_family_falls_back_only_for_unimplemented_peers
- crates/jj-store-cli/tests/native_rpc/inventory/copy_families.rs#rpc_copy_family_deadline_cancels_pending_storage_and_releases_capacity
- crates/jj-store-rpc/src/copy_families.rs#selected_family_request_bounds_and_canonical_seed_flags
- crates/jj-store-rpc/src/copy_families.rs#selected_family_response_rejects_truncation_work_and_invalid_identities
- crates/jj-store-rpc/src/client/copy_families.rs#local_transport_timeout_retains_deadline_semantics
- crates/jj-store-rpc/src/inventory.rs#legacy_inventory_wire_fields_remain_byte_identical
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-READ-077 details 2](details-02.md)
- [E-READ-077 details 3](details-03.md)
