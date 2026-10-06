---
schema: 1
id: E-EXEC-008
---

# E-EXEC-008 details 2

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
cargo test --locked -p jj-collab
cargo test --locked -p jj-store-wal --test collab_commands
cargo test --locked -p jj-store-service --test local_collab
cargo test --locked -p jj-store-service --lib prompt_feedback_is_exact_bounded_and_omits_tombstones
cargo test --locked -p jj-store-http --lib feedback_is_read_only_local_admin_and_rejects_unbound_queries
cargo check --locked -p jj-store-server
```

**Notes**

27 domain, 8 native WAL, 11 local provider, 1 exact context and 1 HTTP admission test passed; server check passed. Before implementation, the exact additive discussion test failed on unknown command variant (1 failure). Legacy canonical bytes, baseline retained dependencies, exact message targets, duplicate receipts, reopened companion metadata, forged witness rejection and tombstone suppression are covered.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/production-threads.test.ts web/tests/vite/agent-execution-controller.test.ts web/tests/vite/agent-runtime-preparation.test.ts web/tests/vite/reaction-feedback.test.ts web/tests/vite/reaction-library.test.ts
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentDiscussion.stories.svelte catalog/components/store-application/AgentExecution.stories.svelte catalog/components/store-application/AgentChatFeedback.stories.svelte catalog/components/store-threads/StoreThreads.stories.svelte
```

**Notes**

36 unit tests and 29 production-component stories passed, including interactions and axe. New discussion family: 4 passed. Initial story failures (3) came from ambiguous assistant selection, existing hover delay and an empty-string assertion; selectors now identify the canonical row/action state and verify actual retained content. Exact create/body receipt recovery remounts the recovered composer without losing a changed draft. The raster asset fixture now serves actual PNG bytes/content type; no foundational styles or contrast gate changed.
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
pnpm web:check
pnpm catalog:check
pnpm check:architecture
pnpm check:ui-ownership
pnpm check:boundaries
pnpm contracts:check
```

**Notes**

Web/catalog checks reported zero errors and warnings. Catalog boundary: 272 modules; architecture: 1256 modules; Rust boundaries: 17 packages; Community ownership passed. Generated public contract comparison and 7 contract-tool tests passed. Initial fixture API import violated catalog boundaries; changed to the existing owned controller type export. Counted-line failures were repaired by extracting production launch context, fixture response/status and browser input helpers; limits remain unchanged. The read-only controller test now permits the added feedback GET while preserving no dispatch assertions.
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
pnpm docs:generate
pnpm docs:review publication "Reviewed additive exact AgentMessage discussion validation and retained external dependencies; manifest CAS, WAL receipts and reducer/publication authority unchanged"
pnpm docs:review collaboration "Reviewed exact repository-backed inbox replies/reactions and immutable AgentMessage discussion baseline; canonical legacy bytes and no implicit dispatch preserved"
pnpm docs:review services "Reviewed authorized pinned feedback projection and explicit prompt context capture with exact receipt replay; service and local provider ownership retained"
pnpm docs:review configuration "Reviewed unchanged execution configuration and private installation state; repository feedback creates no new launch or credential setting"
pnpm docs:review http "Reviewed admitted read-only feedback endpoint and exact discussion command; existing actor and route admission maintained"
pnpm docs:review application "Reviewed Community message actions, exact reaction targets, retained reply drafts and explicit next Send context in agent inbox"
pnpm docs:review contracts "Reviewed generated additive feedback DTO and discussion command; existing canonical IDs and namespace retained"
pnpm docs:check
pnpm web:build
```

**Notes**

Documentation and production build passed: 9 diagrams; 141-file closure with links/anchors valid. Wrong singular docs review topic was rejected and corrected to contracts. Build correctly rejected stale generated installation references, then stale HTTP source review after tests changed; regeneration and explicit source review repaired both. Final build passed. Existing Vite CSS pseudo-class, large-bundle and plugin timing advisories remain; no build threshold changed. Documentation builds were serialized.
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
pnpm dev:backend:build
pnpm dev:backend:restart
```

**Notes**

Rebuilt and replaced only the owned development backend, preserving Vite and .local/dev-demo. Known macOS compact-unwind linker warning only. Current user session remained ended/disconnected; no prompt, reply, reaction, resume or end was submitted to real history. Feedback read returned 6 exact published targets and no replies. Temporary reaction-read failures during backend replacement cleared on bounded refresh; subsequent current UI inspection showed shared Reply/Add reaction controls without failures.
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
cargo test --locked -p jj-store-service --test conversion_agent_discussion
```

**Notes**

Exact conversion test passed: stable Thread/session/message/event IDs, remapped event/content witnesses and fail-closed missing mapping. New test initially failed to compile because Agent types were not imported; the service consumes the existing WAL-owned exports, with no new crate dependency.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentDiscussion.stories.svelte catalog/components/store-application/AgentExecution.stories.svelte catalog/components/store-application/AgentChatFeedback.stories.svelte catalog/components/store-threads/StoreThreads.stories.svelte
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentDiscussion.stories.svelte
```

**Notes**

Final affected stories: 30 passed; exact discussion family: 5 passed. Before the repair, the new unavailable-state story failed once because disabled Save also disabled the editor. The composer now receives no submit handler while preserving editable drafts; toolbar/bar mutation actions remain disabled. Prior cold fixture failures confirmed code-repository publication refusal, not a qualified native write path.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/production-threads.test.ts web/tests/vite/agent-execution-controller.test.ts web/tests/vite/agent-runtime-preparation.test.ts web/tests/vite/reaction-feedback.test.ts web/tests/vite/reaction-library.test.ts
```

**Notes**

Final direct controller/reaction tests: 36 passed after availability changes.
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
env PATH=/Users/stevejuma/.npm/_npx/bb64e4387f65cc68/node_modules/.bin:$PATH JJ_STORE_AGENT_PRIVACY_CACHE=/private/tmp/jj-store-agent-fixture-cache-20261006 python3 scripts/agent-execution-acceptance.py --server target/debug/jj-store --jj target/debug/jj --rustfs runtime/embedded-rustfs/target/debug/jj-store-rustfs --acp ../jj-acp/target/debug/jj-acp --web web/dist --browser --browser-only
```

**Notes**

Cold built native/real-RustFS/installed-Codex fixture passed: code-repository writes explicitly refused, then the supported disposable WorkStore home saved user/assistant replies as ordinary exact ThreadStore messages; Like/clear used the exact original target. Saving replies/reactions emitted no prompt. Reload and service restart retained exact reply revisions and history. Next UI Send included both authored replies and exact revision IDs. Later feedback plus exact original prompt retry did not alter or replay the retained prompt. Existing gallery/editor/presence/inbox/history/review/axe, explicit resume-with-no-prompt and foreground shutdown gates passed. This does not qualify native code repository or registered-folder inbox writes. First cold failure read before asynchronous save and matched editor text; receipt and persisted-message assertions were strengthened. Two later attempts exposed the existing publication refusal before the body request. A response-body eviction in the browser harness was repaired by checking the exact durable command status receipt rather than volatile response bytes.
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
pnpm contracts:generate
cargo test --locked -p jj-store-service --test local_collab
cargo test --locked -p jj-store-service --lib prompt_feedback_is_exact_bounded_and_omits_tombstones
cargo test --locked -p jj-store-http --lib feedback_is_read_only_local_admin_and_rejects_unbound_queries
cargo check --locked -p jj-store-server
pnpm web:check
pnpm catalog:check
```

**Notes**

Availability follow-up: regenerated optional read-only DTO; repeated affected Rust provider/context/HTTP/server checks passed (11, 1, 1 tests) and web/catalog checks reported no errors/warnings. Local provider feedback remains available independently of the unimplemented browser transport.
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
pnpm dev:backend:build
env JJ_STORE_ACP_EXECUTABLE=/Users/stevejuma/code/lapismd/jj-acp/target/debug/jj-acp JJ_STORE_AGENT_JJ_EXECUTABLE=/Users/stevejuma/code/lapismd/jj-store/target/debug/jj pnpm dev:backend:restart
```

**Notes**

Backend-only rebuild/restart preserved Vite, fixture, real chat history and drafts. A restart without the original ACP environment exposed 503 unavailable; restored the explicit owned ACP/JJ executables and status returned ready. Current session is registered local Git-backed, not native. Fresh browser reload showed unavailable explanation, aria-disabled=false editor, disabled Save and zero Reaction failed controls. Opened/closed an empty reply panel by keyboard; submitted no real reply/reaction/prompt/resume.
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

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Native code repository inbox writes require M15.4 reducer-3 qualification; the
  production publication refusal is retained.
- The current user chat uses the registered local Git-backed profile, whose
  browser Thread/Reaction transport remains unimplemented; local
  persistence/context provider tests pass.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Enclosing Jujutsu change unponssp. macOS ARM64; JJ 0.45.1, Rust 1.98.1, Node 24.10.0, pnpm 10.32.1, Svelte 5.57.1, Storybook 10.5.7, Vitest 4.1.11. Owned Community presentation revision 741b8285e2081f1c9af58afe5a9b72f538fad781, Apache-2.0; released Design Core ^0.2.0.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Additive create_agent_discussion typed command and optional omitted-by-default ThreadVersion agent_message target; legacy canonical bytes unchanged. Native manifest format 5 and local companion version 2 unchanged. Generated OpenAPI/TypeScript add read-only feedback DTO/endpoint. Installation journal/request version and exact prepared-prompt receipt semantics unchanged; no native identity remapping. Feedback write_unavailable is an optional read-only browser capability explanation, captured from the same authorized selection.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Repository-owned inbox prompt/assistant feedback, ordinary persisted human reply revisions, exact existing reactions and explicit next-Send context. Bounded authorized selection uses existing capture normalization, suppresses tombstones and staged records, reauthorizes before response/dispatch and fails closed on unavailable reads or budgets. UI composes owned Community messages/actions/reply composer and existing production Thread/Reaction controllers, retaining drafts and original pending identities. Local companion persistence/context is provider-qualified; local browser Thread transport is not claimed. No test writes were made to the current real user chat. Targeted cross-layer verification covers domain, native/local providers, HTTP/generated contracts, directly affected UI stories/controllers and built cold lifecycle; root aggregate suites and unrelated browser/platform lanes are out of scope. Cold testing exposed the existing native code-repository collaboration_replication_unsupported guard before reply body publication. The UI now explains write_unavailable and disables Save/reaction actions while retaining drafts and readable history. Positive inbox write acceptance is restricted to an explicit disposable WorkStore home; no publication guard or reducer qualification is weakened.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Implement and qualify registered-folder inbox Thread/Reaction transport for the current chat, then native code-repository reducer 3. Maintain scoped WorkStore/provider regression proof; broader M13/M15/release and visual baseline gates remain separate.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: "Owning collaboration/16 and /36, api/41, ui/34 and local-jj/07 describe
  additive exact message discussions and explicit prompt feedback context.
  Unchanged protected boundaries: JST-GOV-002/008 owned source and provenance;
  JST-ARCH-001/004 independent layers and source budgets; JST-DOC-002
  source-reviewed generated manuals; JST-MUT-001/JST-CLI-004 original
  receipt/authorization and explicit development paths; JST-COMPAT-001 unchanged
  JJ codecs/version. Publication remains manifest/ref CAS with original durable
  receipts, staged-object invisibility and no replay execution. No storage
  publication format, upstream codec, executable/credential admission or
  foundational component API changed. Optional write availability is captured
  without changing the native code-repository refusal or qualifying
  registered-folder inbox transport."

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M23 remains Done for its previously qualified local macOS execution
  scope. E-EXEC-008 verifies additive discussion/context and retained
  unavailable UI within supported native WorkStore and local provider
  boundaries; native code repository writes still require M15.4,
  registered-folder inbox transport is not qualified, and no broader M13/M15,
  release, extra-runner/platform or visual baseline completion is claimed.

```
<!-- /evidence-field -->

