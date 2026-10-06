---
schema: 1
id: E-READ-075
---

# E-READ-075

<!-- evidence-field:id markdown -->
## Id

E-READ-075
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

xnwpnypq
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Authenticated bounded native kind preparation tasks and strict metadata-only HTTP delivery
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust1.98.1; JJ/jj-lib0.45.1; macOS ARM64.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Additive V1 HTTP start/status/cancel routes, owned process-local task reports, OpenAPI and TypeScript schemas. Native objects, inventory/WAL/manifest/receipt, existing graph task, RPC and cache formats unchanged.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Shared serving preparation admission, pinned native source lifecycle, native HTTP composition and generated transport consumers. Exact new lifecycle regression, complete affected native lifecycle binary and HTTP crate, native inventory units and affected JJ/service/HTTP/RPC/CLI/server all-target checks; complete registered HTTP/SSE response capture required for the added routes. Generated web consumer type check. Root aggregates, unrelated UI/browser/RustFS/scale lanes out of scope; complete qualification explicitly deferred by the user.
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-API-001
- JST-JJ-003
- JST-JJ-005
- JST-MUT-001
- JST-GOV-002
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/tests/native_lifecycle/content.rs
- crates/jj-store-cli/tests/native_lifecycle/http_reads.rs
- crates/jj-store-cli/tests/native_lifecycle/kind_tasks.rs
- crates/jj-store-http/src/application_events/producers.rs
- crates/jj-store-http/src/contract/tests.rs
- crates/jj-store-http/src/error.rs
- crates/jj-store-http/src/lib.rs
- crates/jj-store-http/src/routes/kind_tasks.rs
- crates/jj-store-http/src/routes.rs
- crates/jj-store-jj/src/query/workers.rs
- crates/jj-store-service/src/index_tasks.rs
- crates/jj-store-service/src/kind_tasks/state.rs
- crates/jj-store-service/src/kind_tasks/worker.rs
- crates/jj-store-service/src/kind_tasks.rs
- crates/jj-store-service/src/lib.rs
- crates/jj-store-service/src/native_inventory/preparation/state.rs
- docs/implementation-plan.md
- schemas/openapi.json
- scripts/contracts.types.ts
- spec/src/api.md
- spec/src/architecture.md
- spec/src/cli.md
- spec/src/compatibility.md
- spec/src/jj-integration.md
- spec/src/milestones.md
- spec/src/mutations.md
- spec/src/spec-governance.md
- web/src/lib/api/generated.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/native_lifecycle/kind_tasks.rs#native_kind_tasks_prepare_exact_selection_and_report_bounded_progress
- crates/jj-store-cli/tests/native_lifecycle/kind_tasks.rs#native_kind_tasks_share_graph_capacity_coalesce_cancel_and_drop
- crates/jj-store-cli/tests/native_lifecycle/kind_tasks.rs#native_kind_tasks_expire_pending_io_without_polling_and_bound_retention
- crates/jj-store-cli/tests/native_lifecycle/kind_tasks.rs#native_kind_task_reports_require_current_access_after_revocation
- crates/jj-store-cli/tests/native_lifecycle/http_reads.rs#http_contract_capture_matches_registered_routes
- crates/jj-store-http/src/contract/tests.rs#kind_preparation_contract_is_strict_actor_free_and_metadata_only
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-READ-075 details 2](details-02.md)
- [E-READ-075 details 3](details-03.md)
- [E-READ-075 details 4](details-04.md)
