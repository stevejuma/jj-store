---
schema: 1
id: E-PROJECT-AGENTS-001
---

# E-PROJECT-AGENTS-001

<!-- evidence-field:id markdown -->

## Id

E-PROJECT-AGENTS-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->

## Tasks

- M11.4
- M24

<!-- /evidence-field -->

<!-- evidence-field:requirements list -->

## Requirements

- JST-UI-015
- JST-API-001

<!-- /evidence-field -->

<!-- evidence-field:date markdown -->

## Date

2026-10-11
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->

## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->

## Revision

vzntlqzt
<!-- /evidence-field -->

<!-- evidence-field:files list -->

## Files

- catalog/components/store-work/ProjectsLiveAgents.stories.svelte
- catalog/fixtures/ProjectsLiveAgentsFixture.svelte
- docs/ui-provenance.json
- spec/src/current/contracts/api/41-interactive-agent-execution.md
- spec/src/current/contracts/ui/18-installation-projects-hub.md
- spec/src/current/contracts/ui/34-interactive-agent-sessions.md
- spec/src/current/governance/spec-governance.md
- spec/src/current/progress/M11/index.md
- spec/src/current/progress/M24/index.md
- web/src/lib/ui/store-agent-execution/StoreAgentExecutionDirectory.svelte
- web/src/lib/ui/store-agent-execution/repository-sessions.ts
- web/src/lib/ui/store-collaboration/StoreAgentSessions.svelte
- web/src/lib/ui/store-work/StoreProjectsAgentDirectory.svelte
- web/src/lib/ui/store-work/StoreProjectsAgents.svelte
- web/tests/repository-sessions.test.ts

<!-- /evidence-field -->

<!-- evidence-field:tests list -->

## Tests

- web/tests/repository-sessions.test.ts#explicit_code_bindings_replace_the_metadata_home_for_session_rows
- web/tests/repository-sessions.test.ts#legacy_unbound_sessions_use_the_exact_primary_repository
- web/tests/repository-sessions.test.ts#only_exact_admitted_recorded_aliases_suppress_a_duplicate_live_row
- web/tests/repository-sessions.test.ts#ambiguous_recorded_aliases_do_not_hide_distinct_interactive_sessions
- web/tests/repository-sessions.test.ts#interactive_row_search_preserves_the_repository_boundary

<!-- /evidence-field -->

<!-- evidence-field:story_files list -->

## Story files

- catalog/components/store-work/ProjectsLiveAgents.stories.svelte
- catalog/components/store-work/ProjectsAgents.stories.svelte
- catalog/components/store-work/ProjectsAgentActions.stories.svelte
- catalog/components/store-application/StoreCollaboration.stories.svelte

<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->

## Scope

Main-workspace UI-only correction: interactive session avatar rows appear inside
authorized repository collections in Projects Agents and repository Agents.
Explicit code bindings take precedence over metadata homes; each accessible
multi-binding gains a row, legacy sessions retain primary repositories, and
exact admitted recorded aliases render once. Existing recorded reads, paging,
project/home/code filter scope and navigation remain. The unfiltered
installation view includes accessible repositories with only live sessions
without extra captured reads. Avatar presence replaces the redundant Online
badge.
<!-- /evidence-field -->

<!-- evidence-field:before_state markdown -->

## Before state

The live-session component rendered a standalone section above the Projects and
repository directories. Exact regression
InteractiveSessionsUseRepositoryGroupsAndAvatarRows found
section[aria-label="Interactive agent sessions"] and failed once. Recorded rows
already supplied the desired avatar/collection presentation. Latest user
correction removes the Online badge from new rows because the avatar presence
chip communicates connection status.
<!-- /evidence-field -->

<!-- evidence-field:runs runs -->

## Validation

<!-- run -->

### Run 1

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown
```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook --maxWorkers 1 catalog/components/store-work/ProjectsLiveAgents.stories.svelte
```

**Notes**

Before implementation: expected standalone region to be absent, found the old
section. /tmp/jj-project-agents-before.log.
<!-- /run -->

<!-- run -->

### Run 2

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown
```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook --maxWorkers 1 catalog/components/store-work/ProjectsLiveAgents.stories.svelte
```

**Notes**

Initial grouping regression passes after implementation.
/tmp/jj-project-agents-after.log.
<!-- /run -->

<!-- run -->

### Run 3

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown
```

```shell
pnpm exec tsx --test web/tests/repository-sessions.test.ts
```

**Notes**

Five unit tests pass: explicit multi-bindings, legacy primary, exact admitted
aliases, ambiguity and scoped search. /tmp/jj-project-agents-unit.log.
<!-- /run -->

<!-- run -->

### Run 4

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown
```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook --maxWorkers 1 catalog/components/store-work/ProjectsLiveAgents.stories.svelte catalog/components/store-work/ProjectsAgents.stories.svelte catalog/components/store-work/ProjectsAgentActions.stories.svelte catalog/components/store-application/AgentTiming.stories.svelte
```

**Notes**

Eight tests across the three existing matched files pass, including the
no-Online assertion. AgentTiming.stories.svelte does not exist and contributes
no test coverage. /tmp/jj-project-agents-consumers.log.
<!-- /run -->
<!-- /evidence-field -->
<!-- generated-navigation -->

## Pages

- [E-PROJECT-AGENTS-001 details 2](details-02.md)
- [E-PROJECT-AGENTS-001 details 3](details-03.md)
- [E-PROJECT-AGENTS-001 details 4](details-04.md)
