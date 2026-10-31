# Embedding conversation workflows

`jj-store-conversations` contains reusable headless conversation mechanics. It
coordinates explicit prompts through injected host ports and processes Control
V3 events with private prefix buffering, final host scanning and bounded
per-participant assistant/tool accumulation. The library imports no JJ service,
runtime, object store or transport.

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
or acknowledging the runtime. The stream primitives supply neither a durable
receipt nor an acknowledgement owner. A renderer subscription must not consume
raw runtime events. Undecidable prefix buffers stay memory-only and have no
serialization or debug representation. Close and disable must drain the owning
pipeline before releasing authority.

Message publication, prompt delivery and captured answers remain separate
outcomes. Persisted participant attribution does not resolve a device-local
executable or restore a provider session. Repository stores remain selected
through the [injectable store interfaces](architecture.md). Broader
workspace/store adapters and Notes integration are tracked by the staged M33
specification.

## Native capture before delivery

`capture::NativeCapture` combines the shared stream pipeline with an injected
`DurableCapture` sink. Construct one owner per native worker. Its `Binding`
retains opaque host scope, window, admission generation, original open request,
target and worker identities. These values grant no authority and must contain
no device paths or credentials. Hosts recheck current admission around the
native callback and drain admitted callbacks before releasing its backend.

Each `Input` includes the exact native worker, contiguous processing ordinal,
facts/event publication and validated frame. Facts consume an ordinal without
advancing the native event cursor. The pipeline applies injected stream privacy,
accumulates assistant/tool projections, then scans complete projected frames
again. Deferred prefix frames precede their triggering frame in one batch. Raw
prefixes remain only in memory. Input and canonical output are bounded to 2 MiB;
serialization stops at the byte limit.

The batch's schema-1 JSON sorts all object keys recursively, independent of
downstream `serde_json/preserve_order` feature unification. Its domain-separated
SHA-256 receipt binds sanitized frames, policy, native scope and exact progress.
The expected digest alone proves no durability. `persist` must retain exact
bytes/progress and return that identity only after its actual durability fence.
The host sink owns deduplication, durable request/publication receipts and
unknown-write recovery across owners and restarts.

No output returns before the matching receipt. Refusal, uncertainty, receipt
mismatch, invalid progress, privacy failure or a dropped admitted capture wait
fences the owner permanently. It never retries that write or changes writers. A
replacement owner cannot turn an unknown write into permission to retry; the
sink must recover its retained outcome. Runtime supervision still belongs to the
native host: this headless library cannot keep a cancelled future alive.

Deliver **all** returned frames in order, including a flushed prefix before a
terminal record. Native transport integration must preserve that batch; choosing
only the final frame would lose captured text. This shared port alone supplies
no filesystem journal, Notes runtime adapter, AgentStore/ThreadStore publication
or native downstream acceptance. Those remain separately qualified M33 tasks.

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
existing receipt policy. Journal errors retain original identities and cannot be
treated as permission to retry a side effect.

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

## Typed client and source package

`@lapismd/jj-conversations` exports built ESM and declarations. The private
jj-store application uses its source workspace through those public exports. The
publishable package has no UI, runtime, network or storage dependency and keeps
registry-only development metadata. Installation/publication remains a separate
operator action. Build with `pnpm client:build`; focused package tests use
`pnpm client:test`. An independent archive consumer verifies public runtime and
type exports without access to the source tree.

The package's public npm configuration exposes only this headless client; the
owning application remains private. Publication uses the package-scoped
`npm-publish-jj-conversations.yml` workflow. That workflow tests and packs this
package only. It does not rerun repository-wide heavy qualification. A protected
main-only `attest` run can retain a checksummed tarball and Sigstore provenance.
The predicate includes npm's GitHub event and exact numeric repository and owner
identities, and signing rejects foreign IDs.

After publication, `verify` checks the downloaded package size, SHA-256 and
integrity. A matching tarball with no provenance succeeds. Provenance is checked
when the registry package has it. A different tarball for the same version
fails, and an existing version is not uploaded again. `publish` finalizes the
package-scoped GitHub release only after those registry checks. This flow
publishes no Rust crate, native archive or application.

Construct `ConversationClient` for one app/vault scope with a typed transport
and lossless encoder. The client binds each request ID to immutable exact
encoded bytes in memory. Concurrent callers share one submission. A lost
response keeps an unknown operation; `refresh` invokes only the injected status
reader. Missing status never establishes non-delivery. A foreign receipt is
rejected without changing the retained identity or resubmitting.

The client has no durable storage. Restore interrupted work with
`restoreUnknown(requestId)` from an admitted native journal, then read status.
This needs no raw command body and cannot manufacture a new submission. Native
receipts remain authoritative. Scope, credentials, transport admission and
sanitization of any persisted records remain host-owned. No client error
includes raw command bodies or transport exception text.

A receipt response is separate from message publication, each recipient's
delivery and captured answers. The client retains the host's typed outcome and
full-width integers. Only an explicit host finality decision can retire an
entry; unknown work is never evicted for capacity. Drain active calls before
releasing the owning native scope.

Mention tokens keep exact case-sensitive installation setup IDs. Decoding does
not resolve a member key, executable or execution grant. Delivery projections
retain full-width host sequences; the existing UI adapter preserves its prior
display-only number conversion. The generic client imports no generated
application schema: hosts use their canonical generated wire types directly.

## Existing native privacy policy

Enable `jj-store-conversations`' `native-privacy` feature to reuse Agent
Redaction's transcript sanitizer directly. Construct the re-exported
`native_privacy::TranscriptSanitizer` using host-admitted workspace, home,
temporary and private cache paths, and add all device-resolved exact credentials
before processing starts. Keep those values private and retain current admission
around construction and each callback.

Pass the admitted sanitizer as an `Arc` and the owned durable sink to
`NativeCapture::with_native_privacy`. It selects the original transcript policy
identity, scans nested values and keys, retains undecided stream prefixes in
memory and rescans assembled projections before durability. Scanner refusal or
invalid sanitized output stops that capture owner before a write or display.
Unknown receipts keep the existing no-replay fence. The optional feature does
not choose paths, credentials, targets or persistence, initialize a repository,
or qualify a Notes/JJ backend. Default builds remain independent of the scanner.

## Selected AgentStore and ThreadStore capture

The optional `store-capture` feature supplies `store_capture::StoreCapture`, a
`DurableCapture` implementation using the selected owned store interfaces. It
does not link a concrete provider. Combine it with `native-privacy` when the
host uses the shared transcript policy; either feature may be selected
independently.

Open and explicitly bind your chosen stores to the admitted repository first.
Create the conversation Thread through its own authorized request. Construct
`ContextV1` with that repository, Thread, original native binding and exact
privacy policy ID, then pass the stores and your durable `Journal` to
`StoreCapture::new`. Supply the sink to `NativeCapture`. Construction requires
both stores to share their actual atomic publication capability and explicit
repository binding. It neither initializes a repository nor chooses a backend.

The sink derives a stable source identity from the repository and native
binding, and one request per processing ordinal. Policy, Thread and payload
changes do not rename that request. The host journal must reject changed intent
under the same key. Capture evidence retains the exact sanitized schema-1
envelope and native cursors; its AgentStore observation is ordered by ordinal.
The adapter rejects a session already assigned primarily to another Thread.

Implement the journal under an exclusive host-owned lease for one selected
backend/scope. `begin` must atomically retain the sanitized candidate before
provider reads or preparation and return `New` only to its exclusive claimant.
`prepared` must durably retain the exact returned request, publication identity,
native delta and accepted Thread binding before execute. `confirm` must retain
the original receipt before display or ACK. Use `EntryV1::encode` and `decode`
for checked schema-1 records bounded to 8 MiB, including JSON byte-array
expansion. These are private host recovery DTOs; never publish them as native
repository objects or treat them as repository authority. Retain only sanitized
capture; undecided prefixes and credentials stay outside the journal.

Every existing claim, including an incomplete one, takes the read-only recovery
path. `recover(ordinal)` reconstructs only the retained exact request and
invokes the provider's request-aware receipt replay. It never calls prepare or
execute. Missing preparation, not-known status, unavailable original receipts or
a provider without replay remain unknown. A journal confirmation alone cannot
authorize output: recovery must reauthorize through the provider and compare the
original selection, root, version, actor and provider receipt. Repeated exact
capture may return output only after that proof and durable confirmation.

Hosts still own current physical-vault/window admission, the journal's atomic
replacement/fsync and lease, native callback drain, backend configuration and
safe ordered delivery. Journal methods must not spawn detached writes. An
abandoned wait stops its `NativeCapture` owner even if publication committed.
The next owner may recover status, but cannot silently resubmit a write or
restart a prompt. The provider tests cover real Git-ref publication and native
manifest/WAL behavior with memory object storage. Notes' actual per-vault
backend, cold JJ conversation/restart, real RustFS and UI qualification retain
their separate gates.
