---
schema: 1
id: E-TEST-REPAIR-002
---

# E-TEST-REPAIR-002 details 2

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 10
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-application/AgentDiscussion.stories.svelte catalog/components/store-application/AgentReplyIndependence.stories.svelte
```

**Notes**

Inherited unchanged baseline recorded in E-TEST-REPAIR-001: all ten legacy reply cases fail. No production or runner change since that baseline. User explicitly authorized replacing legacy assertions.
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
pnpm spec:search 'JST-UI-015'
```

**Notes**

QMD native-module Node ABI147 incompatibility remains. Used rg and read canonical ui/03-requirements, ui/34-interactive-agent-sessions, ui/37-document-conversations and collaboration/43-intentional-threads-and-codebases instead.
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
pnpm catalog:check
```

**Notes**

Initial dedicated fixture used the wrong AppShell.Sidebar prop. Fixed to the public side/sidebarController props; no type shim or dependency edit.
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
pnpm test:storybook catalog/components/store-application/AgentDiscussion.stories.svelte
```

**Notes**

Four passed. The first fixture run also reported five stream-close teardown errors. Fixed fixture cancellation ownership. Corrected exact UTF-8 wire offsets to strings and synchronized the temporary replay picker closure.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-application/AgentDiscussion.stories.svelte catalog/components/store-application/AgentReplyIndependence.stories.svelte
```

**Notes**

Seven passed with no unhandled errors. Offline keyboard insertion started at the clicked cursor; explicit End used. Replaced stale context payload and grouped-tools assumptions. Replay-specific story subsequently replaced with current independent-anchor drafts; no legacy assertions retained.
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
pnpm test:storybook catalog/components/store-application/AgentReplyIndependence.stories.svelte
```

**Notes**

Two passed. Closed read-sidebar comments have no visible Edit button; keep the read sidebar open for publication acceptance. Current rich diff label is Edited file.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-application/AgentDiscussion.stories.svelte catalog/components/store-application/AgentReplyIndependence.stories.svelte
```

**Notes**

Nine passed. Remaining raw-toggle visibility assertion did not activate its hover/focus affordance.
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
pnpm test:storybook catalog/components/store-application/AgentDiscussion.stories.svelte catalog/components/store-application/AgentReplyIndependence.stories.svelte
```

**Notes**

Nine passed again. Waiting alone did not reveal the raw toggle: it is focus/hover-only, not a sidebar animation delay. The story now explicitly focuses it and retains the visibility assertion.
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
pnpm test:storybook catalog/components/store-application/AgentDiscussion.stories.svelte catalog/components/store-application/AgentReplyIndependence.stories.svelte
```

**Notes**

Final fresh Vitest/Chromium process: 10/10 current stories pass, including Axe and unchanged runtime diagnostic guard. No unhandled errors or ResizeObserver warnings observed. Complete direct consumers of the new dedicated fixture/helper.
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
pnpm catalog:check
```

**Notes**

Zero type errors/warnings, but catalog ownership audit rejected a direct API-module type import. Import Model from the existing controller thread-types surface; no audit exception or type shim.
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
node scripts/check-catalog.mjs
```

**Notes**

373 catalog modules satisfy transport/ownership boundaries.
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

Final: zero errors/warnings; 373 catalog modules satisfy boundaries after the import correction and exact per-revision fixture batch matching.
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
pnpm exec prettier --write catalog/components/store-application/AgentDiscussion.stories.svelte catalog/components/store-application/AgentReplyIndependence.stories.svelte catalog/fixtures/AgentInlineCommentsFixture.svelte catalog/fixtures/agent-inline-comment-play.ts catalog/fixtures/agent-inline-comment-transport.ts spec/src/current/progress/M24/index.md spec/src/current/progress/M24/SUMMARY.index-index/002/index.md spec/src/current/progress/M24/test-repairs-review.md
```

**Notes**

Formatted changed authored files. Historical evidence follows its structural generator, as specified by prettierignore.
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
pnpm test:storybook catalog/components/store-application/AgentDiscussion.stories.svelte catalog/components/store-application/AgentReplyIndependence.stories.svelte
```

**Notes**

Final verification after exact per-revision fixture batch matching: both full direct-consumer families, 10/10 stories pass in a fresh browser process; no unhandled errors or ResizeObserver warnings. Interaction/Axe/runtime diagnostics remain enabled.
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
pnpm catalog:test web/tests/vite/agent-discussions.test.ts web/tests/vite/conversation-projection.test.ts web/tests/vite/conversation-reactions.test.ts web/tests/vite/conversation-session.test.ts
```

**Notes**

20/20 focused controller/projection/session/reaction tests pass in four files. Production remains unchanged; exact-source, deduplication, offline/API replies, unknown publication, per-revision edits and immutable reaction identities retain their independent proofs.
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

Refreshed history for the new evidence. Retained M24/new-evidence changes and restored 15 unrelated generated normalizations. Original evidence/results remain unchanged.
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
pnpm check:ui-ownership
```

**Notes**

Specification UI ownership validator passes; production/catalog and copied-source boundaries unchanged.
<!-- /run -->

<!-- run -->
### Run 18

```yaml
result: fail
failures: 10
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
```

**Notes**

Evidence used Vitest export identifiers instead of literal Story name values. Fixed named evidence references to canonical display names, including the ten retired cases. No assertions or validator rules changed.
<!-- /run -->
<!-- /evidence-field -->

