---
schema: 1
id: E-UI-068
---

# E-UI-068 details 2

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/NativeReviewDetail.stories.svelte catalog/components/store-work/ProjectsReviewDetail.stories.svelte
```

**Notes**

Before:1 pass/3 fail. Repository and directory checks expose retained list/side-panel composition; selected-project fixture also exposed an initialization race. /tmp/jj-review-page-before.log.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/NativeReviewDetail.stories.svelte catalog/components/store-work/ProjectsReviewDetail.stories.svelte
```

**Notes**

Initial repair:2 pass/2 fail. Corrected the test that chained getByRole on HTMLElement and waited for the selected-project fixture to initialize before changing tabs; retained all layout assertions. /tmp/jj-review-page-after.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/NativeReviewDetail.stories.svelte catalog/components/store-work/ProjectsReviewDetail.stories.svelte
```

**Notes**

Corrected4/4; strengthened5/5 adds fixed header/footer measurements, draft retention and Focus/Split independence. /tmp/jj-review-page-regressions.log; /tmp/jj-review-page-regressions-final.log.
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
pnpm web:check
```

**Notes**

Initial/final/qualified source rounds:0 errors/warnings. Logs web-check, web-final and web-qualified under /tmp/jj-review-page-*.log.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm catalog:check
```

**Notes**

Initial new story typing error from HTMLElement.getByRole; fixed the test query. /tmp/jj-review-page-catalog-check.log.
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
pnpm catalog:check
```

**Notes**

Corrected final/skeleton/qualified rounds:0 errors/warnings;131 catalog modules. Logs catalog-final, catalog-skeleton and catalog-qualified.
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
pnpm check:architecture
```

**Notes**

Initial743 modules. /tmp/jj-review-page-architecture.log.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:architecture
```

**Notes**

Added default-comment depth assertion pushed StoreCollaboration story to403 counted lines versus400. Reused its located message without removing assertions or changing limits. /tmp/jj-review-page-architecture-final.log.
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
pnpm check:architecture
```

**Notes**

Repaired744 modules within limits. /tmp/jj-review-page-architecture-repaired.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/NativeReviewDetail.stories.svelte catalog/components/store-work/ProjectsReviewDetail.stories.svelte catalog/components/store-application/NativeMessageActions.stories.svelte catalog/components/store-application/StoreCollaboration.stories.svelte catalog/components/store-application/AppearanceDiscussions.stories.svelte catalog/components/store-application/RepositoryLoading.stories.svelte catalog/components/store-threads/StoreCapturedReactions.stories.svelte catalog/components/store-work/ProjectsHub.stories.svelte catalog/components/store-work/ProjectsActivity.stories.svelte catalog/components/store-work/ProjectsDirectory.stories.svelte catalog/components/store-work/ProjectsFilters.stories.svelte catalog/components/store-work/ProjectsActivityControls.stories.svelte catalog/components/store-work/TaskScroll.stories.svelte
```

**Notes**

67/68 pass. Existing page-comment test expected Reply to review thread; updated to Review comment/Add comment and asserted native thread depth0. /tmp/jj-review-page-consumers.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/NativeReviewDetail.stories.svelte catalog/components/store-work/ProjectsReviewDetail.stories.svelte catalog/components/store-application/NativeMessageActions.stories.svelte catalog/components/store-application/StoreCollaboration.stories.svelte catalog/components/store-application/AppearanceDiscussions.stories.svelte catalog/components/store-application/RepositoryLoading.stories.svelte catalog/components/store-threads/StoreCapturedReactions.stories.svelte catalog/components/store-work/ProjectsHub.stories.svelte catalog/components/store-work/ProjectsActivity.stories.svelte catalog/components/store-work/ProjectsDirectory.stories.svelte catalog/components/store-work/ProjectsFilters.stories.svelte catalog/components/store-work/ProjectsActivityControls.stories.svelte catalog/components/store-work/TaskScroll.stories.svelte
```

**Notes**

68/68 after label correction;69/69 after representative skeleton and narrow-layout story, on fresh Vitest browser/Vite processes with blocking accessibility checks. Reply/edit/reactions/unknown requests, inline tool/diff discussions, Focus/Split, all ProjectsHubFixture direct families and task layout pass. /tmp/jj-review-page-consumers-final.log; /tmp/jj-review-page-all-final.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/projects-code-navigation.test.ts
```

**Notes**

7/7 existing production boundary tests:authorized repository selection, anchor operation, Back and stale navigation. /tmp/jj-review-page-navigation.log.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/RepositoryLoading.stories.svelte -t ReviewDetail
```

**Notes**

Before skeleton repair:3/3 selected-detail cases fail the representative page layout assertion. /tmp/jj-review-page-skeleton-before.log.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/RepositoryLoading.stories.svelte
```

**Notes**

Initial skeleton:14 pass/3 accessibility failures. Hidden decorative ScrollArea retained a focusable viewport. Marked that region inert using the existing public DOM prop. /tmp/jj-review-page-skeleton-after.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/RepositoryLoading.stories.svelte
```

**Notes**

After inert repair:17/18 pass; new narrow test selected a nonexistent shell class. Corrected to the real body-regions host;380px detail width and overflow assertions retained. /tmp/jj-review-page-skeleton-final.log. Final69/69 direct-consumer run covers all18 loading stories.
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
jj --version; node --version; pnpm --version
```

**Notes**

JJ0.45.1;Node26.9.0;pnpm10.32.1.
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
cua_repl: cold hidden repository reviews; select detail; measure and scroll; Back; Projects Reviews; select detail; capture screenshots
```

**Notes**

Both live entry points replace lists with the same page and no discussion panel. At1280x720, header top49/bottom120.7890625 and footer top569/bottom711 unchanged while scrollTop0 to353.5px (802/448px content). No comments submitted or fixture data reset. /tmp/jj-review-page-final.png.
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
cua_repl: temporarily delay network in the verification tab; capture selected-review loading skeleton; restore normal network
```

**Notes**

Live skeleton occupies full main width with title/date, metadata, four disclosures, Activity and bottom composer. DOM exposes one named busy status and no placeholder controls. First cold-module reload at3000ms latency timed out the browser helper; reconnected and restored latency0. Final warmed selection at8000ms captured the skeleton in under1s and restored latency0 immediately. /tmp/jj-review-page-skeleton.png. Temporary browser network settings restored; no server changes.
<!-- /run -->
<!-- /evidence-field -->

