---
schema: 1
id: E-UI-TAG-LIST-001
---

# E-UI-TAG-LIST-001 details 3

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
pnpm web:check
pnpm catalog:check
```

**Notes**

Final source type checks: zero errors/warnings; 461 catalog modules satisfy boundaries.
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
pnpm exec vite build --config web/vite.config.ts
node scripts/web-manifest.mjs
```

**Notes**

Final production bundle and asset manifest passed. Existing large-chunk advisory is retained; no threshold changed.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 73
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_CI_REPORTS=1 pnpm test:storybook --shard=1/2
JJ_STORE_CI_REPORTS=1 pnpm test:storybook --shard=2/2
```

**Notes**

Broad qualification was interrupted at the user's explicit request. Logs reported 73 failed cases across 36 families before stopping; these are observed failures, not complete-suite totals. Some directly affected old selectors are already repaired and covered by the final 69 passing stories. Other failures remain unqualified/deferred, including transition visibility assertions and the parent-reproducing reference contrast issue. No broad pass is claimed; no rules, snapshots or assertions were weakened. The menu-exit change outside this slice was restored and its candidate patch saved outside the checkout at /tmp/jj-taglist-deferred-menu-test.patch.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 6
hasNote: true
noteEncoding: markdown

```

```shell
Parent baseline: selected failure patterns from the broad run, using /tmp/jj-tags-parent-failures-command.json in a disposable workspace at ae9e4ea0 with locked Design Core 0.3.2
```

**Notes**

19 files: 32 selected tests passed, six failed, 101 skipped. This sampling does not establish that every current broad failure pre-exists. The clean disposable workspace was forgotten and removed after recording results.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec spec-validator validate --only summary,fileLimits,governance,verification
```

**Notes**

First docs run found an over-400-line change map and missing navigation for two newly paginated evidence pages. Moved the short ownership note into the bounded evidence-governance topic and regenerated navigation. Unrelated generated whitespace changes were restored.
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
pnpm test:storybook catalog/components/tag-list-adoption/TagListAdoption.stories.svelte catalog/components/store-mutations/StoreMutationCommitPicker.stories.svelte
```

**Notes**

Additional fresh check after the final accessible-description association: all four stories passed.
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
node --test scripts/web-architecture.test.mjs scripts/ui-ownership.test.mjs
```

**Notes**

All 11 structural/ownership regressions passed.
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
pnpm exec spec-validator validate --only summary,fileLimits,governance,verification,ui-ownership
```

**Notes**

All five selected validators passed over 3962 chapters and 205 requirements after the bounded-doc/navigation repairs.
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
pnpm check:progress
pnpm spec:first
```

**Notes**

Progress evidence bookkeeping passed (156 tasks, 834 records). Spec-first passed for 29 protected files and four canonical chapters. Changed-file Prettier check also passed. Broader Storybook qualification remains explicitly outside the user-approved slice.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/community-projects/CommunityProjectsRevisionGraph.stories.svelte -t 'JJLogLongChipsStayWithinTheRow|OpLogLongChipsStayWithinTheRow'
```

**Notes**

Before the requested tag-only popup correction: both new assertions reproduced the visible repeated Local bookmark description. Two failed, six skipped. See /tmp/jj-tags-pure-baseline.log.
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
pnpm test:storybook catalog/components/community-projects/CommunityProjectsRevisionGraph.stories.svelte -t 'JJLogLongChipsStayWithinTheRow|OpLogLongChipsStayWithinTheRow'
```

**Notes**

After removing consumer-supplied redundant bookmark/workspace descriptions: both exact stories pass, six skipped. Full tag labels remain visible in the popup; repeated descriptions are absent; Escape restores focus and popup actions do not select the row. See /tmp/jj-tags-pure-after.log.
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
pnpm test:storybook catalog/components/community-projects/CommunityProjectsRevisionGraph.stories.svelte
```

**Notes**

After the tag-only popup correction: all eight graph interaction/accessibility stories pass. Full labels, native tones, keyboard popup access, Escape/focus restoration and independent row selection remain covered. See /tmp/jj-tags-pure-family.log.
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
pnpm web:check
pnpm catalog:check
```

**Notes**

Repeated after removing redundant popup descriptions: zero errors and zero warnings in each type check; all 461 catalog modules satisfy import boundaries. See /tmp/jj-tags-pure-types.log and /tmp/jj-tags-pure-catalog.log.
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

Design Core 0.3.2 -&gt; 0.5.0; pnpm 10.32.1; Svelte 5.57.1; Vite 8.3.1; Vitest 4.1.11; Storybook 10.5.7.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Native JJ identities, HTTP/configuration/storage formats, routing, callbacks and selection authority are unchanged. TagList data is an application-owned presentation projection.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Adopt published TagList for revision/op-log metadata, commit metadata, stack aliases/status collections, forum tags, room facts, capture source states/counts and workflow command names. Preserve order, native labels, tones and source provenance. Editable/removable inputs, entity-link cards and standalone identity/status badges remain on their existing controls. Explicit jj-store utilities CSS restores pre-existing app-owned Tailwind compositions after Design Core stopped supplying the compiler entry. UI suites are widened for this shared CSS/dependency boundary; root aggregate, Rust/storage/service and release lanes are out of scope. The user explicitly directed: Keep this slice focused; defer broader qualification. This pass covers the focused adoption slice, not full Storybook or release qualification. The user retained the flat bookmark list and requested tag-only popovers. Revision/op-log mappings omit redundant descriptions; shared TagList still owns full-label truncation tooltips.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Review the TagList adoption; perform broader catalog compatibility cleanup/qualification in a separate slice. No downstream publication, default-workspace merge or service reset is performed.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: Owning UI/API/architecture topics now specify bounded TagList
  presentation, the independent listbox preview and the application-owned
  utility entry. JST-GOV-008 provenance and controller/native-model ownership
  remain unchanged. Revision/op-log popovers retain a flat list with tag labels
  only, without repeated bookmark/workspace descriptions.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: "Reviewed M4.2: this is a local shared-UI dependency adoption; no
  milestone completion or native/backend qualification claim is added. Existing
  milestone status and gates remain unchanged. Broader Storybook qualification
  was explicitly deferred by the user; this evidence must not select a full
  catalog/release gate."

```
<!-- /evidence-field -->

