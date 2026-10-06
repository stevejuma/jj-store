---
schema: 1
id: E-M20-003
---

# E-M20-003

<!-- evidence-field:id markdown -->
## Id

E-M20-003
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M20
- M20.1
- M20.2
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-04
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change xounzrpv
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

commit-row-resize-regression
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

JJ 0.45.1, Node 26.9.0, pnpm 10.32.1, Svelte 5.57.1, Storybook 10.5.7, Vitest 4.1.11
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Presentation CSS only; no contract or persisted format change.
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Fix the owned revision graph action-row copy flex basis so resizing across the 440px breakpoint does not turn a horizontal 12rem reserve into vertical height. Before: widening the stack grew a commit row from 88.109px to 240.578px. After: widening and restoring the column retains natural measured heights. The existing general repository graph layout retains its horizontal reserves. Preserve all 436 historical evidence objects byte-for-byte. Root aggregates, Rust and production bundle lanes are out of scope for this UI-only slice.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- web/src/lib/ui/community-projects/CommunityProjectsRevisionGraph.svelte
- catalog/components/store-stack-workspace/StoreStackWorkspaceRefinements.stories.svelte
- spec/src/ui.md
- spec/src/verification.md
- spec/src/milestones.md
- spec/src/api.md
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
pnpm test:storybook catalog/components/store-stack-workspace/StoreStackWorkspaceRefinements.stories.svelte -t ResizingStackColumnsKeepsNaturalCommitRowHeights --reporter=verbose
```

**Notes**

.local/stack-spacer-evidence/resize-before.log: One measured resize regression failed.
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
pnpm test:storybook catalog/components/store-stack-workspace/StoreStackWorkspaceRefinements.stories.svelte -t ResizingStackColumnsKeepsNaturalCommitRowHeights --reporter=verbose
```

**Notes**

.local/stack-spacer-evidence/resize-after.log: Exact regression passed; 10 skipped.
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
pnpm test:storybook catalog/components/community-projects/CommunityProjectsRevisionGraph.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLog.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceRefinements.stories.svelte --reporter=verbose
```

**Notes**

.local/stack-spacer-evidence/resize-stories.log: 24/24 directly affected scenarios in 3 files passed interaction/accessibility.
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

.local/stack-spacer-evidence/resize-types.log: Zero errors/warnings.
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
pnpm catalog:check
```

**Notes**

.local/stack-spacer-evidence/resize-catalog.log: Zero errors/warnings; 214 catalog modules.
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
pnpm exec prettier --write web/src/lib/ui/community-projects/CommunityProjectsRevisionGraph.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceRefinements.stories.svelte spec/src/ui.md
```

**Notes**

.local/stack-spacer-evidence/resize-format.log: Formatted only named files.
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
pnpm exec prettier --write spec/src/verification.md
```

**Notes**

.local/stack-spacer-evidence/resize-verification-format.log: Named verification map formatted.
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
pnpm spec:validate
```

**Notes**

.local/stack-spacer-evidence/resize-spec-validate.log: 5 validators, 23 chapters, 173 requirements passed.
<!-- /run -->

<!-- run -->
### Run 9

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

.local/stack-spacer-evidence/resize-spec-first.log: required API chapter missing from mapped protected graph change; repaired with explicit presentation-state boundary clarification.
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

.local/stack-spacer-evidence/resize-check-progress.log: 141 tasks; bookkeeping only.
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
pnpm check:ui-ownership
```

**Notes**

.local/stack-spacer-evidence/resize-check-ui-ownership.log: Owned presentation boundary passed.
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
pnpm exec prettier --write spec/src/api.md
```

**Notes**

.local/stack-spacer-evidence/resize-api-format.log: API chapter formatted.
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
pnpm spec:first --base @-- --head @-
```

**Notes**

.local/stack-spacer-evidence/resize-spec-first-qualified.log: checker expects concrete Git commit IDs for explicit ranges, not JJ revsets.
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
pnpm spec:first --base 044165d6 --head 617f6bd6 (full IDs resolved by jj log)
```

**Notes**

.local/stack-spacer-evidence/resize-spec-first-committed.log: explicit-range checker uses Git diff, which does not access this non-colocated JJ workspace; use the normal JJ-aware local gate in the owning change.
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
pnpm spec:first (in owning JJ change xounzrpv)
```

**Notes**

.local/stack-spacer-evidence/resize-spec-first-local.log: 2 protected files and 3 canonical chapters passed on corrected full slice.
<!-- /run -->
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M20-003 details 2](details-02.md)
