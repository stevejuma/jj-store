---
schema: 1
id: E-UI-105
---

# E-UI-105 details 2

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
tests: 3
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec tsx --test web/tests/agent-message-content.test.ts
```

**Notes**

Before implementation: ordered OpenCode text still rendered as JSON. Two fallback regressions passed. Log: .local/agent-text-before.log.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
tests: 1
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreCollaboration.stories.svelte -t AgentCollectionTypography
```

**Notes**

Before styling fix: heading was 14px against a 12px count in the catalog; nine stories skipped. Live heading measured 16px against the 12px secondary token. Log: .local/agent-font-before.log.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: pass
tests: 10
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec tsx --test web/tests/agent-message-content.test.ts web/tests/collaboration-presentation.test.ts
```

**Notes**

After implementation and final repeat: ordered safe Markdown text, stable identity and conservative JSON/literal fallback pass alongside existing presentation tests. Logs: .local/agent-text-after.log and .local/agent-text-final-unit.log.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: pass
tests: 1
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreCollaboration.stories.svelte -t AgentCollectionTypography
```

**Notes**

Exact heading regression passes; nine stories skipped. Log: .local/agent-font-after.log.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
tests: 2
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentPromptMarkup.stories.svelte -t OpenCode
```

**Notes**

First story assertion incorrectly expected live fixture author Local rather than catalog author Maya Chen; one fallback story passed and seven skipped. Corrected the exact author assertion. Log: .local/agent-parts-stories.log.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
tests: 2
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentPromptMarkup.stories.svelte -t OpenCode
```

**Notes**

Second attempt reached the correct message but clicked the hover menu before data-actions-visible; added the established visibility wait. One fallback story passed and seven skipped. Log: .local/agent-parts-stories-final.log.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: pass
tests: 2
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentPromptMarkup.stories.svelte -t OpenCode
```

**Notes**

Both exact regressions pass; seven stories skipped. Raw captured JSON remains the Copy message argument. Log: .local/agent-parts-stories-verified.log.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: pass
tests: 1
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:check
```

**Notes**

Zero errors and warnings. Log: .local/agent-text-web-check.log.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
tests: 1
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm catalog:check
```

**Notes**

Two new story type errors: Testing Library role options do not support exact. Replaced these with anchored name expressions. Log: .local/agent-text-catalog-check.log.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
tests: 1
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm catalog:check
```

**Notes**

Type checking passed, but the fixture imported API/client types directly. Removed that boundary violation: integration derives from existing presentation fixture data and mediaType is the fixture’s supported text/json input. Log: .local/agent-text-catalog-check-fixed.log.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: pass
tests: 1
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm catalog:check
```

**Notes**

Zero errors and warnings; catalog boundaries pass. Log: .local/agent-text-catalog-check-final.log.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: fail
tests: 12
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work catalog/components/store-threads catalog/components/store-application/StoreCollaboration.stories.svelte catalog/components/store-application/AgentPromptMarkup.stories.svelte catalog/components/store-application/AgentComments.stories.svelte catalog/components/store-application/AgentExecution.stories.svelte catalog/components/store-application/AgentLayout.stories.svelte catalog/components/store-application/AgentToolGroups.stories.svelte catalog/components/store-application/NativeAgentMenus.stories.svelte catalog/components/store-application/NativeMessageActions.stories.svelte catalog/components/store-application/NativeReviewDetail.stories.svelte catalog/components/store-application/RepositoryReview.stories.svelte
```

**Notes**

Overbroad diagnostic interrupted after the Review refinements and prompt families reported one failure each; other files were not qualified by this interrupted run. The fresh isolated prompt family subsequently passes all nine stories. The unrelated Review refinements Op Log status assertion remains a separate diagnostic gap. Log: .local/agent-text-consumer-stories.log.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: pass
tests: 9
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentPromptMarkup.stories.svelte
```

**Notes**

Complete directly affected prompt fixture family, including activity, fallback, original JSON copy, safe Markdown/XML and unknown timing; fresh browser/server process. Log: .local/agent-parts-family.log.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: pass
tests: 43
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreCollaboration.stories.svelte catalog/components/store-application/AgentComments.stories.svelte catalog/components/store-application/AgentExecution.stories.svelte catalog/components/store-application/AgentLayout.stories.svelte catalog/components/store-application/AgentToolGroups.stories.svelte catalog/components/store-application/NativeAgentMenus.stories.svelte catalog/components/store-application/NativeMessageActions.stories.svelte catalog/components/store-threads/StoreCapturedReactions.stories.svelte
```

**Notes**

Eight complete Agent transcript/context/copy/reaction consumer families pass. Log: .local/agent-text-transcript-stories.log.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: pass
tests: 71
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work
```

**Notes**

Fifteen complete Projects/Tasks/activity/directory families pass. Shared collection CSS is used beyond Agents, and inline activity consumers render the transcript; this direct-consumer coverage is the reason for escalation. Log: .local/agent-heading-work-stories.log.
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: pass
tests: 19
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-threads/StoreThreads.stories.svelte catalog/components/store-threads/StoreThreadAlignment.stories.svelte catalog/components/store-threads/StoreThreadNavigation.stories.svelte catalog/components/store-threads/StoreThreadSearch.stories.svelte catalog/components/store-application/NativeReviewDetail.stories.svelte
```

**Notes**

Five complete discussion/review collection consumer families pass, including accessible presentation. Log: .local/agent-heading-review-collection-stories.log.
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: fail
tests: 1
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/RepositoryReview.stories.svelte -t ShellAndRowLocalGraphSpacing
```

**Notes**

Separate diagnostic reproduces an absent role=status assertion after switching to Op Log; two stories skipped. The story and Op Log production status rendering are unchanged by this slice. This is outside the heading/Agent text acceptance surface and is not claimed as passing. Log: .local/agent-heading-review-diagnostic.log.
<!-- /run -->
<!-- /evidence-field -->

