# Lightweight workflow host (M28)

## Requirements

| ID         | Requirement                                                                                                                                                                                                         |
| ---------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| JST-WF-001 | A configured native metadata home MUST select typed immutable workflow definitions, evaluations, runs and evidence through manifest CAS and durable receipts; source repositories remain independent.               |
| JST-WF-002 | Committed source events MUST retain canonical origin identities and exact operation/commit or immutable conversation anchors; duplicate observation MUST return the first evaluation, including no-match decisions. |
| JST-WF-003 | A workflow run MUST have one authority-selected owner; uncertain execution MUST retain its attempt identity and MUST NOT be reassigned or replayed automatically.                                                   |
| JST-WF-004 | Only explicitly admitted execution hosts MAY dispatch named operator-configured executors; repository contents MUST NOT choose executables or grant source, secret or agent authority.                              |
| JST-WF-005 | Agent execution MUST use existing admitted dispatch and original receipts; command execution MUST use a bounded revision-pinned workspace and retain reporting receipts across reconnect/restart.                   |
| JST-WF-006 | HTTP, native RPC, CLI and browser workflow commands MUST share authorization, validation and outcome recovery; reports MUST be bound to the assigned attempt.                                                       |
| JST-WF-007 | Ordered actions MUST retain logs, step reports, artifacts and agent references; cancellation MUST remain requested until termination is confirmed and explicit retries MUST use a new attempt.                      |
| JST-WF-008 | The Workflows page MUST reproduce Buzz overview, Form/YAML editor, sequence inspector and history presentation with repository/executor controls, preserving drafts and historical selection across live updates.   |

## Execution profile

WorkflowStore uses a new format-6 native home root. It requires an existing
WorkStore home, preserves its frozen origin ledger and supports native copy and
recovery. Source replication does not acquire execution authority. One home
coordinates all hosts; no failover or exactly-once external-effects claim
exists.

The execution host is optional and disabled without operator configuration.
Source-event publication never waits for matching or execution. Initial triggers
are manual, code publication, bookmark movement, new thread messages and added
thread/message reactions; initial actions are admitted agents and named
commands. Conditions are bounded declarative filters. Cron, webhooks, action
DAGs and the Forgejo ConnectRPC runner protocol are deferred.

Trigger conditions use bounded bookmark `*`/`?` patterns, native JJ revsets,
author identities, text comparisons and exact thread/message/emoji selectors.
Source queries evaluate conditions against committed, pinned content; only
condition results and immutable conversation references cross into the metadata
home. Raw source messages are not duplicated there. Code authors use their
recorded email; conversation authors use authenticated principal identities.

Evaluation captures a pinned definition catalog before source polling. A
concurrent edit affects the next event, while duplicate observation returns the
first evaluation regardless of another host's condition set. Only admitted hosts
may submit these trusted source observations. New message events exclude
edits/tombstones; reaction events exclude clears/retractions. Conversation
triggers retain immutable content references and publication position. They have
no invented code revision: command actions receive an empty workspace when no
code anchor exists, and admitted agent attachments may omit code pins.

The neutral workflow model reuses `jj-collab` identity/reference types. This
domain-model dependency adds no UI, storage provider, controller or executor
runtime to the model/WAL layers; the boundary check permits this specific edge.
