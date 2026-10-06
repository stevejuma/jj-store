---
schema: 1
id: E-M21-001
---

# E-M21-001

<!-- evidence-field:id markdown -->
## Id

E-M21-001
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

enclosing Jujutsu change uvzrpntl
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

shared-read-cancellation-and-target-cache-repair
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

JJ 0.45.1; pnpm 10.32.1; Vitest 4.1.11; source parent d6442442
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No wire/storage/layout format changes. Session cache lease and exact reaction argument matching corrected; manual source coverage refreshed.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- web/src/lib/controller/read-query.ts
- web/src/lib/controller/http-threads-data-source.ts
- web/tests/vite/read-query.test.ts
- web/tests/browser/history-query-retry.spec.ts
- spec/src/api.md
- spec/src/ui.md
- spec/src/documentation.md
- docs/site/technical/architecture.md
- docs/maintenance/coverage.json
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Two existing native/local immediate retry regressions timed out. Wider consumer run exposed one exact reaction target invalidation regression (query argument was mistaken for trailing epoch).
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Synchronously release canceled observers before identical retries, retain shared transports for active callers, and match reaction arguments independently of epoch keys. Shared lifecycle/cache code affects native/local history, Threads and collaboration; this concrete impact requires the complete catalog unit lane, affected history/Thread stories and cold served-app acceptance.
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/vite/read-query.test.ts#an immediate same-key retry cannot inherit
  released transport
- web/tests/vite/read-query.test.ts#one observer cancellation cannot abort
  another consumer's shared read
- web/tests/vite/thread-query-cache.test.ts#Thread queries share session cache
  and target invalidation retains unrelated data
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M21-001 details 2](details-02.md)
- [E-M21-001 details 3](details-03.md)
