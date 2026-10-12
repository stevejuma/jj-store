# Interactive agent sessions

## Agents on a remote server

Use **Add server** above Help in the sidebar to register an authenticated server
address and sign in with its access token. Select its chip to browse that
server's repositories, projects, profiles and allowed agents within this app.
Local remains the first-use default. Selection and destinations resume after
restart; credentials do not. Sign in again to unlock a restored connection.

The operator must enable `remote_agents` in that server's private configuration
and include your installation principal in each agent's allowlist. Repository
Write access alone does not grant execution. Your token must also have Write
access, under that same principal, to the existing native WorkStore home and
each attached repository. Select a home for a general conversation before Send.
Agent definitions, instructions, providers and credentials are operator-owned;
remote callers cannot install or configure runtimes.

Operators can select the remote server, open **Agents → Manage server access**,
and edit definitions and **Who can message this agent**. This editor requires a
separate `manage-agents` operator token, enrolled in the installation directory.
It shows saved and running configuration; a server restart activates changes. It
changes only the private `remote_agents` configuration, without installing
runtimes or restarting the host. Public directory profiles and avatars help
identify principals; they do not grant execution authority.

**Access onboarding** exposes token enrollment/revocation with
`manage-identities`, and explicit Write grants for native repositories where the
current token has Admin. The principal and token must match. Grant the existing
conversation home and each attached code repository separately. Operator tokens
have independent roles; use the appropriate sign-in for each operation. Unknown
saves and grants retain the original intent for **Check original outcome**.

Execution and its isolated native workspaces run on the selected remote host.
Activity streams back through the device gateway. Review and published history
remain governed by their repository permissions. Switching servers retains
unsent drafts and pauses hidden readers; it does not stop a remote job. An
unknown request outcome retains its original identity for recovery. Restart does
not replay prompts. Shared links use the remote server's canonical origin.

The initial remote profile supports trusted collaborators on Unix hosts with
native repositories already present. Filesystem repositories, automatic home
provisioning and Git transfer are outside this profile. It provides no hostile
user OS sandbox. Source/fixture checks and installed-runtime qualification are
recorded separately in M27.

For isolated gateway qualification, build the production binaries and UI, then
run
`python3 scripts/remote-agents-acceptance.py --server /absolute/path/to/jj-store --jj /absolute/path/to/jj --rustfs /absolute/path/to/jj-store-rustfs --web /absolute/path/to/web/dist`
from this repository. The harness provisions private disposable installations,
real RustFS volumes and synthetic credentials; it exercises the built browser,
authorized remote execution, publication, revocation, identity fencing and
restart recovery. It removes its own state and processes and does not restart a
development installation. An optional
`--privacy-cache /absolute/path/to/privacy-cache` copies that rule cache into
its disposable root. The deterministic ACP process proves integration, not
installed provider or production deployment qualification.

For installed Codex and operator-UI acceptance on the same machine, use
`python3 scripts/remote-agent-administration-acceptance.py --server /absolute/path/to/jj-store --jj /absolute/path/to/jj --rustfs /absolute/path/to/jj-store-rustfs --acp /absolute/path/to/jj-acp --web /absolute/path/to/web/dist`.
This provisions two disposable real RustFS instances, synthetic role tokens and
native repositories, exercises the cold built app, then sends a bounded
file-edit prompt to the already installed and authenticated Codex provider. It
does not install a provider or copy its credentials. The optional
`--privacy-cache` has the same meaning as above. Host/platform and desktop
delivery are separate qualification gates. Use
`--tooling-bin /absolute/path/to/prepared/bin` when the installed `codex-acp`
adapter is in private tooling rather than the ambient PATH. The harness reuses
that selection without installing or copying credentials.

## Local agent sessions

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

Add/Edit agent presents AI configuration in one Settings card with divided rows,
right-aligned controls and a separate model-retry footer. The other editor
fields retain their layout. New chat omits the redundant first-prompt context
banner; captured context still accompanies the prompt.

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
committed, visible replies as follow-up context, with their original message and
exact reply revisions. Deleted messages and tombstoned discussions are omitted.
If feedback cannot be read or exceeds the bounded context allowance, Send keeps
your draft and reports the failure. Unknown reply outcomes keep the original
request for Check reply status or Retry same reply; successful recovery clears
the resolved error. Reactions themselves are not treated as instructions.

Inbox reply/reaction writes support registered local folders and the qualified
native WorkStore-home profile. Native code repositories still require
collaboration replication qualification. The reply composer explains unavailable
writes and retains your draft. Reading existing feedback and provider-side
prompt context does not bypass these gates.

## Runtime implementation reference

The private tooling path and explicit runtime-settings design were reviewed
against Buzz Desktop revision `ef2aa1ae38fadcc0bc22b8bf6ed96b35933146be`,
notably `desktop/src-tauri/src/managed_agents/managed_node_paths.rs` and
`desktop/src-tauri/src/commands/agent_discovery.rs` (Apache-2.0). This
application maintains its own fixed catalog, typed commands and durable
selection; it has no Buzz runtime/build dependency. Existing Community avatars
and settings components retain their attribution in `docs/ui-provenance.json`.

## Connect a ChatGPT subscription

On local macOS, open Settings → Agents → Defaults and select **Continue with
ChatGPT** in ChatGPT accounts. Complete authorization in the system browser. The
application stores credentials in Keychain; the browser UI receives account
status only. You can add separate personal or team workspace registrations.

Choose JJ Agent as the harness and ChatGPT plan as the provider. Select the
connected account and a model advertised for that account, then save your
defaults or agent configuration. Per-agent accounts override the application
default. Changing the account clears its old model and thinking selections;
saved runtime changes apply on the next explicit start. Codex CLI is not
required for this provider. Configure Conversation metadata separately if it
should use the subscription too.

Reconnect an account when authorization or plan permission needs renewal. Sign
out stops dependent workers and clears local credentials. If remote revocation
could not be confirmed, disconnect the app in ChatGPT settings. Responses count
toward the eligible account's plan limits; Manage usage opens ChatGPT usage
settings. No API-key or other-provider fallback happens automatically.

The initial profile is same-machine macOS only. Source and controlled fixtures
are qualified separately from live subscription, Keychain and signed desktop
acceptance; those remaining gates are recorded in E-CHATGPT-001.
