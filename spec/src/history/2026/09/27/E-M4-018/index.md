---
schema: 1
id: E-M4-018
---

# E-M4-018

<!-- evidence-field:id markdown -->
## Id

E-M4-018
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M4
- M4.3
- M4.10
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-27
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

implementation
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/tests/native_lifecycle/browsing.rs
- crates/jj-store-cli/tests/native_lifecycle/http_browsing.rs
- crates/jj-store-cli/tests/native_lifecycle.rs
- crates/jj-store-http/src/contract/tests.rs
- crates/jj-store-http/src/dto.rs
- crates/jj-store-http/src/lib.rs
- crates/jj-store-http/src/routes.rs
- crates/jj-store-jj/src/query/content/diff_summary.rs
- crates/jj-store-jj/src/query/content/operation_changes.rs
- crates/jj-store-jj/src/query/content/operations.rs
- crates/jj-store-jj/src/query/content/paths.rs
- crates/jj-store-jj/src/query/content/references.rs
- crates/jj-store-jj/src/query/content.rs
- crates/jj-store-service/src/pagination/browsing.rs
- crates/jj-store-service/src/pagination/cursors.rs
- crates/jj-store-service/src/pagination/operation_browsing.rs
- crates/jj-store-service/src/pagination.rs
- crates/jj-store-service/src/reads.rs
- docs/implementation-plan.md
- schemas/openapi.json
- scripts/contracts.mjs
- scripts/contracts.types.ts
- spec/src/api.md
- spec/src/architecture.md
- spec/src/cli.md
- spec/src/compatibility.md
- spec/src/jj-integration.md
- spec/src/mutations.md
- spec/src/spec-governance.md
- spec/src/ui.md
- spec/src/milestones.md
- web/src/lib/api/generated.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/native_lifecycle/browsing.rs#native_browser_windows_preserve_references_paths_groups_and_exact_diffs
- crates/jj-store-cli/tests/native_lifecycle/browsing.rs#signed_browser_cursors_bind_queries_support_backward_windows_and_reset_on_restart
- crates/jj-store-cli/tests/native_lifecycle/http_browsing.rs#http_browsing_windows_are_authorized_pinned_cursor_bound_and_contract_captured
- crates/jj-store-http/src/contract/tests.rs#browsing_contracts_keep_complete_reads_and_expose_bound_windows
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M4-018 details 2](details-02.md)
