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
weights remain separate explicit downloads or imports.

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
