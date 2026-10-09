---
schema: 1
id: E-CHAT-LIVE-014
---

# E-CHAT-LIVE-014 details 2

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
pnpm test:storybook catalog/components/store-application/AgentInboxTree.stories.svelte catalog/components/store-application/AgentInboxNavigation.stories.svelte -t 'RepositoryGroupsRetainExactChildrenAndIndependentLinks|OfflineChatRowUsesPresenceAndASeparateRepositoryHeaderAction'
```

**Notes**

Exact before: two fail, five skipped (.local/inbox-header-link-before.log).
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
pnpm test:storybook catalog/components/store-application/AgentInboxTree.stories.svelte catalog/components/store-application/AgentInboxNavigation.stories.svelte
```

**Notes**

Initial link placement: seven pass (.local/inbox-header-link-after.log).
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
pnpm test:storybook catalog/components/store-application/AgentInboxTree.stories.svelte -t RepositoryHeaderSurfaceIncludesTheSeparateActionHover
```

**Notes**

Exact before for full row background: one fail, four skipped (.local/inbox-header-background-before.log).
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
pnpm test:storybook catalog/components/store-application/AgentInboxTree.stories.svelte catalog/components/store-application/AgentInboxNavigation.stories.svelte
```

**Notes**

Intermediate surface run: seven pass, one fails because synthetic Storybook hover does not activate native CSS :hover (.local/inbox-header-link-final-stories.log). Hover CSS was verified with real browser pointer input instead; Storybook checks tooltip behavior, focus surface and geometric clearance.
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
pnpm test:storybook catalog/components/store-application/AgentInboxTree.stories.svelte -t RepositoryHeaderSurfaceIncludesTheSeparateActionHover
```

**Notes**

Exact inset before: action width was 32px rather than the requested compact size (.local/inbox-header-inset-before.log).
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
pnpm test:storybook catalog/components/store-application/AgentInboxTree.stories.svelte -t LoadingSessionsMatchesRepositoryGroupsAndCompactChatRows
```

**Notes**

Exact loading before: one fail, five skipped because grouped header/child placeholders were absent (.local/inbox-header-skeleton-before.log).
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
pnpm test:storybook catalog/components/store-application/AgentInboxTree.stories.svelte catalog/components/store-application/AgentInboxNavigation.stories.svelte
```

**Notes**

Final complete navigation family: nine interaction/accessibility stories pass (.local/inbox-header-all-after.log), including both disclosures, compact insets, narrow resize, same-name identities, draft retention and loading-to-loaded geometry.
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
pnpm test:storybook catalog/components/store-application/AgentSessions.stories.svelte -t 'ViewSessionsKeepsEmptyInbox|ActivityLogKeepsExactChatAndItsRetainedDraft|LegacyInboxSessionsOmitUnknownActivityTime|InboxBodySkeletonWaitsForRetainedHistory'
```

**Notes**

Four directly affected history/empty/timing/loading stories pass; seven unrelated directory stories skipped (.local/inbox-header-sessions-after.log).
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
```

**Notes**

All three runs pass with zero errors/warnings, after link placement, shared surface and final compact controls/loader (.local/inbox-header-link-web-check.log; inbox-header-link-web-check-final.log; inbox-header-all-web-check.log).
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
pnpm catalog:check
```

**Notes**

All three runs pass with zero errors/warnings; 349 modules satisfy catalog boundaries (.local/inbox-header-link-catalog-check.log; inbox-header-link-catalog-check-final.log; inbox-header-all-catalog-check.log).
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
pnpm check:ui-ownership
```

**Notes**

Both intermediate and final ownership checks pass (.local/inbox-header-link-ownership.log; inbox-header-all-ownership.log).
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
pnpm exec prettier --write web/src/lib/ui/community-inbox/CommunityInboxItem.svelte web/src/lib/ui/community-inbox/CommunityInboxSessions.svelte web/src/lib/ui/community-inbox/inbox.css catalog/components/store-application/AgentInboxTree.stories.svelte catalog/components/store-application/AgentInboxNavigation.stories.svelte spec/src/current/contracts/ui/34-interactive-agent-sessions.md spec/src/current/contracts/api/41-interactive-agent-execution.md
```

**Notes**

Sources and canonical requirements formatted. Targeted formatting repeated after surface, compact controls, loader fixture and milestone edits.
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
pnpm exec prettier --check web/src/lib/ui/community-inbox/CommunityInbox.svelte web/src/lib/ui/community-inbox/CommunityInboxItem.svelte web/src/lib/ui/community-inbox/CommunityInboxSessions.svelte web/src/lib/ui/community-inbox/inbox.css catalog/fixtures/AgentInboxTreeFixture.svelte catalog/components/store-application/AgentInboxTree.stories.svelte catalog/components/store-application/AgentInboxNavigation.stories.svelte spec/src/current/contracts/ui/34-interactive-agent-sessions.md spec/src/current/contracts/api/41-interactive-agent-execution.md spec/src/current/progress/M24/index.md
```

**Notes**

All changed authored files pass formatting (.local/inbox-header-format-check.log).
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
pnpm evidence:new -- --id E-CHAT-LIVE-014 --task M24 --requirement JST-UI-012 --requirement JST-API-001 --requirement JST-GOV-002
```

**Notes**

Fresh Markdown record initialized and filled with actual verification and M24 review.
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
node .local/inbox-header-evidence.mjs
```

**Notes**

Evidence writer records actual outcomes, commands, remaining gates and milestone review.
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
pnpm progress:history
```

**Notes**

Generated M24 evidence history; unrelated M25/Bootstrap generator churn was restored.
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
pnpm exec prettier --write spec/src/current/progress/M24/index.md
```

**Notes**

Milestone prose formatted after navigation generation.
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
pnpm spec:first
```

**Notes**

Seven protected files and two canonical chapters pass (.local/inbox-header-spec-first.log).
<!-- /run -->
<!-- /evidence-field -->

