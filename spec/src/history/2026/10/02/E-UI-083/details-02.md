---
schema: 1
id: E-UI-083
---

# E-UI-083 details 2

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
jj --no-pager st; jj --no-pager diff; inspect Community and released Design Core Button/Collapsible source
```

**Notes**

Clean working change after committed row/skeleton slice. Collections used local state and reset on remount; expanded ghost buttons used muted background. Narrow acceptance is persisted choices and identical transparent resting sidebar actions.
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
pnpm exec prettier --write (changed production/story/test/chapter files)
```

**Notes**

Initial formatting, subsequent missed-open/snippet fixes, test selectors and Op Log scenario formatted. /tmp/jj-disclosures-format.log; /tmp/jj-disclosures-format-story.log.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 7
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:check (initial)
```

**Notes**

Seven diagnostics in two files: Activity state collided with existing snippet and one NativeAgentContext open reference remained. Fixed naming and reference; no assertion changes. /tmp/jj-disclosures-web.log.
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
pnpm web:check (after fixes and final)
```

**Notes**

0errors0warnings. /tmp/jj-disclosures-web-final.log; /tmp/jj-disclosures-web-qualified.log.
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
pnpm catalog:check (initial and final)
```

**Notes**

0errors0warnings;148catalog modules. /tmp/jj-disclosures-catalog.log; /tmp/jj-disclosures-catalog-qualified.log.
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
pnpm check:architecture (initial)
```

**Notes**

CommunityAppShell305counted lines exceeded unchanged300line budget. Moved public token boundary to existing owned right-sidebar.css, without increasing any budget.
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
pnpm check:architecture; pnpm check:ui-ownership (qualified)
```

**Notes**

792modules; ownership1validator/21chapters/147requirements. /tmp/jj-disclosures-architecture-qualified.log; /tmp/jj-disclosures-ownership-qualified.log.
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
pnpm spec:first (initial)
```

**Notes**

19protected files/2canonical chapters. /tmp/jj-disclosures-spec-first.log.
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
pnpm catalog:test web/tests/vite/disclosure-layout.test.ts web/tests/vite/store-shell-layout.test.ts
```

**Notes**

9/9 across2files: restore open/closed, owner/task/group/project isolation, stable filter/operation keys, malformed storage, blocked/quota fallback and opaque1024entry bound. /tmp/jj-disclosures-unit.log.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-work/DisclosurePersistence.stories.svelte (initial cold run)
```

**Notes**

1suite/0tests: newly imported SHA256 module caused Vite dependency optimization and reload during import. Prepared optimizer cache reused for the next fresh browser process; no infrastructure changes. /tmp/jj-disclosures-regression.log.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-work/DisclosurePersistence.stories.svelte (second)
```

**Notes**

3passed/2failed. Agent context was queried after collapsing its parent Activity; synthetic Testing Library hover does not activate CSS :hover. Test now opens context before Activity and uses Vitest native pointer for CSS hover. /tmp/jj-disclosures-regression-second.log.
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
pnpm test:storybook catalog/components/store-work/DisclosurePersistence.stories.svelte (third)
```

**Notes**

5passed/1failed: restored Files changed includes its count in the accessible name. Count-aware anchored selector retains the expansion assertion. /tmp/jj-disclosures-regression-third.log.
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
pnpm test:storybook catalog/components/store-work/DisclosurePersistence.stories.svelte (qualified)
```

**Notes**

6/6 in fresh browser/Vite process: Projects and Repository groups across tab/remount; task sections and retained composer; review sections including nested agent context; Op Log summary; both sidebar states and native CSS hover. /tmp/jj-disclosures-regression-final.log.
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
pnpm test:storybook catalog/components/store-application/StoreApplication.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte catalog/components/store-application/StoreCollaboration.stories.svelte catalog/components/store-application/NativeReviewDetail.stories.svelte catalog/components/store-application/AgentLayout.stories.svelte catalog/components/store-application/RepositoryReview.stories.svelte catalog/components/store-work/ProjectsDirectory.stories.svelte catalog/components/store-work/ProjectsHub.stories.svelte catalog/components/store-work/ProjectsAgents.stories.svelte catalog/components/store-work/ProjectsReviewDetail.stories.svelte catalog/components/store-work/StoreWork.stories.svelte catalog/components/store-work/TaskScroll.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLogDiff.stories.svelte
```

**Notes**

82/82 across13 directly affected files in a fresh Vitest browser/Vite process. Shell/application, collection, task/review, agent and Op Log interaction/accessibility consumers. /tmp/jj-disclosures-consumers.log.
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
Live reload verification through CUA temporary background tab
```

**Notes**

Projects agent repository group and Repository Agent sessions stay aria-expanded=false after reload; restored original expanded choices. Both sidebar actions background rgba(0,0,0,0), including expanded Details; left button32x32. Screenshot saved and inspected. Restored initial closed Details preference and closed temporary tab; user tab untouched. No repository data writes or service restart.
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
pnpm spec:first; pnpm spec:validate; pnpm check:progress (initial ledger)
```

**Notes**

Spec-first24protected files/3chapters and specification5validators/21chapters/147requirements passed. Progress checker rejects a temporary pending run result; replaced it with the completed82/82consumer pass. No product change. /tmp/jj-disclosures-spec-first-qualified.log; /tmp/jj-disclosures-spec-validate.log; /tmp/jj-disclosures-progress.log.
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
pnpm spec:first; pnpm spec:validate; pnpm check:progress; pnpm exec prettier --check (27 changed files)
```

**Notes**

24protected files/3canonical chapters;5validators/21chapters/147requirements;120tasks;all changed files formatted. /tmp/jj-disclosures-spec-first-final.log; /tmp/jj-disclosures-spec-validate-final.log; /tmp/jj-disclosures-progress-final.log; /tmp/jj-disclosures-format-check.log. This evidence insertion is formatted and rechecked before the standing JJ commit.
<!-- /run -->
<!-- /evidence-field -->

