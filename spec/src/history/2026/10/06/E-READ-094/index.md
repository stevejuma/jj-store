---
schema: 1
id: E-READ-094
---

# E-READ-094

<!-- evidence-field:id markdown -->
## Id

E-READ-094
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-06
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M4.6
- M13.4
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-JJ-003
- JST-LOCAL-004
- JST-UI-005
- JST-GOV-007
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Enable progressive native and local JJ Blame with accepted cooperative cancellation
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change qntsvuuv
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Production Blame was disabled without an annotation provider. User explicitly accepted pinned JJ semantics and latency, replacing the previous hard comparison deadline gate. Work began above main fe3cb82c in retained bounded-reads; main and preview were clean and unrelated Stacks work remains in ancestry.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Shared FileAnnotator adapter, data-only progress cache, native/local authorized readers and registered routes, generated contracts and existing UI provider. No Git blame, JJ fork, reader/index redesign or changed-path preparation.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64; jj-lib 0.45.1; development Rust profile; pnpm 10.32.1; Vitest 4.1.11; existing released Design Core public APIs.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Additive BlamePage V1, native/local GET commits/{commit}/blame, opaque algorithm/source/operation/file-bound continuation and 409 blame_reset. No native identity, repository persistence, installed CLI, RPC or source-state format change.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/tests/local_browser/managed_code.rs
- crates/jj-store-cli/tests/local_browser/managed_http.rs
- crates/jj-store-cli/tests/native_lifecycle/http_browsing.rs
- crates/jj-store-cli/tests/native_lifecycle/path_metadata.rs
- crates/jj-store-http/src/error.rs
- crates/jj-store-http/src/local_browser.rs
- crates/jj-store-http/src/local_browser/blame.rs
- crates/jj-store-http/src/routes.rs
- crates/jj-store-jj/src/local_browser/budget.rs
- crates/jj-store-jj/src/local_browser/code_index.rs
- crates/jj-store-jj/src/local_browser/code_index/blame_probe.rs
- crates/jj-store-jj/src/local_browser/mod.rs
- crates/jj-store-jj/src/query/access.rs
- crates/jj-store-jj/src/query/blame.rs
- crates/jj-store-jj/src/query/blame/tests.rs
- crates/jj-store-jj/src/query/blame_probe.rs
- crates/jj-store-jj/src/query/code_index/native.rs
- crates/jj-store-jj/src/query/mod.rs
- crates/jj-store-service/src/code_indexes.rs
- crates/jj-store-service/src/code_indexes/blame.rs
- crates/jj-store-service/src/code_indexes/paths.rs
- docs/maintenance/coverage.json
- docs/site/guides/browsing.md
- docs/site/reference/api/code.md
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
- spec/src/milestones.md
- spec/src/mutations.md
- spec/src/spec-governance.md
- spec/src/ui.md
- web/src/lib/api/generated.ts
- web/src/lib/controller/blame-query.ts
- web/src/lib/controller/blame-types.ts
- web/src/lib/controller/http-blame.ts
- web/src/lib/controller/http-local-query.ts
- web/src/lib/controller/http-store-data-source.ts
- web/src/lib/controller/native-data-source.ts
- web/src/lib/controller/production-store-view.ts
- web/src/lib/ui/store-blame/StoreFileBlame.svelte
- web/src/lib/ui/store-blame/blame-model.ts
- web/tests/store-blame.test.ts
- web/tests/vite/http-blame.test.ts
<!-- /evidence-field -->


<!-- evidence-relations -->
## Resolution and supersession

The original result remains **pending**. Milestone gates select their own evidence.

- [E-READ-096](../E-READ-096/index.md) resolves this entry. Main integration completed the previously recorded landing blockers.
- This entry supersedes [E-READ-091](../E-READ-091/index.md). Accepted cooperative cancellation replaces the proposed hard comparison deadline; the original failed probe remains a failure.
<!-- /evidence-relations -->
<!-- generated-navigation -->
## Pages

- [E-READ-094 details 2](details-02.md)
- [E-READ-094 details 3](details-03.md)
