---
schema: 1
id: E-CI-SETTINGS-CONTRAST-001
---

# E-CI-SETTINGS-CONTRAST-001

<!-- evidence-field:id markdown -->
## Id

E-CI-SETTINGS-CONTRAST-001
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
- JST-API-005
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/fixtures/workflow-recorded-console-play.ts
- web/src/lib/ui/store-settings/StoreSettings.css
- spec/src/history/2026/10/29/E-CI-SETTINGS-CONTRAST-001/index.md
- spec/src/history/2026/10/29/E-CI-SETTINGS-CONTRAST-001/SUMMARY.nav.md
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
gh run view 40800025132 --repo stevejuma/jj-store --log-failed
```

**Notes**

Tip 482964ca required failed on Storybook shard 2/3 WorkflowConsole RecordedStep waitFor Step console while the story was still preparing, and on Application browser shard 2/2 local-capture-settings axe color-contrast for the History import Inspect outline button at 4.13:1 on white. StoreConflictEditor NativeEditorSharesMergeHeaderSettingsAndRetainsFilteredDrafts passed on the same tip.
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
pnpm test:storybook -- catalog/components/store-workflows/WorkflowConsole.stories.svelte -t RecordedStepHasAForgejoConsole
```

**Notes**

Hardened verifyRecordedConsole to findByRole Step console with a 15s timeout before visibility waitFor. Focused story passed.
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
JJ_STORE_VALIDATION_RUNTIME=native pnpm check:progress
```

**Notes**

Progress evidence gate accepts this leaf. Settings action buttons now force foreground color so outline actions in muted settings chrome meet WCAG AA.
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

pnpm 10.32.1; Design Core 0.7.0 already pinned on the enclosing tip.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No stored format, DTO, or public API schema change. Story wait helper and settings action text color only.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Clear hosted PR 20 required failures after the conflict-sheet wait harden: WorkflowConsole Step console open under CI load, and capture Import outline-button contrast under muted settings chrome.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Push the tip, confirm Storybook shard 2/3 and Application browser shard 2/2, then merge PR 20 when required is green.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: unchanged
reason: JST-UI-002 presentation contracts are unchanged. WorkflowConsole still
  exposes Step console; settings actions still use Design Core outline buttons,
  now with explicit foreground color inside store-settings action rows.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M4.5 gates are unchanged. This repairs hosted Storybook and axe flakes
  without promoting new product gates.

```
<!-- /evidence-field -->
