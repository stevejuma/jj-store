---
schema: 1
id: E-CHAT-LIVE-004
---

# E-CHAT-LIVE-004

<!-- evidence-field:id markdown -->
## Id

E-CHAT-LIVE-004
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
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-09
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Recorded conversation feedback and immutable submitted history
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change noyvypvs
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Recorded detail lacked exact conversation feedback; edited submitted comments and comment-only reply parents were not retained by all read projections.
<!-- /evidence-field -->

<!-- evidence-field:findings markdown -->
## Findings

Native and local recorded detail now expose authorized bounded feedback and exact tool targets. Sent context carries the canonical publication batch identity. Submitted old revisions remain visible with exact answers while the current comment is visible; tombstones hide every revision and historical revisions do not enter new prompt context. Comments-only batches retain exact reply parents. Legacy whole-message discussions remain compatible.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-collab/src/conversation_submission.rs
- crates/jj-collab/src/graph.rs
- crates/jj-collab/src/objects.rs
- crates/jj-collab/src/threads.rs
- crates/jj-collab/tests/conversation_submission.rs
- crates/jj-store-cli/tests/agent_execution/comment_answers/history.rs
- crates/jj-store-cli/tests/agent_execution/comment_answers.rs
- crates/jj-store-server/src/agent_execution/prompt_context.rs
- crates/jj-store-service/src/agent_execution/context.rs
- crates/jj-store-service/src/agent_execution/context_submission.rs
- crates/jj-store-service/src/agent_execution.rs
- crates/jj-store-service/src/agent_feedback/tests.rs
- crates/jj-store-service/src/agent_feedback.rs
- crates/jj-store-service/src/collab_reads/agent_feedback.rs
- crates/jj-store-service/src/collaboration/agent_details.rs
- crates/jj-store-service/src/collaboration/models.rs
- crates/jj-store-service/src/conversion/collab.rs
- crates/jj-store-service/src/local_repository/browser_agents/models.rs
- crates/jj-store-service/src/local_repository/browser_agents.rs
- crates/jj-store-service/tests/agent_sessions.rs
- schemas/openapi.json
- spec/src/current/contracts/collaboration/41-conversation-comment-submissions.md
- spec/src/current/progress/M24/index.md
- spec/src/history/2026/10/09/E-CHAT-LIVE-004/SUMMARY.nav.md
- spec/src/history/2026/10/09/E-CHAT-LIVE-004/details-02.md
- spec/src/history/2026/10/09/E-CHAT-LIVE-004/index.md
- spec/src/history/2026/10/09/SUMMARY.nav.md
- spec/src/history/2026/10/09/index.md
- web/src/lib/api/generated.ts
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
cargo test -p jj-store-service --test agent_sessions session_detail_resolves_bounded_messages_and_final_tool_snapshots --locked
```

**Notes**

1 passed: native recorded feedback and tool targets.
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
cargo test -p jj-store-cli --locked --test agent_execution comment_answers::
```

**Notes**

2 passed after adding edited-history checks (123.94s); 2 passed after adding cold-reopen tombstone checks (126.90s). Controlled local managed and standalone ACP harnesses, not real-provider or browser acceptance.
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
cargo test -p jj-store-cli --locked --test agent_execution comment_answers:: (initial)
```

**Notes**

History helper initially passed Option&lt;GitOid&gt; rather than its required unwrapped fixture value; corrected.
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
cargo test -p jj-collab --test conversation_submission --locked (initial)
```

**Notes**

New compatibility test initially used the wrong jj_agent_store module path; corrected to jj_agent.
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
cargo test -p jj-collab --test conversation_submission --locked
```

**Notes**

6 passed.
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
cargo test -p jj-store-service --locked --lib agent_feedback::tests
```

**Notes**

1 passed: historical revisions excluded from incremental context.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
cargo check -p jj-store-server --locked
```
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm contracts:generate
```
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
cmp .local/chat-recorded-generated-before.sha .local/chat-recorded-generated-after.sha
```

**Notes**

Generated schema and TypeScript hashes unchanged on regeneration.
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
pnpm test:contracts
```

**Notes**

7 schema tests and contract TypeScript check.
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
pnpm exec spec-validator validate --only summary,fileLimits,governance,verification,book
```

**Notes**

5 validators passed.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm exec prettier --write <changed spec Markdown>
```
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
rustfmt --edition 2024 <changed Rust files>
```
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
pnpm web:check
```

**Notes**

0 errors, 0 warnings for generated-contract consumers; production UI remains a separate unfinished slice.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Production renderer parity, delivery-fault acceptance and isolated cold
  native/local browser flows remain open.
- macOS linker reports __eh_frame section too large. Prior ResizeObserver
  warnings await fresh browser acceptance. Visual baselines unapproved.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-CHAT-LIVE-004 details 2](details-02.md)
