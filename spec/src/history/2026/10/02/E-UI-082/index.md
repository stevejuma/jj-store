---
schema: 1
id: E-UI-082
---

# E-UI-082

<!-- evidence-field:id markdown -->
## Id

E-UI-082
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

Aligned agent session rows and loading insets
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

Enclosing Jujutsu change mkopqqvt
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-005
- JST-UI-006
- JST-UI-015
- JST-API-006
- JST-GOV-002
- JST-GOV-003
- JST-GOV-007
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Visual baselines remain visual-pending. No milestone status promotion. Rust,
  root aggregate lanes, production bundle builds, backend rebuild/reset/restart,
  main landing and publication are out of scope for this UI-only row and
  skeleton slice.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Reuse the clean appearance workspace and existing Vite HMR. Community avatar/layout source inspected first, Design Core button/skeleton second. Repository and Projects agent lists share heading-relative horizontal inset and borderless collection rows. Initial Projects and per-repository skeletons share that inset, row gap and horizontal padding; the agent-specific variant preserves other collaboration skeletons. Agent avatars align to row center. Actions and optional code association controls participate in the title row with their full existing button target and independent pointer interaction, leaving duration below. Existing directly affected stories gain geometry and menu assertions; no new test family or foundational dependency change.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-application/AgentLayout.stories.svelte
- catalog/components/store-application/StoreCollaboration.stories.svelte
- catalog/components/store-work/ProjectsAgents.stories.svelte
- spec/src/api.md
- spec/src/milestones.md
- spec/src/ui.md
- web/src/lib/ui/store-collaboration/StoreAgentSessionRow.svelte
- web/src/lib/ui/store-collaboration/StoreAgentSessions.svelte
- web/src/lib/ui/store-collaboration/StoreCollaborationSkeleton.svelte
- web/src/lib/ui/store-collaboration/store-collaboration.css
- web/src/lib/ui/store-work/StoreProjectsAgentDirectory.svelte
- web/src/lib/ui/store-work/StoreProjectsLoading.svelte
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

[]
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Node26.9.0; pnpm10.32.1; Svelte5.57.1; Vitest4.1.11; Storybook10.5.7; existing Design Core ^0.2.0 and owned Community provenance retained. No dependency changes.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

AgentStore summaries, pagination, native session/snapshot IDs, route/hash, HTTP/OpenAPI, local object/manifest/WAL/receipt formats unchanged. New skeleton prop and CSS inset are presentation only.
<!-- /evidence-field -->

<!-- evidence-field:artifacts markdown -->
## Artifacts

/tmp/jj-agent-row-*.log; /tmp/jj-agent-row-live.jpg
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Retain visual-pending and the committed workspace for further UI changes.
<!-- /evidence-field -->

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
Read-only live Projects agent row geometry (before)
```

**Notes**

Avatar center was7.296875px above row center; actions button bottom258.1796875px overlapped duration top252.7734375px by5.40625px. Exact shared row layout regression.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first (initial)
```

**Notes**

Mapped api.md missing from the protected change. Added presentation-only session contract statement; no API changes.
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
pnpm exec prettier --write (11 changed production/story/chapter files); pnpm exec prettier --write catalog/components/store-application/AgentLayout.stories.svelte
```

**Notes**

Formatted changed files; format and format-story logs.
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
pnpm web:check
```

**Notes**

0errors0warnings. /tmp/jj-agent-row-web.log.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm catalog:check (initial)
```

**Notes**

Storybook Testing Library ByRoleOptions rejects exact:true. Replaced with anchored /^Copy$/ accessible name; same runtime matching semantics. /tmp/jj-agent-row-catalog.log.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm catalog:check (qualified)
```

**Notes**

0errors0warnings;146catalog modules. /tmp/jj-agent-row-catalog-final.log.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-application/AgentLayout.stories.svelte catalog/components/store-work/ProjectsAgents.stories.svelte catalog/components/store-application/StoreCollaboration.stories.svelte catalog/components/store-application/RepositoryLoading.stories.svelte
```

**Notes**

41passed across4files in fresh Vitest browser process. Projects/selected-project/repository row gap, avatar centers, menu opening and duration clearance, header insets/no separators, initial Projects and repository skeleton geometry, loading/reviews and Focus/Split/mobile direct consumers. /tmp/jj-agent-row-stories.log.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:architecture; pnpm check:ui-ownership
```

**Notes**

788modules within unchanged budgets; ownership1validator/21chapters/147requirements.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first; pnpm spec:validate (qualified)
```

**Notes**

9protected files/2canonical chapters;5validators/21chapters/147requirements. spec-first/spec-validate logs.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
Read-only live Projects and repository Agents verification (CUA temporary background tab)
```

**Notes**

Both avatar center offsets0px and action/duration gap5.59375px. Repository row left/right match heading exactly, border0px and inner padding10.4px; row height79.984375px. Projects actions menu opens; repository initial loading observed. Saved and inspected full viewport screenshot. Temporary tab closed; user tab untouched. No data writes or service resets.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress; pnpm spec:first; pnpm spec:validate; pnpm exec prettier --check (12 changed files)
```

**Notes**

120tasks;9protected files/3canonical chapters;5validators/21chapters/147requirements;all changed files formatted. Final progress/spec-first/spec-validate/format logs. This evidence insertion is formatted and checked before standing JJ commit.
<!-- /run -->
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


