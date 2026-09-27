# Native JJ integration

Commit ancestry, operation ancestry, evolution, and publication order are four
separate histories. Multiple operation heads are legal. CLI repository opening
can use native reconciliation, but HTTP GET cannot. Backend copy lookup and
submodule data need full coverage beyond the reference SimpleBackend.

## Requirements

| ID         | Requirement                                                                                                                                                                          |
| ---------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| JST-JJ-001 | Custom CliRunner factories MUST implement the exact pinned Backend, OpStore, and OpHeadsStore contracts over RPC, with equivalent in-process stores on the server.                   |
| JST-JJ-002 | Only guarded OpHeadsStore publication MUST advance authority; request-scoped principal, intent, snapshot, and receipt data MUST NOT use global mutable context.                      |
| JST-JJ-003 | Read sessions MUST pin explicit operations through load_at and MUST NOT reconcile heads; authorized mutation sessions MUST use native transactions and finish descendant rewrites.   |
| JST-JJ-004 | Physical checkout bookkeeping MUST stay client-local; logical workspace IDs MUST be distinct across unrelated clients and stale-workspace updates MUST preserve unsnapshotted edits. |
| JST-JJ-005 | Indexes and prefix lookup MUST be bounded, rebuildable, and versioned by native profile; normal required methods MUST NOT contain silent stubs.                                      |

## M0 extension qualification

M0 probes register a request-scoped OpHeadsStore through StoreFactories,
preserve per-principal publication IDs and exact preconditions across real
loopback gRPC, and load explicitly pinned divergent operations in a bounded
local async task. The RPC fixture is test-only and does not implement durable
publication or policy.

## Native codec boundary

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
annotations, logical workspaces, visibility and divergence. Evolution
distinguishes absent predecessor records from known-empty records.

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
retained bytes. Defaults retain at most 16 slots, 64 MiB of file bytes per slot,
256 MiB in total, and 4096 filesystem entries per slot. These are admission and
retention limits, not measured throughput guarantees. Least-recently-used slots
are evicted under the namespace lock; failed builds and abandoned staging
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
shared worker pool, retained across repositories, routes and clones. Admission
is non-queuing and returns Busy when the aggregate count is exhausted. A worker
retains its slot through native execution, including canceled synchronous steps.
A verified file retains its slot with its spool until the handle is dropped,
including after EOF. Validation, authorization and worker failures return their
slots without weakening current permission or snapshot membership checks.

The composed executor exposes capacity and active-slot diagnostics without
credentials or repository data. This bounds simultaneous native jobs/streams;
per-query byte/work/shape limits remain independent. It is not an OS memory
reservation. Snapshot admission, response delivery, native RPC and background
jobs also need explicit composition at the serving boundary.

## Explicit index preparation

Graph execution has a separate inline cold-build threshold (1000 retained
commits by default) within the hard index-admission budget. Above that
threshold, a missing or damaged cache returns IndexRebuildRequired before native
index construction. A verified warm cache remains usable. An explicit
preparation job loads the exact selected operation and builds/checks the native
index without evaluating a user revset or publishing repository state. Its
report binds the operation and local cache outcome; it does not pin that cache
against later eviction or loss.

Preparation uses the same byte, shape, work, index-count, elapsed and shared
worker bounds as graph execution. It can bypass only the inline threshold. An
owned cancellation signal stops pending waits and is checked around synchronous
native steps; awaiting preparation returns only after the worker exits. The
service retains task identity/status and fresh request authorization separately.

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
