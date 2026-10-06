---
schema: 1
id: E-M20-005
---

# E-M20-005 details 4

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
STORYBOOK_PORT=9139 pnpm test:shadcn:pointer tests/shadcn/panel-toggle-pointer.spec.ts -g 'column header hover'
```

**Notes**

Design Core .local/panel-toggle/hover-before.log: Exact hover-token regression fails because Panel Toggle paint masks the custom canvas background token.
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
STORYBOOK_PORT=9139 pnpm test:shadcn:pointer tests/shadcn/panel-toggle-pointer.spec.ts
```

**Notes**

Design Core .local/panel-toggle/pointer-qualified.log: 5/5 native cases pass after lowering ghost paint specificity. Public header hover colors, controlled geometry, native focus, disabled behavior, reduced motion and forced colors retained.
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
STORYBOOK_PORT=9139 pnpm checks
```

**Notes**

Design Core .local/panel-toggle/checks-final.log: complete required nonvisual gate passes: 715 units, 659 stories, type/format/governance, static build/audit, 14 workspace, 11 shell, 3 AI and 41 shadcn browser cases; one intentional platform skip. Existing nonfatal proxy/ResizeObserver diagnostics remain in unrelated stories; no assertion/timeouts relaxed.
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
pnpm exec prettier --check catalog/components/store-stack-workspace/StoreStackWorkspaceApplication.stories.svelte catalog/fixtures/stack-workspace/story-actions.ts web/src/lib/ui/store-stack-workspace/StoreStackUnstaged.svelte spec/src/ui.md spec/src/api.md spec/src/verification.md
```

**Notes**

.local/panel-toggle-evidence/format-check.log: changed consumer files formatted; milestones.md intentionally excluded to preserve all historical raw evidence objects.
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

.local/panel-toggle-evidence/storybook-6057-final.log: final owned preview started; live browser verified compact geometry after source-theme fix.
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

.local/panel-toggle-evidence/progress-final.log: 141 tasks; final source and consumer evidence accepted.
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
pnpm spec:validate
```

**Notes**

.local/panel-toggle-evidence/spec-validate-final.log: 5 validators, 23 chapters, 173 requirements.
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
pnpm spec:first
```

**Notes**

.local/panel-toggle-evidence/spec-first-final.log: 3 protected files and 3 canonical chapters.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:source_acceptance markdown -->
## Source acceptance

Design Core committed qwzmvmkk / bab5dc582e33 after required complete nonvisual qualification. Final gate passes 715 units, 659 Storybook interaction/accessibility cases, type/format/governance, static build/audit, 14 workspace, 11 shell, 3 AI and 41 shadcn browser cases; one intentional platform-specific skip. Five new native panel tests include real hover/focus, reversed bar widths without state/geometry mutation, activation, right orientation, disabled controls, reduced motion, forced colors and public header hover colors. Registry-resolved released acceptance remains separately pending. No assertions or timeouts weakened.
<!-- /evidence-field -->

<!-- evidence-field:browser_acceptance markdown -->
## Browser acceptance

Owned Storybook 6057 restarted after the final source hover-token fix, clearing only its generated sb-vite cache. Fresh final app-shell page measures one Unstaged header, a 28×24px button and 18×12px icon; initial fresh proof also exercised collapse/expand with correct Unstaged rail label. Final proof saved .local/panel-toggle-evidence/workspace-compact-toggles-final.png. Temporary final proof tab closed. Earlier first-tab reload/reconnect timed out; a fresh tab verified the final result. Owned Storybook 6057 remains running at PID 92150.
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Backend and physical checkout integration still require explicit user approval.
- Published registry-resolved foundation qualification remains pending.
- No screenshot comparison, baseline update, package publication or
  service/backend rebuild.
<!-- /evidence-field -->

