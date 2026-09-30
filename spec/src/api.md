# Read APIs and browser

The served browser and Storybook consume the same production-owned shell,
repository views, presentation models and styles under `web/src/lib`. Catalog
fixtures implement the shared asynchronous controller contract without a
transport; production transport and access policy remain owned by the web API
layer. The current release target includes the LocalTrusted application on
literal loopback and manual-token repository discovery for authenticated
deployments. External-provider qualification and registry-only package
finalization remain deferred. Snapshot operation, inspected operation, commit,
change and comparison identities are distinct fields and must not be substituted
for one another.

Use tonic/protobuf for native RPC and Axum JSON/SSE for browser/agent APIs.
Generate OpenAPI and client types. Full IDs are authoritative; shortened IDs are
presentation. Historical loading requires no physical server working copy. Graph
rendering does not decide ancestry. The route inventory in the archived
attachment section 10 is implemented under /v1/repos/{repo}.

## Requirements

| ID          | Requirement                                                                                                                                                                                                                                               |
| ----------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| JST-API-001 | Read APIs MUST expose versioned DTOs for capabilities, snapshots, native objects, commit/operation/evolution graphs, trees, files, diffs, and conflicts without parsing CLI output.                                                                       |
| JST-API-002 | Pagination MUST bind an explicit operation, query, repository, and epoch; ambiguous heads and workspace-dependent expressions MUST require explicit resolution.                                                                                           |
| JST-API-003 | SSE MUST resume from committed positions and detect writes from other serving processes; slow consumers MUST NOT block repository publication.                                                                                                            |
| JST-API-004 | Queries MUST enforce authorization and resource budgets, safely render untrusted source content, and expose incomplete history or rebuild tasks explicitly.                                                                                               |
| JST-API-005 | The Svelte UI MUST show repository selection, snapshot state, three graph views, object/diff inspection, context-aware right-rail metadata/actions, Community-composer mutation prose, conflict editing, mutation previews/results, and search freshness. |
| JST-API-006 | HTTP routes and owned DTOs MUST generate OpenAPI and TypeScript contracts; normal and specification checks MUST reject drift and validate representative actual responses.                                                                                |
| JST-API-007 | Versioned bootstrap MUST expose LocalTrusted access mode, public capabilities, embedded-storage readiness and installation-owned repository discovery without exposing internal credentials or treating discovery metadata as repository authority.       |
| JST-API-008 | `GET /v1/repositories` MUST provide bounded cursor-paged installation discovery: all registered repositories in LocalTrusted mode and only currently reauthorized token candidates in authenticated mode; bootstrap MUST NOT embed repository entries.    |

## Bootstrap and browser admission

`GET /v1/bootstrap` precedes repository UI and returns access mode, public
capabilities and authoritative-storage readiness. Repository discovery is a
separate bounded `GET /v1/repositories` request. LocalTrusted returns the
identity-bound installation catalog; authenticated mode accepts one Bearer
credential, narrows by its public token ID and reauthorizes every candidate
against current repository authority. Reserved backup fields remain unavailable
while backup and restore are deferred. Neither response contains an internal
credential, and discovery metadata never confers repository access.

Every `/v1` request supplies `X-JJ-Store-Protocol: 1`. The server validates the
configured literal loopback authority, rejects forwarded-host trust, null or
cross-origin browser requests, cross-site Fetch Metadata and method overrides,
and accepts only endpoint-specific JSON or binary request media types. SSE uses
the same authority, origin, protocol and access checks. LocalTrusted rejects a
Bearer header and binds the installation HTTP principal. Authenticated discovery
and repository requests require the same manual bearer credential; browser
credentials remain memory-only. OpenAPI declares deployment-dependent access
rather than promising an unauthenticated public listener.

The transient Settings presentation composes only existing bootstrap, repository
discovery, capabilities, snapshot, search-status and index-task contracts. The
production controller retains each discovered repository's effective permission
and exposes the already captured capabilities and snapshot authority to
presentation. Search status is requested only when search is advertised, and its
failure does not replace repository content or connection state. Index
preparation submits the operation reported by an index-lag response when
present, otherwise the explicitly selected snapshot operation; concurrent
unselected heads cannot start a task. Repository switching, disconnection and
revocation close the source session, abort outstanding requests and clear this
settings projection. This UI adds no server route, schema or generated contract.
The transient destination consumes the shell's existing compact state so a
collapsed desktop rail exposes Back and each settings section as labelled icon
controls while keeping section selection and repository authority unchanged.

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

Browser collection routes add bidirectional windows without changing the
complete-response graph, tree, operation-difference or file-difference APIs.
`GET /references` searches real local bookmarks and visible native change IDs;
bookmark conflict terms, every explicit target, target metadata and logical
workspace associations remain intact. The route never assumes a `main` bookmark
and cannot manufacture deleted-bookmark history. Browser state retains the
selected bookmark or change identity separately from the commit resolved from
that reference; resolving one target must not replace the selected reference.
`GET /commits/{commit}/paths` returns direct file and directory terms for a
nonrecursive path window, while a recursive query walks leaf paths only as far
as the requested forward window plus a bounded probe. It reports exact native
terms and batches bounded last-change attribution. Merge ambiguity and the fixed
ancestry budget are explicit unavailable states rather than substitutions with
the selected commit.

Each browsing response carries signed `previous_cursor` and `next_cursor` values
over the same retained snapshot registry as graph and directory pages. Their
claims bind repository, epoch, operation, publication position, manifest
revision, query/filter/order and page size. A stable bookmark/change/path anchor
and direction can start a new bounded seek; only opaque cursor values resume a
retained window. Cursors never belong in permanent navigation state. Changed
queries, credentials, repository, operation or page size fail closed, and expiry
or process restart requires an explicit snapshot reselection.

File responses verify the complete envelope and native identity into a bounded
private spool before exposing any bytes, including for ranges. The stream checks
current authorization and elapsed budgets before and after every bounded chunk.
Stream lifetime retains its concurrency permit and private spool. File bytes are
binary media, not JSON or trusted HTML; HTTP safe content-disposition and
browser rendering remain transport/presentation requirements. A client composing
multiple file previews MUST keep those reads within native admission rather than
bursting one request per preview; a Busy response must not discard an already
validated directory listing.

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

`GET /operations/{operation}/changes` requires one immediate comparison parent
and pages affected changes by full native change ID. Recorded predecessor edges
take precedence. When no predecessor record exists, a newly visible commit's
native parents are comparison candidates. A single before/after pair is an exact
comparison; multiple candidates require explicit selection. Removed-only
visibility has no invented file comparison. Operations whose view changes no
visible commits return no groups, including metadata-only operations.

Each affected-change continuation remains bound to the selected operation and
parent. Every exact group's `GET /diff/paths` continuation remains separately
bound to that group's full from/to commit identities and the same captured read
snapshot. Clients consume these opaque cursors independently; they do not reuse
a path cursor across groups or substitute an operation ID for a commit ID.

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

`GET /diff/paths` pages changed-path summaries independently from selected-file
content. It retains before/after terms and normal/copy/rename sources while
marking content as not requested. The existing complete `/diff` route remains
compatible and supplies bounded content for one selected path. Operation-group
and changed-path continuations support stable anchors and backward windows under
the same signed snapshot bindings as references and recursive paths.

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

## Collaboration read transport

`GET /v1/repos/{repo}/agent/sessions`,
`GET /v1/repos/{repo}/agent/sessions/{session}` and
`GET /v1/repos/{repo}/agent/sessions/{session}/transcripts/{transcript}`, plus
`GET /v1/repos/{repo}/agent/associations`,
`GET /v1/repos/{repo}/agent/checkpoints`,
`GET /v1/repos/{repo}/agent/checkpoints/{checkpoint}` and
`GET /v1/repos/{repo}/agent/checkpoints/{checkpoint}/versions/{version}`, plus
`GET /v1/repos/{repo}/reviews/threads` and
`GET /v1/repos/{repo}/reviews/threads/{thread}`, are bounded collaboration
routes. LocalTrusted admits them through the existing literal-loopback
protection and installation principal, without a bearer credential.
Authenticated deployments retain their repository bearer policy. Both modes
require current repository read permission before and after every root-pinned
read.

The JSON response contains version 1, an exact AgentStore read snapshot, bounded
session summaries and an optional continuation. The snapshot carries repository,
epoch, publication position, manifest revision, AgentStore root digest and state
version. Session rows keep full stable session and immutable snapshot
identities; integration, state, optional parent, coverage and bounded object
counts are service-owned presentation data rather than serialized WAL records.
Nullable first and last activity milliseconds are derived only from exact event
IDs matched to their selected sanitized observation evidence. Missing
timestamps, unmatched evidence or a session beyond the 4,096-event timing bound
yields null timing fields.

The detail route accepts one canonical full stable session ID. It resolves that
key only through the current selected AgentStore session catalog and returns 404
when no matching selection exists. Its response repeats the exact read snapshot
and session summary, then supplies ordered immutable IDs for evidence and event
batches, stream descriptors for each selected transcript manifest and stable
tool-call to immutable tool-snapshot links. It also resolves at most 4,096
sequence-ordered activity events, eight MiB of sanitized message/tool content
and 64 exact code associations from those selected objects. Activity variants
retain session state, message, tool, permission, observed-file or capture-gap
semantics. A final tool record carries its stable call and snapshot identities,
native name, kind, state, optional sanitized input/output, observed paths and
matching code associations. Separate truncation flags make both bounds visible.
Each transcript descriptor retains the manifest and source identities,
generation, source/decoder/sanitizer versions, representation, logical byte
count, record count and gaps. It never fetches unselected objects, returns raw
capture payloads or treats an immutable snapshot ID as a stable session route.

The transcript route requires the full stable session ID, immutable transcript
manifest ID, source instance, generation and a caller-selected byte ceiling. It
first proves that the current or cursor-pinned session selects that manifest and
that the manifest owns the requested source generation. Version 1 pages return
only service-owned sequence, media type and UTF-8 sanitized content DTOs. A page
contains at most 256 records and eight MiB; a record that cannot fit the
selected budget fails explicitly instead of being truncated. Missing sessions,
transcripts or streams return 404.

The associations route accepts exactly one typed subject: `session`,
`tool_session` together with `tool`, `operation`, `commit` or `checkpoint`.
Missing, incomplete or contradictory selectors fail as an invalid request. A
version 1 page repeats the exact typed subject and returns ordered immutable
links containing either a checkpoint-version or association object ID. An
association link additionally returns the verified stable association, session
and tool identities, relation, native before/after operation and commit anchors,
change IDs, paths and creation time needed for bounded browser context lookup. A
valid subject with no links returns an empty page rather than inventing an
association. Raw AgentStore objects are not serialized by this route.

The checkpoint collection enumerates stable checkpoint IDs and their currently
selected immutable versions from one exact AgentStore root. Summaries retain the
stable checkpoint, immutable version, session, optional turn/scope, exact native
before/after anchors, code relation, evidence/trust, coverage, capture status,
execution outcome, bounded reference counts and prior version link. The stable
checkpoint detail route returns its current selected summary plus the exact
session snapshot, transcript, event, tool and association links.

The version route accepts the stable checkpoint ID and one full immutable
checkpoint-version ID. It can return only the current selection or a version in
that selection's validated `supersedes` chain. A foreign or unselected version
returns 404 even when some other selected AgentStore object refers to it. Both
detail routes pin the current root for the whole request and never reinterpret
the stable checkpoint ID as an immutable object ID.

The review collection enumerates stable thread IDs and their current immutable
snapshots from one exact ReviewStore root. Its version 1 response carries the
repository, epoch, publication position, manifest revision, ReviewStore root
digest and state version. Thread summaries preserve exact snapshot, anchor and
event-page object IDs, the validated typed anchor, workflow state and version
counters. The collection rejects an anchor whose repository identity differs
from the pinned repository. Stable-thread detail returns the same validated
anchor value, ordered selected comment revisions with safe visible bodies or
explicit tombstone/moderation states, and the complete bounded immutable event
stream. The combined visible comment bodies in one detail response are limited
to eight MiB. Each item retains its exact object identity and authenticated
actor. Missing stable threads return 404; no route reads an arbitrary
ReviewStore object or an unvalidated historical snapshot.

`POST /v1/repos/{repo}/reviews/commands` accepts a strict version-1 request ID
and one typed ReviewStore command. The authenticated session supplies the actor
and derives a repository-bound publication ID before the existing ReviewStore
WAL publication path runs. A committed response returns the exact selected WAL
receipt and immutable result identities; a timeout or unavailable publication
returns `outcome_unknown` with the original request ID.
`GET /v1/repos/{repo}/reviews/commands/{request_id}` reauthorizes current write
access and resolves that durable receipt after server reconstruction.
`not_known` never proves rejection. Both routes reject caller-authored
principals, publication IDs and unknown fields.

`limit` defaults to the common page size and accepts 1 through 256. The opaque
continuation is bound to the entire snapshot and original page size, retains its
immutable root after later publications and is never suitable for a permanent
URL. Transcript continuations additionally bind the stable session, manifest,
source instance, source generation, byte ceiling and last returned sequence.
Association continuations additionally bind the complete typed relation subject
and the root-local relation page and offset. A continuation for one subject
cannot resume another subject. Checkpoint collection continuations bind the
radix key after the last stable checkpoint in the selected catalog. Review
thread continuations use a separate signature domain and bind the corresponding
ReviewStore root/state version and last stable thread radix key. Invalid or
query-mismatched continuations return `invalid_cursor`; expiry, process restart
or epoch replacement returns `cursor_reset`. Request concurrency, retained pins,
transitive closure objects and lifetime are bounded by the shared read limits.
The route has no request body, mutation capability, receipt or agent dispatch
behavior.

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

Browser presentation preserves empty native descriptions rather than replacing
them with an invented commit message. The working-copy view renders the explicit
`(empty) (no description set)` label while retaining the full native commit and
change identities. Repository landing-document ordering, friendly labels,
responsive overflow, hidden-document selected treatment and selected-document
edit routing, bookmark-filter empty states and no-file diff states are client
presentation rules and do not change the read DTOs or transport format. The
responsive commit-details action and progressive commit-metadata hiding are also
client presentation; the action selects the Commits page before the same
complete commit identity. The content-sized breadcrumb filename field,
line-numbered Mira MDE edit body, focus treatment and filename-selected source
language do not add file-content fields.

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

Branch graph navigation resolves the selected bookmark or working-copy target
from the captured reference response before issuing the log request. One target
uses its exact ancestry, several conflicted targets use the union of their exact
ancestries and a deleted bookmark uses an explicitly empty revset. Paging keeps
that same target-bound revset and time anchor. The production controller retains
commit metadata for local and remote bookmarks, logical working copies, conflict
and divergence state; operation rows independently retain workspace, snapshot
and concurrent-head metadata.

Read errors retain explicit cursor reset, incomplete history and index
preparation states. Users may prepare/cancel the selected native index. Source
strings, signatures, paths, diff hunks and conflict terms render as text. File
previews have a byte cap, distinguish binary/invalid UTF-8, and never execute or
follow repository content. JSON integers outside JavaScript's safe range remain
exact through lossless parsing and bigint presentation.

Static browser assets are loaded once from a bounded, explicit manifest. Only
the listed index, JavaScript, CSS, font and fixed `agent-icons/*.svg` files are
served; repository paths cannot select filesystem content. Agent icons retain
their stable same-origin `/agent-icons/<name>.svg` URLs. The document permits
same-origin scripts, styles, connections and images plus image loads from the
exact `https://www.gravatar.com` origin. The only admitted inline style is the
exact SHA-256 hash of the pinned emoji-mart 5.6.0 shadow stylesheet used by
Community's emoji picker; arbitrary inline style and every inline script remain
blocked. The document also prohibits framing and plugins. The browser read
client is independent of the later typed mutation and search capabilities.

Generated schemas express reference siblings through equivalent allOf
composition so client generators retain internal enum discriminators. The
TypeScript consumer checks operation-diff narrowing across every status.

The snapshot bar identifies the loaded operation independently of editable query
fields. Query retries and pages retain original inputs; index preparation must
not reset the time anchor. Explicit refresh after an event reset starts a new
subscription. Browser graph rendering admits at most 1000 nodes and 4096 edges;
exceeding the display budget fails explicitly before layout.

Collection controllers retain server cursors independently for every bounded
resource. Directory navigation requests only the selected directory until a tree
node is expanded; list/package modes use the recursive path route without
forcing tree requests to become recursive. Commit inspection continues changed
path pages separately from commit detail. Operation inspection continues
affected-change groups and each selected group's changed paths separately, while
preserving the explicit comparison parent and exact before/after identities.

Agent sessions, selected session detail, transcript chunks, review threads and
selected review detail use the same repository-session query cache and bounded
HTTP client. Their keys include the active access-session identity, repository
and exact stable resource identity; server continuations remain opaque and
memory-only. A committed review command invalidates the repository-session scope
before refetching its list and exact thread. A durable unknown outcome retains
the complete request ID for the status route. Repository SSE changes only set
the shared freshness flag. Explicit snapshot refresh invalidates the same scope
before collaboration reads can expose a newer AgentStore or ReviewStore root.

Revision context lookup follows opaque association and review continuations
until it finds the exact selected commit; path context likewise requires an
exact code-text or code-artifact commit/path anchor. It deduplicates rows,
rejects repeated cursors and inspects no more than 4,096 rows. The resulting
stable session and thread IDs are retained together in Log or Branches routes so
Agent context and Review discussion render as sibling columns. Selecting a path
replaces the thread selection with its exact anchored review. Closing either
column clears only that route identity.

## Browsing UI execution

The LocalTrusted application reuses the reviewed Storybook shell for Files,
Branches, JJ Log, Op Log, evolution, context-aware right-rail details, typed
mutations and native conflict editing. Branch bookmark cards stay square through
the final visible row; the branch command view container does not round that row
and its unselected hover matches JJ Log's neutral row hover. Bookmark selection
exposes exact metadata and branch-capable actions before a revision is selected;
revision selection replaces that bookmark detail. The rail uses a page-specific
Community heading and create action instead of a duplicate Details close header.
It bootstraps the installation catalog and opens the sole repository without
credentials. An explicitly configured LocalTrusted search entry starts the
existing background Tantivy projection through a stable installation-derived
read-only principal; repository capabilities expose the command only when that
index is configured. M4 registry-only dependency finalization and shared browser
release qualification are deferred. The authenticated browser supports one
memory-only manual token and bounded repository discovery without adding an
account or OIDC layer.

The Files controller fetches the selected commit independently from graph
history so the production header can show the last author avatar, commit
message, Git revision, JJ change ID and commit time. Mutation prose uses
Community's form composer. A fixed Edit description or Abandon target is
presented as one compact row with author avatar, existing description, copyable
Git/JJ identities and relative time rather than an editable or disabled raw ID
input. The same presentation includes bookmark and workspace chips; selectable
Create change, Create bookmark and Move bookmark targets open a searchable
Design Core Command View over loaded revisions and retain the selected full
native commit ID in the typed request. Collaboration presentation uses the
Community message/thread/composer models and Design Core tool/diff primitives.
Agent session cards show the integration's agent avatar and name, state,
captured start time and compact captured duration when evidence supplies both
endpoints. SHA-shaped agent, review and anchor identities use copy controls;
tool details can embed an exact loaded patch or sanitized old/new edit input.
Revision and path selections open route-addressable Agent context and Review
discussion columns, and an agent header links back to its associated change.

The browser derives integration avatars from bundled, reviewed agent artwork;
agent identities keep Community's rounded-square shape while people and device
authors remain circular. Git author email is normalized before the same Gravatar
identicon lookup used by Community. Collaboration summaries may group the agent
and review participants already proven by the loaded root-pinned context, but do
not infer associations that the collaboration APIs did not return. Exact
code-text or code-artifact review anchors may load their immutable comparison
through the existing pinned diff reads so the discussion can render the comment
above its inline file diff.

The production diff controller passes Design Core line accessories and
annotations through its single-file wrapper. Creating a line comment resolves
the selected before or after file object at its explicit comparison operation,
verifies the displayed line text, derives the UTF-8 byte interval and digest,
and sends one typed `create_thread` command. Tool reply actions bind
`agent_artifact` anchors to the loaded session and tool snapshots, using the
tool-call identity as the V1 attempt identity when capture supplies no separate
attempt. A committed receipt refreshes summaries, selects and reloads the exact
returned thread, and renders it inline; `outcome_unknown` retains the stable
request ID for the existing status route.

Local workspace validation may resolve `@lapismd/design-core` from the sibling
source repository through the guarded pnpm hook. That development resolution
does not change browser request contracts, static-asset policy or runtime
transport behavior. Clean environments without the sibling source use the
registry range and committed registry lockfile.

Real browser deployment acceptance is a separate opt-in lane from captured
response tests. It resets an isolated OS-temporary LocalTrusted installation,
starts the built bundle and native server with embedded RustFS, auto-opens the
sole catalog repository without credentials, publishes through another native
CLI process over the owner-only Unix socket, and requires explicit refresh
before the displayed operation changes. The selected bookmark, commit and change
identities must survive that refresh. Its environment-selected fixture root and
cleanup cannot alter the interactive `.local/dev-demo` installation.

The disposable repository used by this lane contains a native unresolved
multi-parent file conflict and a concurrent rewrite pair with one JJ change ID
and two commit IDs. The browser consumes the server-provided identities and
edges for both cases; seeding them does not add an API-only ancestry or conflict
representation.

## Typed mutation HTTP transport

`POST /v1/repos/{repo}/mutations/preview` and `POST .../mutations` accept a
versioned typed request with request_id, exact expected snapshot and command.
`GET .../mutations/{request_id}` resolves that principal's durable receipt. All
three require current repository write permission, including before parsing
command bodies. Unknown fields, unsupported command variants, malformed IDs,
unknown query parameters and bodies above 512 KiB fail explicitly. Native
commands share the serving instance's worker pool with graph/content reads.
Capabilities advertise only implemented commands and the request limit.

Preview returns prepared provisional native results or the already committed
receipt. Apply returns committed or outcome_unknown; status returns committed,
not_known or outcome_unknown. These tagged results use HTTP 200; unknown does
not prove acceptance or rejection. Stale snapshots and changed intent return 409
with distinct codes. Semantic protection returns 409 protected_transition, while
missing, revoked or insufficient credentials return 401/403 access_denied. No
rejection is persisted as a receipt, so later not_known status is never proof of
rejection. Retain the original request ID on every connection, delivery or
timeout failure. Once apply/status has parsed and authorized an identity, an
HTTP deadline or oversized result returns outcome_unknown with that identity.
Preview cannot publish and does not authorize later apply.

Generated OpenAPI and TypeScript contracts include these routes and owned DTOs.
The generated client declaration remains an API artifact; M4.16 separately
qualifies the shared browser controls and real native mutation workflow. The
review-command contract enumerates all eight owned command tags, including
`resolve_comment_conflict` with its required observed revision IDs; its contract
test checks the exact set rather than a variant count.

The typed capability registry also advertises reconcile_operations. This command
requires `base` as a full current operation ID and an exact expected head set
with at least two heads. It merges all heads explicitly; GET and HEAD routes
remain incapable of reconciliation. The base selects report comparison and merge
ordering, while native JJ computes actual common ancestors. API types are
regenerated without browser feature work.

Abandon is an advertised typed command with explicit full commit IDs and boolean
retain_bookmarks/restore_descendants options. Reports include nonempty abandoned
and skipped_hidden lists. Empty lists are omitted to preserve V1 receipt bytes.
An all-hidden request returns HTTP 409 no_changes with zero publications; this
rejection is not a retained receipt. Snapshot and request identity rules remain
identical to other commands. Generated schemas/types expose this API and M4.16
connects the corresponding browser mutation controls.

ResolveConflicts is advertised as resolve_conflicts and accepts a commit plus
1–256 conflict paths with fingerprints. Each resolution is select_term with an
ordered zero-based term index, or delete. A stale fingerprint or a path that is
no longer a conflict returns HTTP 409 stale_conflict; malformed/overlapping
paths and invalid term indices return 400. Preview/apply/status retain the
existing authorization, limits, atomic snapshot and retry behavior. Uploaded
replacement-file actions use the authorized endpoint specified below. M4.17
connects these declarations to the shared conflict editor without changing the
native resolution contract.

EditFile is advertised as `edit_file` and carries the exact local bookmark name
and expected single target, canonical source and destination paths, bounded
UTF-8 replacement contents and the new child change description. The same
complete command, snapshot expectation and request ID are sent to preview and
apply; a lost result continues through the existing status route. The generated
OpenAPI and TypeScript unions expose this command on both mutation endpoints.
The browser offers it only for a current ordinary text file reached through an
unambiguous single-target bookmark, then follows a committed receipt to the new
bookmark target and destination path. Historical, conflicted, binary and
oversized selections remain read-only.

## Conflict content uploads

`POST /v1/repos/{repo}/conflict-uploads` takes application/octet-stream and
strict query parameters epoch, position, operation, commit, path and
fingerprint. The operation is the single expected head. Write authorization
precedes parsing and body consumption; content encodings, wrong media types,
malformed/duplicate headers, unknown query fields, excessive bodies and short
declared transfers fail explicitly. Both streaming bytes and request duration
are bounded. A 200 response supplies the server-computed native file ID, byte
length, context and scoped upload handle; it is not a mutation receipt. Retry
identical bytes and binding after a lost response. Uploads never publish native
history.

resolve_conflicts now also accepts uploaded_file with upload, executable and
optional copy_from_term. It validates the complete authorized binding and file
integrity before atomic native rewriting. Invalid handles return 409
invalid_upload; stale snapshots and conflicts retain their existing errors.
Capabilities expose the upload byte cap and supported resolution actions.
Generated OpenAPI and TypeScript include binary upload and resolution contracts;
the browser stages bounded bytes separately and retains the returned scoped
handle until preview or apply.

ResolveConflicts previews and retained results expose an optional
conflict_resolution report: exact target before/after IDs, sorted selected_paths
and complete remaining native conflict paths for every changed result commit.
Empty lists explicitly mean no remaining conflicts on that result; absent
reports are legacy or non-resolution responses. Preview IDs are provisional. The
generated OpenAPI and TypeScript contracts describe this extension; the M4.17
editor renders the report through the existing durable mutation workflow.

## Search HTTP transport

Configured repositories expose GET /v1/repos/{repo}/search and GET
/v1/repos/{repo}/search/status. Capabilities advertise search per repository;
unconfigured repositories return search_not_configured after authorization.
Search requires q and accepts explicit mode, at_op, scope, page_size,
minimum_position, wait_millis and cursor. Unknown or duplicate fields fail.
Historical mode is explicit and cannot also select at_op. Continuations retain
all original query inputs and their selected native view and index generation.

Search shares HTTP admission, response limits and native content workers with
other read routes. Search pages and status report indexed and repository
positions separately. A bounded freshness wait that cannot reach its target
returns 409 index_lag with currently authorized search_status. Expired or
restarted cursor state returns cursor_reset; multi-head queries require explicit
selection. Current authorization precedes lookup and parsing of search inputs;
revoked access returns no freshness, counts or index metadata. These GETs never
publish history. Generated contracts include responses and errors. M4.15 wires
the browser command overlay to this existing endpoint with selected-operation
queries and opaque in-memory continuations; it introduces no new transport
shape.

Browser native-ID controls derive their compact labels from the complete DTO
identities, preserving the suffix rather than using caller-truncated labels.
Clipboard actions retain the exact underlying identity; the split-control
presentation and interaction contract is owned by JST-UI-003.

Paired native-ID labels default to four leading characters alone. Hover or
keyboard focus reveals the longer prefix, two dots and final two characters. A
single identifier shows that longer abbreviation immediately; configuring the
display never changes clipboard values.

## Code-state mutation fencing

Head and read snapshots expose `code_state_version` separately from the global
publication sequence and manifest revision. Typed mutation requests and
conflict-upload bindings return that token with the captured epoch and heads.
Agent, review, settings and checkpoint publications may advance repository
position without invalidating a prepared code command; any native code-state
change advances the token and makes the command stale. Generated OpenAPI and
TypeScript contracts carry the field without weakening existing exact snapshot
validation.

Workspace selectors consume the existing native reference windows at explicit
operations, preserving each exact logical workspace name even when several names
share a commit. Browser workspace context is separate from commit/change and
bookmark inspection and is passed to native log queries. Complete workspace
discovery follows pinned reference continuations; it never publishes, reconciles
heads or reads client working-copy files. No HTTP or generated DTO changes are
required for this presentation.

Repository tab labels and right-rail navigation are presentation-only. Overview
retains the existing `files` route, Commits retains `log`, and all six
destinations preserve their operation and workspace context through existing
controller actions. Responsive overflow neither changes the route grammar nor
publishes native state.

## WorkStore service and transports

WorkStore HTTP exposes bounded project/task/comment pages and typed command
apply/status under /v1/repos/{repo}/work. Continuations retain one selected
WorkStore root, repeat the query and page size, reauthorize each page, and reset
after expiry or serving restart. The browser keeps code pins separate and
refreshes task metadata explicitly.

WorkStore browser reads use an independent pinned root per paged collection. The
Projects destination does not require a selected code operation. Stable
project/task route selections reload on explicit refresh. A saved receipt marks
freshness; ambiguous outcomes keep the complete request, including exact version
preconditions and entity IDs. Status and retry share that identity, even after
refresh. Linked repository navigation uses the normal target session and token
checks, never the home metadata grant.
