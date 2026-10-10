---
schema: 1
id: E-INBOX-CONTROLS-001
---

# E-INBOX-CONTROLS-001

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
pnpm progress:history
pnpm check:progress
```

**Notes**

First record was 480 lines and still marked pending; navigation generation refused the oversized authored index and progress validation refused its invalid result and missing coverage. Re-rendered through the owned evidence writer into bounded pages, supplied actual accepted results and coverage, then re-ran these gates. Unrelated generated navigation churn is restored.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook --browser.api.port 63421 --browser.api.strictPort catalog/components/store-application/InboxControls.stories.svelte catalog/components/store-application/ResponsiveNavigation.stories.svelte
```

**Notes**

Final measurement-observer change rechecked cold: all 7 focused stories passed, 20.97s. /tmp/inbox-last-measurement.log. Natural probe widths also refit when fonts/content change; existing ResizeObserver warning persists.
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
pnpm web:check
pnpm catalog:check
```

**Notes**

Latest source: both Svelte checks pass with 0 errors/warnings; catalog ownership covers 364 modules. /tmp/inbox-web-verified.log and /tmp/inbox-catalog-verified.log.
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
pnpm progress:history
pnpm spec:validate --only summary,fileLimits,governance,verification,book
```

**Notes**

Bounded evidence/history generation and 5 document validators pass. /tmp/inbox-history-verified.log and /tmp/inbox-spec-verified.log. Independent visual/provider/ResizeObserver qualification stays in the milestone review and scope, rather than being reported as gaps in this accepted UI slice.
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
pnpm spec:first --base 94f1cf6c --head b9bd435297a6e0b87e44dc66b13048e51d1b79f1
pnpm exec prettier --check docs/maintenance/coverage.json spec/src/current/progress/M24/index.md spec/src/current/progress/M24/history/M24/index.md spec/src/current/progress/M24/SUMMARY.index-index/002/index.md
```

**Notes**

Progress passes at 146 tasks/729 records. Spec-first passes for 36 protected files and 6 chapters. Broad changed-file formatting initially flagged these four generated/review bookkeeping files; formatted them and the exact repaired check passes. Logs: /tmp/inbox-progress-complete.log, /tmp/inbox-spec-first.log, /tmp/inbox-format-repaired.log. Actual fresh revision is used by the final gates; no future commit hash is referenced by the milestone review.
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

Node 26.9.0, pnpm 10.32.1, JJ 0.45.1, Svelte 5.57.1, Vitest 4.1.11, Storybook 10.5.7; macOS ARM64. Current released Design Core and Lucide dependencies retained.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Device-local preferences use jj-store/inbox/v1/thread and jj-store/inbox/v1/session: version 1, canonical pinned/archived ID arrays, each bounded to 1024 IDs of at most 512 characters. No content, drafts, shared lifecycle or native persistence formats change.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Default workspace, enclosing change mlrnlkzl. One UI presentation surface spans owned navigation and inbox consumers. Shared tabs retain panel context, disabled sections and selection; all tabs enter More at no-fit widths. Search and exact loaded-only filters perform no reads. Both inboxes have one 52px header with 8px start inset, icon Empty states, absolute 24px joined Pin/Archive controls, a leading canonical Pinned group and local archive restoration. Footer repository pickers use secondary rest/hover surfaces. Threads hub highlighting survives conversation navigation. Natural-width fitting uses inert probes and untracked callbacks, avoiding the feedback loop seen during development.

Live browser verification retained the default Vite/backend fixture at ports 7460/7462. Fresh-load Threads and Sessions were resized between 224px and 480px, no-fit menus included All, search/filter state and selections survived widening, and both Empty states contained SVG icons. Projects used More at 520px and restored full tabs at normal width. Pin/reload, canonical multi-repo deduplication and archive/restore were exercised with Coordinate two codebases; no service mutation was dispatched. Final live geometry confirmed 24px actions, joined inner square/outer round corners, grey backgrounds and unchanged title widths. Thread/Agent repositories had nontransparent resting surfaces, footer hover changed its background, and Threads carried aria-current=page with a conversation open. .local/inbox-controls-final.png is ignored review evidence, not a baseline.

Test escalation covers direct shared tab-strip and inbox consumers, including WorkspaceFiles, ProjectsHub, legacy Work, Settings, Stacks, Threads and agent navigation. Rust, root aggregate/build lanes and native cold publication are out of scope for this UI-only slice; no API or contract generation changes. The warning previously observed during shell resizing is rechecked and is not claimed resolved. Read-only inspections and tool reconnect timeouts are diagnostic, not passing browser acceptance.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Retain M24 provider/lifecycle and visual gates; separately qualify the existing ResizeObserver diagnostic.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-UI-006/JST-UI-012/JST-UI-015 responsive navigation and inbox
  presentation are updated in their owning chapters. JST-API-001 read
  identities, cache, events and publication authority remain unchanged.
  JST-DOC-001 manual and application coverage are updated.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M24 remains In progress. This presentation slice does not qualify
  installed adapters, permission/cancellation, native publication/recovery,
  desktop/platform or visual baselines. Existing ResizeObserver diagnostics
  remain explicit.

```
<!-- /evidence-field -->

<!-- evidence-field:id markdown -->
## Id

E-INBOX-CONTROLS-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M24
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-015
- JST-UI-006
- JST-UI-012
- JST-API-001
- JST-DOC-001
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-10
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-application/AgentInboxNavigation.stories.svelte
- catalog/components/store-application/InboxControls.stories.svelte
- catalog/components/store-application/ResponsiveNavigation.stories.svelte
- catalog/components/store-threads/StoreThreadNavigation.stories.svelte
- catalog/components/store-threads/StoreThreadSearch.stories.svelte
- catalog/fixtures/AgentInboxTreeFixture.svelte
- catalog/fixtures/ResponsiveNavigationFixture.svelte
- docs/maintenance/coverage.json
- docs/site/guides/agent-execution.md
- docs/ui-provenance.json
- spec/src/current/contracts/api/01-read-apis-and-browser.md
- spec/src/current/contracts/documentation/01-application-documentation.md
- spec/src/current/contracts/ui/16-repository-destinations-and-tab-overflow.md
- spec/src/current/contracts/ui/18-installation-projects-hub.md
- spec/src/current/contracts/ui/39-intentional-thread-inbox.md
- spec/src/current/progress/M24/intentional-threads-review.md
- web/src/lib/ui/StoreToolbarSearch.svelte
- web/src/lib/ui/community-inbox/CommunityInbox.svelte
- web/src/lib/ui/community-inbox/CommunityInboxFilters.svelte
- web/src/lib/ui/community-inbox/CommunityInboxItem.svelte
- web/src/lib/ui/community-inbox/CommunityInboxSessions.svelte
- web/src/lib/ui/community-inbox/inbox-filters.ts
- web/src/lib/ui/community-inbox/inbox-preferences.ts
- web/src/lib/ui/community-inbox/inbox.css
- web/src/lib/ui/community-inbox/model.ts
- web/src/lib/ui/store-agent-execution/StoreAgentExecutionHost.svelte
- web/src/lib/ui/store-agent-execution/session-presentation.ts
- web/src/lib/ui/store-agent-workspace/StoreWorkspaceFiles.svelte
- web/src/lib/ui/store-application/StoreViewTabs.svelte
- web/src/lib/ui/store-application/store-application.css
- web/src/lib/ui/store-application/store-navigation.ts
- web/src/lib/ui/store-application/view-tabs.css
- web/src/lib/ui/store-settings/StoreSettingsPage.svelte
- web/src/lib/ui/store-threads/StoreThreadAgentRepositories.svelte
- web/src/lib/ui/store-threads/StoreThreads.svelte
- web/src/lib/ui/store-threads/thread-inbox.ts
- web/src/lib/ui/store-work/StoreProjectsNavigation.svelte
- web/src/lib/ui/store-work/StoreWork.svelte
- web/tests/agent-inbox-filters.test.ts
- web/tests/agent-inbox-groups.test.ts
- web/tests/agent-inbox-preferences.test.ts
- web/tests/vite/thread-navigation.test.ts
- spec/src/current/governance/spec-governance.md
- spec/src/current/progress/M24/index.md
- spec/src/current/progress/M24/SUMMARY.index-index/002/index.md
- spec/src/current/progress/M24/history/M24/index.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

[]
<!-- /evidence-field -->

