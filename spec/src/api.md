# Read APIs and browser

Use tonic/protobuf for native RPC and Axum JSON/SSE for browser/agent APIs.
Generate OpenAPI and client types. Full IDs are authoritative; shortened IDs are
presentation. Historical loading requires no physical server working copy. Graph
rendering does not decide ancestry. The route inventory in the archived
attachment section 10 is implemented under /v1/repos/{repo}.

## Requirements

| ID          | Requirement                                                                                                                                                                         |
| ----------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| JST-API-001 | Read APIs MUST expose versioned DTOs for capabilities, snapshots, native objects, commit/operation/evolution graphs, trees, files, diffs, and conflicts without parsing CLI output. |
| JST-API-002 | Pagination MUST bind an explicit operation, query, repository, and epoch; ambiguous heads and workspace-dependent expressions MUST require explicit resolution.                     |
| JST-API-003 | SSE MUST resume from committed positions and detect writes from other serving processes; slow consumers MUST NOT block repository publication.                                      |
| JST-API-004 | Queries MUST enforce authorization and resource budgets, safely render untrusted source content, and expose incomplete history or rebuild tasks explicitly.                         |
| JST-API-005 | The Svelte UI MUST show repository selection, snapshot state, three graph views, object/diff inspection, conflict editing, mutation previews/results, and search freshness.         |

## Read-session foundation

Application read sessions capture the checked manifest's repository, epoch,
publication position, revision and native head set. An omitted operation selects
only a sole head; concurrent heads return an explicit selection response without
reconciliation. An explicit historical operation must belong to the captured
committed inventory. Staged candidates and objects published after selection
cannot enter that session's membership set.

The owned read capability has no publication methods and cannot be reconstructed
from its informational DTO. Inventory count and elapsed time are capped by
trusted server limits. Each later access rechecks current permission and epoch
while retaining the original snapshot. Epoch changes require reselection; later
publications in the same epoch do not silently replace the selected view. The
graph layer owns index/query budgets and workspace-dependent revsets; the
pagination layer binds continuations. HTTP and browser integration consume these
owned service interfaces.

Elapsed-time guards check before and after polling an operation as well as
applying asynchronous timeouts. An immediately ready in-memory or cached result
cannot bypass an expired request deadline.

The native graph interface has explicit log context (logical workspace, author
email for identity-relative expressions, and a fixed time anchor). It uses no
user configuration, physical checkout, shell command or CLI output parser.
Continuation is a native graph-node identity in the same query traversal;
missing continuation nodes fail explicitly. Signing and binding these internal
continuations to transport requests is owned by the service pagination layer.

## Graph and directory continuations

The service signs opaque V1 read cursors with a process-local HMAC key and
retains a bounded, expiring snapshot pin without credentials. The signature
binds repository, epoch, operation, publication position, manifest revision,
query kind and inputs (including workspace/time context or commit/directory
path), native traversal ordering, page size and last node ID or directory entry
name. Every page authenticates its current request independently. Resuming
renews only the request deadline; the captured inventory and snapshot remain
identical even after publication or checkpointing.

Expired pins and server restarts return an explicit cursor-reset result. They
never fall back to latest. Invalid signatures or changed query bindings fail
explicitly. Pin count, retained inventory nodes, request concurrency and token
size are bounded. The process-local registry requires sticky routing in any
future multi-instance deployment; the first release serves one instance.

## Native content reads

Object responses carry an explicit native-profile version, typed key and
complete project-owned V1 metadata. They preserve signatures, ordered conflict
terms, copies, external submodule IDs, remote/local references, workspace
mappings and absent-versus-empty operation predecessor data. Virtual roots
require the same current authorization and captured committed membership as
stored objects.

Tree listing uses pinned native merge semantics without resolving or writing
objects. Paths are canonical repository paths; symlinks and submodules are never
followed. Dot, parent and NUL path components are rejected without
normalization. File/tree conflicts remain explicit terms. Directory pagination
uses the last existing native entry name internally. The service removes raw
continuations from responses and exposes only signed cursors. Graph and
directory requests share bounded snapshot retention and concurrency when
composed with the same cursor registry; changing query kind, commit, path or
page size is rejected. Oversized continuation tokens fail explicitly instead of
returning an unsigned name. Cursor registries retain neither credentials nor
mutable native sessions.

File responses verify the complete envelope and native identity into a bounded
private spool before exposing any bytes, including for ranges. The stream checks
current authorization and elapsed budgets before and after every bounded chunk.
Stream lifetime retains its concurrency permit and private spool. File bytes are
binary media, not JSON or trusted HTML; HTTP safe content-disposition and
browser rendering remain transport/presentation requirements.

Canceling a chunk invalidates that stream handle. Reusing it fails explicitly;
resumption requires opening a fresh verified range. This prevents cancellation
between local I/O and final authorization from silently skipping content.

## Operation differences

Operation differences compare an explicitly selected immediate native parent. A
sole parent is the default; merge operations require explicit parent selection,
and the virtual root reports that it has no parent. Another committed operation
cannot be substituted for an immediate parent.

The response distinguishes head changes from visible commit changes. Visibility
is the native commit-parent closure of each view's heads; predecessor-only
objects remain historical dependencies rather than visible commits. Complete
view deltas preserve local and remote bookmarks/tags, conflict terms, tracking
state, per-workspace Git heads, Git refs and logical workspace mappings. Changed
remote views carry complete before/after values, including empty remote views.
The operation's predecessor map preserves absent versus known-empty data.

Native-ID order makes set deltas deterministic; this order is not topological
ordering or publication order. Commit visits, storage reads, work, response
bytes and elapsed time are bounded. A difference is complete or fails
explicitly; it never silently truncates history. Equal head sets need no
ancestry traversal.

## File differences and conflicts

Structured file differences use the pinned native merged-tree diff stream. Each
change retains both ordered before/after terms at the target path. Copy ancestry
adds explicit normal/copy/rename source paths and values without suppressing
structural deletions. Multiple source lineages remain explicit. Placeholders
mean no recorded history and never become synthetic objects or shared ancestors.
Bounded source lookup selects the nearest available ancestor on each lineage;
when none is present, descendant lookup supports reverse comparisons. Paths
filter target files by canonical repository prefix.

Optional content uses exact native line diffing on bounded UTF-8 text,
preserving CRLF and missing final newlines. Every blob is fully verified before
reading. Binary, oversized, structural, conflicted or multiple-source content
returns an explicit unavailability reason while keeping its structural delta.
Traversal, change count, text bytes, estimated line-comparison work, output and
time are bounded; complete structured responses fail rather than silently
truncating.

Conflict descriptors preserve ordered add/remove terms, absent terms, native
values, labels, executable flags, copies and external submodules. A distinct V1
fingerprint binds repository, epoch, native profile, exact commit/change IDs,
path and the ordered labeled terms. A term ID is its zero-based position within
that fingerprinted descriptor, never an object capability. Fingerprints grant no
authority and are recomputed during eventual resolution.

The text editor is supported only for three present, verified UTF-8 file terms
with identical executable mode and copy history, within the configured text
budget. Other conflicts return a typed reason; term selection/deletion remain
separate mutation capabilities. Conflict reads never materialize conflict
markers, write merged trees or invoke a merge tool.

Graph responses include local cache diagnostics: built, reused or rebuilt;
evictions; retained entry count and bytes. Native operation identity remains the
snapshot authority. Rebuilding a corrupt or evicted cache never selects newer
heads or publishes a native operation. Busy graph-cache workers return an
explicit retryable result rather than queuing unbounded work.

Dropping an in-flight graph/content request cancels its native worker budget.
Pending authorization and storage waits terminate promptly; synchronous native
steps observe cancellation at their next bounded checkpoint. An already returned
verified file stream remains usable until it is dropped, expires or loses
access.

Query metadata admission scans the verified envelope payload before allocating
native DTOs. It bounds cumulative JSON value/key count, persisted merge arity,
parent/predecessor edges and entries in a tree object. These are request limits:
exceeding them returns a resource-limit error without changing native codecs,
identities or stored data. The scan retains no metadata collections and checks
cancellation/deadlines as it visits input. Native shape and identity validation
still follow admission; admission alone never makes an object trustworthy.
