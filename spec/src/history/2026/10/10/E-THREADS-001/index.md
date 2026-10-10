---
schema: 1
id: E-THREADS-001
---

# E-THREADS-001

<!-- evidence-field:id markdown -->
## Id

E-THREADS-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M24
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-COLLAB-034
- JST-COLLAB-035
- JST-COLLAB-036
- JST-API-001
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

- crates/jj-collab/src/commands.rs
- crates/jj-collab/src/graph.rs
- crates/jj-collab/src/lib.rs
- crates/jj-collab/src/thread_codebase.rs
- crates/jj-collab/src/threads.rs
- crates/jj-collab/tests/target_resolution.rs
- crates/jj-collab/tests/thread_codebase.rs
- crates/jj-store-cli/tests/agent_execution/comment_answers.rs
- crates/jj-store-cli/tests/agent_execution/context_delivery.rs
- crates/jj-store-cli/tests/agent_execution/conversations.rs
- crates/jj-store-cli/tests/agent_execution/metadata.rs
- crates/jj-store-cli/tests/agent_execution/real.rs
- crates/jj-store-cli/tests/agent_execution/thread_workspace.rs
- crates/jj-store-cli/tests/agent_execution.rs
- crates/jj-store-http/src/agent_execution.rs
- crates/jj-store-http/src/collab/commands.rs
- crates/jj-store-http/src/local_browser/collaboration.rs
- crates/jj-store-server/src/agent_execution/commands.rs
- crates/jj-store-server/src/agent_execution/comment_submissions.rs
- crates/jj-store-server/src/agent_execution/conversation_links.rs
- crates/jj-store-server/src/agent_execution/conversation_origin.rs
- crates/jj-store-server/src/agent_execution/conversations.rs
- crates/jj-store-server/src/agent_execution/journal.rs
- crates/jj-store-server/src/agent_execution/metadata_store.rs
- crates/jj-store-server/src/agent_execution/mod.rs
- crates/jj-store-server/src/agent_execution/thread_workspace.rs
- crates/jj-store-server/src/agent_execution/workspace_turns.rs
- crates/jj-store-service/src/agent_execution/conversations.rs
- crates/jj-store-service/src/agent_execution/thread_workspace.rs
- crates/jj-store-service/src/agent_execution.rs
- crates/jj-store-service/src/collab_reads.rs
- crates/jj-store-service/src/content_search/index.rs
- crates/jj-store-service/src/local_repository/browser_collab.rs
- schemas/openapi.json
- spec/src/current/contracts/api/44-thread-workspaces.md
- spec/src/current/contracts/collaboration/43-intentional-threads-and-codebases.md
- spec/src/current/contracts/compatibility/37-intentional-thread-formats.md
- web/src/lib/api/generated.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-collab/tests/thread_codebase.rs#annotation_owner_remains_internal_after_message_and_reload
- crates/jj-collab/tests/thread_codebase.rs#legacy_thread_versions_omit_new_fields_and_remain_user_threads
- crates/jj-store-cli/tests/agent_execution/conversations.rs#repository_free_chat_reuses_isolated_bindings_and_never_replays_after_restart
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
cargo test -p jj-collab --locked
```

**Notes**

Initial 50 model tests pass. A new graph transition initially failed because Codebase changes lacked an allowed transition; fixed with exact identity/history validation. Final annotation-owner regression and all-package rerun are recorded in E-THREADS-002.
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
cargo check -p jj-store-server -p jj-store-http --locked
```

**Notes**

Affected Rust packages compile.
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
cargo test -p jj-store-cli --test agent_execution thread_workspace_tests --locked
```

**Notes**

2 pass, 214.68s. Covers explicit zero/multiple Codebase creation, stable duplicate receipts and conflicting intent, stale edits, forged scopes, independent authorization, running-work refusal, removal and fresh subsequent participation. Log /tmp/intentional-threads-rust.log.
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
cargo test -p jj-store-cli --test agent_execution conversations::repository_free_chat_reuses_isolated_bindings_and_never_replays_after_restart --locked
```

**Notes**

Exact existing repository-free/restart regression plus internal-owner directory assertion: 1 pass, 184.03s. The log retains the actual test name; see /tmp/intentional-conversation-regression.log.
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
cargo test -p jj-store-server --lib legacy_codebase_removal_uses_the_same_home_fallback_as_send --locked
```

**Notes**

1 pass. An absent legacy Codebase has the same home-repository fallback in removal admission and Send; explicit empty membership remains empty.
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
pnpm contracts:check
```

**Notes**

Generated schema/type check and actual HTTP response capture pass; 8 contract tests pass. See /tmp/intentional-threads-contracts.log. Final CreateAnnotation regeneration/recheck is recorded in E-THREADS-002.
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
cargo test -p jj-collab --locked
pnpm contracts:generate
pnpm contracts:check
```

**Notes**

Final model package 51 pass. Regenerated CreateAnnotation schema/types and actual HTTP capture: 300 responses conform, 8 contract tests pass. Independent provider/platform/visual and ResizeObserver gates remain outside this model/service target.
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

JJ 0.45.1; cargo 1.98.1; Node 26.9.0; pnpm 10.32.1. macOS ARM64. Existing linker eh_frame size warning remains diagnostic.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

ThreadVersionV1 optional purpose/Codebase fields omitted when absent; native manifest and local companion authority unchanged. Version-1 Thread workspace requests, stable journal/receipts and exact Send witnesses. Existing canonical legacy bytes remain readable.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Intentional user Threads have one metadata home and zero through eight independently authorized repository identities. Internal conversation/annotation owners stay out of ordinary directories/search; exact reads, captures and comments are retained. Per-agent participation and workspace isolation remain distinct from Delta shared worktrees.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Complete shared presentation/cold live qualification in E-THREADS-002; retain independent M24 gates.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-COLLAB-034/035/036 and JST-API-001 define intentional creation,
  internal ownership, independently authorized Codebase, frozen exact Send
  membership and receipt recovery; owning collaboration, API, mutations, CLI and
  compatibility topics updated.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: Enclosing default workspace change potttyzn. M24 remains In progress;
  this is focused model/service evidence, not full platform/provider or visual
  acceptance.

```
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


