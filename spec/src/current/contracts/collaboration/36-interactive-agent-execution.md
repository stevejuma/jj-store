# Interactive agent execution

JST-COLLAB-031 through JST-COLLAB-033 govern explicit local execution using
jj-acp control version 2. The application owns authorization, durable intents,
workspace selection, prompt preparation and evidence publication. The harness
owns protocol, warm ACP sessions, permission exchange and bounded termination.

The application sanitizes assistant and reasoning text across ACP chunk
boundaries before journaling, events or capture publication. Possible privacy
root prefixes remain bounded memory-only state until resolved; split workspace,
home, temporary and exact credential replacements cannot bypass normalization.
Message identity changes, tool boundaries and terminal frames flush the held
text in order through the same scanner. Buffer failure is unavailable, never
raw fallback. Restart never replays an inference to repair historical captures.

Codex and Cursor are the initial qualification targets. Executable availability
is distinct from installed-product acceptance. Ordinary reads, capture imports,
comments and reviews never dispatch execution. Launch actions are available only
through admitted LocalTrusted Admin APIs and independently authorized code
repositories. A project or task home does not authorize linked code.

Saved named setups and process receipts are installation-owned private state.
Setups retain name, avatar, optional description, instructions and runtime
preferences with conditional revisions. Sessions retain an optional original
setup_id independently of setup edits/deletion; legacy sessions remain
unassigned. The read-only connected projection requires a current supervised
process and completed ACP handshake, never persisted history or executable
availability. Restart disconnects all sessions without replay. The executable
and process environment remain launch configuration. No browser request supplies
commands, environment, credential or arbitrary path. A stable original request
identity precedes every dispatch. Unknown outcomes are queryable and never cause
prompt replay. Service restart preserves history and workspace ownership, not
executing processes. Resume is explicit and capability gated; unsupported resume
offers a new session linked to retained context.

JST-API-001 and JST-API-006 also cover the additive local status DTO fields. The
admitted local execution status exposes read-only recorded_session_ids,
workspace_name and workspace_operation projections. Recorded IDs come from the
capture normalizer's exact committed session aliases, never a client-side hash
or name match. The workspace name/operation come from the retained, ready
workspace proof and are omitted after cleanup. These projections are recomputed
for legacy journals; their saved values are not authority. Repository UI joins
require the exact repository and recorded session ID, or exact workspace name,
and refuse ambiguous matches. Captured session title/avatar and the original
setup_id remain installation metadata after setup editing/deletion. Native
AgentStore bytes and session identities are unchanged; these presentation joins
do not publish agent settings.

Every session owns a new named physical workspace from the exact selected
operation/revision. Creation retains an original journal and preserves source
working files, including unsnapshotted edits. Completed or stopped turns may
snapshot only their isolated workspace under exclusive ownership. Changes stay
available for review. Cleanup revalidates physical identity and refuses file
loss; merging, pushing and automatic deletion remain outside this profile.

An independent bounded stream exposes sanitized live activity and current
permissions. Published AgentStore observations retain stable controller session,
turn, tool and source-sequence identities. Pending live activity is
distinguished from committed immutable evidence and deduplicates after
publication. Permission options preserve exact adapter identifiers. Stale,
foreign and expired decisions fail closed; no permission is automatically
approved. Model/mode controls derive from the negotiated session, and
security-relevant configuration must be applied before prompting.

Local execution has bounded process/queue/event/session-state limits, defaulting
to one active process and sixteen queued turns. Owner shutdown cancels and
cleans up before authority storage stops. Publication failure preserves
sanitized evidence and command identity independently of process completion.
Automatic capture and managed execution sources do not publish duplicate session
copies.

Controller tool aliases use `json:` followed by the JSON tuple of scoped turn
and exact adapter tool-call ID. This reversibly escapes controls in opaque IDs,
including installed Cursor newline-bearing IDs. If the encoded alias exceeds
1024 bytes, `sha256:` selects the full tuple digest. The exact ID remains in the
sanitized observation; projected tool IDs continue to derive from the original
full scoped ID. No native JJ identity, enum encoding or validation limit
changes.

Avatar values admit legacy emoji, credential-free HTTPS URLs, up to 1 MiB of
PNG/JPEG/GIF/WebP inline bytes, or Community's fixed colored emoji SVG template.
Arbitrary SVG markup and other schemes are rejected. Authored text remains
sanitized; validated image bytes remain opaque. The setup endpoint is bounded to
2 MiB; other command limits remain unchanged. Setup and retained session avatars
together are bounded to 8 MiB, within the existing 32 MiB journal and 16 MiB
response limits. Failed size checks publish no partial save.

The standalone profile and inbox controls end an explicitly selected associated
session by ID. Activity log navigates to the associated active session or the
filtered inbox; legacy activity links retain explicit session selection.
Browsing the session inbox or activity log does not launch a process, replay a
prompt or replace a chat draft. Session end retains its captured configuration,
workspace and durable history.

Session last_activity_at_millis is optional private execution metadata, captured
when an event is retained and preserved through restart/resume. Legacy history
without a timestamp stays unknown until new activity. Status reads, setup edits
and publication progress never fabricate activity. It does not change native
AgentStore event identities or publication authority.

Explicit Send may apply the selected saved agent runtime preferences to its live
session before prompting. It requires an advertised execution mode, confirms
negotiated readiness and an accepted receipt for the exact configure request,
and retains the exact selected session and message. Failed/unknown
configuration, disconnection, navigation or bounded timeout retains the draft
and dispatches no prompt. Reads and editor navigation never apply preferences or
resume a process automatically.

Each execution event optionally retains observed_at_millis, the device
wall-clock observation time captured when that event is appended. It is
read-only, survives journal restart/resume and publication, and defaults to
unknown for legacy events. It is not a provider-authored timestamp and does not
change native AgentStore identities or authority. Refresh, status reads and
rendering never fabricate missing times.

Repository discussion in the inbox targets the exact published AgentStore user
or assistant message. A ThreadVersion optionally retains an immutable
agent_message target (session, message, event batch/event and content); the
CreateAgentDiscussion command accepts only that typed target and validates it
against the currently selected AgentStore before publication. Legacy versions
omit the field without changing their canonical bytes. Replies remain ordinary
ThreadStore human message revisions; reactions retain the original exact
AgentStore target and server-derived actor. Pending capture supports drafting
but cannot receive durable feedback until its original record is selected.

ACP session-load history updates scoped to the supervised opening request do not
create new canonical user/assistant messages. Inbox projection excludes those
replay rows instead of assigning fabricated feedback targets. Retained reply
drafts require explicit selection of a validated original message; restoring a
draft never saves feedback or dispatches execution.

The next explicit Prompt captures one authorized AgentStore/ThreadStore
selection. It collects visible human replies to exact published session messages
and visible human messages in its active primary repository thread. Only new or
edited immutable revisions are selected, identified by
repository/thread/message/revision IDs; overlapping paths deduplicate. Parent
relationships, authors and original message/thread context accompany the
separated feedback section. Reactions, review comments, unrelated threads and
tombstones are excluded. The selected batch remains bounded to 128 replies and
32 KiB. Read or budget failures retain the draft and dispatch nothing.

The private journal atomically retains original sanitized prompt intent,
prepared frame and exact selected feedback batch before dispatch. Queued batches
reserve revisions; the exact harness started event atomically marks them
delivered. Proven rejection/cancellation before start releases reservations;
uncertain outcomes retain the batch without rereading context or replaying
prompts. New replies after the captured selection wait for the next explicit
Send. Resume and restart preserve delivery state. Fresh adapter continuation
uses whole retained records and initializes delivery state from structured
feedback actually included; it never infers delivery from transcript text.
Carried context reserves its revisions until that prompt starts; cancelling the
first queued prompt keeps initial context eligible for a later explicit Send.
Legacy journals begin with empty delivery metadata and include currently
eligible replies once on their next Send. ThreadStore, ReactionStore and jj-acp
protocol formats remain unchanged.

The inbox retains the existing native code-repository Thread/Reaction refusal
until reducer 3 is qualified. The supported native write profile is a WorkStore
home with a frozen origin ledger; local companion providers retain their own
qualified ref-CAS behavior. Browser capability presentation cannot bypass these
publication rules. Registered-folder inbox commands use the same local provider,
captured profile, exact device actor and original ref-CAS receipts as the CLI;
their browser transport requires independent cold proof.

Ordinary discussion commands remain available for published targets after end,
disconnection and restart. A retained validated target survives transient read
failure; mutation-time authorization and target validation remain authoritative.
Reply and edit contexts retain exact parent/revision witnesses across refresh; a
changed or unavailable target fails explicitly without replacing the draft.

JST-COLLAB-031 through JST-COLLAB-033 keep runtime installation separate from
saved agents, repository collaboration and execution. Settings installation
changes future tooling discovery only; it never starts/resumes/prompts an agent,
changes saved preferences or alters repository history. A live process blocks
replacement. Authentication remains the harness vendor's explicit device login.
