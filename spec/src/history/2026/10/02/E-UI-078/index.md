---
schema: 1
id: E-UI-078
---

# E-UI-078

<!-- evidence-field:id markdown -->
## Id

E-UI-078
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

Unclipped agent tool actions, contiguous groups and message-edge alignment
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

Enclosing Jujutsu change mwtxzwry
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-002
- JST-UI-005
- JST-UI-006
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Visual baseline approval remains separate. Root aggregate lanes, Rust
  builds/tests and production built-CSP qualification are out of scope for this
  owned UI-only change.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Inspected Community TranscriptTools and TranscriptComments before released Design Core ToolCalls. The owned per-call discussion wrapper incorrectly created a 24px scrolling context around a 38px toolbar and forced full-height sizing. Removed that inner viewport and full-height constraint; desktop contexts retain their intrinsic height and narrow stacked contexts stretch horizontally. Compact grouped calls stay contiguous, with details consuming only their own row. Owned transcript tool rows now share the visible message edge, including the message's 8px bleed, rather than comparing against its avatar. Native IDs, review actions, menus, independent reply drafts and Focus/Split sheets remain unchanged. Reused appearance workspace and existing Vite HMR on7460/backend7462; no Rust restart, fixture mutation, reseed, main landing or dependency change.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- web/src/lib/ui/store-discussion/StoreDiscussionWorkspace.svelte
- web/src/lib/ui/store-collaboration/StoreAgentTranscript.svelte
- catalog/components/store-application/AgentToolGroups.stories.svelte
- catalog/components/store-application/agent-sidebar-play.ts
- spec/src/ui.md
- spec/src/api.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

[]
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Svelte5.57.1;Vitest4.1.11;Storybook10.5.7;released Design Core ToolCalls;existing Community source provenance retained.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Presentation-only. No AgentStore/ReviewStore encoding, API/schema, native identity, request, authorization or review anchor changes.
<!-- /evidence-field -->

<!-- evidence-field:artifacts markdown -->
## Artifacts

/tmp/jj-agent-tool-layout-*.log; /tmp/jj-agent-tools-layout-before.png; /tmp/jj-agent-tool-layout-fixed.png; /tmp/jj-agent-tool-layout-menu-fixed.png
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Review the live HMR UI; visual baseline approval remains separate.
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
CUA isolated live agent transcript geometry and hover before
```

**Notes**

Tool root983px versus visible message edge975px:8px inset. Each context clientHeight24/scrollHeight38 with overflow:auto; lower menu-button edge hits the next call instead of the toolbar. Reference screenshot has contiguous grouped rows and aligned heading/status icons. /tmp/jj-agent-tools-layout-before.png.
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
pnpm test:storybook catalog/components/store-application/AgentToolGroups.stories.svelte -t GroupedRowsAlignWithMessagesAndKeepHoverActionsClickable (before)
```

**Notes**

1failed/2skipped: exact visible-edge alignment reports8px. /tmp/jj-agent-tool-layout-before.log.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-application/AgentToolGroups.stories.svelte -t GroupedRowsAlignWithMessagesAndKeepHoverActionsClickable (alignment fixed, clipping before)
```

**Notes**

1failed/2skipped: alignment passes, pointer hit test at the toolbar button edge is false. /tmp/jj-agent-tool-layout-hover-before.log.
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
pnpm test:storybook catalog/components/store-application/AgentToolGroups.stories.svelte (after)
```

**Notes**

3/3 including exact message-edge alignment, compact contiguous rows before/after detail disclosure, heading/status alignment, every toolbar button's upper/lower pointer hit tests, no per-call scroll viewport, menus, native Copy ID and independent Split-to-Focus drafts. /tmp/jj-agent-tool-layout-after.log.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-application/AgentToolGroups.stories.svelte catalog/components/store-application/AppearanceDiscussions.stories.svelte catalog/components/store-application/StoreCollaboration.stories.svelte catalog/components/store-application/NativeAgentMenus.stories.svelte catalog/components/store-application/NativeReviewDetail.stories.svelte catalog/components/store-work/ProjectsAgents.stories.svelte catalog/components/store-threads/StoreCapturedReactions.stories.svelte (initial and final)
```

**Notes**

Each25/25 direct consumer interaction/accessibility scenarios on fresh Vitest browser/Vite processes. Final also retains narrow stacked-context horizontal stretch. /tmp/jj-agent-tool-layout-consumers.log; /tmp/jj-agent-tool-layout-consumers-final.log.
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
pnpm web:check; pnpm catalog:check; pnpm check:architecture
```

**Notes**

Zero type errors/warnings;140catalog modules;773production modules. /tmp/jj-agent-tool-layout-web.log; /tmp/jj-agent-tool-layout-catalog.log; /tmp/jj-agent-tool-layout-architecture.log.
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
pnpm test:storybook catalog/components/store-application/AgentToolGroups.stories.svelte -t GroupedRowsAlignWithMessagesAndKeepHoverActionsClickable --browser.viewport.width=390 --browser.viewport.height=844
```

**Notes**

1pass/2skipped. Not accepted as narrow-screen qualification: the existing browser instance explicitly selects1280px, overriding the top-level viewport. A disposable instance configuration follows. /tmp/jj-agent-tool-layout-narrow.log.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config /tmp/jj-agent-tool-layout-narrow.vitest.config.ts --project storybook catalog/components/store-application/AgentToolGroups.stories.svelte -t GroupedRowsAlignWithMessagesAndKeepHoverActionsClickable
```

**Notes**

Disposable /tmp TypeScript config loaded as CommonJS and failed Tailwind ESM interop at startup. Retried the same configuration with .mts; no product change. /tmp/jj-agent-tool-layout-narrow-qualified.log.
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
pnpm exec vitest run --config /tmp/jj-agent-tool-layout-narrow.vitest.config.mts --project storybook catalog/components/store-application/AgentToolGroups.stories.svelte -t GroupedRowsAlignWithMessagesAndKeepHoverActionsClickable
```

**Notes**

1pass/2skipped with explicit390x844 browser instance. Alignment, contiguous rows, expanded detail spacing and full upper/lower toolbar hit tests pass at the narrow breakpoint. Disposable config only; tracked test infrastructure unchanged. /tmp/jj-agent-tool-layout-narrow-qualified-esm.log.
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
CUA cold live Projects agent transcript geometry and lower-edge native pointer menu click
```

**Notes**

Final message/group/heading/call left edges1046px; closed call top positions277/301/325px and heights24px; contexts overflow:visible. Native pointer moving onto and clicking the formerly clipped lower edge opens the exact apply_patch menu. Initial earlier click encountered active session navigation and hit Bash; repeated with fresh hovered-button geometry confirms exact call selection. /tmp/jj-agent-tool-layout-menu-fixed.png. No live data writes or user-tab navigation.
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
pnpm exec prettier --write web/src/lib/ui/store-collaboration/StoreAgentTranscript.svelte web/src/lib/ui/store-discussion/StoreDiscussionWorkspace.svelte catalog/components/store-application/AgentToolGroups.stories.svelte catalog/components/store-application/agent-sidebar-play.ts spec/src/ui.md spec/src/api.md
```

**Notes**

Changed production, regression and canonical files formatted.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --write web/src/lib/ui/store-discussion/StoreDiscussionWorkspace.svelte spec/src/milestones.md; pnpm exec prettier --check web/src/lib/ui/store-discussion/StoreDiscussionWorkspace.svelte web/src/lib/ui/store-collaboration/StoreAgentTranscript.svelte catalog/components/store-application/AgentToolGroups.stories.svelte catalog/components/store-application/agent-sidebar-play.ts spec/src/ui.md spec/src/api.md spec/src/milestones.md; pnpm spec:first; pnpm spec:validate; pnpm check:ui-ownership; pnpm check:progress (initial and final)
```

**Notes**

Formatting passes;4protected files/3canonical chapters;5validators/21chapters/147requirements;Community ownership boundary and120task evidence bookkeeping pass. /tmp/jj-agent-tool-layout-spec-first.log; /tmp/jj-agent-tool-layout-spec-validate.log; /tmp/jj-agent-tool-layout-ownership.log; /tmp/jj-agent-tool-layout-progress.log; /tmp/jj-agent-tool-layout-progress-final.log.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
jj --no-pager diff --check
```

**Notes**

CLI rejected unsupported --check flag before execution. Native targeted diff was reviewed separately; the seven-file Prettier check is the passing formatting gate. No source or repository-state mutation.
<!-- /run -->
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


