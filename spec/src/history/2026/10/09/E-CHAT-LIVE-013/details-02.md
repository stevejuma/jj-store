---
schema: 1
id: E-CHAT-LIVE-013
---

# E-CHAT-LIVE-013 details 2

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
pnpm test:storybook catalog/components/store-application/AgentInboxTree.stories.svelte
```

**Notes**

Corrected before fixture: three genuine missing-tree failures, no unhandled errors (.local/inbox-tree-before-final.log). Initial shell fixture attempt also failed with three unhandled errors (.local/inbox-tree-before.log).
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
pnpm exec tsx --test --test-name-pattern 'inbox repository links' web/tests/managed-agent.test.ts
```

**Notes**

Before: repository.id was undefined (.local/inbox-adapter-before.log).
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
pnpm test:storybook catalog/components/store-application/AgentInboxTree.stories.svelte
```

**Notes**

First four-story after run: two pass, two fail for long-label overflow and asynchronous disclosure visibility (.local/inbox-tree-after.log). Intermediate runs each passed three and failed the disclosure assertion before it awaited teardown: after-2, refresh-debug-2 and refresh-debug-3 logs. No final visibility expectation was weakened.
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
pnpm test:storybook catalog/components/store-application/AgentInboxTree.stories.svelte -t CollapseSurvivesRefreshAndExactSelectionRevealsTheGroup
```

**Notes**

Isolated disclosure inspection: one pass, three skipped (.local/inbox-tree-refresh-debug.log). Full-family rerun remained required.
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
pnpm test:storybook catalog/components/store-application/AgentInboxTree.stories.svelte catalog/components/store-application/AgentInboxNavigation.stories.svelte
```

**Notes**

Intermediate family: six pass, one asynchronous disclosure visibility failure (.local/inbox-tree-family.log).
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
pnpm test:storybook catalog/components/store-application/AgentInboxTree.stories.svelte catalog/components/store-application/AgentInboxNavigation.stories.svelte
```

**Notes**

Final complete surface: seven interaction/accessibility stories pass (.local/inbox-tree-family-complete.log), including empty-list reload and hide/show. Earlier final run also passed seven (.local/inbox-tree-family-final.log). Each run uses a fresh Vitest/Vite browser process.
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
pnpm test:storybook catalog/components/store-application/AgentSessions.stories.svelte -t 'ViewSessionsKeepsEmptyInbox|ActivityLogKeepsExactChatAndItsRetainedDraft|LegacyInboxSessionsOmitUnknownActivityTime|InboxBodySkeletonWaitsForRetainedHistory'
```

**Notes**

Four directly affected empty/history/timing/loading interaction/accessibility stories pass; seven unrelated directory stories skipped (.local/inbox-tree-sessions.log).
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
pnpm exec tsx --test --test-name-pattern 'inbox repository links' web/tests/managed-agent.test.ts
```

**Notes**

Exact before regression now passes: one test (.local/inbox-adapter-after.log).
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
pnpm exec tsx --test web/tests/agent-inbox-groups.test.ts web/tests/agent-inbox-layout.test.ts
```

**Notes**

Four tests pass for exact IDs, same-name groups, source order, immutable per-chat links, empty input and retained/blocked layout storage (.local/inbox-groups-after.log).
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
pnpm web:check
```

**Notes**

Initial and final checks report zero errors and warnings (.local/inbox-tree-web-check.log; inbox-tree-web-check-final.log).
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
pnpm catalog:check
```

**Notes**

Initial check caught the new fixture calling createStoreShell with one argument instead of its two-argument public contract (.local/inbox-tree-catalog-check.log).
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
pnpm catalog:check
```

**Notes**

Corrected fixture and final retained-group implementation: zero errors/warnings; 349 catalog modules satisfy boundaries (.local/inbox-tree-catalog-check-complete.log). Earlier corrected run also passes (.local/inbox-tree-catalog-check-final.log).
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
pnpm check:ui-ownership
```

**Notes**

New maintained Community navigation adaptation is fully attributed and production remains catalog/checkout-independent (.local/inbox-tree-ownership.log).
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
pnpm exec prettier --write web/src/lib/ui/community-inbox/CommunityInbox.svelte web/src/lib/ui/community-inbox/CommunityInboxSessions.svelte web/src/lib/ui/community-inbox/CommunityInboxItem.svelte web/src/lib/ui/community-inbox/inbox.css web/src/lib/ui/community-inbox/model.ts web/src/lib/ui/store-agent-execution/session-presentation.ts web/tests/managed-agent.test.ts catalog/components/store-application/AgentInboxTree.stories.svelte catalog/fixtures/AgentInboxTreeFixture.svelte docs/ui-provenance.json spec/src/current/contracts/api/41-interactive-agent-execution.md spec/src/current/contracts/ui/34-interactive-agent-sessions.md spec/src/current/governance/spec-governance.md
```

**Notes**

Changed sources formatted. Targeted formatting repeated after follow-up edits, including web/tests/agent-inbox-groups.test.ts.
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
pnpm exec storybook dev --host 127.0.0.1 --port 6048 --no-open --disable-telemetry
```

**Notes**

Fresh supplemental Storybook process starts for browser inspection; no production startup changes (.local/inbox-tree-storybook.log).
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
pnpm evidence:new -- --id E-CHAT-LIVE-013 --task M24 --requirement JST-UI-012 --requirement JST-API-001 --requirement JST-GOV-008 --requirement JST-GOV-002
```

**Notes**

Fresh Markdown evidence initialized and filled with actual scope and results.
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
node .local/inbox-tree-evidence.mjs
```

**Notes**

Recorded actual commands, results, open gates and M24 review through the evidence writer.
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
pnpm progress:history
```

**Notes**

Generated task history. Unrelated M25/Bootstrap generator changes were restored to preserve existing work.
<!-- /run -->
<!-- /evidence-field -->

