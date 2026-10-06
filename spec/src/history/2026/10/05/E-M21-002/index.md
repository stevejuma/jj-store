---
schema: 1
id: E-M21-002
---

# E-M21-002

<!-- evidence-field:id markdown -->
## Id

E-M21-002
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

enclosing Jujutsu change qqyqtppu
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

bounded-operation-pinned-stack-reads
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

JJ 0.45.1; pnpm 10.32.1; Vitest 4.1.11
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Adds a typed stack_history variant to the existing local query union and generated OpenAPI/client. No source repository, companion, checkout or command execution format changes.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/tests/local_browser/stacks.rs
- crates/jj-store-cli/tests/local_browser.rs
- crates/jj-store-http/src/local_browser.rs
- crates/jj-store-jj/src/local_browser/dispatch.rs
- crates/jj-store-jj/src/local_browser/mod.rs
- crates/jj-store-jj/src/local_browser/preparation/graph.rs
- crates/jj-store-jj/src/local_browser/stacks.rs
- docs/maintenance/coverage.json
- docs/site/reference/api/installation.md
- docs/site/reference/api/types.md
- docs/site/reference/transport.md
- docs/site/technical/local.md
- schemas/openapi.json
- spec/src/api.md
- spec/src/cli.md
- spec/src/compatibility.md
- spec/src/documentation.md
- spec/src/jj-integration.md
- spec/src/local-jj.md
- web/src/lib/api/generated.ts
- web/src/lib/controller/http-local-code-data-source.ts
- web/src/lib/controller/native-data-source.ts
- web/tests/vite/local-stack-history.test.ts
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Before: eleven missing stack module/variant compiler diagnostics, and two missing client validation regressions. Intermediate HTTP test expected 409 instead of the canonical 400 invalid_request; test corrected without changing admission. Client runs also exposed cancellation failures repaired separately in enclosing parent uvzrpntl.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Bounded explicit head/base local graph history and typed transport. Existing prepared index, operation/binding pin, opaque cursor and read budgets reused. Net files remain exact tree comparisons. Shared public query union requires the unchanged complete registered HTTP/SSE capture inventory.
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/local_browser/stacks.rs#local_stack_history_is_bounded_pinned_and_read_only
- web/tests/vite/local-stack-history.test.ts#local_stack_transport_keeps_operation_range_and_opaque_cursor
- web/tests/vite/local-stack-history.test.ts#local_stack_transport_rejects_foreign_nested_range_and_oversized_pages
- web/tests/vite/local-stack-history.test.ts#local_stack_transport_rejects_invalid_bounds_before_network
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M21-002 details 2](details-02.md)
- [E-M21-002 details 3](details-03.md)
