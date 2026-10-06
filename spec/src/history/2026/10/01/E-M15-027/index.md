---
schema: 1
id: E-M15-027
---

# E-M15-027

<!-- evidence-field:id markdown -->
## Id

E-M15-027
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M15
- M15.5
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-01
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

direct-thread-settings-and-reaction-tooltips
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change wytvknwz
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Main clean above landing b59fe1a8. Kebab opens a single-item menu. Existing alignment story updated to require immediate Thread settings: initial exact display-name filter ran zero tests (3 skipped); complete story file fails 2 / 3 on missing direct button, 1 passes. /tmp/jj-thread-direct-before.log and before2.log. Tooltip count regression: 2 captured-message stories fail before with custom actor popover and no count label. Cold alignment failed after edits because message menu document scroll lock remained active (body pointer-events none, no menu). Tooltip replacement alone does not repair that; existing canvas scroll-lock policy applies to message menus too.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

JJ 0.45.1; pnpm 10.32.1; inherited Design Core 0.1.2 development snapshot, registry foundation qualification separate.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Existing Button directly opens standard settings pane. No dropdown intermediary; accessible label Thread settings; toolbar placement remains last. No native/API formats changed. Existing ReactionBar owns shadcn hover/focus tooltip; labels carry counts/value/loaded authorized actors. Exact-value toggling preserved. Message menu disables redundant document scrolling through public preventScroll=false.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- web/src/lib/ui/store-threads/StoreThreadHeader.svelte
- web/src/lib/ui/store-threads/StoreThreadMenu.svelte
- catalog/components/store-threads/StoreThreadAlignment.stories.svelte
- web/tests/browser/threads-alignment.spec.ts
- spec/src/ui.md
- spec/src/milestones.md
- web/src/lib/ui/community-reactions/CommunityReactions.svelte
- web/src/lib/ui/community-reactions/CommunityReactionActors.svelte
- web/src/lib/ui/community-message-actions/CommunityMessageActions.svelte
- catalog/components/store-threads/StoreCapturedReactions.stories.svelte
- catalog/components/store-threads/StoreThreads.stories.svelte
- docs/ui-provenance.json
- spec/src/spec-governance.md
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-threads/StoreThreadAlignment.stories.svelte
```

**Notes**

Exact regression after implementation: 3 / 3 pass, direct settings and read-only permissions remain correct. /tmp/jj-thread-direct-after.log.
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
pnpm web:check; pnpm catalog:check; pnpm web:build
```

**Notes**

Both type checks have 0 errors and warnings; 121 catalog modules pass. Main production build passes with inherited chunk-size advisory. /tmp/jj-thread-direct-web-check.log, catalog-check.log, build.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-threads
```

**Notes**

8/8 direct Thread, Agent and Review reaction tests; hover tooltip initially checked as role tooltip but current library renders a generic data-ui tooltip. Tests now verify the actual shared component and text; keyboard focus in stories plus real hover in cold browser. /tmp/jj-thread-tooltip-final-stories.log.
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
pnpm exec playwright test --config /tmp/jj-thread-main-slices-playwright.config.mjs threads-alignment.spec.ts
```

**Notes**

Cold built bundle: 1/1; direct settings, independent pane/rail, edit then reply and actual reaction hover. Scroll-lock regression 1 failure -&gt; 0. /tmp/jj-thread-controls-browser.log.
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
JJ_THREAD_SLICE_EVIDENCE_PREFIX=/tmp/jj-thread-controls node /tmp/jj-thread-main-slice-restart.mjs
```

**Notes**

Main server 28089/storage 28090; served index equals build; snapshots, selected Thread/messages, fixture and installation unchanged; content search HTTP200 and ready. /tmp/jj-thread-controls-restart.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreCollaboration.stories.svelte catalog/components/store-application/NativeReviewDetail.stories.svelte
```

**Notes**

Direct agent/review conversations: 10/10.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/StoreWork.stories.svelte
```

**Notes**

Direct WorkStore discussion consumer: 5/5.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-collaboration catalog/components/store-work
```

**Notes**

Overbroad directory selection includes unrelated Projects activity/filter stories: 23/26 plus one ScrollArea derived_inert teardown diagnostic. The named collaboration directory does not exist. Actual direct consumers qualified separately. /tmp/jj-thread-controls-consumers.log.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
Parent-source comparison: temporarily restore parent CommunityMessageActions, CommunityReactions and actor adapter; run exact ProjectsActivity and ProjectsFilters story files; finally restore current bytes
```

**Notes**

Parent source reproduces both retained filter search failures (2/4 checks); Activity passes in isolation, and broad ScrollArea teardown diagnostic does not recur. Both filter checks are outside changed interaction impact. No assertions/source altered. /tmp/jj-thread-controls-unrelated-parent.log.
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
pnpm spec:validate; pnpm check:progress; pnpm check:architecture; pnpm web:check; pnpm catalog:check; scoped pnpm exec prettier --write/check
```

**Notes**

Spec initially failed removed actor provenance, progress initially rejected remaining gap with pass result; removed provenance entry and corrected evidence scope. Final validators/checks pass. Both type checks zero errors/warnings.
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

<!-- evidence-field:scope markdown -->
## Scope

User-authorized main workspace slice and rebuild/restart after each slice. Direct owning Thread story family and cold served browser only; Rust and root aggregate suites out of scope. Existing data preserved, no package publication or baseline approval. Stories remain visual-pending. Shared message actions are consumed by WorkStore and Agent/Review conversation presentation, so direct consumer stories are included. Overbroad Projects checks are recorded as out-of-scope parent failures; no full-suite claim.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Commit controls and tooltip slice, then align pane headers and put targeted reaction feedback in Chat Message Metadata.
<!-- /evidence-field -->

<!-- evidence-field:removed_files list -->
## Removed files

- web/src/lib/ui/store-threads/StoreThreadMenu.svelte
- web/src/lib/ui/community-reactions/CommunityReactionActors.svelte
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


