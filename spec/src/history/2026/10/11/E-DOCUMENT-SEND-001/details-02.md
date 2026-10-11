---
schema: 1
id: E-DOCUMENT-SEND-001
---

# E-DOCUMENT-SEND-001 details 2

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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook --maxWorkers 1 catalog/components/store-application/AgentSessionState.stories.svelte -t "Working follows messages and drafted content selects Queue"
```

**Notes**

No test matched the display-name filter; three skipped. This is not acceptance evidence. Retried with the generated Storybook export prefix Working.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook --maxWorkers 1 catalog/components/store-application/AgentSessionState.stories.svelte -t Working (before change)
```

**Notes**

WorkingFollowsMessagesAndDraftedContentSelectsQueue fails the rendered circular-shape assertion; two other stories skipped. Log /tmp/jj-document-send-before-exact.log. Assertions also cover Stop square/destructive and idle Send horizontal/circular.
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
pnpm spec:search JST-UI-018; pnpm evidence:new -- --id E-DOCUMENT-SEND-001 --task M24 --requirement JST-UI-018 --requirement JST-API-001
```

**Notes**

Owning canonical UI and API chapters read. Evidence starter regenerated navigation globally; all 31 unrelated generated changes were restored. Exact original bytes of the 28 pre-existing pending files were explicitly restored into retained change tkyzsuoz from c9306e0b and compared without differences. Only this evidence folder/day navigation and M24 review/identifier are retained.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook --maxWorkers 1 catalog/components/store-application/AgentSessionState.stories.svelte -t Working (first post-change run)
```

**Notes**

Concurrent Prettier/Vite invalidation caused Vite to reload the test and one unhandled TypeError reading call; one failed/two skipped plus one runner error. No assertion weakened. Stable fresh rerun follows. Log /tmp/jj-document-send-after-exact.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook --maxWorkers 1 catalog/components/store-application/AgentSessionState.stories.svelte -t Working (fresh confirmation)
```

**Notes**

Exact regression passes; two unrelated stories skipped. No source edits or optimizer invalidation during this run. Log /tmp/jj-document-send-after-exact-confirm.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook --maxWorkers 1 catalog/components/store-application/AgentSessionState.stories.svelte catalog/components/store-application/QueuedAgentMessages.stories.svelte catalog/components/store-application/ConversationFooterLayout.stories.svelte
```

**Notes**

14 tests pass in three direct families with configured axe checks. Covers active Stop, typed Queue, return to idle Send, queue/comment-only submissions, pending acknowledgement/Stop, Classic interaction, cancellation/retained drafts and responsive/keyboard footer layout. Log /tmp/jj-document-send-stories.log.
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
pnpm catalog:check
```

**Notes**

Zero Svelte errors/warnings; 384 catalog boundary modules. Log /tmp/jj-document-send-types.log.
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
pnpm check:ui-ownership
```

**Notes**

Ownership validator passes; no dependency or production/catalog boundary change.
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
pnpm exec prettier --write web/src/lib/ui/store-agent-chat-body/ChatMessageFooter.svelte catalog/components/store-application/AgentSessionState.stories.svelte spec/src/current/contracts/ui/37-document-conversations.md spec/src/current/progress/M24/index.md spec/src/current/progress/M24/queued-messages-review.md; pnpm exec prettier --check (same files)
```

**Notes**

Relevant authored production/story/contract/milestone source formatted. M24 index remains 400 lines; historical evidence is generated through the owned bounded writer.
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
CUA main Storybook preview: clear active draft; complete two fixture turns; type idle draft; read rendered action styles and save screenshots
```

**Notes**

Queue: arrow-up/default variant/50% radius. Stop: square/destructive/red oklch(0.577 0.245 27.325)/8px radius. Idle Send: send-horizontal/default/50% radius and enabled with authored text. Saved document-queue-button.png, document-stop-button.png and document-idle-send-button.png under /Users/stevejuma/.codex/visualizations/2026/10/10/01a127c9-6bab-7c30-852d-d059d10f3b20/. Preview tab kept as a deliverable; no baseline comparison or update.
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
pnpm exec prettier --write spec/src/current/contracts/api/41-interactive-agent-execution.md
```

**Notes**

Owning API chapter records unchanged wire semantics alongside the explicit presentation contract.
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
pnpm check:progress
```

**Notes**

146 tasks and 749 evidence records; fresh evidence covers the exact UI/spec/milestone impact surface.
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
pnpm spec:first
```

**Notes**

Two protected files and both mapped canonical chapters pass.
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
pnpm spec:validate
```

**Notes**

Seven validators pass; all authored source/evidence remains within file budgets.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
jj commit -m "Fix Document conversation Send, Stop and Queue button styling"
```

**Notes**

Commit history was written, but resetting colocated Git HEAD failed because index.lock was held. The subsequent JJ operation re-imported the old HEAD and exposed a divergent copy of the same change. No source or pending file was lost.
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
jj diff --from mwxqqtwv/0 --to mwxqqtwv/1 --stat; jj rebase -r @ -o 13f3412816a346762e26960b748952cfd703d7b4; jj abandon 7adfc2e7ca4e51b861427789eb8d3aada71d71da
```

**Notes**

Both trees were byte-identical (zero changed files). Rebased retained main working change onto the committed copy and abandoned only the duplicate prior revision. No lockfile deletion or Git write command used.
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
jj diff --from c9306e0b --to tkyzsuoz --name-only (original pending paths); jj edit tkyzsuoz
```

**Notes**

All 28 original pending paths retain exact bytes. Main returns to the retained working change with this verified UI change as its ancestor; no temporary workspace created.
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

<!-- evidence-field:versions markdown -->
## Versions

Node 26.9.0; pnpm 10.32.1; Svelte 5.57.1; Storybook 10.5.7; Vitest 4.1.11. Foundation dependencies and manifests unchanged.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No API/DTO, native format-2 storage, ACP control version 3, execution journal, browser-storage or credential changes.
<!-- /evidence-field -->

