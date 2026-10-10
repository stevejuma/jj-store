---
schema: 1
id: E-REPOSITORY-BANNERS-001
---

# E-REPOSITORY-BANNERS-001

<!-- evidence-field:id markdown -->
## Id

E-REPOSITORY-BANNERS-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M9.5
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-COLLAB-005
- JST-UI-006
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

- web/src/lib/ui/community-projects/CommunityProjectsColumnAlert.svelte
- web/src/lib/ui/store-collaboration/StoreReviewThreads.svelte
- web/src/lib/ui/store-collaboration/StoreAgentSessions.svelte
- web/src/lib/ui/store-collaboration/StoreAgentSidebar.svelte
- catalog/components/store-application/RepositoryNotices.stories.svelte
- catalog/fixtures/RepositoryUnavailableDetailFixture.svelte
- catalog/components/store-application/agent-sidebar-play.ts
- docs/site/guides/browsing.md
- docs/maintenance/coverage.json
- spec/src/current/contracts/ui/12-repository-presentation.md
- spec/src/current/progress/M9/index.md
- spec/src/current/progress/M9/history/M9.5/index.md
- spec/src/current/contracts/api/01-read-apis-and-browser.md
- spec/src/current/contracts/documentation/01-application-documentation.md
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/RepositoryNotices.stories.svelte -t "Reviews banner matches"
```

**Notes**

First invocation selected no tests: Storybook generates CamelCase test names. Three skipped; not acceptance. Corrected the filter below. /tmp/repository-banner-before.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/RepositoryNotices.stories.svelte -t ReviewsBannerMatches
```

**Notes**

Before state: banner left edge did not align with the Reviews surface header. Existing shared root forced flush, square bottom-border presentation. /tmp/repository-banner-before-corrected.log. Story names and focused filters are recorded in runs; the ledger tests field recognizes only Rust/Python/test() definitions, so it does not contain Svelte Story name attributes. Initial check:progress refused the five story references; corrected the evidence schema without changing tests.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/RepositoryNotices.stories.svelte -t ReviewsBannerMatches
```

**Notes**

After state: 1 pass /2 unselected. Both edges and radius align, 8px gap and full border, existing padding retained. /tmp/repository-banner-fixed.log.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/RepositoryNotices.stories.svelte catalog/components/store-application/StoreCollaboration.stories.svelte catalog/components/store-application/RepositoryLoading.stories.svelte
```

**Notes**

32 pass /1 failure, 3 files. All five new banners and all loading scenarios pass. AgentResponseActivity fails on an obsolete accessible diff label: expected Changed file src/lib.rs, actual shared TranscriptToolDetail exposes Diff for src/lib.rs. The content and region are present. Updated only the helper selector; no production tool behavior changed. /tmp/repository-banner-consumers.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreCollaboration.stories.svelte -t AgentResponseActivity
```

**Notes**

Exact corrected selector target passes: 1 pass /9 unselected. It still verifies the complete diff, sidebar geometry, tool interactions, comments and selection. The helper has one consumer. /tmp/repository-banner-sidebar-selector.log.
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
pnpm catalog:check
pnpm check:ui-ownership
```

**Notes**

Web zero errors/warnings. Final catalog after the new detail fixture: zero errors/warnings, 361 modules. Ownership passes, 3466 chapters /181 requirements. Logs /tmp/repository-banner-web.log, /tmp/repository-banner-catalog-final.log, /tmp/repository-banner-ownership.log. UI-only impact: Rust builds/tests, generated HTTP contracts and root aggregates are out of scope. Fresh Storybook processes exercise the production composition; no production startup or asset contract changed.
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
cua_repl browser inspection on http://127.0.0.1:7460/
```

**Notes**

Latest default-workspace HMR app. The user-reported historical repository is absent after the prior authorized reset, so the live unavailable banner says Reconnect to open the requested repository. This verifies presentation, not repository recovery. Reviews header/banner edges 253/1114 px, radius 10.4 px, margins 8px vertical and 8/16px sides, 1px full border, internal padding 12/16px, no overflow. Agents header/banner edges 261/1114 px, radius 7.2 px, margins 8/16px, full border and no overflow. Saved ignored proof .local/repository-banner-after.png. Read-only tab navigation publishes nothing.
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
pnpm exec prettier --write web/src/lib/ui/community-projects/CommunityProjectsColumnAlert.svelte web/src/lib/ui/store-collaboration/StoreReviewThreads.svelte web/src/lib/ui/store-collaboration/StoreAgentSessions.svelte web/src/lib/ui/store-collaboration/StoreAgentSidebar.svelte catalog/components/store-application/RepositoryNotices.stories.svelte spec/src/current/contracts/ui/12-repository-presentation.md
pnpm exec prettier --write catalog/fixtures/RepositoryUnavailableDetailFixture.svelte catalog/components/store-application/RepositoryNotices.stories.svelte
pnpm exec prettier --write web/src/lib/ui/community-projects/CommunityProjectsColumnAlert.svelte catalog/components/store-application/RepositoryNotices.stories.svelte catalog/fixtures/RepositoryUnavailableDetailFixture.svelte spec/src/current/contracts/ui/12-repository-presentation.md spec/src/current/progress/M9/index.md docs/site/guides/browsing.md
pnpm docs:review application "Reviewed contextual repository banner presentation: shared collection/surface insets align Reviews with its header, retain Agents geometry and contain unavailable detail panes; other tabs keep scoped toast and exact-recovery semantics."
pnpm exec prettier --write docs/maintenance/coverage.json catalog/components/store-application/agent-sidebar-play.ts
```

**Notes**

Formatted changed sources and manual topics; refreshed mapped application review. Logs /tmp/repository-banner-format.log, /tmp/repository-banner-details-format.log, /tmp/repository-banner-final-format.log, /tmp/repository-banner-doc-review.log, /tmp/repository-banner-review-format.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreCollaboration.stories.svelte
pnpm docs:check
```

**Notes**

Final collaboration family 10 pass. Combined with five banner and eighteen loading stories passing in the prior consumer run, all 33 affected scenarios are verified. Manual closure 141 files /22474943 bytes, links/anchors valid, 9 Mermaid diagrams rendered in both themes. Logs /tmp/repository-banner-collaboration-final.log and /tmp/repository-banner-docs.log.
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
pnpm evidence:new -- --id E-REPOSITORY-BANNERS-001 --task M9.5 --requirement JST-COLLAB-005 --requirement JST-UI-006
node scripts/spec/progress-history.mjs
pnpm exec prettier --write spec/src/current/progress/M9/index.md
node scripts/spec/progress-history.mjs
```

**Notes**

Initial history refresh refused an invalid task row: missing blank line before the milestone review let the formatter extend the preceding Markdown table. Restored the ledger from its parent and reapplied only the evidence link and properly separated review; formatted and regenerated successfully. Conservative normalization review refused a generated navigation addition in older E-THREADS-002; inspected it and restored unrelated old evidence/navigation and table-format churn. No prior evidence facts changed. Logs /tmp/repository-banner-navigation.log, /tmp/repository-banner-ledger-format.log and /tmp/repository-banner-navigation-repaired.log.
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
pnpm check:progress
pnpm exec spec-validator validate --only governance,summary,fileLimits,verification,book
pnpm spec:first --base 3125812e --head 88dbbc0a78499ae979ac6d1a6272e447999ee5e3
pnpm exec prettier --check web/src/lib/ui/community-projects/CommunityProjectsColumnAlert.svelte web/src/lib/ui/store-collaboration/StoreReviewThreads.svelte web/src/lib/ui/store-collaboration/StoreAgentSessions.svelte web/src/lib/ui/store-collaboration/StoreAgentSidebar.svelte catalog/components/store-application/RepositoryNotices.stories.svelte catalog/fixtures/RepositoryUnavailableDetailFixture.svelte catalog/components/store-application/agent-sidebar-play.ts spec/src/current/contracts/ui/12-repository-presentation.md spec/src/current/contracts/api/01-read-apis-and-browser.md spec/src/current/contracts/documentation/01-application-documentation.md spec/src/current/progress/M9/index.md docs/site/guides/browsing.md docs/maintenance/coverage.json
```

**Notes**

Final progress 146 tasks /728 evidence records; focused spec 5 validators /3466 chapters /181 requirements; spec-first 9 protected files /3 canonical chapters. Formatting passes. Initial spec-first at 48386131 refused missing API/documentation mappings after the manual/source review changed; added explicit unchanged API read/authorization/retry confirmation and owning manual guidance. Initial format check identified regenerated M9 navigation spacing; reformatted only the changed ledger. Final logs /tmp/repository-banner-progress-contracts.log, /tmp/repository-banner-spec-final.log, /tmp/repository-banner-spec-first-final.log and /tmp/repository-banner-format-accepted.log. Previous failed attempts remain recorded; no public API, generated contract or Rust behavior changed.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Visual baseline approval remains separate from the focused
  interaction/accessibility acceptance.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Node 26.9.0; pnpm 10.32.1; Svelte 5.57.1; Vitest 4.1.11; Storybook 10.5.7; JJ 0.45.1; macOS ARM64. The existing Vite missing-root Svelte config message remains informational.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Optional local presentation-only inset prop: collection / surface. Existing column and inline shapes are unchanged when omitted. No storage, API, receipt or source identity format changes.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Shared contained unavailable-content alerts in repository Reviews, its unavailable detail, Agent sessions and Agent response. Community source inspected first; the adapter is locally authored per provenance. Design Core public Alert composition inspected second; no foundational defect or dependency workaround. Surface shape matches the Reviews header; collection shape preserves the prior Agents geometry. The inset root does not stretch vertically in a flex detail pane. Branches/Commits/Op Log main request failures and Projects home/review/agent/activity failures use StoreNotice toasts or contextual Empty; no similar flush unavailable banners to restyle. Review mutation outcome/recovery, stack preview/conflict/target-history/diff notices are integrated column/form content and retain their presentation. No toast lifetime or error classification changed.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Retain scoped banner regressions and separate visual baseline approval.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: "JST-COLLAB-005 and JST-UI-006: updated repository presentation to cover
  shared contained unavailable banners and production geometry stories.
  JST-UI-019 operational toast/contextual recovery semantics are unchanged.
  JST-API-001 explicitly confirms unchanged wire responses, authorization and
  retry contracts; the documentation chapter governs the browsing manual
  clarification."

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M9.5 retains its prior Done status. Enclosing default change tumtyulr
  contains this narrow presentation regression and review. No independent M24
  provider/platform or ResizeObserver qualification claim; visual baselines
  remain unapproved.

```
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


