# Settings and maintenance

Settings separates device presentation preferences, installation operation and
repository-specific management. Read the selected context before changing a
setting: a device appearance preference does not change repository metadata, and
a repository policy change is not a cosmetic preference.

The visible Settings section reacts to its own events. Storage statistics,
search health, folder registrations and configuration do not repeatedly fetch
while unchanged. Feed recovery reconciles missed changes; unavailable feeds use
bounded fallback only while visible.

## Where to find settings

Creation dialogs follow the same Settings form layout. **Add repository → Add
Server Repository** opens Repository details with the name field. **Projects →
New project** opens Metadata home and Project details; select a writable home,
enter a title and optionally add a Markdown description. Labels and help sit
beside controls on wide screens and stack on narrow screens. Close/Done and the
create action share the bottom dialog footer. Unknown requests retain their
original identity and recovery controls.

For authenticated remote agent administration, select the server and use
**Agents → Manage server access**. An enrolled `manage-agents` operator can edit
definitions and messaging allowlists; use **Reload saved settings** to activate.
Access onboarding separately exposes identity enrollment/revocation and native
repository Write grants according to the current token's permissions. The form
retains drafts and exact pending intents across tabs, blocks connection changes
while unsettled, and pauses hidden reads. See
[agent sessions](agent-execution.md).

Open **Settings** from the profile menu for Profile, Custom emoji, Appearance,
Local service, Agents and Compute. Local service includes installation-wide
Protection and Storage configuration, workflow hosts, folder registration and
Add folder. **Local service → Workflows** configures execution-principal
admission and an optional local event/action host. Save retains an inactive
draft; Reload saved settings activates it without replacing listeners. Busy or
uncertain work defers replacement and preserves its journal. The normal form
focuses on enablement, name, labels and executors. Registered native workspace
links and each job's recorded repository/revision provide source context.
Advanced starts collapsed and offers optional path/polling overrides and owner
policy; Execution access separately offers explicit admission for the derived
local identity. No per-repo mapping or execution-principal input is needed. Use
the Form/JSON toggle at the right of the local-host header to switch editors.
Both retain one draft; JSON has syntax highlighting and includes named commands,
environment variables and secret references. Invalid JSON remains editable and
reports a toast. Legacy host documents remain accepted; Form edits convert their
execution settings on explicit Save while retaining custom paths and recovery.
Runtime status uses bounded checks while this tab is visible. Appearance
separates Theme and Preferences into subtabs; changes still save immediately on
this device. Link previews and Thread layout have their own cards under
Preferences. Launch and Storage group their read-only inventory by purpose. Use
Copy beside a configured path or endpoint to copy the complete value.

Global Settings links record the section and tab, for example
`#/settings?section=agents&tab=runtimes`. Reload restores that page, and a root
app restart resumes the last open Settings destination. An explicit link takes
precedence. Back to jj-store returns to your session entry and clears restart
resume; a fresh Settings link returns to Projects.

Use the **Repository settings** gear at the far right of a repository's tab row
for General, Local folders, Search, Storage, Sync and Advanced. Available
categories and actions depend on that repository's type, capabilities and your
permissions. Local folders lists only this repository's registration or links.
General identifies Local JJ or Server Hosted repositories.

Category and subtab selections are part of the repository URL. Both navigation
rows stay visible while the content scrolls. Back returns to the view you
entered from and restores its Details panel. Conversation and workspace
inspection panels stay hidden while Settings is open; returning retains their
selection and unsent drafts. Opening a shared settings link directly returns to
Overview at the retained operation instead.

Settings group status, editable fields and maintenance separately. Save/Cancel
and maintenance commands sit in their respective section footers. Search shows
Index status, Search options and Index maintenance; Sync separates status,
upstream fields and limits. Copyable identifiers use a single line with an
ellipsis: hovering reveals the full value, and Copy always copies it exactly.
The adjacent Open action navigates independently.

Both global and repository Settings use full-width cards, aligned rows and
section footers. Subtabs use icons and an underline; their navigation stays
visible while the content scrolls. Both record their selected destination in the
URL and support browser Back/Forward.

While settings first load, placeholders follow the selected page’s cards and
fields. Later refreshes keep existing values and unsaved drafts on screen.

## Saved and effective settings

Where settings expose both saved and effective values, saved configuration is
what a future activation will use; effective state describes the current
worker/service. Follow Apply or reload indicators. A successful configuration
save alone does not prove a worker restarted or a long-running operation
completed.

Server inventory includes values that are intentionally read-only. Storage
location, access transitions, launch configuration and backup execution are not
made editable merely because their current values are listed. Keep secret
references separate from secret values.

## Workspaces and capture

Workspace management links verified native workspaces or explicitly admitted JJ
folders to the device. It does not migrate or import a repository implicitly.
Capture settings expose the configured source profile, roots, worker state and
retained original import/control request identities. See [capture](capture.md)
for the difference between pause/resume and startup configuration.

Eligible repository settings expose **Repository storage** for an independent
local/native copy. Preflight and retained request status precede opening the new
destination; see [repository conversion](conversion.md). This does not change
the meaning of device folder registration or workspace links.

## Search and sync

Search can be disabled independently of repository authority. Preparing an index
is an explicit bounded action; inspect its result rather than assuming that a
spinner disappearing means a complete index. Sync exposes configured worker
state, finite waits, pause/resume and explicit reconciliation. A remote outage
does not require moving ordinary reads to the upstream.

Repository Advanced settings can prepare the selected graph index and show its
retained task status. Use **Cancel preparation** to stop that task and wait for
the confirmed outcome. Navigation may stop observing it; a canceled HTTP request
alone does not establish that the server worker stopped.

## Integrity and checkpoints

Storage integrity checks operate on a prepared exact target with finite budgets.
An incomplete report means verification did not finish within those limits; it
must not be interpreted as healthy. Retain the target identity and report when
investigating. A small bounded repository can have a complete report while a
larger installation reaches the time budget.

Non-destructive checkpoints retain recovery semantics and durable receipts. They
are not an export archive or a substitute for qualified backup/restore.
Destructive garbage collection, automatic promotion and multi-writer serving
remain disabled. Do not delete candidates, journals or storage files manually to
resolve a reported maintenance condition.

## Local Stacks workspace settings

For a registered ordinary JJ repository, open **Stacks → Workspace actions →
Workspace settings**. The Details table starts with the copyable Workspace ID,
followed by the active workspace name and target branch. The branch picker
searches available unambiguous heads. Save reviews the exact change; Cancel
leaves the saved configuration unchanged.

Deleting a workspace requires confirmation. It removes the saved workspace
configuration, preserving the checkout, branches, commits and unfinished native
JJ work. Settings are stored locally in `.jj/jj-store/stacks.toml` and do not
enter repository history. See
[local stack workspaces](../technical/local.md#local-stack-workspaces) for
retention and unknown-command outcomes.

## Agent runtimes

Open **Agents → Runtimes** to check Codex and Cursor tooling on this device.
**Check again** refreshes readiness independently of agent Online/Offline
status. Codex **Install/Update** confirms its maintained adapter/CLI versions
and stores them in a private jj-store directory. Node.js 22+ with npm is
required. Cursor uses its vendor setup/update guide. End connected sessions
before changing tooling. Installation never connects an agent; authenticate and
start explicitly.

Ready runtimes use Community's green success chip and omit Install. Guide links
and installation confirmation actions use the shared semantic button styling,
with inline icons. Runtime and capture tabs show only their own controls.

## Reusable agents and chat

Use these controls to open chats, manage recipient preferences and check agent
readiness.

### Open an agent chat

On the Agents screen, Start warms an agent in place; it does not open a chat.
New chat opens a repository-free draft. In a project or repository, use Chat
beside the top-bar Details toggle to open the right body sidebar. Opening never
sends a message. Attach registered repositories, select one or more saved agents
with @, write a message and press Send.

Automatically mention agents is opt-in and retains exact recipients for replies.

### Configure automatic mentions

The switch in the agent picker and Settings → Agents → Runtimes → Preferences
controls the same saved preference; changes save immediately. Turning it off
pauses automatic mentions and preserves your draft and selected agents. Remove a
tag or unpin it to stop including that agent.

### Check readiness when sending

ACP avatars show the current worker state across the app. Send automatically
prepares the selected agent's participation and observes its readiness without
periodic status reads.

Choosing another agent prepares that agent's own participation on the next Send.

## Local compute and agent defaults

Prepare compute resources separately from the provider and model defaults used
by your agents.

### Prepare models, resources and sharing

Compute settings separates Models, Resources and Sharing into tabs. Models
contains runtime preparation, GGUF downloads/imports, start/stop and
managed-weight removal; Resources contains hardware and memory limits; Sharing
contains trusted machine pairing and relay configuration. Drafts remain intact
across tabs, and Save compute settings is available from each tab.

### Choose agent defaults and a model provider

Agent settings add global defaults, automatic-mention preferences, macOS desktop
keep-awake, and collapsed Advanced environment configuration in both defaults
and Add/Edit agent.

Choose JJ Agent to reveal a separate LLM provider selector: This machine, JJ
Store shared compute, Anthropic, OpenAI, OpenAI-compatible, OpenRouter,
Databricks or Databricks v2. Shared compute uses the selected trusted host in
Compute → Sharing. The default option identifies the inherited provider, even
while another provider is selected.

### Discover models and apply provider changes

Provider endpoint and model discovery use the draft; Advanced holds credential
references. Discovery never saves or starts a chat.

Unknown saved models remain visible; Retry models and exact custom external IDs
are available. Changing provider clears incompatible choices. Save provider
changes, then explicitly restart the agent. Harness sections are collapsed on
initial load. See
[agent configuration and compute](agent-execution.md#compute-defaults-and-captured-context)
for precedence, credentials, pairing and context disclosure.

## Conversation appearance

Choose **Appearance → Conversation view → Document / Classic** for live and
recorded agents, Threads and thread replies. Document is the default when no
choice is saved. The device preference synchronizes across tabs and is
independent of Focus/Split placement. Both views retain selection comments,
reactions and reply navigation. Specialized task and review panels keep their
own presentation. Enter inserts a new line; Send or Cmd/Ctrl+Enter submits the
entire pending prompt.

### Choose a separate metadata model

Expand **Conversation metadata** in **Agents → Runtimes → Preferences** to
choose its provider and model. It defaults to unconfigured. Collapsing the
section retains unsaved edits. These settings apply to future metadata jobs and
never change a coding agent's model, start managed compute or trigger historical
backfill. Credential environment entries retain the existing installation
custody rules.

## ChatGPT accounts

On local macOS, **Continue with ChatGPT** in Agents settings opens the system
browser. Connect personal and team workspace registrations separately, then
select ChatGPT plan and its account for JJ Agent defaults or a saved agent. The
model picker shows that account's advertised models. Changing accounts clears
the previous model and thinking choices. Reconnect renews authorization; Sign
out stops dependent workers and clears local credentials. Manage usage opens
ChatGPT plan usage settings. See the
[connection guide](agent-execution.md#connect-a-chatgpt-subscription) for scope
and the separate live-account qualification gates.

## Numeric settings

Numeric inputs in repository **Search** and **Sync → Configuration**, global
**Local service → Limits**, **Agents → Configuration** and **Compute →
Resources** use the shared joined control. Fields with multiple units accept
convenient display units. Use the attached selector for milliseconds, seconds,
minutes, bytes, KiB, MiB or GiB where available. Switching units does not edit
the draft. The exact base value is shown beside converted entries, and invalid
fractions or out-of-range values cannot be saved. The icon-only **Reset**
appears after the units when the draft differs from its documented default. It
restores that default in your draft and then disappears. Search retains **Save
settings** and **Apply saved settings**; Sync, global service and capture
changes activate through **Reload saved settings** without a server restart.
Compute memory keeps its GB unit and accepts an empty value for automatic
allocation. Its slider covers common 1–64 GB limits in whole-GB increments;
typing a larger limit expands the slider while preserving the supported 1–1024
GB input range. Changing either control edits the draft until **Save compute
settings**. Reset is offered only where an authoritative default is available.

Capture options in Agents / Configuration offers sliders for readers (1–4),
discovery interval (1–30 seconds), tracked sources (1–4,096) and pending byte
capacity (1 byte–64 MiB). The numeric input and unit selector remain available
for exact entry. Slider endpoint labels follow the selected unit without
changing the draft or its bounds. Labels are compact, showing zero for very
small converted minima; exact entry keeps the original minimum. Sliders edit the
draft; Save configuration and Reload saved settings retain their separate roles.

## Profile and custom emoji

Object-backed installations offer the same Profile form to the local
administrator and enrolled authenticated users. Save a display name and Markdown
description, optionally add a private email, and choose an uploaded avatar,
credential-free HTTPS image URL or emoji avatar. Email is visible only to its
owner. With no explicit avatar, other users see the derived Gravatar image;
unavailable images fall back to initials. Changes use an observed profile
revision. Refresh keeps drafts; review a conflicting saved version before
submitting them again.

Avatar and custom emoji images may be up to 20 MiB. The server resizes large
images to fit 512 × 512 pixels, preserving aspect ratio and JPEG orientation.
Resized animations use their first frame; images already within the
retained-media limits keep their original bytes. Identity appears as a copyable
row inside Profile info. Profile and Custom emoji use the same cards and footer
actions as other Settings pages. The circular profile photo has a small Clear
button on its lower-right edge; when no avatar is set, that button opens the
picker with an Edit icon. Click the photo to choose a replacement. Clear edits
the draft; Save profile applies it.

Custom emoji accepts PNG, JPEG, GIF and WebP uploads with the same resizing as
profile photos. Stored images remain within 2 MiB, 512 × 512 pixels and 100
frames. The shortcode field supplies the surrounding colons; enter only the
name. The selected image appears beside Choose image. Selection stays local;
Save emoji uploads and resizes the image before saving its shortcode. Cancel
discards the local preview without uploading it. Active shortcodes are unique
across the installation. Everyone enrolled can use them; owners can edit or
remove their own entries. Removal prevents new selections while retained Thread
messages and reactions keep their original definitions and images. Unknown save
outcomes retain the original request: use Check save status or Retry original
save before another action. Filesystem-only installations show these settings as
unavailable.

Global Settings groups Profile, Appearance and Custom emoji under Personal;
Local service, Agents and Compute appear under App. The app version is at the
bottom of the sidebar, below Help. The top-right Application menu (vertical
kebab) contains **Reload saved settings**. It applies saved supported server
configuration while preserving drafts and the running server.
