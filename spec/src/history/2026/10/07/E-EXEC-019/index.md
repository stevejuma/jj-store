---
schema: 1
id: E-EXEC-019
---

# E-EXEC-019

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

- catalog/components/store-application/AgentChatFeedback.stories.svelte
- catalog/components/store-application/AgentChatLayout.stories.svelte
- catalog/components/store-application/AgentInboxNavigation.stories.svelte
- catalog/components/store-application/AgentSessions.stories.svelte
- catalog/fixtures/AgentExecutionCommandOutput.svelte
- catalog/fixtures/AgentExecutionFixture.svelte
- docs/maintenance/coverage.json
- docs/site/guides/agent-execution.md
- docs/ui-provenance.json
- scripts/agent-inbox-presentation.mjs
- spec/src/current/contracts/ui/34-interactive-agent-sessions.md
- spec/src/current/progress/M23/history/M23/index.md
- spec/src/current/progress/M23/index.md
- spec/src/history/2026/10/07/E-EXEC-018/SUMMARY.nav.md
- spec/src/history/2026/10/07/E-EXEC-018/details-02.md
- spec/src/history/2026/10/07/E-EXEC-018/details-03.md
- spec/src/history/2026/10/07/E-EXEC-018/index.md
- spec/src/history/2026/10/07/E-EXEC-019/SUMMARY.nav.md
- spec/src/history/2026/10/07/E-EXEC-019/index.md
- spec/src/history/2026/10/07/SUMMARY.nav.md
- spec/src/history/2026/10/07/index.md
- web/src/lib/ui/community-agent-activity/activity-feed.css
- web/src/lib/ui/community-agents/CommunityAgents.svelte
- web/src/lib/ui/community-agents/agents.css
- web/src/lib/ui/community-inbox/CommunityInbox.svelte
- web/src/lib/ui/community-inbox/CommunityInboxItem.svelte
- web/src/lib/ui/community-inbox/inbox-layout.ts
- web/src/lib/ui/community-inbox/inbox-resize.svelte.ts
- web/src/lib/ui/community-inbox/inbox.css
- web/src/lib/ui/community-inbox/model.ts
- web/src/lib/ui/store-agent-execution/StoreAgentExecutionHost.svelte
- web/src/lib/ui/store-agent-execution/StoreAgentInboxMessage.svelte
- web/src/lib/ui/store-agent-execution/StoreAgentTranscript.svelte
- web/src/lib/ui/store-agent-execution/presentation.ts
- web/src/lib/ui/store-agent-execution/session-presentation.ts
- web/tests/agent-execution.test.ts
- web/tests/agent-inbox-layout.test.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/agent-execution.test.ts#message times retain first observation
  across chunks and never fabricate legacy times
- web/tests/agent-execution.test.ts#inbox day dividers share thread labels and
  group genuinely unrecorded times
- web/tests/agent-inbox-layout.test.ts#inbox width survives reopening with an
  independent version and panel identity
- web/tests/agent-inbox-layout.test.ts#blocked device storage keeps inbox layout
  usable
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

Enclosing Jujutsu change xpopuyuk; macOS ARM64, Node 24.10.0, pnpm 10.32.1, Vitest 4.1.11. Community source 741b8285e2081f1c9af58afe5a9b72f538fad781, Apache-2.0.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

UI-only; existing event DTO and canonical stores unchanged. New private version-1 device layout key jj-store/layout/v1/agent-inbox/sessions, independent from repository shell/thread layout. No contract regeneration or Rust lane required.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Maintain qualified inbox behavior. Exact historical times cannot be reconstructed for events with no recorded time; pooling, native code publication qualification and visual baseline approval remain separate. Rust and root aggregate check/test/build lanes are out of scope under targeted UI verification policy.
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M23 original Done gates remain unchanged. This maintenance slice
  qualifies session navigation, honest timing presentation and layout
  persistence; it does not expand harness/publication scope or approve visual
  baselines.

```
<!-- /evidence-field -->

<!-- evidence-field:id markdown -->
## Id

E-EXEC-019
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
pnpm web:check
pnpm catalog:check
pnpm check:architecture
pnpm check:ui-ownership
```

**Notes**

Final web/catalog zero errors and warnings; 274 catalog modules, 1268 production/fixture modules and Community ownership pass.
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
node scripts/docs.mjs review application "Reviewed Community inbox resizing with isolated, versioned device persistence, independent session repository links/presence, shared day and turn dividers, readable legacy timing groups and shadcn icon-action tooltips. The read-only current-UI acceptance covers pointer/keyboard/reload/mobile and light/dark accessibility."
pnpm web:build
```

**Notes**

Application documentation ownership reviewed and production web/docs built; nine diagrams and 141-file closure with valid links/anchors. Existing large chunk warning remains.
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
node scripts/agent-inbox-presentation.mjs http://127.0.0.1:7460 442e819f72434178bfa58bd56d2a44bd light
node scripts/agent-inbox-presentation.mjs http://127.0.0.1:7460 442e819f72434178bfa58bd56d2a44bd dark
```

**Notes**

Two fresh isolated browser contexts passed against the current development UI; actual light/dark mode confirmed. Mouse and keyboard resize/reset, reload persistence, mobile hidden handle, repository anchor, avatar presence, date/turn presentation, unclipped first hover toolbar and desktop/mobile axe pass. Zero agent command/launch/setup POSTs and zero page errors. User tab and draft untouched.
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
pnpm check:progress
pnpm spec:validate
pnpm spec:first
node --input-type=module <<'FORMAT'
import {execFileSync} from 'node:child_process';
const files=execFileSync('jj',['--no-pager','diff','--name-only'],{encoding:'utf8'})
  .trim().split('\n').filter(f=>/\.(svelte|ts|mjs|md|json)$/.test(f));
execFileSync('pnpm',['exec','prettier','--check',...files],{stdio:'inherit'});
FORMAT
```

**Notes**

Final corrected documents pass: 144 tasks, 642 evidence records; seven validators, 3114 chapters, 178 requirements; 26 protected files and one canonical chapter; all scoped files formatted. M23 history contains contiguous E-EXEC-018/E-EXEC-019 table rows. Historical timestamp test name retained; unrelated evidence/navigation restored. Product checks were already complete and no production edits followed them.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-ARCH-001
- JST-ARCH-004
- JST-GOV-002
- JST-GOV-008
- JST-DOC-002
- JST-API-001
- JST-COLLAB-031
- JST-COLLAB-033
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Unchanged authority and source ownership confirmation for the agent inbox UI refinement. Runtime reads, original event identities, ordinary reply/command recovery and published contracts remain unchanged. Browser verification proves presentation actions dispatch no execution POST.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: unchanged
reason: JST-ARCH-001/JST-ARCH-004 and JST-GOV-008 source boundaries remain
  enforced by provenance/ownership and architecture checks. JST-API-001
  session/event contracts and JST-COLLAB-031/JST-COLLAB-033
  execution/publication/receipt semantics unchanged. JST-DOC-002 owning
  documentation reviewed; JST-GOV-002 evidence and M23 review retained. Private
  layout storage contains only widths, never credentials or repository state.

```
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


