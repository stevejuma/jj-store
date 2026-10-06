---
schema: 1
id: E-M21-005
---

# E-M21-005 details 2

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm catalog:test web/tests/vite/stack-workspace-setup.test.ts
```

**Notes**

Before: stack-setup-before.log. After initial nullable support: three pass in stack-setup-after.log.
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
pnpm catalog:test web/tests/vite/stack-workspace-setup.test.ts
```

**Notes**

Expanded six regressions pass in stack-setup-unit-final.log; refresh/close races, absent setup, explicit target and missing active ID included.
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
pnpm catalog:test stack-workspace
```

**Notes**

89 tests/10 files pass in stack-setup-units-final.log.
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
pnpm test:storybook catalog/components/store-stack-workspace/StoreStackWorkspaceSetup.stories.svelte
```

**Notes**

First/second run used an incorrect button selector for a Command View option; stack-setup-story-first.log and stack-setup-story-second.log. The corrected option revealed ambiguous Workspace tabs in stack-setup-story-exact.log (one failure). Scoped the tab to its owning navigation.
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
pnpm test:storybook catalog/components/store-stack-workspace/StoreStackWorkspaceSetup.stories.svelte
```

**Notes**

Two fresh interaction/accessibility stories pass; stack-setup-story-final.log.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-stack-workspace catalog/components/community-projects/CommunityProjectsRepositoryLog.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLogDiff.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLogFiles.stories.svelte catalog/components/community-repository-branch-command/CommunityRepositoryBranchCommand.stories.svelte
```

**Notes**

107 passed and one known setup selector failed in stack-setup-stories.log; run began before the selector correction.
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
pnpm test:storybook catalog/components/store-stack-workspace catalog/components/community-projects/CommunityProjectsRepositoryLog.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLogDiff.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLogFiles.stories.svelte catalog/components/community-repository-branch-command/CommunityRepositoryBranchCommand.stories.svelte
```

**Notes**

108 stories/24 files pass in fresh Vitest/Vite Chromium processes; stack-setup-stories-final.log. This is source interaction/axe evidence, not production provider or durable checkout qualification.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 29
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:check
```

**Notes**

Nullable call sites in stack-setup-web-first.log; fixed explicit guards rather than fabricated setup or weakened types.
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
pnpm web:check
```

**Notes**

Zero errors/warnings in stack-setup-web-second.log and stack-setup-web-final.log.
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
pnpm catalog:check
```

**Notes**

Zero errors/warnings; 224 catalog modules; stack-setup-catalog.log.
<!-- /run -->

<!-- run -->
### Run 11

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

314 counted controller lines exceeded the 300-line budget; extracted WorkspaceReadSession.
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

Source budget and production dependency boundaries pass; stack-setup-architecture.log.
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
pnpm check:ui-ownership
```

**Notes**

One validator passes; stack-setup-ownership.log.
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
node --test scripts/progress-mapping.test.mjs scripts/ui-ownership.test.mjs scripts/web-architecture.test.mjs
```

**Notes**

16 tests pass; stack-setup-governance.log.
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
pnpm docs:review application "Reviewed nullable workspace setup, Settings-style exact target choice, empty workspace, membership eligibility, guarded drafts and shared read-session invalidation. Target-history reset clears bounded continuation state. Production binding adapter and durable execution remain incomplete."
```

**Notes**

Source coverage updated after final source edits; stack-setup-docs-review-final.log.
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
pnpm exec prettier --write docs/maintenance/coverage.json
```

**Notes**

stack-setup-coverage-final.log. All source-specific prettier commands omit milestones.md.
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
pnpm exec prettier --check <current jj diff file list excluding spec/src/milestones.md>
```

**Notes**

Exact changed-file formatter invocation via Python subprocess; stack-setup-format-final.log. Historical ledger never formatted.
<!-- /run -->

<!-- run -->
### Run 18

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm docs:check
```

**Notes**

139 files, nine diagrams in light/dark, valid links and anchors; stack-setup-docs.log and stack-setup-docs-final.log.
<!-- /run -->
<!-- /evidence-field -->

