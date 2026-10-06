---
schema: 1
id: E-M20-006
---

# E-M20-006

<!-- evidence-field:id markdown -->
## Id

E-M20-006
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M20
- M20.1
- M20.2
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-04
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change npxlrxyw
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

workspace-target-branch-history
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

JJ 0.45.1; Node 26.9.0; pnpm 10.32.1; Svelte 5.57.1; Storybook 10.5.7; Vitest 4.1.11. Existing ignored Design Core source validation remains separate from registry qualification.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Frontend-only snapshot target metadata and injected, revision/head-pinned bounded history read. No HTTP/RPC/Rust/storage/persistence changes. package.json and pnpm-lock.yaml unchanged.
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Add a target bookmark card above Branches column filters using the existing owned repository command picker. One read-only target-history stack uses the existing Card, author, revision graph, virtual rows, inline file list and adjacent diff compositions. History requests load 40 newest-first commits per page and deduplicate concurrent requests and identities. Errors retain the exact cursor and loaded history for retry. Refresh/head changes and teardown discard obsolete replies. Target history never enters the applied-stack snapshot, drafts or mutations. The 240-commit example tests paging to exhaustion and bounded DOM rows, while origin/trunk tests keyboard access independent of bookmark filters. Header geometry reuses the displayed graph inset and measures title containment, icon centering with its border accounted for, and common title/author/chip/commit-content alignment. All 439 prior raw evidence objects are preserved byte-for-byte. UI/controller-only targeted verification covers the shared picker and its repository-log direct consumers; root aggregates, Rust, production builds and backend work are out of scope. Final direct-consumer acceptance passes 64 controller tests and 103 stories in 21 files, with zero type diagnostics. The duplicate target-branch header above the page tabs is removed at user request. The target remains above the Branches column filters, survives ordinary branch selection and tab round trips, and is absent on Workspace. Applied/local grouping remains covered. After removing the duplicate body header, all 83 stack-workspace interaction/accessibility stories in 17 files pass in a fresh Vitest/Vite/browser process; earlier 103-story direct-consumer qualification also covers the unchanged shared picker and repository-log consumers.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-stack-workspace/StoreStackWorkspaceBranches.stories.svelte
- catalog/components/store-stack-workspace/StoreStackWorkspaceTarget.stories.svelte
- catalog/fixtures/StackWorkspaceApplicationFixture.svelte
- catalog/fixtures/stack-workspace/adapter.ts
- catalog/fixtures/stack-workspace/data.ts
- catalog/fixtures/stack-workspace/target-history.ts
- spec/src/api.md
- spec/src/milestones.md
- spec/src/ui.md
- spec/src/verification.md
- web/src/lib/controller/stack-workspace/controller.svelte.ts
- web/src/lib/controller/stack-workspace/files.ts
- web/src/lib/controller/stack-workspace/presentation.ts
- web/src/lib/controller/stack-workspace/target-history.svelte.ts
- web/src/lib/controller/stack-workspace/types.ts
- web/src/lib/ui/community-repository-branch-command/CommunityRepositoryBranchCommand.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackTargetHistory.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackWorkspace.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceCanvas.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceDiff.svelte
- web/tests/vite/stack-workspace-target-history.test.ts
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M20-006 details 2](details-02.md)
- [E-M20-006 details 3](details-03.md)
- [E-M20-006 details 4](details-04.md)
