# Read APIs and browser

Repository directory discovery and metadata loading are independent browser
reads. Discovery may display authorized names without fetching every card's
code/WorkStore/ReviewStore pages. Viewport-owned metadata requests share two
actual concurrent directory reads, propagate cancellation to their transport,
preserve independent pins and discard obsolete results. Existing paging,
authorization and partial-activity contracts remain unchanged; lightweight
code-summary delivery and complete browser qualification are separate slices.

Native `GET /v1/repos/{repo}/work/activity` and local
`POST /v1/local/repos/{repo}/work/activity` return independently pinned
body-free task count/timestamp groups with at most 256 rows, total group count
and bounded query-bound continuations. Local bodies require an explicit
companion metadata commit and use the existing generation-bound signed cursor
envelope. Directory cards fetch one group page per home and retain partial state
for unloaded groups; they do not load task summaries to count activity. Public
schema and representative transport evidence accompany this performance
delivery; complete combined contract and browser qualification remain deferred.

Native `GET /v1/repos/{repo}/reviews/activity` and local
`POST /v1/local/repos/{repo}/reviews/activity` return compact independently
pinned review counts and maximum accepted timestamps for a bounded first window
of 1..256 radix-ordered threads. The response explicitly marks counts incomplete
when more threads exist; it contains no thread/anchor/comment presentation or
continuation. The local body requires its committed companion metadata
selection. Directory cards request 50 selections without fetching review
presentations and retain their existing partial-activity indication. Native card
dates include sampled review activity; local card dates retain their existing
code/task policy. These transport and focused consumer proofs do not complete
combined browser, scale or release qualification.

Repository entry menus, Settings-style creation/conversion forms, action rows
and trailing shell spacing change presentation only. Local/server capability
gates, exact pending creation/conversion identities, preflight and receipt
recovery retain their existing contracts. Repository-summary failures use the
session-only notification service; retrying preserves the error episode until a
successful read clears it. Successful metadata from other repositories remains
visible. No route, schema, storage or generated client changes are introduced.

LocalTrusted installation Admin conversion uses strict versioned preview/start
bodies and original-request status. Preview returns a bounded eligibility result
and archive digest; start must match that digest and one explicit source
operation. Accepted requests retain the exact source archive and generated
destination identities before preparation or publication. Status resumes final
publication from retained prepared artifacts. Interrupted preparation requires
an explicit retry and status polling does not allocate new staging attempts;
absence is not rejection. Reports expose mappings and provenance digests, never
private archive bodies or source credentials.

Thread directory and selected conversation reads start concurrently through the
repository/session/profile query cache. Navigation cancels obsolete reads;
confirmed reactions invalidate and reload only their exact target, retaining
other reactions, messages, reply context, emoji definitions and raster URLs.
Unknown outcomes keep the original intent and receipt identity. Explicit refresh
invalidates displayed collections; SSE queues automatic active-view refresh.
Reaction reads are deduplicated across main/reply views and limited to four
concurrent requests. Reply counts and descendants use indexed passes without
changing loaded-history, exact revision, draft or scroll semantics. Unneeded JJ
log/reference reads do not run when restoring Threads. Cached reads retain
current repository-session isolation and cursor-reset behavior; no batch API or
durable read index is added.

Thread/Reaction HTTP reads share bounded validated immutable root data with
native RPC. Directory/detail/history load Thread state; library/image reads load
Reaction state; reaction queries load the exact target's owning selection too,
including the captured WorkStore root for task comments. Complete response
snapshots and current authorization remain request-bound. Root-keyed data reuse
and exact-target reaction indexes do not retain credentials or viewer-filtered
results and never replace cursor pinning or cold validation.

`POST /v1/local/capture/rescan` adds an Admin-only conditional discovery hint.
Its strict body contains version, control generation and expected revision; it
selects no policy or durable receipt. The response reports acceptance and cached
status, rather than scan completion or source admission. Paused/stopped/lost,
disabled and stale-policy requests are refused. The same 4096-byte body and
transport admission apply. Discovery root counts and phase, rescan pending and
degraded-source counts are cached body-free observations; status never scans
transcripts. Runtime harness groups add typed profile/product-version metadata
and cached root/source counts in deterministic profile order, with no paths or
session keys. Configured runtime returns at most five qualified groups;
unsupported profiles do not gain a runtime or qualification through this schema.
Import and source-level coverage remain separate gates.

Installation capture management uses LocalTrusted Admin-only
`GET /v1/local/capture` and `POST /v1/local/capture/control`. Both retain exact
authority, protocol, Origin, Fetch Metadata and body admission. Commands are
strict actor-free versioned bodies binding a request ID, opaque control
generation, expected revision and pause/resume action. Responses distinguish
selected device policy, the original durable receipt, replay and runtime request
acceptance; acceptance is not proof that pause/drain completed. Status exposes
only bounded body-free counters and phase, with no source paths or evidence.
Disabled status touches no capture state. The authenticated actor is derived
from the admitted local connection and current server configuration is
revalidated on every request. Missing/corrupt/foreign policy stays unavailable;
controls never initialize, enable or restart a worker. Import and detailed
source coverage use the separately governed installation routes below. CLI and
browser controls share these typed commands and preserve original uncertain
intent.

Thread event pages include their own human revisions from the captured graph.
Emoji pages include exact definition version references, and retained previews
are fetched through the credential-bound client into revocable memory-only URLs.

The browser composes owned CommunityWorkspaceSurface, CommunityThreadPanel,
CommunityDetails and CommunityReactions contracts. Threads have a channel
section in the left sidebar, bounded history, exact persisted replies, visible
actor-derived participants and reactions on existing review comments and
supported Agent messages. It preserves automatic browser refresh and durable
request identities for unknown outcomes.

UI refresh preserves loaded conversation history at one captured Thread root,
and focused replies load their exact-revision ancestor closure within bounded
paging. New replies preserve the main message ID, revision, body and reaction
target. Inline forum creation retains its in-memory draft and stable native
create/message request identities through unknown outcomes; status recovery
resumes the same post only after committed receipts. Drafts are scoped to the
repository and principal, survive transport recreation within that credential
session and are discarded on disconnect. A definitively rejected first message
leaves its composer draft available; publishing the same title again continues
the created thread with a fresh message request. It does not claim an atomic
two-command creation. Reaction hover/focus tooltips show counts and loaded
authorized actors, exclude conflicts and group only values represented by the
selected message revision's displayed chip. Pending/error UI is scoped to the
exact reaction target in message metadata, including replies; unknown outcomes
keep the same request ID and status/retry semantics, while non-reaction
operational failures use the scoped application notification service. The
metadata row shows a spinner while that target's command is in flight or
awaiting confirmation, after timestamp and reply information. Compact retry
controls retain the existing request/status semantics; error details remain
available from the failure indicator without changing message content.

The rebased HTTP inventory retains all local workspace, Search and Sync
management routes alongside Thread, Reaction and retained-image routes. The
merged inventory retains all 67 native routes and 20 separately scoped local
routes. Generated contracts and response captures qualify the combined 87-path
inventory before UI composition.

Native Agent capture preparation adds optional canonical ThreadBindingV1 bytes
on protobuf field 7. Empty bytes preserve legacy unlinked behavior. The server
derives actor and acceptance time; the client checks the exact returned binding,
request, publication and result identities. Publication retains the binding and
selects Agent plus Thread roots atomically through the existing manifest.

Codex harness-location discovery is a device-only native-capture capability. It
does not register filesystem folders, admit a local browser repository or
replace generation-checked local access. Metadata discovery and filesystem
browser snapshots retain independent bindings and publication boundaries.

Diagnostic-only bound capture exposes an additive capture_gap Thread-link target
with exact source/event references and original observed binding. The same
canonical capture request and receipt framing apply. Thread history shows the
gap without claiming a logical session, and its context link opens the Agent
directory. Direct gap-link commands must validate selected source/evidence
membership before publication.

Persistent Threads use Community channels as their presentation template, with
shared conversation/composer/message/reaction semantics. The browser adapter
retains jj-store authorization, exact revisions, receipts and automatic browser
refresh.

HTTP exposes typed Thread/reaction command bodies without actor fields, signed
Thread/event/emoji GET pages and an exact-target Reaction query POST. Exact
human message revisions use typed path identities. Custom emoji definitions use
a bounded two-MiB binary POST with finite query metadata; retirement uses typed
JSON. Retained images return verified raster bytes with snapshot/content
headers, nosniff and no-store; SVG, external URLs and byte ranges are
unsupported. All routes share current repository admission, including status and
assets.

Header chrome uses the sidebar's theme-relative tint for hover, keyboard focus
and open menu triggers. Its native command admission, snapshot refresh and
navigation actions retain their existing controller contracts.

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

The Vite development listener serves UI modules and HMR on the browser origin,
proxying all `/v1` routes, including local administration and SSE, to the
existing compiled loopback backend. The backend public origin stays equal to the
Vite origin; Host, Origin, Fetch Metadata and protocol headers are forwarded
unchanged, without adding CORS or credentials. Development CSS and the exact HMR
WebSocket origin have a separate local-only document CSP. Production built
assets and backend API security keep their existing strict policies. Vite file
serving rejects development state, JJ/Git metadata, private key/env files and
generated native build directories.

Appearance uses the device-local `jj-store.appearance.v1` preference record. The
Original light/dark pair and Buzz-derived palettes change browser presentation
only. Theme, composer and modal changes retain the current route, captured
snapshot, conversation draft and native request identities; selecting a theme
sends no repository mutation and adds no HTTP/RPC contract fields.

Projects review presentation distinguishes unsupported local JJ capability from
recoverable native review read failures: unsupported repositories perform no
review read and offer no retry, while native failures retain explicit retry.
Task Activity belongs after Subtasks in the main detail and contains threaded
comments; repository commit activity remains in the project's Activity tab.
Native comment focus opens the disclosure; the composer and retained drafts
remain below it in the same page. Collapsing Activity does not unmount the
composer. There is no task discussion side panel or Open task discussion action;
existing WorkStore commands, snapshots and pending identities remain unchanged.
The themed repository/edit/subtask buttons retain those callbacks and access
checks. Centering an empty Threads listing changes no list read, native identity
or create capability. Settings content navigation begins with one section title
and leaves return navigation to the existing sidebar, preserving the repository
route and snapshot.

Local Settings present registration, Search configuration and Sync information
in bordered rows with labels/descriptions before controls. Unsupported
capabilities use icon Empty states or bordered graph-status rows, while loading,
guidance and recoverable failures use public Design Core status and error
alerts. This presentation does not change advertised capabilities, exact folder
registration intent, saved/effective Search revisions or Sync control semantics.

Repository cards resolve their resting, hover and selected surfaces against the
same palette in Branches, JJ Log and Op Log. Pointer hover leaves the bookmark
selection and captured operation intact. Keyboard Enter and repository menu
commands continue to use the controller's exact native identities and advertised
capabilities.

Use tonic/protobuf for native RPC and Axum JSON/SSE for browser/agent APIs.
Generate OpenAPI and client types. Full IDs are authoritative; shortened IDs are
presentation. Historical loading requires no physical server working copy. Graph
rendering does not decide ancestry. Message presentation recognizes both native
commit widths and Git SHA-1 commit widths without remapping either identity.
Compact inline controls copy full IDs; message prose retains its existing
operation-selection behavior. The route inventory in the archived attachment
section 10 is implemented under /v1/repos/{repo}. Browser navigation
distinguishes project entries and compact project groups with Lucide Folder
Kanban, collapsed Threads with Messages Square, and repository entries with
Folder Git; labels and identities stay unchanged. The collapsed Projects Command
View contains project entries rather than linked repository children. Its
choices keep exact home/project identities, and the separate Repositories
Command View retains repository access and selection.

## Captured application content search

Native application content search retains bounded data-only indexes keyed by
repository, epoch and immutable Thread/Work roots. Validate complete selected
domains once per retained generation, admit normalized body growth before
allocation, and retain separately ordered recent and text-search keys. First
queries filter these compact fields and merge existing orders; continuations
retain matching keys. Hydrate excerpts only for returned hits, with bounded
Unicode-safe snippet scanning. Unchanged roots survive new filters and fresh
manifest envelopes; changed roots rebuild independently. Current tombstones,
archived Thread rules, task states, scoped matching and stable ordering remain
unchanged. Warm and continued pages recheck viewer, inventory admission, epoch
and current Read permission. Root indexes and query keys have separate bounded
retention charges; neither search cache nor cursor becomes repository authority.

Profile-aware connection and route restoration remain shared by repository
navigation, Threads and entity search. Global native search skips local
Git-backed profiles without issuing native API requests and reports their
unsupported search explicitly. The local profile's existing capability gates
remain in effect; a source change never fabricates native Thread authority.

`GET /v1/repos/{repo}/content/search` searches the complete bounded current
Thread and WorkStore roots, independently of the selected code operation and
Tantivy file scope. It includes current visible human messages and replies,
project/task descriptions and current comments. Edits replace prior content and
tombstones remove it. Empty queries return recent active entity summaries.
Continuations retain the immutable result projection, bind every filter and page
size, expire after five minutes and reauthorize the same viewer and repository
before every page. Epoch replacement requires an explicit refresh. No body,
count or identity is returned before repository admission. Full source IDs and
revisions accompany results; navigation preserves their native meaning.

The browser fans out at concurrency two over all connected authorized sources,
clears protected results on session replacement/disconnect, and shows per-source
failures. Section, repository, Thread, project and task scopes filter on the
server. Code results keep their own operation-selected Tantivy snapshots.

## Requirements

| ID          | Requirement                                                                                                                                                                                                                                                                                                                                                                                            |
| ----------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| JST-API-001 | Read APIs MUST expose versioned DTOs for capabilities, snapshots, native objects, commit/operation/evolution graphs, trees, files, diffs, and conflicts without parsing CLI output.                                                                                                                                                                                                                    |
| JST-API-002 | Pagination MUST bind an explicit operation, query, repository, and epoch; ambiguous heads and workspace-dependent expressions MUST require explicit resolution.                                                                                                                                                                                                                                        |
| JST-API-003 | SSE MUST resume from committed positions and detect writes from other serving processes; slow consumers MUST NOT block repository publication.                                                                                                                                                                                                                                                         |
| JST-API-004 | Queries MUST enforce authorization and resource budgets, safely render untrusted source content, and expose incomplete history or rebuild tasks explicitly.                                                                                                                                                                                                                                            |
| JST-API-005 | The Svelte UI MUST show repository selection, snapshot state, three graph views, object/diff inspection, context-aware right-rail metadata/actions, Community-composer mutation prose, conflict editing, mutation previews/results, and search freshness.                                                                                                                                              |
| JST-API-006 | HTTP routes and owned DTOs MUST generate OpenAPI and TypeScript contracts; normal and specification checks MUST reject drift and validate representative actual responses.                                                                                                                                                                                                                             |
| JST-API-007 | Versioned bootstrap MUST expose LocalTrusted access mode, public capabilities, embedded-storage readiness and installation-owned repository discovery without exposing internal credentials or treating discovery metadata as repository authority.                                                                                                                                                    |
| JST-API-008 | `GET /v1/repositories` MUST provide bounded cursor-paged installation discovery: all registered repositories in LocalTrusted mode and only currently reauthorized token candidates in authenticated mode; each entry MUST expose its current authenticated principal and effective permission without credential material; bootstrap MUST NOT embed repository entries.                                |
| JST-API-009 | Application content search MUST use one bounded manifest-selected current Thread and WorkStore projection, return full native identities and revisions with bounded excerpts, filter recent and scoped queries, exclude superseded bodies and tombstones, and bind opaque expiring continuations to query, limit, repository, viewer and authority epoch with current authorization before every page. |

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

Review thread summaries include optional root-pinned presentation with creation
actor/time, last accepted event time and a nullable current-comment excerpt.
Repository directory presentation composes existing snapshot, log, WorkStore
browse and ReviewStore page contracts with separate authorization and bounded
pages; it never adds persisted discovery ownership or reconciles operation
heads.

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

The sidebar Add project action uses the existing permission-gated WorkStore
create dialog and home selector. Sidebar repository-add actions reuse the same
LocalTrusted folder manager as Projects and preserve explicit Inspect/Add and
pending registration identities. Activity row navigation uses the existing hash
route with the row's repository, operation and complete commit/change IDs.
Parent-task and picker indicators use already loaded metadata. These changes add
no transport or persistence contract.

Projects header Add-menu dispatch and repository dialogs reuse the existing
local-folder registration and WorkStore set_repository contracts. The local
registration manager remains alive outside the transient menu. Moving link
selection into a dialog retains picker/manual full-ID selection, permission and
version gates, pending request identity and receipt recovery; accepted metadata
refresh closes a successfully linked dialog. Settings-style form composition
changes presentation only and adds no request or persistence format.

The transient destination consumes the shell's existing compact state so a
collapsed desktop rail exposes Back and each settings section as labelled icon
controls while keeping section selection and repository authority unchanged.
Settings page titles and section icons precede subsection tabs in the content
navigation; capability-gated refresh actions sit at its far right as labelled
icon buttons with tooltips. Back remains in the sidebar. Settings content has no
extra top padding, and action groups stack with equal-width buttons except on
Appearance. These layout choices preserve controller drafts and action guards.
The centered Settings repository picker composes the existing discovery list and
repository-open action, using a compact ghost trigger. It requires explicit
selection from Projects, retains the Settings section across a switch, avoids
reopening the current repository and blocks switching while drafts or pending
work would be lost. The selected repository and Settings refresh use existing
controller transitions; no new server route or generated contract is introduced.
Settings identifiers use public Design Core CopyableValue with complete native
values unchanged; abbreviation affects display only. Repository, Search
generation and Sync frontier values share this control. The sidebar Back action
invokes the existing transient Settings close callback without a new read or
persisted Settings-route write. Overview insets, full-width file editor tokens,
a stationary review header and native record Copy submenus are
presentation-only. Clipboard values, read sessions, routes, capabilities and
publication callbacks retain their existing contracts; these layouts add no
request or generated schema. Compact header insets and Source syntax
highlighting remain presentation-only; recorded and draft bytes, safe Markdown
previews and edit publication are unchanged. Highlighting uses detected filename
languages and static token styles under the existing production asset policy.

## Read-session foundation

Application read sessions capture the checked manifest's repository, epoch,
publication position, revision and native head set. An omitted operation selects
only a sole head; concurrent heads return an explicit selection response without
reconciliation. An explicit historical operation must belong to the captured
committed inventory. Staged candidates and objects published after selection
cannot enter that session's membership set.

The owned read capability has no publication methods and cannot be reconstructed
from its informational DTO. Interactive sessions retain the immutable inventory
catalog root, not a complete repository inventory. Requested object membership
uses verified catalog point lookups. Distinct requested objects and elapsed time
are capped by trusted server limits; complete inventory enumeration has its own
explicit bound. Each later access rechecks current permission and epoch while
retaining the original snapshot. Epoch changes require reselection; later
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

Native RPC exposes the same typed collaboration command, durable status, page,
Thread detail, exact human-message and retained-image reads. Request actors and
publication identities remain service-derived. JSON is bounded to four MiB;
custom raster bytes travel in separate two-MiB protobuf fields to avoid JSON
array expansion. Image metadata, bytes and requested content identity must
agree. Current peer/token admission applies to every call; replicated code
writes stay gated on reducer 3 and WorkStore homes retain their frozen origin.

Thread, Thread-event, exact-target Reaction and custom-emoji pages use a
separate bounded credential-free collaboration registry. Signed tokens bind
every captured source root, repository, viewer, query and page size. Object
count, encoded collaboration bytes, pin capacity, fixed expiry, token size and
request concurrency remain bounded. Expiry or process replacement returns an
explicit cursor-reset; publication does not change a resumed page's snapshot.
Every resumed query authenticates its current request, and no raw continuation
crosses the public response boundary.

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
explicitly. Pin count, retained inventory roots and head identities, request
concurrency and token size are bounded. Pins do not retain request-local
membership caches. The process-local registry requires sticky routing in any
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

The browser presents these recorded operation parents in a compact searchable
picker beside the workspace selector. Choosing a description or identity sends
the full native operation ID and preserves the pinned snapshot and logical
workspace in the hash; the selector does not widen the set of permitted
comparison parents.

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

Explicit index preparation uses one shared background worker in addition to the
configured interactive read capacity. Preparation MUST NOT occupy an interactive
slot, and a second preparation cannot gain another slot by choosing a different
repository or cache namespace. Its permit remains held until actual worker exit
after cancellation. Cache namespace locking remains independently enforced;
directory, object and operation-only reads do not require that lock.

Cold graph requests above the configured inline index threshold return an
explicit rebuild-required result. Index preparation remains tied to the selected
operation while repository heads advance. Clients retry that snapshot after a
preparation task reports completion; evicted or lost caches can require another
preparation. Completion never means repository history or local cache files are
permanently pinned. Preparation has a separately bounded trusted construction
policy; ordinary page budgets remain unchanged. Neither policy can be relaxed by
a request parameter.

Index preparation tasks are process-local and require an explicit operation.
Starting, inspecting and canceling a task require current repository read
access; only its initiating principal may cancel it. Status also checks the
current epoch. Task IDs are opaque and confer no authority. Running tasks for
the same principal, repository, epoch and operation coalesce. Completed reports
retain no credentials or full inventories. Retention, snapshot capture,
concurrent jobs and expiry are bounded; exhaustion returns Busy. Expiry or
server restart returns task-reset rather than selecting current heads. A new
start may prepare an evicted cache.

The native running deadline defaults to one hour, independently of the five
minute completed-report retention period, which starts at actual worker exit. An
explicit preparation captures the already-checked catalog root and operation
before its initiating page capability expires. Current repository read access
and epoch are rechecked during construction, with ten-second bounds on each
authorization or point-membership observation. The task retains at most 1,024
positive membership cache entries; eviction repeats verified point membership at
the same root. It MUST NOT retain an ever-growing page membership set or
enumerate the catalog. Expiration or cancellation does not release the worker
until it exits. The completed task retains no session credential.

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
observation timestamp fields may be recovered from the original RFC 3339
timestamp in the matched sanitized Codex JSONL record, only for the qualified
CLI/Desktop profile, envelope version and local-transcript source/trust. The
explicit observation timestamp takes precedence. Unrecognized or malformed
native timestamps, unmatched evidence or a session beyond the 4,096-event timing
bound yields null timing fields. This read-only projection also recovers timing
for previously published transcripts; it never rewrites evidence or requires
reimport, and filesystem modification or ingestion times are not substitutes.

Message activity and transcript chunks expose an additive optional
`occurred_at_millis` field. It is the timestamp of the exact selected message
observation, with the same qualified native timestamp recovery and explicit
observation precedence as session timing. Missing, malformed or unmatched times
are omitted. Transcript page timestamps additionally match the selected current
source-generation-1 chunk's normalized message event and content; unsupported
generations remain unknown. Recovery inspects at most 4,096 selected events and
retains the original cursor/root pin across pages. Times never derive from
sequence numbers, session start, filesystem metadata or ingestion.

Tool activity adds optional nonnegative `duration_millis`, omitted when the
exact selected start/end evidence cannot prove an ordered span. Qualified Codex
call/result timestamps and explicit timed hook lifecycle endpoints preserve
source/profile identity and the 4,096-event bound. Duration may be known while
outcome remains unknown. Native and companion pins retain the original state; no
storage, normalization or receipt encoding changes.

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
language do not add file-content fields. Static application focus styling
suppresses the native editable-content outline in file editors and Community
comment, reply and mutation composers under the served asset policy. It does not
require admitting injected editor styles or alter edit/submission semantics.
Directory and root navigation clears the file edit presentation and its
workspace guard; an empty selected file path does not initiate editing or
publish a mutation.

## Browser read client

The served Svelte application selects a repository by full ID and authenticates
with an in-memory Bearer token. Disconnect, authorization failure and page
teardown abort outstanding reads/subscriptions and clear repository data. No
credential enters URL state, browser storage, analytics or logs. Requests stay
on the application origin and omit cookies.

Commit, operation and evolution graphs render server-provided edges without
inferring ancestry. Multiple heads require an explicit choice; revset context
and pagination retain an exact selected operation and time anchor. Live events
refresh current-head views while retaining historical selections. Reconnect
resumes only fully parsed SSE IDs; ready/reset announcements compare against the
retained head and queue automatic browser refresh.

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
their stable same-origin `/agent-icons/<name>.svg` URLs. The production document
permits same-origin scripts, styles, connections and images plus image loads
from the exact `https://www.gravatar.com` origin. The only admitted inline style
element is the exact SHA-256 hash of the pinned emoji-mart 5.6.0 shadow
stylesheet used by Community's emoji picker. Style attributes admit only the
SHA-256 hash of the empty string with `unsafe-hashes`, allowing dialog cleanup
to clear its temporary CSSOM lock. Nonempty inline style attributes and every
inline script remain blocked. The document also prohibits framing and plugins.
The browser read client is independent of the later typed mutation and search
capabilities.

The explicit loopback LocalTrusted development annotation setting admits inline
scripts and styles only for the static bundle. It defaults to false; the owned
interactive demo enables it, while generic cold acceptance uses the strict
policy. API responses and request admission keep their existing restrictions.
The configuration guard and development opt-out are owned by `security.md` and
`cli.md`.

Generated schemas express reference siblings through equivalent allOf
composition so client generators retain internal enum discriminators. The
TypeScript consumer checks operation-diff narrowing across every status.

The snapshot bar identifies the loaded operation independently of editable query
fields. Query retries and pages retain original inputs; index preparation must
not reset the time anchor. Event resets queue a snapshot recapture
automatically; manual refresh remains available. Browser graph rendering admits
at most 1000 nodes and 4096 edges; exceeding the display budget fails explicitly
before layout.

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
memory-only. Review list selection opens the same main-page detail from
repositories and Projects. Fixed page composition and default comment labels
change no root, anchor, permission, command or version precondition; explicit
Reply actions retain their stable parent comment ID.

A committed review command invalidates the repository-session scope before
refetching its list and exact thread. A durable unknown outcome retains the
complete request ID for the status route. Repository SSE queues automatic
refresh. Snapshot recapture invalidates the same scope before collaboration
reads expose a newer AgentStore or ReviewStore root, including for historical
code selections.

Revision context lookup follows opaque association and review continuations
until it finds the exact selected commit; path context likewise requires an
exact code-text or code-artifact commit/path anchor. It deduplicates rows,
rejects repeated cursors and inspects no more than 4,096 rows. The resulting
stable session and thread IDs are retained together in Log or Branches routes so
Agent context and Review discussion render as sibling columns. Selecting a path
replaces the thread selection with its exact anchored review. Closing either
column clears only that route identity.

## Browsing UI execution

Review detail and Projects Activity consume the existing authorized native read
routes without adding repository authority. Exact review anchors determine code
comparisons; related agent context comes only from root-pinned associations and
sanitized session details. Project/task activity reads independently authorized
linked repositories at explicit code operations, preserves bounded continuation
snapshots, and treats missing capture or read failures as unavailable context.
Activity tab routes and the validated all/agent/unassisted classification
survive hash/history restoration. Classification uses bounded root-pinned
association metadata and does not load sanitized session bodies; unresolved
metadata remains unavailable, not evidence of an unassisted commit. The
repository Projects route browses authorized metadata homes with an explicit
linked-repository query while retaining its independent code operation and
repository navigation. Cache scope includes the target repository. Activity tab
routes survive hash/history restoration. Native commit links from Projects
authorize a code connection even when no code source is active, retain the exact
operation and restore the Projects scope with Back. Task sections initially
expand only Description and main-tab skeletons use a page inset.

Initial browser placeholders cover connection and repository discovery as well
as the individual Projects, repository-tab and task/review detail reads. These
are presentation states over the existing asynchronous controller; they add no
requests or read authority. Previously displayed snapshots remain visible during
background refresh, with empty/error/unsupported states shown after reads
settle.

Repository sidebar Threads use the existing authorized, bounded Threads list
endpoint independently from the selected code source. Each entry retains its own
native repository and stable thread ID; selecting it connects that owner before
opening the thread. Memory-only directories are cleared with the credential
session, discard late or foreign-repository pages, retain loaded windows on
refresh and omit unsupported local folders. This presentation adds no endpoint
or persisted contract. Browse threads uses the connected native repository or an
authorized native fallback; repository/Threads breadcrumbs preserve the
available code-operation context.

A selected JJ Log or Branches change exposes a read-only Evo Log action in the
right-rail Actions section. Its separate resizable column reads the existing
native evolution graph at the displayed operation, preserves server predecessor
edges and shows incomplete, empty, loading and failed states. The full anchor
commit is stored as `evolution` in the hash; opening, closing and selecting a
history row preserve the main commit, workspace and typed mutation target.
Snapshot-checked opaque pagination deduplicates identities, rejects repeated
cursors and caps displayed nodes at 1,000. Browser history and reload restore
the column; page, main revision, bookmark or workspace changes close it.
Repositories navigation sections have no top margin in expanded or compact form.
The conflict sheet header has a bottom border separating its actions from the
body.

The LocalTrusted application reuses the reviewed Storybook shell for Files,
Branches, JJ Log, Op Log, evolution, context-aware right-rail details, typed
mutations and native conflict editing. Branch bookmark cards stay square through
the final visible row; the branch command view container does not round that row
and its unselected hover matches JJ Log's neutral row hover. Bookmark selection
exposes exact metadata and branch-capable actions before a revision is selected;
revision selection replaces that bookmark detail. The rail uses a page-specific
Community heading and create action instead of a duplicate Details close header.
A selected change, operation or bookmark exposes its existing capability-gated
Actions section below Details navigation. The section follows the exact loaded
selection and disappears when that context is cleared. Operation selection
exposes reconciliation only for multiple heads and omits Edit description and
Abandon change commands. This is presentation only; typed mutation previews,
authorization and confirmation flows remain unchanged. It bootstraps the
installation catalog and opens the sole repository without credentials. An
explicitly configured LocalTrusted search entry starts the existing background
Tantivy projection through a stable installation-derived read-only principal;
repository capabilities expose the command only when that index is configured.
M4 registry-only dependency finalization and shared browser release
qualification are deferred. The authenticated browser supports one memory-only
manual token and bounded repository discovery without adding an account or OIDC
layer.

The Files controller fetches the selected commit independently from graph
history so the production header can show the last author avatar, commit
message, Git revision, JJ change ID and commit time. Mutation prose uses
Community's form composer. A fixed Edit description or Abandon target is
presented as one compact row with author avatar, existing description, copyable
Git/JJ identities and relative time rather than an editable or disabled raw ID
input. The same presentation includes bookmark and workspace chips; selectable
Create change, Create bookmark and Move bookmark targets open a searchable
Design Core Command View over loaded revisions and retain the selected full
native commit ID in the typed request. The selector aligns avatar and content
vertically for descriptionless commits and uses compact oval GIT/JJ labels in
its selected button and options. Their badge widths match so identifier text
aligns. Those labels are 10px, one pixel larger than the revision-ID copy-button
labels. A fixed identifier column stacks Git above JJ and keeps their chips
aligned across options, reserving the second row for missing change IDs;
relative commit time sits under the title at the same left edge, with
bookmark/workspace chips alongside it on that row. This styling leaves the bound
identities and mutation commands unchanged. Collaboration presentation uses the
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

M4.24 hosts this workflow in the AppShell structural sheet. Built-in text-side
accept-all controls retain an exact native term selection when the center bytes
match that side; custom edits clear a previous selection until staged. Native
term IDs and fingerprints remain in the typed request even though metadata cards
are omitted from the editor. Uploads capture the selected descriptor so path
switching cannot rebind staged bytes to another conflict.

The conflict sheet reuses the repository file tree in an independently scrolling
left body sidebar. Its leading header toggle retains per-path drafts while
showing or hiding the browser; narrow layouts return to the editor after file
selection. Delete and staged-file actions target the exact active descriptor
from the header, alongside atomic Preview and prepared/recovery actions. The
sheet has no footer or Cancel action. Close retains the existing pending-upload
and mutation guards, and path navigation stays locked while an upload or
prepared mutation binds its captured descriptors.

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
retains the existing `files` route, Commits retains `log`, and all seven
destinations preserve their operation and workspace context through existing
controller actions. Responsive overflow neither changes the route grammar nor
publishes native state.

## WorkStore service and transports

WorkStore HTTP exposes bounded project/task/comment pages and typed command
apply/status under /v1/repos/{repo}/work. Continuations retain one selected
WorkStore root, repeat the query and page size, reauthorize each page, and reset
after expiry or serving restart. The browser keeps code pins separate and
refreshes task metadata automatically after saves and new data.

WorkStore browser reads use an independent pinned root per paged collection. The
page-header filter controls reuse the existing Projects controller filter action
and native route values. Moving their presentation from the details rail adds no
requests or contracts; search submission and field selection retain the existing
bounded snapshot reads and automatic browser refresh semantics. The Projects
destination does not require a selected code operation. Stable project/task
route selections reload automatically after live events and bounded periodic
checks. A saved receipt immediately reloads the WorkStore view; ambiguous
outcomes keep the complete request, including exact version preconditions and
entity IDs. Status and retry share that identity, even after refresh. Linked
repository navigation uses the normal target session and token checks, never the
home metadata grant.

Project list rows reserve aligned title, description, count, creator-avatar and
date columns. Card/list selection always retains one layout. Agent row metric
placement and tool toolbar bounds change presentation only; native records,
snapshots and clipboard values retain their existing contracts.

Project list rows reserve aligned creator-avatar and date columns regardless of
count or date-text length, including narrow lists. They retain the existing
native actor, count and timestamp values.

Task-detail presentation keeps its header and composer outside the content
scroller. Moving the composer changes no comment command, version precondition,
permission or draft owner; Activity and reply/edit actions keep the same native
records and retained draft state.

The development seed exercises these same commands for linked sample projects,
all task states, subtasks and threaded comments. Its cold browser gate reads the
physically reopened native records, verifies zero publication on additive
reseed, and opens the seeded task discussion through the production hub.

The shared browser history independently restores Projects task identities and
the JJ Log evolution anchor. Page changes clear incompatible selections while
preserving the full code operation pin; Back, Forward and reload restore the
selected destination without publishing repository state.

Cold Git bridge browser acceptance uses actual LocalTrusted RustFS publication
and an external merge imported by the custom CLI. SSE automatically reloads the
active current-head graph and operation route while retaining selected commits.
The refreshed graph exposes the imported merge without synthetic agent history.
The same cold fixture verifies the native search adapter catches up to that
import.

## WorkStore browse summaries

`GET /v1/repos/{repo}/work/browse` exposes typed project/task summary pages.
Optional project, repository, parent, state and title/body substring filters are
applied over the complete selected root before paging. Query text is bounded to
512 UTF-8 bytes; limits remain 1-256 records and four MiB per response. Exact
query, home, epoch and root binding, reauthorization, five-minute expiry and
resource admission match existing WorkStore reads. Existing detail and mutation
contracts retain their meaning.

### Browser review edit adaptation

The browser controller exposes the existing `edit_comment` command using a
stable comment ID, the edit-start `expected_revision_id` and a V1
`safe_markdown` replacement body with `review-markdown-v1` sanitization. It does
not add an HTTP schema variant or accept client author/time fields. Reply menus
pass the stable parent through `append_reply`. Unknown command outcomes retain
the request ID and prevent a new command from replacing the pending identity;
receipt processing checks the owning connection before refreshing or navigating.

Browser branch/working-copy selection within one pinned repository operation
reuses the loaded reference collection and paging cursor. Only scoped graph and
detail reads run for that selection; their loading state cannot replace the
Branches column. Cold restoration, operation changes and automatic updates
continue to load operation-pinned references through the existing read contract.
Repository cards use full-width content and right-aligned name-row status chips
without changing metadata or permission contracts.

## Local workspace links

The optional `local_workspace_links` capability is advertised only by a
LocalTrusted admin deployment with a composed workspace manager. Repository
`GET /local/workspaces` returns a versioned revision and bounded list of
canonical device folders with explicit profile and ready/unavailable state.
`POST /local/workspaces/link` accepts only version, expected revision and an
absolute folder path; it returns the updated list. Both routes reauthorize the
repository and require local administration before consulting device state.
Errors distinguish invalid folder, unsupported profile, stale revision, limits
and unavailable state. The transport delegates to owned service interfaces; CLI
composition owns JJ discovery and the server owns registry persistence. No
request may supply a principal, socket, executable, state directory or shell
command.

The Settings client uses this advertised capability independently of Search. It
binds link requests to the displayed device revision and preserves the path
draft after failures, including stale or unknown outcomes. An explicit folder
refresh rereads saved device state before retry without moving the repository
snapshot. Repository/source changes invalidate pending responses and clear
private folder drafts; a late request cannot populate another repository.

The optional `local_folder_picker` capability additionally requires a composed
OS picker. `POST /local/workspaces/pick` accepts an empty JSON object and
returns `{version: 1, path: string | null}`; null denotes cancellation. It
requires the same LocalTrusted admin admission and repository authorization as
linking. It neither links nor validates a workspace. Failures distinguish busy,
timeout, invalid helper output and unavailable picker. No executable, prompt or
initial path is accepted from a caller. Existing HTTP request deadlines still
apply.

Settings grouping and joined ID Copy controls preserve full wire identities and
existing explicit actions. Tabs and clipboard feedback are local presentation
state and never publish or change the selected repository snapshot.

LocalTrusted admin Sync settings use optional `local_sync_management`.
Repository GET `/local/sync` returns a V1 local-only or configured-worker
report; POST `/local/sync/control` accepts only V1 pause, resume or retry
intent. Both reauthorize before accessing the worker. No upstream URL, token or
environment variable is exposed or editable.

LocalTrusted Admin `local_search_management` adds GET `/local/search`, POST
`/local/search/save` and POST `/local/search/control`. V1 reports contain a
configuration-byte revision, desired/effective enablement, explicit source
scopes/prefixes/index budget and operation state. Save and Apply carry the
observed revision. Control intents are apply, retry and rebuild. Browser
requests cannot select configuration paths or credentials. Accepted lifecycle
operations continue after HTTP disconnect and remain queryable through status.

Sidebar disclosure persistence is presentation state only: versioned boolean
maps for Projects/Repositories sections and individual projects. It cannot
restore authorization, credentials, repository snapshots or capability claims.

## Filesystem local repository API

LocalTrusted Admin installations with native authority advertise
`local_native_repository_creation`. POST `/v1/local/repositories/native` accepts
only V1 request identity and discovery name; GET
`/v1/local/repositories/native/requests/{request_id}` resolves that exact
installation-owned request. The server durably retains repository and epoch
identities before name reservation and manifest publication, then registers the
verified repository. Replays cannot allocate another repository, changed intent
fails conflict, and unknown outcomes retain the original request. Creation does
not require a physical workspace. Filesystem-only and authenticated deployments
do not advertise or admit this capability.

LocalTrusted administrative installation endpoints under
`/v1/local/repositories` inspect, register, remove and recover device-local
Git-backed JJ bindings. The OS picker is available before repository selection.
Registration retains its request identity through companion initialization and
discovery persistence. `/v1/local/repos/{repo}/...` serves profile-aware code
snapshots, bounded code queries and existing local
WorkStore/AgentStore/ReviewStore semantics. Native repository endpoints and
native identities retain their contracts. Discovery includes profile and
availability without filesystem paths. All local routes reuse the existing
origin, protocol, media-type and literal-loopback admission checks.

Local code reads use a strictly typed, non-publishing POST
`/v1/local/repos/{repo}/query` with explicit operation and owned query variant.
Snapshot, capabilities, registration inspection/list/add/remove/status and
installation folder picker endpoints use the same admission. Continuations bind
repository, generation, operation and exact request, expire after five minutes
and reset across process restart. Native ID schemas are unchanged.

Local WorkStore POST `/work/page` and `/work/browse` under the local repository
namespace require `metadata_commit` and a strict owned query. Their signed
continuations bind that committed selection and complete filters. POST
`/work/commands` and GET `/work/commands/{request_id}` reuse CLI device actor
provenance, original request receipts and unknown outcomes. They do not require
or fabricate a native code snapshot.

Local ReviewStore POST `/reviews/page` and `/reviews/detail` require an explicit
committed `metadata_commit`. List continuations bind the repository, binding
generation, exact metadata selection and page size and expire after five
minutes. They cannot select staged or foreign metadata. Detail uses the same
bounded comment/event reconstruction and integrity checks as native serving,
returning `LocalReviewSnapshot` without fabricated epoch or WAL fields. POST
`/reviews/commands` accepts the existing typed review command body; GET
`/reviews/commands/{request_id}` returns the original local receipt. Both use
the CLI device actor. Versioned capabilities advertise review reads, actions,
repository anchors, code anchors and agent anchors independently. Repository
anchors are qualified; code and agent anchors remain false. Local repository
capabilities do not contain a synchronization field.

Local repository AgentStore POST endpoints `/agents/sessions`,
`/agents/session`, `/agents/transcript` and `/agents/relations` require an
independent committed `metadata_commit`. Signed continuations bind the complete
query, repository and binding generation. GET `/events` announces bounded local
head/ref freshness with local-profile event DTOs. The browser automatically
recaptures active collaboration/current-head views while retaining historical
code pins. GET remains read-only.

POST `/v1/local/repositories/{repo}/reconnect` requires the current device
registry revision and an inspected folder path with the original companion
identity. It replaces only the device binding, increments its generation and
rejects a conflicting ready copy. It does not initialize replacement metadata.

The application dispatches Git-backed repositories only through the LocalTrusted
local namespace. Native ID validation, endpoints and event parser retain their
original contracts. Shared screen models use explicit native/local snapshot
unions; local data cannot fabricate native epoch, WAL positions or graph cache
reports.

Admitted local requests wait within the existing HTTP request/deadline bound for
a local worker. Parallel startup code and project reads cannot fail merely
because the bounded worker pool is briefly occupied. Cancellation before
admission performs no folder work.

Cold bounded-read integration acceptance uses the current capability-dependent
repository Add menu and folder controls alongside files, prepared history and
AgentStore reads. It preserves the source JJ operation, working files and
companion selection; menu presentation changes do not grant native authority to
a filesystem-only installation.

## Browser appearance state

Device appearance is browser-owned presentation state under JST-UI-018. It is
independent of server settings revisions, repository operations and receipts.
Appearance changes make no repository mutation or website metadata requests.
Merge qualification retains physical working-copy actions, native tool review
reads and local repository review creation in the shared controller contract.
Confirmed WorkStore reactions reload only their exact target through existing
authorized reads; request identities and observed slot preconditions are
unchanged.

Local POST `/v1/local/repos/{repo}/working-copy/read` takes a relative file
path. It returns `WorkingFile` with a device selection and bounded physical
UTF-8 text. POST `/working-copy/save` takes that exact selection and replacement
text. `working_copy_edit` capability is independent of recorded
`code_mutations`. Both routes validate current folder identity and generation;
Save revalidates the selected physical identity before replacement. Stale file
selections use `stale_working_file`; invalid, unsupported, unavailable and
oversized files retain separate responses. Physical Save has no native operation
or companion selection in its result and requires no RustFS runtime.

Buzz shell gradient continuity and compact navigation picker surfaces are owned
CSS presentation. The picker retains native repository/thread selection and
keyboard navigation, with app-background surface tokens and spacing independent
of browser glass. The generated palette's shared popover role supplies the same
app surface to search commands, menus and popovers, without per-component search
overrides. These adjustments add no preference fields or API requests.

The saved accent selects one circular swatch indicator in Appearance. Its ring
stays centered and circular across color modes, hover and responsive layouts;
keyboard focus uses its own visible outline. This presentation does not change
the preference format or native API.

Projects Agents tabs compose existing agent session reads from independently
authorized native/local transports. Main scope derives from unfiltered project
links with bounded project continuations, never installation membership alone.
Session pages validate their repository identity and exact retained AgentStore
snapshot. No server API or canonical record changes. The Projects route admits
`tab=agents` for both landing and selected-project views, with paired
`agent_repository` and `session` native IDs for an inline response. These keys
are valid only on Agents and retain scope through navigation history. Repository
Agents and Projects responses follow the device Thread layout: Split occupies
the standard body right sidebar and Focus uses the structural discussion sheet.
Selection, Close and layout switching preserve the loaded session directory and
the same native selected session; no request or route identity changes. The
response composes existing separately authorized session/transcript reads and
reaction/tool-review commands without switching the global code connection.
Session list and skeleton insets, centered avatars and title-row actions are
presentation only: they preserve pagination, selection identities, independently
operable code association controls and existing agent read/command contracts.

The Projects tab-row search submits the existing `q` filter explicitly on Enter.
Close clears that filter through the same route action and restores the tabs.
Clear removes draft and submitted text through the existing filter action while
retaining the open input in its current scope. The 512 UTF-8 byte bound,
independent home/repository admission and unsupported Review text-search
capability remain unchanged; displaying search controls grants no extra access.

The Threads directory shares that toolbar control while filtering its already
authorized, bounded ThreadSummary titles and directory excerpts. It retains the
existing continuation action and exact native thread selection, without a new
query parameter, indexed-search dependency, API contract or persisted query.

Focus/Split reply presentation preserves the existing native conversation, reply
and tool-call target IDs. Agent tool discussion remains in a full-width inline
row under the exact call, with independently read existing comments and retained
drafts inside either response layout. Bounded review context/detail reads never
change the selected review or route. Comment Reply/Edit selects that exact
review before activating its retained composer; native permission and unknown
request guards remain in force. Other Focus discussions use one
application-owned structural sheet. These choices do not alter API authorization
or mutation identities.

Adjacent tool-call grouping is presentation-only. AgentActivity order and exact
tool-call, event, snapshot and turn IDs remain unchanged; grouped actions still
use the selected individual native call and existing authorized review commands.
Grouped rows and their hover controls share the transcript scroll viewport;
inline call sizing and message-edge alignment do not change those command
targets or retained Focus/Split drafts.

Session comments reuse typed ReviewStore creation and reply commands with the
additive `agent_session` anchor target. It names the exact captured session
snapshot and stable session, retains the full AgentStore closure, and fails
closed on mismatched identity. Independent bounded reads for session and tool
comments do not replace the route or another selected review. Existing request,
receipt, authorization and unknown-outcome semantics apply unchanged.

Page section disclosure preferences use versioned, bounded device-local layout
storage with opaque scope/section keys and boolean flags. They restore only
presentation and do not restore repository access, entity selection, code pins,
read snapshots, drafts or commands. Rename/filter/operation changes do not reset
saved sections; native task/review identities isolate detail choices. Shell
toggle styling retains the existing accessible expansion and action contracts.

## Rich history query delivery

`GET /v1/repos/{repo}/query-info?at_op=...` MUST return versioned `QueryInfo`
inside the existing authorized operation-pinned envelope, including JJ version,
callable signatures/insertion text, standard aliases, operators, operation
fields and profile restrictions. `GET /operations` accepts optional metadata
`query` text through the existing signed pagination contract. Malformed draft
queries return HTTP 400 `invalid_query` with a bounded optional `diagnostic`
carrying kind and message; existing non-query error codes and HTTP envelopes
remain unchanged. There is no separate validation request or query publication.

## Desktop presentation metadata

The native host's version-1 macOS marker is an immutable window-local
presentation hint consumed before shell construction. It selects native control
clearance and drag-region markup. It is neither an HTTP capability nor an
authorization claim, and browser clients cannot obtain additional repository or
filesystem access by setting it. The host's window-only permissions remain
restricted to the managed main window and exact LocalTrusted origin; bootstrap,
HTTP admission and browser CSP retain their existing contracts.

Review transport availability is expressed by its optional reader, rather than
being inferred from a repository profile. Unsupported catalog fixtures omit that
reader explicitly; the typed production application props do not alter HTTP
admission, read envelopes or review persistence.

QueryInfo MAY include optional `default_user_email` only for LocalTrusted
serving. The trimmed value is bounded to 1024 bytes and excludes control
characters. Missing, malformed or unavailable configuration omits the hint;
authenticated native serving MUST omit host identity. This additive metadata
never changes request identity, admission, operation pins or explicit query
context. Multiline query whitespace retains the existing 4096-byte bound.

A client awaiting query metadata MUST discard the waiting submission when its
query scope is invalidated, including view disposal. Metadata completion grants
no authority to resume a canceled query; explicit operation/admission contracts
remain unchanged.

Local Sync adds protected POST `/local/sync/barrier` with an empty body and POST
`/local/sync/wait` with the existing versioned SyncWaitRequest. Both use
LocalTrusted admin admission before control-plane access. Wait requests are
bounded to one second and half the HTTP deadline; a worker timeout returns a
LocalSyncWait report with reached=false and the unchanged barrier. Capture is
explicit and may contact the configured peer; ordinary status GETs remain local.

The Settings reconciliation entry reuses the native preview/apply/status routes
and captures a current head independently from the historical browsing pin.
Transport failures after dispatch remain unknown; closing a presentation cannot
replace its request. Confirmed reconciliation uses automatic refresh semantics.

LocalTrusted native administrators may read `/local/storage`, submit versioned
`/local/storage/operations` commands, and read a bounded operation by request
ID. Commands are check, prepare_checkpoint, publish_checkpoint and
cancel_checkpoint; no paths, shell commands or browser-authored manifests are
accepted. A private durable journal allows cancellation only before checkpoint
dispatch, retains up to 128 operation identities per repository with bounded
bytes, one unresolved operation per repository and one running maintenance
worker per service, and rejects identity reuse with different intent. GET does
not create the journal or start a check. Reports expose captured position,
limits, phase and exact/covering checkpoint selection; checkpoint authority
remains the manifest. Operations continue independently of HTTP request
cancellation.

The managed macOS appearance bridge exposes only `set_glass_background` with a
boolean `enabled` argument, admitted by host ACL for the exact managed origin
and main window. It changes native presentation only; no HTTP, repository,
credential, filesystem or publication contract changes. Browser glass remains an
application-background effect. Native composition and handshake follow
desktop.md and JST-UI-018.

Settings read-only row actions copy complete displayed identities without an API
mutation. Copy actions are opt-in for identifiers and hashes, or enabled by an
internal navigation destination; ordinary status/count values stay
non-interactive. Separate repository and operation links reuse the existing
authorized routes and retain explicit operation pins; immutable thread
revisions, manifest digests and companion metadata IDs remain copy-only.
Copy-only rows reserve the trailing link space for alignment without adding a
navigation action. Navigation is disabled while Settings drafts or pending
operations require resolution. History search visibility and query-context
disclosure are presentation state: toggling them preserves the controller-owned
draft, accepted query, session email and pinned read context. Workspace context
uses the existing explicit workspace selection action.

Application feedback changes presentation only. Toast dismissal never resets
pending mutation identities, drops a draft or retries a command. Obsolete and
cancelled reads remain silent, session changes clear protected notices, and
confirmed state recovery removes the matching condition notice.

## Installation capture source coverage and import

Installation-local `GET /v1/local/capture/sources` requires current local Admin
management, accepts only bounded limit/cursor query fields and returns typed
cached source coverage. Pages are at most 100 rows; selectors/cursors bind a
fresh runtime generation and monotonic slot ordinal without paths, native
session keys or admission proofs. A last bound repository is cached metadata,
not authorization. Disabled pages are empty without capture-state I/O; an absent
or unexpectedly lost runtime is unavailable. Known completed stopped
observations remain readable. Continuations tolerate current membership
retirement/growth without reusing selectors; foreign runtime generations return
a distinct stale selection error. Status, source and receipt reads use a fair
coordination queue with a one-second acquisition bound; timeout returns the
existing Busy response. The wait does not grant authority or retain transcript
work. Current admission and private journal checks still run after acquisition.
This read endpoint cannot enroll sources, import history, advance reader
cursors, publish roots or change device policy.

Browser harness, multiple-status and multiple-repository filtering apply only to
already loaded source observations. Their combination cannot add query fields,
change page bounds or cursors, or reinterpret stable source selectors. Loaded
and observed coverage counts remain those of the accepted source pages.
Repository choices use exact cached last-bound identities; a missing last-bound
identity remains an unknown observation and cannot imply repository access or
prove that a source is unbound.

`POST /v1/local/capture/import` is an actor-free version-1 command with
request_id, generation, expected_revision and selector. Current local Admin and
independent repository Write govern historical enrollment. Acknowledgement
returns the exact durable import receipt and bounded progress, not a committed
AgentStore root. Exact replay resolves an accepted original selector after
restart without retargeting its source. Input, authority, stale/new versus
known/exact requests, private state and owner semantics follow the governed gate
in collaboration.md. Receipt/progress inspection requires current management
admission and exposes no paths, native session keys, source proofs or raw
content. The command dispatches once to an existing retained source owner. Its
response requires synchronized durable acceptance; mailbox acknowledgement alone
cannot return success. A five-second bounded acceptance observation may return
an unknown outcome, retaining every original input for retry. Cancellation
cannot release or recreate the source owner. Exact accepted replay returns its
original receipt even while paused or stopped, without transcript work or
dispatch.

`GET /v1/local/capture/imports?request_id=<request>` inspects one nonzero
request under current LocalTrusted Admin/configuration/installation admission.
Its strict query is actor-free. Missing intent returns no receipt and cached
pending or unavailable phase, never successful acceptance. Public receipts
expose original command, journal ordinal, qualified profile/version and
complete-record boundary; opaque source/reader/prefix digests and principals
remain private. Progress uses selected, pending, running, withheld, completed or
unavailable, with bounded accepted captured/published byte counts and separate
tail phase/coverage. Only exact associated reader metadata and contiguous native
receipts can establish completion. Inspection reads existing metadata without
pending bodies, recovery, source enrollment or owner creation. Known receipts
survive absent transcripts; missing/unqualified evidence withholds completion.
Disabled/revoked requests do no capture-state work. Missing import journals are
unavailable and never created by management. Configured Codex whole-server
startup creates journals only within a fresh absent private capture root; an
existing root is never repaired. A separately supervised metadata lane may
recover original accepted commands when a new retained runtime discovers their
exact physical/header bindings. Inspection itself performs no recovery dispatch.
Configured Codex startup uses these existing typed controls; browser management
uses installation-local, token-free protocol transport after advertised
LocalTrusted bootstrap admission. It preserves exact uint64 values and original
unknown control/import intent, validates response and receipt identity, and
withholds mixed runtime source pages. Browser management cannot enable capture,
enroll a changed selector on retry, or select repository roots. Other readers
remain pending.

`GET /v1/local/capture/sources` optionally includes `snapshot_publication` on
API source rows. Its closed record reports source `captured_sequence`,
`captured_logical_records`, `committed_sequence`, optional
`committed_logical_records`, `history_available`, `outstanding_ranges` (at
most 4096) and `receipts`. Missing or corrupt evidence omits the field; it
cannot be represented as successful zero coverage. Zero committed messages are
valid only with qualified receipt coverage, including empty-session lifecycle
evidence. Source sequence is a position, not a message count. Counts remain a
source publication observation, separate from accepted history-import boundaries
and current activity/eligibility. A page inspects at most its 100 selected
source associations with one metadata read under existing coordination. Current
installation Admin, configuration and control-file admission are rechecked after
inspection. No paths, native keys, reader source IDs or generation witnesses
appear in the response; the read contacts no harness and reads no pending body.

Capture management is presented in installation-scoped Agents settings. Leaving
that section cancels visible observations without replacing the original pending
command or request-ID draft; the section does not select a repository. The
browser treats `capture_control_busy` on ordinary GET observations as retryable
contention. At most five read attempts with 150/300/450/600ms delays run per
observation, with cancellation on closure. Exhaustion defers to the visible
polling cycle without an unavailable toast. Other refusals and malformed
responses remain errors; no POST request is retried by this read policy. Loading
uses Settings skeleton rows, retained data survives refresh, and Capture
controls stay visible with their existing capability/precondition guards.

The browser may retain the time it accepts a validated source-page observation
or continuation as session-only presentation metadata. Failed, busy, cancelled
or rejected reads and local filtering MUST NOT advance that observation time.
The source response supplies no per-source update timestamp; this metadata MUST
NOT be presented as transcript activity, file modification time or a
server-stored fact. Harness/status filtering leaves coverage, cursor identities
and exact selected import commands unchanged.

LocalTrusted Admin capture status exposes optional `configured_roots` per
qualified harness as a bounded list of startup-resolved directory paths. Each
path is an absolute UTF-8 string of at most 4096 characters, with at most
sixteen paths per harness. The server populates this from retained startup
metadata, without inspecting directories on GET. Source pages and import
receipts remain free of transcript files and native session identities. A
backend that omits the field has unavailable directory metadata, rather than an
empty configured list.

Capture notification dismissal changes presentation only. It neither clears an
unknown command nor changes its original request identity, draft or receipt.
Successful observations clear operational failure feedback only after status,
source coverage and any selected import inspection are accepted.

Agent activity and recorded transcript APIs retain their exact text content. The
browser may derive generic nested prompt-element disclosures through the safe
Markdown extension described in `ui.md`; tag names and literal attributes never
become trusted metadata or executable HTML. Preview expansion changes only
presentation, with Copy message retaining the complete original body. Compact
single-line leaf rows include the closing tag and safe inline Markdown, while
syntax coloring treats source attributes only as escaped text spans. No
transport schema, media type, stored transcript or review anchor changes.

### Installation configuration editing

JST-API-001 includes actor-free version-1 `GET /v1/local/settings` and
`POST /v1/local/settings`, advertised by `local_settings_management` only for
LocalTrusted administrators. Typed section writes cover service resource limits,
installation-wide protection rules and configured replication entries. They
require an exact saved-file revision, current installation identity and
unchanged authority infrastructure; the shared settings-file lock, bounded
regular-file read and synchronized atomic replacement apply across Search,
Agents and these sections. Only the selected section is replaced. Saved and
startup-effective values are distinct; saving starts no worker, contacts no peer
and publishes no repository state. Lost acknowledgement remains unknown until
fresh observation. Credential references are names or paths; credential values
are never read or accepted. Deployment/identity/storage paths and deferred
backup configuration are exposed as read-only inventory with their lifecycle or
migration reason. They are not generic TOML or filesystem editing capabilities.

Search configuration also includes bounded polling, timeout and snapshot
interval fields. Its existing conditional save and explicit Apply worker
replacement activate these fields with the other search configuration.

### Local capture configuration editing

JST-API-001 includes installation-scoped `GET /v1/local/capture/settings` and
`POST /v1/local/capture/settings`. Only current LocalTrusted Admin authority may
read roots or save configuration. Bodies are strict, actor-free version 1 and
carry the exact saved-file revision plus typed capture configuration. A save
replaces only `agent_capture` under the shared settings-file lock; it preserves
unrelated TOML values and detects stale file revisions, including concurrent
search editing. Bounded same-directory atomic replacement and file/parent
synchronization precede acknowledgement. A failed acknowledgement is an unknown
outcome: clients retain their draft and compare a fresh saved configuration
before retrying. Startup-effective settings and restart-required state are
distinct from saved configuration and runtime health. The API never executes an
agent or exposes a generic filesystem/config editor.

Configuration observations and saves MUST remain available while transcript
inspection occupies the runtime control gate. They use bounded file reads and
the shared settings-file lock independently from capture journals, spools or
worker command ownership.

Capture configuration accepts the two Codex profiles, separate Cursor Desktop
and CLI transcript profiles, and OpenCode read-only API (at most five selected
profiles). Harness settings carry an optional endpoint only for OpenCode; that
profile requires an explicit credential-free literal-loopback root URL and no
filesystem roots. Cursor profiles expose resolved roots and independently
qualified product versions. Existing saved/effective revision, conditional-save
and restart semantics remain unchanged. Export is not editable or enabled.

Grouping harness controls into shared presentation rows MUST preserve each
profile's independent enablement, roots or endpoint in the saved draft. Opening
or closing a harness row performs no configuration save or capture command.

Configured runtime coverage supports up to five independently qualified product
profiles. Cursor sources use cached ordinary coverage and repository binding,
but never advertise an inactive-history import target. OpenCode endpoint errors
do not turn the entire capture service unavailable. Source GETs remain cached
and perform no live directory scan or endpoint request.

## Desktop terminal consumer

The desktop host consumes existing protected local repository registrations and
workspace-link reads to resolve terminal targets. These routes remain read-only
and do not execute shells. Terminal frames and controls use the desktop-only
Tauri interface, not an HTTP or repository publication API. Maximizing the dock
closes the application-owned details sidebar through local layout state; this
presentation action adds no terminal IPC command or server mutation.

## Bundled application documentation

The Help entry uses the primary sidebar's collapsed and expanded spacing. This
presentation change preserves the same overlay, local document routes and
admission policy.

Help reads same-origin presentation attributes to inherit live application
appearance; it adds no API, mutation or credential transport. Generated fonts,
palette CSS and diagram variants remain in the independent documentation asset
manifest and use the existing CSP. Standalone docs do not read app preferences.
The standalone Original palette and initial navigation collapse are local
presentation defaults; they do not change host preferences or asset admission.

Bundled Help is independently allowlisted under `/help/`, with a separate
512-file, 8-MiB-per-file and 32-MiB-total bound. The existing browser bundle
retains its own bounds. Application documents may frame same-origin content, but
only documentation responses permit same-origin ancestors; API responses remain
unframeable. New browser builds include Help; older valid bundles without a help
directory remain loadable.

Nonrecursive path pages include direct file, directory, symlink, submodule and
unresolved structural-conflict entries. They load the selected directory terms
without walking descendant directories. Unfiltered bookmark windows select rows
before target hydration; each returned target retains exact pinned membership.

Path windows accept `include_last_change` (default true for existing clients).
False returns row identities and `not_requested` attribution without walking
parents. Attribution traverses one shared bounded first-parent chain per window,
with explicit merge/budget outcomes. Clients load rows before attribution, use
independent cursor bindings for each query, and join metadata only to matching
immutable commit/path/term identities. Metadata failure cannot discard rows.

Reference windows support `kind=workspaces` independently of visible-change
history. Each row names one workspace and its exact selected target. Bookmarks
and workspaces support an optional `exact` name; exact lookup uses the captured
view's keyed map, returns no continuation, and rejects a simultaneous text query
or page anchor. Workspace windows seek their own ordered names and hydrate only
the requested window. Continuations bind kind, exact name, query and page size.

Operation log windows without an ID-prefix selector MUST walk bounded operation
metadata directly, without loading commits, trees, views or a commit index.
Native responses report `not_used` for the graph cache. Operation ordering,
parent/child selectors relative to `@`, filtering, pinned membership,
authorization and cancellation retain pinned JJ semantics. ID-prefix selectors
retain their existing complete admission until the prepared operation lookup is
qualified. Local reads must not repair or rewrite a damaged commit index as a
side effect of reading operation metadata.

Native commit-graph startup and explicit preparation count actual verified
commit reads rather than materializing the committed inventory for admission.
Crossing the cold inline threshold returns rebuild-required and discards
staging; every warm or newly built index still observes the hard index-size
limit. Syntax work is independent of total retained repository size. This
preserves query identities, native evaluation, signed continuations, current
authorization and explicit resource errors; broader prepared-lookup and
CPU/memory qualification remain separate gates.

Local history preparation has a distinct process-local service task contract
with local snapshot and statistics, never a fabricated native epoch or cache
report. Its default construction deadline is one hour, completed retention is
five minutes and retained task count is 32. Trusted composition may lower these
limits. Current device binding authorizes every operation; running or usable
ready tasks coalesce by repository, generation, metadata path and operation.
Expired/restarted/evicted task IDs reset; they cannot become valid again on
later completion. A cold prepared-log request reports rebuild-required for its
exact operation. HTTP and browser delivery require their own contract and
acceptance before this service capability is advertised.

LocalTrusted code capabilities advertise `index_tasks`. POST
`/v1/local/repos/{repo}/index/tasks` accepts only an explicit operation and
returns a local task; GET `/index/tasks/{task}` inspects it and bodyless POST
`/index/tasks/{task}/cancel` requests cancellation. All three return HTTP200
with a local snapshot/status; expired or unknown IDs return410 `task_reset`.
Bounds remain trusted server configuration, never request fields. Native task
routes and snapshot types remain unchanged.

Local log, detailed commit and evolution POST queries use the retained prepared
generation. A cold request proves its requested operation without commit
preflight, then returns409 `index_rebuild_required` with `local_snapshot`. Both
first pages and cursor resumes retain the original operation, generation and
complete query. Files, references, headers and operation metadata remain
independently readable. A local error must not fabricate a native
snapshot/epoch. The browser validates the error pin and all start/status/cancel
responses before resuming that query.

Native and local browser index-task adapters forward the owning observation's
abort signal for start, status and bodyless cancel requests. Aborting that HTTP
observation is not a server cancellation result. Clients serialize polling and
explicit cancellation for the captured task ID, retain the original operation,
and continue observing an unconfirmed cancel without automatically replaying the
cancel command. Server-confirmed canceled and ready outcomes are distinct.

Local selected commit-summary, path, file-at, diff, changed-path and conflict
queries use an exact ready local generation when available, retaining signed
cursor and current registration checks. Absence alone selects the existing
bounded scoped reader, keeping directly referenced files usable without
preparation. Generation/authorization/selection errors propagate explicitly;
there is no fallback that admits unrelated or later Git objects. These readers
change no HTTP request, response, schema or capability. Their comprehensive
transport/browser qualification remains in the later bounded-read slice.

Local unfiltered Changes/All reference queries whose embedded operation equals
the outer operation use prepared ordered visibility. When cold they return the
existing exact-operation rebuild-required response, allowing the existing
recovery flow. Continuations preserve lexical order and hydrate only page
targets. A different embedded operation or nonempty description search keeps its
previous bounded dispatch until separately indexed. Reference DTOs, signed
cursors and preparation task contracts are unchanged.
