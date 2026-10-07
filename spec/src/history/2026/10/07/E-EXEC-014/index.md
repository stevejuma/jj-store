---
schema: 1
id: E-EXEC-014
---

# E-EXEC-014

<!-- evidence-field:id markdown -->
## Id

E-EXEC-014
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-07
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M23
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-015
- JST-COLLAB-031
- JST-COLLAB-033
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-application/AgentReplyIndependence.stories.svelte
- catalog/fixtures/AgentExecutionFixture.svelte
- catalog/fixtures/agent-discussion-fixture.ts
- docs/maintenance/coverage.json
- spec/src/current/contracts/collaboration/36-interactive-agent-execution.md
- spec/src/current/contracts/ui/34-interactive-agent-sessions.md
- spec/src/current/progress/M23/history/M23/index.md
- spec/src/current/progress/M23/index.md
- spec/src/history/2026/10/07/E-EXEC-014/SUMMARY.nav.md
- spec/src/history/2026/10/07/E-EXEC-014/index.md
- spec/src/history/2026/10/07/E-EXEC-015/SUMMARY.nav.md
- spec/src/history/2026/10/07/E-EXEC-015/index.md
- spec/src/history/2026/10/07/SUMMARY.nav.md
- spec/src/history/2026/10/07/index.md
- web/src/lib/controller/agent-activity-session.ts
- web/src/lib/controller/agent-discussion-refresh.ts
- web/src/lib/controller/agent-discussions.svelte.ts
- web/src/lib/controller/agent-execution-launch-context.ts
- web/src/lib/controller/agent-execution.svelte.ts
- web/src/lib/controller/production-local-configuration.ts
- web/src/lib/ui/store-agent-execution/StoreAgentDiscussion.svelte
- web/src/lib/ui/store-agent-execution/StoreAgentInboxMessage.svelte
- web/src/lib/ui/store-agent-execution/StoreAgentReplyDraftRecovery.svelte
- web/src/lib/ui/store-agent-execution/StoreAgentTranscript.svelte
- web/tests/vite/agent-discussions.test.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/vite/agent-discussions.test.ts#an ended agent accepts ordinary
  replies after feedback refresh fails
<!-- /evidence-field -->

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/agent-discussions.test.ts
```

**Notes**

Before: ended agent with retained published target rejected reply after feedback refresh error, reporting waiting for publication.
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
pnpm web:check
```

**Notes**

First draft found readonly tree message typing and untyped snippet parameter. Fixed without weakening checks.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm catalog:check
```

**Notes**

Initial new story used unsupported Testing Library exact option. A second attempt retained one such error; all removed, exact string names retain exact matching by default.
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
pnpm check:architecture
```

**Notes**

Four iterations rejected execution-controller growth (311, 306, 305, 302 counted lines). Moved unchanged launch/activity navigation into existing owning modules; final controller stays within 300.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentReplyIndependence.stories.svelte catalog/components/store-application/AgentDiscussion.stories.svelte
```

**Notes**

First run found fixture feedback ordering unlike production; corrected selected latest revisions and chronology. Second run failed exact text lookup during edited-row replacement; assertion now waits for the persisted article content. Both original assertions are covered by the passing final stories.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/agent-discussions.test.ts web/tests/vite/agent-execution-controller.test.ts web/tests/vite/agent-runtime-preparation.test.ts web/tests/vite/local-inbox-threads.test.ts
```

**Notes**

14 tests; ended reply after failed refresh, bounded independent refresh teardown, existing exact execution and local collaboration controls.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentReplyIndependence.stories.svelte catalog/components/store-application/AgentDiscussion.stories.svelte catalog/components/store-application/AgentChatFeedback.stories.svelte catalog/components/store-application/AgentExecution.stories.svelte
```

**Notes**

Fresh Vitest/Vite/browser process: 26 stories in four directly affected families, including accessibility enforcement. Offline save, working-session nested replies/edit, retained replay draft and exact recovery pass. No installed adapter or current user-tab mutation.
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
pnpm web:check
pnpm catalog:check
pnpm check:architecture
pnpm check:ui-ownership
pnpm docs:check
```

**Notes**

Zero web/catalog type diagnostics; source budgets, Community ownership and manual links/rendering pass. Source review: pnpm docs:review application with this slice description. Scoped prettier --write prepared changed files; no Rust, public contracts or root aggregates for this UI-only slice.
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
pnpm check:progress
pnpm spec:validate
pnpm spec:first
```

**Notes**

144 tasks / 638 evidence records, seven spec validators, and 15 protected files with two canonical chapter updates pass. UI-only targeted verification; root aggregates, Rust and installed-adapter tests remain out of scope.
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
pnpm exec prettier --check catalog/components/store-application/AgentReplyIndependence.stories.svelte catalog/fixtures/AgentExecutionFixture.svelte catalog/fixtures/agent-discussion-fixture.ts docs/maintenance/coverage.json spec/src/current/contracts/collaboration/36-interactive-agent-execution.md spec/src/current/contracts/ui/34-interactive-agent-sessions.md spec/src/current/progress/M23/history/M23/index.md spec/src/current/progress/M23/index.md web/src/lib/controller/agent-activity-session.ts web/src/lib/controller/agent-discussion-refresh.ts web/src/lib/controller/agent-discussions.svelte.ts web/src/lib/controller/agent-execution-launch-context.ts web/src/lib/controller/agent-execution.svelte.ts web/src/lib/controller/production-local-configuration.ts web/src/lib/ui/store-agent-execution/StoreAgentDiscussion.svelte web/src/lib/ui/store-agent-execution/StoreAgentInboxMessage.svelte web/src/lib/ui/store-agent-execution/StoreAgentReplyDraftRecovery.svelte web/src/lib/ui/store-agent-execution/StoreAgentTranscript.svelte web/tests/vite/agent-discussions.test.ts
```

**Notes**

Changed authored sources pass scoped formatting; evidence uses the governed Markdown writer.
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

Enclosing Jujutsu change tkqnkook. macOS ARM64; Node 24.10.0, pnpm 10.32.1, JJ 0.45.1, Svelte 5.57.1, Vitest 4.1.11. Existing owned Community presentation and released Design Core.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

UI-only reply eligibility, thread presentation and freshness. No public DTO, repository canonical format, execution journal or receipt changes.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

First slice: ordinary replies independent of runner status, transient feedback failures and execution commands. Reuse Community thread rows and normal repository commands with exact edit/parent context.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Implement separately governed incremental feedback collection and private delivery tracking. Native code reducer 3 and visual baselines remain separate; Rust and root aggregate lanes are out of scope for this UI slice.
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M23 remains Done under existing gates; this maintenance slice qualifies
  ordinary discussion while stopped and during refresh failure. Incremental
  context work follows as a separate slice.

```
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: unchanged
reason: JST-API-001 public interfaces, JST-ARCH-001/JST-ARCH-004 layer and
  source ownership, JST-GOV-002/JST-GOV-008 protected boundaries and JST-DOC-002
  documentation contracts remain unchanged. New application composition reuses
  existing owned Community rows/composer; no sibling source imports or copies.
  UI and collaboration behavior changes have owning chapter updates.

```
<!-- /evidence-field -->

