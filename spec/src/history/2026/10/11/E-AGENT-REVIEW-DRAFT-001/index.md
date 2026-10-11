---
schema: 1
id: E-AGENT-REVIEW-DRAFT-001
---

# E-AGENT-REVIEW-DRAFT-001

<!-- evidence-field:id markdown -->

## Id

E-AGENT-REVIEW-DRAFT-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->

## Tasks

- M9.5

<!-- /evidence-field -->

<!-- evidence-field:requirements list -->

## Requirements

- JST-UI-015
- JST-COLLAB-005
- JST-API-001

<!-- /evidence-field -->

<!-- evidence-field:date markdown -->

## Date

2026-10-11
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->

## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->

## Revision

sxxpmnrx
<!-- /evidence-field -->

<!-- evidence-field:files list -->

## Files

- catalog/components/store-application/AgentComments.stories.svelte
- catalog/components/store-application/AgentReviewDraft.stories.svelte
- catalog/components/store-application/AgentTranscriptSettings.stories.svelte
- docs/ui-provenance.json
- spec/src/current/contracts/api/23-browser-review-edit-adaptation.md
- spec/src/current/contracts/ui/02-repository-owned-community-presentation.md
- spec/src/current/contracts/ui/19-native-collaboration-action-placement.md
- spec/src/current/progress/M9/index.md
- web/src/lib/ui/store-collaboration/StoreAgentComments.svelte

<!-- /evidence-field -->

<!-- evidence-field:tests list -->

## Tests

[]
<!-- /evidence-field -->

<!-- evidence-field:story_files list -->

## Story files

- catalog/components/store-application/AgentReviewDraft.stories.svelte
- catalog/components/store-application/AgentComments.stories.svelte
- catalog/components/store-application/AgentTranscriptSettings.stories.svelte

<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->

## Scope

UI-only main-workspace presentation: stored native session responses offer a
collapsed Add review disclosure with a review icon and chevron. The opened draft
reuses the maintained muted comment context, white avatar/editor card and
existing Markdown composer/toolbar. Document reviews stay after the transcript;
Classic reviews keep their footer placement. Collapse lazily unmounts the editor
while retaining its exact draft and session; pending requests keep the editor
and receipt control visible. Native permission, anchors, submission, receipt
recovery and request identity are unchanged.
<!-- /evidence-field -->

<!-- evidence-field:before_state markdown -->

## Before state

The Document response exposed Add session review comment as a secondary button
without the comment-card disclosure styling. Classic immediately mounted a
separate composer. The exact new Document story failed because Add review did
not exist (one failure); the original action was present in the before DOM.
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
pnpm spec:search JST-UI-017; pnpm spec:search JST-UI-015; pnpm spec:search JST-COLLAB-005
```

**Notes**

Initial UI-017 lookup was refined to the owning UI-015 and COLLAB-005 contract.
Read UI 02/19, API 23 and M9.5; inspect Community transcript/comment/review
compositions first, Design Core collapsible and composer source second, then
maintained comment-card and composer integrations. Official Collapsible docs
read through curl -fsSL
https://shadcn-svelte.com/docs/components/collapsible.md. Logs
/tmp/jj-session-review-requirement.log,
/tmp/jj-session-review-ui-requirement.log,
/tmp/jj-session-review-collab-requirement.log.
<!-- /run -->

<!-- run -->

### Run 2

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown
```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook --maxWorkers 1 catalog/components/store-application/AgentReviewDraft.stories.svelte -t DocumentReviewUsesAnExpandableCommentCard
```

**Notes**

Qualified before state: missing Add review trigger. One story failed; one
Classic story skipped by the exact filter. /tmp/jj-session-review-before.log.
<!-- /run -->

<!-- run -->

### Run 3

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown
```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook --maxWorkers 1 catalog/components/store-application/AgentReviewDraft.stories.svelte -t DocumentReviewUsesAnExpandableCommentCard
```

**Notes**

Initial implementation: one story failure plus one unhandled runtime error from
a fixture repository with no principal. Added local display fallback without
changing permission checks. /tmp/jj-session-review-after.log.
<!-- /run -->

<!-- run -->

### Run 4

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

Same missing-principal case exposed a string | undefined avatar-helper type
error. /tmp/jj-session-review-types.log.
<!-- /run -->
<!-- /evidence-field -->
<!-- generated-navigation -->

## Pages

- [E-AGENT-REVIEW-DRAFT-001 details 2](details-02.md)
- [E-AGENT-REVIEW-DRAFT-001 details 3](details-03.md)
