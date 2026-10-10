# Settings and maintenance

Settings separates device presentation preferences, installation operation and
repository-specific management. Read the selected context before changing a
setting: a device appearance preference does not change repository metadata, and
a repository policy change is not a cosmetic preference.

The visible Settings section reacts to its own events. Storage statistics,
search health, folder registrations and configuration do not repeatedly fetch
while unchanged. Feed recovery reconciles missed changes; unavailable feeds use
bounded fallback only while visible.

## Saved and effective settings

Where settings expose both saved and effective values, saved configuration is
what a future activation will use; effective state describes the current
worker/service. Follow Apply or restart indicators. A successful configuration
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
Settings**. The Details table starts with the copyable Workspace ID, followed by
the active workspace name and target branch. The branch picker searches
available unambiguous heads. Save reviews the exact change; Cancel leaves the
saved configuration unchanged.

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

On the Agents screen, Start warms an agent in place; it does not open a chat.
New chat opens a repository-free draft. In a project or repository, use Chat
beside the top-bar Details toggle to open the right body sidebar. Opening never
sends a message. Attach registered repositories, select one or more saved agents
with @, write a message and press Send. Automatically mention agents is opt-in
and retains exact recipients for replies. The switch in the agent picker and
Settings → Agents controls the same saved preference; changes save immediately.
Turning it off pauses automatic mentions and preserves your draft and selected
agents. Remove a tag or unpin it to stop including that agent. ACP avatars show
the current worker state across the app.

## Local compute and agent defaults

Compute settings separates Models, Resources and Sharing into tabs. Models
contains runtime preparation, GGUF downloads/imports, start/stop and
managed-weight removal; Resources contains hardware and memory limits; Sharing
contains trusted machine pairing and relay configuration. Drafts remain intact
across tabs, and Save compute settings is available from each tab. Agent
settings add global defaults, automatic-mention preferences, macOS desktop
keep-awake, and collapsed Advanced environment configuration in both defaults
and Add/Edit agent. Choose JJ Agent to reveal a separate LLM provider selector:
This machine, JJ Store shared compute, Anthropic, OpenAI, OpenAI-compatible,
OpenRouter, Databricks or Databricks v2. Shared compute uses the selected
trusted host in Compute → Sharing. The default option identifies the inherited
provider, even while another provider is selected. Provider endpoint and model
discovery use the draft; Advanced holds credential references. Discovery never
saves or starts a chat. Unknown saved models remain visible; Retry models and
exact custom external IDs are available. Changing provider clears incompatible
choices. Save provider changes, then explicitly restart the agent. Harness
sections are collapsed on initial load. See
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

Conversation metadata has its own provider/model under **Agents**. It defaults
to unconfigured. These settings apply to future metadata jobs and never change a
coding agent's model, start managed compute or trigger historical backfill.
Credential environment entries retain the existing installation custody rules.

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
