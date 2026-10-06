---
schema: 1
id: E-M20-006
---

# E-M20-006 details 3

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
pnpm check:ui-ownership
```

**Notes**

.local/target-branch-evidence/ownership.log: 1 validator, 23 chapters, 173 requirements.
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
pnpm test:storybook catalog/components/store-stack-workspace/*.stories.svelte catalog/components/community-repository-branch-command/CommunityRepositoryBranchCommand.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLog*.stories.svelte
```

**Notes**

.local/target-branch-evidence/affected-stories-final.log: 103/103 interactions/accessibility in all 21 affected files. Fresh Vitest-owned Vite/browser process; target geometry and existing body header both pass.
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

.local/target-branch-evidence/web-check-qualified.log: zero errors and warnings after the final presentation correction.
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
pnpm catalog:check
```

**Notes**

.local/target-branch-evidence/catalog-check-qualified.log: zero errors and warnings, 219 catalog modules.
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
pnpm check:architecture
```

**Notes**

.local/target-branch-evidence/architecture-qualified.log: 1024 modules, unchanged architectural limits.
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
pnpm spec:validate
```

**Notes**

.local/target-branch-evidence/spec-validate.log: 5 validators, 23 chapters, 173 requirements.
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
pnpm spec:first
```

**Notes**

.local/target-branch-evidence/spec-first.log: 16 protected files, 3 canonical chapters.
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
pnpm check:progress
```

**Notes**

.local/target-branch-evidence/progress.log: 141 tasks; evidence bookkeeping only.
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
pnpm exec prettier --check catalog/components/store-stack-workspace/StoreStackWorkspaceBranches.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceTarget.stories.svelte catalog/fixtures/StackWorkspaceApplicationFixture.svelte catalog/fixtures/stack-workspace/adapter.ts catalog/fixtures/stack-workspace/data.ts catalog/fixtures/stack-workspace/target-history.ts spec/src/api.md spec/src/ui.md spec/src/verification.md web/src/lib/controller/stack-workspace/controller.svelte.ts web/src/lib/controller/stack-workspace/files.ts web/src/lib/controller/stack-workspace/presentation.ts web/src/lib/controller/stack-workspace/target-history.svelte.ts web/src/lib/controller/stack-workspace/types.ts web/src/lib/ui/community-repository-branch-command/CommunityRepositoryBranchCommand.svelte web/src/lib/ui/store-stack-workspace/StoreStackTargetHistory.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceCanvas.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceDiff.svelte web/tests/vite/stack-workspace-target-history.test.ts
```

**Notes**

.local/target-branch-evidence/format-check.log: all 19 changed code/story/test/spec files pass; milestones.md deliberately excluded to preserve 439 historical raw evidence objects.
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
pnpm check:progress
```

**Notes**

.local/target-branch-evidence/progress-final.log: 141 tasks; completed prior final-ledger validation.
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
pnpm spec:validate
```

**Notes**

.local/target-branch-evidence/spec-validate-final.log: 5 validators, 23 chapters, 173 requirements; completed prior final-ledger validation.
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
pnpm test:storybook catalog/components/store-stack-workspace/StoreStackWorkspaceBranches.stories.svelte -t 'Workspace target appears only in the Branches column'
```

**Notes**

.local/target-branch-evidence/target-row-before.log: 3 skipped; human-readable title did not match the generated export name. No regression qualification; corrected filter in next run.
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
pnpm test:storybook catalog/components/store-stack-workspace/StoreStackWorkspaceBranches.stories.svelte -t WorkspaceTargetAppearsOnlyInTheBranchesColumn
```

**Notes**

.local/target-branch-evidence/target-row-before-exact.log: 1 failed, 2 skipped; exact regression detects the duplicate page header before source removal.
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
pnpm exec prettier --write catalog/components/store-stack-workspace/StoreStackWorkspaceBranches.stories.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspace.svelte spec/src/ui.md spec/src/verification.md
```

**Notes**

.local/target-branch-evidence/target-row-format.log: changed presentation, story and canonical text formatted; historical ledger excluded.
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
pnpm test:storybook catalog/components/store-stack-workspace/StoreStackWorkspaceBranches.stories.svelte -t WorkspaceTargetAppearsOnlyInTheBranchesColumn
```

**Notes**

.local/target-branch-evidence/target-row-after-exact.log: 1 passed, 2 skipped after source removal; retained target card and tab round trip verified.
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
pnpm web:check
```

**Notes**

.local/target-branch-evidence/web-check-after-removal.log: zero errors and warnings.
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
pnpm catalog:check
```

**Notes**

.local/target-branch-evidence/catalog-check-after-removal.log: zero errors and warnings; 219 modules.
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
pnpm check:ui-ownership
```

**Notes**

.local/target-branch-evidence/ownership-after-removal.log: 1 validator, 23 chapters, 173 requirements.
<!-- /run -->
<!-- /evidence-field -->

