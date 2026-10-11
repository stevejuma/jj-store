---
schema: 1
id: E-AGENT-REVIEW-DRAFT-001
---

# E-AGENT-REVIEW-DRAFT-001 details 2

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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook --maxWorkers 1 catalog/components/store-application/AgentReviewDraft.stories.svelte -t DocumentReviewUsesAnExpandableCommentCard
```

**Notes**

Exact Document regression passes after the principal display fallback.
/tmp/jj-session-review-after-fixed.log.
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

Zero errors/warnings; 391-module catalog boundary passes.
/tmp/jj-session-review-types-fixed.log and
/tmp/jj-session-review-types-final.log. Final check after lazy editor guard also
passes: /tmp/jj-session-review-types-lazy-editor.log.
<!-- /run -->

<!-- run -->

### Run 3

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown
```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook --maxWorkers 1 catalog/components/store-application/AgentReviewDraft.stories.svelte catalog/components/store-application/AgentComments.stories.svelte catalog/components/store-application/AgentTranscriptSettings.stories.svelte
```

**Notes**

Initial direct-consumer run: 13 pass, four transcript-settings stories fail
because the new collapsed wrapper carried the mounted-editor marker.
/tmp/jj-session-review-consumers.log. Moved the marker onto the actual editor
card, preserving existing assertions.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook --maxWorkers 1 catalog/components/store-application/AgentReviewDraft.stories.svelte catalog/components/store-application/AgentComments.stories.svelte catalog/components/store-application/AgentTranscriptSettings.stories.svelte
```

**Notes**

Marker-only repair still failed the same four checks: the public Collapsible
hides its children without unmounting them. Added the original
draft-open/pending guard around editor contents so a collapsed review mounts no
hidden editor. /tmp/jj-session-review-consumers-fixed.log.
<!-- /run -->
<!-- /evidence-field -->
