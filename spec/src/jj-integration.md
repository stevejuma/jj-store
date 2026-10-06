# Native JJ integration

## Progressive file blame feasibility gate

The proposed file Blame reader uses the pinned JJ `FileAnnotator` at an explicit
operation and commit, with same-path, merge-aware semantics. Before exposing
live routes or enabling attribution, qualify incremental topological domains
against complete JJ annotation and measure Linux through the existing read-only
local adapter. Every successful window must preserve unresolved origins as
unresolved; a snapshot or an interrupted computation is never a proven line
origin.

The feasibility slice must establish bounded history discovery, historical file
reads, comparison work, retained memory and cancellation. A small output or a
64-commit domain alone does not establish those properties. Warm progress
responses target five seconds, independently from complete-attribution time. If
the upstream API cannot satisfy the gate with existing readers and indexes,
record the blocker and stop before adding public contracts or enabling
attribution; reader/index redesign and an upstream annotation fork require a
separate scope.

The initial gate is stopped at JJ 0.45.1's synchronous line comparison. A
one-commit domain over two 240,000-byte files with 40,000 reverse-ordered unique
lines ran for 6.85 seconds in the development test profile and returned success
after a surrounding one-second Tokio timeout. `FileAnnotator::compute` calls
`ContentDiff::by_line` internally without a budget/cancellation callback;
neither a smaller ancestry window nor the outer timeout admits or interrupts
that work. The existing bounded line-diff helper is not used by annotation. Do
not expose the proposed reader until comparison admission and cancellation are
qualified. This finding is distinct from the successful Linux first-window
latency probes; complete Linux attribution, server retention and HTTP/UI
behavior are unqualified. The separately authorized UI slice may use fixture
providers to verify presentation and navigation while keeping production Blame
disabled; it does not satisfy this reader gate.

## Existing native read behavior

The local read capability keeps authoritative `authorize` at request and task
boundaries. Its default inner-loop `check` also authorizes; server composition
may implement it using a one-second validated binding cache with synchronous
registration invalidation. Scoped object membership is checked independently for
each read. The worker validates again before exposing its result; cached checks
cannot bypass cancellation or resource budgets.

Selected native and local text comparisons share service admission around the
pinned JJ default line-diff implementation. Exact common boundaries and ordered
unique common lines may split larger inputs into bounded native comparisons; the
aggregate comparison-work ceiling, request cancellation, byte verification,
output limits and explicit unavailable reasons remain enforced. This is a
rebuildable read projection and changes no native identity or stored format.

The additive CopyFamily RPC gives the ordinary remote native backend the same
selected-family capability as its in-process authority. Capture one checked
current inventory root after bounded admission; retain it for the entire call
and recheck current permission, epoch and deadline. Historical browser providers
continue to use their original pinned capability. Requests carry sorted unique
copy seeds with required flags and tightening node/work/scratch/time allowances.
Server-owned defaults, transport sizes and shared construction leases remain
ceilings. Return complete sorted unique selected identities and charged
traversal work, never bodies or a truncated success. Decode requests before set
growth and responses before result growth. Required seeds must appear in the
result. Only an older peer's explicit Unimplemented permits bounded inventory
compatibility; authorization, deadline, resource, missing-selection and
corruption failures remain errors. Client and server cancel pending I/O and
reject late success. Acknowledged staged relationships remain captured and
merged in the client backend, with no staged records entering the RPC
projection. Existing Inventory, InventoryPage and persisted formats retain their
exact framing.

Prepared operation history may retain a disposable compact DAG containing exact
operation identities, ordered parent identities and end timestamps, including
timezone offsets. Capture it during the existing bounded operation pass. Verify
selection, checksum, sorted identities, complete lookup membership and acyclic
parent edges under disk/work and resident admission. The pinned JJ topology
walker orders these compact nodes; no heuristic order may replace it. Ready
operation pages skip prior bodies before hydrating the anchor and predicate/page
window. Fresh source admission and canonical fingerprints remain mandatory for
hydrated metadata. Invalid or nonmatching anchors fail explicitly. Traversal
work remains bounded. Retain the captured selection's exact order and inverse
ranks during generation admission using the pinned compact topology walker. Seek
selected-start anchors through those ranks and visit only subsequent
predicate/page candidates. Charge complete traversal scratch before derivation
and the immutable order before growth; it shares the generation lifetime and
contains no source or request capability. Reject disconnected captured graphs.
Historical-start pages may retain at most32 data-only checkpoints per admitted
graph, keyed by exact resolved start and anchor. Use the pinned JJ chunk walker
with the complete frontier, pending chunk and visited bitmap. Reserve scratch
and retained growth before use; share immutable chunks and copy visited bits
only under admission. Checkpoints carry no source, request, filter or access
capability and share bounded process retention. A reused anchor is hydrated and
validated again. Missing or evicted checkpoints fall back to ordinary bounded
compact traversal; no partial page is represented as complete.

Prepared local operation graph admission compares fresh selected operation
metadata with the retained canonical fingerprint, without reading its view or
creating a scoped commit backend. Unfiltered and metadata-filtered pages reuse
the same ready identities and neighbors. The operation capability exposes no
view unless a separate code reader has already admitted that selected view.
Fresh access, source membership, binding, generation and metadata checks remain
mandatory. Code readers preserve the combined operation/view seal.

Local prepared operation comparisons retain canonical view fingerprints as well
as operation fingerprints. They read admitted historical metadata directly and
compare visibility through bounded immutable index edges, stopping at a shared
frontier. The compact graph decoder consumes the checksum-verified owned
format-6 segment, admits its resident bytes before allocation and validates
framing, sorted lookup, parent ordering and change offsets. Change-group paging
uses indexed change identities and hydrates only selected added commits. Neither
comparison repairs a source index, loads trees, invents rewrite relationships,
changes public DTOs or publishes state.

Native/local code-card summary readers consume only the selected operation/view
and bounded direct commit heads, excluding the virtual root. Their counts and
maximum committer timestamps describe that head window; unvisited heads or
nonvirtual parents force explicit incomplete status. They perform no ancestry,
graph preparation, source index repair or publication. Native manifest
membership and local scoped pointer proof retain fresh authorization and exact
pins.

The local stacks recovery boundary uses the owning provider's stable physical
workspace identity independently of current code. Identity admission reuses the
current working-tree observer's digest and bounded worker lifetime, checks
registered root/`.jj` directory identity across authorization, and never loads
operation stores, checkout/tree-state, ignore policy or working files. It cannot
initialize or repair state, establish original request ownership or enable a
command. The registered runtime must still enforce those separate authorities.

The private local working-tree baseline reader selects the named workspace's
commit from an exact authorized operation/view through the existing scoped Git
reader. It never loads latest state, repairs the source index, snapshots the
working copy or publishes heads. Complete bounded raw file/link digests and
modes are provider inputs, not a wire capability or evidence of current physical
checkout. JJ's ignore matcher consumes captured bytes; no upstream ignore or
working-copy type crosses into service policy. The current provider reads
bounded existing checkout/tree-state protobuf through descriptor-relative file
handles; it does not call the upstream working-copy loader or initialize absent
state. It proves exact operation/workspace/tree terms and labels, retains sparse
prefixes, and captures actual configured/default global and Git repository
exclude bytes. Metadata/policy/head repetition detects races; repository and
working-copy directory identity contribute to the private witness. Service code
uses owned sparse/ignore interfaces, never upstream protobuf or matcher types.

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
index repair, extra-table import or publication is permitted. Prepared local
operation prefixes resolve against sorted retained ancestor identities, without
commit-index access or a second full membership walk. Their read-only operation
store retains fresh metadata, cancellation and source admission, and rejects
foreign/future operations.

Prepared operation prefix reads compare historical source metadata against
captured canonical fingerprints because SimpleOpStore does not hash operation
filenames. Fingerprint mismatch fails explicitly. Prefix resolution and
membership charge logarithmic lookup work. Prepared local child suffixes seek
retained reverse edges and hydrate selected metadata. Native explicit prepared
generations now use the same identity/edge framing; cold and small-cache native
reads retain their bounded upstream fallback.

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

The local stack preparation retainer additionally selects exact archived
operation/view and code closures through create-only reserved refs under
`refs/lapis/stacks/prepared/v1`. Physical/request scope and original principal,
intent and preparation identity bind each immutable slot. Complete Git ancestry,
evolution and nonstandard tree dependencies use real retention edges before ref
selection; source operation heads, checkout and companion selection remain
unchanged. Read-only lookup never initializes a missing slot, and exact replay
validates the original archive and syncs either loose or packed selected refs.
Unknown selection is not rejection. This owns archival retention only, separate
from unpublished command generation, operation-ancestry recovery, head
publication and foreground checkout qualification.

The local execution coordinator uses a bounded, nonblocking foreground lease
compatible with pinned JJ's local working-copy lock. Descriptor-relative
no-follow opens, regular single-link identity checks and replacement-safe
release preserve the upstream Unix locking protocol without its indefinite
blocking acquisition. Only owned checkout witnesses and authorization interfaces
cross the boundary. Admission is retained while leased; post-lock and explicit
later metadata checks repeat the exact witness, operation heads and
directory/lock identities. There is no snapshot, reconciliation, code
publication or working-copy bookkeeping update. External editor bytes are
separately guarded by composition.

Local reword preparation consumes the foreground lease on an isolated blocking
worker and uses the pinned ordinary GitBackend transaction at its exact source
operation. No latest load, import or reconciliation is permitted. Before any
candidate write it checks the complete bounded source archive, visible parent
membership, supported unsigned tree records and command bounds. It completes
descendant rewrites and verifies the exact source/prepared archive, rewrite map,
local bookmark terms and unchanged checkout tree. Candidate writes remain
unpublished. Cancellation retains admission and the lock until the worker exits;
no upstream transaction or repository object crosses the owned result boundary.

Reword input additionally accepts trusted immutable heads from composition. The
bounded exact source-parent closure excludes each protected head and every
ancestor before any candidate writes. Heads require visible membership;
duplicate or excessive policy input fails without a transaction. Request patch
or commit claims do not establish the immutable policy. This strengthens
preparation only, without enabling publication or checkout.

The bounded Stacks operation archive walks the explicit candidate's full source
operation DAG and all referenced views through descriptor-relative no-follow
reads. Bounded raw operation/view files are decoded by the pinned upstream
SimpleOpStore in an owned temporary cache, avoiding source index creation and
unbounded source protobuf reads. Owned records verify all native identities and
complete ancestry/code closure. The owned bounded Git reader avoids upstream
extra import, reconciliation and no-GC ref writes during historical code reads.
Private materialization uses upstream writes and reads to prove exact IDs and
fields. Version-2 reserved retention links complete historical code closure
while legacy version-1 slots retain their original immutable format. This is
retained recovery input, not source repair or head publication; provider/runtime
orchestration remains required.

The real reword provider now captures this strict source history before loading
the mutation transaction and returns complete candidate history. It verifies
that all original operations, views and code records remain unchanged in that
history. Missing original extras cannot reach upstream implicit import, and the
returned archive can feed version-2 retention directly under the same lease.

The explicit source-history recovery provider reads a saved bounded private
witness and original reserved retention key while holding the actual foreground
working-copy lock. Its witness checks no-follow checkout, Git binding, policy
and head-marker observations without requiring an operation or index loader.
Private upstream materialization supplies exact native bytes; existing records
are decoded in an isolated cache and compared before any source writes. Synced
no-replace rename creates only missing operation/view records and the original
operation-store profile marker. Exact replay verifies and syncs selected
records. No heads, index, extras, companion or checkout files are selected or
repaired. Source restoration remains separate from runtime journal policy,
publication and guarded checkout composition.

Saved-history retention MAY resume the exact server-selected candidate archive
through the original recovery witness without opening a latest operation or
source index. The coordinator acquires the same nonblocking foreground lock,
revalidates saved binding/checkout/policy/heads and links real code before
reserved selection. The existing lease supports the same retention boundary
without releasing coordination. Both retain admission and lock until canceled
work exits. No source metadata restoration, head selection or physical checkout
occurs. Server journal selection precedes retention and acknowledgement follows
it; command authority and publication/checkout remain composition gates.

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

A disposable immutable copy relation index may retain owned copy IDs and
distinct parent/child adjacency derived from verified descriptors. Construction
admits node, edge, work and conservative allocation bounds before growth;
sealing rejects missing parents and cycles. Family lookup walks the requested
IDs' ancestors and all descendants of those ancestors through retained
adjacency, without scanning unrelated nodes or hydrating copy bodies. Parents of
a merge descendant do not themselves become ancestors of the original request.
Each lookup admits visited identities, scratch bytes and edge work
independently, checks cancellation and monotonic deadlines, and fails rather
than returning a truncated family. This data-only primitive retains no source,
credentials or publication capability. Source scope, global allocation leases,
checked-root selection, current authorization, acknowledged staged copies and
deterministic ordered-body delivery remain the integrating provider's
responsibilities.

Native selected copy-family providers retain only descriptor relations, scoped
to the process-local source, repository, epoch and checked inventory root. Share
the existing 64 MiB allocation pool and eight work permits with complete kind
projections. At most 32 fixed five-minute root entries retain checked selection
metadata; active owners keep their charges after eviction. Coalesce construction
for the same root without retaining authentication. Scan selected descriptors
under the existing window and two-million-descriptor cold-work limits. Complete
addition traversal can reuse an unchanged graph, or extend only copy relations;
removed or replaced baselines restart within the original remaining work/time.
Matching-family result/work/scratch bounds stay independent of construction.
Fresh permission, epoch and deadline checks precede all returned data.

The upstream backend combines that committed family with an append-only snapshot
of this session's acknowledged copy-parent relations. Retain bounded parent and
reverse-child identities after a successful upload only, admitting node, edge
and byte growth before upload and again before insertion. A captured sequence
excludes later writes without cloning the whole staged graph. Traverse staged
ancestors, resolve their selected seeds and follow staged descendants; do not
hydrate unrelated committed or staged bodies. Required unstaged seeds must be
selected. Duplicate acknowledged writes remain idempotent, unknown writes add no
staged relation, and complete ordered-body delivery retains deterministic
children-before-parents order. Compatibility providers keep their existing
bounded complete-inventory path. No native, RPC or persisted format changes.

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

The additive InventoryPage RPC pins a checked manifest inventory root, epoch,
publication position, code-state version and exact head set. Signed
continuations bind repository, object kind and scan size; they retain data only,
never a credential or request. Each page rechecks current permission and epoch
before and after its bounded scan. At most64 process-local pins live for five
minutes without expiry renewal; expiry, restart and epoch change require
reselection. The request scan size bounds inspected descriptors, not matching
IDs. Empty filtered windows retain their continuation. Responses report
inspected count and the original snapshot. Legacy Inventory framing remains
supported with its existing complete-result bound. NativeClient's
complete-result convenience uses the server's single selected complete-kind
lookup, benefiting from exact source-scoped reuse and additions advancement. It
rejects oversized, malformed, duplicate or unordered identities and never
returns a truncated success. Its caller count limit, server-owned
two-million-descriptor scan limit and whole-call monotonic deadline remain
explicit. Progressive callers continue to use the unchanged paged contract.

Legacy RPC Inventory and in-process native operation/copy enumeration use the
same selected descriptor windows, retaining only matching identities before
growth. Their caller and trusted session bounds limit the complete matching
result; an independent two-million-descriptor scan ceiling and session deadline
limit total work. Each window rechecks permission and epoch while retaining the
original manifest root. Return an error on exhaustion, never a truncated prefix
or copy match. These ordinary reads rely on publication/recovery closure
certification; semantic publication still performs its complete inventory audit.
Both paged and complete inventory paths enforce a monotonic deadline before
work, between scan windows and before returning data, as well as canceling
pending I/O at that deadline. Ready I/O or synchronous decoding cannot produce a
successful result after the admitted time budget.

Complete native kind projections contain only exact matching IDs and checked
immutable manifest data. Scope reuse to the native source capability,
repository, epoch, inventory root and operation/copy kind. Current
authorization, smaller caller/session limits and monotonic deadlines precede
response delivery even on a warm root. Complete changed-path additions advance a
verified baseline without reading unchanged descriptors. Fixed root/TTL/global
allocation and concurrency bounds apply; eviction cannot release an active
owner's charge. Cold scans remain explicit bounded work and do not retain
unrelated object identities. No legacy wire field, paged inspected-count
semantics, authoritative format or mutation policy changes.

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

Core browsing uses the pinned `jj-lib::DefaultIndexStore` and its standard
incremental builder in server-owned storage. One repository/profile/authority
namespace contains JJ's per-operation associations; bookmarks share its graph.
Reuse an indexed ancestor and read only newly needed commit bodies. Native
backend and operation-store adapters preserve committed membership and current
read authorization. Missing core indexes use the existing preparation task
contract; metadata, bookmark pages and scoped content do not require an index.
Core history must not depend on custom visibility, operation, substring or copy
projections. Those advanced optimisations retain their explicit bounded paths
and are outside this simplification's acceptance target.

Local warm reuse copies compatible source index files under existing disk,
file-count, read, memory and cancellation bounds before opening the private copy
with JJ. Only the managed copy may be repaired. The existing read-only Git
commit decoder remains mandatory for any incremental construction. Standard JJ
construction is subject to owned admission; if it cannot meet core acceptance,
report its specific limitation without introducing another custom writer.

Prepared graph visibility and divergence flags use exact visible change groups
from the selected admitted projection, with logarithmic point membership. They
MUST NOT enumerate hidden incarnations merely to describe one selected commit.
Cold readers retain upstream resolution; full prefix/offset contracts are
unchanged.

Prepared evolution pages may reuse the admitted selected operation topology and
retain at most32 evictable data-only predecessor-walk checkpoints per
generation, keyed by exact starting commit and exclusive anchor. Preserve JJ
0.45.1 operation order, within-operation topological sorting, duplicate handling
and the first legacy predecessor boundary. Retain only IDs, pending/queued walk
state and operation ranks; no store, stream, credential or request survives.
Checkpoints require proof that the anchor has exactly one known
predecessor-metadata occurrence in the captured operation closure; unknown or
possibly repeated anchors use bounded replay. The owned disposable
operation-history format2 captures a checksum-bound conservative repeated-ID
bitmap during the existing operation pass. A positive bitmap result only
disables caching; false positives cannot alter results. Format1 remains readable
with checkpoint reuse disabled. Admit copied state and traversal scratch before
growth and charge work/cancellation. Reused anchors and selected rows require
fresh source/operation admission; absent and known-empty predecessor records
remain distinct. Probe continuation using IDs without loading a commit body.
Missing/evicted checkpoints use bounded body-free replay; arbitrary filtered log
continuations remain separate delivery work.

Prepared evolution MUST use an admitted exact commit-to-operation occurrence
lookup to skip unrelated operation bodies. Seek the earliest relevant captured
operation at or after the walk's current rank, considering every pending commit
and the first legacy predecessor-metadata boundary. Preserve repeated IDs,
known-empty records, pending order and within-operation splice/topology
behavior. The disposable operation-history format3 checksum covers map-presence
flags and sorted occurrence identities; derive ranked lookup under resident/work
admission. Fresh readers validate and hydrate only relevant operations and
selected commits. Formats1/2 remain readable through the existing bounded replay
path. No source, credential, request capability or predecessor body is retained
by the lookup.

Prepared default `all()` log pages MUST retain the selected view's exact JJ
visible order and compact identity-to-rank lookup during generation admission.
Deep continuations seek a visible anchor and hydrate only the selected rows;
they MUST NOT replay preceding graph entries or load a body for lookahead.
Preserve ordered direct parent edges for this ancestor-closed set, current
authorization, selected membership, annotations and end-of-history semantics.
Absent or hidden anchors fail explicitly. Admit all retained arrays and sort
work before growth; retain no source, credential, stream or request capability.
Other revsets keep bounded upstream traversal until separately qualified
continuation delivery. Evolution uses the separately governed prepared walker
above; this default-log optimization does not claim general graph seeking.

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

LocalTrusted serving activates core indexing on the first repository open. The
existing repository monitor retains this activation independently of browser
subscriptions and supplies freshly authorized exact operations when code heads
change. It MUST NOT introduce a second watcher, poller or full search bootstrap.
Non-code events MUST NOT rebuild the graph. Restart discovers bounded persistent
cache bindings and resumes the same monitors. One shared native/local owner
coalesces selections, bounds pending/retained tasks, reauthorizes reads and task
control, and holds construction admission through actual worker exit. Old pinned
readers remain usable during newer construction. Cancellation, access loss and
changed epoch/registration MUST prevent adoption. Failure does not automatically
repeat on unchanged monitor witnesses; new code episodes or explicit reads may
retry. Authenticated automatic maintenance remains separately deferred.
Transient observation or admission failures retain usable pinned readers and
retry an uncompleted activation through the existing monitor. Only confirmed
access or identity loss invalidates readiness; a busy worker or deadline does
not prove either condition. Background local head observation shares interactive
admission and leaves a foreground slot available when capacity exceeds one. It
yields without queuing when that headroom is unavailable; existing freshness
checks catch up later.

Core native and local indexes share the existing cache admission namespace and
its slot, repository, global-byte and file-count limits. Persistent managed
reservations retain their source profile plus native epoch or local registration
generation, never credentials. Reserve the full construction allowance while a
writer runs, then charge actual retained bytes. Resume activation from complete
retained namespaces after restart. Incomplete construction may only be repaired
in private storage. Eviction may reclaim idle derived namespaces; memory readers
retain their exact operations independently.

The server query cache uses a private `query-cache-v1` namespace below the
pinned upstream revision and native profile. Bounded slots retain repository,
epoch and operation bindings, a recency counter and checksums of complete
derived files. The checksums detect local cache damage; native objects remain
the authority. Query execution checks current authorization before touching
these files.

An exclusive namespace lock serializes primary graph-cache admission and
bookkeeping, including separate executors/processes sharing that directory.
Contention returns Busy unless the request can use its exact immutable resident.
Primary index preparation installs its complete staged cache and bounded receipt
before graph evaluation releases this lock. Missing files, invalid receipts or
checksum mismatches trigger reconstruction. A mismatched native backend/profile
binding fails explicitly. Source corruption or denied reads cannot be repaired
by cache rebuilding and remain errors.

Retention bounds slot count, file count, individual-cache bytes and total
retained bytes. Defaults retain at most 16 slots, 2 GiB of file bytes per slot,
4 GiB per repository, 16 GiB in total, and 4096 filesystem entries per slot.
Cold staging shares these byte allowances. These are admission and retention
limits, not measured throughput guarantees. Least-recently-used slots are
evicted under the namespace lock; failed builds and abandoned staging
directories are disposable. Symlinks are never followed. A canceled worker
retains its independent reservation lock and worker permit until it exits;
construction does not retain the namespace lease. Responses report whether their
cache was built, reused or rebuilt and retained cache counts/bytes. These are
local diagnostics, never publication positions. Transient build disk use, native
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

Explicit native kind preparation retains a checked source/repository/epoch/root,
exclusive cursor and matching identities independently of an ordinary GET's
two-million-descriptor ceiling. Each advance inspects at most 256 descriptors
under its own call deadline, while original total descriptor, record-read,
encoded-byte, matching-ID, allocation and lifetime limits never renew. Reserve
the bounded window's work before I/O; unknown or canceled windows retain that
charge and leave their cursor/identities unchanged. Advance cursor/identities
only after successful verified window reads and fresh authorization/epoch
checks. Partial progress grants no complete IDs or cache certification. Only
terminal traversal from the start promotes a complete kind projection. Shared
allocation leases precede growth and follow active progress/data owners beyond
cache eviction. Source capabilities and credentials stay caller-owned; later
publication cannot replace a preparation root. The source primitive changes no
authoritative format. Supervised native kind tasks expose bounded metadata
through their separate HTTP contract and share the serving instance's reserved
preparation lease with native/local graph preparation. The lease follows actual
execution through cancel/expiry/teardown; interactive capacity remains
available.

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
the requested window and report unavailable last-change metadata when ancestry
or work budgets prevent exact first-parent attribution. Operation grouping uses
recorded predecessors first, preserves ambiguous lineage and requires explicit
parent selection for merge operations. Diff summaries retain exact before/after
terms, copy and rename sources, conflicts and binary/oversized reasons; selected
file content remains a separate verified read. These read-only queries cannot
stage objects, reconcile heads or publish repository state.

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

A ready local prepared reader additionally serves selected summaries, paths,
scoped content, diffs and conflicts through its point-membership index. Attach
the immutable generation only after exact binding and operation/view seal
verification, then use a fresh scoped Store directly. These queries MUST NOT
construct a graph RepoLoader or repeat retained commit-edge proof. Existing
ordinary byte/work/deadline and selected-tree grants still apply. Required
prepared-history kinds retain cold rebuild-required semantics; optional scoped
kinds may use their ordinary reader only when the generation is absent.

Local visibility preparation MAY stream commit/change pairs directly from the
verified JJ index to construct ordered visible-reference identities. Reserve
projection growth before traversal and retain it with the generation. Unfiltered
change/all reference windows use a fresh scoped Store, seek before target
hydration and need no graph RepoLoader. The wrapper rejects a different embedded
operation. Preserve ordinary output/record/work limits. Local nonempty filters
use the disk projection governed below. Native default-profile preparation also
captures descriptions for indexed nonempty bookmark/change/all filters; cold
filtered requests return explicit preparation recovery. Small-cache memory-only
profiles retain their ordinary bounded reader.

Local and eligible native preparation MAY add an owned `JSTLRF1` substring
projection beside the byte-identical upstream JJ index. Its redb4.3.0 file binds
the selected operation and lexical row count and has a complete SHA-256
admission sidecar. Keep an 8MiB database cache within an independently retained
allocation lease, with no whole-description resident projection. Reuse the
already admitted commit description during construction; record lengths admit
exact candidate text before reads. One-, two- and three-byte grams follow
whole-string lowercasing; postings use sparse 16-bit offsets or 4096-bit masks
per lexical block. Candidate intersection is never authoritative: pinned
visibility and exact substring checks decide matching, and selected bodies still
cross the scoped Store or native capability. Checksum, selected operation,
framing, work and byte admission precede retained reader use. Coalesce postings
within one 4096-row block behind a pre-reserved 64MiB scratch lease, flushing
before the 65,536-entry bound or next block. Grow neither resident state nor
write batches with repository size. Reserve final database, scratch and sidecar
growth through the same cache quota before writes. Ordinary reads do not
reconstruct or hash this projection. Retain file and cache ownership with the
prepared generation through active readers and cancellation cleanup. This
disposable local format changes neither upstream JJ format6 bytes nor any
native/companion/wire identity. Native filter preparation requires at least two
cache slots, a 32 MiB entry allowance and 64 inventory entries; smaller
memory-only policies keep ordinary bounded content reads.

Native graph generations MUST retain selected-view visibility and reverse
bookmark/workspace annotations alongside their verified immutable JJ index.
Derive visibility once from indexed parents and reserve resident projection
allocation before growth. Preserve pinned change-prefix ambiguity, hidden target
offsets and divergence by overlaying exact visibility on the shared upstream
change index. Reject different head selections. Fresh readers attach current
request budgets and authorization, never an earlier caller's budget or access.
Warm log/evolution rows charge and copy only their returned annotations rather
than scanning all references or reconstructing ancestor visibility. Returned
labels preserve local/remote ordering, identities and conflict terms. Selected
operation/view objects still cross native integrity and current authorization;
cache data cannot grant native membership. No authoritative/wire format changes.
Complete scale/RSS/lifecycle qualification remains in the deferred slice.

Native substring readers MUST retain a shared generation-file pin alongside
their database and allocation lease. Cache eviction cannot unlink a pinned
generation or stop accounting its bytes. Pin bookkeeping is a bounded empty file
outside the index, excluded from immutable receipts; it grants no source
membership. Drop the database before its pin. Quota/slot pressure MAY reclaim
idle filtered residents before retrying admission; executing readers keep their
files and leases. Eviction seeks another removable slot rather than deleting an
active generation. Recovered files remain privately reserved before allocating
decoders. An already pinned generation stays in its charged slot under an
independent shared pin instead: failed or canceled recovery cannot move or
delete another reader's files. Reopened namespace completion replaces only that
slot's bookkeeping without double-counting it. Namespace locks end before
allocating decode, construction and selected hydration. Cold complete receipt
hashing MUST run after independent quota ownership and namespace release, before
native loading or allocating decode. Namespace admission checks only bounded
names, types and actual sizes against the receipt. A checksum mismatch discards
an unpinned private generation and rebuilds through fresh construction
admission; a damaged shared generation fails without deleting another reader's
files or using resident fallback. Work, deadline and cancellation failures
retain their precise outcome. Failed admission before ownership transfer
preserves the existing complete files and receipt. Existing index-only
generations may be upgraded only by explicit preparation in private ownership;
checksums and the exact operation still precede use.

Native selected-view derivation MUST also retain ordered visible
change-to-commit groups from the indexed commit/change stream. Reserve group
keys and targets with visibility before allocation, sort identities once, and
preserve hidden rewrite exclusion and divergence. The composed content reader
shares the graph executor's generation and worker admission for exact-operation
unfiltered Changes/All pages, selecting keys before metadata hydration.
Workspace labels use retained reverse annotations. Selection borrows the freshly
verified view already loaded for that request, without reloading operation/view
objects or copying unselected bookmark terms. Fresh native membership,
integrity, access, work and output checks remain mandatory. Cold construction
and disk recovery use the existing inline threshold and explicit preparation;
namespace leases end before selected-target reads. Standalone metadata readers
retain their previous bounds. A composed reference request with a differently
embedded operation MUST select that operation's own generation after current
point membership admission through the outer read capability. Preserve the outer
repository, epoch, committed membership and authorization; selecting an
operation cannot extend that capability. Fresh operation/view proof and target
admission remain mandatory even on a warm generation. Never reuse the outer
operation's visibility or filter for another selection. Cold source generations
retain the same inline threshold and explicit preparation, and retry selects the
exact embedded operation without changing the outer read. Nonempty
Changes/All/Bookmarks queries use the same generation when the policy supports
filtering. Postings identify candidates; exact key/description matching uses
selected visible change targets or all bookmark conflict terms, including
removed terms. Hidden predecessors cannot create visible change results. Both
cursor directions probe matching rows before selected-only hydration. Missing
filter artifacts require explicit preparation even for an otherwise ready
index-only generation.

Native retained visibility and reference identities MUST use pre-admitted flat
arrays rather than one tree node, formatted key and target allocation per
commit. Keep exact visible JJ commit order with compact lexical ranks for
membership and default-log seeking, sorted native change/commit pairs for
lexical reference groups, and compact group boundaries. Seek arbitrary lexical
anchors without formatting or copying unselected keys. The construction
reservation covers maximum array capacities, upstream prefix bitsets and bounded
traversal scratch before growth. Sort in place with charged, cancellable
comparison batches, rejecting duplicate visible identities. Prefix ambiguity and
hidden target offsets remain delegated to the pinned upstream index. This
changes no persisted index, cache receipt or public contract. Admission
arithmetic for a two-million-row format-6 index must leave room inside the
unchanged process pool for further admitted work; that arithmetic is not
measured RSS, construction or large-fixture qualification.

A verified disk index without its resident selected-view projection follows the
same inline construction threshold as a missing index. Large generations return
explicit rebuild-required recovery; preparation rebuilds the projection with its
independent budget from verified disk data. Executor restart does not authorize
an interactive ancestor traversal above that threshold. Before decoding a
verified disk generation without a matching resident, admission MUST move its
files into a privately locked, quota-charged reservation under the namespace
lease. Charge existing data, bounded receipt replacement and reservation
metadata before moving; recovery does not reserve an unrelated construction
maximum or copy the index. Remove the old receipt from the private generation so
receipt replacement cannot allocate an uncharged second receipt. Release the
namespace before operation/view reads, allocating JJ decode and projection
derivation. Retain verified file identities/stamps where available; unsupported
precise metadata performs complete receipt proof before loading and again before
reacquiring the namespace for installation. It cannot retain a stamp-free warm
resident. Final namespace checks remain bounded names/types/sizes under the
generation's independent ownership. Explicit filter upgrades reserve their full
writable construction allowance before growth, instead of the smaller immutable
recovery allowance. Completion installs only the complete verified immutable
generation against current namespace state. Failed/canceled workers retain their
reservation until cleanup. Another job may install the same pin or an unrelated
generation; neither can change the privately owned source. An existing readonly
disk-filter pin instead retains the unchanged generation in place under
independent shared ownership, as governed above. The inline/hard index
thresholds, current authority and allocation leases still apply. No repository
state or persisted/wire format changes.

Shared resident allocation pressure MUST reclaim idle prepared native and local
generations in process-wide least-recently-used order before refusing new
growth. Keep weak cache registrations and drop their immutable handles outside
the allocation registry lock. Oversized reservations fail without eviction. An
executing reader retains its complete allocation lease and remains readable; do
not evict a generation held by a reader or wait on its cache-slot lock. Native
disk receipts remain disposable and independently recoverable after resident
eviction. A local evicted task resets and may be prepared again for the same
exact operation; it cannot report Ready without a retained generation.

Native resident admission MUST charge the framed JJ allocation, selected-view
projection, bounded database cache and receipt metadata rather than treating the
complete disk filter as an in-memory index. Carry prior index-allocation charges
when wrapping an existing resident in fresh request ownership. Complete database
files remain counted in slot/repository/process disk quotas. Admission
arithmetic with two million native rows and a large disk-only filter must leave
headroom inside the unchanged resident pool; this is not an RSS or scale proof.

Replication controls expose an optional process-local semantic change channel
through their existing owner. It carries no transport authority and changes no
RPC schema. Alternating idle pull/push scheduling does not trigger browser
reads; frontiers, checkpoint settlement, faults and pause/lifetime transitions
do.

The local adapter's independently embedded reference operation requires an
operation-metadata parent proof from the retained outer selection. Bound visits,
frontier memory, work, cancellation and deadlines before reading the prepared
source generation. Preserve registration checks and exact source selection
seals; a warm index never establishes source authority.

The local prepared operation lookup may retain a disposable sorted parent/child
edge projection built during the existing bounded source operation pass. It is
selected by the same exact operation and checked fingerprints, never authority
for new objects. Seek at most two candidate identities to distinguish a unique
child from ambiguity; hydrate only a selected child and verify its parent edge.
No native JJ index, authoritative object or public schema format changes.

Native prepared operation selectors may reuse an exact-operation retained
identity and reverse-edge projection captured by the existing source preparation
pass. Native envelopes and full identities retain authority; native fingerprint
slots hold SHA256 of the verified native operation/view IDs. Fresh request
stores MUST authorize and prove selected-object membership on every metadata
read. Indexed prefix/child selection MUST avoid complete inventory enumeration,
view or commit reads, preserve ambiguity and exclude later and sibling
operations outside the prepared ancestor closure. Shared resident admission
covers retained lookup allocations; ordinary cold and small-cache paths remain
bounded.

Local stack history uses the existing operation-pinned prepared graph engine
through typed head/base inputs, under JST-LOCAL-004. The provider constructs
only the exact ancestry/range expression from canonical identities; it never
reads a repository alias or executable configuration. Page bounds, authorized
membership, explicit graph edges and private disposable index ownership remain
unchanged. All stack command preparation and publication stay outside this read
path.

Retained extra recovery uses the owned archived commit fields and pinned
upstream protobuf/table serialization in a private materialization. Existing
source extra records are read through bounded no-follow capture and compared as
complete native commit metadata before any missing segment is selected. Explicit
repair adds canonical missing records while preserving original segment/head
bytes and never calling GitBackend implicit import. The upstream extra-table
lock shares the existing nonblocking descriptor-relative Unix protocol, with
replacement checks and worker lifetime retained through cancellation.
Publication, index reconstruction and checkout remain independent qualification
boundaries.

Explicit retained-index restoration reuses the bounded disk-backed index builder
with an archive-only Source. It supplies exact native operation/view metadata
and compact commit fields from the acknowledged archive; no source backend read
or synthetic identity participates in construction. Private format/framing
admission precedes upstream decoding under denied fallback operation/view
access. Exact create-only source commit segments and operation associations are
independent of extra restoration, operation-head publication and checkout.

Retained head publication consumes only the original private witness and exact
reserved preparation. Descriptor-relative native operation reads and private
upstream decoding verify bounded current ancestry without graph-index loading,
reconciliation or extra import. The original working-copy lock and pinned
upstream head-store lock remain owned throughout synced candidate-before-old
marker selection. Head-only publication proof does not qualify physical checkout
or installation combination updates.

Unchanged-tree checkout uses the pinned native checkout protobuf under the same
foreground lease, without invoking a second blocking upstream working-copy lock,
snapshotting files or rewriting tree state. A saved private canonical witness
permits normalization of only the intended operation-pointer file stamp; all
other original metadata/policy/binding fingerprints stay exact. Deterministic
per-witness staging admits only an exact output prefix and completes interrupted
writes before synced atomic pointer replacement. An owned service guard uses the
existing complete physical scanner through data-only provider interfaces.
Provider proof does not select the runtime journal, combination state or a
tree-changing checkout.

The separate tree-changing plan provider reads only the original retained
archive and its real reachable Git objects. It verifies bounded raw object
identities before deriving the exact named-workspace tree effects; it neither
loads latest history nor writes a transaction or checkout. Descriptor-relative
physical reads capture per-path fingerprints and parent directory identities,
including planned missing parents, between complete trusted physical guards and
original metadata/policy validation. Resolved regular/executable files and full
sparse coverage are the initial qualified preparation profile; unsupported
shapes, aliases and obstacles fail closed. Canonical plan decoding remains data
validation. Matching bytes do not normalize a replaced file/directory identity.
The original worker/foreground lease remains alive until cancellation exits.
Per-path durable installation and native tree-state completion remain separate
execution qualification; no completion proof or writable capability follows from
capturing/reopening a plan.

Checkout witness linkage accessors expose only owned original key, recovery
digest, candidate operation/view and physical selection. They MUST preserve
canonical validation and cannot construct a provider proof. Runtime composition
uses those identities to durably bind the private witness before calling pointer
completion; historical receipt state never grants fresh provider execution.

A recovered lease exposes data-only retained commit/archive and candidate
operation/view identity from the checked archive under its existing worker
budget. Runtime compares that selection to the original journal before provider
publication or pointer completion; the informative identity is not itself a
publication proof and cannot authorize a different retained archive.

Retained reword policy validation uses the original archived view's visible
parent closure, excluding hidden evolution-only records. Trusted immutable heads
must be unique, bounded and visible; protected ancestry cannot be rewritten.
Validation runs under the existing original lease with authorization, bounded
work and cancellation retained through worker exit. It writes no code or source
metadata. Informative publication status does not grant selection authority; an
acknowledgement-only provider path refuses to create a candidate head when the
original operation is no longer selected through current head ancestry.

Tree-plan linkage accessors expose retained commit/archive, source/candidate
commit and tree identities and decoded workspace name as owned data. They cannot
create a publication or checkout proof. The server saves canonical plan data
separately from its command journal and validates the original request before
storage access. Provider revalidation remains mandatory before any future
physical installation, even after successful plan/reference reopen.

The owned PhysicalInventoryV1 interface carries complete original scanner file
fingerprints and directory stamps for future interrupted-checkout guards. It
reproduces the existing scope-bound physical hash, validates canonical framing,
path namespace and bounds, and exposes no native write capability. The optional
PhysicalGuard capture method defaults to unavailable; trusted service capture
must use its qualified scanner. Merely decoding inventory data cannot normalize
changed files, attest installation or create a provider completion proof.

Runtime inventory linkage MUST validate the complete original recovery binding
and exact tree-plan physical digest before selecting a private inventory body.
Canonical identity and byte length are informative input references only; even
after immutable reopen/replay they cannot grant a provider progress or checkout
completion proof. No native write interface is added by this server composition.

## Retained tree-file progress

Physical installation uses exact retained Git blobs and provider-owned canonical
progress under the existing foreground lease. Prepared file/directory
identities, synced prior intent and no-replace moves preserve originals as
backups. Retry compares the entire namespace and admits only restricted known
transitions. No upstream working-copy loader is invoked and native metadata
stays original.

Native tree completion separately stages the pinned upstream protobufs and
retains a provider-owned plan/inventory-bound completion frame before selecting
tree-state and then checkout. Native RepoPath ordering and signed millisecond
timestamps match JJ; unaffected file states stay exact and Watchman clock is
cleared for the next scan. Exact prepared inode identities limit normalization
during interrupted recovery; complete physical namespace and backup proofs
remain required. This provider completion does not select combination metadata
or acknowledge an application command.

## Initial combination setup candidate

The local provider constructs actual unpublished merge and WIP commits from an
exact source operation and resolved selected bookmark. It preserves the current
checkout tree, source archive, all named refs and unrelated workspaces; a fresh
complete physical guard brackets candidate generation. No source snapshot,
operation-head selection or physical file write accompanies preparation.
Cancellation retains foreground/admission until worker exit. Runtime retention,
native pointer completion and installation state remain separate owned joins.

## Ordinary local Stacks engine

JST-JJ-003's mutation boundary for local Stacks uses pinned jj-lib Workspace,
normal settings/ignore rules, native working-copy mutation locks, snapshot,
transactions, merge_commit_trees, rebase_commit, descendant rewrites and native
checkout. `.jj/jj-store/stacks.toml` retains only local
presentation/configuration and bookmark names. Stable WIP change identities
follow normal JJ rewrites. This path does not use retained operation archives or
custom op-head/tree-state publication. A different external working change
detaches active selection and retains the old named WIP; later selection
restores it. Concurrent edits cause stale command rejection. Native conflicts
remain native conflict trees.

Active workspace settings use the existing native build/rebase path only when
the target changes; renaming preserves JJ identities. Deletion removes local
configuration and retains the native WIP, checkout and retention bookmarks. Both
commands validate the shown operation/configuration revision under the existing
settings and working-copy locks.

Selected-file commit creation inserts after an applied head, rebases
descendants, then squashes from the current rewritten WIP with `keep_emptied`.
Selections bind exact source commit IDs and canonical paths. The merge and WIP
retain their change identities through native rewriting; preview validation does
not publish or snapshot. Target ancestry and configured immutable descendants
remain protected.

Basic local Stacks actions use native bookmark targets, commit descriptions and
new --insert-after/--insert-before equivalent rebases. Above/below bookmarks
change references only. Point inspection admits at most 500 exact visible IDs;
configured immutable descendants and workspace target ancestry protect rewrites.

The first commit on a bookmark still pointing at the workspace target MUST fork
from that target without rewriting unrelated target children. Explicitly
retarget only the retained merges applying that bookmark; later commits use
normal insertion and descendant propagation. Empty Commit selections create an
empty commit. Refresh must not reconstruct these already-correct parent sets.

Local selected-file Transfer, Split and Edit use CommitWithSelection and
squash_commits with native descendant rebasing. Insertion resolves current
visible source change identities before squash, so rewritten source IDs cannot
accidentally select another commit. Empty WIP and source changes remain
retained.

Native local Stacks hunk selections are issued by the guarded read-only POST
`/v1/local/repos/{repo}/stacks/hunks`. Tokens bind native source change
identity, canonical path and complete before/after bytes; apply rederives them
at the exact operation/configuration pin. Client patches and display-only IDs
are rejected. Partial text deltas use native squash with emptied sources
retained, without reconstructing the megamerge. Binary, conflict, mode/type and
bounded-diff limits remain whole-file-only. Net branch comparisons remain
presentation-only.

Local graph commands use native `move_commits` insertion/reordering and full
`squash_commits` for squash/uncommit. WIP is retained as a native child; graph
rewrites do not reconstruct the merge. Branch squash/rebase discovers at most
500 commits in its segment and stops at the next user bookmark boundary. Visible
source identity, configured immutability and workspace-target protection are
validated at preview and apply; retention commits are not graph sources.

Branch rebases move the segment root with descendants. Single-revision moves
detach ordinary children as JJ does; if this detaches a saved applied edge,
native rebase restores that edge on the existing retained merge change. This
preserves its identity and WIP delta without constructing a replacement merge.

The ordinary local Stacks mutation profile now includes native bookmark deletion
(top bookmark only; lower bookmarks and commits retained), abandonment, selected
whole-file/native-hunk WIP discard, and complete native text-conflict
resolution. Conflict terms are read lazily through guarded POST
`stacks/conflict`; ordered three-term text conflicts use the existing Merge
Editor. Multi-sided text conflicts expose bounded ordered versions for pairwise
comparison in the same editable Merge Editor. The user reviews every version and
explicitly stages the complete resolved file before preview. Structural, mode,
binary and oversized values retain explicit unsupported reasons. Conflict paths
are bounded to 50 per current read.

Undo/Redo restores the native repository operation stack, including CLI work,
with native descriptions and remote tracking semantics. Current Git bookkeeping
is retained, as in JJ. TOML is not an Undo journal. Missing or restored
references remain visible as unavailable choices/reasons; Refresh and Switch
never rebuild membership automatically. Explicit Reapply saved branches
reconciles membership and preserves WIP. Operation/config/source guards and
unknown-outcome inspection remain required. Production file dragging uses exact
loaded source/path pairs; partial dragging uses issued native hunk tokens. Net
branch comparisons remain read-only. Undo/Redo remains available without a
selected named workspace.

### Correct ordinary workspace composition and topology

Local Stacks compose the target and distinct applied tip commit IDs, rewriting
only the retained app-owned merge on explicit membership or target changes.
Native descendant rebasing preserves the merge and WIP change identities,
conflicts, user bookmarks and other workspaces. Normal content mutations
continue through native propagation. Operation-owned topology visits at most
50,000 ancestry steps; unavailable topology cannot be presented as independent
columns. Aliases share a segment; linear dependencies group below their highest
applied tip, while siblings, shared applied bases and merge sides retain
distinct DAG meaning. Strict target ancestors remain saved but leave active
columns.

Explicit reconciliation can retire eligible legacy heads proven by the same
internal merge reference moving between distinct changes in recorded Stacks
operations. Search at most 256 linear operations, stopping at forks. Referenced
commits and any user descendants are protected; matching descriptions alone
never authorize cleanup. Refresh does not compose or clean the graph, and native
operation history retains recovery. The local mutation regressions include a
stock-JJ `testy` topology fixture and guarded cleanup, separately from live-page
read verification.
