---
schema: 1
id: E-AGENT-TRANSCRIPT-VIEW-001
---

# E-AGENT-TRANSCRIPT-VIEW-001

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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook --browser.api.port 63422 --browser.api.strictPort catalog/components/store-application/AgentPromptMarkup.stories.svelte -t OpenCodeTypedTextMessageParts
```

**Notes**

One passed, nine skipped; 13.63s. The unchanged Classic submenu/clipboard test passes in its fresh exact recheck. /tmp/transcript-copy-recheck.log.
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
node /tmp/transcript-evidence.mjs
```

**Notes**

The first local evidence-navigation refresh supplied link objects instead of Markdown lines. Corrected the helper arguments and reran successfully; no production change.
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
node /tmp/transcript-evidence.mjs
```

**Notes**

Fresh evidence pages and narrow navigation generated without unrelated ledger churn.
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
pnpm exec prettier --check catalog/components/store-application/AgentComments.stories.svelte catalog/components/store-application/AgentPromptMarkup.stories.svelte catalog/components/store-application/AgentTranscriptSettings.stories.svelte catalog/components/store-application/NativeReviewDetail.stories.svelte catalog/components/store-application/agent-sidebar-play.ts catalog/components/store-application/appearance-tool-discussion-play.ts catalog/components/store-work/ProjectsAgents.stories.svelte catalog/fixtures/AgentLayoutFixture.svelte catalog/fixtures/AgentLegacyTranscriptFixture.svelte catalog/fixtures/AgentPromptFixture.svelte catalog/fixtures/StoreCapturedReactionsFixture.svelte spec/src/current/contracts/api/26-browser-appearance-state.md spec/src/current/contracts/ui/37-document-conversations.md spec/src/current/progress/M24/footer-review.md spec/src/current/progress/M24/history/M24/index.md spec/src/current/progress/M24/index.md spec/src/history/2026/10/10/index.md web/src/lib/conversations/recorded-adapter.ts web/src/lib/ui/store-collaboration/StoreAgentComments.svelte web/src/lib/ui/store-collaboration/StoreAgentContext.svelte web/src/lib/ui/store-collaboration/StoreAgentSidebar.svelte web/src/lib/ui/store-collaboration/StoreAgentTranscript.svelte web/src/lib/ui/store-collaboration/StoreRecordedConversation.svelte web/tests/vite/conversation-projection.test.ts
```

**Notes**

Focused changed sources and canonical/review/day-index Markdown pass. /tmp/transcript-format.log. Evidence is bounded through the repository serializer.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook --browser.api.port 63422 --browser.api.strictPort catalog/components/store-application/AgentTranscriptSettings.stories.svelte catalog/components/store-work/ProjectsAgents.stories.svelte catalog/components/store-application/AgentComments.stories.svelte catalog/components/store-application/AgentLayout.stories.svelte catalog/components/store-application/StoreCollaboration.stories.svelte catalog/components/store-application/AppearanceDiscussions.stories.svelte catalog/components/store-work/NativeActivity.stories.svelte catalog/components/store-application/NativeReviewDetail.stories.svelte catalog/components/store-threads/StoreCapturedReactions.stories.svelte catalog/components/store-application/AgentPromptMarkup.stories.svelte
```

**Notes**

Final cold pre-copy audit: 56 stories passed across ten direct families, 75.28s. /tmp/transcript-cold-final.log. No assertion or baseline was weakened.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook --browser.api.port 63422 --browser.api.strictPort catalog/components/store-application/AgentTranscriptSettings.stories.svelte catalog/components/store-work/ProjectsAgents.stories.svelte catalog/components/store-application/AgentComments.stories.svelte catalog/components/store-application/AgentLayout.stories.svelte catalog/components/store-application/StoreCollaboration.stories.svelte catalog/components/store-application/AppearanceDiscussions.stories.svelte catalog/components/store-work/NativeActivity.stories.svelte catalog/components/store-application/NativeReviewDetail.stories.svelte catalog/components/store-threads/StoreCapturedReactions.stories.svelte catalog/components/store-application/AgentPromptMarkup.stories.svelte
```

**Notes**

Delivery source audit: 56 passed, one new action-bar story failed, 82.06s. /tmp/transcript-copy-final.log. Storybook DOM hover does not activate the native CSS hover pseudo-class; retained visibility assertions and used the accessible reading-editor focus path, which exercises the same Document action bar. The old Classic wrapper would remain hidden through this path; it is replaced by the owned record menu.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook --browser.api.port 63422 --browser.api.strictPort catalog/components/store-application/AgentTranscriptSettings.stories.svelte
```

**Notes**

All nine transcript-setting stories pass cold in 25.62s, including the copy identity action bar. /tmp/transcript-actions-pass.log. Combined with the unchanged 48 direct-consumer cases in the preceding cold group, all 57 focused story cases are verified. This is focused consumer acceptance, not a repository-wide suite.
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
node /tmp/transcript-finish-evidence.mjs
```

**Notes**

Temporary finalization helper matched -t within the store-threads file path and selected no wide command. Corrected to the explicit CLI token and reran; no product change.
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
pnpm web:check
pnpm catalog:check
```

**Notes**

Delivery production and catalog checks: zero errors and warnings; catalog boundary audit passes. /tmp/transcript-web-delivery.log and /tmp/transcript-catalog-delivery.log.
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
pnpm spec:validate --only summary,fileLimits,governance,book
```

**Notes**

Four focused document validators pass, 3475 chapters, 181 requirements. /tmp/transcript-document-structure.log.
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
pnpm check:progress
```

**Notes**

Final evidence/progress bookkeeping passes, 146 tasks and 731 records; M24 status is unchanged. /tmp/transcript-progress-final.log.
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
pnpm spec:validate --only summary,fileLimits,governance,verification,book
```

**Notes**

All five focused document validators pass, 3475 chapters/181 requirements. /tmp/transcript-spec-final.log.
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
pnpm catalog:check
```

**Notes**

Last test-source check: zero errors/warnings and 367-module boundary audit passes. /tmp/transcript-catalog-last.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook --browser.api.port 63422 --browser.api.strictPort catalog/components/store-application/AgentTranscriptSettings.stories.svelte
```

**Notes**

Before: all three initial regression stories failed in 25.10s. Projects landing, selected project and repository responses ignored Document or mounted the Classic session footer. /tmp/transcript-setting-before.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook --browser.api.port 63422 --browser.api.strictPort catalog/components/store-application/AgentTranscriptSettings.stories.svelte
```

**Notes**

Initial narrow repair: three stories passed in 22.24s. /tmp/transcript-setting-after.log. Later cases expand the final cold acceptance below.
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: fail
failures: 9
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook --browser.api.port 63422 --browser.api.strictPort catalog/components/store-application/AgentTranscriptSettings.stories.svelte catalog/components/store-work/ProjectsAgents.stories.svelte catalog/components/store-application/AgentComments.stories.svelte catalog/components/store-application/AgentLayout.stories.svelte catalog/components/store-application/StoreCollaboration.stories.svelte catalog/components/store-application/AppearanceDiscussions.stories.svelte catalog/components/store-work/NativeActivity.stories.svelte catalog/components/store-application/NativeReviewDetail.stories.svelte catalog/components/store-threads/StoreCapturedReactions.stories.svelte
```

**Notes**

Consumer audit: 34 passed, 9 failed, 56.88s. Classic-only tool review, footer and layout assertions implicitly used the prior renderer. Updated those tests to explicitly select Classic; kept new default-Document coverage. /tmp/transcript-consumers.log.
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook --browser.api.port 63422 --browser.api.strictPort catalog/components/store-application/AgentTranscriptSettings.stories.svelte catalog/components/store-work/ProjectsAgents.stories.svelte catalog/components/store-application/AgentComments.stories.svelte catalog/components/store-application/AgentLayout.stories.svelte catalog/components/store-application/StoreCollaboration.stories.svelte catalog/components/store-application/AppearanceDiscussions.stories.svelte catalog/components/store-work/NativeActivity.stories.svelte catalog/components/store-application/NativeReviewDetail.stories.svelte catalog/components/store-threads/StoreCapturedReactions.stories.svelte
```

**Notes**

Next audit: 41 passed, 2 failed, 68.86s. Projects geometry now measures the first document item including its recorded date divider; the prior article offset excluded that content. Embedded native review assertion now uses the shared rich tool label Edited file. /tmp/transcript-consumers-final.log.
<!-- /run -->

<!-- run -->
### Run 18

```yaml
result: fail
failures: 8
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook --browser.api.port 63422 --browser.api.strictPort catalog/components/store-application/AgentTranscriptSettings.stories.svelte catalog/components/store-work/ProjectsAgents.stories.svelte catalog/components/store-application/AgentComments.stories.svelte catalog/components/store-application/AgentLayout.stories.svelte catalog/components/store-application/StoreCollaboration.stories.svelte catalog/components/store-application/AppearanceDiscussions.stories.svelte catalog/components/store-work/NativeActivity.stories.svelte catalog/components/store-application/NativeReviewDetail.stories.svelte catalog/components/store-threads/StoreCapturedReactions.stories.svelte catalog/components/store-application/AgentPromptMarkup.stories.svelte
```

**Notes**

Expanded direct consumers: 46 passed, 8 failed, 79.84s. AgentPromptMarkup protects Classic HTML/strong/pre markup and its native copy menus. It now explicitly selects Classic. Added separate Document prompt-markup coverage and preserved inert full-source prompt rendering for capability-limited captures. /tmp/transcript-consumers-verified.log.
<!-- /run -->
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-AGENT-TRANSCRIPT-VIEW-001 details 2](details-02.md)
- [E-AGENT-TRANSCRIPT-VIEW-001 details 3](details-03.md)
