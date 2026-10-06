---
schema: 1
id: E-M21-010
---

# E-M21-010

<!-- evidence-field:id markdown -->
## Id

E-M21-010
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

mwwqxzmr
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Registered bounded physical working-tree observation transport
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; pinned JJ 0.45.1; pnpm 10.32.1; macOS ARM64. Only current workspace development/test/example outputs built; no installed executable or service replacement.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Additive version-1 LocalTrusted HTTP ObserveWorkingTree/ObservedWorkingTree and generated client/OpenAPI records. No native/WAL/companion/private persisted format change. Read fingerprints are observations, not issued staging tokens or preparations.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/tests/local_browser.rs
- crates/jj-store-cli/tests/local_browser/working_tree.rs
- crates/jj-store-http/src/contract/tests.rs
- crates/jj-store-http/src/lib.rs
- crates/jj-store-http/src/local_browser.rs
- crates/jj-store-http/src/local_browser/working_tree.rs
- crates/jj-store-http/src/local_browser/working_tree/tests.rs
- crates/jj-store-service/src/working_tree/contracts.rs
- crates/jj-store-service/src/working_tree/mod.rs
- crates/jj-store-service/src/working_tree/projection.rs
- docs/implementation-plan.md
- docs/maintenance/coverage.json
- docs/site/guides/browsing.md
- docs/site/reference/api/installation.md
- docs/site/reference/api/types.md
- docs/site/reference/transport.md
- docs/site/technical/local.md
- schemas/openapi.json
- spec/src/api.md
- spec/src/architecture.md
- spec/src/cli.md
- spec/src/compatibility.md
- spec/src/documentation.md
- spec/src/local-jj.md
- spec/src/milestones.md
- spec/src/mutations.md
- spec/src/security.md
- spec/src/spec-governance.md
- spec/src/ui.md
- spec/src/verification.md
- web/src/lib/api/generated.ts
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

No registered working-tree/observe endpoint existed. Exact HTTP contract regression initially fails on the missing route.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Strict version/generation/operation request with no client path; composition resolves and revalidates registered folder. Complete bounded changes retain raw-content/mode metadata and exact WC/operation/view plus physical/metadata fingerprints; no policy/contents/command authority. Shared HTTP state and binding helper impact requires complete HTTP library and existing registered-response capture lanes.
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-http/src/contract/tests.rs#working_tree_observation_contract_is_registered_pinned_and_path_free
- crates/jj-store-http/src/contract/tests.rs#contract_inventory_covers_security_methods_media_and_task_bodies
- crates/jj-store-http/src/local_browser/working_tree/tests.rs#observation_route_rejects_client_paths_stale_bindings_and_foreign_origins
- crates/jj-store-http/src/local_browser/working_tree/tests.rs#observation_route_requires_administrative_localtrusted_admission
- crates/jj-store-http/src/local_browser/working_tree/tests.rs#registered_observation_route_reads_real_checkout_without_publication_or_content_leaks
- crates/jj-store-service/src/working_tree/contracts.rs#transport_summary_rejects_oversized_complete_change_sets
- crates/jj-store-cli/tests/local_browser.rs#filesystem_runtime_registers_and_reads_without_native_storage
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M21-010 details 2](details-02.md)
- [E-M21-010 details 3](details-03.md)
- [E-M21-010 details 4](details-04.md)
