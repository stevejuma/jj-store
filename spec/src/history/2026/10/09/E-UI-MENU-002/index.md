---
schema: 1
id: E-UI-MENU-002
---

# E-UI-MENU-002

<!-- evidence-field:id markdown -->
## Id

E-UI-MENU-002
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M24
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-001
- JST-UI-018
- JST-API-001
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-09
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- web/src/lib/ui/community-typography.css
- spec/src/current/contracts/ui/08-device-appearance.md
- spec/src/current/contracts/api/41-interactive-agent-execution.md
- spec/src/current/progress/M24/footer-review.md
- spec/src/current/progress/M24/history/M24/index.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

[]
<!-- /evidence-field -->

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
cua_repl workspace menu on live session eeb7049d90fcd39c6610f89c2f7102bc, before and after HMR
```

**Notes**

Before title/item both 14px and weight 400. After title is 12px, 400, muted rgb(87,94,103); item remains 14px, 400, rgb(36,41,46), same Inter sans family. Escape closes menu and restores button focus. Initial menu lookup preceded session loading and timed out; observed readiness before retry. No prompt or preference submitted.
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

0 errors, 0 warnings. Log: .local/menu-secondary-web.log.
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
pnpm test:storybook catalog/components/store-application/StoreComparisonParentPicker.stories.svelte catalog/components/store-application/WorkspaceFiles.stories.svelte catalog/components/store-mutations/StoreMutationCommitPicker.stories.svelte catalog/components/store-mutations/StoreRepositoryActions.stories.svelte catalog/components/community-repository-branch-command/CommunityRepositoryBranchCommand.stories.svelte catalog/components/store-work/ProjectsFilters.stories.svelte
```

**Notes**

25 tests across 6 files pass in a fresh test process. Existing ResizeObserver loop completed with undelivered notifications diagnostic reproduces; not suppressed or resolved. Log: .local/menu-secondary-stories.log.
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
pnpm check:ui-ownership
```

**Notes**

Shared app typography uses owned presentation and public foundation parts. Log: .local/menu-secondary-ownership.log.
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
pnpm exec prettier --check web/src/lib/ui/community-typography.css spec/src/current/contracts/ui/08-device-appearance.md
```

**Notes**

Changed CSS and appearance chapter formatting pass.
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
pnpm check:progress
```

**Notes**

146 tasks; 711 evidence records.
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

1 protected file, 2 owning chapters.
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
pnpm exec spec-validator validate --only governance,summary,fileLimits,book
```

**Notes**

4 validators, 3392 chapters, 181 requirements.
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
pnpm exec prettier --check web/src/lib/ui/community-typography.css spec/src/current/contracts/ui/08-device-appearance.md spec/src/current/contracts/api/41-interactive-agent-execution.md spec/src/current/progress/M24/footer-review.md
```

**Notes**

Final changed source and chapters pass formatting; footer review was formatted after navigation generation.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- M24 remains In progress. Existing ResizeObserver diagnostics and independently
  governed workspace publication/recovery gates remain open; visual baselines
  are unapproved.
- Rust, root aggregate and full-catalog lanes are out of scope for this CSS-only
  slice. Main-workspace integration and cold server handoff are recorded
  separately.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

JJ 0.45.1, pnpm 10.32.1, Vitest 4.1.11, Svelte 5.57.1; macOS ARM64. Revision is the enclosing Jujutsu change.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No stored, API, source-anchor, preference or request formats change.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Community branch picker inspected first, Design Core Popover/DropdownMenu second. App-owned menu titles now use secondary text size, regular weight and semantic muted foreground. Menu items, dialog/page headings, focus, selection and action behavior are unchanged.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Land the chat-body stack into main and restart there while preserving the existing fixture and settings; retain independent M24 gates.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-UI-001/018 specify secondary-sized muted regular menu titles in the
  owning appearance chapter. JST-API-001 remains unchanged, with explicit
  confirmation in the execution chapter.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M24 stays In progress. Focused menu interaction and live style checks do
  not close wider qualification or visual approval.

```
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


