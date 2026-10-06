---
schema: 1
id: E-M21-063
---

# E-M21-063

<!-- evidence-field:id markdown -->
## Id

E-M21-063
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M21
- M21.1
- M21.2
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-06
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

immediate stack membership actions
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

Enclosing Jujutsu change otvxrvnt: Apply and unapply stacks without confirmation
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Single-click Apply/Unapply from Branches and column/branch menus; retain exact request creation, execution guards, grouped membership, drafts and uncertain-outcome recovery. Other command confirmations unchanged.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-stack-workspace/StoreStackTopology.stories.svelte
- catalog/components/store-stack-workspace/StoreStackWorkspace.stories.svelte
- catalog/components/store-stack-workspace/StoreStackWorkspaceBranches.stories.svelte
- catalog/components/store-stack-workspace/StoreStackWorkspaceRecovery.stories.svelte
- catalog/fixtures/StackTopologyFixture.svelte
- docs/maintenance/coverage.json
- docs/site/guides/browsing.md
- spec/src/api.md
- spec/src/documentation.md
- spec/src/ui.md
- web/src/lib/controller/stack-workspace/command-request.ts
- web/src/lib/controller/stack-workspace/controller.svelte.ts
- web/src/lib/controller/stack-workspace/local-session.ts
- web/src/lib/ui/store-stack-workspace/StoreStackWorkspace.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackWorkspacePreview.svelte
- web/tests/real/stacks-browser.spec.ts
- web/tests/vite/stack-workspace-membership.test.ts
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64; JJ/jj-lib0.45.1; pnpm10.32.1; Svelte5.57.1; Storybook10.5.7; Vitest4.1.11. Existing Design Core validation setup unchanged.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Frontend-only execution dispatch and recovery presentation. Existing command POST,operation/binding/configuration guards,exact request identities,TOMLv1,native merge/WIP and OpenAPI/TypeScript formats unchanged.
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Apply/Unapply entered a generic advisory confirmation; no mutation ran until a second Apply command click. The updated Branches story reproduced that mismatch before implementation.
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/vite/stack-workspace-membership.test.ts#one click applies and
  unapplies the exact branch without requesting a preview
- web/tests/vite/stack-workspace-membership.test.ts#pending membership blocks
  duplicate clicks and disposal cannot install the result
- web/tests/vite/stack-workspace-membership.test.ts#unknown membership retains
  its exact request and never retries a local executor automatically
- web/tests/vite/stack-workspace-membership.test.ts#stale membership is rejected
  without a stuck preview and can refresh
- web/tests/vite/stack-workspace-membership.test.ts#capability denial and
  non-membership commands cannot use immediate execution
- web/tests/real/stacks-browser.spec.ts#cold_local_megamerges_apply_unapply_and_named_wip_survive_reload
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M21-063 details 2](details-02.md)
