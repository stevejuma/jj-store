---
schema: 1
id: E-FRESH-002
---

# E-FRESH-002

<!-- evidence-field:id markdown -->
## Id

E-FRESH-002
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Authorization-session query ownership, immutable cache identity and cancellation leases
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Node26.9.0; pnpm10.32.1; Vitest4.1.11; TanStack Svelte Query6.3.0
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No wire or persisted format change. Native exact content keys add epoch; local keys retain registration generation and companion pin. Live family invalidation is separate from snapshot capture.
<!-- /evidence-field -->

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit --maxWorkers=2 web/tests/vite/read-query.test.ts web/tests/vite/repository-sources.test.ts web/tests/vite/production-store-data-source.test.ts web/tests/vite/local-browser.test.ts web/tests/vite/production-file-reader.test.ts web/tests/vite/production-reference-reader.test.ts web/tests/vite/production-store-bootstrap.test.ts web/tests/vite/repository-thread-navigation.test.ts web/tests/vite/production-threads.test.ts
```

**Notes**

74 tests/9 files after shared graph extraction and epoch keys. Initial duplicate-read and fresh-local-capture failures fixed without weakening supersession.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit --maxWorkers=2 web/tests/vite/http-query.test.ts web/tests/vite/repository-sources.test.ts
```

**Notes**

3 tests/2 files. Native immutable key separates epochs; prepared pages remain transient.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:check
```

**Notes**

0 errors/0 warnings after final helpers.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:architecture; pnpm check:ui-ownership
```

**Notes**

Initial oversized orchestrators extracted; final973 modules pass. All production boundaries owned.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/vite/read-query.test.ts#one observer cancellation cannot abort
  another consumer's shared read
- web/tests/vite/read-query.test.ts#the final observer release aborts obsolete
  transport
- web/tests/vite/http-query.test.ts#immutable content shares exact epoch while
  transient prepared pages expire
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Application event hub/producers, browser healthy-poll migration, generated
  contracts and cold idle/recovery filesystem/real-storage qualification remain.
  Full plan not complete; root aggregate and visual baselines out of scope.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Add authenticated scoped application feed with shared monitor/replay ownership and independent SSE delivery lifetime.
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M20
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-04
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

uvknxprmzuyl
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- spec/src/api.md
- spec/src/ui.md
- web/src/lib/controller/http-collaboration-data-source.ts
- web/src/lib/controller/http-file-reads.ts
- web/src/lib/controller/http-graph-reads.ts
- web/src/lib/controller/http-local-query.ts
- web/src/lib/controller/http-query.ts
- web/src/lib/controller/http-reference-reads.ts
- web/src/lib/controller/http-store-data-source.ts
- web/src/lib/controller/http-threads-data-source.ts
- web/src/lib/controller/http-work-data-source.ts
- web/src/lib/controller/production-projects-integration.ts
- web/src/lib/controller/production-repository-connect.ts
- web/src/lib/controller/production-store-controller.svelte.ts
- web/src/lib/controller/production-store-lifecycle.svelte.ts
- web/src/lib/controller/read-query.ts
- web/src/lib/controller/repository-source.ts
- web/src/lib/controller/repository-sources.ts
- web/src/lib/controller/repository-thread-navigation.svelte.ts
- web/src/lib/controller/store-query-keys.ts
- web/tests/vite/http-query.test.ts
- web/tests/vite/production-store-data-source.test.ts
- web/tests/vite/read-query.test.ts
- web/tests/vite/repository-sources.test.ts
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


