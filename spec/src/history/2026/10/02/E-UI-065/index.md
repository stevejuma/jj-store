---
schema: 1
id: E-UI-065
---

# E-UI-065

<!-- evidence-field:id markdown -->
## Id

E-UI-065
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M4.9
- M11.4
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-02
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

focused repository card layout and branch loading regression
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

Enclosing Jujutsu change konqzwkq
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-003
- JST-UI-005
- JST-UI-015
- JST-API-005
- JST-WORK-008
- JST-ARCH-004
- JST-GOV-007
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Functional checks pass. Visual baseline approval and released-registry Design
  Core qualification remain separate existing gates; no visual approval or
  registry-only qualification is claimed.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

UI-only repository directory and Branches loading presentation in the same web package. Verification covers exact regressions, web/catalog typing, production branch actions and history/automatic-refresh consumers, affected Storybook interaction/accessibility families, and a cold UI tab. No route cache or storage contract is introduced. Rust builds/tests, root aggregate check/test/build/spec:check, full unrelated Storybook suite, native publications, fixture reset, backend restart, package publication, main landing and production built-bundle CSP/release qualification are out of scope under the targeted UI policy. Existing milestone statuses are unchanged.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Node26.9.0; pnpm10.32.1; JJ0.45.1; Svelte5.57.1; Storybook10.5.7; Vitest4.1.11; Playwright1.63.0. Existing private Design Core source hook retained; tracked registry range^0.2.0 unchanged.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No API, manifest, WAL, code operation, WorkStore, cursor or permission format changes. Reference loading is separate from graph loading; selection continues existing operation-pinned scoped reads. Native directory metadata and full bound identities are unchanged.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Review this committed UI slice in the Appearance HMR workspace; keep registry qualification and visual baseline approval separate.
<!-- /evidence-field -->

<!-- evidence-field:artifacts markdown -->
## Artifacts

/tmp/jj-repository-layout-*.log; /tmp/jj-repository-cards-before*.log; /tmp/jj-repository-cards-final.jpg. Interactive Vite7460/backend7462 and persistent main fixture preserved.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-application/StoreRepositoryBranchContext.stories.svelte
- catalog/components/store-work/ProjectsDirectory.stories.svelte
- catalog/controller/loading-store-controller.ts
- catalog/fixtures/ProjectsHubFixture.svelte
- catalog/fixtures/RepositoryFixture.svelte
- catalog/fixtures/StoreRepositoryApplicationFixture.svelte
- web/src/lib/ui/community-projects/CommunityProjectsRepositoryLog.svelte
- web/src/lib/ui/store-application/StoreRepositoryApplication.svelte
- web/src/lib/ui/store-work/StoreRepositoryDirectoryRow.svelte
- web/src/lib/ui/store-work/community-work-directory.css
- web/tests/vite/store-workspaces.test.ts
- spec/src/ui.md
- spec/src/api.md
- spec/src/projects.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/vite/store-workspaces.test.ts#branch selections preserve loaded
  references and paging while reading only scoped history
- web/tests/vite/production-automatic-refresh.test.ts#a missed repository
  publication reloads without an SSE or a manual action
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-UI-065 details 2](details-02.md)
