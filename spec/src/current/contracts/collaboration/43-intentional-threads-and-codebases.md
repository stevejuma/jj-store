# Intentional Threads and conversation ownership

JST-COLLAB-034 through JST-COLLAB-036 distinguish intentional user Threads from
internal conversation and annotation records. AgentStore retains canonical
transcripts. ThreadStore retains collaboration records selected through the
existing native manifest or local companion publication authority. Internal
records never become ordinary directory entries or primary user Thread
assignments. Opening a chat, warming an agent and generating metadata cannot
create an intentional Thread.

A Thread has one immutable metadata home and an independently versioned Codebase
of zero through eight unique registered repository identities. New explicit
Threads use the personal metadata home; repository-context creation preselects
that code repository without moving authority. Existing Thread owners and exact
links remain readable. Repository membership grants no code authorization.

Codebase changes use observed versions and durable request receipts. A Thread
appears under every authorized attached repository using one canonical
home/Thread identity. Internal records are excluded from directory counts and
ordinary search, while exact comment, metadata and historical reads retain them.
Missing optional fields retain legacy canonical bytes. Legacy records are never
classified by matching a title.

Browser-owned recorded or standalone comment containers use the typed
`CreateAnnotation` command, never ordinary `Create`. Exact committed source
validation remains mandatory when their submitted discussions are published.
This command grants no agent execution or repository access, and its owner stays
excluded after edits and reloads. Existing containers retain their identities;
legacy records are not reclassified by title.

Explicit agent sends freeze the committed Codebase and selected subset. All
attached repositories are initially selected, but each repository and workspace
is authorized and pinned independently. Forged or removed attachments are
refused before dispatch. Membership edits cannot remove bindings from running,
queued or unknown work. After settled removal, future sends use fresh isolated
participation when the prior bindings contain removed repositories; captured
history, files, reviews and receipts remain retained.

Delta's documented Thread can stand alone or include several projects, with one
worktree per project. This application adopts intentional creation, repository
grouping and conversation/file inspection while retaining separate agent
workspaces. DeltaDB synchronization, shared workspaces, forks and rewind are
outside this change. Reference documentation was read on 2026-10-10:
[concepts](https://delta.dev/docs/concepts/core-concepts),
[Threads](https://delta.dev/docs/agents/threads) and
[worktrees](https://delta.dev/docs/concepts/worktrees).

The shared Threads inbox follows the agent inbox. Repository management stays in
the conversation footer; the outer rail reuses the agent inbox Files/Changes
presentation. File reads before execution use authorized pinned repository
sources; subsequent changes name an exact agent workspace. Event-driven cached
readers are shared with repository navigation. M24 remains In progress; visual
baselines require separate approval. The default demo may be destructively reset
only after verification, as explicitly requested for this direction.
