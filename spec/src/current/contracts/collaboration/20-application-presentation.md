# Application presentation

The repository Reviews page opens the Community review detail composition with
Description, Files changed, Commits and Activity. Its detail replaces the list
in the main page from either repository or Projects review navigation,
independently of the thread discussion layout preference. The fixed header and
comment composer surround one scrolling body; comments remain under Activity,
and page comments select no parent until an explicit reply action.
Draft/edit/reply state and existing native commands are retained. Its shared
native activity timeline is also used by Projects and Tasks: native commit rows
contain inline Agent context disclosures using recorded associations, sanitized
messages and tool-call primitives. Code and agent roots remain independent. An
anchored agent snapshot that differs from the available selected session is
explicitly unavailable; the UI does not represent the latest session as the
original artifact. Review comments, resolve/reopen, durable unknown requests and
inline diff/tool threads retain their existing service contracts.

The shared Activity timeline and sanitized transcript use Community's measured
virtual rows in shadcn ScrollArea viewports. Agent association metadata is read
for mounted commit rows only, under the captured AgentStore root and a bounded
reader budget. Session detail reads and transcript rendering are deferred until
the disclosure opens. A changed root is reported as unavailable until explicit
refresh, rather than substituting newer context. Samples without recorded code
associations cannot imply that a session produced a commit.

The LocalTrusted application exposes Agent activity and Reviews as first-class
repository pages. Their stable hash routes retain the selected operation plus
either the complete stable session or thread identity. Reload and browser
Back/Forward restore that exact selection; collaboration identities never occupy
native operation, commit or change fields.

Agent session, transcript and review-thread collections use cursor-paged
TanStack Query reads scoped to the active access session and repository.
Measured virtual rows bound rendered session, transcript, thread and comment
collections. Agent session rows pair the integration's agent avatar and name
with state, captured start time and captured duration when exact evidence timing
is available. The selected session displays its independently pinned AgentStore
root alongside the selected native code snapshot. Review detail displays its
independently pinned ReviewStore root and exact anchor, retaining the durable
request ID whenever a command outcome is unknown. Reply, resolve and reopen use
the same optimistic-concurrency inputs and status route as the LocalTrusted
service contract.

The presentation consumes Community's public `CommunityMessage` and
`CommunityChatComposer` components plus Design Core chat tool-call and tool
detail primitives, then adapts native agent and review records into their public
presentation models. Agent activity renders as a normal Community conversation;
tool rows retain their native kind and state, expose copyable stable identities
and use a kind-specific detail without escaping the transcript's inline inset.
Edit, delete and move details embed the exact changed-file diff when the loaded
revision patch or sanitized old/new tool input can supply it. The agent header
exposes the associated Git/JJ identities and a route link back to the related
change. Review comments use Community's separator-free compact timeline and
bounded reply-tree indentation, guides and connectors, retain copyable
author/comment identities and keep the composer at the bottom of an open
discussion.

Each rendered text-diff row exposes a keyboard and pointer reachable comment
action through Design Core's line-accessory boundary. Activating it opens an
inline composer after that row. A committed first comment creates one
`code_text` ReviewStore thread whose immutable file object, UTF-8 byte
selection, digest and explicit before/after comparison operations are resolved
from the selected native comparison; an ambiguous, mismatched, conflicted or
unselectable row fails closed. The resulting discussion renders through the
line-annotation boundary immediately after the anchored row. Existing code-text
threads use the same exact row placement, while whole-file anchors retain their
below-diff placement.

Native diff hunk starts remain byte offsets in the transport. Before exposing a
line comment action, presentation MUST derive one-based unified-diff positions
by accumulating the ordered hunk contents on each side; a zero-length side uses
the preceding position required by unified diff syntax. A native byte offset
MUST NOT be presented or resolved as a source line number.

Each agent tool-call row likewise exposes a keyboard and pointer reachable reply
action. Its first committed comment creates one `agent_artifact` thread bound to
the selected session snapshot and exact tool snapshot; subsequent comments
append through the thread's normal workflow precondition. A captured V1 tool
call without finer attempt evidence uses its stable tool-call identity as the
single coalesced attempt identity. Tool discussions render directly after the
call with the same bounded reply indentation as Community review threads, and
the Agents route retains the selected review-thread identity.

Design Core supplies the column canvas, diff, Empty, Alert, Button and Skeleton
primitives. Revision and operation selection follow bounded association and
review pages to the exact commit or operation match, open sibling Agent context
and Review discussion columns when present and record both stable selections in
the route. Operation review matching includes direct operation anchors,
revision/code anchor operation IDs and both sides of an explicit code-text diff
context. Selecting a changed path follows review pages to the exact code-text or
code-artifact anchor for the selected revision or operation and keeps that
discussion in the review and inline diff surfaces. Repeated cursors or more than
4,096 inspected context rows fail closed. Agent and review list rows use the
same separator, metadata and tag hierarchy as Branches and Op Log, and
SHA-shaped identities are presented through the existing copy controls instead
of bare shortened text. The Files header resolves the selected commit
independently of graph history and displays its author avatar, message, Git/JJ
identities and relative time. Collaboration columns have explicit close actions
that clear their route selections. All canvases are keyboard and pointer
resizable and persist their versioned layout through the application's public
storage boundary. Catalog stories inject memory storage and cover ready
interaction, exact revision/path context, pagination-compatible virtualization,
loading, empty, failed and durable command states with automatic accessibility
checks. Stories remain `visual-pending`; interaction and accessibility evidence
does not approve a visual baseline.

Repository SSE events announce that newer collaboration history exists without
mutating displayed roots or records. Explicit Refresh invalidates the complete
repository-session query scope, captures the current operation heads and then
restores the selected collaboration route against the new snapshot. Disconnect,
credential replacement and repository switching cancel requests and clear the
protected query cache; late detail responses are ignored before assigning
session or thread state.
