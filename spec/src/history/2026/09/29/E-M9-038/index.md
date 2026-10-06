---
schema: 1
id: E-M9-038
---

# E-M9-038

<!-- evidence-field:id markdown -->
## Id

E-M9-038
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M9
- M9.5
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-29
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

inline-diff-and-tool-review-comments
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-application/StoreCollaboration.stories.svelte
- catalog/controller/in-memory-inline-review.ts
- catalog/controller/in-memory-store-collaboration.svelte.ts
- catalog/fixtures/collaboration-review.ts
- catalog/fixtures/collaboration.ts
- spec/src/api.md
- spec/src/collaboration.md
- spec/src/milestones.md
- spec/src/spec-governance.md
- spec/src/ui.md
- web/src/lib/collaboration/presentation.ts
- web/src/lib/controller/contracts.ts
- web/src/lib/controller/production-inline-review-actions.ts
- web/src/lib/controller/production-inspection-pages.ts
- web/src/lib/controller/production-review-actions.ts
- web/src/lib/controller/production-route-restorer.ts
- web/src/lib/controller/production-store-actions.ts
- web/src/lib/controller/production-store-collaboration.svelte.ts
- web/src/lib/controller/production-store-loaders.ts
- web/src/lib/controller/review-anchor-patch.ts
- web/src/lib/controller/review-comment-anchor.ts
- web/src/lib/controller/store-route-codec.ts
- web/src/lib/ui/community-commit-diff/CommunityCommitDiff.svelte
- web/src/lib/ui/community-projects/CommunityProjectsRepositoryLogDiff.svelte
- web/src/lib/ui/store-collaboration/StoreAgentContext.svelte
- web/src/lib/ui/store-collaboration/StoreAgentToolReview.svelte
- web/src/lib/ui/store-collaboration/StoreAgentTranscript.svelte
- web/src/lib/ui/store-collaboration/StoreCommentableCommitDiff.svelte
- web/src/lib/ui/store-collaboration/StoreReviewConversation.svelte
- web/tests/collaboration-presentation.test.ts
- web/tests/production-inspection-pages.test.ts
- web/tests/production-store-loaders.test.ts
- web/tests/review-comment-anchor.test.ts
- web/tests/store-route-codec.test.ts
- web/tests/vite/production-store-collaboration.test.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/review-comment-anchor.test.ts#after-line anchors preserve UTF-8
  offsets, digest and operation pair
- web/tests/review-comment-anchor.test.ts#removed-line anchors resolve the
  before file and operation
- web/tests/review-comment-anchor.test.ts#line anchoring fails closed for stale
  text, conflicts and final blank lines
- web/tests/vite/production-store-collaboration.test.ts#diff line comments
  create an exact code-text thread and select its receipt
- web/tests/vite/production-store-collaboration.test.ts#tool comments create an
  exact agent-artifact thread
- web/tests/store-route-codec.test.ts#collaboration routes preserve stable
  session and thread selections
<!-- /evidence-field -->

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
source audit of Community transcript comments and Design Core FileDiff line extension points
```

**Notes**

Before implementation, jj-store dropped Design Core line accessories and annotations in its single-file wrapper, exposed no ReviewStore create-thread controller action, and rendered agent tool calls without a reply surface.
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
pnpm web:test
```

**Notes**

All 53 web unit tests pass, including exact after/before line anchors, UTF-8 byte offsets and digests, operation pairs, fail-closed stale/conflict cases, path placement and Agents route retention.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/production-store-collaboration.test.ts web/tests/vite/store-route-history.test.ts catalog/controller/store-controller.test.ts
```

**Notes**

All 23 focused controller tests pass. Review commands create exact code-text and tool-owned agent-artifact anchors, select the committed receipt thread and preserve existing durable reply behavior.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreCollaboration.stories.svelte
```

**Notes**

All eight focused Storybook interaction and automatic accessibility cases pass. Hover tool actions open an inline reply composer, diff-row actions open an inline comment composer, and both submitted discussions render beside their source.
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
pnpm web:check; pnpm catalog:check; pnpm spec:first
```

**Notes**

Both Svelte projects report zero diagnostics, catalog boundaries pass and the changed protected files map to all five owning canonical chapters after the new production helpers were split below the architecture limit.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check
```

**Notes**

The aggregate specification, package-boundary, architecture, progress, Rust formatting and Clippy, contract, web and catalog checks pass.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test
```

**Notes**

The complete governance, embedded and managed RustFS, LocalTrusted, contract, 53-case web, 60-case catalog, 85-case Storybook, cold catalog browser and Rust workspace suites pass.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build
```

**Notes**

The complete Storybook, production web, embedded RustFS and Rust workspace build passes; the known Lightning CSS :global(svg) and large-chunk warnings remain informational.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 20
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:check
```

**Notes**

Specification validation and cold catalog browser acceptance pass, then the inherited 20 captured production-browser cases time out while waiting for removed Repository ID and Repository token controls; this slice introduces no new failure category.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm format:check
```

**Notes**

The first final formatting pass found only spec/src/ui.md; the repository formatter corrected that owned file before the final rerun.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm format:check
```

**Notes**

Rust formatting and repository-wide Prettier checks pass after formatting the owned UI chapter.
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
pnpm check:progress
```

**Notes**

The updated milestone evidence ledger is structurally valid across all 61 tasks.
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
pnpm spec:first
```

**Notes**

The final spec-first gate maps all 30 changed protected files to five canonical chapters.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Visual baselines remain unapproved.
- The inherited captured production-browser fixtures still require migration
  from removed Repository ID and Repository token controls before pnpm
  spec:check can pass.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Node.js 24.10.0; pnpm 10.32.1; Svelte 5.57.1; Design Core ^0.2.0; Community ^0.1.0.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No native object, manifest, WAL, receipt, AgentStore or ReviewStore encoding changes. Browser comment creation uses the existing ReviewCommandV1 create_thread and append_reply contract. Code-text anchors retain exact native file identity, UTF-8 byte selection and digest plus explicit before/after operation and commit context. V1 tool anchors use the stable tool-call ID as the single coalesced attempt identity when capture has no finer attempt record.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


