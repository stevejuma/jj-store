# Interactive agent execution

JST-COLLAB-031 through JST-COLLAB-033 govern explicit local execution using
jj-acp control version 2. The application owns authorization, durable intents,
workspace selection, prompt preparation and evidence publication. The harness
owns protocol, warm ACP sessions, permission exchange and bounded termination.

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

The next explicit Prompt reads authorized committed, visible discussion replies
from one captured repository selection, with bounded object, reply and byte
budgets. Target membership is revalidated; tombstoned threads/messages and
unavailable targets are excluded. Context identifies the original message and
exact reply revisions, separates human feedback from the new prompt, and is
retained with the original prompt request before dispatch. Retry/status reads
never re-read feedback or dispatch again. Saving feedback, reacting and browsing
never execute or resume an agent. Read failure or exhausted context budget
retains the prompt draft and fails closed instead of silently omitting replies.

The inbox retains the existing native code-repository Thread/Reaction refusal
until reducer 3 is qualified. The supported native write profile is a WorkStore
home with a frozen origin ledger; local companion providers retain their own
qualified ref-CAS behavior. Browser capability presentation cannot bypass these
publication rules or qualify an unimplemented registered-folder transport.
