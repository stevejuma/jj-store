---
schema: 1
id: E-READ-076
---

# E-READ-076

<!-- evidence-field:id markdown -->
## Id

E-READ-076
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

ymrvkrow
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Main landing and focused combined verification of selected copy families and resumable native kind tasks
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust1.98.1; JJ/jj-lib0.45.1; macOS ARM64.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Existing E-READ-072 through075 source and additive V1 task contracts preserved. This landing changes canonical documentation and evidence only; no further production, authoritative, generated contract or persisted format change.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

User-requested landing of the four verified follow-up slices into clean main, followed by focused combined source verification. Complete copy_index/query/native_backend/native_rpc/native_lifecycle and native inventory/preparation/HTTP consumer surfaces, affected JJ/service/HTTP/RPC/CLI/server all-target checks, generated consumer/web type checks and canonical bookkeeping. Wider root aggregates, visual baselines, complete scale/RSS/latency/fault/cold-browser/RustFS/release and Help qualification remain out of scope and explicitly deferred.
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-JJ-003
- JST-JJ-005
- JST-API-001
- JST-MUT-001
- JST-GOV-002
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- spec/src/architecture.md
- spec/src/spec-governance.md
- docs/implementation-plan.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-jj/tests/copy_index.rs#copy_family_includes_ancestor_descendants_without_unrelated_parent_branches
- crates/jj-store-jj/tests/native_backend/families.rs#projected_family_merges_committed_and_acknowledged_relations_only
- crates/jj-store-cli/tests/native_rpc/inventory.rs#native_copy_family_ignores_unrelated_inventory_and_body_pressure
- crates/jj-store-cli/tests/native_lifecycle/kind_tasks.rs#native_kind_tasks_prepare_exact_selection_and_report_bounded_progress
- crates/jj-store-cli/tests/native_lifecycle/kind_tasks.rs#native_kind_tasks_share_graph_capacity_coalesce_cancel_and_drop
- crates/jj-store-cli/tests/native_lifecycle/kind_tasks.rs#native_kind_tasks_expire_pending_io_without_polling_and_bound_retention
- crates/jj-store-cli/tests/native_lifecycle/kind_tasks.rs#native_kind_task_reports_require_current_access_after_revocation
- crates/jj-store-service/src/native_inventory/preparation/tests.rs#native_kind_preparation_resumes_exact_windows_and_promotes_only_complete_ids
- crates/jj-store-service/src/reads/preparation/tests.rs#prepared_kind_lookup_outlives_page_deadline_without_full_inventory
- crates/jj-store-http/src/contract/tests.rs#kind_preparation_contract_is_strict_actor_free_and_metadata_only
- scripts/contracts.test.mjs#tagged_union_and_required_nullable_fields_fail_closed
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-READ-076 details 2](details-02.md)
- [E-READ-076 details 3](details-03.md)
