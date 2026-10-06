---
schema: 1
id: E-READ-051
---

# E-READ-051

<!-- evidence-field:id markdown -->
## Id

E-READ-051
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

enclosing Jujutsu change lvproltr
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Local prepared reference windows under a distinct outer operation with exact source recovery
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; jj-lib/jj-cli 0.45.1; local Git SHA-1 browser profile; macOS ARM64.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No public DTO/schema, native/local persistence, index, signed cursor, WAL or receipt format change. Cold mixed indexed references now report their exact source in the existing local_snapshot operation. Successful snapshots and signed continuations retain the outer operation, request digest and registration generation; reference pages identify the source.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/tests/local_browser/reference_embedded.rs
- crates/jj-store-cli/tests/local_browser.rs
- crates/jj-store-http/src/local_browser.rs
- crates/jj-store-jj/src/local_browser/mod.rs
- crates/jj-store-jj/src/local_browser/preparation/graph.rs
- crates/jj-store-jj/src/local_browser/references/selection.rs
- crates/jj-store-jj/src/local_browser/references.rs
- crates/jj-store-service/src/local_index_tasks.rs
- docs/implementation-plan.md
- spec/src/api.md
- spec/src/architecture.md
- spec/src/cli.md
- spec/src/collaboration.md
- spec/src/jj-integration.md
- spec/src/local-jj.md
- spec/src/milestones.md
- spec/src/mutations.md
- web/src/lib/controller/http-local-query.ts
- web/src/lib/controller/http-threads-data-source.ts
- web/src/lib/controller/read-query.ts
- web/tests/vite/local-browser.test.ts
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Commit the verified local embedded-operation slice, integrate accumulated delivery into main and run combined focused verification there, then continue high-priority features in bounded-reads. Full scale/RSS/latency/fault/cold browser/RustFS/release qualification stays deferred.
<!-- /evidence-field -->

<!-- evidence-field:commands list -->
## Commands

[]
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/local_browser/reference_embedded.rs#local_embedded_references_prepare_exact_source_and_preserve_outer_cursor_authority
- web/tests/vite/local-browser.test.ts#local_embedded_reference_query_validates_source_recovery_and_outer_success
- web/tests/vite/read-query.test.ts#one observer cancellation cannot abort
  another consumer's shared read
- web/tests/vite/read-query.test.ts#the final observer release aborts obsolete
  transport
- web/tests/vite/thread-query-cache.test.ts#Thread queries share session cache
  and target invalidation retains unrelated data
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-READ-051 details 2](details-02.md)
- [E-READ-051 details 3](details-03.md)
