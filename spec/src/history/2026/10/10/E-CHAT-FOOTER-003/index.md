---
schema: 1
id: E-CHAT-FOOTER-003
---

# E-CHAT-FOOTER-003

<!-- evidence-field:id markdown -->
## Id

E-CHAT-FOOTER-003
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M24
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-015
- JST-UI-018
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-10
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- spec/src/current/contracts/api/26-browser-appearance-state.md
- spec/src/current/contracts/ui/39-intentional-thread-inbox.md
- spec/src/current/progress/M24/footer-review.md
- spec/src/current/progress/M24/history/M24/index.md
- spec/src/current/progress/M24/index.md
- spec/src/history/2026/10/10/SUMMARY.nav.md
- spec/src/history/2026/10/10/index.md
- web/src/lib/ui/community-inbox/CommunityInboxItem.svelte
- web/src/lib/ui/community-inbox/inbox.css
- web/src/lib/ui/store-agent-chat-body/ChatMessageFooter.svelte
- web/src/lib/ui/store-agent-workspace/StoreWorkspaceButton.svelte
- web/src/lib/ui/store-threads/StoreThreadAgentRepositories.svelte
- web/src/lib/ui/store-threads/StoreThreadCodebase.svelte
- web/src/lib/ui/store-threads/StoreThreadConversation.svelte
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

[]
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook --browser.api.port 63423 --browser.api.strictPort catalog/components/store-application/ConversationFooterLayout.stories.svelte catalog/components/store-application/WorkspaceFiles.stories.svelte catalog/components/store-application/InboxControls.stories.svelte catalog/components/store-application/AgentInboxTree.stories.svelte catalog/components/store-threads/StoreThreads.stories.svelte
```

**Notes**

32 focused interaction/accessibility stories passed in 55.03s across five direct families in a fresh Vitest browser process. /tmp/footer-controls-stories.log. The existing ResizeObserver loop diagnostic appeared once as an unhandled Vite/console error; it did not fail assertions or the command, but remains open. This is focused acceptance, not a full-suite or warning-free result.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook --browser.api.port 63423 --browser.api.strictPort catalog/components/store-application/InboxControls.stories.svelte catalog/components/store-application/AgentInboxTree.stories.svelte
```

**Notes**

After individual split-button hover styling: 12 stories passed cold in 34.24s. /tmp/footer-controls-split-final.log. No ResizeObserver diagnostic in this run.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook --browser.api.port 63423 --browser.api.strictPort catalog/components/store-application/InboxControls.stories.svelte catalog/components/store-application/AgentInboxTree.stories.svelte
```

**Notes**

After reducing sidebar trailing insets: all 12 directly affected stories passed cold in 34.99s. /tmp/footer-controls-sidebar-final.log. No ResizeObserver diagnostic in this run. The other 20 previously passing cases are unchanged by this inbox-only refinement.
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

Zero errors and warnings. /tmp/footer-controls-web.log. Final added inbox rules only change CSS presentation.
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
pnpm catalog:check
```

**Notes**

Zero errors/warnings; catalog boundaries pass for 367 modules. /tmp/footer-controls-catalog.log.
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
pnpm check:ui-ownership
```

**Notes**

One ownership validator passed for 3475 chapters, 181 requirements. /tmp/footer-controls-ownership.log. Production uses owned Community compositions and released Design Core primitives; no sibling runtime, type or catalog imports added.
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
pnpm exec prettier --write web/src/lib/ui/store-threads/StoreThreadConversation.svelte web/src/lib/ui/store-threads/StoreThreadAgentRepositories.svelte web/src/lib/ui/store-threads/StoreThreadCodebase.svelte web/src/lib/ui/store-agent-workspace/StoreWorkspaceButton.svelte web/src/lib/ui/store-agent-chat-body/ChatMessageFooter.svelte web/src/lib/ui/community-inbox/CommunityInboxItem.svelte spec/src/current/contracts/ui/39-intentional-thread-inbox.md spec/src/current/contracts/api/26-browser-appearance-state.md
```

**Notes**

Formatted the initial production and canonical changes.
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
pnpm exec prettier --write web/src/lib/ui/community-inbox/inbox.css spec/src/current/contracts/ui/39-intentional-thread-inbox.md spec/src/current/contracts/api/26-browser-appearance-state.md spec/src/current/progress/M24/footer-review.md
```

**Notes**

Formatted the additional sidebar insets and M24 review. Split hover component also passes its exact prettier --check.
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
node /tmp/footer-controls-evidence.mjs
```

**Notes**

Generated bounded fresh evidence and targeted day/M24 history navigation without unrelated document generation.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
```

**Notes**

Initial evidence files list was collected before adding the M24 index/history rows, leaving those two paths without fresh coverage. Refreshed the complete changed-path coverage; no product failure or contract waiver.
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
pnpm spec:validate --only summary,fileLimits,governance,verification,book
```

**Notes**

All five focused document validators pass, 3477 chapters, 181 requirements. /tmp/footer-controls-spec.log.
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
pnpm web:check
pnpm catalog:check
```

**Notes**

Final sources including sidebar/split CSS: zero errors and warnings, 367 catalog modules audited. /tmp/footer-controls-web-final.log and /tmp/footer-controls-catalog-final.log.
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
pnpm check:progress
```

**Notes**

Final coverage repair passes: 146 tasks, 732 evidence records at 7b2546b3; bookkeeping only. /tmp/footer-controls-progress-final.log.
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
pnpm exec prettier --check web/src/lib/ui/community-inbox/CommunityInboxItem.svelte web/src/lib/ui/community-inbox/inbox.css web/src/lib/ui/store-agent-chat-body/ChatMessageFooter.svelte web/src/lib/ui/store-agent-workspace/StoreWorkspaceButton.svelte web/src/lib/ui/store-threads/StoreThreadAgentRepositories.svelte web/src/lib/ui/store-threads/StoreThreadCodebase.svelte web/src/lib/ui/store-threads/StoreThreadConversation.svelte spec/src/current/contracts/api/26-browser-appearance-state.md spec/src/current/contracts/ui/39-intentional-thread-inbox.md spec/src/current/progress/M24/footer-review.md spec/src/current/progress/M24/index.md spec/src/current/progress/M24/history/M24/index.md
```

**Notes**

Every changed production source, canonical topic and M24 review/index passes formatting. /tmp/footer-controls-format-final.log. Evidence pages use the bounded canonical serializer.
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
pnpm spec:first --base 952a6fcf --head 7b2546b34784fc95afe12f50433ab8b10f232dd4
```

**Notes**

Protected presentation changes covered by UI/API topics and fresh evidence; /tmp/footer-controls-spec-first.log.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pass
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

[]
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-CHAT-FOOTER-003 details 2](details-02.md)
