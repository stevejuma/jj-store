---
schema: 1
id: E-M21-005
---

# E-M21-005

<!-- evidence-field:id markdown -->
## Id

E-M21-005
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

enclosing Jujutsu change mrkpwllo
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

explicit-unconfigured-stack-workspace-and-read-target
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

JJ 0.45.1; pnpm 10.32.1; Vitest 4.1.11; parent 213faf0f
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Nullable target/base/active state and exact targetOptions are presentation contracts. Stateful catalog execution uses ConfiguredWorkspaceSnapshot. No wire/storage/checkout format changes.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-stack-workspace/StoreStackWorkspaceSetup.stories.svelte
- catalog/fixtures/StackWorkspaceSetupApplicationFixture.svelte
- catalog/fixtures/stack-workspace/adapter.ts
- catalog/fixtures/stack-workspace/changes.ts
- catalog/fixtures/stack-workspace/data.ts
- catalog/fixtures/stack-workspace/menu-reducer.ts
- catalog/fixtures/stack-workspace/menu-validation.ts
- catalog/fixtures/stack-workspace/preview.ts
- catalog/fixtures/stack-workspace/reducer.ts
- catalog/fixtures/stack-workspace/validation.ts
- docs/maintenance/coverage.json
- docs/site/technical/architecture.md
- spec/src/api.md
- spec/src/documentation.md
- spec/src/ui.md
- web/src/lib/controller/stack-workspace/commit-drafts.svelte.ts
- web/src/lib/controller/stack-workspace/controller.svelte.ts
- web/src/lib/controller/stack-workspace/eligibility.ts
- web/src/lib/controller/stack-workspace/files.ts
- web/src/lib/controller/stack-workspace/presentation.ts
- web/src/lib/controller/stack-workspace/read-session.ts
- web/src/lib/controller/stack-workspace/retained-diffs.ts
- web/src/lib/controller/stack-workspace/target-history.svelte.ts
- web/src/lib/controller/stack-workspace/types.ts
- web/src/lib/ui/store-stack-workspace/StoreStackCommitComposer.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackTargetHistory.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackWorkspace.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceCanvas.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackWorkspacePicker.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceSetup.svelte
- web/tests/vite/stack-workspace-setup.test.ts
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Three exact setup regressions fail before nullable production state is supported. First migrated web type check finds 29 nullable call-site errors. Initial story tests have selector failures; the ordinary mutation controller exceeds its source budget until read ownership is extracted.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Represent no target/combination without fabricating main, applied membership or Unstaged files. Reuse Settings Field.Group/StoreWorkField and repository Command View picker for exact operation-scoped target choices. Workspace remains empty until a real named combination exists. Missing active IDs fail closed. Shared read epoch rejects late target choices after refresh/close. Preserve strict configured catalog mutation fixtures. Keep body tabs and existing application styling.
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/vite/stack-workspace-setup.test.ts#an unconfigured checkout loads
  without inventing a target or combination
- web/tests/vite/stack-workspace-setup.test.ts#advertised mutations cannot
  bypass missing target and combination setup
- web/tests/vite/stack-workspace-setup.test.ts#target selection uses the current
  read context and never calls the executor
- web/tests/vite/stack-workspace-setup.test.ts#an unknown active combination ID
  cannot imply applied membership or enable mutations
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M21-005 details 2](details-02.md)
- [E-M21-005 details 3](details-03.md)
