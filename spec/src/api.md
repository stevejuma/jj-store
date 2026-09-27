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

## Graph continuations

The service signs opaque V1 graph cursors with a process-local HMAC key and
retains a bounded, expiring snapshot pin without credentials. The signature
binds repository, epoch, operation, publication position, manifest revision,
query (including workspace/time context), native traversal ordering, page size
and last node ID. Every page authenticates its current request independently.
Resuming renews only the request deadline; the captured inventory and snapshot
remain identical even after publication or checkpointing.

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
uses the last existing native entry name; transport signing is required before
this internal continuation becomes an HTTP cursor.

File responses verify the complete envelope and native identity into a bounded
private spool before exposing any bytes, including for ranges. The stream checks
current authorization and elapsed budgets before and after every bounded chunk.
Stream lifetime retains its concurrency permit and private spool. File bytes are
binary media, not JSON or trusted HTML; HTTP safe content-disposition and
browser rendering remain transport/presentation requirements.

Canceling a chunk invalidates that stream handle. Reusing it fails explicitly;
resumption requires opening a fresh verified range. This prevents cancellation
between local I/O and final authorization from silently skipping content.
