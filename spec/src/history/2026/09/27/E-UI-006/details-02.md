---
schema: 1
id: E-UI-006
---

# E-UI-006 details 2

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
pnpm test:storybook catalog/components/store-application/RepositoryReview.stories.svelte
```

**Notes**

Before fix: malformed StoreApplication style block caused Svelte unexpected_eof before interaction tests loaded.
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
pnpm test:storybook catalog/components/store-application/RepositoryReview.stories.svelte
```

**Notes**

Three review stories passed after moving Files breadcrumb/snapshot selector into the page nav, using the body file-tree sidebar, moving the tree toggle next to the avatar row and removing file-browser card borders.
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
pnpm test:storybook catalog/components/store-application/StoreRepositoryApplication.stories.svelte
```

**Notes**

Connected application story still asserted the removed Files heading. The rendered app exposed Files through the body breadcrumb and toolbar instead.
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
pnpm test:storybook catalog/components/store-application/RepositoryReview.stories.svelte
```

**Notes**

Final focused review pass: verifies toolbar branch selector, Files-rooted breadcrumb, toolbar bottom divider matching the sidebar border, padded file-tree body sidebar, full-height divider, moved tree toggle, borderless directory surface and directory drill-down.
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
pnpm test:storybook catalog/components/store-application/StoreRepositoryApplication.stories.svelte
```

**Notes**

Four connected app stories pass with the Files breadcrumb replacing the removed heading and the snapshot picker remaining in the page nav.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm format:check
```

**Notes**

Rustfmt passed; Prettier reported three Svelte files after the initial Files layout edit. Ran targeted Prettier over touched source/spec files.
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
pnpm format:check
```

**Notes**

Rustfmt and Prettier pass after formatting the touched Svelte files.
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
pnpm test:storybook catalog/components/store-application/RepositoryReview.stories.svelte
```

**Notes**

File action-row assertion found left sidebar padding remained on the inner listing instead of the body-sidebar container.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-application/RepositoryReview.stories.svelte
```

**Notes**

Action-row/header alignment assertion found the shared header height variable only existed in the right content subtree, leaving the left action row shorter.
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
pnpm test:storybook catalog/components/store-application/RepositoryReview.stories.svelte
```

**Notes**

Three review stories pass with action-row toggles, aligned file-tree/commit row height and bottom edge, compact table header and reduced commit row left padding.
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
pnpm test:storybook catalog/components/store-application/StoreRepositoryApplication.stories.svelte
```

**Notes**

Four connected app stories still pass after adding the action row, shared file-listing mode toggle and compact header density.
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
pnpm test:storybook catalog/components/store-application/RepositoryReview.stories.svelte
```

**Notes**

Three review stories pass after increasing action-row left padding; Files in the body file-tree action row aligns with the page navigation Files label.
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
pnpm test:storybook catalog/components/store-application/StoreRepositoryApplication.stories.svelte
```

**Notes**

Four connected app stories still pass after the final action-row left padding alignment.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test:storybook catalog/components/store-application/RepositoryReview.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLog.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte
```

**Notes**

Extended review reproduced two failed assertions plus one unhandled operation file-selection error: op-log changed files rendered but route validation rejected non-snapshot operation paths, and collapsed-column assertions used the internal revisions id after restoring body padding.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test:storybook catalog/components/store-application/RepositoryReview.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLog.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte
```

**Notes**

15 focused stories pass: body padding restored, collapsed columns fill the padded content box, operation-head rows use muted grey, log graphs remove internal top inset, Op Log opens changed files and file diffs, and connected Files navigation still passes.
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
pnpm format:check
```

**Notes**

Rustfmt passed; Prettier reported spec/src/spec-governance.md after the spec wording edits.
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
pnpm format:check
```

**Notes**

Rustfmt and Prettier pass after formatting spec/src/spec-governance.md.
<!-- /run -->

<!-- run -->
### Run 18

```yaml
result: fail
failures: 6
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check
```

**Notes**

Executable spec lanes passed through browser, contracts, rustfmt and boundaries, then progress bookkeeping rejected five Storybook story-name references in the tests field plus the partial result token.
<!-- /run -->
<!-- /evidence-field -->

