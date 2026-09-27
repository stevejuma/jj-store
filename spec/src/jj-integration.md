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
reconstruction. Index-file corruption recovery and large rebuild task reporting
remain separate qualification work.

The current work/deadline checks bound admission, I/O and returned rows. Native
synchronous CPU algorithms cannot be preempted by a Tokio timeout; these limits
do not yet establish production CPU/memory qualification. That qualification and
cache retention limits must precede shared HTTP query exposure.

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
