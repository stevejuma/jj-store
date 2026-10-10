---
schema: 1
id: E-TEST-REPAIR-001
---

# E-TEST-REPAIR-001 details 3

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: pass
failures: 0
tests: 84
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec tsx --test --test-concurrency=4 web/tests/agent-directory.test.ts web/tests/agent-execution.test.ts web/tests/agent-harness-catalog.test.ts web/tests/agent-inbox-filters.test.ts web/tests/agent-inbox-groups.test.ts web/tests/agent-inbox-layout.test.ts web/tests/agent-inbox-preferences.test.ts web/tests/agent-mentions.test.ts web/tests/agent-message-content.test.ts web/tests/agent-revisions.test.ts web/tests/agents-route.test.ts web/tests/client.test.ts web/tests/collaboration-presentation.test.ts web/tests/compute-model-catalog.test.ts web/tests/conversation-merge.test.ts web/tests/conversation-ordering.test.ts web/tests/conversation-source-map.test.ts web/tests/help-session.test.ts web/tests/layout-storage.test.ts web/tests/lazy-view.test.ts web/tests/managed-agent.test.ts web/tests/merge-editor-settings.test.ts web/tests/native-entity-links.test.ts web/tests/native-repository-manager.test.ts
```

**Notes**

Batch 1: 84 passed, exit 0.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: pass
failures: 0
tests: 65
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec tsx --test --test-concurrency=4 web/tests/navigation-storage.test.ts web/tests/production-inspection-pages.test.ts web/tests/production-store-conflicts.test.ts web/tests/production-store-loaders.test.ts web/tests/production-store-mutations.test.ts web/tests/production-store-search.test.ts web/tests/repository-conversion-manager.test.ts web/tests/repository-diff.test.ts web/tests/repository-log-author.test.ts web/tests/repository-source-tokenizer.test.ts web/tests/review-comment-anchor.test.ts web/tests/store-blame.test.ts web/tests/store-conflict-model.test.ts web/tests/store-mutation-model.test.ts web/tests/store-route-codec.test.ts web/tests/work-presentation.test.ts
```

**Notes**

Batch 2: 65 passed, exit 0.
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
pnpm catalog:test
```

**Notes**

Full catalog-unit project: 156 files; 935 passing and one existing skipped test. Escalated to verify all known catalog unit failures and direct controller consumers.
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
node .local/test-repairs/web-unit-batches.mjs
```

**Notes**

Ignored orchestration script runs the complete root Node test selection in two batches of at most 24 files, capped at four subprocesses. Actual expanded pnpm commands are recorded separately.
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
pnpm check:ui-ownership
```

**Notes**

Owned presentation import boundary passes; no Community/catalog dependency introduced.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:search "JST-UI-015"
```

**Notes**

QMD native dependencies are unavailable for Node ABI147 in copied runtime; owning canonical pages inspected directly with rg. No runtime dependency installation or shared toolchain changes.
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
pnpm test:storybook catalog/components/store-stack-workspace/StoreStackWorkspaceProduction.stories.svelte
```

**Notes**

Final fresh process: complete production adapter family, 4 passed; includes cold graph-index recovery and dependent branch segments.
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
pnpm progress:history
```

**Notes**

Regenerated task histories, retained M24 entry and restored unrelated historical normalization edits.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
```

**Notes**

First documentation pass found new M24 navigation/history files absent from the evidence file list. Added fresh coverage; product checks unchanged.
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
pnpm check:progress
```

**Notes**

Fresh evidence covers all changed catalog/web/current progress files: 146 tasks, 736 evidence records.
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
pnpm spec:validate
```

**Notes**

Formatting added a blank line to the full M24 index, exceeding its 400-line cap by one. Moved another navigation entry into its existing second page.
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
pnpm spec:validate
```

**Notes**

Seven validators, 3498 chapters and 181 requirements; M24 remains within 400 physical lines.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:links
```

**Notes**

Fresh workspace has no generated HTML book. Build the documentation first, then rerun the anchor audit.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:build
```

**Notes**

Documentation book built. Existing warnings: unclosed execution-id placeholder in unchanged API chapter 41; large generated search index (50391459 bytes). No Rust/product build.
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
pnpm spec:links
```

**Notes**

After book generation: 2543 HTML files and 95860 local references have valid links and anchors.
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --check catalog/components/store-application/ManagedAgentIdentity.stories.svelte catalog/components/store-application/RepositoryReview.stories.svelte catalog/components/store-application/repository-directory-story-assertions.ts catalog/components/store-stack-workspace/StoreStackWorkspaceApplication.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceForms.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspacePresentation.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceSetup.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceSpacer.stories.svelte catalog/fixtures/StackWorkspaceProductionFixture.svelte catalog/fixtures/stack-workspace/production-http.ts spec/src/current/contracts/ui/13-shared-application.md spec/src/current/progress/M24/SUMMARY.index-index/002/index.md spec/src/current/progress/M24/SUMMARY.nav.md spec/src/current/progress/M24/history/M24/index.md spec/src/current/progress/M24/index.md spec/src/current/progress/M24/test-repairs-review.md spec/src/history/2026/10/10/E-TEST-REPAIR-001/SUMMARY.nav.md spec/src/history/2026/10/10/E-TEST-REPAIR-001/details-02.md spec/src/history/2026/10/10/E-TEST-REPAIR-001/details-03.md spec/src/history/2026/10/10/E-TEST-REPAIR-001/details-04.md spec/src/history/2026/10/10/E-TEST-REPAIR-001/index.md spec/src/history/2026/10/10/SUMMARY.nav.md spec/src/history/2026/10/10/index.md web/src/lib/controller/production-store-mutations.ts web/src/lib/controller/production-store-search-controller.ts web/src/lib/conversations/ordered-activity.ts web/src/lib/ui/community-projects/repository-directory.css web/tests/conversation-ordering.test.ts web/tests/vite/conversation-create.test.ts web/tests/vite/local-browser-merge.test.ts
```

**Notes**

All matched authored files pass; immutable evidence history is intentionally excluded by repository formatter policy and checked structurally.
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
pnpm spec:first --base 7601e3cf --head vslkntzy
```

**Notes**

Spec-first passes for 17 protected files and one canonical chapter; qualified unchanged-contract evidence retains API and ownership boundaries.
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

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64; Node 26.9.0; pnpm 10.32.1; JJ 0.45.1; Svelte 5.57.1; Storybook 10.5.7; Vitest 4.1.11. Existing local packed Design Core 0.1.2 validation runtime; registry release acceptance remains separate.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Unchanged public API/schema, request IDs, persisted Thread metadata, UTF-16 capture strings, application ownership and visual baselines.
<!-- /evidence-field -->

