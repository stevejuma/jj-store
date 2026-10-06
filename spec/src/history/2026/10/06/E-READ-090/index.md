---
schema: 1
id: E-READ-090
---

# E-READ-090

<!-- evidence-field:id markdown -->
## Id

E-READ-090
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-06
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M4.11
- M13.4
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-005
- JST-UI-011
- JST-GOV-007
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Retain file directory responses and metadata progress within the active read scope
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change xqkplmwn
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Directory selections always reread names and overwrite enriched pages. Concurrent clicks duplicate names requests. Returning after canceled history repeats quick attribution; loaded continuation windows are discarded. Corrected exact regression: four failed, three passed. Local transport staleTime=0 reread identical names; native history used a 30-second cache for pending batches.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Reuse the existing page map, with source/operation/commit/read-version invalidation, in-flight names coalescing and per-window quick/history/completed attribution progress. Keep successful local names responses in the existing session QueryClient for 30 seconds; generation and exact request keys isolate them. Native history is transient. Preserve source close, cancellation, refresh clearing, pagination, skeletons and shared feedback. No source persistence, backend or public-contract change. Prioritize quick row metadata ahead of optional commit headers and landing documents; these no longer hold up Description/Updated.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64; Vitest 4.1.11, Playwright 1.63.0, Svelte 5.57.1; pinned repository dependencies.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Internal optional StoredPathPage metadata windows; HTTP DTOs, cursor formats, source JJ/Git metadata, storage and credentials unchanged.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- spec/src/ui.md
- web/src/lib/controller/http-local-query.ts
- web/src/lib/controller/http-query.ts
- web/src/lib/controller/production-file-metadata.ts
- web/src/lib/controller/production-file-pages.ts
- web/src/lib/controller/production-file-reader.svelte.ts
- web/tests/browser/directory-response-cache.spec.ts
- web/tests/vite/directory-query-cache.test.ts
- web/tests/vite/directory-response-cache.test.ts
- spec/src/milestones.md
- spec/src/api.md
- web/tests/vite/quick-file-metadata.test.ts
- web/tests/vite/production-file-reader.test.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/vite/directory-response-cache.test.ts#revisiting a directory retains
  names and completed attribution without another request
- web/tests/vite/directory-query-cache.test.ts#local directory reads reuse
  successful responses within their generation and expire cursors
- web/tests/browser/directory-response-cache.spec.ts#directory navigation shows
  loading once and reuses completed rows and metadata
- web/tests/vite/quick-file-metadata.test.ts#quick path metadata does not wait
  for the optional commit header or landing documents
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-READ-090 details 2](details-02.md)
- [E-READ-090 details 3](details-03.md)
