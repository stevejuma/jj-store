---
schema: 1
id: E-EXEC-009
---

# E-EXEC-009

<!-- evidence-field:id markdown -->
## Id

E-EXEC-009
<!-- /evidence-field -->

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
- JST-PUB-001
- JST-PUB-002
- JST-PUB-004
- JST-PUB-005
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-application/AgentDiscussion.stories.svelte
- catalog/fixtures/AgentExecutionCommandOutput.svelte
- catalog/fixtures/AgentExecutionFixture.svelte
- catalog/fixtures/agent-discussion-fixture.ts
- catalog/fixtures/agent-execution-session.ts
- catalog/fixtures/agent-execution-status.ts
- catalog/fixtures/thread-transport.ts
- crates/jj-collab/src/commands.rs
- crates/jj-collab/src/objects.rs
- crates/jj-collab/src/threads.rs
- crates/jj-collab/tests/agent_discussion.rs
- crates/jj-collab/tests/target_resolution.rs
- crates/jj-store-http/src/agent_execution/tests.rs
- crates/jj-store-http/src/agent_execution.rs
- crates/jj-store-server/src/agent_execution/commands.rs
- crates/jj-store-server/src/agent_execution/feedback.rs
- crates/jj-store-server/src/agent_execution/mod.rs
- crates/jj-store-server/src/agent_execution/publication.rs
- crates/jj-store-service/src/agent_execution.rs
- crates/jj-store-service/src/agent_feedback/tests.rs
- crates/jj-store-service/src/agent_feedback.rs
- crates/jj-store-service/src/collab_reads/agent_feedback.rs
- crates/jj-store-service/src/collab_reads.rs
- crates/jj-store-service/src/conversion/collab.rs
- crates/jj-store-service/src/lib.rs
- crates/jj-store-service/src/local_repository/agent_feedback.rs
- crates/jj-store-service/src/local_repository/collab_commands.rs
- crates/jj-store-service/src/local_repository.rs
- crates/jj-store-service/tests/conversion_agent_discussion.rs
- crates/jj-store-service/tests/local_collab/agent_discussion.rs
- crates/jj-store-service/tests/local_collab.rs
- crates/jj-store-wal/src/publication/collab_commands.rs
- docs/agent-execution.md
- docs/maintenance/coverage.json
- docs/site/concepts/collaboration.md
- docs/site/guides/agent-execution.md
- docs/site/reference/api/collaboration.md
- docs/site/reference/api/installation.md
- docs/site/reference/api/types.md
- docs/ui-provenance.json
- schemas/openapi.json
- scripts/agent-execution-acceptance.py
- scripts/agent-execution-browser-input.mjs
- scripts/agent-execution-browser.mjs
- scripts/agent-execution-discussion.mjs
- spec/src/current/contracts/api/41-interactive-agent-execution.md
- spec/src/current/contracts/collaboration/16-persistent-threads-and-reactions-m15.md
- spec/src/current/contracts/collaboration/36-interactive-agent-execution.md
- spec/src/current/contracts/local-jj/07-m15-additive-collaboration-publication.md
- spec/src/current/contracts/ui/34-interactive-agent-sessions.md
- spec/src/current/progress/M23/history/M23/index.md
- spec/src/current/progress/M23/index.md
- spec/src/history/2026/10/06/E-EXEC-005/index.md
- spec/src/history/2026/10/06/E-EXEC-006/index.md
- spec/src/history/2026/10/06/E-EXEC-007/index.md
- spec/src/history/2026/10/06/E-EXEC-008/SUMMARY.nav.md
- spec/src/history/2026/10/06/E-EXEC-008/details-02.md
- spec/src/history/2026/10/06/E-EXEC-008/index.md
- spec/src/history/2026/10/06/E-EXEC-009/SUMMARY.nav.md
- spec/src/history/2026/10/06/E-EXEC-009/index.md
- spec/src/history/2026/10/06/SUMMARY.nav.md
- spec/src/history/2026/10/06/index.md
- spec/src/previous-locations.md
- web/src/lib/api/generated.ts
- web/src/lib/controller/agent-discussions.svelte.ts
- web/src/lib/controller/agent-execution-launch-context.ts
- web/src/lib/controller/agent-execution.svelte.ts
- web/src/lib/controller/production-local-configuration.ts
- web/src/lib/controller/production-threads.svelte.ts
- web/src/lib/controller/thread-post.svelte.ts
- web/src/lib/ui/store-agent-execution/StoreAgentInboxMessage.svelte
- web/src/lib/ui/store-agent-execution/StoreAgentTranscript.svelte
- web/tests/vite/agent-execution-controller.test.ts
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

1256 browser modules and 17 Rust packages passed boundaries; owned Community source revision/Apache-2.0 provenance passed. No foundational dependency, source budget, external checkout or transport ownership exception introduced.
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
pnpm docs:generate
pnpm docs:review configuration "Reviewed feedback availability and pinned read metadata; installation settings, launch configuration and credential handling remain unchanged"
pnpm docs:review application "Reviewed unavailable feedback projection retaining editable drafts and readable replies without unsupported reaction queries; exact shared Community messages/actions remain unchanged"
pnpm web:build
```

**Notes**

Generated manual references and actual source-topic reviews were refreshed. Final build/document closure passed with 9 offline diagrams and 141 valid files. Initial final docs check refused stale configuration review after the availability DTO change; exact review fixed it. Vite CSS/bundle and macOS linker advisories are retained, with no threshold or component override. All additional topic reviews and exact commands are recorded in E-EXEC-008 and the same-change logs.
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
cargo test --locked -p jj-store-wal --test collab_commands
cargo test --locked -p jj-store-service --test local_collab
cargo test --locked -p jj-store-service --test conversion_agent_discussion
```

**Notes**

8 native WAL and 11 local companion tests passed, plus 1 exact witness conversion test. Publication guard, source membership, original receipt replay, missing mapping refusal and legacy canonical byte behavior remain intact. E-EXEC-008 records the broader additive feature and its explicit unqualified scopes.
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
pnpm spec:validate
pnpm check:progress
pnpm spec:first --base ce6237cc --head @
pnpm format:rust:check
```

**Notes**

Final specification, progress, protected-change and Rust formatting gates passed: 7 validators, 3081 chapters, 178 requirements; 144 tasks/632 evidence records; 55 protected files and 5 owning canonical chapters. Initial progress check rejected bare test names and missing new ledger file coverage; exact file#test references and fresh paths repaired them. progress:history also removed whitespace in unrelated M9/M11/M13/M18 pages; restored those generated changes from the parent and retained prior M23 evidence descriptions, appending only the new scoped records. Changed-file Prettier check found generated navigation formatting; formatted the changed files. No root aggregate suite was run. Full inbox write availability remains explicitly pending.
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

Enclosing Jujutsu change unponssp. macOS ARM64; JJ 0.45.1, Rust 1.98.1, Node 24.10.0, pnpm 10.32.1, Svelte 5.57.1, Storybook 10.5.7, Vitest 4.1.11. Owned Community presentation revision 741b8285e2081f1c9af58afe5a9b72f538fad781, Apache-2.0; released Design Core ^0.2.0.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No native identity codec, manifest/ref-CAS publication authority, existing WAL/receipt/companion envelope, upstream JJ version, launch/environment/credential admission, package resolution or foundational component API changed. Additive discussion/feedback contracts and source copies are governed in E-EXEC-008 and updated canonical topics.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Completed unchanged-contract maintenance confirmation for protected publication, service, CLI, compatibility, governance, architecture and documentation paths in E-EXEC-008. Exact target validation/dependency closure uses existing owned resolver and conversion mapping. Native code-repository reducer-3 refusal remains enforced; fixture WorkStore initialization is explicit and does not qualify code writes. Test/tool and provenance changes preserve budget, source ownership, explicit development executable and cold evidence boundaries. The user-requested full inbox write scope remains incomplete as recorded by pending E-EXEC-008; this maintenance pass is not a feature-completion claim.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Retain the native code publication guard and qualify the registered-folder inbox transport and M15.4 separately; visual baseline approval and broader release/platform gates remain separate.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: unchanged
reason: JST-PUB-001/002/004/005 preserve manifest CAS, selected-only reads,
  exact original receipts and unknown outcomes. JST-MUT-001 retains provider
  authorization and semantic command revalidation. JST-CLI-004 keeps explicit
  development binaries, no installation/reset/unrelated restart. JST-GOV-002/008
  and JST-ARCH-001/004 preserve owned Community source/provenance, independent
  layers and budgets. JST-DOC-002 retains source-reviewed generated manuals.
  JST-COMPAT-001 retains upstream JJ 0.45.1 codecs and golden identities.
  Changed additive collaboration/UI/API/local behavior is documented separately;
  no existing publication/authority/compatibility rule is weakened.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M23 stays Done only for its previously qualified execution profile. This
  record confirms completed maintenance and retained guards. E-EXEC-008 remains
  pending for code-repository replication and registered-folder inbox transport;
  no M13/M15 or wider release/platform/visual completion is claimed.

```
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


