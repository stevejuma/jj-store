---
schema: 1
id: E-READ-029
---

# E-READ-029 details 3

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
pnpm exec prettier --check docs/implementation-plan.md spec/src/api.md spec/src/architecture.md spec/src/projects.md spec/src/ui.md web/src/lib/controller/http-local-store-data-source.ts web/src/lib/controller/http-local-work-data-source.ts web/src/lib/controller/local-repository-directory.ts web/src/lib/controller/local-review-snapshot.ts web/src/lib/controller/native-data-source.ts web/src/lib/controller/projects-repositories.svelte.ts web/src/lib/ui/store-work/StoreProjects.svelte web/src/lib/ui/store-work/StoreProjectsRepositories.svelte web/src/lib/ui/store-work/StoreRepositoryDirectoryRow.svelte web/src/lib/ui/store-work/StoreWorkRepositories.svelte web/src/lib/ui/store-work/visible-repository.ts web/tests/vite/projects-repositories.test.ts web/tests/vite/projects-repository-visibility.test.ts web/tests/vite/visible-repository.test.ts
```

**Notes**

All final changed source/test/chapter/plan paths pass; ledger historical bytes excluded. format-029-complete.log.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Card reads still use existing bounded code/Work/Review pages, including
  mandatory prepared history for a cold local code card. Lightweight server
  summaries remain delivery work. Offscreen names remain navigable; incomplete
  metadata homes make activity partial. Large directory DOM/render/window and
  native/local capacity qualification remain deferred.
- Root-bound collaboration/search projections and keyset paging, native
  immutable cache/additional-operation construction and prepared local
  operation/prefix/comparison/deep/native RPC lookup remain implementation
  features.
- "Complete combined qualification is deferred: comprehensive
  integration/contracts, full cold production browser, latest Linux/native large
  fixtures, release/RSS/pressure/fault/real-RustFS, registry-only dependency
  qualification and Help review refresh. Original dev services, main, source
  Linux repo and visual baselines are untouched."
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Continue lightweight repository summaries and collaboration projections alongside remaining high-priority native/local indexed read delivery in the retained temporary workspace. Complete combined verification remains its separate later slice.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Deliver bounded viewport-owned card requests, immediate navigation, cancellation/refresh and partial metadata behavior with focused controller/transport/browser evidence; no full-plan or milestone completion.
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/vite/projects-repository-visibility.test.ts#directory discovery
  shows every repository without reading hidden card metadata
- web/tests/vite/projects-repository-visibility.test.ts#visible card reads stay
  at two while canceled transports settle and late output is discarded
- web/tests/vite/projects-repository-visibility.test.ts#refresh reads the shown
  window and defers hidden stale rows until they return
- web/tests/vite/projects-repository-visibility.test.ts#canceling a local card
  during a shared snapshot read starts no collaboration or graph work
- web/tests/vite/visible-repository.test.ts#viewport entry, identity changes,
  exit and unmount own exactly one directory lease
- web/tests/vite/projects-repositories.test.ts#directory retries retain an error
  episode until successful recovery, including queued reads
- web/tests/vite/local-browser.test.ts#local_review_reads_reject_foreign_companion_snapshots
- web/tests/vite/projects-hub.test.ts#refresh becomes busy before discovery and
  remains busy through reads
<!-- /evidence-field -->

