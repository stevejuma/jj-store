---
schema: 1
id: E-READ-090
---

# E-READ-090 details 3

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
pnpm exec prettier --check web/src/lib/controller/http-local-query.ts web/src/lib/controller/http-query.ts web/src/lib/controller/production-file-pages.ts web/src/lib/controller/production-file-metadata.ts web/src/lib/controller/production-file-reader.svelte.ts web/tests/vite/directory-response-cache.test.ts web/tests/vite/directory-query-cache.test.ts web/tests/browser/directory-response-cache.spec.ts spec/src/ui.md spec/src/api.md spec/src/milestones.md
```

**Notes**

Final changed-surface check; no aggregate suite.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
node .local/directory-cache/linux-navigation.mjs
```

**Notes**

Initial live probe timed out waiting for quick attribution after 60s. A follow-up traced the existing serial enrichment queue: headers/landing documents precede quick rows; corrected scheduling.
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
node .local/directory-cache/inspect-live.mjs; node .local/directory-cache/inspect-metadata.mjs
```

**Notes**

Diagnostic fresh-browser reads: filenames usable, no page errors; commit_summary then file_at started before row attribution. Logs retained without credentials.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/quick-file-metadata.test.ts
```

**Notes**

priority-before.log: pending optional header prevents quick metadata. Exact regression fails before reordering.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/quick-file-metadata.test.ts web/tests/vite/directory-query-cache.test.ts web/tests/vite/directory-response-cache.test.ts web/tests/vite/production-file-reader.test.ts web/tests/vite/http-query.test.ts web/tests/vite/local-browser.test.ts web/tests/vite/production-automatic-refresh.test.ts
```

**Notes**

priority-final.log: one prior test assumed header starts synchronously, plus unhandled rejection in new optional-read fixture when canceled before starting. Wait for header start while preserving cancellation assertions; handle unused fixture promise.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/quick-file-metadata.test.ts web/tests/vite/directory-query-cache.test.ts web/tests/vite/directory-response-cache.test.ts web/tests/vite/production-file-reader.test.ts web/tests/vite/http-query.test.ts web/tests/vite/local-browser.test.ts web/tests/vite/production-automatic-refresh.test.ts
```

**Notes**

priority-final-2.log: 54 tests passed across seven directly affected files, no unhandled errors.
<!-- /run -->

<!-- run -->
### Run 7

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

web-priority.log: zero errors/warnings after scheduling change.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec playwright test --config .local/directory-cache/browser.config.ts directory-response-cache.spec.ts
```

**Notes**

browser-priority.log: one cold browser test passes after scheduling change (27.9s with concurrent checks).
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first
```

**Notes**

Checking only the scheduling amendment atop the already committed slice lacked its mapped API chapter. Squashed into enclosing slice, which includes api.md; merged-change gate passes.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first
```

**Notes**

Final merged xqkplmwn: ten protected files and three canonical chapters.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
node .local/directory-cache/linux-navigation.mjs
```

**Notes**

linux-navigation-final.log: probe waited for network completion but navigated before root quick metadata was published, correctly resuming canceled quick work. Probe now waits for a rendered attribution row before claiming a completed read.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
node .local/directory-cache/linux-navigation.mjs
```

**Notes**

linux-navigation-settled.log/linux-navigation.json: real Linux root and arch at the recorded operation/commit. First arch names and rendering 1399ms; root-to-arch revisit round trip 256ms. Exactly one successful names and quick metadata request per directory. Revisit sends zero names or quick requests; two unfinished history windows resume. Warm development app/OS caches; not release or OS-cold qualification.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pass
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

[]
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

UI and admission slices landed in main and the running preview, with the temporary workspace retained. Conditional Git pack-reader reuse remains a separate backend slice: isolated names reads still about 1.2s, live arch first visit 1.4s. Completed server path-result caching and a soft attribution allowance are not added in this measured delivery. No broader indexing/cache redesign or release qualification claimed.
<!-- /evidence-field -->

<!-- evidence-field:out_of_scope markdown -->
## Out of scope

Rust, full UI/catalog, released built-bundle/CSP, OS-cold Linux, RustFS and visual baselines are out of scope for this UI/controller slice. Exact cold Vite lifecycle smoke is included. Optional web-perf DevTools MCP was unavailable; used native process sampling and direct HTTP timings, not a browser performance trace.
<!-- /evidence-field -->

