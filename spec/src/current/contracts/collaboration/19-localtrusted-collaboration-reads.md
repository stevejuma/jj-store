# LocalTrusted collaboration reads

The M9.4 service surfaces list AgentStore sessions from one exact selected root,
resolve one stable session against the current selected root, page one selected
transcript stream and traverse typed relations for session, tool, operation,
revision or checkpoint subjects. They also page the stable checkpoint catalog,
resolve its current selected version and read a requested immutable version
through that selection's supersession chain. Their owned responses identify the
repository, epoch, publication position, manifest revision, AgentStore root
digest and AgentStore state version. Each session row retains the complete
stable session ID and immutable session-snapshot object ID, then exposes only
bounded integration, lifecycle, parent, coverage and referenced object counts.
Session summaries derive optional first and last activity times only by matching
selected normalized event IDs to timestamps in their exact sanitized observation
batches; missing or over-bound evidence leaves timing absent rather than
inferred. For qualified Codex CLI/Desktop JSONL records whose observation
timestamp field is absent, timing may use the preserved RFC 3339 timestamp in
the matched sanitized native record. The exact profile/envelope and local
transcript source/trust remain required; explicit observation timestamps take
precedence. Invalid, missing and unsupported native timestamps remain unknown.
This applies to both native and companion-pinned list/detail reads, including
already published records, without changing immutable evidence, receipts,
normalization or capture recovery identities. Duration remains the captured
activity span, not elapsed time since ingestion or a guessed process lifetime.
Message presentation retains each matched observation's optional occurrence time
independently of the session activity span. Native and companion-pinned activity
and transcript paging use the same qualified decoder. A transcript chunk's
source identity/sequence, selected message event and exact content must match
before returning its time; current generation 1 is supported. Unknown or
over-bound evidence never borrows another message's timestamp. Read projection
changes neither immutable content/evidence nor publication and recovery
identity.

Tool activity exposes optional `duration_millis` only from ordered start and end
observations matched to the exact selected tool events. Qualified Codex
CLI/Desktop call and result records provide an observed invocation span even
when outcome remains unknown; timed Pending/Running and terminal hook events use
the same bounded projection. Both endpoints must belong to the same source and
normalizer. Missing or malformed endpoints, reversed times, repeated ends,
source changes and activity truncation leave duration absent. Timing never
changes tool status or borrows session/message/ingestion times. Native and
companion-pinned reads share this projection without rewriting stored objects.

Session detail also returns the exact immutable observation-batch, event-batch
and tool-snapshot IDs selected by that session; tool links retain their full
stable tool-call IDs. Selected transcript manifests expand only into stream
descriptors carrying the exact manifest/source identity, generation,
format/decoder/sanitizer versions, representation, byte and record counts and
gaps. The same detail resolves a presentation-ready, sequence-ordered activity
stream from the selected event batches. It preserves session-state, message,
tool, permission, observed-file and capture-gap variants; message and tool
payloads remain sanitized native content. Final tool state resolves the selected
tool snapshot, optional input/output, observed paths and exact code
associations. One response admits at most 4,096 source events, eight MiB of
resolved content and 64 code associations, and reports activity or association
truncation explicitly. Relation pages repeat the exact typed subject and return
ordered immutable checkpoint-version or association links; association links
also expose their verified stable association/session/tool identities, relation,
native before/after operation and commit anchors, change IDs, paths and creation
time. An unlinked subject returns an empty page. Transport DTOs do not expose
WAL objects or reinterpret stable entity identities as immutable object
identities.

Checkpoint summaries keep their complete stable checkpoint and immutable version
IDs, session, optional turn/scope, native before/after anchors, code relation,
evidence/trust state, coverage, capture status, execution outcome and bounded
reference counts. Detail adds exact selected session, transcript, event, tool
and association object links. The immutable version route walks at most 4,096
unique checkpoint versions from the current catalog selection and rejects a
cycle, wrong stable checkpoint or unrelated object. It does not expose arbitrary
reads from the selected AgentStore closure.

ReviewStore thread collection likewise enumerates the stable thread catalog from
one exact selected root. Each summary retains the stable thread and current
immutable snapshot IDs, exact anchor and event-page object IDs, workflow state,
three monotonic versions and bounded comment/event counts. Collection reads
resolve and return the validated typed anchor so a browser can select an exact
revision or path without issuing an unbounded detail request; the anchor's
repository must equal the pinned repository. Stable-thread detail resolves that
current catalog selection, its validated anchor, ordered selected comment
revisions and safe bodies, plus the complete bounded immutable event stream. The
combined visible comment bodies in one detail response are limited to eight MiB.
Comment and event DTOs retain exact object IDs and authenticated actors;
tombstone and moderation states remain explicit. The service never serializes a
raw `ReviewObjectV1` envelope or permits arbitrary closure-object reads. No
historical thread-version route exists because v1 thread snapshots do not carry
a validated predecessor chain.

Initial and resumed pages require current repository read permission. Page sizes
are limited to 1 through 256. An opaque signed continuation binds repository,
epoch, publication position, manifest revision, AgentStore root, AgentStore
state version, page size and the radix key after the last row. Transcript
continuations bind the same root pin plus stable session, immutable manifest,
source instance, source generation, byte ceiling and last returned sequence. The
relation continuation instead binds the complete typed subject plus the
root-local relation page and offset. The process-local cursor registry retains
the exact audited immutable AgentStore snapshot across later publications,
bounds retained snapshots and closure objects, reauthorizes every page and
resets after expiry, server restart or epoch replacement. Tampering, repository
reuse and query changes fail explicitly. Reads neither dispatch an agent nor
stage or publish repository state.

Review collection continuations use the separate signed `jsr1` domain and bind
repository, epoch, publication position, manifest revision, ReviewStore root,
ReviewStore state version, page size and the radix key after the last row. They
retain the exact audited ReviewStore closure within the same snapshot, object,
concurrency, lifetime and authorization bounds.

ReviewStore commands accept a versioned request ID plus one validated typed
create, reply, edit, tombstone, moderation, resolve or reopen command. The
service derives the authenticated principal and a repository-bound publication
ID; neither is caller-authored. Successful responses decode the
manifest-selected WAL receipt into exact accepted snapshot and ReviewStore
object identities. Identical retries and status after service reconstruction
return that durable receipt. An unavailable or timed-out apply remains
`outcome_unknown`; a missing status remains `not_known`, which never proves
rejection. Stale workflow and thread-version preconditions fail without
publication.

The native RPC exposes the same bounded ReviewStore page, detail, typed command
and durable status models used by HTTP. `jj review list/show` preserve full
stable identities and cursors, while create, reply, edit, tombstone, hide,
resolve and reopen require an explicit request ID and exact applicable workflow,
revision or thread-snapshot precondition. Thread creation accepts one complete
typed anchor document for the bound repository rather than inferring an anchor
from display text. Text and lossless JSON presentation share this contract.

A committed ReviewStore command sends only the existing bounded process-local
repository wake hint. An already idle subscription replays the selected WAL
record and reports `repository-changed` at the receipt's accepted position; the
hint contains neither collaboration content nor credentials. Polling and WAL
replay remain authoritative across lost, coalesced or lagged hints. This closes
the LocalTrusted CLI and live-hint portion of JST-COLLAB-004 without adding a
public listener or controller dispatch.
