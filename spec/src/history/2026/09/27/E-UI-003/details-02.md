---
schema: 1
id: E-UI-003
---

# E-UI-003 details 2

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
pnpm test:storybook catalog/components/community-projects/CommunityProjectsRepositoryLog.stories.svelte
```

**Notes**

Before implementation: no matching stories.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
pnpm catalog:check
```

**Notes**

Initial fixture state name shadowed the Svelte rune: 5 errors and 6 warnings; renamed it.
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
pnpm test:storybook catalog/components/community-projects/CommunityProjectsRepositoryLog.stories.svelte
```

**Notes**

Initial suite import/reoptimization failure; source compilation exclusions retained.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/community-projects/CommunityProjectsRepositoryLog.stories.svelte
```

**Notes**

CSF harness children were mounted as component children: 4 failed, 4 passed, 8 unhandled errors; fixed template snippets.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/community-projects/CommunityProjectsRepositoryLog.stories.svelte
```

**Notes**

Radio controls were queried as buttons; corrected public roles.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/community-projects/CommunityProjectsRepositoryLog.stories.svelte
```

**Notes**

Highlighted code spans split the text query; assert the complete diff textContent. Repeated once before final correction.
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
pnpm test:storybook catalog/components/community-projects/CommunityProjectsRepositoryLog.stories.svelte
```

**Notes**

8 repository stories pass after corrections.
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
node --test scripts/catalog-browser.test.mjs
```

**Notes**

Geometry regression exposed wrapped native bookmark labels beyond fixed row height; measured row heights now feed original graph paths.
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
node --test scripts/catalog-browser.test.mjs
```

**Notes**

Narrow bookmark labels overlapped inline IDs; IDs now wrap beneath content in narrow graph columns.
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
node --test scripts/catalog-browser.test.mjs
```

**Notes**

Keyboard branch input queried by a label replaced by public Command context; switched to its placeholder. First unbounded attempt interrupted; regression now has bounded timeouts.
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
pnpm catalog:test:browser
```

**Notes**

Autodocs includes its primary preview plus named stories; corrected explicit iframe count.
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
pnpm catalog:test:browser
```

**Notes**

Cold static build, desktop/mobile geometry, keyboard selection/resize, Docs canvases, state reset, font/icon contracts and no external/API/WebSocket requests pass.
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
pnpm test:storybook
```

**Notes**

33 interaction/axe tests across 10 story files pass before the additional Full app story.
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
pnpm catalog:test
```

**Notes**

15 unit tests pass before copied file-browser helper tests were added.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: fail
failures: 17
hasNote: true
noteEncoding: markdown

```

```shell
pnpm catalog:check
```

**Notes**

Final review found unsupported Testing Library exact options in two story files. Removed the unsupported options; string role names are already exact.
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
pnpm catalog:check
```

**Notes**

One multiline exact option remained; removed it before the passing rerun.
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: fail
failures: 17
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check
```

**Notes**

Stopped at catalog-check with the same 17 story type errors; later passed all 18 lanes.
<!-- /run -->

<!-- run -->
### Run 18

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm format:check
```

**Notes**

Prettier parsed literal script tags inside two CSF play attributes as Svelte script blocks. Moved the unsafe-source fixture string to the fixture module; formatter then passed.
<!-- /run -->
<!-- /evidence-field -->

