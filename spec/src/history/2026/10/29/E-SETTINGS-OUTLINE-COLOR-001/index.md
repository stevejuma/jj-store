---
schema: 1
id: E-SETTINGS-OUTLINE-COLOR-001
---

# E-SETTINGS-OUTLINE-COLOR-001

<!-- evidence-field:id markdown -->
## Id

E-SETTINGS-OUTLINE-COLOR-001
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
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- web/src/lib/ui/store-settings/StoreSettings.css
- spec/src/history/2026/10/29/E-SETTINGS-OUTLINE-COLOR-001/index.md
- spec/src/history/2026/10/29/E-SETTINGS-OUTLINE-COLOR-001/SUMMARY.nav.md
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
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
gh run view 40806327364 --repo stevejuma/jj-store --log-failed
```

**Notes**

Tip aa1a8e67 forced foreground color on every settings action button, including default Save compute settings, so Storybook a11y reported 2.39:1 purple primary contrast and all LocalCompute stories failed.
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
pnpm test:storybook -- catalog/components/store-application/LocalCompute.stories.svelte -t SharedSaveFooter
```

**Notes**

Limited the foreground override to outline and ghost variants only. SharedSaveFooterStaysSeparatedOnEveryComputeTab passed.
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

Progress evidence gate accepts this leaf.
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

pnpm 10.32.1.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No stored format, DTO, or public API schema change. Settings action CSS selector scope only.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Restore primary settings action contrast while keeping outline and ghost labels on theme foreground under muted settings chrome.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Push and confirm Storybook plus Application browser required lanes on PR 20.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: unchanged
reason: JST-UI-002 settings actions still use Design Core button variants. Only
  the local foreground override is limited to outline and ghost.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M4.5 unchanged. Selector scope repair only.

```
<!-- /evidence-field -->
