---
schema: 1
id: E-WORKFLOW-HISTORY-LAYOUT-001
---

# E-WORKFLOW-HISTORY-LAYOUT-001 details 2

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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/workflow-history.test.ts web/tests/vite/workflows-route.test.ts
```

**Notes**

Before: 7 existing tests passed. Existing History presented Mine/Other groups with a filter popover, no repository workflow body panel, and task-shaped rows.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/workflow-history.test.ts web/tests/vite/workflows-route.test.ts web/tests/vite/workflow-history-presentation.test.ts
```

**Notes**

After: 11 tests passed. Recorded requester/manual-event/bookmark filtering, duplicate workflow IDs in distinct homes, exact uint64 pins, missing timestamps and current attempt durations; pinned continuation and uncertain command regressions retained.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-workflows/WorkflowHistory.stories.svelte catalog/components/store-workflows/WorkflowHistoryNavigation.stories.svelte
```

**Notes**

Final cold-process run: 14 interaction/Axe stories passed. Initial run failed to import one suite after new icon dependency optimization and reported four failed tests plus three shell-context errors. Added real fixture shell context and removed unsupported Testing Library exact options (five type errors). Follow-ups exposed three then one interaction failures: continuation refresh lost an in-flight requested window, menu dismissal retained pointer suppression briefly, and a mobile test had desktop width. Fixed window retention, awaited menu teardown and used an actual 390px shell. A new hover assertion failed because synthetic events do not set CSS :hover; keyboard focus styling is covered instead. All functional failures resolved.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-workflows/WorkflowHistoryNavigation.stories.svelte -t RepositoryWorkflowNavigation
```

**Notes**

Exact continuation regression passed after retaining the requested 50-row window through catalog refresh and comparing owner submit functions rather than Svelte proxy identities. Earlier focused runs failed this regression; three other stories were deliberately skipped.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-workflows/*.stories.svelte
```

**Notes**

53 interaction/Axe stories passed across 11 files. Escalated to all workflow stories because the shared fixture now supplies shell context. Final narrow rerun covers subsequent local CSS cleanup/chevron hover styling.
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
pnpm web:check
```

**Notes**

0 errors, 0 warnings.
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
pnpm catalog:check
```

**Notes**

0 errors, 0 warnings; 438-module catalog boundary check passed.
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
pnpm check:architecture
```

**Notes**

1868 modules passed. An intermediate stylesheet exceeded the 300 counted-line limit; removed obsolete and duplicate selectors without increasing the budget.
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
pnpm check:ui-ownership
```

**Notes**

Released Design Core and maintained owned Community components only; no upstream source/runtime/type imports.
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
node scripts/docs.mjs review application "Reviewed repository-scoped workflow History navigation, flat author/bookmark/status/trigger filters, aligned avatars, full-width pinned details and continuation retention; updated the browsing guide."
```

**Notes**

Application coverage fingerprint refreshed and formatted.
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
pnpm docs:check
```

**Notes**

143-file documentation closure built; links and anchors valid.
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
pnpm check:progress
```

**Notes**

153 tasks and 792 evidence records passed. Removed unsupported test-reference/glob values from the evidence schema; exact executable cases remain recorded in commands.
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
pnpm spec:validate
```

**Notes**

7 validators, 3779 chapters and 196 requirements passed.
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
pnpm exec prettier --write changed UI/catalog/docs/spec files
```

**Notes**

Only the named change files were formatted; no aggregate formatter or unrelated files.
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
CUA: inspect live History, repository scope, divider keyboard resize, and selected completed run
```

**Notes**

Preserved dev service on 127.0.0.1:7460. Repository selection showed its recorded runs; ArrowRight grew the sidebar by 16px and ArrowLeft restored it. Completed a035c8a4 details had no workflow sidebar and retained metadata/step console. A fresh background History tab showed six currently loaded runs and saved .local/workflow-history-layout.png. An attempted lookup of the earlier Persistence test run was absent in that fresh list; no stored data was modified or inferred from the older tab. Original user tab is preserved.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Visual baseline approval is separate. M28 live agents, remote Forgejo
  registration/polling/ConnectRPC and additional platform gates remain
  unchanged.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64; JJ 0.45.1; Node 24.10.0; pnpm 10.32.1; Svelte 5.57.1; Design Core 0.3.1; Vitest 4.1.11; Storybook 10.5.7.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Frontend WorkflowSelection hash fields add author, trigger origin and exact bookmark; skipped status is selectable. Native WorkflowStore, APIs, CLI, receipts, publication pins and evidence formats are unchanged.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

White full-height source-repository/workflow sidebar, All workflows, selectable repository groups, separate chevrons, bounded pointer/keyboard resize, flat runs with aligned status/avatar columns, URL filters, full-width details and representative skeletons. Existing committed-event/reconnect/fallback refresh and toast feedback retained.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Keep M28 In progress; qualify the separate provider, remote protocol/platform and visual gates. Add repository Actions navigation as a separately verified requested slice.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: Updated JST-WF-008 presentation in UI chapter 41, owned provenance and
  browsing guide. JST-UI-019 feedback and native execution/storage/API authority
  unchanged.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M28 remains In progress; this evidence qualifies the requested History
  presentation and continuation behavior, not execution/provider/remote protocol
  or approved visual parity.

```
<!-- /evidence-field -->

