---
schema: 1
id: E-UI-072
---

# E-UI-072

<!-- evidence-field:id markdown -->
## Id

E-UI-072
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M11.4
- M9.5
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-02
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

scoped Projects Agents directories and inline authorized responses
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

Enclosing Jujutsu change rqpmklxo
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-005
- JST-UI-015
- JST-COLLAB-005
- JST-WORK-008
- JST-API-005
- JST-ARCH-004
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Visual approval remains separate. Root aggregate, Rust, unrelated UI lanes and
  persistent fixture writes are out of scope.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Existing browser/controller surface only. Selected-project Agents covers independently authorized linked repositories, including retained retired links; landing Agents drains unfiltered project links with two readers and at most 80 continuations per home/action, deduplicates scope and reports inaccessible links and incomplete/failed scope. Independent AgentStore snapshot paging, bounded refresh windows and generation guards remain. Selection opens the shared response in the body right pane without directory reload or global connection change; close, Back/Forward and cold hash restoration retain the scoped directory. Cached responses obtain the current authorized source after rediscovery. Shared owned row uses collection inset and no separators. No backend build/restart, native encoding/API changes, persistent fixture writes, main landing, publication or baseline update. Former evidence source StoreProjectsAgentResponse.svelte was retired in E-UI-075; file pointer now names its shared standard-sidebar replacement.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Node26.9.0;pnpm10.32.1;JJ0.45.1;Svelte5.57.1;Vitest4.1.11;Storybook10.5.7;existing Design Core public components.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Projects hash accepts tab=agents and paired full agent_repository/session IDs only on Agents. Repository routes, AgentStore captures and native/local identities remain unchanged.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Continue authorized top-row search and Community structural Focus sheet.
<!-- /evidence-field -->

<!-- evidence-field:artifacts markdown -->
## Artifacts

/tmp/jj-project-agents-*.log; /tmp/jj-agents-*.log; /tmp/jj-project-agents-inline.png; live read-only temporary tab19.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-work/ProjectsAgents.stories.svelte
- catalog/components/store-work/ProjectsHub.stories.svelte
- catalog/fixtures/ProjectsHubFixture.svelte
- catalog/fixtures/projects-hub-fixture.ts
- spec/src/api.md
- spec/src/architecture.md
- spec/src/collaboration.md
- spec/src/milestones.md
- spec/src/projects.md
- spec/src/ui.md
- web/src/lib/controller/contracts.ts
- web/src/lib/controller/http-local-work-data-source.ts
- web/src/lib/controller/http-work-data-source.ts
- web/src/lib/controller/production-projects-hub.svelte.ts
- web/src/lib/controller/production-projects-integration.ts
- web/src/lib/controller/production-store-controller.svelte.ts
- web/src/lib/controller/project-agent-response.svelte.ts
- web/src/lib/controller/projects-agent-scope.ts
- web/src/lib/controller/projects-agent-selection.svelte.ts
- web/src/lib/controller/projects-agents.svelte.ts
- web/src/lib/controller/projects-hub-homes.ts
- web/src/lib/controller/projects-hub-types.ts
- web/src/lib/controller/projects-navigation.svelte.ts
- web/src/lib/controller/projects-related-views.ts
- web/src/lib/controller/projects-route-actions.ts
- web/src/lib/controller/projects-route-codec.ts
- web/src/lib/ui/store-collaboration/StoreAgentSessionRow.svelte
- web/src/lib/ui/store-collaboration/StoreAgentSessions.svelte
- web/src/lib/ui/store-collaboration/store-collaboration.css
- web/src/lib/ui/store-work/StoreProjects.svelte
- web/src/lib/ui/store-work/StoreProjectsActions.svelte
- web/src/lib/ui/store-work/StoreProjectsAgentDirectory.svelte
- web/src/lib/ui/store-collaboration/StoreAgentSidebar.svelte
- web/src/lib/ui/store-work/StoreProjectsAgents.svelte
- web/src/lib/ui/store-work/StoreProjectsViewport.svelte
- web/src/lib/ui/store-work/projects-initial-loading.ts
- web/src/lib/ui/store-work/store-projects.css
- web/tests/vite/projects-agent-response.test.ts
- web/tests/vite/projects-agents.test.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/vite/projects-agents.test.ts#Agents reads only deduplicated project
  repositories and retains pinned continuations
- web/tests/vite/projects-agents.test.ts#Selected project scope never falls back
  to all repositories
- web/tests/vite/projects-agents.test.ts#Denied reads and rediscovery clear
  retained agent sessions
- web/tests/vite/projects-agents.test.ts#Agent scope includes repositories
  linked on later project pages
- web/tests/vite/projects-agents.test.ts#Agent pane selection retains the scoped
  directory and exact route identities
- web/tests/vite/projects-agent-response.test.ts#Agent response rejects late
  selection and clears protected data on revocation
- web/tests/vite/projects-agent-response.test.ts#Agent response verifies session
  identity and reports denied reads
- web/tests/vite/projects-agent-response.test.ts#Cached agent response uses the
  current source after access is restored
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-UI-072 details 2](details-02.md)
- [E-UI-072 details 3](details-03.md)
