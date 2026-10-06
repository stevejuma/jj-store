# Agents, threads and reviews

The application keeps several kinds of collaboration separate because they
answer different questions.

| Record               | Purpose                                                                        |
| -------------------- | ------------------------------------------------------------------------------ |
| Agent activity       | Retained, sanitized evidence of an observed coding-agent session               |
| Thread/message       | Repository-owned human conversation and replies                                |
| Review thread        | Discussion attached to an exact repository artifact or supported anchor        |
| Reaction             | A versioned response to an exact target, with observed-state conflict handling |
| Project/task comment | Work discussion owned by its project metadata home                             |

## Observation is not execution

An Agent page shows retained activity. Capturing a session does not launch,
continue or authorize the agent. A code association is an explicit assertion
with exact source evidence; temporal proximity between a tool call and a commit
is not proof that the agent authored the commit.

Automatic Codex capture and explicit hook integrations have different setup and
qualification contracts. Supported source versions are checked. A sample session
from another harness does not establish that its automatic reader is enabled or
supported.

## Exact targets

A review can refer to a repository, code range, operation, diff or agent
artifact only where the selected profile supports that anchor and retains its
evidence. Display labels and current file paths are not adequate substitutes for
exact versions. Local repository discussions are supported independently of
portable code/agent-anchor qualification.

Reactions retain exact target and actor identity. Conflicting states are shown
rather than silently choosing the last request by wall-clock time. Comment
editing and deletion retain revision/tombstone semantics; they do not erase
history or rewrite the original evidence.

## Independent lifetimes

Code, agent activity, reviews, Threads and WorkStore records have distinct roots
or versions. Viewing one does not initialize the others. An ordinary code undo
need not remove a discussion about that code. The selected provider is
responsible for authorization, durable publication and keeping referenced
immutable evidence reachable.

## Interactive execution

[Agent sessions](../guides/agent-execution.md) explicitly dispatch installed
Codex/Cursor adapters in new isolated JJ workspaces. The device supervisor owns
request receipts, permission lifetimes and process state; AgentStore owns
committed sanitized observations through its versioned controller projection.
Multiline tool titles remain evidence while tool snapshot labels are bounded.
Unknown or unusable turns never project an idle session.

In the native execution inbox, replies to saved agent and prompt messages use
ordinary ThreadStore revisions with an exact AgentMessage baseline. Saving them
is separate from execution. The next explicit Send includes committed visible
replies as bounded follow-up context; reads, reactions and reply saves do not
start or resume a harness. See [Agent sessions](../guides/agent-execution.md).
