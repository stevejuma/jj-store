# Local compute and agent configuration

The installation owns agent defaults, environment references, model storage and
paired-machine inference. Repository metadata remains independent of these
runtimes. The local agent is a maintained Buzz agent fork speaking ACP; `jj-acp`
remains a separately supervised, bundled process.

## Configure an agent

Settings → Agents contains Conversations, Preferences and Agent defaults above
Agent runtimes. Defaults apply to new agents that inherit them. Existing agents
retain their harness preferences. Advanced environment rows merge global values
with agent overrides; Unset removes an inherited value. Credentials use device
Keychain entries on macOS or operator environment references on headless hosts.
Literal credentials and URLs containing credentials are refused. Secrets are
write-only, at least eight bytes, and redacted by exact value before transcript
capture. Unsaved secret drafts remain in memory only.

A running worker keeps its launch configuration. Save changes, then use Restart
when the agent reports pending configuration. Model, effort and execution mode
choices come from the harness. Local agents advertise the permission-gated Ask
mode. Keep awake is macOS desktop-only, disabled initially, and expires after an
hour without actual agent activity. Status polling does not renew it.

## Run a local model

Settings → Compute shows hardware and managed models. Prepare runtime verifies a
compatible bundled runtime or explicitly downloads one when none is bundled.
Choose a GGUF model, Download, then Start. Import copies an absolute GGUF file
path from the server machine into managed storage. Remove weights deletes
managed files while compute is stopped. Start uses the installed-path receipt
and never downloads weights. One model, an 8,192-token context and two
concurrent inference requests are supported initially. Managed agents default to
an 8,192-token context budget and a 2,048-token response budget; Advanced
environment rows can override these limits.

In Add/Edit agent, choose Local agent. Advanced selects This machine, Paired
compute or an explicit OpenAI compatible endpoint. Remote endpoints need their
credential environment reference in Advanced. No provider falls back to cloud
inference automatically.

## Pair trusted machines

Start/prepare compute on each machine to obtain its invitation and identity. Add
each invitation on the other machine and verify the identity. Enable Share this
machine on the inference host. Pairing grants inference access only; repository
files and tool execution remain on the requesting machine. Select a paired
Inference host and use Paired compute in the agent configuration.

Direct addresses use a persisted UDP port. Optional HTTPS transport relays are
explicit; changing them takes effect after Stop compute and the next start.
Public discovery and default relays are disabled. Disable sharing or Unpair to
close admitted remote connections without unloading local inference.

Headless operators provide a stable 32-byte machine secret as hexadecimal in
`JJ_STORE_COMPUTE_IDENTITY` through their process/service environment. It must
not be placed in a checkout, URL, command argument or transcript. macOS uses an
installation-scoped Keychain identity.

## Bundle the installation

Build `runtime/local-agent` and `runtime/compute` with their committed
lockfiles. Stage with
`pnpm desktop:prepare -- --acp-executable /absolute/path/jj-acp`. The ACP build
must include this change's trusted tool declarations. Optional
`--native-runtime-dir /absolute/path/pinned-runtime` embeds the version/ABI
manifest and native libraries; weights remain separate.

For a headless resource directory, use
`node scripts/desktop.mjs stage --destination /absolute/output --acp-executable /absolute/path/jj-acp`
after the binaries and web assets are built. The staged directory includes the
custom `jj`, server, storage helper, ACP, local agent, coding helper, compute
helper, web assets and source/checksum notices. Configure the server with
explicit paths from that directory rather than installing helpers globally.
Source/fixture checks are separate from real local model, transport and platform
distribution acceptance.

## Inspect message context

New outgoing messages retain the sanitized application context at dispatch.
Expand Sent context to inspect rich captured context or the raw payload.
Delivery is distinguished as prepared, queued, sent, failed or unknown.
Historical context uses its captured content and pinned identities. Old messages
without snapshots are not reconstructed, and adapter-internal context is outside
this disclosure.
