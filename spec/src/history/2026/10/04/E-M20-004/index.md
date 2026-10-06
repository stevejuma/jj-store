---
schema: 1
id: E-M20-004
---

# E-M20-004

<!-- evidence-field:id markdown -->
## Id

E-M20-004
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

enclosing Jujutsu change rqvrxvzk
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

workspace-trailing-spacer-presentation
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

JJ 0.45.1, Node 26.9.0, pnpm 10.32.1, Svelte 5.57.1, Storybook 10.5.7, Vitest 4.1.11; Design Core source validation remains separate from registry qualification; new Design Core public seam committed at 73e8d5bbeb69
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Frontend mock create_stack input adds optional exact selections; no Rust/API/RPC or persisted format changes.
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

The Workspace trailing spacer now composes shadcn Empty, a branch icon and Create a new branch action, centered vertically. Following UI review it uses a plain surface without dots or a leading border, widened from 240px to 390px to match a default stack column. Branches retains its blank 240px reserve. It is never a registered column and only Workspace renders the prompt. Button opens the existing independent/dependent Settings form. File/folder/hunk drops retain exact selections for preview and stage an inline draft only after confirmed creation. Source files remain until the commit is confirmed; unknown requests retain their identity and stage only after status confirmation. The generic public trailingSpacer snippet was implemented upstream first. Controller retains selected branch after confirmed creation and preserves existing selections through a pure helper. Resize geometry scenario moved to its own story family to keep the 400-line story budget. All 437 prior raw evidence objects are preserved. Root aggregates, Rust and production built-bundle tests are out of scope for this UI-only consumer slice. The commit composer to first branch gap increases from 8px to 12px, matching branch-to-branch spacing through one application token. The measured regression repeats after the inline form opens; Branches Apply card spacing is unchanged.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-stack-workspace/StoreStackWorkspaceApplication.stories.svelte
- catalog/components/store-stack-workspace/StoreStackWorkspaceRefinements.stories.svelte
- catalog/components/store-stack-workspace/StoreStackWorkspaceResize.stories.svelte
- catalog/components/store-stack-workspace/StoreStackWorkspaceSpacer.stories.svelte
- catalog/components/store-stack-workspace/StoreStackWorkspaceSpacing.stories.svelte
- catalog/fixtures/stack-workspace/preview.ts
- catalog/fixtures/stack-workspace/validation.ts
- docs/ui-provenance.json
- spec/src/api.md
- spec/src/milestones.md
- spec/src/spec-governance.md
- spec/src/ui.md
- spec/src/verification.md
- web/src/lib/controller/stack-workspace/commit-drafts.svelte.ts
- web/src/lib/controller/stack-workspace/controller.svelte.ts
- web/src/lib/controller/stack-workspace/drag.ts
- web/src/lib/controller/stack-workspace/presentation.ts
- web/src/lib/controller/stack-workspace/types.ts
- web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceCanvas.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceForm.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceSpacer.svelte
- web/src/lib/ui/store-stack-workspace/stack-workspace-layout.css
- web/src/lib/ui/store-stack-workspace/stack-workspace.css
- web/tests/vite/stack-workspace-spacer.test.ts
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M20-004 details 2](details-02.md)
- [E-M20-004 details 3](details-03.md)
- [E-M20-004 details 4](details-04.md)
- [E-M20-004 details 5](details-05.md)
