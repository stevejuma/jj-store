---
schema: 1
id: E-M21-067
---

# E-M21-067

<!-- evidence-field:id markdown -->
## Id

E-M21-067
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-06
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M21
- M21.1
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-020
- JST-UI-014
- JST-API-001
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Stacks Details navigation and applicable action presentation
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

Enclosing Jujutsu change zrnnkmxu: Match the Stacks details rail to repository navigation
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Replace the far-right Stacks metadata list with the repository heading and compact Details navigation. Keep focused routes and retained canvas selections/drafts/diffs. Show only applicable workspace actions and omit an empty Actions section. Work and commit in the default/main workspace. Screenshot comparisons/baselines, full product suites, Rust builds/tests, package publication and release qualification were out of scope under the targeted UI policy. This slice changes no startup or lifecycle source.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-stack-workspace/StoreStacksNavigation.stories.svelte
- catalog/components/store-stack-workspace/StoreStackWorkspaceConflicts.stories.svelte
- catalog/components/store-stack-workspace/StoreStackWorkspaceSetup.stories.svelte
- web/src/lib/ui/store-application/StoreRepositoryNavigation.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceDetails.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceDetailsActions.svelte
- spec/src/current/contracts/ui/04-local-stack-workspace-catalog.md
- spec/src/current/contracts/api/38-ordinary-local-stacks-transport.md
- spec/src/current/progress/M21/history/M21/index.md
- spec/src/current/progress/M21/history/M21.1/index.md
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Svelte 5.57.1; Storybook 10.5.7; Vitest 4.1.11; pnpm 10.32.1; macOS ARM64. Existing main Vite uses Node 24.10.0. Released Design Core dependency unchanged.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No endpoint, DTO, route field, native JJ command or version-1 .jj/jj-store/stacks.toml changes. Frontend navigation composition and visibility only.
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Main/default was clean at zrnnkmxu/e9b232e9. Workspace showed introductory Settings-style rows and disabled actions, unlike the repository Details navigation. The two new focused targets failed before production edits. An initial spaced-name Vitest filter skipped all 12 tests; vitest list identified the CamelCase generated names and the correct target was rerun.
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

[]
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M21-067 details 2](details-02.md)
- [E-M21-067 details 3](details-03.md)
