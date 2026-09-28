# Agent activity and repository reviews

AgentStore and ReviewStore are first-party repository capabilities. They share
the existing object envelopes, WAL, receipts and manifest publication service,
but retain independent models, authorization and state versions. They are not
upstream JJ traits, Tantivy indexes, Nostr projections or agent executors.

The first release is LocalTrusted. It captures Codex, Cursor and OpenCode
through qualified integrations, stores sanitized evidence before normalized
views and supports exact discussions on immutable code, diff and agent
artifacts. Review task context is explicit and version-bound; replaying or
reading a comment never dispatches an agent.

## Requirements

| ID             | Requirement                                                                                                                                                                                                     |
| -------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| JST-COLLAB-001 | The repository facade MUST expose independent root-pinned AgentStore and ReviewStore services backed only by the existing immutable object, WAL, receipt and manifest publication authority.                    |
| JST-COLLAB-002 | Capture MUST preserve stable session/turn/tool/checkpoint identities, sanitized native evidence, deterministic normalized activity and explicit evidence-backed JJ associations for Codex, Cursor and OpenCode. |
| JST-COLLAB-003 | Review threads MUST target exact immutable artifact versions and validated UTF-8 byte ranges, with stale-safe replies, edits, tombstones, resolve/reopen and conservative explicit re-anchoring.                |
| JST-COLLAB-004 | LocalTrusted CLI and HTTP APIs MUST provide bounded root-pinned reads, typed writes, pagination and live hints without bearer credentials, public listeners or implicit controller dispatch.                    |
| JST-COLLAB-005 | The application MUST reuse Community-owned thread/comment/agent presentation and Design Core diff extension points, with accessible interactions, virtualization and pinned-code versus live-activity state.    |
| JST-COLLAB-006 | Sanitization MUST occur before every durable capture copy through the qualified in-process redaction library; credentials, unrelated sessions and unsafe rendered content MUST fail closed.                     |
| JST-COLLAB-007 | Cold recovery and native copy MUST verify code, agent and review closures directly; search and other projections remain rebuildable, and unsupported remote/export/backup behavior stays deferred.              |

## Initial delivery boundaries

The active sequence is publication foundation, AgentStore model and capture,
ReviewStore exact anchors, LocalTrusted API/CLI integrations, shared UI, then
cold recovery and real-harness qualification. Codex CLI 0.156.1, Cursor 3.22.7
and OpenCode 1.18.30 are the initial compatibility surfaces. Deterministic
sanitized contract fixtures exist for all three; an executed capability matrix
and real-harness smoke tests remain required before support is claimed.

Authenticated/shared serving, remote replication, backup/restore, Git or Nostr
export, controller job dispatch, arbitrary cross-repository targets, CRDT/edit
identity, advanced semantic re-anchoring, other harnesses and visual baselines
are deferred. Tantivy may index committed collaboration records later but never
becomes their authority.

## AgentStore object foundation

`jj-agent` owns distinct fixed-width stable IDs for sessions, attempts, turns,
messages, blocks, tool calls, terminals, checkpoints, associations,
observations, events, devices and capture sources. Content-addressed
`AgentObjectId` values remain separate from those durable entity IDs. Native JJ
operation, commit and change IDs retain their existing owned types and are not
reinterpreted as agent identities.

`AgentRootV1` selects independent persistent radix catalogs for sessions,
aliases, checkpoints, associations, relations, sources and projections. Empty
catalogs are represented by absent page roots in one canonical empty root.
Catalog pages are bounded immutable objects with canonical digest keys and typed
strong references. Every staged root and page validates its complete
repository-bound dependency closure before it can be selected; missing objects,
semantic kind mismatches, unknown fields and noncanonical bytes fail closed.
Only the publication coordinator can turn staged objects into visible state.

Sanitized observation batches and normalized event batches are distinct
immutable objects. Observation batches record integration, source instance,
source order, source kind/trust, representation and sanitizer version; their
payload is validated canonical JSON or UTF-8 text rather than an arbitrary
untyped JSON value. Event batches reference their exact evidence batch and use
stable event/entity IDs. Normalization therefore cannot replace or rename the
accepted source evidence.

Session snapshots select exact evidence, event, transcript and tool object
versions. Tool snapshots keep execution state separate from capture evidence and
reference typed sanitized input/output content. Transcript manifests keep each
source generation separate and root an ordered immutable chunk-page closure
sufficient for direct reconstruction; they have no indispensable
previous-manifest link. Checkpoint discovery remains in its root-level catalog,
so a session snapshot has no mutable or circular checkpoint reference.

Checkpoint versions pin exact session, transcript, event, tool and association
objects plus complete before/after native operation and commit anchor sets. They
distinguish changed, verified no-change and not-observed code relations; a
failed tool can still have changed code. Completeness, capture status and
execution outcome remain independent. Superseded checkpoints are immutable
strong edges and cannot point back from a session snapshot.

Code associations are many-to-many assertions with stable association, session,
optional checkpoint/turn/tool/scope identities, full native IDs, optional JJ
change IDs, repository-relative paths, evidence method, trust, coverage and
exclusivity. Correction versions must name a prior immutable association and a
reason. File claims reject absolute or traversing paths, and manual assertions
cannot masquerade as stronger evidence. Stable IDs and JJ change IDs are lookup
facts; exact operation/commit IDs and immutable evidence objects are retention
edges.

An AgentStore read is pinned to one selected `AgentRootV1`. Stable session,
checkpoint and association keys resolve through bounded persistent radix pages;
immutable object reads succeed only for references in that root's audited
transitive closure. A staged but unselected object is not readable through the
pinned facade. Catalog leaves contain at most 32 selections, branch updates copy
only the affected path, and prior roots remain readable.

Typed agent deltas carry expected and replacement selections for all seven
catalogs. They reject duplicate edits, semantic kind substitution and no-op
edits. A disjoint edit can reapply to a newer agent root after a CAS race, while
a changed expected selection is stale. The stable request intent covers the
original delta rather than generated page/root IDs, so receipt replay remains
valid after later agent publications.

Alias objects bind an integration, source instance, native entity kind and
native ID to a matching stable session, turn, message, tool, checkpoint or
association identity. These lookup facts are not strong object references.
Relation subjects are typed session, tool, operation, revision or checkpoint
identities. Their catalog selections are immutable linked pages containing only
checkpoint and association object references; each page repeats its subject so a
cursor from another relation fails closed.

Source-state objects retain the integration/source identity, generation,
versioned parser, optional native session key, source-specific accepted
position, strictly ordered non-overlapping capture-batch receipts and coverage.
Projection catalogs retain one normalizer version and activation time plus exact
selected session, normalized-event and source-state object versions. Replaying a
parser therefore publishes new selected objects rather than mutating prior
facts.

Catalog enumeration is radix ordered and root pinned, returns at most 256
selections and resumes after a stable catalog key. Relation traversal uses a
root-local page/offset cursor and rejects mismatched subjects. Transcript reads
select one source generation from an immutable manifest, verify strictly
increasing chunk sequences across linked pages and enforce both a record count
and an eight MiB caller-selected byte ceiling. Session/checkpoint enumeration,
relation traversal and transcript streaming do not depend on Tantivy.

## LocalTrusted collaboration reads

The first M9.4 service surfaces list AgentStore sessions from one exact selected
root, resolve one stable session against the current selected root, page one
selected transcript stream and traverse typed relations for session, tool,
operation, revision or checkpoint subjects. Their owned responses identify the
repository, epoch, publication position, manifest revision, AgentStore root
digest and AgentStore state version. Each row retains the complete stable
session ID and immutable session-snapshot object ID, then exposes only bounded
integration, lifecycle, parent, coverage and referenced object counts. Detail
also returns the exact immutable observation-batch, event-batch and
tool-snapshot IDs selected by that session; tool links retain their full stable
tool-call IDs. Selected transcript manifests expand only into stream descriptors
carrying the exact manifest/source identity, generation,
format/decoder/sanitizer versions, representation, byte and record counts and
gaps. Relation pages repeat the exact typed subject and return ordered immutable
checkpoint-version or association links; an unlinked subject returns an empty
page. Transport DTOs do not expose WAL objects or reinterpret stable entity
identities as immutable object identities.

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

Checkpoint detail reads, ReviewStore reads and typed writes, user-facing CLI
commands, durable receipts and live collaboration hints remain later M9.4
slices. Session, transcript and relation reads alone do not complete
JST-COLLAB-004.

## Capture sanitization boundary

The capture boundary uses MongoDB Kingfisher 2.7.0 in-process at source revision
`63dccba231b6b01990c84319d25f92f49428a431` under Apache-2.0. Scanner default
features are disabled, so validation and network calls are not compiled. Base64
decoding is also disabled. The fixed policy identity is
`mongodb-kingfisher@2.7.0/builtin-medium/v1`.

Each hook submits one bounded JSON object. The boundary canonicalizes the
object, normalizes the canonical workspace path to `$WORKSPACE`, scans those
in-memory bytes with the built-in medium-confidence rules and replaces merged
finding byte ranges with `[REDACTED]`. It reparses the result before
constructing the bounded sanitized payload. Invalid JSON, oversized input or
output, scanner/cache errors and invalid finding ranges produce a fixed omitted
state without returning raw values or secret-bearing diagnostics. The raw hook
payload is never copied to a temporary file or durable state. A user-restricted
compiled-rule cache is warmed during real integration setup and reused by hook
processes.

The packaged hook applies this boundary before capture-state creation. It writes
only a validated sanitized payload or fixed `{"status":"omitted"}` object into
the durable spool. Publication recovery remains required before M9.2 capture
support can be claimed.

## Version-bound normalization

Normalization validates the complete sanitized observation batch before
dispatch, then accepts only the exact integration/version pairs Codex 0.156.1,
Cursor 3.22.7 and OpenCode 1.18.30. An unknown profile returns one safe
unsupported-version diagnostic without native record dispatch. Cursor records
also carry and verify their embedded product version; OpenCode session events
verify the version when the native callback supplies it. Diagnostics contain a
stable code and optional source sequence, never rejected source content.

Stable session, turn, message and tool IDs are SHA-256 domain-separated over the
integration, capture-source identity, entity kind and scoped native ID. Native
aliases remain explicit lookup facts. Replaying the same sanitized batch
therefore returns byte-equivalent normalized records and aliases. Native tool
names remain intact while the common kind is conservative; unknown names map to
`other`, and a file observation without a native tool-call ID cannot create one.

Lifecycle records remain source ordered. Capture completeness, permission state
and execution state are separate fields. The reducer enriches one stable tool
from input and output observations, does not regress a terminal state when a
late running event arrives, and reports conflicting terminal observations
without selecting one by wall-clock time.

The Codex fixture follows the locally installed 0.156.1 hook fields. The
OpenCode fixture follows the exact `@opencode-ai/plugin` and SDK 1.18.30
callbacks; the published plugin archive observed during qualification had npm
integrity
`sha512-bXWcA2VD478PXXke1qAYANTq58fCcbgY0GAUqZIFZqPpo48FoG8RH37nNZ78eHtz+ddiKIhFTBZJRaHF3IgFqw==`.
Both remain sanitized synthetic contract fixtures rather than captured smoke
runs. Cursor 3.22.7 is synthetic contract acceptance only because the locally
available Cursor agent reports a different 2026.09.26 build. These distinctions
must remain visible until M9.6 executes pinned real-harness qualification.

## Exact review anchors

`jj-review` owns fixed-width stable identifiers for review threads, comments,
actions and work requests, plus separate SHA-256 identifiers for anchors,
comment revisions, thread events, snapshots, work-request versions and roots.
Stable workflow identities never substitute for immutable version identities.
All identifiers use canonical lowercase hexadecimal encoding.

`ArtifactAnchorV1` binds one repository and exactly one code text, whole code
artifact, agent text, whole agent artifact, revision, operation or repository
target. Code anchors carry full operation and commit IDs, optional change IDs,
repository-relative paths and the exact native file/object identity. Diff text
anchors additionally carry an explicit before/after operation, commit and path
pair; the selected side must equal the anchor's primary identity. Whole native
artifacts accept matching file or symlink object kinds. External-width gitlink
identities have no owned native object key and therefore fail closed in v1.

Text selections use canonical decimal-string UTF-8 byte offsets and a SHA-256
digest of the selected bytes. Empty, reversed, out-of-bounds, non-UTF-8-boundary
or digest-mismatched selections are invalid. No line number, display text or
search result can replace this byte-level check.

Agent anchors pin an immutable session snapshot plus the exact tool attempt,
message or plan owner snapshot. Text targets also pin the block version, content
object, content part, source coverage and representation, sanitizer and
normalizer versions. Kind mismatches fail closed. Anchor dependency traversal
enumerates every native or AgentStore object needed for original-context reads;
stable entity IDs and optional change IDs remain lookup facts rather than strong
object edges.

Canonical JSON encoding rejects unknown fields, alternate byte-offset forms and
noncanonical round trips. The anchor ID hashes those validated canonical bytes.
The v1 resolver maps an immutable anchor only against an explicitly identified
target supplied through a narrow read port. Callers provide retained original
bytes, bounded target bytes and typed validated predecessor, rename, copy or
agent-block lineage. A same `ChangeId` never substitutes for an exact revision
or for lineage evidence.

## Immutable ReviewStore objects

`ReviewRootV1` selects independent COW catalog roots for threads, comments,
thread events, original-anchor relations, artifact relations, statuses, work
requests, external aliases and accepted source positions. The canonical empty
root omits all page references. Every populated root entry must select a typed
review index page; leaves contain at most 32 stable-key selections and branches
at most sixteen lowercase-hex children.

The registered v1 object kinds are index page, anchor, comment body, comment
revision, thread event, event page and thread snapshot. Review objects use
bounded canonical encoding, typed review-object references and explicit native
or AgentStore dependencies. Anchors forward their exact external retention
edges; other review objects can only point through declared ReviewStore kinds.
Root, revision, event and snapshot content IDs hash their validated canonical
forms and remain separate from stable workflow identities.

Comment bodies are bounded safe Markdown or plain UTF-8 text with an explicit
sanitizer version. Empty bodies, disallowed control characters and oversized
content fail closed. A comment revision retains stable thread/comment IDs,
original author, acting principal and accepted time. Visible edits select a body
object; tombstones and moderated hides require a prior revision, preserving
auditable history rather than deleting prior bytes.

Thread events have repository-accepted, strictly positive per-thread sequence
numbers and authenticated actors. Creation binds the original anchor and first
comment revision. Replies and revisions bind exact selected comment objects;
resolve/reopen actions retain their stable action identity and optional bounded
reason. Event pages contain at most 256 distinct, contiguous sequence entries
and a typed continuation, avoiding an unbounded reverse-linked event chain.

A thread snapshot selects one immutable original anchor, ordered unique comment
revisions, one event-page root, the last accepted sequence, open/resolved state
and selected workflow action. Its overall thread version advances for every
accepted event, while content and workflow versions remain distinct monotonic
preconditions. Parent comments must already occur in the selected order and a
resolved snapshot must identify its workflow action, while an open snapshot has
none. Cross-object loading reconstructs the ordered comments, selected
revisions, open/resolved state and content/workflow counters from the immutable
event stream. It rejects comment revisions from another thread, invalid reply
parents or revision predecessors, repeated workflow transitions, event-page or
event thread/sequence mismatches, missing creation identity and any
reconstructed state that disagrees with the snapshot.

## Root-pinned review store

The WAL coordinator stages each validated review object in the existing
repository-bound envelope and traverses every typed ReviewStore, native code and
AgentStore strong edge before staging a root. A review root independently audits
the semantic value kind of its thread, comment, thread-event, relation and
status catalogs. Work-request, alias and source catalogs still fail closed until
their typed object schemas are implemented.

Original-anchor and exact-artifact relations are bounded immutable linked pages
of stable thread IDs and current thread-snapshot references. Status pages use
the same bounded entry shape for open and resolved threads. Catalog keys are
derived from the typed anchor, artifact or status subject. Root audit verifies
the key, page-chain subject, unique threads, current thread-catalog selection,
original anchor or artifact identity and selected workflow state. A stale
relation snapshot or an open thread in the resolved catalog fails closed.

`ReviewDeltaV1` carries expected and replacement selections for thread
snapshots, comment revisions, thread-event pages, original-anchor relations,
artifact relations and statuses. It rejects no-op edits, duplicate catalog keys
and semantic kind substitution. Applying a delta checks every expected selection
against one pinned root, rewrites only affected persistent radix paths and
stages a fully re-audited replacement root. Thread commands rebuild the affected
direct-index pages in the same review-domain publication, including moving a
thread between open and resolved status pages. Staged objects and roots remain
invisible until the shared domain publication selects them.

A `ReviewStoreSnapshot` binds exact lookup, bounded radix-ordered enumeration
and object reads to the selected root closure. Reading an unselected staged
object returns stale. Catalog enumeration accepts at most 256 selections per
call and uses the last returned stable catalog key for continuation. Old
snapshots retain their original roots and selections after later COW updates.

## Conservative anchor placement

`AnchorResolverV1` is a bounded read-only service over an original immutable
anchor and one explicit target. It returns `exact`, `mapped_unchanged`,
`candidate_modified`, `deleted`, `ambiguous`, `unresolved` or `unavailable` with
categorical evidence and at most the caller's bounded candidate count. A
root-pinned `ReviewStoreSnapshot` can load a selected anchor and invoke the same
pure resolver; the call has no publication, WAL or manifest capability.

Code placement accepts exact object identity or lineage whose from/to operation,
commit, path and file identities match both endpoints. It maps whole-file
identity and byte ranges proven unchanged by common prefix or suffix, returns a
modified candidate when the selected range intersects changed bytes, and keeps
multiple paths or repeated candidate ranges ambiguous. A complete supported
lineage with no target path is deleted. Missing lineage, unsupported targets and
exhausted byte budgets are unresolved; missing or redacted retained content is
unavailable.

Agent text maps to a newer block only when the session/block identity,
representation, sanitizer and normalizer versions match, retained block/content
lineage names both immutable versions, and the newer bytes retain the complete
old content as an unchanged prefix. Exact content versions remain exact. The
resolver never uses a stable tool or change ID alone, mutates the original
anchor, publishes a JJ operation, changes workflow state or records a permanent
placement decision.

## Stale-safe review commands

`ReviewCommandV1` admits only create thread, append reply, edit comment,
tombstone comment, moderated hide, resolve thread and reopen thread. A create
stages its validated anchor, safe body, initial revision, creation event,
bounded event pages and first snapshot atomically. No staged subset is visible
until the shared manifest CAS selects the replacement review root.

The authenticated principal is the author and actor for new comments and the
actor for later revisions or workflow actions. The command has no client author
or accepted-time field. Stable thread, comment and action IDs are derived from
the repository plus authenticated request/publication identity before retry, and
server acceptance time is frozen for the command attempt. Receipt results return
those stable IDs and the exact committed event, comment revision and thread
snapshot references.

Concurrency is scoped to the entity being changed. Replies require the observed
workflow version and an open thread, so two replies can reapply to a newer
content version and both survive while workflow state is unchanged. Edits,
tombstones and hides require the exact selected comment revision. Resolve and
reopen require the complete observed thread snapshot ID, including all visible
comment activity. Resolution therefore rejects an unseen reply; a reply that
loses to resolution returns `thread_resolved` until an exact-snapshot reopen
commits.

Each accepted action appends one immutable event. The coordinator reads the
bounded selected event stream and rebuilds pages with at most 256 entries rather
than appending one reverse-linked page per action. Comment revisions retain the
prior revision edge and original author. Tombstone and moderated-hide records
remain auditable selected history rather than erasing prior bytes.

## Root-pinned capture projection

The publication coordinator can stage one validated sanitized batch and its
normalized result against an explicitly pinned AgentStore root. It stages the
evidence batch first, then typed content, normalized event, reduced tool,
session, alias, source-receipt and projection objects. Permission and file
observations have explicit event variants; a file observation still cannot
become a tool invocation. Every dependency is audited by the existing typed
AgentStore object closure before the returned delta can select it.

The returned `AgentDeltaV1` updates session, alias, source and projection
catalogs in one existing manifest-domain publication. Session snapshots retain
prior selected evidence, event batches, transcripts and tool versions. Source
state rejects repeated or overlapping receipts, records the accepted native
sequence and carries forward explicit coverage gaps. Projection updates replace
superseded session/source versions while preserving prior immutable event
batches. The caller must durably retain the resulting delta and request identity
before publication.

## Durable pending capture

`CaptureCursorV1` records the device-local integration and source identity,
nonzero generation, bounded parser version, optional native session key and the
source-specific accepted position. It is distinct from the published source
state. A native-sequence cursor can acknowledge a batch only at that batch's
last accepted observation.

The local capture spool is an exclusive owner outside repository checkouts. Its
directories are user-restricted on Unix, its entry count and aggregate bytes are
bounded, and every entry is a versioned regular file named by the capture batch
ID. Enqueue validates the supported normalizer profile, sanitized batch, cursor
and repository binding, writes through an atomically renamed temporary file,
synchronizes both file and directory and only then returns the cursor
acknowledgement. Exact retries are idempotent; a changed repository, profile,
cursor or batch under the same ID fails closed.

Preparing publication atomically replaces that entry with the exact validated
`AgentDeltaRequest`, including stable principal, request and publication IDs.
Restart discards only unacknowledged atomic temporary files, then reopens and
validates every pending entry. Corruption, symlinks, unexpected files,
concurrent ownership and byte-limit exhaustion fail closed. Completion removes
only a prepared entry whose request and publication IDs match the committed
outcome, then synchronizes the directory. An unprepared capture cannot be
discarded through the completion path.

Native capture publication is two phase. A bounded canonical prepare request
sends the supported profile, sanitized batch, stable request/publication IDs and
result to the repository service. The server derives the principal and
activation time from the admitted peer, pins the current AgentStore root and
returns the exact validated delta request; the device spool must persist those
bytes before publication. The second call submits only that prepared request.
Committed receipts and unknown outcomes retain the same identities, while a
different peer principal, changed retry or stale same-source edit is rejected.
Preparing again after an unpersisted response may stage a fresh intent, but a
persisted prepared entry is never restaged under its existing receipt identity.
The CLI worker validates every pending entry against the active repository
before its first network call. It orders by integration, source instance,
generation, first source sequence and batch ID, derives domain-separated stable
request and publication IDs from repository plus batch ID, and uses a bounded
batch acknowledgement as the receipt result. Only a matching committed receipt
removes the synchronized prepared entry. Unknown outcomes stop the drain;
transport errors, staleness, rejection and mismatched replies retain the exact
entry and prevent later captures from passing it.

## Installed hook ingestion

`jj agent hook <codex|cursor|opencode>` is the owned, noninteractive ingestion
entrypoint installed by setup. It accepts no caller-selected repository or
source identity. Before sanitization it reads the device registry and verifies
the repository, canonical workspace, explicit enablement, exact qualified
version, recorded installed state and current owned project-file hash. A missing
compiled-rule cache fails closed and does not trigger a cold hook-time build.

Standard input is bounded to four MiB plus one overflow byte. After in-memory
sanitization, a dedicated bounded lock allocates a strictly increasing native
sequence under one stable random source instance and generation. The source
reservation is atomically synchronized before spool enqueue; a failed or
interrupted enqueue can leave an explicit sequence gap, while restart and
unknown outcomes cannot reuse the sequence. Batch and observation IDs are
domain-separated hashes of source identity, generation and sequence.

Codex and Cursor observations use local-hook trust; OpenCode observations use
local-plugin trust. Every batch records the exact Kingfisher policy and matching
version-bound normalizer profile. Concurrent hook processes share the allocator
without opening the registry's exclusive administrative lock. Raw input,
findings and secret-bearing errors are never written or logged. Pending batches
remain in the existing capture spool until the hook's bounded best-effort
publication worker prepares and commits their exact AgentStore delta and receipt
identity. A server outage or three-second hook budget expiry preserves the
capture and defers another attempt to a later hook invocation.

## Device-local integration registry

The CLI owns a versioned registry outside the checkout for each native
repository and canonical physical workspace. It persists one nonzero random
device identity and separate detection, desired enablement and setup state for
Codex, Cursor and OpenCode under an exclusive user-restricted lock. Atomic
synced replacement and strict repository/workspace validation preserve the
identity across restart and reject another authority opening the state.

Discovery searches the current PATH for each owned executable and executes a
two-second, 256-byte version probe. Only the exact qualified versions Codex
0.156.1, Cursor 3.22.7 and OpenCode 1.18.30 are supported. Missing executables,
other versions and failed probes are explicit states; stderr is discarded and
never enters the registry or command output. Explicit noninteractive enablement
may select only supported integrations, and dry-run does not modify durable
state. Enablement records desired policy while setup remains not installed.

Setup requires both enabled policy and exact-version support. It merges only the
owned command handlers into Codex or Cursor project hook documents and installs
one namespaced OpenCode project plugin. Before writing, the device registry
atomically stores exact original bytes and the original and owned SHA-256
hashes. Project files use bounded atomic replacement, preserve unrelated JSON
values and reject symlinked target directories. Restart recognizes either the
exact original-to-owned transition or the installed hash; any other bytes are
drift and block replacement. Dry-run writes nothing.

Owned removal is independent of current executable detection so an absent or
upgraded harness can still be disconnected safely. It accepts only the exact
owned target or an exact original already restored before interruption. The
former is atomically replaced by the recorded original, or deleted when setup
created it; the latter resumes ownership cleanup. A synchronized tombstone makes
record retirement restart-safe. Only after target and ownership cleanup does the
registry record disabled/not-installed state. Drift, symlinks and unexpected
ownership files fail closed, while dry-run and completed retries do not modify
project bytes.

The Codex contract follows `openai/codex` tag `rust-v0.156.1` at source revision
`b412ff32c417f855c2b2d1581b77058eed87c84b`; the OpenCode plugin contract follows
`anomalyco/opencode` tag `v1.18.30` at source revision
`3104c1428ec91f809e5ab86631300de41eb6952e`. Cursor uses the documented version-1
project hooks shape, but remains synthetic setup acceptance because the local
agent reports `2026.09.26-dd393fe` rather than qualified `3.22.7`. Drift-safe
removal and source-ordered exact publication are implemented; pinned
real-harness qualification remains M9.6 work.
