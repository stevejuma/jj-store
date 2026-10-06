# Interactive agent execution

The installation Agents destination is a primary sidebar item beside Projects.
`#/agents` lists saved named setups as agents without selecting a repository.
`#/agents/new?agent=<id>` opens a new draft chat in the inbox;
`#/agents/sessions?agent=<id>` opens the Community inbox layout, with chats on
the left and the selected session on the right. `#/agents/sessions/<id>` retains
direct session links and an optional agent filter. Activity log opens the
associated connected running session, otherwise a connected or starting session,
or the filtered inbox when no session is active. An explicit session selection
keeps its exact history. Legacy `#/agents/activity?agent=<id>` routes redirect
to this inbox behavior. Empty history never redirects to launch preparation.
Existing repository Agents routes retain operation-pinned recorded activity.

Community gallery, avatar/presence, editor, navigation, activity and composer
presentation is copied and maintained under web/src/lib with source revision and
license provenance. Application adapters own routes, local execution data and
commands. Agent cards open the owned Community profile in a closeable right
sidebar, retaining the standalone gallery. The sidebar has Info/Runtime, current
presence, saved instructions/preferences and explicit launch/history/edit
actions. The agent details header has no bottom border. Closing returns keyboard
focus to the card; mobile uses the shell sidebar view. Overflow actions offer
configuration, explicit launch, session history and deleting the saved
configuration. Deletion never ends or cleans up a session. The standalone
management page contains the gallery only. Agent and Add tiles retain a fixed
width and height when details opens; available width only changes columns.
Add/edit uses the owned Community dialog, image upload/HTTPS URL/colored emoji
avatar picker, optional description, instructions and combined harness/model
picker. Legacy plain emoji values remain readable and unchanged until explicitly
edited. Uploads retain bounded inline image data in private installation state;
there is no remote upload or server image fetch.

Online requires a live supervised ACP session whose handshake has completed.
Working requires a connected running turn. Preparing/opening sessions show
Starting; no connected or starting associated session means Offline. Failed
status refresh means Unknown, retaining cards and drafts. Runner installation
availability is separate. Status refreshes while the workspace is active, after
commands and reconnect, without prompt replay.

Sessions retain their original setup ID; legacy sessions remain unassigned.
Editing/deleting a setup affects future launches, never the captured
configuration of existing sessions. Configuration options derive from negotiated
capabilities. The agent editor uses current live negotiation or the latest
retained catalog for the same harness, explaining remembered choices and
checking availability on connection. A live negotiation overrides retained
choices even when empty. Missing catalogs explain unavailable choices and
preserve saved preferences.

Launch chooses an authorized repository and exact revision, creates an isolated
workspace, negotiates runtime options, then accepts an explicit prompt.
Repository/project/task actions prefill visible exact context. Session
navigation, queued follow-ups, tools/plans, exact permission cards,
stop/end/resume and review links retain drafts and pending request identities
across navigation. Operational feedback uses application toasts and contextual
recovery controls.

Activity-stream reconnection retains explicit command refusals independently. A
valid recovered event frame resolves the stream condition; unrelated status
refreshes do not erase command errors or imply stream recovery. Reconnection
only requests retained event ranges and never dispatches a prompt or command.
Conversation text inherits Community/Design Core message foreground; cold
browser accessibility checks populated user and assistant foreground/background
contrast.

A stale setup save keeps its draft and original conditional version. The editor
shows contextual recovery with the latest saved fields and an explicit action to
keep the draft while adopting the current version for a new save. Status refresh
alone never overwrites a draft or advances that saved precondition.

While configuration privacy preparation is incomplete, the editor keeps draft
controls and Cancel available, explains the state and disables Save. It does not
dispatch an unavailable save or turn preparation into a retained unknown
request. Agent ID copying uses application-owned success/failure feedback.

Profile Start session opens explicit preparation. Stop agent ends its associated
connected session by exact ID; it never targets an unrelated selected chat.
Multiple connected sessions expose individual controls in the inbox. Stopping
preserves captured configuration, workspaces, history and chat drafts. Opening
Activity log retains the draft and pending command across Back/Forward.

Tiles shrink in width only when the screen is narrower than one tile.

Gallery tiles show only avatar, name and saved model (or harness default).
Presence remains on the avatar; readable state and harness installation stay in
the details sidebar. The gallery has no duplicate status badge or description.

Profile actions use compact centered icon/label tiles: a filled Start/Stop
control and a muted View sessions control. A single connected session replaces
Start with Stop; the action targets that exact associated session.

Compact gallery cards retain a 13rem width and 16.25rem height. Offline agents
show Community's pill-shaped Start control over the avatar; it opens preparation
for that setup, requires explicit workspace creation and never submits a prompt.

Starting from the profile or gallery avatar opens the inbox with a selected
draft chat and Community composer. A centered shadcn Empty state with an icon
contains Agent, Repository and Starting revision command pickers in the existing
settings form composition. Agent avatars retain their own fixed dimensions
within picker triggers and rows. Repository revision choices use bounded,
operation-pinned reads from only the selected repository, reject mismatched pins
and repeated cursors, cancel superseded reads and retain the selected revision
on failure. Changing a draft agent preserves repository/revision selection and
retained messages. Connect agent explicitly creates the isolated session and
transfers the draft to its composer. It never submits the draft. Repository,
revision and agent selection are fixed after launch. A matching structured
index_rebuild_required refusal offers an explicit Rebuild index button for the
selected repository and operation, using its existing profile-owned preparation
transport. Task polling is bounded and validates the full stable pin; changing
scope cancels it. Failure retains the revision and chat draft and never silently
replays index creation or rebuilds for authorization errors.

The inbox header uses normal Community padding, the shared application sidebar
toggle icon for the chat list and a kebab menu. Activity log and End session
have icons; Workspace and runtime opens a closeable right body sidebar using the
same settings field groups and rows as Settings. Conversation, composer and
drafts remain visible, and unapplied preferences survive closing/reopening the
panel. Mobile uses the shell's right body panel. Resume, continuation, review
and cleanup remain explicit and target the exact session. The composer supplies
its own Send/Queue action and Stop turn; only an accepted outcome clears an
execution draft. Below the composer, only the compact Community activity strip
and exact pending-request recovery actions appear; no ordinary settings or
explanatory text is shown. Empty conversation bodies use shadcn Empty with an
icon. Loading the selected session or its retained history shows the owned
message skeleton and keeps the header, list and drafts intact; superseded
history completion cannot clear a newer loading state.

Agents gallery skeletons use the same fixed dimensions, grid gap, padding and
rounded avatar/title/model layout as loaded cards, including narrow screens.

Inbox session rows show the last retained activity time using the owned
Community relative-time presentation with its exact-date tooltip. They omit the
bottom row when timing is unknown; lifecycle state and repository/session IDs
are not repeated there. Draft rows retain their message preview. The header menu
is wide enough for its action labels and remains bounded by the viewport.

Session settings use the conversation body background. Workspace and Starting
revision reuse the complete Settings StoreReadOnlyRow component, truncating
visible text while copying the full value. Starting revision offers an internal
entity link bound to the exact repository, base commit and retained operation.
Model, Thinking and Execution mode reuse Community agent-profile runtime rows
with values from the associated saved agent, falling back to retained session
preferences only when no saved agent exists; their edit action opens the same
StoreAgentSetupEditor and CommunityAgentEditorDialog used by agent details,
including avatar, identity, instructions and runtime fields. Save agent
conditionally saves future defaults, then explicitly configures the connected
current session when its harness is unchanged and a mode is selected. The dialog
explains these two scopes; each command retains its own request identity and a
failed second command retains the draft and the now-current saved revision.
Session instructions, workspace and original association are unchanged. No
separate runtime dialog or inline Apply runtime preferences button is shown. The
current session harness stays fixed; future defaults may choose another. Dialog
preferences survive closing/reopening, and unavailable capabilities retain
values with an explanation. Review changes and Clean up workspace form one group
directly after the agent runtime rows. Workspace, Agent settings and Actions use
the existing Settings section headings. Cleanup is a red row with a trash icon
and is available only for a disconnected session that is not already cleaned. It
preserves the existing exact-session cleanup and file-loss refusal behavior.

A disconnected session keeps the composer editable for drafts while Send
requires an explicit resume or continuation. Its contextual message names that
lifecycle state instead of treating configured agent values as missing. On
explicit Send, a connected session may configure its associated saved runtime
choices when its negotiated mode is available; the prompt waits for confirmed
mode readiness and an accepted receipt for that exact configure request before
prompting, retaining its captured session identity and message. Failed/unknown
configuration, disconnection, navigation or bounded readiness timeout retains
the draft and sends no prompt.

Resume and current-session command failures use the Community composer's error
status, retaining its editable draft and history. They are not duplicated in the
conversation body or a toast while that chat is selected. Disconnected history
prefers its latest retained safe ACP error code over a generic process outcome;
successful reconnection clears the historical failure presentation. Provider
errors remain within the existing sanitized ACP boundary. Recovery requires an
explicit resume or new session with saved context; no automatic fallback or
prompt replay occurs.

Agent draft and session chats reuse Community's conversation row layout: an
independent scrolling message area above a bottom composer row. Composer growth
resizes the message viewport instead of covering messages. The shared Design
Core LayoutScrollButton offers keyboard-operable Scroll to latest/New messages
recovery when scrolled back, with scroll state scoped to the selected chat.

Inbox messages reuse the shared Design Core Message, MessageBubble and
MessageMetadata composition used by recorded agent chats. Known retained event
observation times use the existing CommunityMessageMeta and
agentMessageTimestamp formatter with clock text and an exact-date tooltip. User
prompts and coalesced assistant messages keep the first event time across
streaming chunks and refresh; legacy messages with no recorded time show Time
unknown without a fabricated time element. Turn lifecycle records render as
shared SystemMessage notices within the chat log. A connected running session
shows Community's streaming Reasoning Working indicator in the message area and
its compact avatar/activity strip below the composer. Pending outcomes use
readable labels in that same strip, preserve exact retry/review actions and
never replay a prompt. The running strip is scoped to the selected session and
cannot leak into another chat or the directory.

Published inbox prompt and assistant messages expose the existing Community
message action toolbar, exact reactions and a reply discussion composer. Replies
use the ordinary repository ThreadStore and stay visible after navigation and
restart. Unpublished records retain reply drafts but show that publication is
required before saving. Saving a reply never sends a prompt. On the next
explicit Send, committed replies become follow-up context; reaction values
themselves are not treated as authored instructions. Reply/feedback failures
retain drafts and exact pending collaboration identities beside their controls.

Repository feedback availability is shown beside the reply composer. Unsupported
native code repositories and registered-folder inbox transports retain editable
reply drafts while Save and reaction actions are disabled. Existing replies stay
readable. Native WorkStore homes retain their qualified publication profile;
code repository reducer-3 qualification remains a separate gate.
