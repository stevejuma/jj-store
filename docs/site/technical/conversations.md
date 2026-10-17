# Embedding conversation workflows

`jj-store-conversations` contains reusable headless conversation mechanics.
It coordinates explicit prompts through injected host ports and processes
Control V3 events with private prefix buffering, final host scanning and bounded
per-participant assistant/tool accumulation.
The library imports no JJ service, runtime, object store or transport.

Implement `privacy::Sanitizer` using an admitted native privacy policy. The
capability must perform prefix-safe replacement and scan complete output frames,
including keys and nested tool content. Return fixed safe errors when privacy is
unavailable. jj-store adapts its existing bounded `TranscriptSanitizer`; its
policy IDs, scanner versions and sanitized payload limits remain unchanged.

A host keeps one raw event owner. Pass raw frames to `StreamPrivacy` for one
participation or `control_v3::Stream` for several isolated participations. All
emitted frames pass the injected scanner. Apply `Stream::observe` to the safe
frames to accumulate assistant text and merge tool snapshots. Preserve frame
ordering when a boundary emits a buffered prefix before a terminal frame.

Durably journal sanitized frames and their exact progress before displaying them
or acknowledging the runtime. The stream library supplies neither a durable
receipt nor an acknowledgement owner. A renderer subscription must not consume
raw runtime events. Undecidable prefix buffers stay memory-only and have no
serialization or debug representation. Close and disable must drain the owning
pipeline before releasing authority.

Message publication, prompt delivery and captured answers remain separate
outcomes. Persisted participant attribution does not resolve a device-local
executable or restore a provider session. Repository stores remain selected
through the [injectable store interfaces](architecture.md). The remaining typed client, broader workspace/store adapters and Notes
integration are tracked by the staged M33 specification.

## Durable explicit turns

`coordination::Coordinator` accepts independent admission, context, journal and
runtime capabilities. `Types` supplies the host's existing parent identity,
commands, frames, outcomes and fixed safe error type. Ports can borrow an owning
host lease; no server or asynchronous runtime dependency is required.

The admission port rechecks current authority and bounds the complete recipient
set. `Journal::begin` atomically claims a stable explicit intent or verifies an
exclusive admitted host lease. It binds the original actor, request content and
repository scope. Existing or interrupted intents return retained outcomes;
constructing another coordinator cannot authorize replay.

Context preparation retains exact private delivery intent before returning each
frame. All recipients prepare before `Journal::prepared` establishes the durable
dispatch fence. If preparation fails, the coordinator releases prior exact
reservations and sends no prompts. The host settles the parent error using its
existing receipt policy. Journal errors retain original identities and cannot
be treated as permission to retry a side effect.

The runtime port must distinguish queue acceptance, proven not sent and unknown
delivery. Queue acceptance does not prove provider execution. Only proven
not-sent commands release their reservation. Unknown commands retain their
original request and require explicit status/recovery; they never replay.
Captured answers and message publication have independent receipts.

The JJ adapter retains its existing home, workspace and ThreadStore preparation
before constructing the coordinator. An opaque scoped lease keeps the owning
supervisor's conversation mutex held through preparation and dispatch. Original
child request derivation, prepared/accepted states, comment publication,
deferred file queues and fixed failure diagnostics remain unchanged. The host
continues publishing captured answers through its selected AgentStore and
ThreadStore. Independent downstream adapters provide their own admitted
workspace/store preparation and durable journal instead of importing the server.
