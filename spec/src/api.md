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
| JST-API-006 | HTTP routes and owned DTOs MUST generate OpenAPI and TypeScript contracts; normal and specification checks MUST reject drift and validate representative actual responses.          |

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

Native file predicates have separate byte and line-work admission from streamed
file downloads. Recursive parent merges are preflighted against the selected
native index before predicate evaluation, reserving ancestry work and bounding
unreduced merge terms/labels. The original native predicate evaluates admitted
inputs; limits return explicit errors without publishing or truncating results.

Implicit graph/content tree walks use the same 64-component, 4096-byte path
limits as explicit content paths. Repeated subtree objects at different paths
still require admission at each native cache key.

Graph, object, directory, diff, conflict and file reads share one native worker
capacity in the serving composition. Saturated reads return an explicit
retryable Busy result. A client cannot gain extra capacity by switching routes
or snapshot repositories. A dropped request releases capacity after its worker
exits; an open file handle keeps capacity until its verified spool is released.
Transport admission also bounds snapshot capture and response delivery, which
are separate from native worker execution.

Cold graph requests above the configured inline index threshold return an
explicit rebuild-required result. Index preparation remains tied to the selected
operation while repository heads advance. Clients retry that snapshot after a
preparation task reports completion; evicted or lost caches can require another
preparation. Completion never means repository history or local cache files are
permanently pinned. Hard query budgets also apply to preparation and cannot be
relaxed by a request parameter.

Index preparation tasks are process-local and require an explicit operation.
Starting, inspecting and canceling a task require current repository read
access; only its initiating principal may cancel it. Status also checks the
current epoch. Task IDs are opaque and confer no authority. Running tasks for
the same principal, repository, epoch and operation coalesce. Completed reports
retain no credentials or full inventories. Retention, snapshot capture,
concurrent jobs and expiry are bounded; exhaustion returns Busy. Expiry or
server restart returns task-reset rather than selecting current heads. A new
start may prepare an evicted cache.

Task cancellation or service teardown signals the native budget, and Canceling
becomes Canceled after worker exit. Expiry also signals cancellation without a
polling client. A task can fail with the native resource or integrity error;
completion does not waive those budgets. HTTP routes consume this service
contract and bound request/response delivery separately.

Both initial graph pages and signed-cursor resumes return the captured snapshot
with rebuild-required errors. A later publication cannot change the operation
selected for preparation. Continuing with the original cursor retains its
original snapshot/inventory after the exact operation cache has been prepared.

## HTTP read transport

The Axum V1 read transport binds repository-scoped Bearer credentials to the
same native sessions used by RPC. Reads include snapshot/capabilities, objects,
log, operation/evolution graphs, commit/operation detail, directories,
differences, conflicts, verified binary files, and index-task
start/status/cancel. Native publication is absent from these routes. Events,
search, typed mutations and browser assets are advertised only after their
separate implementations pass.

Graph and directory responses expose a snapshot, owned data, and a signed
next_cursor. Log requests provide an explicit at_time_millis anchor, retained on
continuations; user_email and logical workspace are explicit query context. The
operations heads parameter selects one full operation, and contradictory at_op
and heads inputs fail. Multiple current heads require selection, never automatic
reconciliation. Operation detail includes its native view and predecessor data;
commit detail adds snapshot-relative state annotations to the lossless object.

Transport limits bound URI/headers, JSON input/output, admitted requests and
total request/response lifetime. A response owns its slot through delivery;
expiry also drops an unpolled response and verified spool. Errors use fixed JSON
codes without storage details or credentials. Invalid/unknown query fields fail
explicitly. Authorized rebuild-required responses carry the captured snapshot.

Binary responses are application/octet-stream attachments with nosniff and
no-store. A single closed, open-ended or suffix byte range is selected only
after complete native verification; bounds are clamped to EOF, and
invalid/multiple or unsatisfiable ranges return 416. Range selection cannot
reset an already-consumed stream or repeatedly extend its byte allowance. File
streams reauthorize every chunk. JSON source strings are data; no endpoint
returns trusted source HTML.

## Committed event subscriptions

SSE at /v1/repos/{repo}/events emits only manifest-selected publications, in
publication order. V1 event IDs bind repository, epoch and committed position;
resume accepts after or Last-Event-ID, rejecting contradictory values. A new
subscription starts at the current position with a ready event and snapshot.
Replay never uses candidate filenames, bucket listing or native operation order.
Checkpoint-only manifest revisions do not emit repository-change events.

A subscription checks current read permission and epoch before each delivered
record and heartbeat. Local publication notifications are bounded wake hints;
periodic manifest refresh also observes independent writers. Neither missing nor
lagged hints can skip committed records. Expired replay windows, changed epochs
and positions ahead of authority produce explicit reset/bootstrap events. V1
retains committed history across checkpoints; the bounded SSE replay window does
not delete that history. Resume IDs survive server restart and confer no access.

Subscriptions have separate bounded transport slots, a finite response lifetime,
bounded replay pages/window and storage-read deadlines. Consumers that stop
polling cannot retain resources indefinitely or block publication. Errors after
headers terminate the stream; clients resume with the last fully parsed ID. The
browser uses authenticated fetch streaming because credentials stay in memory.

## Generated application contracts

The HTTP route inventory also generates OpenAPI 3.1. JSON schemas derive from
the exact owned Rust request and response types, using deserialization rules for
requests and serialization rules for responses. Optional request fields are
distinct from required nullable response fields. Native metadata schema names
are separate from graph presentation names. IDs remain canonical lowercase hex
with their exact widths; tuple arrays and tagged unions retain their wire form.

The generated document includes Bearer security, explicit snapshot selection,
query defaults, index-task bodies, errors, binary ranges, HEAD semantics and SSE
framing. Event data schemas describe JSON inside SSE frames, not the stream as a
JSON response. Heartbeats are comments. TypeScript types derive from this
document and are checked for drift together with it. Generation is offline once
locked build dependencies have been installed and requires no credentials or
server. Generated files are not edited by hand.

Contract shape validation does not establish runtime authorization or native
semantics; the existing transport/lifecycle tests remain required. Rust numeric
widths are retained in JSON schemas. Browser clients must reject unsafe integer
conversion or use lossless parsing when native values exceed JavaScript's exact
integer range; generated TypeScript number types alone do not guarantee this.

## Browser read client

The served Svelte application selects a repository by full ID and authenticates
with an in-memory Bearer token. Disconnect, authorization failure and page
teardown abort outstanding reads/subscriptions and clear repository data. No
credential enters URL state, browser storage, analytics or logs. Requests stay
on the application origin and omit cookies.

Commit, operation and evolution graphs render server-provided edges without
inferring ancestry. Multiple heads require an explicit choice; revset context
and pagination retain an exact selected operation and time anchor. Live events
indicate newer history without replacing the displayed snapshot. Reconnect
resumes only fully parsed SSE IDs, while reset asks for explicit refresh.

Read errors retain explicit cursor reset, incomplete history and index
preparation states. Users may prepare/cancel the selected native index. Source
strings, signatures, paths, diff hunks and conflict terms render as text. File
previews have a byte cap, distinguish binary/invalid UTF-8, and never execute or
follow repository content. JSON integers outside JavaScript's safe range remain
exact through lossless parsing and bigint presentation.

Static browser assets are loaded once from a bounded, explicit manifest. Only
listed index/JavaScript/CSS files are served; repository paths cannot select
filesystem content. The document permits same-origin scripts/styles/connects and
prohibits framing, plugins and inline script. The browser read client is
independent of the later typed mutation and search capabilities.

Generated schemas express reference siblings through equivalent allOf
composition so client generators retain internal enum discriminators. The
TypeScript consumer checks operation-diff narrowing across every status.

The snapshot bar identifies the loaded operation independently of editable query
fields. Query retries and pages retain original inputs; index preparation must
not reset the time anchor. Explicit refresh after an event reset starts a new
subscription. Browser graph rendering admits at most 1000 nodes and 4096 edges;
exceeding the display budget fails explicitly before layout.

## Deferred UI execution

As requested on 27 September 2026, further UI changes and browser deployment
acceptance are deferred to M4.3. Existing read UI and its checks remain intact.
Typed mutation, conflict, search and freshness UI requirements remain open;
backend/API/SSE implementation and qualification continue independently.
