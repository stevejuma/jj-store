---
schema: 1
id: E-M20-006
---

# E-M20-006 details 4

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
pnpm check:architecture
```

**Notes**

.local/target-branch-evidence/architecture-after-removal.log: 1024 modules; unchanged limits.
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
pnpm spec:first
```

**Notes**

.local/target-branch-evidence/spec-first-after-removal.log: 17 protected files, 3 canonical chapters.
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
pnpm exec prettier --check catalog/components/store-stack-workspace/StoreStackWorkspaceBranches.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceTarget.stories.svelte catalog/fixtures/StackWorkspaceApplicationFixture.svelte catalog/fixtures/stack-workspace/adapter.ts catalog/fixtures/stack-workspace/data.ts catalog/fixtures/stack-workspace/target-history.ts spec/src/api.md spec/src/ui.md spec/src/verification.md web/src/lib/controller/stack-workspace/controller.svelte.ts web/src/lib/controller/stack-workspace/files.ts web/src/lib/controller/stack-workspace/presentation.ts web/src/lib/controller/stack-workspace/target-history.svelte.ts web/src/lib/controller/stack-workspace/types.ts web/src/lib/ui/community-repository-branch-command/CommunityRepositoryBranchCommand.svelte web/src/lib/ui/store-stack-workspace/StoreStackTargetHistory.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspace.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceCanvas.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceDiff.svelte web/tests/vite/stack-workspace-target-history.test.ts
```

**Notes**

.local/target-branch-evidence/format-after-removal.log: checks all 20 changed files excluding milestones; 439 historical evidence objects preserved.
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
pnpm test:storybook catalog/components/store-stack-workspace/*.stories.svelte
```

**Notes**

.local/target-branch-evidence/workspace-stories-after-removal.log: 83/83 interaction/accessibility stories in all 17 directly affected stack-workspace files; fresh Vitest/Vite/browser process after exact regression passed.
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
pnpm check:progress
```

**Notes**

.local/target-branch-evidence/progress-after-removal.log: 141 tasks; updated E-M20-006 validated.
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

.local/target-branch-evidence/spec-validate-after-removal.log: 5 validators, 23 chapters, 173 requirements; updated E-M20-006 validated.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:stories list -->
## Stories

- catalog/components/store-stack-workspace/StoreStackWorkspaceTarget.stories.svelte#Target
  header uses the stack card layout
- catalog/components/store-stack-workspace/StoreStackWorkspaceTarget.stories.svelte#Workspace
  target supports keyboard selection above the filters
- catalog/components/store-stack-workspace/StoreStackWorkspaceTarget.stories.svelte#Target
  branch opens its own commit stack
- catalog/components/store-stack-workspace/StoreStackWorkspaceTarget.stories.svelte#Large
  target history loads lazily on scroll
- catalog/components/store-stack-workspace/StoreStackWorkspaceBranches.stories.svelte#Workspace
  target appears only in the Branches column
<!-- /evidence-field -->

<!-- evidence-field:browser_acceptance markdown -->
## Browser acceptance

Running owned Storybook PID 92150 on 6057 was retained with HMR. Fresh in-app-browser tab verified the main target card, one stack, the corrected 32px/332px header grid, and selected commit inline files. After duplicate page-header removal, the live app shell was rechecked with the target selected and entry-240.ts expanded inline. Updated screenshot: .local/target-branch-evidence/target-history-in-app-shell.png. Automated stories use fresh Vitest browser contexts. No screenshot comparison or visual baseline update.
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- UI review and visual baseline approval remain separate.
- Released registry-resolved Design Core qualification remains pending under the
  existing temporary source validation setup.
- Backend execution, physical checkout and durable workspace persistence require
  explicit approval.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Review the UI slice; proceed with local backend integration only after explicit approval.
<!-- /evidence-field -->

