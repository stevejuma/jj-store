---
schema: 1
id: E-M20-005
---

# E-M20-005

<!-- evidence-field:id markdown -->
## Id

E-M20-005
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

enclosing Jujutsu change smlsqupk
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

compact-column-panel-toggles
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

JJ 0.45.1; Node 26.9.0; pnpm 10.32.1; Svelte 5.57.1; consumer Storybook 10.5.7; source Storybook 10.5.2; Vitest 4.1.11; Design Core source qualification is separate from released registry qualification. Design Core source change qwzmvmkk / bab5dc582e33.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No backend, storage, API, RPC, controller or persisted layout format changes. Tracked package manifests and registry lockfile remain unchanged. Existing ignored temporary foundational validation resolves byte-identical Design Core source.
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Add a compact rectangular Panel Toggle family upstream in Design Core. Default Button geometry is 28×24px and decorative SVG is 18×12px. Controlled expanded state uses a filled bar; hover and keyboard focus reversibly preview the opposite width without changing state or geometry. Right orientation mirrors the bar. Disabled, reduced-motion, forced-color and keyboard semantics remain supported. Column Canvas uses it by default in headers and both collapsed-rail variants, retaining custom icons, rails and transient preview/controller behavior. New optional columnTitle labels custom headers and collapsed rails without generating a second header; Unstaged uses it. The consumer app-shell story measures exact geometry and exercises collapse/expand. Broader direct-consumer stories cover stack workspace and repository log layouts because both use the shared toggle. Source pnpm checks is required by Design Core AGENTS.md for the public family/default change. Consumer root aggregates, Rust, controller units and production builds are out of scope for this UI-only composition slice. All 438 prior raw evidence objects are preserved. Ghost paint uses low-specificity selectors so Column Canvas retains its existing public header action hover colors. Exact new hover-token regression failed before and passed after the source correction.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-stack-workspace/StoreStackWorkspaceApplication.stories.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackUnstaged.svelte
- spec/src/ui.md
- spec/src/verification.md
- spec/src/milestones.md
- catalog/fixtures/stack-workspace/story-actions.ts
- spec/src/api.md
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M20-005 details 2](details-02.md)
- [E-M20-005 details 3](details-03.md)
- [E-M20-005 details 4](details-04.md)
