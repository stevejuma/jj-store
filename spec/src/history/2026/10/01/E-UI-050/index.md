---
schema: 1
id: E-UI-050
---

# E-UI-050

<!-- evidence-field:id markdown -->
## Id

E-UI-050
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M11.4
- M3.3
- M13.2
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-01
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

development-fixtures-and-navigation
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- README.md
- catalog/components/store-work/NavigationPersistence.stories.svelte
- catalog/fixtures/NavigationPersistenceFixture.svelte
- catalog/fixtures/ProjectsHubFixture.svelte
- package.json
- scripts/dev-demo-agents.mjs
- scripts/dev-demo-collaboration.test.mjs
- scripts/dev-demo-folder.mjs
- scripts/dev-demo-folder.test.mjs
- scripts/dev-demo-profiles.acceptance.mjs
- scripts/dev-demo-profiles.mjs
- scripts/dev-demo-profiles.test.mjs
- scripts/dev-demo-work-run.mjs
- scripts/dev-demo-work-samples.mjs
- scripts/dev-demo-work.mjs
- scripts/dev-demo-work.test.mjs
- scripts/dev-demo.mjs
- scripts/install-cli.mjs
- scripts/install-cli.test.mjs
- spec/src/api.md
- spec/src/architecture.md
- spec/src/cli.md
- spec/src/milestones.md
- spec/src/spec-governance.md
- spec/src/ui.md
- web/src/lib/ui/community-navigation/CommunityNavigation.svelte
- web/src/lib/ui/navigation-storage.ts
- web/src/lib/ui/store-application/StoreApplication.svelte
- web/tests/navigation-storage.test.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/navigation-storage.test.ts#navigation restores explicit collapsed
  sections and expanded projects
- scripts/dev-demo-work.test.mjs#work samples use one distinct repository per
  project and migrate only untouched seeded targets
- scripts/dev-demo-profiles.test.mjs#profiles seed real native and Git backends,
  remove server checkout, and reuse identities
- scripts/install-cli.test.mjs#CLI installer copies both tools without replacing
  jj or following target symlinks
- scripts/dev-demo-collaboration.test.mjs#agent seed installs, publishes, and
  removes each owned integration
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-UI-050 details 2](details-02.md)
- [E-UI-050 details 3](details-03.md)
- [E-UI-050 details 4](details-04.md)
