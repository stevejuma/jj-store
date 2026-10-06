---
schema: 1
id: E-UI-064
---

# E-UI-064 details 4

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
pnpm exec prettier --write web/tests/real/task-reactions.spec.ts spec/src/milestones.md; cargo fmt --all --check
```

**Notes**

Formatting and Rust format checks pass.
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
Design Core: pnpm checks
```

**Notes**

Final full source run passed14 workspace pointers and shell lane;2 AI story readiness timeouts. Exact AI lane retry follows, no production/test changes.
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
Design Core: pnpm test:ai-chat-browser
```

**Notes**

3/3 after cold restart; previous2 readiness failures resolved without code/test changes.
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
Design Core: pnpm test:shadcn:pointer
```

**Notes**

35 passed,1 existing skipped; remaining full source browser lane completed.
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
Design Core: pnpm exec prettier --write spec/src/verification.md; pnpm spec:check
```

**Notes**

Canonical verification row names leading-icon story; validate/mdbook/spec-first3 lanes pass.
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
Design Core: pnpm visual-delta test --help
```

**Notes**

CLI requires exactly one affected/all/story-id selector; targeted visual test used instead, no baseline writes.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/StoreWork.stories.svelte catalog/components/store-work/ProjectsActivity.stories.svelte catalog/components/store-work/ProjectsActivityControls.stories.svelte catalog/components/store-threads/StoreThreads.stories.svelte catalog/components/store-threads/StoreCapturedReactions.stories.svelte catalog/components/store-threads/StoreThreadAlignment.stories.svelte catalog/components/store-application/NativeReviewDetail.stories.svelte
```

**Notes**

25 passed; new task story clicked the delayed hover toolbar before interactive. Added the existing visible-state wait.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/StoreWork.stories.svelte -t TaskCommentReactionsAndActionSurfaces
```

**Notes**

Toolbar alignment/reactions passed; synthetic userEvent.hover does not set native CSS :hover. Preserved both real hover assertions in the cold Playwright target.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/StoreWork.stories.svelte -t TaskCommentReactionsAndActionSurfaces
```

**Notes**

Legacy StoreWork catalog host only supplies Edit; ProjectsHub supplies Add subtask. Kept both button assertions in the actual page cold test.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/StoreWork.stories.svelte -t TaskCommentReactionsAndActionSurfaces
```

**Notes**

Exact final retry1/1, other6 excluded.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/StoreWork.stories.svelte catalog/components/store-work/ProjectsActivity.stories.svelte catalog/components/store-work/ProjectsActivityControls.stories.svelte catalog/components/store-threads/StoreThreads.stories.svelte catalog/components/store-threads/StoreCapturedReactions.stories.svelte catalog/components/store-threads/StoreThreadAlignment.stories.svelte catalog/components/store-application/NativeReviewDetail.stories.svelte
```

**Notes**

Final affected interaction/accessibility families26/26 in7 files; task emoji set/clear, tombstone and readonly controls, leading Copy alignment and visible Edit rest background.
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
Design Core: pnpm visual-delta test --story-id shadcn-overlays-dropdown-menu--open-menu --story-id shadcn-overlays-dropdown-menu--submenu-leading-icon-alignment --baseline-path-mode nested-import --fresh
```

**Notes**

Canonical Linux/ARM64 container static build passed (5m44); capture stopped at EACCES copying storybook-static into mounted canonical-build-cache. No comparison or baseline writes; visual-pending.
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
Design Core: pnpm exec prettier --write spec/src/verification.md; pnpm spec:check; jj commit -m Keep dropdown submenu leading icons aligned with menu items
```

**Notes**

3 spec lanes passed. Source committed as b3eb19e7/kupxxqyt with a fresh empty change; no package publication.
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
CUA: inspect task page, open comment menu, save screenshot and Escape
```

**Notes**

Live7460 shows themed outline task actions, aligned Copy and reaction picker. Read-only inspection, no comment/reaction writes. /tmp/jj-task-actions.jpg.
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
pnpm exec prettier --write (changed supported files); pnpm spec:first; pnpm spec:validate; pnpm check:progress; pnpm check:architecture; pnpm check:ui-ownership; pnpm catalog:check
```

**Notes**

Focused final governance before retained-status refinement:34 protected files/12 chapters;5 validators/21 chapters/147 requirements;120 tasks;737 architecture modules;129 catalog modules;0 type errors/warnings. No app root aggregate lanes.
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
pnpm web:check; pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/StoreWork.stories.svelte; pnpm spec:first
```

**Notes**

Retained original status target after comment edit/removal:0 errors/warnings,7/7 task stories and spec-first pass. Picker/counts remain absent on tombstones; only original pending request recovery stays visible.
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_DEV_PORT=7484 JJ_STORE_REAL_TEST=task-reactions.spec.ts pnpm web:test:real
```

**Notes**

Setup intentionally stopped while rebuilding unchanged RustFS; no browser tests ran. Retried using the existing explicit development runtime executable. No persistent user fixture touched.
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
JJ_STORE_DEV_RUSTFS_RUNTIME=/Users/stevejuma/code/lapismd/jj-store/runtime/embedded-rustfs/target/debug/jj-store-rustfs JJ_STORE_DEV_PORT=7484 JJ_STORE_REAL_TEST=task-reactions.spec.ts pnpm web:test:real
```

**Notes**

Cold stack:1091ms startup,882ms restart,177504KiB resident,405325ms fixture preparation. Real saving/reload persistence, two-client updates, count clear, both button backgrounds/hover and Copy alignment passed. Test then clicked an opacity-hidden hover toolbar after clear and timed out at first lost-acknowledgement setup. Added the same data-actions-visible wait used by existing real WorkStore test; no force click or assertion relaxation. Isolated stack/root cleaned. Exact fresh retry follows.
<!-- /run -->
<!-- /evidence-field -->

