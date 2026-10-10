---
schema: 1
id: E-UI-VIEW-READINESS-001
---

# E-UI-VIEW-READINESS-001 details 3

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
```

**Notes**

0 Svelte errors and warnings. /tmp/readiness-web-check.log.
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
pnpm catalog:check
```

**Notes**

Repeated after final fixture edit: 0 errors/0 warnings, 370 catalog modules. /tmp/readiness-catalog-acceptance.log. Earlier final check also passed in /tmp/readiness-catalog-final.log.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:build
```

**Notes**

Vite bundle completed; documentation staging rejected stale application source review. Fixed the manual and acknowledged current application coverage. /tmp/readiness-web-build.log.
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
pnpm docs:review application "Reviewed shared file-tree toolbar presentation and stable tab-row search geometry against the browsing guide; ownership and controls remain unchanged."
```

**Notes**

Reviewed current source fingerprints against the updated browsing guide. /tmp/readiness-doc-review.log.
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
pnpm web:build
```

**Notes**

Current production bundle and dynamic manifest generated; documentation closure contains 141 files/22479125 bytes with valid links and anchors. Existing large-chunk/Tauri dynamic-static warnings retained. No Rust build or demo restart. /tmp/readiness-web-build-final.log.
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
pnpm exec playwright test --config .local/view-loading-proof/playwright.config.ts web/tests/browser/appearance.spec.ts
```

**Notes**

Cold native-static browser: 1/1 passes after mobile heading query moves to the actual Settings navigation. /tmp/readiness-appearance-browser.log.
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
node --test scripts/view-loading-assets.test.mjs scripts/web-manifest.test.mjs
```

**Notes**

Fresh built manifest: 4/4 pass; dynamic closure, fonts, unsafe/missing assets and unsplit stylesheet checks. /tmp/readiness-assets-final.log.
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
pnpm exec playwright test --config .local/view-loading-proof/playwright.config.ts web/tests/browser/lazy-views.spec.ts web/tests/browser/repository-navigation.spec.ts web/tests/browser/appearance.spec.ts
```

**Notes**

Current built UI, isolated native preview and fresh Chromium: 7/7 pass in 13.5s. Existing preview executable reused without a Rust rebuild. Production CSP, retry without replay, obsolete navigation, reload/cross-tab appearance, mobile Axe, snapshot/overflow/keyboard navigation and notifications retained. Cold Files fixture: 95 requests, 494ms render, 308.2ms DOMContentLoaded, 312.8ms load, 320ms first-contentful-paint. Fixture timing is not live backend performance qualification. /tmp/readiness-cold-final.log.
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

Rerun after file-tree CSS extraction and provenance update: 1 validator, 3491 chapters, 181 requirements. /tmp/readiness-ownership-final.log.
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
pnpm check:progress
```

**Notes**

Initial evidence lacked changed canonical startup and milestone files; refreshed full changed-file coverage before final validation. /tmp/readiness-progress-initial.log.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-appearance/StoreAppearance.stories.svelte catalog/components/store-application/AgentLayout.stories.svelte catalog/components/store-application/AgentSessionSettings.stories.svelte catalog/components/store-application/AgentToolGroups.stories.svelte catalog/components/store-application/AgentTranscriptSettings.stories.svelte catalog/components/store-application/ConversationCaret.stories.svelte catalog/components/store-application/LocalCompute.stories.svelte catalog/components/store-application/NativeAgentMenus.stories.svelte catalog/components/store-application/NativeReviewDetail.stories.svelte catalog/components/store-application/RepositoryApplication.stories.svelte catalog/components/store-application/RepositoryLayout.stories.svelte catalog/components/store-application/RepositoryLoading.stories.svelte catalog/components/store-application/RepositoryNotices.stories.svelte catalog/components/store-application/StoreApplication.stories.svelte catalog/components/store-application/StoreCollaboration.stories.svelte catalog/components/store-application/StoreDirectoryNavigation.stories.svelte catalog/components/store-application/StoreEvolutionColumn.stories.svelte catalog/components/store-application/StoreFileEditor.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte catalog/components/store-application/StoreRepositoryBranchContext.stories.svelte catalog/components/store-application/StoreRepositoryFeedback.stories.svelte catalog/components/store-application/StoreRepositoryNavigation.stories.svelte catalog/components/store-application/StoreRepositorySearch.stories.svelte catalog/components/store-application/StoreWorkspaces.stories.svelte catalog/components/store-application/WorkspaceFiles.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceLoading.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceSettings.stories.svelte catalog/components/store-threads/StoreThreadAlignment.stories.svelte catalog/components/store-threads/StoreThreads.stories.svelte catalog/components/store-work/ProjectsActivity.stories.svelte catalog/components/store-work/ProjectsAgentActions.stories.svelte catalog/components/store-work/ProjectsFilters.stories.svelte catalog/components/store-work/ProjectsHub.stories.svelte catalog/components/store-work/ProjectsReviewDetail.stories.svelte catalog/components/store-application/AppearanceDiscussions.stories.svelte catalog/components/store-application/StoreCaptureSettings.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceConflictVersions.stories.svelte catalog/components/store-mutations/StoreConflictEditor.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceConflicts.stories.svelte catalog/components/store-application/InboxControls.stories.svelte catalog/components/store-threads/StoreThreadSearch.stories.svelte
```

**Notes**

Fresh complete affected set: 237 pass, 1 fail, 41 families/238 stories, 368.42s. Sole failure: AgentToolGroups GroupedCallsRetainNativeMenusAndIndependentReplyDrafts could not find Focus within default 1s after navigating into the cold lazy Appearance view. That specific semantic lookup now has a bounded 10s readiness deadline; no fixed sleep, blanket timeout change or relaxed geometry/draft assertion. ResizeObserver warnings recur. /tmp/readiness-acceptance-final.log.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:validate --only summary,fileLimits,governance,verification,book
```

**Notes**

New evidence details and their navigation include were not reachable from the date SUMMARY. Added the bounded record navigation include; all five validators then pass. /tmp/readiness-spec-after.log.
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
pnpm spec:validate --only summary,fileLimits,governance,verification,book
```

**Notes**

5 validators, 3493 chapters, 181 requirements. /tmp/readiness-spec-final.log. Final post-format check follows separately.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first --base d1a4b3f3 --head 5e4d4d5cc13aae53450acbe2fb433a34093fef41
```

**Notes**

Pending wider qualification cannot be used as a completed unchanged-contract confirmation. Record now explicitly reviews the two presentation requirement clarifications as changed contracts in their owning UI chapters; broader gates remain pending. /tmp/readiness-spec-first.log.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-application/AgentToolGroups.stories.svelte catalog/components/store-application/ResponsiveNavigation.stories.svelte
```

**Notes**

8 pass, 1 fail across 2 families/9 stories, 110.76s. Appearance readiness repair passes; first grouped-row story still queries the cold transcript tool header within 1s. The private openSession helper now awaits that exact mounted header with the same bounded 10s deadline. No geometry or hit-target assertion changed. /tmp/readiness-final-followup.log.
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first --base d1a4b3f3 --head a763d6cd391c18bb9e3cca02c4c17abd428271ac
```

**Notes**

UI web paths also require their mapped API chapter. Added explicit JST-API-001 review confirming endpoints, admission, identities, continuations/source mappings and formats remain unchanged. /tmp/readiness-spec-first-final.log.
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-application/AgentToolGroups.stories.svelte catalog/components/store-application/ResponsiveNavigation.stories.svelte
```

**Notes**

Fresh isolated process: all 9 stories across 2 complete families pass, 30.23s. Private grouped-tool helper awaits its cold transcript header and Appearance control using bounded 10s semantic lookups. Original width/spacing, hover hit-target, native-menu clipboard identities, independent reply drafts and layout switch assertions remain intact. Responsive search direct consumer also passes. Together with 231 unchanged stories in the preceding affected run, all 240 unique stories across 42 affected/direct-consumer families are verified; this is not a single full-suite pass. ResizeObserver warnings recur. /tmp/readiness-final-followup-complete.log.
<!-- /run -->

<!-- run -->
### Run 18

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

0 errors/0 warnings, 370 catalog modules. /tmp/readiness-catalog-complete.log. Subsequent private helper timeout change uses the same checked query signature.
<!-- /run -->
<!-- /evidence-field -->

