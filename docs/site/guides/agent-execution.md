# Interactive agent sessions

The Agents views and repository, project and task actions can start Codex or
Cursor sessions on this device. Register an ordinary Git-backed JJ folder, or
link a native repository workspace, then choose its exact starting revision. The
app creates a new isolated JJ workspace. Dirty source files remain in the source
folder. Agents edit the isolated workspace, which stays available for review
after the session ends.

Open **Agents** in the main sidebar to manage agents independently of a
repository. Fixed-size tiles show avatar, name and model; presence stays on the
avatar and details contains the readable status. Click a card for its Community
details sidebar; choose Edit there or in the card menu to change its
configuration. Add agent uses Community's form, with image upload, HTTPS image
URL or colored emoji, an optional description, instructions, and a combined
harness/model picker. Images stay in private device state; uploads are limited
to 1 MiB. Existing emoji avatars remain supported.

**View sessions** opens an inbox with session chats on the left and the selected
conversation on the right. Empty lists stay visible until you explicitly start a
session. Session rows show the last activity time, with an exact date on hover;
older sessions without recorded timing omit that row until new activity.
**Activity log** opens the active session for that agent, or its session inbox
when no session is active. The conversation is the activity log. **Stop agent**
in the details sidebar ends that agent's connected session while preserving its
workspace and history. Chat drafts survive switching to Activity log and using
Back/Forward.

Named setups save an avatar, description, instructions, runner and model, effort
and mode preferences. Start opens a draft chat in the inbox. Choose the agent,
repository/workspace and starting revision in the centered setup form, then
Connect agent to create its isolated session. The command pickers use the
settings form styling. Your typed message stays in the composer for explicit
Send; agent, repository and starting revision are fixed after connection. Online
means a connected ACP session, Working means a running turn, Starting means
connection is in progress, and Offline means no live associated session. Unknown
means status could not be refreshed; cards and drafts remain available.
Installed adapters are shown separately. After connection, the app shows the
runner's advertised choices and applied preferences. Choose **Session options →
Workspace and runtime → Edit execution mode**, then Save agent before sending a
prompt. Instructions and project/task context accompany the first prompt.

The conversation scrolls above a bottom composer row, so growing drafts never
cover messages. The shared Scroll to latest/New messages control returns to
recent activity when you scroll back. It shows streamed text, tool activity,
plans and exact permission choices. Messages show their retained observation
time using the same clock-time metadata as recorded agent chats, with an
exact-date tooltip. Older untimed history shows Time unknown. Turn completion
appears as a system notice. During a connected turn, Working appears in the chat
and a compact agent activity strip below the composer. Follow-ups queue in the
warm session; Stop turn cancels the selected turn, and **Session options → End
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
continuing. Server restart never resubmits prompts. Sanitized history publishes
through AgentStore, and the published counter distinguishes durable observations
from activity still awaiting publication.

Workspace, Agent settings and Actions have the existing Settings section titles.
Review changes and Clean up workspace are grouped after the agent runtime rows.
Cleanup is a red row with a trash icon. Review opens the retained revision.
Explicit cleanup retains a JJ bookmark and refuses ignored, untracked or special
files. It does not merge or push agent changes.

## Enable execution

Start the LocalTrusted server with `JJ_STORE_ACP_EXECUTABLE` set to the absolute
path of a rebuilt `jj-acp` binary supporting control version 2. Keep the paired
custom `jj` beside the server, or select it with `JJ_STORE_AGENT_JJ_EXECUTABLE`.
Install and authenticate `codex-acp` or `cursor-agent` separately and make it
available on the server's PATH. The app does not install adapters or store their
credentials. Privacy-rule preparation may briefly show Preparing agent sessions
before launch is available.

For desktop resource staging, pass `--acp-executable /absolute/path/to/jj-acp`
to `node scripts/desktop.mjs stage`. The optional executable is bundled with the
paired JJ runtime. Without it, ordinary browsing remains available.

Execution currently admits one warm process at a time, up to sixteen queued
commands, sixty-four retained sessions and sixty-four saved setups. Setups and
lifecycle receipts are private device state outside repository checkouts.
Repository history remains subject to independent code-repository access; access
to a project or task home does not grant execution in linked code.

The agent editor can show models from the harness's latest retained negotiated
catalog when no session is online. It labels these remembered choices, retains
saved preferences and checks availability again when a new session connects.
Opening the editor does not start a harness. If Starting revision reports an
index that needs preparation, choose Rebuild index to prepare that exact
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
They remain in repository history after the process ends or the service
restarts. You can draft a reply before its original message is published, then
save once it is available.

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

Message clocks use the first available recorded observation time. Older saved
events written before timestamp capture have no exact time to recover; their
group shows Time not recorded instead of repeating an unknown clock on each
message. Refresh and the last activity time do not supply historical timestamps.

Inbox reply/reaction writes support registered local folders and the qualified
native WorkStore-home profile. Native code repositories still require
collaboration replication qualification. The reply composer explains unavailable
writes and retains your draft. Reading existing feedback and provider-side
prompt context does not bypass these gates.
