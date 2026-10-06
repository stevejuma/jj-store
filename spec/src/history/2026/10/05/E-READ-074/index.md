---
schema: 1
id: E-READ-074
---

# E-READ-074

<!-- evidence-field:id markdown -->
## Id

E-READ-074
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

ptnmxplv
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Callable bounded resumable native kind source preparation and complete-root cache promotion
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust1.98.1; JJ/jj-lib0.45.1; macOS ARM64.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Additive owned Rust source APIs and caller-owned process-local progress only. No native, manifest, WAL, receipt, DTO/schema, HTTP/RPC, signed cursor or persisted index format change.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Rust-only native inventory/preparation capability with a pinned CommittedRead entry point. Exact new resume/promotion acceptance, complete native_inventory cache/family/preparation units, existing independent read preparation units and service all-target check. Production graph/background-task execution and all transport payloads/call sites are unchanged; JJ/RPC/CLI/UI tests, root aggregates and contract generation remain out of scope. Full scale/RSS/latency/fault/cold-browser/RustFS/release/Help qualification remains explicitly deferred.
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-JJ-003
- JST-JJ-005
- JST-MUT-001
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-service/src/native_inventory/cache.rs
- crates/jj-store-service/src/native_inventory/preparation/state.rs
- crates/jj-store-service/src/native_inventory/preparation/tests/cancellation.rs
- crates/jj-store-service/src/native_inventory/preparation/tests.rs
- crates/jj-store-service/src/native_inventory/preparation.rs
- crates/jj-store-service/src/native_inventory.rs
- crates/jj-store-service/src/reads.rs
- docs/implementation-plan.md
- spec/src/architecture.md
- spec/src/jj-integration.md
- spec/src/mutations.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-service/src/native_inventory/preparation/tests.rs#native_kind_preparation_resumes_exact_windows_and_promotes_only_complete_ids
- crates/jj-store-service/src/native_inventory/preparation/tests.rs#native_kind_preparation_keeps_total_bounds_after_failed_windows
- crates/jj-store-service/src/native_inventory/preparation/tests.rs#native_kind_preparation_authorization_and_deadlines_precede_window_admission
- crates/jj-store-service/src/native_inventory/preparation/tests.rs#native_kind_preparation_rejects_foreign_sources_and_epoch_changes_after_promotion
- crates/jj-store-service/src/native_inventory/preparation/tests.rs#pinned_kind_preparation_outlives_the_initiating_page_and_empty_kind_is_complete
- crates/jj-store-service/src/native_inventory/preparation/tests.rs#prepared_kind_progress_and_ready_ids_reauthorize_after_token_revocation
- crates/jj-store-service/src/native_inventory/preparation/tests/cancellation.rs#interrupted_kind_windows_retain_conservative_work_without_advancing_or_promoting
- crates/jj-store-service/src/reads/preparation/tests.rs#prepared_kind_lookup_outlives_page_deadline_without_full_inventory
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-READ-074 details 2](details-02.md)
- [E-READ-074 details 3](details-03.md)
