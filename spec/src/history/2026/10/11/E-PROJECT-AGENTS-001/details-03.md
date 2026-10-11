---
schema: 1
id: E-PROJECT-AGENTS-001
---

# E-PROJECT-AGENTS-001 details 3

<!-- evidence-field:runs runs -->

## Validation

<!-- run -->

### Run 1

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

Final rerun after timestamp guard and Online removal: one regression passes.
/tmp/jj-project-agents-final-exact.log. Independent Vitest browser process per
invocation; existing benign Svelte default-config and Node deprecation
diagnostics do not cause failures.
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
CUA background Projects Agents tab: fresh load, inspect repository group and avatar, save screenshot
```

**Notes**

Actual Add MIT license session renders beneath Agents in demo/server-only with
an avatar and no Online badge. Warm rendering initially retained the old badge;
fresh background-tab load confirmed final source. Screenshot
/tmp/jj-store-agents-grouped.jpg. No user-tab draft reset or backend rebuild.
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
pnpm web:check
```

**Notes**

Production web type check has zero errors or warnings.
/tmp/jj-project-agents-web.log.
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
pnpm exec prettier --ignore-path /dev/null --write [explicit changed files]
```

**Notes**

Targeted source and evidence formatting succeeds. No production source changes
from formatting. History paths explicitly included.
<!-- /run -->
<!-- /evidence-field -->
