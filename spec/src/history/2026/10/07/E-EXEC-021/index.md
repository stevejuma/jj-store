---
schema: 1
id: E-EXEC-021
---

# E-EXEC-021

<!-- evidence-field:date markdown -->
## Date

2026-10-07
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M23
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-application/AgentInboxNavigation.stories.svelte
- catalog/components/store-application/AgentPromptMarkup.stories.svelte
- catalog/components/store-application/ManagedAgentIdentity.stories.svelte
- catalog/fixtures/AgentExecutionFixture.svelte
- catalog/fixtures/ManagedAgentRepositoryFixture.svelte
- catalog/fixtures/agent-execution-session.ts
- crates/jj-store-server/src/agent_execution/commands.rs
- crates/jj-store-server/src/agent_execution/feedback.rs
- crates/jj-store-server/src/agent_execution/journal.rs
- crates/jj-store-server/src/agent_execution/launch.rs
- crates/jj-store-service/src/agent_execution.rs
- docs/maintenance/coverage.json
- docs/site/guides/agent-execution.md
- docs/site/reference/api/types.md
- docs/site/reference/configuration.md
- docs/site/reference/transport.md
- docs/site/technical/architecture.md
- schemas/openapi.json
- scripts/agent-inbox-presentation.mjs
- scripts/managed-agent-identity.mjs
- spec/src/current/contracts/api/41-interactive-agent-execution.md
- spec/src/current/contracts/cli/35-interactive-agent-execution.md
- spec/src/current/contracts/collaboration/36-interactive-agent-execution.md
- spec/src/current/contracts/compatibility/13-interactive-agent-execution.md
- spec/src/current/contracts/mutations/22-interactive-agent-execution.md
- spec/src/current/contracts/ui/34-interactive-agent-sessions.md
- spec/src/current/progress/M23/history/M23/index.md
- spec/src/current/progress/M23/index.md
- spec/src/history/2026/10/07/E-EXEC-020/SUMMARY.nav.md
- spec/src/history/2026/10/07/E-EXEC-020/details-02.md
- spec/src/history/2026/10/07/E-EXEC-020/details-03.md
- spec/src/history/2026/10/07/E-EXEC-020/index.md
- spec/src/history/2026/10/07/E-EXEC-021/SUMMARY.nav.md
- spec/src/history/2026/10/07/E-EXEC-021/index.md
- spec/src/history/2026/10/07/SUMMARY.nav.md
- spec/src/history/2026/10/07/index.md
- web/src/lib/api/generated.ts
- web/src/lib/collaboration/managed-agent.ts
- web/src/lib/collaboration/presentation.ts
- web/src/lib/ui/store-agent-execution/StoreAgentExecutionHost.svelte
- web/src/lib/ui/store-agent-execution/session-presentation.ts
- web/src/lib/ui/store-application/StoreApplication.svelte
- web/src/lib/ui/store-application/StoreWorkspacePicker.svelte
- web/src/lib/ui/store-collaboration/StoreAgentContext.svelte
- web/src/lib/ui/store-collaboration/StoreAgentSessionRow.svelte
- web/src/lib/ui/store-collaboration/StoreAgentSessions.svelte
- web/src/lib/ui/store-collaboration/StoreAgentSidebar.svelte
- web/tests/collaboration-presentation.test.ts
- web/tests/managed-agent.test.ts
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
pnpm check:ui-ownership
pnpm check:architecture
pnpm web:check
pnpm catalog:check
```

**Notes**

Community ownership passes; architecture covers 1272 modules; web/catalog have zero errors/warnings and catalog covers 276 modules. No external Community imports or package substitutions added.
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
node scripts/managed-agent-identity.mjs http://127.0.0.1:7460 442e819f72434178bfa58bd56d2a44bd light
node scripts/managed-agent-identity.mjs http://127.0.0.1:7460 442e819f72434178bfa58bd56d2a44bd dark
```

**Notes**

Two cold isolated browser contexts after owned backend replacement pass exact capture aliases/avatar, repository tooltip and workspace operation navigation, Back/Forward/reload and desktop/mobile axe. No agent command/launch/setup POST, page error, user-tab edit, prompt or repository mutation.
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
pnpm contracts:check
```

**Notes**

Generated contracts match, 300 actual HTTP/SSE responses conform and seven contract tests plus contract TypeScript check pass. Local read fields are additive; canonical identities/publication/receipts remain unchanged.
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
pnpm check:architecture
pnpm check:ui-ownership
```

**Notes**

Latest architecture result covers 1273 modules; UI source ownership passes. Captured identity remains presentation from admitted installation metadata and exact normalizer aliases.
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
pnpm check:progress
pnpm spec:validate
pnpm spec:first
pnpm docs:check
```

**Notes**

Mapped unchanged authority chapters and M23 maintenance evidence pass progress, all seven document validators, spec-first and documentation closure. Public generated contracts and cold browser acceptance are recorded above; visual baselines remain separately gated.
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

Enclosing Jujutsu change lowkpvrl; macOS ARM64, Node 24.10.0, pnpm 10.32.1, Vitest 4.1.11. Existing owned Community/Design Core components reused.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Additive version-1 LocalTrusted execution read fields: recorded_session_ids, workspace_name and workspace_operation. Old private journals default these fields; read clears stored projections and admitted status recomputes them. Canonical AgentStore/ThreadStore/ReactionStore bytes and IDs unchanged.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Maintain exact admitted identity and workspace presentation. Visual baselines and native code-publication qualification remain separately gated. Root aggregate Rust/web suites are out of scope under targeted policy.
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M23 retains its original Done qualification. This maintenance slice
  joins admitted installation identities to exact recorded sessions and
  workspace proofs; it does not qualify another harness, native code publication
  or visual baselines.

```
<!-- /evidence-field -->

<!-- evidence-field:id markdown -->
## Id

E-EXEC-021
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-ARCH-001
- JST-ARCH-004
- JST-GOV-002
- JST-GOV-008
- JST-DOC-002
- JST-COLLAB-032
- JST-CLI-004
- JST-COMPAT-001
- JST-COMPAT-003
- JST-COMPAT-005
- JST-MUT-001
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Unchanged canonical storage/publication/receipt authority and source-ownership confirmation for managed agent identity presentation. Local read DTO additions are separately governed by E-EXEC-020. Captured installation identity does not become canonical repository agent metadata.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: unchanged
reason: JST-ARCH-001/JST-ARCH-004 retain owned service interfaces and normalizer
  authority. JST-GOV-008 reuses existing maintained Community
  avatar/time/Command View presentation and public Design Core primitives
  without external source imports. JST-COLLAB-032 native evidence,
  thread/reaction targets and publication receipts remain intact. JST-DOC-002
  owning pages/reviews and JST-GOV-002 evidence/M23 maintenance review updated.

```
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


