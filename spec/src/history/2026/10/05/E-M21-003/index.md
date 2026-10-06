---
schema: 1
id: E-M21-003
---

# E-M21-003

<!-- evidence-field:id markdown -->
## Id

E-M21-003
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

enclosing Jujutsu change pxztpulz
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

lazy-operation-pinned-stack-read-projection
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

JJ 0.45.1; pnpm 10.32.1; Vitest 4.1.11; read provider parent 6c167204
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Additive frontend presentation fields preserve provider graph annotations and unloaded/unavailable file metadata. Optional cancellation arguments on existing diff reads; no wire/storage/checkout/layout format changes.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- docs/maintenance/coverage.json
- docs/site/technical/architecture.md
- spec/src/api.md
- spec/src/documentation.md
- spec/src/ui.md
- web/src/lib/controller/http-local-code-data-source.ts
- web/src/lib/controller/http-store-data-source.ts
- web/src/lib/controller/native-data-source.ts
- web/src/lib/controller/stack-workspace/presentation.ts
- web/src/lib/controller/stack-workspace/read-presentation.ts
- web/src/lib/controller/stack-workspace/reads.ts
- web/src/lib/controller/stack-workspace/types.ts
- web/tests/vite/stack-diff-transport.test.ts
- web/tests/vite/stack-workspace-reads.test.ts
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

The new exact regression failed to load the absent read owner. First implemented run had four passes/two assertion errors: Unicode file name accidentally matched a content exclusion, and a human error message was compared to its machine code. Assertions now target context lines and structured error codes.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Introduce one explicit operation/profile/device-generation read owner over existing transports. Page history without eager files, compare exact trees for net files, load only selected patches, preserve graph/ordered conflict/mode/rename metadata, and cancel without closing shared source. The new class is a read foundation, not the production stack adapter/controller injection. Tests include native/local diff cancellation and both direct UI consumers.
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/vite/stack-workspace-reads.test.ts#stack reads preserve real graph
  metadata and load no file details with history
- web/tests/vite/stack-workspace-reads.test.ts#net files use exact boundary
  trees and preserve unloaded state instead of unioning commit hunks
- web/tests/vite/stack-workspace-reads.test.ts#diff presentation keeps byte
  offsets out of line numbers and skips matching-only sections
- web/tests/vite/stack-workspace-reads.test.ts#unavailable diffs retain ordered
  conflict terms and rename sources without fabricated text
- web/tests/vite/stack-workspace-reads.test.ts#reads reject foreign snapshots,
  wrong tree ranges and nonadvancing pages
- web/tests/vite/stack-workspace-reads.test.ts#canceling or closing rejects late
  reads without closing the repository source
- web/tests/vite/stack-workspace-reads.test.ts#stack reads validate profile,
  page limits and advertised history capability before transport
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M21-003 details 2](details-02.md)
- [E-M21-003 details 3](details-03.md)
