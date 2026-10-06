---
schema: 1
id: E-M21-011
---

# E-M21-011

<!-- evidence-field:id markdown -->
## Id

E-M21-011
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

qtqkzlsy
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Owned current physical-read client and independent Stacks observation lifecycle
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Node 22+; pnpm 10.32.1; macOS ARM64; existing explicitly built development JJ 0.45.1/server binaries. No installed executable/service changes.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Consumes existing version-1 ObserveWorkingTree/ObservedWorkingTree records with exact lossless numbers. No public schema, native/WAL/companion/private persisted format change. Current physical evidence remains separate from historical selections and combination WIP.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/fixtures/stack-workspace/production-http.ts
- docs/implementation-plan.md
- docs/maintenance/coverage.json
- docs/site/guides/browsing.md
- scripts/stacks-browser-acceptance.mjs
- spec/src/api.md
- spec/src/architecture.md
- spec/src/documentation.md
- spec/src/local-jj.md
- spec/src/spec-governance.md
- spec/src/ui.md
- spec/src/verification.md
- web/src/lib/controller/http-local-store-data-source.ts
- web/src/lib/controller/http-working-tree.ts
- web/src/lib/controller/native-data-source.ts
- web/src/lib/controller/stack-workspace/controller.svelte.ts
- web/src/lib/controller/stack-workspace/current-working-tree.ts
- web/src/lib/controller/stack-workspace/physical-observation.svelte.ts
- web/src/lib/controller/stack-workspace/production-adapter.ts
- web/src/lib/controller/stack-workspace/types.ts
- web/src/lib/controller/working-tree-read.ts
- web/src/lib/controller/working-tree-types.ts
- web/src/lib/controller/working-tree-validation.ts
- web/tests/real/stacks-browser.spec.ts
- web/tests/vite/production-stacks-application.test.ts
- web/tests/vite/stack-working-tree.test.ts
- web/tests/vite/working-tree-http.test.ts
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

No owned physical-tree transport/client or separately canceled production Stacks observation existed; the exact transport test cannot import its missing module.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Uncached registered read client validates exact pins, identity widths, complete ordered paths/terms, bounded native integers/bytes and coherent before/after file states. Production adapter brackets observation with fresh verified single-current-head snapshots; controller owns supersession/cancellation independently of history. Shared controller lifecycle is covered by all directly affected Stacks unit/story consumers and one cold built real dirty-folder flow.
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/vite/working-tree-http.test.ts#physical observation preserves exact
  integers and sends only pinned registered inputs
- web/tests/vite/working-tree-http.test.ts#physical observation rejects
  mismatched snapshots, unsafe paths and incoherent complete summaries
- web/tests/vite/working-tree-http.test.ts#cancellation rejects a held
  response-body read before the late body completes
- web/tests/vite/stack-working-tree.test.ts#production adapter physical reads
  preserve a distinct historical operation selection
- web/tests/vite/stack-working-tree.test.ts#observation owner rejects superseded
  and post-disposal results and clears failed evidence
- web/tests/vite/production-stacks-application.test.ts#real bootstrap injects a
  verified Stacks controller and its route never reads the ordinary graph
- web/tests/real/stacks-browser.spec.ts#cold_registered_stacks_browse_real_ancestors_files_and_diffs_without_writes
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M21-011 details 2](details-02.md)
- [E-M21-011 details 3](details-03.md)
