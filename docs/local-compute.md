# Local compute and agent configuration

The installation owns agent defaults, environment references, model storage and
paired-machine inference. Repository metadata remains independent of these
runtimes. The maintained Buzz-derived agent, coding tools and compute
implementation come from Lapis AI. Its ACP helper uses Control V3; jj-store owns
admission, credentials, settings and durable history.

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

In Add/Edit agent or Agent defaults, choose JJ Agent, then LLM provider. Choices
include This machine, JJ Store shared compute, Anthropic, OpenAI,
OpenAI-compatible, OpenRouter, Databricks and Databricks v2. Shared compute uses
the trusted host selected in Compute → Sharing. The same bordered Settings rows
contain the provider, endpoint, searchable model and execution mode.

External providers use credential references in Advanced. The form shows the
provider's environment variable name. Databricks also accepts its existing agent
sign-in cache; discovery never starts sign-in. No new OAuth flow is provided.
Model discovery uses the unsaved provider/endpoint/environment configuration and
transient secret inputs without saving them, starting a chat or sending a
prompt. Empty, unavailable and authentication-required catalogs retain the saved
choice; Retry models repeats discovery. External providers also permit an exact
custom model ID. Changing provider or endpoint clears incompatible model/effort
choices. Thinking appears only when advertised. No provider falls back to cloud
inference automatically. Existing saved `openai` means a compatible endpoint;
new first-party OpenAI configurations use `openai-api`.

## Pair trusted machines

Start/prepare compute on each machine to obtain its invitation and identity. Add
each invitation on the other machine and verify the identity. Enable Share this
machine on the inference host. Pairing grants inference access only; repository
files and tool execution remain on the requesting machine. Select a paired
Inference host and choose JJ Store shared compute in the agent configuration.

Direct addresses use a persisted UDP port. Optional HTTPS transport relays are
explicit; changing them takes effect after Stop compute and the next start.
Public discovery and default relays are disabled. Disable sharing or Unpair to
close admitted remote connections without unloading local inference.

Headless operators provide a stable 32-byte machine secret as hexadecimal in
`JJ_STORE_COMPUTE_IDENTITY` through their process/service environment. It must
not be placed in a checkout, URL, command argument or transcript. macOS uses an
installation-scoped Keychain identity.

## Bundle the installation

Keep the `lapis-ai` checkout beside `jj-store`. `pnpm ai:prepare` builds its
current source and stages a verified local bundle. Both Rust workspaces use
versioned Cargo path dependencies on that checkout. A future release can replace
the path selectors with crates.io dependencies; no library code is copied here.

Native privacy also consumes the sibling `agent-redaction` Rust crate. Keep that
checkout available during source development. CI materializes independently
pinned Lapis AI and Agent Redaction sources before Cargo resolution; it refuses
to replace an existing checkout. This changes no runtime launch or credential
policy and performs no package publication.

`pnpm desktop:prepare` builds the app and stages `lapis-acp`, `lapis-agent`,
`lapis-agent-tools` and `lapis-compute` with licenses and the upstream TOML
manifest. Pass `--ai-bundle /absolute/path/to/bundle` to select an already built
bundle, or `--native-runtime-dir /absolute/path/to/pinned-runtime` to embed the
native ABI assets. Model weights remain explicit downloads/imports.

A headless service can select the same bundle with `JJ_STORE_AI_BUNDLE_DIR`.
Existing explicit executable overrides remain available for trusted native
configuration. All execution and discovery use Control V3. The native verifier
rejects missing, modified, symlinked, wrong-version or wrong-target resources
before launch. Existing identity, pairings, settings, model indexes and caches
remain in their original installation directories.

For UI development, `pnpm dev:ui` reuses prepared binaries. After native edits,
`pnpm dev:backend:build` rebuilds the local AI helpers and backend; restart with
`pnpm dev:backend:restart`. None of these commands resets installation state.
Source/fixture checks remain separate from native model and platform acceptance.

## Inspect message context

New outgoing messages retain the sanitized application context at dispatch.
Expand Sent context to inspect rich captured context or the raw payload.
Delivery is distinguished as prepared, queued, sent, failed or unknown.
Historical context uses its captured content and pinned identities. Old messages
without snapshots are not reconstructed, and adapter-internal context is outside
this disclosure.

The shared `jj-store-conversations` library handles in-memory Control V3 privacy
framing and per-participant assistant/tool projections. The server supplies the
same admitted native sanitizer and retains bounded payload construction, private
journal ordering, publication and runtime ownership. This extraction does not
change credentials, process admission, provider acknowledgement or capture
policy IDs. See `docs/site/technical/conversations.md` for host embedding.
