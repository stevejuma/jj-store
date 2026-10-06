---
schema: 1
id: E-EXEC-007
---

# E-EXEC-007

<!-- evidence-field:date markdown -->
## Date

2026-10-06
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
- catalog/components/store-application/AgentChatRuntime.stories.svelte
- catalog/fixtures/AgentExecutionFixture.svelte
- catalog/fixtures/agent-execution-events.ts
- catalog/fixtures/agent-execution-session.ts
- crates/jj-store-cli/tests/agent_execution/setups.rs
- crates/jj-store-server/src/agent_execution/journal.rs
- crates/jj-store-server/src/agent_execution/mod.rs
- crates/jj-store-server/src/agent_execution/publication.rs
- crates/jj-store-service/src/agent_execution.rs
- docs/agent-execution.md
- docs/maintenance/coverage.json
- docs/site/guides/agent-execution.md
- docs/site/reference/api/types.md
- docs/site/technical/architecture.md
- docs/site/technical/lifecycle.md
- docs/ui-provenance.json
- schemas/openapi.json
- scripts/agent-execution-acceptance.py
- scripts/agent-execution-browser.mjs
- scripts/agent-execution-chat-feedback.mjs
- spec/src/current/contracts/api/41-interactive-agent-execution.md
- spec/src/current/contracts/collaboration/36-interactive-agent-execution.md
- spec/src/current/contracts/ui/34-interactive-agent-sessions.md
- spec/src/current/progress/M23/history/M23/index.md
- spec/src/current/progress/M23/index.md
- web/src/lib/api/generated.ts
- web/src/lib/ui/community-agent-activity/CommunityAgentTranscriptMessage.svelte
- web/src/lib/ui/community-agent-composer-activity/CommunityAgentComposerActivity.svelte
- web/src/lib/ui/store-agent-execution/StoreAgentConversation.svelte
- web/src/lib/ui/store-agent-execution/StoreAgentExecutionHost.svelte
- web/src/lib/ui/store-agent-execution/StoreAgentRequestRecovery.svelte
- web/src/lib/ui/store-agent-execution/StoreAgentTranscript.svelte
- web/src/lib/ui/store-agent-execution/presentation.ts
- web/tests/agent-execution.test.ts
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Enclosing Jujutsu change qukluvnk. macOS ARM64; JJ 0.45.1, Rust 1.98.1, Node 24.10.0, pnpm 10.32.1, Svelte 5.57.1, Storybook 10.5.7, Vitest 4.1.11. Owned Community presentation pinned to 741b8285e2081f1c9af58afe5a9b72f538fad781, Apache-2.0. Released Design Core ^0.2.0.
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

[]
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M23 remains Done for the previously qualified local macOS profile. This
  maintenance slice preserves the cold lifecycle gates and adds truthful chat
  timestamps and Community feedback; no wider release, additional
  runner/platform or visual baseline qualification is claimed.

```
<!-- /evidence-field -->

<!-- evidence-field:id markdown -->
## Id

E-EXEC-007
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-GOV-002
- JST-GOV-008
- JST-ARCH-001
- JST-ARCH-004
- JST-DOC-002
- JST-MUT-001
- JST-CLI-004
- JST-COMPAT-001
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
pnpm check:architecture
pnpm check:boundaries
pnpm check:ui-ownership
```

**Notes**

Architecture passed at 1246 modules, Rust package boundaries at 17 packages, and complete owned Community provenance passed. Initial fixture exceeded 300 counted lines by 2; response construction moved to its existing session helper. Initial browser flow exceeded 300 by 8; dedicated chat feedback/Working verification moved to its owned helper. Budgets and diagnostics are unchanged.
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
pnpm evidence:new -- --id E-EXEC-006 --task M23 --requirement JST-UI-015
pnpm evidence:new -- --id E-EXEC-007 --task M23 --requirement JST-GOV-002
pnpm docs:generate
pnpm docs:review application "Agent inbox shared timestamp metadata, retained observation times and capture propagation, Community system/Working messages and compact exact-request recovery strip"
pnpm docs:review services "Execution journal records device observation time, generated event DTO and captured observations retain it; reviewed restart, publication authority and legacy unknown timing in lifecycle and architecture pages"
pnpm docs:review collaboration "Reviewed timestamp propagation through existing AgentStore capture metadata; native identity, immutable publication, sanitization and receipt authority unchanged"
pnpm docs:review configuration "Reviewed unchanged configuration/environment controls and updated lifecycle journal persistence; event observation metadata adds no setting, launch flag or credential surface"
pnpm docs:review contracts "Regenerated event DTO and type references for optional retained observation time; HTTP version, admission, CLI commands and transports remain unchanged"
pnpm docs:check
pnpm web:build
```

**Notes**

Generation, reviewed source topics, serial document checks and production build passed. Earlier builds correctly rejected stale generated type references, then services/configuration reviews. Parallel docs/site builds caused a transient file-count refusal; serial docs:check passed with 141 files and valid links/anchors. Vite emitted bundle-size/plugin timing advisories; no threshold was changed.
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
pnpm spec:validate
pnpm check:progress
pnpm format:rust:check
```

**Notes**

Specification validation, complete evidence bookkeeping and Rust formatting passed. Preliminary progress/spec-first rejected unfilled evidence, then unsupported named Story references and invalid pending run metadata. Named tests now use exact Rust/unit regressions and only completed validation is recorded. Completed evidence passed spec-first: 30 protected files and 3 canonical chapters. M23 status remains unchanged.
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
pnpm spec:first
pnpm spec:validate
pnpm check:progress
```

**Notes**

Final gate passed: 30 protected files and 3 canonical chapters; 7 validators, 3063 chapters, 178 requirements; bookkeeping covers 144 tasks and 627 evidence records. No full milestone/release claim is inferred. A changed-file Prettier check found generated previous-locations navigation spacing; formatted it and reran the same changed-file check.
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
python3 - <<'PY'
import subprocess
paths=subprocess.check_output(['jj','--no-pager','diff','--name-only'],text=True).splitlines()
paths=[p for p in paths if p.endswith(('.svelte','.ts','.mjs','.md','.json'))]
with open('.local/agent-chat-format-final.log','w') as log:
 result=subprocess.run(['pnpm','exec','prettier','--check',*paths],stdout=log,stderr=log)
raise SystemExit(result.returncode)
PY
```

**Notes**

All changed authored/generated text formatting passed after formatting the coverage/helper and generated navigation differences.
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
pnpm spec:search -- 'JST-UI-015'
pnpm spec:search 'JST-UI-015'
```

**Notes**

Initial forwarded -- was rejected by the released search CLI; corrected invocation returned the owning requirement, which was read before implementation.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pass
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Owned source copies and Apache-2.0 provenance extended; generated OpenAPI/TypeScript/manual references updated. No dependency, foundational component API, native identity/storage authority, CLI command, credentials or receipt/publication format changes.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Unchanged-contract confirmation for protected maintenance and service/CLI/test harness paths in E-EXEC-006. JST-GOV-002/008 still require owned Community copies and fresh same-change evidence. JST-ARCH-001/004 still enforce independent native layers, production/catalog independence and source budgets. JST-DOC-002 still requires source-reviewed manuals and generated references. JST-MUT-001, JST-CLI-004 and JST-COMPAT-001 retain native publication authority, explicit development executables and compatibility/license boundaries. The changed execution/UI/API behavior is governed separately by E-EXEC-006 and its owning chapters. Verification uses the repository targeted ladder: exact/direct UI stories and affected Rust/public-contract checks, plus built cold lifecycle acceptance; no root aggregate suites, full workspace Rust tests or broad unrelated browser lanes were run.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Maintain captured observation times and exact request/draft semantics; visual baselines and broader release/platform qualification remain separately gated.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: unchanged
reason: Native manifest CAS, source identities, sanitized capture authority,
  durable spool receipts, publication flags and explicit request semantics are
  untouched. Observation time uses existing captured metadata. No CLI flags,
  launch executable/environment admission, package-resolution, component
  foundation or source budget change. Documentation and private test helpers
  were updated within their owning boundaries.

```
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


