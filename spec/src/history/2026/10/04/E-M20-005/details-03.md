---
schema: 1
id: E-M20-005
---

# E-M20-005 details 3

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
pnpm test:storybook catalog/components/store-stack-workspace/StoreStackWorkspaceApplication.stories.svelte -t WorkspaceInAppShell
```

**Notes**

.local/panel-toggle-evidence/exact-verified.log: Final composition 1/1 passed after cold refresh; seven unrelated stories skipped.
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
pnpm web:check
```

**Notes**

.local/panel-toggle-evidence/web-check.log: zero diagnostics
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
pnpm catalog:check
```

**Notes**

.local/panel-toggle-evidence/catalog-check.log: zero diagnostics and 217-module boundary check
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm storybook -- --port 6057
```

**Notes**

Storybook rejected the literal separator as unexpected arguments; no listener started.
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
pnpm storybook --port 6057
```

**Notes**

.local/panel-toggle-evidence/storybook-6057-retry.log, then storybook-6057-cold.log: Correct owned process restarted. Cleared only its generated sb-vite transform cache for stale source dependency transforms.
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
pnpm test:storybook catalog/components/store-stack-workspace catalog/components/community-projects/CommunityProjectsRepositoryLog*.stories.svelte catalog/components/store-application/RepositoryLayout.stories.svelte
```

**Notes**

.local/panel-toggle-evidence/affected-stories.log: 97/97 stories in all 20 affected story files. Geometry helper later moved without changing assertions to satisfy the 400-line story budget; exact app-shell regression reruns afterward.
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
pnpm check:ui-ownership
```

**Notes**

.local/panel-toggle-evidence/ownership.log: 1 validator, 23 chapters, 173 requirements.
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
pnpm check:architecture
```

**Notes**

.local/panel-toggle-evidence/architecture.log: Application story exceeded 400 counted lines; moved exact assertion group into the existing fixture helper.
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
pnpm spec:validate
```

**Notes**

.local/panel-toggle-evidence/spec-validate.log: 5 validators, 23 chapters, 173 requirements.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first
```

**Notes**

.local/panel-toggle-evidence/spec-first.log: Web presentation maps to API chapter too; added explicit no-transport contract note.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
```

**Notes**

.local/panel-toggle-evidence/progress.log: Pending run result is unsupported and remaining field was missing; corrected evidence schema without changing historical entries.
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
pnpm check:architecture
```

**Notes**

.local/panel-toggle-evidence/architecture-retry.log: 1020 modules; exact assertion group moved into existing fixture helper, preserving all checks and the 400-line story cap.
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
pnpm spec:first
```

**Notes**

.local/panel-toggle-evidence/spec-first-retry.log: 3 protected files and 3 canonical chapters.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
```

**Notes**

.local/panel-toggle-evidence/progress-retry.log: 141 tasks; evidence bookkeeping only.
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
pnpm catalog:check
```

**Notes**

.local/panel-toggle-evidence/catalog-helper-check.log: Zero errors/warnings after assertion extraction; 217 catalog modules.
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-stack-workspace/StoreStackWorkspaceApplication.stories.svelte -t WorkspaceInAppShell
```

**Notes**

.local/panel-toggle-evidence/exact-helper.log: Exact final helper-based app-shell regression passes 1/1; seven unrelated cases skipped.
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
pnpm test:storybook catalog/components/store-stack-workspace catalog/components/community-projects/CommunityProjectsRepositoryLog*.stories.svelte catalog/components/store-application/RepositoryLayout.stories.svelte
```

**Notes**

.local/panel-toggle-evidence/affected-stories-final.log: 97/97 affected cases in 20 files pass after final public hover-token fix.
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
STORYBOOK_PORT=9139 pnpm checks
```

**Notes**

Design Core .local/panel-toggle/checks.log: Intentionally interrupted (exit 130) during shell browser phase after final review found a public header-hover token precedence issue. Previously passed 715 units, 659 stories, type/format/governance, build/audit and workspace browser suite. No completed aggregate qualification claimed.
<!-- /run -->
<!-- /evidence-field -->

