---
schema: 1
id: E-CHAT-LIVE-003
---

# E-CHAT-LIVE-003 details 2

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
cargo test -p jj-store-cli --locked --test agent_execution comment_answers::comments_only_batches_link_multiple_answers_in_one_turn_and_survive_restart -- --exact (initial)
```

**Notes**

Fixture had not started the publication worker. Diagnostic rerun was stopped after identifying this fixture omission; added supervisor.start_publication before the passing run.
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
cargo test -p jj-store-server --locked --lib agent_execution::prompt_context::tests (review run)
```

**Notes**

A concurrent source edit added Outcome.comment_publication after Cargo had compiled the service dependency. The final sequential check rebuilt consistent sources and passed.
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
cargo test -p jj-collab --locked --test conversation_submission --test comment_reply --test target_resolution
```

**Notes**

4 submission, 4 parser and 13 target-resolution tests passed.
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
cargo test -p jj-store-server --locked --lib agent_execution::prompt_context::tests
```

**Notes**

5 context tests passed, including trusted token preservation and captured markup escaping.
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
cargo test -p jj-store-cli --locked --test agent_execution comment_answers::
```

**Notes**

2 controlled-harness lifecycle tests passed: actual local managed and standalone sessions, 3 comments, shared plus separate answers in one turn, split output chunks, empty authored text, duplicate prevention and cold supervisor reopen. Not a real model or browser test.
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
cargo test -p jj-store-wal --locked --test collab_commands conversation_submission_is_atomic_receipt_backed_and_rejects_forged_ranges -- --exact
```

**Notes**

1 native fake-store boundary test.
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
cargo test -p jj-store-service --locked --test local_collab local_conversation_batch_survives_reopen_with_exact_ranges_and_receipt -- --exact
```

**Notes**

1 local companion/reopen test.
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
cargo fmt -p jj-collab -p jj-store-service -p jj-store-server -p jj-store-cli
```
<!-- /run -->

<!-- run -->
### Run 9

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
### Run 10

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm contracts:generate; shasum -a 256 schemas/openapi.json web/src/lib/api/generated.ts; cmp .local/chat-reply-generated-before.sha .local/chat-reply-generated-after.sha
```

**Notes**

Generated artifacts identical on regeneration.
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
pnpm test:contracts
```

**Notes**

7 schema tests plus contract TypeScript check.
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
pnpm web:check
```

**Notes**

Latest direct generated-consumer check: 0 errors, 0 warnings. Intermediate UI integration failures belong to the pending UI slice.
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
pnpm exec spec-validator validate --only summary,fileLimits,governance,verification,book
```
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm exec prettier --write <four changed canonical/milestone pages>
```
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
pnpm check:progress (before backend split)
```

**Notes**

Evidence tests field requires registered test identifiers, not source paths; corrected to empty with exact commands retained above. Concurrent unfinished UI paths intentionally await their own evidence slice; the committed backend revision is checked separately.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Shared production renderers, default-setting acceptance and isolated cold
  native/local browser publication/restart/navigation remain open.
- Dedicated partial-recipient delivery and unknown-publication recovery
  acceptance remains open. Native answer publication has shared model/provider
  validation but no full native browser qualification.
- MacOS linker warned that __eh_frame exceeds compact-unwind limits. Prior
  ResizeObserver warnings require fresh browser observation; visual baselines
  remain unapproved.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

JJ 0.45.1; locked Rust workspace; Node 26.9.0; pnpm 10.32.1.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

CommentReply ThreadLink variant, version-1 reply protocol, optional Prompt.comments and execution outcome comment_publication; legacy omitted fields retain original encodings. Conversion of answer links deliberately refuses unqualified context rewriting.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Collaboration model, native/local providers, execution supervisor, feedback and generated contracts. No root aggregate lanes, real-provider claims or visual-baseline approval.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Finish shared live/recorded/thread rendering and recovery parity, then isolated browser acceptance.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-COLLAB-034..036 and JST-API-001 define frozen prompt submissions,
  exact answer references and separate publication outcomes.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M24 stays In progress. Controlled-harness and model evidence does not
  close browser, partial delivery, real-adapter or visual gates.

```
<!-- /evidence-field -->

