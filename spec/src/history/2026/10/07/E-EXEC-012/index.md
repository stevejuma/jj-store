---
schema: 1
id: E-EXEC-012
---

# E-EXEC-012

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

<!-- evidence-field:files list -->
## Files

- catalog/components/store-application/AgentDiscussion.stories.svelte
- catalog/fixtures/AgentExecutionFixture.svelte
- catalog/fixtures/agent-discussion-fixture.ts
- catalog/fixtures/agent-execution-events.ts
- docs/maintenance/coverage.json
- spec/src/current/contracts/collaboration/36-interactive-agent-execution.md
- spec/src/current/contracts/ui/34-interactive-agent-sessions.md
- spec/src/current/progress/M23/history/M23/index.md
- spec/src/current/progress/M23/index.md
- spec/src/history/2026/10/07/E-EXEC-012/SUMMARY.nav.md
- spec/src/history/2026/10/07/E-EXEC-012/details-02.md
- spec/src/history/2026/10/07/E-EXEC-012/index.md
- spec/src/history/2026/10/07/E-EXEC-013/SUMMARY.nav.md
- spec/src/history/2026/10/07/E-EXEC-013/index.md
- spec/src/history/2026/10/07/SUMMARY.nav.md
- spec/src/history/2026/10/07/index.md
- web/src/lib/ui/store-agent-execution/StoreAgentInboxMessage.svelte
- web/src/lib/ui/store-agent-execution/StoreAgentReplyDraftRecovery.svelte
- web/src/lib/ui/store-agent-execution/StoreAgentTranscript.svelte
- web/src/lib/ui/store-agent-execution/presentation.ts
- web/tests/agent-execution.test.ts
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

[]
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pass
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Enclosing Jujutsu change ztkrmlsz. macOS ARM64; Node 24.10.0, pnpm 10.32.1, JJ 0.45.1, Svelte 5.57.1, Vitest 4.1.11, Storybook 10.5.7. Released Design Core ^0.2.0 and existing owned Community presentation; no upstream edits or new source copies.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

UI projection and installation-owned in-memory reply draft recovery only. No API/schema, execution journal, canonical AgentStore/ThreadStore/ReactionStore identity, publication, companion/ref-CAS, request receipt, process or persisted data format changes.
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M23 remains Done under its existing gates. This maintenance slice
  qualifies resume-replay projection and draft recovery with cold focused
  fixtures; it does not requalify installed adapters, native code reducer 3,
  release/platform or visual gates. Historical E-EXEC-008 and other evidence
  retain their original outcomes.

```
<!-- /evidence-field -->

<!-- evidence-field:id markdown -->
## Id

E-EXEC-012
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-015
- JST-COLLAB-031
- JST-COLLAB-033
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/agent-execution.test.ts#resume history is not a new message or
  feedback target
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Fix the disabled Save reply on a synthetic assistant message produced by ACP session/load replay. Only the exact activity request open-&lt;controllerSessionId&gt; is excluded. Original messages, genuine turn streaming, first observation times and raw journal evidence stay intact. Existing orphan reply drafts are visible and require explicit selection of a published original message; another draft or unresolved receipt cannot be overwritten/moved. Saving remains the existing exact collaboration action, without execution or resume.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Current-tab automation inspection remains unavailable; the tab was not reloaded or its real draft submitted. User selects the original message in Retained reply draft before explicit Save reply. Visual baselines remain separately gated.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: Updated owning UI and collaboration topics with exact session-load
  replay suppression and explicit original-message draft recovery. Persistence,
  feedback target validation and execution dispatch remain unchanged, qualified
  separately in E-EXEC-013.

```
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-EXEC-012 details 2](details-02.md)
