# Local compute and agent configuration

JST-COLLAB-034 through JST-COLLAB-036 extend to M25. Installation-wide agent
defaults and per-agent environment overrides live outside repository checkouts.
Existing agents retain their harness defaults; newly authored agents may inherit
compatible default preferences. Environment precedence is application defaults
then explicit agent overrides; explicit unset removes an inherited variable.
Controller-owned launch names are protected. A narrow inherited process
environment excludes server credentials. Provider configuration and model
catalogs use the same effective launch settings.

Configuration writes use independent conditional revisions and durable request
identities. Changes apply on the next worker start, never interrupting a turn.
Browser persistence, diagnostics and repository publication contain no
credentials. Ordinary environment values are persisted, while sensitive values
use external environment references or OS credential references. Explicit
desktop secret entry is stored in the device credential store; only opaque
references enter the journal. The memory-only browser credential boundary
remains in force.

M25 adds a managed local inference runtime and a separate ACP local agent with
owned coding tools. Compute may be shared only with explicitly paired trusted
machine identities. Pairing grants inference access, never repository or tool
access. Public discovery and automatic cloud fallback remain disabled. Runtime
artifacts are version/ABI/checksum pinned; model downloads and starts are
explicit. One loaded model and bounded concurrency/resource use are initially
supported. Full desktop and server distributions bundle internal helpers; model
weights remain separate explicit downloads or imports. The bundled Buzz-derived
local harness is presented as JJ Agent. Runtime status requires both its fixed
companion executable and the owned coding-tool helper; its saved `local` runner
identity is retained without journal migration.

Each new outgoing message retains its exact sanitized application context,
separate from the human text, in a versioned structured payload. Delivery status
uses existing request/start fences. Rich historical previews use captured
content and pinned identities, never current page state. Canonical publication
preserves these snapshots through the existing sanitized AgentStore payload and
receipt authority. Legacy messages without snapshots are not reconstructed.
Adapter-internal context is outside this disclosure.

Resolved credentials are registered as exact in-memory redactions before
capture. Credential values require at least eight bytes; shorter values are
refused rather than corrupting structural identities through substring
redaction. The scanner continues to sanitize before any journal/publication
write.

## Provider selection

JST-COLLAB-034 through JST-COLLAB-036 support Anthropic, OpenAI, compatible
endpoints, OpenRouter, Databricks and Databricks v2 through the bundled JJ
Agent. Local and paired compute retain their authority and explicit lifecycle.
The paired choice is presented as JJ Store shared compute and uses the
installation's selected trusted host. Existing `openai` configurations remain
compatible-endpoint configurations; `openai-api` identifies first-party OpenAI.
Provider-specific model, effort and endpoint inheritance requires matching
provider/endpoint context. Catalog discovery uses the same resolved launch
configuration, performs no prompt, tool execution or interactive sign-in, and
reports failures without invented models. Managed local/shared catalog previews
use only an already-running compute endpoint. Discovery never starts compute,
creates its identity or opens an interactive credential request; unavailable
compute remains an explicit unavailable catalog result. Provider changes take
effect only on explicit restart, never by applying a foreign model to a running
session. Credentials retain their existing custody/redaction.

The additive [ChatGPT plan provider](41-chatgpt-plan-provider.md) owns its
account-scoped model inheritance and renewable credential lifecycle.
