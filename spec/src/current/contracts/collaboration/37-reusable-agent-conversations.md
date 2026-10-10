# Reusable agents and contextual conversations

M24 extends M23 with explicitly selected jj-acp control version 3. M23's prior
qualification remains unchanged. A saved agent is installation-wide; its worker
is initialized independently of sessions. Start warms in place with no prompt,
repository or workspace. Stop retains history/workspaces and disables background
warming until explicit Start or Send. Newly migrated saved agents begin
disabled. Enabled agents are warmed on service startup within the four-worker
cap.

A conversation has a stable identity and optional origin (project, repository,
thread or task). Opening prepares a draft only. Explicit Send creates or resumes
agent participation; separate agents have isolated directories. Reuse is keyed
by conversation, agent and repository. Multiple code attachments are authorized
and pinned independently before any prompt; source working files are preserved.
Registered native repositories without physical workspaces may be materialized.
Unknown external repositories require registration; cloning is deferred.

Repository-only attachments may pin an immutable operation/revision pair from
the visible code snapshot independently of the source checkout's recorded
operation. Named workspace attachments retain their current-operation fence.
Project metadata pins are never substituted for linked code repository pins.
Preparing an isolated checkout loads the selected code operation and revision
without refreshing or snapshotting source working files.

A repository-free chat lazily provisions a personal WorkStore home on first
Send: native on managed-native installations and a local companion otherwise. An
existing discussion keeps its metadata home. One canonical transcript is
published at the home through existing AgentStore authority and durable
receipts; originating discussions retain typed links rather than fabricated
human replies. Repository-free and project/task conversations have a
receipt-backed metadata thread. An agent-message reply creates a typed
discussion target for the exact recorded message. Visible originating
discussions watch their canonical agent responses even with the chat sidebar
closed. Code evidence is separate and independently authorized. Native code
ThreadStore reducer refusal and WorkStore origin freeze remain unchanged.

Logical participation survives process generations. Each generation has exact
worker, session and turn fences. Explicit Send prepares the selected agent,
reuses a connected participation, and resumes only an advertised adapter session
with a retained prompt-start acknowledgement. A newly opened adapter session
without an acknowledged turn may have no persisted rollout; preparation creates
a new session with bounded retained context in that case. Changing the selected
agent keeps the conversation identity and prepares that agent's own
participation on Send. Preparation never prompts on selection alone. Otherwise
an explicit continuation includes bounded retained context and discloses the new
adapter session. No restart, crash or unknown outcome replays prompts. The first
real prompt includes structured bootstrap instructions, origin and pinned
attachments; warmup never sends an instruction-only turn. Context revisions are
reserved and marked delivered only at the exact started event.

Failed explicit Resume/Continue commands settle their own durable receipt. An
exact adapter rejection is failed; an unconfirmed open remains outcome unknown.
A scoped error that tears down the adapter also retires the controller worker
and invalidates its live participations, preserving the classified failure and
never replaying prompts. Ending an opening participation cancels its opening
receipt; delayed handshake frames cannot reopen that ended participation.

A valid adapter RPC rejection of open/configure interrupts only that
participation and settles its receipt; the agent and its other sessions stay
usable. Failed preparation sends no prompt. No automatic fallback follows a
rejected or uncertain resumed turn.

Limits are four workers, one active turn per worker and participation, sixteen
queued turns per participation and sixty-four total. Surplus idle workers retire
after ten minutes. Stopping a turn, ending participation and stopping an agent
are distinct actions. Permission decisions retain exact live identities/options.
Each repository snapshot is reported independently; no cross-repository
atomicity is promised. No automatic merge, push or destructive cleanup is
introduced.

The very top bar places Chat immediately before Details, following Terminal when
present. Project and repository chat opens in the standard right body sidebar,
with a fixed header/composer and scrolling transcript; the outer details rail is
independent. Reopening restores the scoped chat and draft. The sidebar header
shows the exact selected agent's name/avatar with authoritative ACP presence, or
the participants' avatars for multiple recipients, and the standard far-right
close icon; an unaddressed draft retains the Chat title. New-chat actions remain
in the agent navigation. Repository selections use the previous centered chat
draft and divided form rows inside the body. Multiple attachments remain
supported. Repositories with multiple physical sources expose a workspace
picker; choosing one captures that target's exact operation/revision. The
far-right header control is the standard close icon. Visible context/repository
attachments can be edited before sending; unresolved requests freeze their
original selections.

Chat update/reconnect failures appear as warnings inside the composer. Exact
Send/recovery errors retain precedence; the warning clears on successful history
refresh without disabling composition or replaying a prompt.

Each completed turn with confirmed repository start/completion snapshots keeps
its own full-width panel in the transcript, including during later work. It
compares that turn's exact base against its saved commit at the recorded
operation. It shows edited-file totals, captured line counts, three initial
paths, an expandable remaining list and View changes with the same comparison in
the shared Changes rail and file inspector. The saved SHA/Change ID control and
list/folder/package toggle reuse existing components. Older cards load when near
the viewport and share bounded session-memory caches. Failed reads use a warning
alert with a themed icon-bearing Retry button, retaining drafts. Native and
registered-folder reads remain pinned and bounded; partial, binary, unavailable
and unknown outcomes cannot imply zero changes or fabricated counts. Reads admit
at most two 50-path pages and four concurrent file comparisons, retaining at
most 2 MiB of diff text. Counts from available hunks remain exact even when a
text preview exceeds that retained-byte limit. Each repository has an
independent panel. No panel read, expansion or diff view dispatches execution.
Generic repository-operation Undo is not a session-scoped inverse; the panel
exposes that action as unavailable until an exact guarded agent-change undo
contract is qualified.

The composer supports @ autocomplete using exact saved-agent IDs and multiple
recipients. Its automatic-mention switch appears inside the agent picker.
Selecting or pinning an agent while enabled immediately shows Community's
addressed-agent avatar pill beside @; its dismiss control removes that exact
recipient. The picker also exposes individual pin controls. These controls are
disabled while the composer is blocked and never send a message. Encoded agent
tokens have an explicit closing delimiter, so focus changes and adjacent text
cannot consume the recipient identity. Automatically mention agents is opt-in.
Pins are scoped by device, actor, home and conversation, persist across
successful sends/reopening, and can be removed explicitly. Origin discussion
recipients seed the audience. The device-local personal-home alias scopes pins
before and after lazy provisioning without resetting the opt-in. Disabling the
shared installation preference suspends automatic insertion but retains scoped
pins and explicit draft tokens. Settings and picker switches save the same value
immediately; pinning a recipient also enables it through the same conditional
settings command. Preference saves never dispatch prompts or save unrelated
configuration drafts. Legacy per-conversation flags no longer override the saved
preference. Editing ordinary text never dispatches agents.

Every exactly associated ACP avatar shows runtime state from one app-lifetime
subscription: Starting, Idle, Queued, Working, Needs attention, Stopping,
Offline, Error or Unknown. Attention outranks working, queued and idle. A failed
runtime-status refresh means Unknown; independent workspace-discovery failures
retain authoritative runtime status and report selection unavailability. Offline
requires authoritative absence. Historical identity and avatar remain captured;
live status requires an exact saved setup association. Unassigned legacy records
are never guessed. Indicators have shape/text and accessible labels, tooltips,
keyboard support and reduced-motion behavior.

The app-lifetime runtime subscription consumes installation `agent_execution`
notices on the shared application feed. Defaults use separate `agent_settings`
notices; target discovery follows repository/workspace changes. Healthy idle
agents and contextual chat perform no periodic status/settings/target/history
reads. Initial capture, semantic events, explicit actions and continuity
recovery drive coalesced reads, preserving drafts and exact unresolved requests.
Internal publication retries with unchanged visible state cannot create a
refresh loop.

Publication-boundary recapture retains visible message rows until a complete
replacement arrives. New sequences append without clearing prior history.
Refresh errors preserve retained rows and drafts; a changed commitment/source
mapping updates the existing keyed row. Reset recaptures authority but does not
temporarily remove the selected chat's messages. A failed recapture does not
advance the captured publication boundary; recovery retries its authority read.

Development startup validates prepared text/context and context-only v3 frames
against the selected helper without warming an agent. A control-version flag
alone does not establish compatibility. Incompatible resources fail preflight
before replacing a running service.

Real adapter, cold browser, restart and multi-repository qualification remain
independent acceptance gates. Fixture success does not close them.

Composer mention widgets inherit the application runtime context when mounted
inside the editor, so their avatar state follows the same authoritative source.

Add agent discovers advertised models, thinking and modes before a first chat. A
bounded read-only ACP catalog probe may negotiate a disposable adapter session
in a private global directory, without a prompt, repository, worktree, saved
agent or application conversation. It occupies one of the four process slots,
refuses permission/activity frames, sanitizes configuration only, closes and
reaps its process and retains choices in memory for five minutes. Failure
remains visible and retryable. No discovery read installs tooling or warms a
saved agent.

Bootstrap, repository and discussion context remains separate from user-authored
text in both retained prompt frames and ACP session/prompt content blocks. The
controller prepares bounded XML sections, retains the exact sanitized frame
before dispatch and reserves delivery as before. The harness uses an embedded
resource only when the adapter advertises embeddedContext; otherwise a separate
ordered text block supplies the same XML in the same turn. No context-only turn,
context replay or unauthenticated filesystem read is introduced.

The captured Sent context snapshot retains sanitized authored wire text and the
separate XML wire context. Delivery updates enrich the same user-message row,
preserving its original observation time and independently inspectable snapshot.

Execution feedback includes exact committed tool aliases and selected invocation
or output snapshots alongside messages. Tool discussion replies use ordinary
ThreadStore authority and participate in incremental explicit-Send context.
Reactions remain outside prompt context. Tool discussions admit the existing
typed ToolInvocation/ToolOutput targets in ThreadVersion and
CreateAgentDiscussion; forged or unsupported targets remain refused by exact
resolution. Existing storage formats, reducers, publication receipts and
unsupported-profile refusals remain unchanged.

OwnedController tool normalization retains rawOutput as captured JSON alongside
any structured content so recorded read/command views can display original
bytes. Existing content-only observations keep their representation. Native
aliases, normalizer profile identity and publication receipts remain unchanged;
older observations and snapshots are never rewritten.

## Markdown context and coordination CLI

JST-COLLAB-034 through JST-COLLAB-036 use semantic XML sections containing
native Markdown, separate from authored text. One sanitized typed document
produces both retained evidence and ACP context. Captured markup cannot create
application sections; literal code remains code. Continuations contain bounded
authored messages and assistant replies, never nested injected context.

Bootstrap delivery belongs to the adapter session, independently of origin and
repository section revisions. Only changed sections and new/edited committed
human replies accompany subsequent explicit Sends. Reservations, exact started
acknowledgement, proven non-delivery release and unknown-outcome retention apply
to both section revisions and replies. Loaded sessions retain progress; fresh
adapters receive bootstrap and bounded continuation. Old evidence is immutable.

The controller supplies its verified paired custom `jj` executable through a
reserved environment variable. It provisions and verifies a separate
coordination checkout for the metadata home before contextual dispatch, using
existing native attachment or local workspace authority. This is not a code
attachment or code change. Paths in evidence retain transcript privacy
substitutions; runnable examples use the executable variable and relative
verified directories. Task and review writes require the user's task scope,
exact versions and stable request IDs; uncertain outcomes use existing receipt
lookup, never blind retry.

[Conversation comment submissions](41-conversation-comment-submissions.md)
extend explicit Send with exact range comments and validated answer links,
without changing canonical transcript ownership or automatic-dispatch
restrictions.

## Workspace inspection and file feedback

JST-COLLAB-034 through JST-COLLAB-036 extend explicit conversation Send to
workspace file reviews. The footer identifies a verified participant/repository
binding; inspection never switches a checkout or rebinds an agent. Files reads
current disk content, including tracked and nonignored untracked files. Changes
compares only that binding. Listing, file reads and comparisons never snapshot,
publish operation heads, reconcile repositories or dispatch an agent.

Since last commit compares the current work in progress with its verified parent
revision, including unsnapshotted edits. Last turn compares exact durable start
and completion snapshots for one completed participant request. Older captures
without both boundaries cannot supply that comparison. Branch compares current
content with the verified common ancestor of the configured target or default
branch. Missing configuration requires explicit comparison-only base selection;
ambiguous parents or ancestry are unavailable rather than guessed.

Inspection uses session-bound, independently authorized repository selectors,
bounded pages and content reads, exact source fingerprints and stale-response
rejection. Filesystem roots come only from retained workspace ownership proofs.
Metadata paths and symlink traversal are forbidden. Unsupported content remains
visible with a reason, never an invented empty file or zero-change count.

File discussions remain ReviewStore-owned; conversation batches carry exact
repository-qualified references. Explicit Send freezes drafts, validates their
original fingerprints, snapshots the authorized workspace and verifies the
captured bytes before publishing reviews and the reference batch. Stale sources
retain drafts and require explicit reselection. Required publication and frozen
context preparation precede dispatch. Independent repository receipts preserve
partial success and unknown outcomes; sending is not cross-repository atomic.

Native anchors retain their existing identities. Local file publication must
qualify portable code evidence, complete retained closures and cold original
context reads before enabling that capability. Source and answer links bind the
exact submitted revisions, never current paths or quote matching. No answer tag
resolves a review. M24 and local-review qualification remain separate gates.

Intentional user Thread ownership and the bounded Codebase supersede implicit
directory-visible conversation owners as specified in
[Intentional Threads](43-intentional-threads-and-codebases.md). Standalone chats
still use internal ThreadStore records for comments and metadata. They do not
create an intentional Thread or a primary user Thread assignment.
