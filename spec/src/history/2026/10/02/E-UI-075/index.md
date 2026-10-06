---
schema: 1
id: E-UI-075
---

# E-UI-075

<!-- evidence-field:id markdown -->
## Id

E-UI-075
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M9.5
- M11.4
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-02
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Thread-reply shell sidebar for repository and Projects agent responses
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

Enclosing Jujutsu change tvzlkoyt
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-002
- JST-UI-005
- JST-UI-006
- JST-WORK-008
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Visual baseline approval remains separate. Root aggregate lanes, Rust
  builds/tests and production built-CSP qualification are out of scope.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Owned UI-only application sidebar composition. Community standard right sidebar inspected first, public Design Core AppShell sidebar/header/body/close and mobile controller second. Repository and Projects Agents retain independently scoped directory controllers and exact routes while responses use the same full-body standard sidebar as Thread replies. Avatar sits beside shell title; no duplicate identity card. Context no longer fills spare height, so recorded activity stays at top. Copy identifiers remain in header menu. Close clears route and returns row focus; standard width resize and separate outer rail retained. Fresh targeted Storybook processes supply cold lifecycle interaction/accessibility evidence; live read-only HMR acceptance uses an isolated tab and existing fixture. No source-dependency or runtime fixture changes.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-application/StoreCollaboration.stories.svelte
- catalog/components/store-application/agent-sidebar-play.ts
- catalog/components/store-work/ProjectsAgents.stories.svelte
- spec/src/api.md
- spec/src/collaboration.md
- spec/src/milestones.md
- spec/src/projects.md
- spec/src/ui.md
- web/src/lib/ui/store-application/StoreApplicationRightPanels.svelte
- web/src/lib/ui/store-application/StoreRepositoryApplication.svelte
- web/src/lib/ui/store-collaboration/StoreAgentContext.svelte
- web/src/lib/ui/store-collaboration/StoreAgentSessionMenu.svelte
- web/src/lib/ui/store-collaboration/StoreAgentSessions.svelte
- web/src/lib/ui/store-collaboration/StoreAgentSidebar.svelte
- web/src/lib/ui/store-collaboration/StoreAgentTranscript.svelte
- web/src/lib/ui/store-work/StoreProjectsAgents.svelte
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

[]
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Svelte5.57.1;Vitest4.1.11;Storybook10.5.7;existing public Design Core AppShell primitives.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No object/receipt/API changes. Existing Projects agent_repository/session hash and repository session route retained; standard right panel persistence reused.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Complete authorized Community structural Focus sheets.
<!-- /evidence-field -->

<!-- evidence-field:artifacts markdown -->
## Artifacts

/tmp/jj-agent-sidebar-*.log; /tmp/jj-agent-standard-sidebar.png.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-UI-075 details 2](details-02.md)
- [E-UI-075 details 3](details-03.md)
