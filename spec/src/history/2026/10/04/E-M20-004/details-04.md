---
schema: 1
id: E-M20-004
---

# E-M20-004 details 4

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
STORYBOOK_PORT=9139 pnpm test:ai-chat-browser -g 'uses native clipboard|working shimmer' --workers=1
```

**Notes**

/Users/stevejuma/code/lapismd/design-core/.local/trailing-spacer/ai-exact-retry.log: Both exact unchanged AI cases passed on a serial cold run; timeout/assertions were not changed.
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
pnpm test:storybook catalog/components/store-stack-workspace/StoreStackWorkspaceSpacer.stories.svelte -t CenteredBranchPromptOccupiesTheExistingSpacer --reporter=verbose
```

**Notes**

.local/stack-spacer-evidence/spacer-clean-before.log: Requested spacer width regression reproduced 240px versus 390px; four skipped.
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
pnpm test:storybook catalog/components/store-stack-workspace/StoreStackWorkspaceSpacer.stories.svelte --reporter=verbose
```

**Notes**

.local/stack-spacer-evidence/spacer-clean-after.log: 5/5 passed, including measured 390px workspace width, no background image or leading border, vertical centering and unchanged blank 240px Branches reserve.
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
pnpm exec prettier --write web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceCanvas.svelte web/src/lib/ui/store-stack-workspace/stack-workspace-layout.css catalog/components/store-stack-workspace/StoreStackWorkspaceSpacer.stories.svelte spec/src/ui.md
```

**Notes**

.local/stack-spacer-evidence/spacer-clean-format.log: Named changed presentation paths only; ledger excluded.
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
pnpm test:storybook catalog/components/store-stack-workspace -t '^(?!CommitPanelGapMatches)' --reporter=verbose
```

**Notes**

.local/stack-spacer-evidence/spacer-clean-stories.log: 77 passed; app-shell story still asserted old 240px workspace spacer width. Updated to the requested 390px. Spacing regression intentionally skipped while pending its separate narrow baseline.
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
pnpm test:storybook catalog/components/store-stack-workspace/StoreStackWorkspaceSpacing.stories.svelte --reporter=verbose
```

**Notes**

.local/stack-spacer-evidence/spacing-before.log: Exact geometry regression measured 8px from composer to first branch versus 12px between branches.
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
pnpm test:storybook catalog/components/store-stack-workspace/StoreStackWorkspaceSpacing.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceApplication.stories.svelte -t 'CommitPanelGapMatches|WorkspaceInAppShell' --reporter=verbose
```

**Notes**

.local/stack-spacer-evidence/spacing-and-shell-after.log: Both exact regressions passed; 7 skipped. Measured 12px gaps before and after opening the commit form, and 390px spacer in the real app shell.
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
pnpm web:check
```

**Notes**

.local/stack-spacer-evidence/spacer-clean-types.log: Zero errors/warnings after the workspace-specific width change.
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
pnpm catalog:check
```

**Notes**

.local/stack-spacer-evidence/spacer-clean-catalog.log: Zero errors/warnings; 217 catalog modules including new spacing story.
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
pnpm exec prettier --write catalog/components/store-stack-workspace/StoreStackWorkspaceApplication.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceSpacing.stories.svelte web/src/lib/ui/store-stack-workspace/stack-workspace.css spec/src/ui.md
```

**Notes**

.local/stack-spacer-evidence/spacing-format.log: Named paths only; historical ledger excluded.
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
STORYBOOK_PORT=9139 pnpm checks
```

**Notes**

/Users/stevejuma/code/lapismd/design-core/.local/trailing-spacer/checks-completion.log: Governance, quality, 715 units, 653 stories, build/audit, 14 workspace, 11 shell and 3 AI browsers passed. Shadcn: 35 passed, one platform skip and one wheel animation marker timeout at column-canvas-scroll.spec.ts:1389. Exact unchanged case rerun next.
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
STORYBOOK_PORT=9139 pnpm test:shadcn:pointer -g 'resizes the trailing pair inversely'
```

**Notes**

/Users/stevejuma/code/lapismd/design-core/.local/trailing-spacer/shadcn-exact-retry.log: Exact unchanged wheel-animation reproduction passed cold; all assertions/timeouts retained.
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
pnpm test:storybook catalog/components/store-stack-workspace --reporter=verbose
```

**Notes**

.local/stack-spacer-evidence/workspace-final-stories.log: 79/79 in all 16 directly affected workspace story families passed. Includes exact 12px closed/open commit-panel geometry, 390px plain spacer, app shell, resize, drop guides, drag/sort, recovery and branch actions. Not a full repository suite.
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
pnpm spec:validate
```

**Notes**

.local/stack-spacer-evidence/workspace-final-spec-validate.log: 5 validators, 23 chapters and 173 requirements.
<!-- /run -->

<!-- run -->
### Run 15

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

.local/stack-spacer-evidence/workspace-final-spec-first.log: Final provenance change additionally maps to spec-governance.md; added the owning original composition and separate source/registry qualification rationale.
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
pnpm check:architecture
```

**Notes**

.local/stack-spacer-evidence/workspace-final-architecture.log: 1020 modules; no source budget violations.
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
pnpm check:progress
```

**Notes**

.local/stack-spacer-evidence/workspace-final-progress.log: 141 tasks; evidence bookkeeping only.
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
pnpm exec prettier --write spec/src/spec-governance.md spec/src/verification.md
```

**Notes**

.local/stack-spacer-evidence/workspace-final-canonical-format.log: Formatted named canonical chapters only; historical evidence JSON untouched.
<!-- /run -->
<!-- /evidence-field -->

