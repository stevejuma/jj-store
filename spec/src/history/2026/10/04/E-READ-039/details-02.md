---
schema: 1
id: E-READ-039
---

# E-READ-039 details 2

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/projects-repositories.test.ts -t 'card review activity' (before)
```

**Notes**

2fail: native/local cards both invoke full review presentation. review-activity-before-039.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/projects-repositories.test.ts -t 'card review activity' (after)
```

**Notes**

2pass/0.84s. Native sampled review counts/time and local existing code/task date policy; no thread presentation. review-activity-after-039.log.
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
cargo check -p jj-store-http -p jj-store-service --locked
```

**Notes**

Affected initial packages pass/48.72s. review-activity-check-initial-039.log.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 7
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-service --lib --locked collaboration::review_activity:: -- --nocapture (initial)
```

**Notes**

Test compilation: ambiguous glob Error import (six diagnostics) and missing Corrupt constructor argument (one). Corrected explicit service Error import and typed WAL error. review-activity-units-039.log.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-service --lib --test review_threads --test local_review --locked review_activity -- --nocapture
```

**Notes**

6pass: four units and two provider regressions. Selected-only object reads, maximum across paged/merged nonmonotonic event times, absent anchors/comments/bodies, empty selection, corrupt key/count/missing events, native no-publication and local historical selection/bounds. review-activity-service-039.log.
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
cargo test -p jj-store-service --test review_threads --test local_review --locked -- --nocapture
```

**Notes**

12pass (7local/5native). Existing review admission/selected-closure/foreign anchors/receipt replay/GC/detail body limit/SSE and pin consumers pass. review-activity-review-consumers-039.log.
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
JJ_STORE_REVIEW_CONTRACT_CAPTURE=$PWD/.local/bounded-reads/performance/review-activity-native-capture-039.json cargo test -p jj-store-cli --test native_lifecycle --locked local_trusted_http_pages_root_pinned_review_threads_and_resolves_detail -- --nocapture
```

**Notes**

1pass/2.84s. Actual native first-window/complete summaries match pins/times; current read permission admitted, limit0/257 return inherited413, unknown cursor returns400. Existing detail/write/receipt/restart fixture retained. review-activity-native-http-039.log.
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
JJ_STORE_LOCAL_BROWSER_CONTRACT_CAPTURE=$PWD/.local/bounded-reads/performance/review-activity-local-capture-039.json cargo test -p jj-store-cli --test local_browser --locked filesystem_runtime_registers_and_reads_without_native_storage -- --nocapture
```

**Notes**

1pass/29.88s. Actual local finite/complete/historical-empty summaries; explicit pins, missing/unknown fields400 and inherited capacity413. Isolated fixture only; complete cold production-browser/RustFS qualification remains deferred. review-activity-local-http-039.log.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
node scripts/contracts.mjs --write; cargo test -p jj-store-http --lib --locked contract::review_tests:: -- --nocapture; node --input-type=module with scripts/contracts-validation.mjs responseValidator over only captured /reviews/activity responses
```

**Notes**

Generated OpenAPI/TypeScript passes schema generation;3HTTP contract units pass/1.18s and5actual native/local activity responses validate. Inventory117/42local. review-activity-generate-039.log/review-activity-contract-units-039.log/review-activity-responses-039.log. Full contract inventory intentionally deferred.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/projects-repositories.test.ts web/tests/vite/projects-repository-visibility.test.ts web/tests/vite/projects-directories.test.ts web/tests/vite/projects-agents.test.ts web/tests/vite/local-browser.test.ts (initial)
```

**Notes**

Three fixture activity calls used an out-of-scope transport variable; one cancellation test expected signal reference identity although LocalClient composes signals. Corrected fixture lookup and prove forwarded signal abortion. review-activity-ui-consumers-039.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/projects-repositories.test.ts web/tests/vite/projects-repository-visibility.test.ts web/tests/vite/projects-directories.test.ts web/tests/vite/projects-agents.test.ts web/tests/vite/local-browser.test.ts (corrected)
```

**Notes**

44pass/1fail: error-message regex used code spelling; ApiError message is human-readable. Assert exact code snapshot_mismatch instead. review-activity-ui-consumers-corrected-039.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/projects-repositories.test.ts web/tests/vite/projects-repository-visibility.test.ts web/tests/vite/projects-directories.test.ts web/tests/vite/projects-agents.test.ts web/tests/vite/local-browser.test.ts (final); pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/local-browser.test.ts (after extraction cleanup)
```

**Notes**

45unique UI tests pass/7.03s;19local direct consumers also pass/5.09s after final extraction cleanup. Current independent pins, foreign repo/epoch/metadata/profile rejection, cancellation, partial groups/counts and existing local reviews retain behavior. review-activity-ui-consumers-final-039.log/review-activity-local-ui-final-039.log.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/ProjectsHub.stories.svelte catalog/components/store-work/ProjectsDirectory.stories.svelte catalog/components/store-work/ProjectsRepositoryActions.stories.svelte
```

**Notes**

20pass/55.96s directly affected card/hub/directory/action interaction and accessibility stories in a fresh test process. review-activity-stories-039.log. Visual baselines and complete production cold browser acceptance stay deferred.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:architecture (initial and first extraction)
```

**Notes**

Local store facade exceeds300counted lines (309 then302). Extracted related activity/page reads to owned http-local-review-reader.ts and removed redundant detail validator method without weakening checks. review-activity-architecture-039.log/review-activity-architecture-final-039.log.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
cargo check -p jj-store-service -p jj-store-http -p jj-store-cli --locked; pnpm web:check (initial, final and extraction cleanup); pnpm catalog:check (initial and final); pnpm check:boundaries; pnpm check:architecture (corrected); pnpm spec:validate
```

**Notes**

Affected Rust packages/direct CLI consumers pass; web/catalog0errors/0warnings;190catalog modules;17package boundaries;973architecture modules;5validators/23chapters/172requirements. review-activity-packages-final-039.log/review-activity-web-check-039.log/review-activity-web-check-final-039.log/review-activity-web-check-extracted-039.log/review-activity-catalog-check-039.log/review-activity-catalog-check-final-039.log/review-activity-boundaries-039.log/review-activity-architecture-corrected-039.log/review-activity-spec-039.log.
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first (initial)
```

**Notes**

Protected presentation helper also maps to ui.md, which was missing from the canonical update. Added its activity/count contract. review-activity-spec-first-039.log.
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --write spec/src/milestones.md spec/src/ui.md docs/implementation-plan.md spec/src/api.md; pnpm check:progress; pnpm spec:first (corrected); pnpm spec:validate (final); pnpm check:ui-ownership; targeted Prettier check; cargo fmt -p jj-store-service -p jj-store-http -p jj-store-cli -- --check; jj --no-pager diff --git plus added-line whitespace validation
```

**Notes**

138-task bookkeeping;34protected files/9canonical chapters;5validators/23chapters/172requirements; UI ownership; canonical/UI/Rust formatting and whitespace pass. review-activity-ledger-format-039.log/review-activity-progress-039.log/review-activity-ui-spec-format-039.log/review-activity-doc-format-final-039.log/review-activity-spec-first-corrected-039.log/review-activity-spec-final-039.log/review-activity-ui-ownership-039.log/review-activity-doc-check-039.log/review-activity-rust-format-final-039.log/review-activity-final-039.patch. 21unique focused Rust tests,45UI units and20interaction/accessibility stories; no full-suite or complete qualification claim.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Lightweight code metadata still fetches a graph window; root-bound review
  activity reuse remains delivery work. Current review summary counts/times
  intentionally describe only the requested catalog window.
- Prepared local operation prefix/comparison, indexed filtered references,
  indexed deep operation/predecessor/copy/search, paged native RPC, local
  operation-difference/change-group paging and remaining local application
  search/index delivery remain open.
- Full combined integration/contracts, Linux/native/near-capacity fixtures,
  measured RSS/release latency, complete cold production browser,
  fault/revocation/restart/publication/real-RustFS, registry/release and
  Help/source-review refresh remain deferred. No milestone promotion, main/Linux
  source or original dev-service change; visual baselines remain unapproved.
- Inherited oversized __eh_frame linker warning remains. E-READ-037 inherited
  strict-lint gaps remain deferred; no full Rust workspace or root aggregate
  lane was run.
<!-- /evidence-field -->

