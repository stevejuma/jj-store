# V1 control records and catalogs

Control records are canonical, versioned JSON envelopes containing repository
identity and record kind. SHA-256 references bind their exact bytes. WAL paths
include a cryptographically random attempt ID as well as the record digest.
Catalogs use immutable bounded radix pages (32 entries per leaf, 16 children per
branch, at most 64 nibbles of depth). Separate selected roots index retained
object descriptors, receipts and committed sequence positions. Reads follow
those roots without bucket enumeration; incremental updates copy only changed
paths. Old roots and records remain readable and are never deleted.

A bounded manifest selects the catalog roots, exact tail, nonempty operation
heads and immutable settings together. Strict requests bind the logical snapshot
(epoch, code-state version and heads), independent of agent/review publications
and checkpoint-only manifest revisions. The global publication sequence still
orders every committed domain event. Native/token canonical intent covers the
complete publication request; request and publication identity indexes share a
durable result receipt. Duplicate lookup precedes stale-snapshot checks and head
changes.

Typed Thread/Reaction commands privately pin their full source manifest digest
through publication. Authorization and matching durable receipt lookup precede
this pin and all staged-root audits. A failed CAS returns stale to the typed
outer loop before any retry can publish against changed source roots. That loop
reloads and resolves source membership and visibility again. This private guard
adds no fields to legacy DomainRequest intent bytes or manifest formats, and
unknown writes still reauthorize and resolve the original request receipt.

Format 2 manifests select canonical AgentStore and ReviewStore roots alongside
native code state. Each domain has an independent version, while one global WAL
sequence and manifest revision retain total publication order. Code transitions
preserve both collaboration roots; agent and review transitions preserve native
heads, settings and the sibling collaboration root. A domain transition may
extend the selected native inventory only with complete verified closures that
its typed artifacts strongly reference. Cold recovery verifies selected current
and historical roots without Tantivy, Nostr or a second manifest.

A format 2 manifest may also select a versioned replication root. Repositories
without that optional root retain their existing publication behavior until the
explicit seed upgrade. A replication-enabled repository records its local origin
stream, accepted origin receipts and sparse causal prefixes in immutable COW
records selected by the same manifest. Every authored native, AgentStore or
ReviewStore publication stages one canonical origin envelope from the successful
CAS base and selects its receipt and next stream prefix in the same manifest CAS
as the content roots and WAL candidate. Receipt replay returns the already
selected publication, and a failed candidate does not advance the accepted
origin sequence.

Cold recovery follows the selected replication root and verifies every receipt,
origin key and digest, local delivery identity, predecessor chain, causal prefix
and contiguous local stream before returning authority. The replication catalog
does not replace the native inventory or any content root. Its local delivery
position is destination-owned bookkeeping; the logical origin key and digest
remain portable.

Receiver publication is a distinct authorized path over the same coordinator.
Object transfer may stage a digest-verified immutable closure but cannot select
it. The receiver verifies that closure and every origin dependency, resolves a
matching stable import receipt before mutation, topologically orders contiguous
origin streams, reduces only unseen origins and selects their content effects,
accepted-origin receipts, causal prefixes and one local WAL candidate in a
single destination manifest CAS. A pure import preserves the original origin
keys and digests and creates no authored origin. A duplicate-only batch returns
the selected destination receipts without another WAL entry or state-version
advance. Code heads use the configured pinned JJ operation-ancestry reducer;
AgentStore and ReviewStore roots use their typed accepted-fact reducers. Each
reducer derives only immutable objects and catalog selections from the imported
delta and authored root. It does not invoke a normalizer, review command,
controller, hook or external adapter. All changed collaboration roots and their
derived relation/status catalogs remain part of the same destination manifest
CAS as the accepted-origin receipts.

Agent and review roots are typed `AgentRootV1` and `ReviewRootV1` records rather
than caller-supplied opaque payloads. Their catalog and selected object edges
use repository-bound immutable record envelopes. Staging validates canonical
schemas, semantic object kinds and a bounded transitive dependency closure;
publication repeats that audit before CAS, and recovery repeats it before
returning authority. Review closure audit additionally verifies selected comment
and event ownership, accepted sequence continuity and snapshot tail agreement.

Checkpoint and association objects expose native operation/commit retention
roots separately from AgentStore object edges. Staging verifies those native
closures through the repository-bound dependency source. Agent publication adds
them to the same selected inventory without reporting them as newly published
code or changing native heads/code-state version. Recovery requires every
current and historical domain retention root in that publication's exact
inventory and verifies its stored descriptor and transitive native closure.

Normal AgentStore writes publish a typed delta rather than a caller-computed
replacement root. Each edit binds a stable catalog key to its expected and
replacement typed object across the session, alias, checkpoint, association,
relation, source and projection catalogs. The coordinator authorizes before
receipt lookup, applies edits to a root-pinned persistent catalog, stages the
resulting root and uses the existing agent-domain publication path. Its intent
digest excludes the generated page/root identities. A concurrent disjoint agent
edit is reapplied to the newer root; a same-key precondition change remains
stale. Code heads, code-state version and the review root remain unchanged, and
unknown outcomes still resolve under the original request/publication
identities.

Normalized capture staging reuses this typed-delta boundary. Against one pinned
AgentStore root it stages a complete evidence/content/event/tool/session closure
plus alias, source-receipt and normalizer-projection selections, then returns
one `AgentDeltaV1`. None of those staged objects are visible until the retained
delta is published with its request identity through the existing agent-domain
CAS. Receipt replay therefore uses the exact retained delta; rebuilding a new
delta after an unknown outcome is not a substitute for a durable pending spool.
Repeated or overlapping source receipts fail stale, and updating this domain
does not move code or review state.

The capture recovery worker derives separate request and publication identities
from the repository and durable batch ID. It persists the server-prepared delta
before publishing and submits that exact request on every restart. It completes
the local entry only for a receipt whose principal, request ID, publication ID
and result match the prepared request. Unknown or mismatched outcomes retain the
entry and stop source-ordered progress rather than rebuilding a later delta.

The publication facade exports the validated normalizer profile, sanitized batch
and exact delta request types needed by the service prepare/publish boundary.
Service and RPC layers consume those owned facade types rather than importing
the neutral agent model or WAL implementation directly. This changes no
serialized record: canonical capture JSON is only transport framing, while the
returned `AgentDeltaRequest` remains the durable intent used by receipt lookup
and manifest CAS.

Review staging likewise accepts typed `ReviewDeltaV1` edits against a pinned
ReviewStore root. It copy-on-write updates thread, comment, thread-event,
original-anchor relation, artifact relation and status catalog paths, rejects
changed expected selections and stages a complete root whose native and
AgentStore dependencies have been verified. Thread commands update the current
snapshot in every direct relation and status index in the same review-domain
candidate; resolve and reopen remove the old status selection and add the new
one. Generated pages and roots are internal candidates rather than
client-controlled publication inputs.

Whole-session review anchors retain the exact immutable AgentStore session
snapshot and its complete dependency closure, even when that session later
advances. Staging and cold selected reads verify that the reference has the
session-snapshot kind, exists and names the anchored stable session. A missing
or substituted snapshot fails closed. Creating or replying to a session comment
uses the existing authorized, receipt-backed ReviewStore publication; only the
manifest CAS selects its candidate and only the review state version advances.
The additive anchor does not change manifest, WAL, request or receipt formats.

Read services may retain an audited `ReviewStoreSnapshot` only within bounded
process-local cursor limits. The snapshot reports its selected closure-object
count for that accounting, while enumeration and detail continue to follow only
the selected root. This adds no staging, command, receipt or manifest-CAS
capability and changes no stored ReviewStore bytes.

Anchor placement is outside publication. A root-pinned read may load a selected
anchor and run the bounded resolver against an explicit immutable target, but it
cannot stage a root, append a WAL record or move manifest position. A future
confirmed placement remains a separately authorized typed mutation.

The receipt-backed review command boundary hashes repository, principal,
request/publication identities and the validated semantic command before it
generates server acceptance time or immutable candidate references. It
authorizes the current review domain and checks the matching receipt before any
thread, workflow or comment precondition. A later retry therefore returns the
original committed result even after newer review activity; altered content
under the same retained identity is an intent conflict.

The transport service accepts no caller-authored actor or publication ID. It
derives the latter from repository, authorized principal and request ID, then
uses this same command intent for publication and replay validation. Durable
status reads the manifest-selected receipt and validates its identity and
canonical result bytes without reapplying the command.

Create, reply, revision and workflow commands apply typed expected/replacement
edits to the current root. A reply binds only the workflow version; an edit,
tombstone or hide binds the exact comment-revision ID; resolve/reopen binds the
complete thread-snapshot ID. If the manifest CAS races, the coordinator reloads
and reapplies those semantic preconditions. It retries a changed review root
only when the bound entity state remains valid, never overwrites the stale whole
root and never treats a workflow/content conflict as an automatic merge. Review
publication increments only the review state version and preserves native code,
settings and AgentStore state from the successful CAS base. Unknown outcomes
retain the original request/publication and deterministically derived entity
identities for receipt resolution.

An authorized collaboration-enable request upgrades a format 1 manifest by
appending a receipt-bearing WAL event. The event selects deterministic empty
agent and review roots, records the exact boundary at which domain history
begins and leaves native heads and code-state version unchanged. Existing WAL,
inventories, settings, receipts, checkpoints and native IDs remain immutable.
CAS conflicts retry from the new base; an unknown manifest outcome resolves by
the original request/publication identity or remains explicitly unknown. A code
command prepared against unchanged pre-enable code state remains valid.

A checkpoint stores the folded state and selected catalog roots at its captured
position. Selecting it changes only manifest representation/revision and keeps
any newer publication tail, heads, settings and catalog roots from the
successful CAS base. It cannot prune receipts, evolution history or committed
log records.

The coordinator requires a publication guard with separate authorization and
semantic-validation methods; no production allow-all implementation exists. It
authorizes before receipt lookup and dependency reads, validates new transitions
before staging, and repeats both checks against a fresh base after CAS conflict.
Receipt replay repeats authorization but never reruns the original mutation.
Transport/service integration supplies the shared policy in M3/M5.

Cold integrity recovery verifies retained historical inventory and settings
records as well as the current roots. Checkpoint catalogs and predecessor
checkpoints must remain readable and agree with the selected history; corruption
in an old root is not hidden by a newer root. A shared bounded page audit avoids
reading unchanged radix subtrees repeatedly. Ordinary startup may read the
manifest's already folded state; the explicit integrity recovery performs a full
retained-data audit and fails if its configured budget is insufficient.

`Coordinator::copy_native_to` begins with that full recovery and derives an
exact bounded closure from selected catalogs rather than storage listing. It
includes every retained native inventory entry, catalog page/value, WAL record,
receipt, settings record, checkpoint and current or historical AgentStore and
ReviewStore root/object dependency. Review-owned AgentStore references remain
part of the copied closure. When present, the selected replication root, its
accepted-origin catalog pages, origin receipts and original envelopes are also
retained. Stored native objects and immutable control records are conditionally
created before the canonical selected manifest is installed as the final object.

The destination may be empty or already contain byte-identical immutable
objects. Different bytes under any selected key fail closed. A lost manifest
write acknowledgement resolves only by reading back the exact bytes. After the
manifest is visible, a coordinator backed by the destination's native dependency
source and the same operation-head reducer performs cold recovery and must
reproduce the source manifest digest and all three roots. History, object and
byte budgets plus the coordinator timeout bound the operation. This primitive
introduces no list, delete, remap, promotion, backup or migration authority.

Selected settings are readable from a checked ManifestSnapshot using an owned
storage accessor, so request-scoped guards can verify policy without
constructing a second publication coordinator or using mutable global
authentication state.

Cancellation conformance synchronizes with the selected storage fault boundary
before advancing a controlled test clock. A pre-staging deadline must not be
misreported as evidence for a post-CAS lost acknowledgement.

Checked manifest snapshots expose the same bounded committed-inventory reader
used by the coordinator for native semantic validation. Authenticated
prefix/copy enumeration collects matching identities through bounded descriptor
windows of the same selected root. These readers do not publish state or
introduce a second commitment test.

Bounded inventory projection windows retain only operation identities and copy
parent identity sets from the already verified selected descriptors. They scan
1–256 descriptors, retain at most 65,536 distinct copy-parent edges per window,
and share the ordinary 1,024-record/32 MiB encoded scan ceiling with separately
reserved catalog-path work. Report processed descriptor count, charged record
reads and successful encoded record bytes so an explicit preparer can impose
independent whole-traversal budgets. Discard unrelated identities and dependency
sets after each descriptor; load no native object bodies. Edge or byte admission
exhaustion returns a nonempty processed prefix with its exclusive continuation,
or an error if no descriptor can be admitted. Empty matching results may still
carry processed descriptors and a continuation; they never certify completion.
Copy descriptors reject non-copy dependencies. This is a dependency relation
projection, not ordered copy terms, a complete closure audit or publication
authority. Paired-root addition projections retain exact baseline descriptor
selection; only complete traversal from the start proves whole-base retention.

Historical settings lookup binds epoch, publication position and exact heads to
manifest-selected history. Position zero selects the immutable initial policy;
later positions select their committed WAL record. Callers authorize against
current policy before this bounded lookup. It cannot load an orphan candidate or
accept caller-supplied record references, and remains valid after checkpointing.
