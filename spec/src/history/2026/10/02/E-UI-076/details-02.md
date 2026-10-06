---
schema: 1
id: E-UI-076
---

# E-UI-076 details 2

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-application/AppearanceDiscussions.stories.svelte catalog/components/store-threads/StoreThreads.stories.svelte -t 'FocusAndSplit|AppearanceFocus' (before)
```

**Notes**

2failures/6skipped: centered Dialog instead of labelled structural region. /tmp/jj-focus-sheet-before.log.
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
pnpm test:storybook catalog/components/store-application/AppearanceDiscussions.stories.svelte catalog/components/store-threads/StoreThreads.stories.svelte -t 'FocusAndSplit|AppearanceFocus' (first after)
```

**Notes**

2interactions passed,2runtime cleanup errors from destroyed active returnFocus getter. Captured returnFocus locally. /tmp/jj-focus-sheet-after.log.
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
pnpm test:storybook catalog/components/store-application/AppearanceDiscussions.stories.svelte catalog/components/store-threads/StoreThreads.stories.svelte -t 'FocusAndSplit|AppearanceFocus' (after2 and3)
```

**Notes**

Each2passed/6skipped. Final effect captures focusKey for cleanup. /tmp/jj-focus-sheet-after-2.log; /tmp/jj-focus-sheet-after-3.log.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-application/DiscussionSheets.stories.svelte catalog/components/store-application/AppearanceDiscussions.stories.svelte catalog/components/store-threads/StoreThreads.stories.svelte (expanded first)
```

**Notes**

8pass/2fail: hidden inert rail has no accessible name; mobile conditional sheet unmount leaves duplicate teleported surface on reopen. Read public rail DOM id and use controlled open composition, matching existing conflict sheet. /tmp/jj-focus-sheet-qualified.log.
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
pnpm test:storybook catalog/components/store-application/DiscussionSheets.stories.svelte -t MobileFocus (before controlled open)
```

**Notes**

1fail/1skip independently reproduces duplicate mobile surface after close/reopen. /tmp/jj-focus-sheet-mobile.log.
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
pnpm test:storybook catalog/components/store-application/DiscussionSheets.stories.svelte catalog/components/store-application/AppearanceDiscussions.stories.svelte catalog/components/store-threads/StoreThreads.stories.svelte (controlled open)
```

**Notes**

10/10 pass with blocking accessibility. Desktop covers both rails; mobile root8px insets, close/reopen, Escape/scrim/focus restoration and two retained Split drafts. Fresh Vitest browser/Vite process. /tmp/jj-focus-sheet-controlled.log.
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
pnpm web:check; pnpm catalog:check (initial and final)
```

**Notes**

Final zero errors/warnings;137catalog modules. /tmp/jj-focus-sheet-final-web.log; /tmp/jj-focus-sheet-final-catalog.log.
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

768production modules accepted. /tmp/jj-focus-sheet-architecture.log.
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
pnpm spec:first; pnpm spec:validate
```

**Notes**

11protected files/2chapters;5validators/21chapters/147requirements. /tmp/jj-focus-sheet-spec-first.log; /tmp/jj-focus-sheet-spec-validate.log.
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
pnpm exec prettier --write (changed source and scenario files)
```

**Notes**

Formatted shared sheet, manager, application, Thread consumer, two-draft fixture and scenarios.
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
pnpm test:storybook catalog/components/store-application/DiscussionSheets.stories.svelte catalog/components/store-application/AppearanceDiscussions.stories.svelte catalog/components/store-application/StoreCollaboration.stories.svelte catalog/components/store-application/NativeAgentMenus.stories.svelte catalog/components/store-threads catalog/components/store-work/ProjectsAgents.stories.svelte
```

**Notes**

31interactions passed but two standalone read-only captured-agent fixtures have no application focus provider. Made optional provider lookup explicit; standalone Split consumers remain usable. /tmp/jj-focus-sheet-consumers.log.
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
pnpm test:storybook catalog/components/store-application/NativeReviewDetail.stories.svelte
```

**Notes**

3/3 exact inline diff/tool review consumers. /tmp/jj-focus-sheet-tool-review.log.
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
pnpm test:storybook catalog/components/store-application/DiscussionSheets.stories.svelte catalog/components/store-application/AppearanceDiscussions.stories.svelte catalog/components/store-application/StoreCollaboration.stories.svelte catalog/components/store-application/NativeAgentMenus.stories.svelte catalog/components/store-application/NativeReviewDetail.stories.svelte catalog/components/store-threads catalog/components/store-work/ProjectsAgents.stories.svelte
```

**Notes**

Final34/34 directly affected interactions/accessibility, no runtime errors; fresh cold browser/Vite. /tmp/jj-focus-sheet-final-consumers.log.
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
pnpm web:check (optional standalone provider)
```

**Notes**

Zero errors/warnings. /tmp/jj-focus-sheet-leaf-web.log.
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
pnpm check:progress; pnpm check:ui-ownership
```

**Notes**

120task bookkeeping and owned source closure accepted.
<!-- /run -->
<!-- /evidence-field -->

