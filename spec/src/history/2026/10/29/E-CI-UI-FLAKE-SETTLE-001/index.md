---
schema: 1
id: E-CI-UI-FLAKE-SETTLE-001
---

# E-CI-UI-FLAKE-SETTLE-001

<!-- evidence-field:id markdown -->
## Id

E-CI-UI-FLAKE-SETTLE-001
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-29
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M4.5
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-002
- JST-UI-016
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-appearance/AppearanceShell.stories.svelte
- web/tests/browser/local-capture-settings.spec.ts
- spec/src/history/2026/10/29/E-CI-UI-FLAKE-SETTLE-001/index.md
- spec/src/history/2026/10/29/E-CI-UI-FLAKE-SETTLE-001/SUMMARY.nav.md
- spec/src/history/2026/10/29/index.md
- spec/src/history/2026/10/29/SUMMARY.nav.md
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
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
gh run view 40899403746 --repo stevejuma/jj-store --log-failed
```

**Notes**

Hosted PR 24 (dependabot cargo bump; Cargo manifests and lock only) failed UI
acceptance twice: Storybook shard 1/3 AppearanceShell
BuzzNavigationPickerHasAppSurfaceAndRailClearance expected the compact section
trigger to regain focus after Escape while the popover was still open with
focus on its combobox input, and Application browser shard 2/2
local-capture-settings axe color-contrast reported 3.04 (foreground #929497 on
#ffffff) for the enabled Refresh import status outline button.
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
node scripts/validation.mjs run test:storybook -- catalog/components/store-appearance/AppearanceShell.stories.svelte
```

**Notes**

External docker validation (omarchy) at PR head b97f3bd8 before the fix:
10/10 including the picker story. The hosted failure is a timing flake, not a
dependency-bump regression.
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
node scripts/validation.mjs run web:test:browser -- web/tests/browser/local-capture-settings.spec.ts
```

**Notes**

External docker validation at PR head before the fix: 5/5. The full image also
compiled the bumped Rust workspace (tokio 1.54.1, tantivy 0.27) cleanly, so the
cargo group itself qualifies for the browser lane.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
node scripts/validation.mjs run web:test:browser -- web/tests/browser/local-capture-settings.spec.ts --repeat-each=25 --workers=1
```

**Notes**

Pre-fix flake baseline: 2 of 25 iterations of cold capture controls preserve
intent and snapshot failed the same axe color-contrast rule (foreground
#7b7e81, 4.08:1). Trace and network analysis show background capture refresh
cycles (observations refresh sets state.busy; an SSE reconnect notice arriving
mid-tick latches a follow-up cycle through automatic reschedule) re-disabling
and re-enabling the outline button while AxeBuilder samples, so the sample
lands inside the 150ms disabled-opacity fade and reads a blended foreground.
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
node scripts/validation.mjs run test:storybook -- catalog/components/store-appearance/AppearanceShell.stories.svelte
```

**Notes**

Post-fix, twice in sequence: 10/10 each. The picker play now waits for the
section picker's mount-focus handoff (combobox input toHaveFocus) before
sending Escape, so dismissal and trigger focus restore are deterministic.
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
JJ_STORE_VALIDATION_RUNTIME=native pnpm check:progress
```

**Notes**

Progress evidence gate accepts this leaf with the new history record
(186 tasks; 994 evidence records).
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
node scripts/validation.mjs run web:test:browser -- web/tests/browser/local-capture-settings.spec.ts --repeat-each=25 --workers=1
```

**Notes**

Post-fix repeat of the Run 4 flake baseline on external docker: 125/125 in
8.0m, zero axe color-contrast violations. The spec now waits for capture
request quiescence (1.2s without a new /v1/local/capture request, longer than
the refresh latch delay plus one burst) before each axe sample, so sampling
sees the settled enabled view; disabled controls remain excluded by axe and
settled colors stay fully checked.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pass
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

[]
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

pnpm 10.32.1; Playwright 1.63.0; validation images
jj-store-validation ui/full 2e6e87fc803db5eb098c on the omarchy docker host.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No stored format, DTO, or public API schema change. Test timing hardening only.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Clear the two hosted UI acceptance flakes blocking PR 24 without changing
product behavior: navigation picker focus handoff in the AppearanceShell
picker play, and axe sampling quiescence in local-capture-settings.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Recorded the post-fix browser repeat as Run 7. Confirm the hosted Storybook
and Application browser lanes of the PR 24 re-run go green.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: unchanged
reason: JST-UI-002 focus and navigation mechanics and JST-UI-016 settings
  presentation are unchanged. The story still asserts popover dismissal and
  trigger focus restore; the spec still asserts zero axe violations on the
  settled view and keeps every existing assertion.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M4.5 gates are unchanged. This hardens hosted Storybook and axe
  sampling timing without promoting new product gates.

```
<!-- /evidence-field -->
