---
schema: 1
id: E-UI-079
---

# E-UI-079 details 2

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
pnpm test:storybook catalog/components/store-work/ProjectsFilters.stories.svelte (before)
```

**Notes**

Desktop/mobile circle regression:38px width vs36px height. /tmp/jj-search-controls-before-tests.log.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-work/ProjectsFilters.stories.svelte (initial after)
```

**Notes**

Geometry corrected; newly authored keyboard assertion used Shift-Tab despite leading addon following input in DOM. Corrected to Tab, then reverted to Shift-Tab when leading button became a persistent header sibling before input. No production failure masked. /tmp/jj-search-controls-after-tests.log.
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
pnpm test:storybook catalog/components/store-work/ProjectsFilters.stories.svelte (after keyboard correction)
```

**Notes**

2/2 before additional user requests: hover glyphs, keyboard focus, borderless row, drafts, submitted and draft Clear, Close and Escape. /tmp/jj-search-controls-after-keyboard.log.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-work/ProjectsFilters.stories.svelte (position before)
```

**Notes**

4px horizontal displacement on expansion; exact x/y/width/height regression added. /tmp/jj-search-controls-position-before.log.
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
pnpm test:storybook catalog/components/store-work/ProjectsFilters.stories.svelte (position after and restored outline)
```

**Notes**

Each2/2: stable position/size, 32px leading and24px Clear, outline border and background. /tmp/jj-search-controls-position-after.log; /tmp/jj-search-controls-final-tests.log. Separate processes overlapped while user refinements arrived; Vitest selected a free browser port. Final serialized combined run follows.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-work/ProjectsActivityControls.stories.svelte (before)
```

**Notes**

2failed/1passed: both hub/project Activity headers expose2filter buttons instead of1. /tmp/jj-search-controls-activity-before.log.
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
pnpm install --offline --force --no-frozen-lockfile; jj restore -- pnpm-lock.yaml
```

**Notes**

Refreshed pre-existing guarded local Design Core dependency without downloads; restored registry lockfile. No node_modules edits or new dependency alias/patch/substitution. /tmp/jj-search-controls-package-refresh.log.
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
Owner-verified UI-only restart using scripts/dev-ui-services.mjs start/ready
```

**Notes**

Vite PID88268 replaced by68050; backend91193 remains alive on7462. Existing fixture/runtime root retained; no server or RustFS restart.
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
pnpm web:check; pnpm catalog:check (initial)
```

**Notes**

0errors/0warnings;140catalog module boundaries. Final checks follow last filter refinement. /tmp/jj-search-controls-web-check.log; /tmp/jj-search-controls-catalog-check.log.
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
Design Core: pnpm test:storybook src/shared/shadcn/input-group/InputGroup.sizing.stories.svelte (before); pnpm test:storybook src/shared/shadcn/input-group (after)
```

**Notes**

Source regression1failed before (xs became default),12/12 after forwarding size as a real Button prop. All4sizes/default/square icons and Clear action. Separate source canonical/verification update and qualification logs /tmp/dc-input-group-size-before.log and /tmp/dc-input-group-size-after.log.
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
Design Core: pnpm spec:first; pnpm packages:pack; pnpm pack --out /tmp/design-core-input-group-fix.tgz
```

**Notes**

Spec-first3protected/2canonical chapters. packages:pack no-ops for already published0.1.2, so explicit pnpm pack creates source-distributed validation artifact; no publication.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
Discovery: pnpm exec visual-delta test --help in app/source; interrupted source help process
```

**Notes**

App has no visual-delta CLI; source test subcommand does not recognize --help and began execution. SIGTERM stopped only that agent-owned process; subsequent source compare uses supported exact story IDs. A guessed search path also did not exist; discovery corrected with rg.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-work/ProjectsFilters.stories.svelte catalog/components/store-work/ProjectsActivityControls.stories.svelte
```

**Notes**

5/5 pass. /tmp/jj-search-controls-combined-final.log.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-work (initial and final-before-isolation)
```

**Notes**

Each54passed/1failed: LandingAgentsAndNativeSessionNavigation could not access Projects and tasks after session selection. Exact isolated rerun1passed/3skipped. /tmp/jj-search-controls-consumers.log; /tmp/jj-search-controls-consumers-final.log; /tmp/jj-search-controls-agents-rerun.log. Temporary wait assertion did not resolve the full lane and was reverted.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-work (diagnostic wait run)
```

**Notes**

55/55 passed with temporary diagnostic catch. Not used as a substitute for restoring original assertions. /tmp/jj-search-controls-consumer-diagnostics.log.
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-work (restored assertions)
```

**Notes**

52passed/3failed: agent main region and both repository Add task keyboard stories. /tmp/jj-search-controls-consumers-restored-story.log.
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-work/ProjectsAgents.stories.svelte catalog/components/store-work/ProjectsRepositoryActions.stories.svelte (diagnostic)
```

**Notes**

7/7 passed in isolation. /tmp/jj-search-controls-agent-action-diagnostics.log.
<!-- /run -->

<!-- run -->
### Run 18

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-work (visibility diagnostics)
```

**Notes**

54passed/1failed. DOM proved main ancestor aria-hidden=true and inert=true with retained collapsed left sidebar and open outer details rail. This is the shell constrained replacement policy under leaked fixture layout, not a missing Projects region. Diagnostic removed. /tmp/jj-search-controls-consumers-visibility-diagnostics.log.
<!-- /run -->
<!-- /evidence-field -->

