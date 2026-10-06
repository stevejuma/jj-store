---
schema: 1
id: E-M21-004
---

# E-M21-004

<!-- evidence-field:id markdown -->
## Id

E-M21-004
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

enclosing Jujutsu change vmyysrmu
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

lazy-stack-controller-and-capability-gated-presentation
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

JJ 0.45.1; pnpm 10.32.1; Vitest 4.1.11; parent read foundation 89239a29
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Optional opaque read keys, capability reasons and lazy adapter methods are presentation-only. Numeric revisions remain UI counters. No wire, storage, checkout or layout-format changes.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-stack-workspace/StoreStackWorkspaceLazy.stories.svelte
- catalog/fixtures/StackWorkspaceLazyApplicationFixture.svelte
- docs/maintenance/coverage.json
- docs/site/technical/architecture.md
- spec/src/api.md
- spec/src/documentation.md
- spec/src/ui.md
- web/src/lib/controller/stack-workspace/controller.svelte.ts
- web/src/lib/controller/stack-workspace/diff-selection.ts
- web/src/lib/controller/stack-workspace/eligibility.ts
- web/src/lib/controller/stack-workspace/file-drag.ts
- web/src/lib/controller/stack-workspace/lazy-details.svelte.ts
- web/src/lib/controller/stack-workspace/retained-diffs.ts
- web/src/lib/controller/stack-workspace/target-history.svelte.ts
- web/src/lib/controller/stack-workspace/types.ts
- web/src/lib/ui/store-stack-workspace/StoreStackBookmarkActions.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackBookmarkCard.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackCommitActions.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackCommitComposer.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackDiffHunk.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackLane.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackRelativeActions.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackSourceFiles.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackTargetHistory.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackUnstaged.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackWorkspace.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceActions.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceConflict.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceDiff.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackWorkspacePicker.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceSpacer.svelte
- web/tests/vite/stack-workspace-lazy-controller.test.ts
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Six exact new controller tests fail because lazy source reads, authoritative files and per-command eligibility do not exist. Initial implementation misplaces selection read hooks, producing five failures and one pass; corrected hooks restore the exact regression.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Use lazy authoritative file summaries and selected patches in branch, commit, target-history, Unstaged and independent adjacent diff views. Keep loading/retry/continuation source-scoped; cancel stale/closed reads and retain other origins. Gate mutation menus, composer actions and drags per advertised capability; preserve browsing and Copy. Render unavailable patches explicitly and accept provider hunk headers without duplicating file headers. Extract diff-selection/retention helpers within the 300-line production budget. No production adapter, command executor or named-combination setup is enabled.
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/vite/stack-workspace-lazy-controller.test.ts#lazy file reads are
  authoritative, selected independently, and do not globally lock browsing
- web/tests/vite/stack-workspace-lazy-controller.test.ts#opening lazy diffs
  keeps each stack's adjacent diff and only loads selected patches
- web/tests/vite/stack-workspace-lazy-controller.test.ts#branch net files load
  inline and never fall back to the mock's reverted file union
- web/tests/vite/stack-workspace-lazy-controller.test.ts#refresh and teardown
  cancel reads and reject late completions
- web/tests/vite/stack-workspace-lazy-controller.test.ts#closing one diff
  cancels its patch while the other stack remains open
- web/tests/vite/stack-workspace-lazy-controller.test.ts#read-only capabilities
  allow browsing and reject preview without invoking the executor
- web/tests/vite/stack-workspace-lazy-controller.test.ts#file page retry retains
  the exact cursor and rejects duplicate and cyclic continuations
- web/tests/vite/stack-workspace-lazy-controller.test.ts#opaque read-key changes
  invalidate loaded details even when the UI revision is unchanged
- web/tests/vite/stack-workspace-lazy-controller.test.ts#target-history
  selection loads only its files and listed patch with caller cancellation
- web/tests/vite/stack-workspace-lazy-controller.test.ts#loaded adjacent diffs
  retain their paths across refresh and remove missing files only after a
  complete summary
- web/tests/vite/stack-workspace-lazy-controller.test.ts#oversized file pages
  are rejected without installing partial files
- web/tests/vite/stack-workspace-lazy-controller.test.ts#target-history reads
  abort when only the opaque read key changes
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M21-004 details 2](details-02.md)
- [E-M21-004 details 3](details-03.md)
