# Agents, threads and reviews

The application keeps several kinds of collaboration separate because they
answer different questions.

| Record               | Purpose                                                                        |
| -------------------- | ------------------------------------------------------------------------------ |
| Agent activity       | Retained, sanitized evidence of an observed coding-agent session               |
| Thread/message       | Human conversation with one metadata home and optional linked repositories     |
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

## Intentional Threads and Codebase

Create a Thread explicitly from Threads beneath Agents. A Thread can stand alone
or include up to eight registered repositories. The create dialog uses a title
and searchable multi-select Codebase picker; Create Thread has its own row. The
inbox groups a Thread under each attached repository, with one shared
conversation and drafts. Repository-free Threads appear under No repository.
Repository pages retain their filtered Threads navigation.

The footer Codebase picker adds or removes repositories. The next agent chat can
use only a subset of those attached repositories, initially all of them. Each
repository still requires independent authorization; attaching it grants no code
access. Agents use separate workspaces. Running, queued or unknown work prevents
removal, and edits check the exact observed Thread revision.

Files and Changes use the same inspector as the agent inbox. Before execution,
they browse a pinned repository snapshot without creating an agent or workspace.
The outer rail has no separate repository picker. Additional message actions,
including Edit and Reply in thread, appear in the hover action menu.

Agent chats and their annotation/metadata owners remain durable internal
records. Starting a chat or commenting does not create an intentional Thread.
Legacy records retain their identities and are not migrated by matching their
titles.

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

## Inline conversation comments

Document and Classic conversations share comments on exact committed human,
agent and tool content. Text selections use UTF-8 byte offsets and a selected
content hash; unsupported or unpublished content explains why selection comments
are unavailable. Existing whole-message discussions remain readable. Review
comments continue to belong to ReviewStore.

Comments appear beside their source and in the pending group above the composer.
Send submits that group with any authored text; comments alone are sufficient.
Drafts survive navigation and view changes within the current application
session. They are not saved to browser storage. Comments sent earlier retain
their exact revision, so editing one does not transfer its old answer to the new
text.

Agents may answer multiple discussions within one turn. A versioned protocol
uses supplied discussion and comment references, and the service validates exact
answer ranges against delivered context before publishing links. A green
double-check becomes Go to reply only when a validated destination exists.
Several destinations offer a chooser; invalid tags remain readable text.

Conversation metadata is an optional ThreadStore catalog of immutable revisions.
It references exact AgentStore snapshots and keeps custom titles, generated
titles/summaries/keywords/goals and per-participation ACP suggestions separate
from captured agent identity. It creates no duplicate human message or primary
Thread assignment. Tombstoned owners are hidden from discovery. Generation
provenance records its provider, model, prompt version, source snapshots, time
and partial coverage.

Installation custom emoji are reusable across repositories. Authored Thread
messages, conversation submissions and reactions retain a validated immutable
snapshot of each selected emoji with its source and static preview. Repository
publication selects that closure with the authored use. Directory changes do not
rewrite history, and retries retain the prepared snapshot. Source metadata is
presentation provenance and grants no permission. Repository-local libraries
keep separate identities and meanings.

Workflow definitions and execution evidence live in a WorkStore metadata home.
An admitted optional host observes source events and dispatches configured
executors. The GitHub YAML executor additionally checks YAML from the exact
recorded code revision; a newer source or YAML edit cannot replace a queued
run's pins. Server publication retains metadata while the CLI-owned host and
external Forgejo Runner execute jobs. See [workflow CLI](../cli/jj-workflow.md).
