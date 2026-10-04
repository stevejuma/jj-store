# Native JJ integration

Read-only local identity hints continue to require an exact secure-config
metadata binding before selecting a repository email. They never migrate
configuration or introduce aliases into query evaluation. Multi-harness capture
verification retains these boundaries; conditional-expression lint cleanup does
not change identity selection or native persistence.

The local prepared index adapter retains selected-view visibility and reverse
reference annotations under shared resident-memory admission. It delegates
change prefix ambiguity and target ordering to the pinned JJ index, then applies
the exact prepared visible membership without creating per-request ancestor
sets. Different head selections fail closed. Prepared log, commit detail and
evolution readers use fresh authorized scoped stores, and operation/predecessor
traversal retains ordinary cancellation, byte, work and record limits. No source
index repair, extra-table import or publication is permitted.

WorkStore comment reactions use the existing native RPC collaboration command
and read envelopes with an additive typed target. Exact WorkStore selection is
captured alongside ReactionStore; no code-operation mutation, upstream JJ import
or new protobuf framing is introduced. Local companion providers apply the same
resolver independently from native RPC.

Native collaboration RPC pages and direct Thread/message/image reads share the
bounded scoped materialization service with HTTP. Current admission and epoch
are rechecked for each response; cache reuse never changes wire DTOs, signed
cursor bindings, native persistence or publication authority.

The collaboration transport carries optional immutable human reply parents
without changing native operation or runner authority.

Explicit local BoundCapture carries the exact observed Thread version and
primary assignment with the original sanitized batch/profile. It retains one
real session replacement or a validated diagnostic-only capture gap, stages the
shared link without changing lifecycle and selects Agent and Thread descriptors
in one companion ref CAS. The Thread descriptor audits and retains the complete
newly staged session closure before selection. Matching receipt replay precedes
entity/source preconditions; stale bindings select neither root. The private
spool retains this original bound request across unknown outcomes. Legacy
unlinked capture tags, field order and receipts remain unchanged.

Native RPC preserves optional capture bindings in both preparation and delta
publication. The durable device spool stores the explicit observed assignment
before source acknowledgment and the exact server-prepared acceptance time
before publication. Retries after unknown outcomes do not refresh either
precondition or identity. Empty optional fields preserve unlinked wire and spool
encodings.

Diagnostic-only preparation retains the same explicit binding in its typed
delta. The additive gap target pins source/event evidence and observed binding;
it never creates a primary assignment or a session. Unix RPC reconstruction and
receipt replay must preserve both this target and the exact prepared identity.

Prepared Agent delta decoding retains and validates an optional explicit Thread
binding; it must not reconstruct the legacy fields and silently drop that
intent. Unlinked capture preparation rejects an unexpected returned binding.
Native bound preparation is explicit; local-provider qualification and future
runner integration have separate gates.

The companion metadata adapter provides bounded immutable blob staging and
staging reads for the shared collaboration reducer. These operations cannot
select metadata. Public retained reads additionally prove membership in the
explicit selected metadata closure; service facades audit their domain root
before exposing immutable objects. Native object access and code publication
keep their existing authority boundaries.

The local Git-backed companion profile is governed by local-jj.md. Its Git
object/ref adapter stays in this crate and does not replace native publication.
The local repository resolver loads only the pinned GitBackend to obtain its
object database; it does not load or reconcile operation heads or snapshot
files.

Source evidence capture pins one explicit operation with `RepoLoader::load_at`
and archives complete owned operation/view and GitBackend commit records. Its
bounded closure follows ancestry and evolution dependencies, preserves ordered
merge terms and extra metadata, and stages immutable blobs without publishing JJ
heads or editing a checkout. Selected capsule reads reconstruct and verify
source operation/view identities without requiring the source operation store or
derived index. Capsule selection remains a domain publication concern.

Capsule retention derives every non-root physical commit and backend tree from
the exact archive. Selected reads require those entries in the selected metadata
retention inventory, even when a blob was retained for another domain. An anchor
additionally verifies its operation/profile and exact commit membership against
that capsule.

Search visibility uses the same bounded native commit-parent traversal as
operation differences. It starts at the explicitly selected operation's view
heads and does not follow predecessor links or reconcile operation heads.
Resource exhaustion returns an error instead of a partial visibility set.

Commit ancestry, operation ancestry, evolution, and publication order are four
separate histories. Multiple operation heads are legal. CLI repository opening
can use native reconciliation, but HTTP GET cannot. Backend copy lookup and
submodule data need full coverage beyond the reference SimpleBackend.

Independent conversion uses a read-only bounded source capsule and owned raw
object archive. The integration boundary translates all selected code references
and ancestry/evolution dependencies into fresh destination identities, retaining
optional predecessor-map semantics. It never reconciles source heads or
snapshots physical checkout files. Native codecs and envelopes remain unchanged.

## Requirements

| ID         | Requirement                                                                                                                                                                          |
| ---------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| JST-JJ-001 | Custom CliRunner factories MUST implement the exact pinned Backend, OpStore, and OpHeadsStore contracts over RPC, with equivalent in-process stores on the server.                   |
| JST-JJ-002 | Only guarded OpHeadsStore publication MUST advance authority; request-scoped principal, intent, snapshot, and receipt data MUST NOT use global mutable context.                      |
| JST-JJ-003 | Read sessions MUST pin explicit operations through load_at and MUST NOT reconcile heads; authorized mutation sessions MUST use native transactions and finish descendant rewrites.   |
| JST-JJ-004 | Physical checkout bookkeeping MUST stay client-local; logical workspace IDs MUST be distinct across unrelated clients and stale-workspace updates MUST preserve unsnapshotted edits. |
| JST-JJ-005 | Indexes and prefix lookup MUST be bounded, rebuildable, and versioned by native profile; normal required methods MUST NOT contain silent stubs.                                      |

## LocalTrusted native transport

On supported Unix hosts, a LocalTrusted server exposes native RPC only through
its installation-owned filesystem socket. The socket is mode 0600 and each
accepted connection must report the owning UID before a stable
installation-derived principal is bound. The transport accepts no bearer
metadata in this mode. Repository creation, attachment, reads and publications
still use the existing native factories and guarded service sessions; operation
IDs never become client or installation identities. Authenticated TLS RPC keeps
its existing bearer policy and wire contract.

## M0 extension qualification

M0 probes register a request-scoped OpHeadsStore through StoreFactories,
preserve per-principal publication IDs and exact preconditions across real
loopback gRPC, and load explicitly pinned divergent operations in a bounded
local async task. The RPC fixture is test-only and does not implement durable
publication or policy.

## Native codec boundary

Collaboration RPC uses owned service/domain contracts for Thread and Reaction
commands and reads; native factories and code topology remain independent. The
admitted peer supplies the actor through the service, and typed receipts retain
the original request across transport reconnection. Raster bytes have their own
bounded binary carrier; neither a cursor nor a content ID authorizes reading an
unselected asset or publishing repository state.

The integration crate owns explicit conversions for all pinned commit, tree,
copy, view, and operation fields. File and symlink IDs hash raw bytes; the other
kinds hash the full native value. Submodule IDs are preserved as opaque external
links. Decoding validates structure before invoking native constructors that
would otherwise panic. Physical integrity alone never establishes native
semantic identity.

## Checked native object access

Native object access verifies the expected kind/ID, envelope checksum and
decoded identity before returning metadata. Immutable-create collisions load and
compare the complete existing native value. File staging and verification stream
with 64 KiB buffers and configured disk limits. A file reader is exposed only
after full checksum and BLAKE2b verification. Unknown upload outcomes remain
explicit unless a subsequent read proves the identical immutable object is
present.

Dependency enumeration covers all conflict terms, nested remote tags/bookmarks,
per-workspace Git heads, logical working-copy commits and both predecessor
representations. The native object accessor implements the WAL inventory source
using checked reads and explicit virtual-root values, including the empty tree.

The real-storage integration fixture publishes complete native operations
through the guarded coordinator and verifies retained native dependencies after
a fresh process and storage restart. The fixture explicitly permits its
disposable test principal; production transports must provide the shared
authorization/semantic guard and cannot instantiate an implicit permissive
coordinator.

## Native operation stores

The native OpStore persists complete owned view/operation codecs and exposes
virtual roots from the pinned profile. A request-bound authority interface is
mandatory for read/write authorization, committed operation-ID enumeration and
head publication. Prefix lookup combines a bounded manifest-selected inventory
with only this session's acknowledged staged operations; it never scans bucket
keys or treats arbitrary candidates as committed history. Truncated enumeration
is an error, never a successful partial prefix match.

OpHeadsStore delegates atomic sibling-preserving transitions to the guarded
publication authority. Its optional coordination lock is intentionally empty:
manifest CAS provides correctness. Unknown publication outcomes propagate as
errors, leaving upstream callers unable to claim success. Destructive GC returns
an explicit disabled error. Backend/factory/RPC wiring remains a separate M3
gate.

A full staged-operation budget rejects a new distinct operation, while an
identical acknowledged metadata-upload retry remains idempotent.

## Native commit backend

The commit backend uses checked native codecs for trees, commits, symlinks and
copy histories, and the verified streaming spool for files. It authorizes each
read or write, including virtual roots. A bounded semaphore limits outstanding
object operations. An unknown upload is an error, never an acknowledged native
write. Existing signatures are preserved; requesting a second signature on a
signed commit is rejected. New signatures cover the domain-separated V1 unsigned
commit payload, with the exact signed bytes retained in the native signature.

Copy enumeration uses only the committed inventory plus this session's
acknowledged writes. Queries enforce node, work, retained-byte and deadline
budgets, fail explicitly instead of truncating, and return deterministic reverse
topological order. Copy records cover the DAG range excluding all ancestors of
the lower bound. A history first introduced at a target is traced to its nearest
ancestor source on each lineage, including intermediate histories created within
one commit. Inherited histories do not generate repeated copy events. Conflicted
trees simplify signed terms per path before retaining positive file
alternatives, so canceled terms cannot reintroduce deleted files. Path filtering
applies to targets. Placeholders have no persisted history and generate no
inferred copy events. A file created and copied entirely within one commit has
no ancestor copy record. This integration does not infer renames from equal file
contents.

Native factories, authenticated transport wiring and full CLI/workspace
lifecycle acceptance remain separate M3 gates.

## Authenticated sessions and factory binding

The service provides one repository-bound authority for all three native stores.
Native head-publication IDs are domain-separated hashes of repository,
principal, removed heads and added operation. An identical transition recovers
the same receipt after process restart without a volatile sequence counter.
Different transitions within one CLI invocation receive distinct identities.
Unknown outcomes remain errors under that original identity.

Factories bind local metadata to the repository and native profile using bounded
credential-free marker files. Derived JJ indexes and physical working-copy state
stay in each client's cache and checkout. Attaching uses the selected current
operation and a supplied unique logical workspace name; it never starts an
existing repository's workspace transaction from the virtual root. Explicit read
loading uses load_at, while authorized mutation loading may reconcile heads.

The in-process authority performs real token and semantic validation. Transport
and executable command integration remain M3 gates; library lifecycle acceptance
alone cannot complete them.

Native store futures and returned streams enter a captured caller-owned Tokio
runtime for each poll. JJ's filesystem worker threads do not inherit a reactor;
enter guards must not survive an await or move between threads. The caller keeps
a multithread runtime alive for the session: synchronous upstream reads may
block their calling thread while I/O drivers must keep running. Factories reject
a single-thread runtime. Cache initialization writes only local store bindings
and derived-store metadata; it avoids upstream initialization paths that assume
root reads cannot fail authorization.

## Native RPC V1

The native gRPC boundary accepts typed repository/object IDs, immutable object
uploads, committed ID queries, and head transitions. It exposes no raw keys,
manifest CAS, policy writes, or caller-selected principal. The service verifies
complete envelope and native identity before immutable storage, preventing a
malformed upload from poisoning a content-addressed key. Remote factory adapters
reuse the same native codecs and runtime binding as in-process factories.

Each RPC authenticates current repository permissions. Uploads recheck write
permission after bounded streaming and validation; downloads recheck read
permission as chunks are emitted. Streams have bounded chunks, total bytes,
concurrency and elapsed time, and drop owned spools when canceled. Publication
uses the shared native semantic policy and deterministic receipt identity.
Ambiguous transport failures preserve unknown publication outcomes. An explicit
insecure-loopback client option is required for local plaintext; shared channels
require TLS. Executable command and administration integration remain separate.

AgentStore capture adds a bounded two-phase service to this same native RPC. A
prepare call carries canonical supported-profile and sanitized-batch JSON plus
stable request/publication IDs; the server pins the current AgentStore root and
returns an exact validated `AgentDeltaRequest` with its admitted peer principal.
The publish call accepts only that prepared request and returns either the full
committed receipt or the original unknown identity. The client cannot select an
AgentStore root or actor, and operation IDs never cross this domain boundary as
commit or capture identities.

Writable replication adds versioned service-owned status, frontier, finite
barrier, wait and search-freshness DTOs plus corresponding `NativeClient`
methods. Authenticated native RPC exposes status, now, pause, resume, barrier
capture, bounded wait and replica inspection without importing upstream JJ
types. Status and inspection read local runtime and durable checkpoint state
only; they do not contact the upstream, advance a frontier or reconcile native
heads. Reconciliation remains an explicit application mutation using the same
guarded command service and durable receipt policy.

The executable installs lazy native factory closures: a native cache's bounded
connection record selects the endpoint and repository, while credentials are
resolved independently. The shared factory instance within one invocation keeps
staged object inventories consistent across backend/operation/head stores.
Ordinary Git-backed workspaces continue to use upstream factories without
resolving native credentials or connecting to the server.

## RPC administration

The native V1 service includes typed repository creation and token list/apply/
status methods. Creation uses the separate operator policy; all token methods
require current repository admin access. Hosted routing binds repository IDs
lazily against the configured storage namespace, without a mutable in-memory
repository catalog or bucket listing. Unknown and unauthorized repository
bindings both deny access. The native object/head surface remains independently
guarded and cannot mutate settings.

Hosted creation reserves the requested installation alias before touching
repository authority. After manifest publication succeeds or an identical retry
recovers it, RPC atomically registers the alias with the initial admin token ID
as a discovery candidate. Committed grant/revoke receipts update that candidate
index, while every repository operation and discovery result still requires
current full-credential authorization against native settings.

Administration requests and responses bound snapshot heads, token counts,
message sizes, deadlines and concurrent work. Credential-bearing protobuf
messages use redacted Debug output; decoded secrets are cleared after parsing.
TLS and explicit loopback transport rules are identical to ordinary native RPC.

## Bounded application graphs

The graph executor loads only the service-selected operation through load_at,
uses a read-only authority and permits physical object access only to the
captured committed inventory. It runs upstream revset, operation-walk and
evolution algorithms without a server checkout. Graph DTOs retain native IDs,
ordered parents, direct/indirect/missing commit edges, local bookmark conflict
annotations, remote bookmark names with their remote identity, logical
workspaces, visibility and divergence. Local and remote bookmark decorations
remain distinct even when they share a bookmark name. Evolution distinguishes
absent predecessor records from known-empty records.

Requests have explicit page, object-byte, output-byte, index-size, traversal,
revset-size and elapsed-time bounds. Native futures run on a caller-owned
multithread reactor inside bounded blocking jobs; cancellation retains the job
permit until worker exit. Query caches bind upstream revision, native profile,
repository, epoch and operation. Cache construction stages a complete directory
before rename, and a local lock rejects competing cache writers. Losing the
cache causes reconstruction from committed native objects.

Operation graph metadata retains snapshot flags, logical workspace names and
attributes alongside timestamps, user, host and description. A corrupt cache
profile or backend type fails explicitly; removing the derived directory allows
reconstruction. Managed index-file corruption recovery is described below. Large
rebuild task reporting remains separate qualification work.

The current work/deadline checks bound admission, I/O and returned rows. Native
synchronous CPU algorithms cannot be preempted by a Tokio timeout; these limits
do not yet establish production CPU/memory qualification. That qualification
must precede shared HTTP query exposure; managed cache retention is bounded
independently.

Native object DTOs reuse the explicit frozen V1 field schema after native
integrity verification. Exporting those owned types does not change persisted
bytes. Content readers apply committed membership and current authorization
before root handling, metadata reads and stream preparation. Native directory
queries run with read-only backend authority and no signer or physical checkout;
trivial merge semantics are delegated to the pinned library.

Operation-diff reads compare decoded native view fields and traverse
commit-parent links for visible-set changes. They do not infer visibility from
predecessor retention, bookmarks or storage inventories. Immediate
operation-parent selection is explicit; read capabilities and bounded workers
remain mandatory.

The application content layer uses native merged-tree diff streams and conflict
traversal. It shares a read-only Store and native merge options with directory
reads. Copy sources and unresolved terms are preserved without heuristic renames
or writes. Native line diffing runs only after conservative text/line-work
admission checks in the bounded query worker.

Copy ancestry queries preserve placeholder parent fields but never attempt to
load a placeholder object. The application diff layer treats placeholders as
lineage boundaries, retaining complete structural deltas and all source choices.
It does not invoke the pinned experimental copy-history diff helper, whose graph
walker requires every parent (including placeholders) to be a stored history.

## Managed graph cache lifecycle

The server query cache uses a private `query-cache-v1` namespace below the
pinned upstream revision and native profile. Bounded slots retain repository,
epoch and operation bindings, a recency counter and checksums of complete
derived files. The checksums detect local cache damage; native objects remain
the authority. Query execution checks current authorization before touching
these files.

An exclusive namespace lock serializes graph-cache users, including separate
executors/processes sharing that directory. Contention returns Busy. Completed
queries record immutable graph data before atomically installing a staged cache
and its bounded receipt. Missing files, invalid cache receipts or checksum
mismatches trigger reconstruction. A mismatched native backend/profile binding
fails explicitly. Source corruption or denied reads cannot be repaired by cache
rebuilding and remain errors.

Retention bounds slot count, file count, individual-cache bytes and total
retained bytes. Defaults retain at most 16 slots, 2 GiB of file bytes per slot,
4 GiB per repository, 16 GiB in total, and 4096 filesystem entries per slot.
Cold staging shares these byte allowances. These are admission and retention
limits, not measured throughput guarantees. Least-recently-used slots are
evicted under the namespace lock; failed builds and abandoned staging
directories are disposable. Symlinks are never followed. A canceled worker
retains its lock and permit until it exits. Responses report whether their cache
was built, reused or rebuilt and retained cache counts/bytes. These are local
diagnostics, never publication positions. Transient build disk use, native
synchronous CPU work and production query memory still need resource
qualification before shared HTTP exposure.

Native query request cancellation signals the shared worker budget. Pending
permission checks and storage I/O are dropped, and later CPU/work checkpoints
reject the canceled request. A worker keeps its permit and cache lock until
actual exit, including while a bounded synchronous native step finishes.
Cancellation never publishes native state. A successfully returned verified file
stream owns its remaining lifetime separately from the preparation request.

The native runtime adapter disables Tokio's scheduler-only cooperative budget
for each backend/stream poll. Pinned JJ can synchronously poll these futures
through pollster; that executor cannot replenish Tokio's task-local budget.
Project work/byte limits, real deadlines and cancellation remain enforced by the
owned boundaries. Real pending I/O remains pending. This rule prevents a
cooperative-yield spin during metadata-filtered native revsets.

Application query stores preflight V1 metadata with an allocation-free
collection visitor before native decoding. Default admission permits 4,000,000
JSON values and map keys cumulatively, 65 terms per persisted merge, 256
parent/predecessor edges per value, and 50,000 entries per tree. JSON nesting is
bounded separately. Native graph/content limits carry this policy and report the
cumulative scanned node count. The native persistence codec remains lossless for
shapes outside these query budgets. Derived native merge expansion and aggregate
memory still require separate qualification before shared HTTP exposure.

Revset file predicates preflight recursive merge shape using the pinned native
ancestor-selection order. The preflight tracks counts rather than constructing
merged trees, bounds intermediate terms before allocation, and reserves work for
both preflight and native predicate evaluations. This policy leaves normal graph
queries intact. Predicate file bodies have per-file/cumulative byte caps and
reserve cumulative squared line work before native file materialization. Binary
streaming through the content API has independent limits.

Read-only query authority also admits each logical backend tree path before
native traversal, including virtual trees. Paths are limited to 64 components
and 4096 bytes. Ordinary native sessions keep their existing authorization; this
query guard does not alter persistence or physical client workspaces.

Returned native file readers also cross the caller-owned runtime adapter on
every poll. The foreign-executor cooperative-budget rule applies after file
preparation as well as during backend futures and copy streams; genuine pending
local I/O remains Pending.

A serving instance composes its graph executor and content reader with one
shared interactive worker pool, retained across repositories, routes and clones.
Explicit index preparation uses one additional shared background slot and MUST
NOT consume configured interactive capacity, including when that capacity is
one. Another preparation returns Busy even across distinct cache namespaces.
Interactive requests cannot borrow the preparation slot. Admission is
non-queuing and returns Busy when the relevant capacity is exhausted. A worker
retains its slot through native execution, including canceled synchronous steps.
A verified file retains its slot with its spool until the handle is dropped,
including after EOF. Validation, authorization and worker failures return their
slots without weakening current permission or snapshot membership checks.

The composed executor exposes total capacity (interactive plus preparation) and
active-slot diagnostics without credentials or repository data. This bounds
simultaneous native jobs/streams; per-query byte/work/shape limits remain
independent. It is not an OS memory reservation. Snapshot admission, response
delivery, native RPC and background jobs also need explicit composition at the
serving boundary.

## Explicit index preparation

Graph execution has a separate inline cold-build threshold (1000 indexed commits
by default) within the hard index-admission budget. A missing or damaged cache
stops with IndexRebuildRequired as soon as verified commit reads reach that
threshold, and discards incomplete staging. A zero threshold rejects before
construction. A verified warm cache remains usable. An explicit preparation job
loads the exact selected operation and builds/checks the native index without
evaluating a user revset or publishing repository state. Its report binds the
operation and local cache outcome; it does not pin that cache against later
eviction or loss.

Preparation uses the same byte, shape, work, index-count and elapsed bounds as
graph execution, with its reserved shared background slot. It can bypass only
the inline threshold. An owned cancellation signal stops pending waits and is
checked around synchronous native steps; awaiting preparation returns only after
the worker exits. The service retains task identity/status and fresh request
authorization separately.

A verified file may select one exact byte range before its first chunk.
Selection uses the already-verified spool, retains authorization/deadline/worker
ownership, and cannot seek again or reset a consumed stream. HTTP range
normalization stays in the transport; native content verification is unchanged.

Owned native metadata and graph/content DTOs supply schema traits to the HTTP
contract generator through service exports. Native metadata schema names are
prefixed to distinguish them from query presentation types. Schema derives do
not reorder fields, change Serde attributes or modify native payload identity.
No upstream JJ type enters generated application contracts.

Typed command preparation owns native transactions in this crate. It stages an
unpublished operation under a bounded authority that cannot publish heads.
New/Describe and bookmark updates use pinned native helpers; descendant rewrites
and logical workspace/bookmark effects are reported from the resulting view and
operation predecessor map. Physical working copies and user configuration do not
participate. Read-only query stores retain their unconditional write denial;
command staging adds only its own immutable native objects to read membership.

Prepared command reports can be decoded from manifest-selected result receipts;
these DTOs carry no native session or publication capability. Command shape
validation is shared with canonical intent construction before serialization.
Native execution still runs anew for a noncommitted apply and stages only
immutable objects until guarded publication.

EditFile loads the exact ordinary source-file term from one visible commit named
by a single-target local bookmark. It preserves the native executable bit and
copy identity, rejects a rename over an existing destination, writes the
replacement tree, creates one child commit and moves only that bookmark in the
same unpublished transaction. It does not rewrite descendants, update a physical
working copy or publish the staged operation. Current authorization is rechecked
around immutable object writes, and the guarded mutation layer remains the only
publication boundary.

Explicit typed reconciliation uses RepoLoader::merge_operations on the exact
current heads, with the requested base first and the other heads sorted by full
ID. Native common-ancestor discovery, recursive ancestor merges and descendant
rebase semantics remain upstream-owned. The returned operation remains
unpublished. Report comparison is against the explicit base view. Preflight
loads the bounded reachable operation ancestry and reserves its squared node
count times head count from the command work budget before native merging.
Requests exceeding these conservative bounds fail; limits are not throughput or
production CPU qualification.

Typed abandon follows the pinned abandon command's native descendant-transform
and RewriteRefsOptions behavior. Target visibility comes from the native index;
hidden committed targets are skipped and absent targets fail. Native reference
rewrites create replacement working-copy commits and update conflicted
bookmarks. Reports retain exact abandoned/hidden IDs without deleting native
objects. New optional empty-skipped report fields preserve older V1 receipt
canonical bytes.

Typed conflict resolution shares read-side native term construction and
fingerprinting. A FilesMatcher selects exact conflict leaves; a complete
preflight validates every path before MergedTreeBuilder changes the tree. Native
rewrite_commit/rebase_descendants maintain change identity, references and
evolution. Ordered term selection retains file/copy/mode, symlink, submodule and
directory values. It never invokes external conflict tools.

UploadedFile resolution imports only service-checked upload files into its
private command read inventory. The executor verifies their complete native
content before tree writes and uses explicit executable/copy-term choices.
Ordinary read inventories remain committed-only. Mixed uploaded/term/deletion
batches share one original tree preflight, native descendant rewrite and guarded
publication. No upstream trait or native codec changes are needed.

Conflict-resolution impact reports enumerate native remaining paths on every
resulting rewritten or created commit under the same command budget and current
authorization. Sorted target/selected-path and per-result conflict data remain
owned DTOs. An optional empty-skipped report extension preserves canonical bytes
for older Prepared/StoredResult V1 receipts.

## Bounded browser reads

Reference lookup exposes native bookmark merge terms and visible changes without
inventing a default bookmark or deleted history. Recursive path reads stop after
the requested window and report unavailable last-change metadata when merge or
work budgets prevent exact attribution. Operation grouping uses recorded
predecessors first, preserves ambiguous lineage and requires explicit parent
selection for merge operations. Diff summaries retain exact before/after terms,
copy and rename sources, conflicts and binary/oversized reasons; selected file
content remains a separate verified read. These read-only queries cannot stage
objects, reconcile heads or publish repository state.

Native RPC V1 snapshot messages include the repository code-state version in
addition to epoch, global publication position and heads. Administration clients
round-trip the field so collaboration-only WAL activity does not make a strict
native policy or code request stale. A missing field decodes as zero for
pre-feature canonical snapshots; new current snapshots always return the
selected value.

Native RPC V1 also exposes bounded ReviewStore list, detail, command and status
methods for the bound repository. Wire messages contain repository and stable
identity bytes plus bounded canonical JSON encoded from service-owned models;
the RPC layer does not import the review model in production or expose WAL
coordinators. LocalTrusted calls retain owner-peer verification and
authenticated calls retain bearer policy. Cursor pins, command authorization,
receipt replay and outcome classification remain service responsibilities.

## WorkStore service and transports

The native WorkStore RPC uses the same authorized service as HTTP, including
pinned collection pages, explicit query-bound opaque continuations and durable
apply/status. CLI project/task commands use the bound local native service and
do not require an upstream hub.

## Exact Git representation conversion

Git conversion remains an owned adapter in this crate. It uses a private pinned
GitBackend as a conversion tool and translates explicit parent/tree/file IDs
into native identities. Original raw bytes and required conversion evidence
remain native durable dependencies. Imported metadata is validated before the
pinned decoder can normalize or replace it. Cache removal must not reidentify
accepted imported commits or their exported children.

The native Git RPC accepts typed records and optional native head transitions,
never an arbitrary Git command, endpoint, selected principal or raw manifest.
The service verifies conversion evidence using the pinned integration crate and
rechecks repository write access before publication. The CLI may bind a pending
Git delta to its existing request-scoped OpHeadsStore publication so normal JJ
transaction/rebase/workspace finishing performs only one acceptance.

An invocation-owned factory registry shares the native client and a scoped
pending Git delta across backend/operation/head adapters. The pending guard is
installed only around the Git command's final transaction. Snapshot and ordinary
head reconciliation remain ordinary native publications, and the guard clears on
success, failure or cancellation. No global principal or pending request is
introduced.

Git push applies only the frozen remote bookmark target and tracking state. It
never resets a local bookmark to a previously uploaded target. The invoking
transaction retains JJ operation/rebase handling and concurrent sibling heads;
causally incomparable remote observations remain durable conflicts. Native undo
can change its code view while retaining external attempt evidence in the Git
root. Private conversion rebuilds the selected closure from retained mappings
and original bodies before any transport-started event.

The native operation reducer also owns data-only verification of imported Git
bytes, mappings and frozen bookmark targets. A new imported push intent proves
its source against accepted published code history; the replica inventory alone
cannot authorize it. Verification uses isolated conversion scratch state and
never invokes Git transport or reconciles native heads.

Git recovery consumers merge the frozen native remote-ref base, current target
and confirmed result with the pinned `merge_ref_targets` algorithm. The frozen
base must match the durable Git lease; missing or ambiguous evidence refuses
apply. A later observation cannot be rolled back by repairing an older attempt.
The Git stage fault fixture exercises before/after manifest-write uncertainty
for activation, intent, started, result and applied facts through cold replay.

## M15 companion envelope extension

The owned local ref retains its existing name. Version 2 adds Thread and
Reaction domain selections and typed batch updates. A batch checks every exact
domain selection before preparing one metadata commit/ref CAS and retains a
single original receipt naming all changed selections. Version 1 receipts and
commits remain byte-identical and readable. All selected domain blobs and
retained immutable object blobs have actual Git reachability; hashes in JSON are
not retention. No ordinary Git transport or code operation is implicit.

## Filesystem browser reads

The ordinary Git-backed reader uses only server defaults and a private device
binding. It checks bounded operation ancestry before `RepoLoader::load_at`, then
checks the selected ancestry and predecessor closure before exposing code. Git
SHA-1 objects and the virtual root retain their source identities. Content reads
prove file membership through selected commit trees. Queries run in admitted
cancelable workers with limits on metadata, work, bytes and time. Repository
aliases, executable settings and content-predicate revsets are not loaded.
Ancestry traversal uses upstream parent identifiers directly, retaining
membership and work limits. Missing, unrelated and ambiguous selections fail
explicitly. Reads may rebuild derived JJ indexes but cannot publish heads, refs
or checkout snapshots.

The local query transport serializes the same owned request variants and output
records used by the qualified reader. Server composition shares query workers
with native reads; transport continuations cannot supply unchecked raw anchors.

## Rich history query delivery

Operation-pinned history reads MUST expose the pinned JJ 0.45.1 callable catalog
and standard aliases independently of device or server user configuration.
Commit revsets use explicit workspace, author email and frozen time context;
`mine()` requires email and `@` requires a workspace. Native content and
cross-operation predicates retain their existing admission and publication
restrictions.

Operation metadata queries MUST filter the full bounded selected ancestry before
pagination, bind exact query text into continuations, charge work for every
scanned operation and retain actual parent IDs and native ordering. Spaces and
explicit AND combine `description:`, `user:`, `host:`, `workspace:`, `id:`,
`after:`, `before:`, `snapshot:` and one `from:` selector; bare text matches
descriptions. Quoted text supports escaped quotes and backslashes. Text matches
case-insensitive substrings, IDs match hexadecimal prefixes, dates use UTC
`YYYY-MM-DD` against end time with inclusive after and exclusive before, and
snapshot accepts true/false. `from:` accepts JJ operation selectors only within
the displayed snapshot's ancestry. OR, NOT, commit revsets, missing values and
malformed selectors fail explicitly. Queries are bounded to 4096 bytes and 64
terms. Diagnostics are bounded and expose no storage or credential details;
these reads never reconcile heads or publish state.

LocalTrusted query metadata MAY supply a bounded default author email from host
JJ configuration, including JJ_EMAIL. Registered local folders fall back to
repository JJ then Git configuration when the host has no usable email.
Configuration reads MUST remain read-only, expose only the email and never load
aliases or other configuration into query evaluation. Authenticated native
serving MUST omit host identity hints; explicit request context remains the
query authority. Secure repository fallback requires a metadata binding that
resolves to the exact registered metadata directory.

## M18 opt-in checkpoint coordinator

The [M18 provenance profile](collaboration.md) keeps working-copy snapshots in
the client JJ boundary. Transcript ingestion alone does not snapshot or
reconcile heads. Explicit per-workspace enablement admits baselines,
before/after/failure scopes and exact operation evidence through existing JJ
transactions and locks. Preexisting dirty work remains a separate baseline;
no-change observations do not manufacture code operations. Immutable provenance
intents avoid hash cycles and late associations never rewrite published
operation identities.

## Query membership capabilities

Owned read access separates verified point membership from explicitly bounded
inventory enumeration. Content and object guards use point membership; graph
preparation and legacy complete enumeration request their own bounded inventory.
A request-local membership cache is bounded and never retained in read cursors.
Every path rechecks current authorization and preserves the captured catalog
root and native operation. This changes no persisted JJ or WAL encoding.

Local Git-backed metadata reads select the retained operation and view directly.
They MUST NOT require upstream index construction or complete commit/tree
preflight. Bookmark-only unfiltered pages hydrate only the selected target
window. Every hydrated object retains bounded bytes and proven reference
membership under the explicit operation, with authorization checked before
returning. Graph-dependent reads keep their own complete-result admission.

Scoped local directory traversal MUST wrap upstream Store reads, rather than
remove eager preflight while leaving unrestricted backend access. Native and
local nonrecursive path pages load only the selected directory's tree terms and
include directory entries. They MUST NOT descend into unrelated child trees to
construct a direct-child page. Conflict terms retain upstream trivial-resolution
semantics; deeper content remains separately admitted.

Optional path attribution is a separately budgeted request. Within a window the
reader shares one parent-chain traversal and retains upstream path-value and
merge semantics; exhaustion reports unavailable attribution without invented
last-change commits. Lightweight local commit headers expose native commit data
without requiring visible-change/evolution evaluation. Scoped file content
requires exact commit/path/file agreement before opening a stream.

Workspace references are projected directly from the pinned view's workspace
map. Shared targets are hydrated once per window; target annotations on a
workspace row are scoped to that named workspace. Exact name lookup does not
invoke substring matching or scan unrelated commit descriptions. Legacy visible
change enumeration retains its graph semantics and bounded admission.

Operation log windows without an ID-prefix selector MUST walk bounded operation
metadata directly, without loading commits, trees, views or a commit index.
Native responses report `not_used` for the graph cache. Operation ordering,
parent/child selectors relative to `@`, filtering, pinned membership,
authorization and cancellation retain pinned JJ semantics. ID-prefix selectors
retain their existing complete admission until the prepared operation lookup is
qualified. Local reads must not repair or rewrite a damaged commit index as a
side effect of reading operation metadata.

Local historical point reads use a read-only edge proof rooted in the selected
operation and its ancestor operation/view references. They stop at the selected
commit and retain only bounded IDs during proof, without loading trees or a
commit index. Commit and file values still cross the scoped backend admission
and identity boundaries. This does not change graph evaluation or publication
closure validation.

Native graph and preparation admission MUST NOT enumerate the complete committed
catalog to count commits. Admission charges distinct commits actually read,
including the virtual root, before fetching their physical bytes. The hard
index-size limit also applies to every loaded warm index and additional
operations loaded by query predicates. Cold graph loads apply the inline limit;
explicit preparation retains the hard limit. Errors retain their typed budget
cause through upstream wrappers, and no incomplete index is retained as ready.
Request syntax work is charged from the query itself rather than total retained
commit count multiplied by query length. Authorization, object/metadata/returned
row work and deadlines remain charged at their existing boundaries. This does
not qualify uninstrumented synchronous upstream CPU/memory work or remove the
hard index-size limit. File-predicate preflight, copy lookup and
operation-prefix lookup retain their separate complete-enumeration paths pending
prepared lookup.

Explicit native commit-index preparation MUST keep retained operation IDs,
commit graph fields, visited sets, ordering queues and sorted lookup tables in
disposable disk-backed scratch storage. It reads each distinct commit directly
through the authorized backend without populating the upstream commit-body
cache. Scratch uses an 8 MiB page cache and transactions of at most 512 table
writes, with a durable scratch checkpoint at least every 16 batches to bound
pending transaction bookkeeping. Per-object metadata and parent limits remain
enforced. Preparation charges traversal, ordering and serialization work and
checks cancellation between records; it never loads trees for commit indexing.

Native explicit preparation uses a separate trusted policy: at most four million
commits, 500 million work units, 16 GiB cumulative object bytes, one billion
metadata shape nodes and one hour. Per-object decoding and parent/merge limits
remain enforced. Ordinary page work, bytes, metadata and deadlines remain
unchanged; a loaded index may contain up to four million commits without
charging every retained commit to a page. A stricter configured index ceiling
still applies to preparation and warm reads. Requests cannot override these
policies. These ceilings are admission bounds, not scale-qualification evidence.

The derived segment writer MUST preserve JJ 0.45.1 format-6 bytes and the cold
builder's reverse-topological timestamp/commit-ID ordering, including parent
overflow and divergent-change lookup. It streams output under a shared
scratch/output allowance reserved from the cache entry before file growth,
including buffered bytes. Complete segments are synced before their operation
links become visible inside the unpublished cache entry. Failed scratch is
discarded. Existing hard index limits, cache receipts and native persisted
formats remain unchanged by this construction stage. Large resident indexes,
immutable generation reuse, aggregate quotas and local-profile preparation
require their separate qualification; this stage does not enable them.

Native and Git-backed local preparation share traversal and segment writing
through profile-specific compact readers. The builder retains only parents,
change IDs and committer timestamps from each commit. Native readers keep the
existing capability and metadata admission; local readers retain the distinct
read-only Git/extra-metadata contract in `local-jj.md`. Neither profile may
substitute an unchecked source index or expose upstream objects through service
contracts. This shared construction does not change native segment bytes.

Warm native graph reads MAY reuse an already verified immutable in-memory index
under the same cache namespace lease. Retained residents bind repository, epoch,
operation, cache path and exact receipt file set; they have an LRU count bound
and a conservative shared process-wide allocation allowance of 1 GiB. Before
reuse, enumerate the bounded file set and compare device, inode, size, mtime and
ctime at nanosecond precision. Filesystems without the required metadata use
full verification. Changed, replaced, missing or added files invalidate reuse. A
metadata comparison only authorizes using the existing verified in-memory index;
it MUST NOT substitute for checksumming disk bytes that will be decoded.
Additional operation indexes retain full disk verification. Process restart,
eviction and loss of the resident require full checksums before loading. Current
authorization and loaded-index hard limits still apply. Disk-generation
immutability, shorter namespace locking and larger aggregate quotas remain
separate work.

Native disk-index loads and local prepared generation loads MUST reserve
resident allocation before upstream decoding. Account twice the complete bounded
segment file bytes, per-file metadata and fixed overhead. Check pinned
commit-index format-6 and changed-path format-0 framing, bounded parent filename
length, exact table lengths, at most 4,096 files and parent chains of at most
128 segments before entering JJ's allocating parser. Cycles and excessive counts
fail closed. This framing check does not replace checksum verification. Native
inline construction retains ordinary query admission and accounts its resulting
segment weight before retention; construction peak admission remains a separate
gate. Allocation leases are shared with retained indexes and live readers across
executor instances and both profiles; dropping a cache owner cannot release
memory still owned by a reader. This allowance accounts resident index
estimates, not the whole server's heap or storage-library caches.

An ordinary warm `all()` page MAY release the cache namespace lease after
loading and validating its exact operation, view and resident index and
finishing cache bookkeeping. Evaluation then uses the retained immutable memory
copy and authorized native object reads. Another job may evict or replace its
disk slot without changing the in-flight page's source. Cache diagnostics in
that response describe the admission-time observation. Resident byte accounting
MUST retain charges until the last executing owner releases an evicted copy.
Queries that can load additional operation indexes retain their exclusive
namespace lease; this optimization does not authorize concurrent disk writers.

When preparation already holds that lease, a new ordinary `all()` page MAY use
an existing resident matching repository, epoch and operation without opening
any cache metadata or index files. Construct fresh request-scoped native backend
and operation capabilities; never retain or reuse an earlier caller's
authorization. The memory-only index store admits exactly the captured
operation, enforces the current hard limit and denies writes. Cold or
graph-dependent reads without a matching resident retain Busy/preparation
admission. Cache retained counts on this path describe the most recent completed
cache observation for that resident; the request performs no eviction.
Revocation, epoch changes, cancellation and final output checks remain current.

The shared native query-cache namespace defaults to 2 GiB per entry, 4 GiB per
repository across all epochs/operations, and 16 GiB total. Cold preparation MUST
reserve its entire entry allowance before creating staging. Existing receipts,
scratch, buffered segment output and the final receipt consume that allowance;
staging is never extra capacity. Repository pressure evicts that repository's
least recently used entries before unrelated repositories. Total pressure uses
global LRU order. Opening a namespace with a reduced policy reclaims excess
retained entries before admitting work. A warm read which creates no files does
not reserve a new entry or evict unrelated pages. These quotas govern one shared
serving namespace; independent standalone executor roots remain independently
configured. General upstream inline writes and additional-operation rebuilding
retain post-build size verification pending guarded immutable generations.

Local prepared log reads reuse only checksum-verified immutable index data and
build fresh operation/backend capabilities for the current binding. Index
membership admits commit decoding without a complete source inventory. Source
extra metadata stays read-only, with bounded seeks and change detection; no
implicit Git import, retention-ref write or source-index fallback is permitted.
The local profile owns source verification details and shares resident
allocation admission with the native profile.

Local prepared generations MUST reserve their full writable allowance in the
same cache namespace as native retained entries before construction. The shared
namespace lock admits reservations briefly; independent lifetime locks preserve
charges across processes without holding up interactive cache access. Count all
live reservations in per-repository, aggregate and entry limits, including
staging and descriptor overhead. Never evict a live external reservation; native
LRU entries may be reclaimed to admit it. The final reader retains the charge.
Interrupted reservations are discarded after their lifetime lock becomes free;
no partial generation is adopted as repository authority. A report's accounted
bytes include conservative live reservations, even before their files grow.

Reserved local preparation MUST move the quota handle into the actual worker and
then into the prepared result. Dropping a requesting future, task or cache entry
cannot release disk admission while another worker/reader still owns the derived
files. The preparation cancellation token covers initial authorization as well
as blocking traversal, and remains effective while the worker owns its permit.

Local scoped commit and reference hydration MUST NOT call upstream GitBackend
commit reading, which can import missing extras and reconcile stacked tables.
The owned decoder shares signatures, shallow-root, conflict and extra-field
semantics with preparation. Interactive source point reads bound table framing
and keyed seeks instead of loading whole extra segments. This source-format read
does not replace full checksum admission for a retained prepared generation.

Companion identity observation verifies only the current selected root and its
declared Git reachability framing. Its distinct owned Identity type cannot be
used for domain reads or publication. Existing Snapshot admission still verifies
domain objects, previous selection and retained evidence closures. Local folder
checks and code freshness polling must not repeat those unrelated audits.

The scoped local content capability also covers selected from/to diffs,
changed-path pages and conflict descriptors. Its two-target proof follows exact
operation/view references and commit edges under ordinary admission, then reuses
the owned decoder and scoped tree/file grants. It never promotes raw Git object
existence to membership or falls back to complete repository preflight.
