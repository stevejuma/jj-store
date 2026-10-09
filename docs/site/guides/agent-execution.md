# Interactive agent sessions

## Document conversations and workspace files

Document is the default conversation view; Appearance can switch to Classic.
Write the next prompt in the document's inline input. The footer retains
formatting, agent, model, thinking and Send controls. Drafts and pending
comments survive view changes during the application session, without browser
persistence. Expand Sent context to inspect its rendered content or
syntax-highlighted raw payload. This is the captured context, not reconstructed
current state.

The repository/workspace button opens **View files** and a **Copy** submenu for
the project path and workspace name. With multiple bindings, choose which agent
workspace to inspect; this does not rebind an agent or switch its checkout. View
files opens a **Files / Changes** rail. Selecting a path opens the resizable
file inspector. Files includes unchanged tracked and nonignored untracked files;
unsupported, binary or oversized content retains its availability explanation.

Changes defaults to **Branch**, against the common ancestor with the configured
target. Choose a base explicitly when none is verified. **Since last commit**
includes current unsnapshotted edits against the verified parent. **Last turn**
uses exact recorded before/after snapshots of the latest completed turn; older
turns without start evidence cannot supply that comparison. Browsing does not
snapshot or publish files. Clean observations are cached until relevant events
or explicit refresh invalidate them.

Comment on an exact supported file selection using its inline comment action.
File comments join conversation comments in the same pending group and Send.
Send validates and retains the source before publishing review references;
changed or unavailable sources keep their drafts for explicit reselection.
Queued comments are validated when their prompt can dispatch. Publication,
delivery and a linked answer are separate states; retain an uncertain request
instead of creating a replacement. Local retained-file support is a narrow
qualification and does not qualify portable agent anchors or every native,
multi-agent, cleanup and recovery flow. See [reviews](reviews.md).

Saved agents are reusable across general chats, repositories, projects and
discussions. Start warms an agent in place without creating a session or sending
instructions. New chat opens a general draft. Stop agent preserves history and
isolated work and disables background warming until explicit Start or Send.
Enabled agents warm on service restart within the four-worker cap.

Open **Agents** in the main sidebar to manage saved names, avatars,
instructions, harnesses and model/effort/mode preferences. Click a card for its
details or use its menu to Edit, Start, Stop or open chats. ACP avatars show
live Starting, Idle, Queued, Working, Needs attention, Stopping, Offline, Error
or Unknown status across the app. Failed status refresh is Unknown; historical
identity is retained and is associated only by its exact saved agent ID.

Use **Chat** beside the top-bar Details toggle in a repository or project to
open the right body sidebar. Its composer and header stay fixed while messages
scroll. Opening is draft-only and reopening restores the scoped chat and draft.
The sidebar header shows the selected agent's avatar and name, or Chat for an
unaddressed draft, with a far-right close icon. The outer details rail remains
independent. Use **@** autocomplete to choose one or more saved agents, write a
message and press Send. **Automatically mention agents** is opt-in and retains
recipients across replies and reopening. Enable it in the agent picker; pin
controls choose individual recipients. The @ control shows the addressed agent's
avatar while automatic tagging is active. Its dismiss action removes that
recipient. Remove a tag or unpin it to stop including that agent. This
preference never sends a message automatically.

A general Send lazily creates a personal metadata home. Existing discussions
keep their own home. Attach registered repositories to authorize code work;
registered native repositories without a selected workspace can be materialized.
Unknown external repositories require registration; cloning is not supported.
The controller prepares every participant and attachment before prompting and
includes instructions, origin context and selected revisions in the first real
message. Reuse is isolated by conversation, agent and repository. Dirty source
files remain untouched, and each attachment reports its own isolated snapshot
and review link. No automatic merge, push or cleanup occurs.

Repository chats use the visible code revision even when its operation is newer
than the source checkout. Project context keeps its metadata selection separate
from linked code. Workspace discovery errors appear independently of live agent
status, so a failed workspace refresh does not obscure a known Idle agent.

Originating discussions retain typed links to one canonical AgentStore
transcript. Agent replies are visible there even when the chat sidebar is
closed. Stop turn, End participation and Stop agent have separate lifetimes.
Restart preserves exact history and never replays an unknown prompt; resume is
capability-gated and a new continuation session is labelled explicitly.

**View chats** also provides the retained session inbox. Its legacy version-2
setup form still supports exact workspace/revision selection and negotiated
runtime options. Historical sessions and their existing qualified controls
remain available independently of the new contextual chat.

The conversation scrolls above a bottom composer row, so growing drafts never
cover messages. The shared Scroll to latest/New messages control returns to
recent activity when you scroll back. It shows streamed text, tool activity,
plans and exact permission choices. Messages show their retained observation
time using the same clock-time metadata as recorded agent chats, with an
exact-date tooltip. Older untimed history shows Time unknown. Turn completion
appears as an outcome icon beside its recorded time in a system divider; the
session starts with a clock icon and its recorded time. Both have accessible
labels and tooltips. During a connected turn, Working appears in the chat and a
compact agent activity strip below the composer. Follow-ups queue in the warm
session; Stop turn cancels the selected turn, and **Session options → End
session** closes the adapter. The header uses the shared sidebar toggle to show
or hide the chat list, and its menu also opens Activity log. Workspace and
runtime opens the inbox's right body sidebar, with the same grouped form styling
and background as Settings. Workspace and starting revision use the complete
Settings copy/link form row; visible values truncate while the full value is
copied. The revision links to its exact repository entity. Model, Thinking and
Execution mode use the same runtime rows as agent details, showing the
associated saved agent configuration (or retained session values when it has no
saved agent) and opening the exact same Edit agent form, including its avatar
and instructions fields. Save agent updates future defaults and applies runtime
choices to a connected session using the same harness. The existing session
keeps its captured instructions and workspace. Closing the sidebar preserves
unapplied preferences and your conversation draft. The composer supplies Send.
Beneath it, only the compact activity strip and explicit pending-request
recovery actions appear. Empty bodies use shadcn Empty with an icon;
session/history loading shows the owned skeleton. Resume uses the adapter's
explicit load capability. New session with saved context starts a fresh adapter
session and labels that distinction. Neither action sends a prompt
automatically. Resume failures appear as an error in the composer, with their
safe ACP code and retained draft/history. If the harness no longer has its saved
conversation, choose New session with saved context explicitly.

Request identities and drafts survive refresh and navigation. If another setup
save changes the configuration version, the editor retains your draft and shows
**Keep draft and use latest version**. Review the latest saved configuration
before choosing this action and saving again. Existing sessions keep their
original configuration after an agent is edited or deleted. After a lost
response, Check outcome queries the original request. An unknown outcome keeps
the request unresolved and the workspace available; review it before explicitly
continuing. The activity strip stays on one line; details and recovery actions
appear in a toast. **Review workspace** keeps the unresolved outcome in history
and opens the original session without resending the prompt. A confirmed missing
request instead offers **Submit retained request**, using its original identity.
Dismissing the toast keeps your pending request and draft, and the same notice
does not reappear on refresh or navigation. The same recovery action remains
available in **Session options**. Server restart never resubmits prompts.
Sanitized history publishes through AgentStore, and the published counter
distinguishes durable observations from activity still awaiting publication.

Workspace, Agent settings and Actions have the existing Settings section titles.
Review changes and Clean up workspace are grouped after the agent runtime rows.
Cleanup is a red row with a trash icon. Review opens the retained revision.
Explicit cleanup retains a JJ bookmark and refuses ignored, untracked or special
files. It does not merge or push agent changes.

## Enable execution

Start the LocalTrusted server with `JJ_STORE_ACP_EXECUTABLE` set to the absolute
path of a rebuilt `jj-acp` binary supporting control version 2. Keep the paired
custom `jj` beside the server, or select it with `JJ_STORE_AGENT_JJ_EXECUTABLE`.
Open **Settings → Agents → Runtimes** to check tooling on this device. Codex's
**Install** or **Update** action downloads ACP 1.13.1 and CLI 0.156.1 into
private jj-store tooling after confirmation. Discovery and launch use that
selection across service restarts, without depending on an npm cache path.
Existing global tools stay available. End connected sessions before changing
tooling. Node.js 22+ with npm is required; the page links to its installer when
needed. Cursor uses its vendor installation/update guide. Additional managed
installers and private Node provisioning are not yet qualified.

Ready describes tooling availability, separately from Online/Working presence
and provider login. Authenticate through the vendor CLI on the device; the app
does not store credentials or start a session after installation. Check again
refreshes status. A failed update keeps the previous selection. Lost responses
retain the original request for status checks; interrupted staging is not used
or replayed after restart. Privacy preparation may briefly show Preparing agent
sessions before launch is available.

For desktop resource staging, pass `--acp-executable /absolute/path/to/jj-acp`
to `node scripts/desktop.mjs stage`. The optional executable is bundled with the
paired JJ runtime. Without it, ordinary browsing remains available.

The legacy version-2 profile admits one warm process at a time, up to sixteen
queued commands, sixty-four retained sessions and sixty-four saved setups. The
version-3 reusable fleet admits four workers, sixteen queued turns per
participation and sixty-four queued turns in total. Setups and lifecycle
receipts are private device state outside repository checkouts. Repository
history remains subject to independent code-repository access; access to a
project or task home does not grant execution in linked code.

Add agent loads model, thinking and execution-mode choices from the installed
harness before your first chat. Discovery sends no prompt and creates no chat or
repository workspace. If discovery fails, Retry models keeps your editor draft.
Connected sessions provide their negotiated choices; saved preferences are
checked again when a chat connects. If Starting revision reports an index that
needs preparation, choose Rebuild index to prepare that exact
repository/operation and reload its choices. Your selected revision and typed
message remain retained during recovery.

You can draft messages while a session is offline. Resume or start a linked new
session explicitly before sending. Send uses the configured agent's negotiated
runtime choices and waits for the exact accepted configuration receipt and
confirmed mode readiness before submitting the message; failed configuration
retains your draft.

## Replies and reactions

Hover or focus a saved user/agent message to reply or react. Replies are
ordinary repository ThreadStore messages linked to that exact captured
AgentStore message; reactions use the original message's exact persisted target.
Tool rows use the same reply and reaction actions for their exact committed
invocation or output. Consecutive tools share one expandable, full-width section
with slim rows and inset icons. They remain in repository history after the
process ends or the service restarts. You can draft a reply before its original
message is published, then save once it is available.

Saving a reply or reaction does not run the agent. The next explicit Send reads
new or edited committed replies as follow-up context, including human discussion
in the session's linked primary repository thread. Replies added from another
screen or client are included. Each revision is delivered once, with its author,
parent relationship and original message/thread context. Edits become eligible
again. Delivery progress survives resume and service restart; queued prompts
reserve their selected replies. Deleted messages and tombstoned discussions are
omitted. Ordinary replies and reactions work while the agent is stopped or
disconnected; only explicit agent Send needs a running session. If feedback
cannot be read or exceeds the bounded context allowance, Send keeps your draft
and reports the failure. Unknown reply outcomes keep the original request for
Check reply status or Retry same reply; successful recovery clears the resolved
error. Reactions themselves are not treated as instructions.

The inbox uses date banners and bordered turn dividers like repository threads.
Its session avatars show connection presence, and the repository icon opens the
chat's repository at its retained operation. Drag the list boundary to resize
it, or focus the boundary and use Arrow keys, Home and End. Double-click resets
its width. The directory's View sessions and Refresh agents icons have tooltips.

Repository history uses the managed session's captured name and avatar when its
exact recorded session ID is associated with this installation. Existing
sessions retain that identity after the saved agent is edited or deleted;
unmatched captures keep the integration label. The row shows its last recorded
activity before the lifecycle state when available. Workspace choices use the
same captured avatar for an exact workspace association. Inbox repository icons
name the repository and open the retained workspace at its operation, including
before the first turn. These links and presentation controls never start an
agent or change repository history.

Message clocks use the first available recorded observation time. Older saved
events written before timestamp capture have no exact time to recover; their
group shows Time not recorded instead of repeating an unknown clock on each
message. Refresh and the last activity time do not supply historical timestamps.

Inbox reply/reaction writes support registered local folders and the qualified
native WorkStore-home profile. Native code repositories still require
collaboration replication qualification. The reply composer explains unavailable
writes and retains your draft. Reading existing feedback and provider-side
prompt context does not bypass these gates.

## Compute, defaults and captured context

Compute's Model picker searches local-model choices with estimated weight size,
machine fit, recommended choices and installed status. Fit reserves 30% of host
RAM and respects your draft memory limit; it does not measure GPU capacity or
guarantee inference will fit. Unknown hardware or imported sizes stay unknown.
Choosing a model only updates the draft; Download model and Start model remain
explicit actions.

Settings → Agents places Conversations and Preferences above Agent runtimes,
with Agent defaults last. Defaults use the same searchable harness/model picker
as Add/Edit agent, including session-advertised and discovered models. The
automatic-mention default seeds conversations which have no explicit preference.
Advanced in defaults and Add/Edit agent manages ordinary values, server
environment references, write-only device Keychain secrets and explicit unset
overrides. Agent values override global environment settings. Existing agents
retain harness defaults; newly created agents may inherit the compatible
application preferences. Running workers show pending configuration until
explicitly restarted. Model catalogs use the saved effective launch settings.

Settings → Compute manages one explicitly downloaded/imported GGUF model, native
runtime preparation, loading progress, memory limits and private machine
pairing. Browse beside Import GGUF selects a local weights file on the service
machine when its native file picker is available. Selection fills the draft;
Import model copies the weights. Manual absolute paths remain available. Start
uses recorded installed weights and does not download them. Local JJ Agent
provides an ACP model/tool loop through the bundled coding helper. Every coding
tool requires permission. Local agents may use this machine, a paired inference
host or an explicitly configured OpenAI compatible endpoint. No provider falls
back to cloud inference automatically.

Add each machine's invitation to the other and verify both identities. Enable
Share this machine on the inference host. Pairing grants inference only; files
and tools remain on the requesting machine. Disabling sharing or unpairing
closes admitted remote connections while local inference remains available.
Direct addresses use a persisted UDP port; optional HTTPS transport relays apply
after Stop compute and restart. Public discovery and default relays are
disabled.

The bundled Buzz-derived local harness is named JJ Agent and uses the jj-store
app logo. Its runtime shows Ready only when both the agent and coding tools are
installed.

Desktop distributions bundle ACP, the local agent, coding tools and compute
helper with checksums verified before launch. Prepare runtime verifies the
distribution's pinned artifact directory or explicitly downloads one. Weights
remain separate downloads/imports. Headless operators supply the stable 32-byte
hexadecimal machine identity through `JJ_STORE_COMPUTE_IDENTITY` in process
configuration; macOS uses the installation-scoped Keychain.

Keep awake is disabled initially and supported by macOS desktop. Actual agent
activity renews a bounded one-hour idle-sleep assertion. Polling and reconnects
do not extend it; stopping agents or exiting the app releases the assertion.

Expand the single context inspector above a user message to inspect its captured
instructions, page/repository identities, bounded file excerpts, project/task
descriptions, reply context and exact sanitized text/context blocks. Rich
previews use captured content and versions rather than fetching current data.
Prepared, queued, sent, failed and unknown delivery are distinct; historical
messages without snapshots are not reconstructed. Adapter-internal context is
outside this disclosure.

Authored text and bootstrap/repository/discussion context are retained
separately. Context uses bounded XML-like sections containing Markdown and
travels in the same ACP turn as a separate content block: an embedded resource
when the adapter advertises support, otherwise a separate ordered text block.
Sent context retains the exact sanitized wire text and XML; delivery updates
preserve the original message identity and recorded time. No context-only prompt
is sent.

Standing instructions, workspace orientation and bundled CLI guidance are sent
once per adapter session. Later explicit Sends include only changed page or
attachment context and newly committed or edited discussion replies. Restoring
an adapter retains delivery progress; an explicit fresh continuation includes
bootstrap and bounded authored history. Queued or uncertain requests are never
silently replayed.

The controller exports the actual bundled custom `jj` path as
`JJ_STORE_AGENT_JJ_EXECUTABLE`. Use `"$JJ_STORE_AGENT_JJ_EXECUTABLE" --help`;
`jj-store` is the server. Contextual chats have a verified `coordination`
checkout for their metadata home. Project and task commands use
`-R ./coordination`; code and review commands use the named
`repositories/<repository-id>` checkout. Do not initialize a project home inside
a code attachment. The context includes help and workflow examples; writes still
require the user's task scope, exact current versions and a stable request ID.
Query the appropriate command receipt after an uncertain outcome before deciding
whether to retry.

Open a tool row to see its captured file contents, edit diff, command output or
search matches. File contents use syntax highlighting when a recorded path
identifies the language. **Raw JSON** shows the complete sanitized tool record;
**Formatted** returns to its readable view. Missing edit preimages and retained
truncation/redaction notices stay visible. These controls display recorded data
and never read the current file or run another command.

Contextual chat headers show the selected agent's avatar and current ACP status.
An update connection failure appears as a warning inside the composer and clears
when history refresh succeeds. Your editable draft and explicit Send remain
available; exact command errors take precedence.

After a completed turn saves a repository snapshot, the panel above the composer
shows the edited files and line counts since the agent's starting revision.
Expand the list to see more paths, or choose **View changes** or a path to
inspect that saved comparison. The inbox uses the same panel. Binary or
unavailable text has unknown line counts, and previews are bounded to 100 files.
These controls read saved repository state and never send another prompt.
**Undo** is currently unavailable: the ordinary repository operation undo is not
a safe inverse of only this agent's edits.

## Conversation details and titles

Open **Session options → Conversation details** to inspect captured session IDs,
activity times, message/tool counts, title, summary, keywords, goals and
generation provenance. Recorded conversations expose the same details from their
options menu. Renaming changes the conversation title; the agent name and avatar
retain the captured identity. Overview and Metadata start closed in one Settings
form group; expanding or collapsing them retains unfinished edits. Long
conversation titles truncate in narrow rows and headers; hover for the full
title.

Choose a dedicated provider and model in **Settings → Agents → Conversation
metadata**. This setting starts unconfigured and is independent of coding-agent
models and thinking controls. Once configured, new conversations generate
metadata after their first completed, published exchange. Opening history,
changing settings and restarting never backfill old conversations.

**Refresh metadata** updates discovery fields and preserves a custom title.
**Regenerate title** explicitly adopts a new generated title. Later activity
marks existing metadata stale until you refresh it. Generation uses visible
authored messages, delivered comments and concise tool names; it excludes hidden
reasoning and tool bodies. A bounded helper performs a single tool-free model
request. Partial captures are identified beside their results.

If generation fails, your existing title and metadata remain. **Check retained
request** inspects or recovers the same request after an uncertain outcome;
**Publish retained result** publishes an already generated result without asking
the model again. Concurrent title edits are protected by exact revision checks.
Optional ACP title suggestions retain their captured provenance; competing agent
suggestions never select a title by arrival order.

Search loaded conversations or recorded captures by title, summary, keywords and
goals. Filtering only searches loaded rows; recorded results can load further
pages explicitly. Native repository search enriches the existing owner Thread
hit. Local indexed search retains its existing capability limit.

For an interrupted generation with no retained result, **Stop waiting for
generation** abandons local waiting so you can make a new explicit request. It
retains the original request and does not claim that the remote model was
cancelled. Unknown publication still requires receipt recovery.
