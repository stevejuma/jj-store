# Query membership capabilities

Owned read access separates verified point membership from explicitly bounded
inventory enumeration. Content and object guards use point membership; graph
preparation and legacy complete enumeration request their own bounded inventory.
A kind-specific owned lookup admits only matching operation or copy identities
against the read capability's original checked manifest, including explicit
preparation pins. Unrelated descriptors consume scan work, not matching-result
capacity. It shares source-scoped immutable kind projections while retaining the
caller's result bound and original monotonic deadline. Ordinary reads keep their
page result ceiling; explicit preparation uses its own caller and trusted
session bounds, independently of the initiating page's capacity and expiry.
Later publications must not replace a historical browser pin. Compatibility
providers may filter their own bounded complete inventory; they reauthorize and
reject exhaustion or elapsed time rather than return an incomplete result.
Read-only upstream operation/copy authorities use this lookup and check their
query budget again before delivering identities. Provider failures retain their
typed resource or authorization cause through upstream JJ error wrappers. This
does not qualify relevant copy-body/path indexing or resumable cold
construction. A request-local membership cache is bounded and never retained in
read cursors. Every path rechecks current authorization and preserves the
captured catalog root and native operation. This changes no persisted JJ or WAL
encoding.

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

Quick path attribution uses one shared first-parent walk through jj-lib commit
and tree APIs, with at most 64 history steps for the visible page. File values
and directory tree IDs are compared against the first parent; integration merges
are valid path changes. Each relevant directory tree is read once per visited
commit, and resolved rows leave the scan. No merged-parent reconstruction or
file-content comparison is required for this presentation. Rows unresolved at
the scan bound return the selected commit's real metadata with
`attribution: snapshot`; discovered changes use `path_change`. These are
terminal quick results, never invented file-change facts. Ordinary
authorization, operation membership, cancellation, corruption and resource
errors remain errors, not snapshot fallback. Names-only reads do no ancestry
work. Files attribution MUST NOT schedule changed-path preparation. Explicit
history mode reuses the existing core-index JJ file predicate and bounded
resumable frontier (up to 64 candidates and 1024 path evaluations per batch).
This mode can prepare the core index, but does not schedule changed-path
indexing. Ready core indexes may still prove historical commit membership. Core
graph preparation and separately requested advanced queries retain their
contracts. Local commit headers and file content retain exact selection and term
checks.

JJ's clean merged-parent comparison may synthesize file content. Read adapters
retain these blobs only for the current request, under fresh read authorization,
existing work/byte/deadline limits, per-file admission, at most 4096 blobs and
64MiB total content. Allocation leases follow returned readers. Identities use
the original backend hash (Git SHA-1 blob framing or native Blake2b-512
content). Synthetic content never writes source objects, indexes or repository
authority, and does not grant access to unrelated content. Ordinary backend
writes retain their existing mutation authorization and publication contract.

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
hard index-size limit. File-predicate preflight, related copy-body/path lookup
and uncached operation-prefix construction retain separately bounded complete
admission pending prepared lookup. Operation/copy identity enumeration uses the
pinned kind provider, independently of unrelated inventory result capacity.

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
after primary cache admission. Retained residents bind repository, epoch,
operation, cache path and exact receipt file set; they have an LRU count bound
and a conservative shared process-wide allocation allowance of 1 GiB. Before
reuse, enumerate the bounded file set and compare device, inode, size, mtime and
ctime at nanosecond precision. Filesystems without the required metadata use
full verification. Changed, replaced, missing or added files invalidate reuse. A
metadata comparison only authorizes using the existing verified in-memory index;
it MUST NOT substitute for checksumming disk bytes that will be decoded.
Privately constructed additional-operation indexes require full checksums and
exact operation-link/segment linkage before decoding. Process restart, eviction
and loss of the resident require full checksums before loading. Current
authorization and loaded-index hard limits still apply. Missing primary
generations use separately reserved staging, then reacquire the namespace to
install the complete generation. Hash private output before reacquisition; where
precise file stamps are supported, installation rechecks the sealed file set and
stamps without hashing large segments under the lock. Changed sealed files fail
closed; other platforms retain full verification. Reacquisition retains
cancellation, work and deadline checks; it cannot overwrite another generation
admitted during construction. Existing verified disk-index loads without a
resident still retain the namespace through decoding and projection
construction; separating this recovery path remains delivery work.

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

Every graph request MUST release the cache namespace lease after loading and
validating its exact operation, view and primary index and finishing cache
bookkeeping. Evaluation then uses the retained immutable memory copy and
authorized native object reads. Another job may evict or replace its disk slot
without changing the in-flight page's source. Cache diagnostics in that response
describe the admission-time observation. Resident byte accounting MUST retain
charges until the last executing owner releases an evicted copy. The loader
retains primary data and its own allocation handle across subsequent loads,
including after a staging rename or disk eviction. Evaluation MUST NOT extend
the primary generation with additional operation files.

Additional native operation loads MAY reuse a separately retained prepared
resident under the exact repository/epoch/operation key. The operation and view
still cross the current request's native capabilities before that lookup, and
the index reader rechecks current authorization and hard commit admission.
Retain the saved allocation leases on each fresh request reader; its separate
generation accounting does not acquire the secondary reader's allocations.
Deduplicate shared parent handles when a generation includes earlier index
allocations. This path does not read, hash or copy another cache slot and does
not add its segments/links to the primary generation. Preserve each operation's
own visibility, prefixes and annotations; a current-view index is not a
substitute for a historical one. Missing historical operations acquire a private
locked reservation in the shared native/local namespace before construction. Its
complete allowance stays charged until worker cleanup; acquisition releases the
namespace lock before building. Use the ordinary inline threshold, compact
admitted scratch and owned format-6 writer. Verify the exact two-file output,
selected operation link and content-addressed segment, then admit frame memory
before JJ decoding. Derive visibility and annotations from that operation's own
view. After readonly arrays are fully owned, discard construction files and
retain only immutable data in a count/weight-bounded auxiliary LRU sharing the
process allocation pool. Cache entries carry no caller or native capability.
Primary slot corruption and missing-projection recovery keep their ordinary
admission; a secondary resident cannot repair or override that selected slot
implicitly.

When preparation already holds that lease, any graph request MAY use an existing
resident matching repository, epoch and operation without opening any cache
metadata or index files. Construct fresh request-scoped native backend and
operation capabilities; never retain or reuse an earlier caller's authorization.
The memory-only index store admits the captured operation and loads additional
operations through the same guarded historical reader, enforces the current hard
limit and denies writes. Cold primary reads without a matching resident retain
Busy/preparation admission. Cache retained counts on this path describe the most
recent completed cache observation for that resident; the request performs no
eviction. Revocation, epoch changes, cancellation and final output checks remain
current. A live reservation may evict primary disk data and prevent new disk
admission; its matching immutable resident MAY still serve the request. Detected
cache damage MUST retain the normal recovery path. Memory fallback does not
waive missing historical construction admission or follow another caller's
source.

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
configured. Missing native operation indexes, including additional operations
selected by predicates, use the owned format-6 constructor. Interactive
construction uses compact ordered scratch capped at 64MiB, charging the shared
resident pool before copying keys/fields or growing tables; it retains no commit
bodies and enforces the inline commit threshold before adding nodes. Small
explicit preparation uses the same scratch when its writable allowance is below
32MiB. Larger explicit preparation keeps bounded disk-backed scratch. Both paths
reserve streamed segment/link bytes before writing and admit the actual receipt
bytes before creating its temporary file. A small cache does not require a
fixed1MiB receipt reservation or a database's minimum allocation. Framing and
resident decoding remain admitted before JJ allocation. Primary and additional
constructors release the namespace after registering their full reservation.
Primary completion reopens it, replaces its own reservation with admitted final
bytes and selects a currently free slot. A concurrently installed generation for
the same exact pin may be replaced with this independently verified generation;
other pins remain governed by ordinary LRU quotas. Failed or canceled workers
retain the reservation until actual cleanup. Additional operations retain
verified data only and never extend the primary slot.

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
