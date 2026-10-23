# Read APIs and browser

E-STORY-PARITY-001 retains JST-API-001/002 admission, DTO and immutable read
contracts. Browser refresh keeps the identities attached to visible file rows
until replacement publication; selecting a row records its known commit before
reference restoration completes. Source reads still use the admitted operation,
commit and file identities. The shared application root, catalog adapters and
Docker browser preparation add no endpoint, authorization or storage format.

E-CRATE-READMES-001 documents HttpApi composition and links the detailed API
manual. JST-API-001/002 DTO, admission, snapshot, cursor and event contracts are
unchanged; the README adds no endpoint or browser behavior.

E-CI-BROWSER-001 confirms JST-API-001/004 read admission, operation pinning,
mutation receipts and native formats remain unchanged. Browser qualification
follows the advertised application feed, waits for the final CLI operation after
snapshot-plus-describe publication, and exercises the current explicit menu and
parent selection. Work comment reactions invalidate the affected home's visible
task projection; hidden projections retain dirtiness until restored.

Build-time browser manifest generation enforces the existing 256-entry and
16,384-byte inventory bounds before artifacts are admitted. CI prebuilt reads
retain complete same-source scripts, fonts, styles, icons and Help. Grouping
shared helpers changes artifact layout only; public DTOs, operation-pinned
queries, authorization, cursors and event framing are unchanged. Static server
allowlists, byte/path/symlink rejection and CSP remain independently verified.

The repository unavailable-content banner presentation does not alter API read
contracts, unavailable/error responses, repository authorization or retry
semantics. Its collection/surface inset is local UI state; clients do not send
it or receive it in structured service responses (JST-API-001).

Native and local `GET .../commits/{commit}/blame` accept an explicit `at_op`,
canonical file `path` and optional opaque `continuation`. The shared versioned
response identifies repository, operation, commit, path and file; contains line
count, one-based end-exclusive attributed ranges, deduplicated genuine commit
summaries, completion, continuation and work counters; and never retransmits
source. Cache state cannot authorize reads. Expired progress returns an explicit
`blame_reset`; clients may restart only the same pinned selection.

The browser accepts `file_view=source|preview|blame` only on Files with a
selected file, commit and operation. Source causes no annotation request. Blame
uses one sequential request at a time and the existing scoped query cache.
Syntax highlighting derives from the existing file-content response and filename
on the client, including while origins remain pending. It adds no blame requests
or response fields and preserves the source line offsets used by attributed
ranges.

The top Stacks conflict banner is scoped to the active Workspace tab. This
presentation condition adds no transport fields or commands and does not change
existing conflict actions in focused history or local workspace configuration.

The browser may reuse successful local names-only path responses for 30 seconds
in its existing session cache, keyed by repository, registration generation,
captured head and the exact operation/commit/path/window request. Closed sources
cannot serve cached content. Retained directory rows and attribution progress
remain within the current controller read scope; refresh or source/selection
replacement clears them. Native and local history batches remain transient so a
cached pending batch cannot delay further progress. No cache grants server
authorization or extends an opaque cursor's lifetime.

Local code requests may reuse server-owned folder validation during bounded
traversal. Full registration and physical identity checks remain at read entry
and completion, with short-lived checks renewed during long work. Cache hits
never grant object membership or turn failed admission into snapshot metadata.
Wire responses, operation pins and cursor claims are unchanged.

Stacks conflict presentation reuses existing native conflict data and resolution
commands; conflict-count chips and session-only notification action variants
introduce no HTTP/RPC request fields or persisted configuration changes.

LocalTrusted native/local capabilities MAY advertise `automatic_code_index`;
absence or false retains explicit preparation. Additive native and local
`code-index/tasks` start/status/cancel routes use existing versioned index-task
reports and exact-operation bodies. Native start returns202; local start retains
its200 contract. Task control reauthorizes the repository and epoch/generation;
restart, expiry or evicted readiness reports task reset. Authenticated serving
rejects automatic core routes. Existing `index/tasks` advanced preparation
routes and `index/kinds/tasks` remain compatible and explicit. Ordinary browsing
does not depend on their full projections.

The browser automatically starts or observes core preparation after a core read
needs it, retains the selected operation and navigation, and retries that view
once readiness is confirmed. Failed or canceled episodes remain explicit; do not
spin retry loops. Navigation aborts obsolete observation. Files-first loading,
independent best-effort metadata and bounded bookmark windows remain intact.
Available path metadata adds `attribution: path_change | snapshot`. Snapshot
values describe the exact selected commit when the 64-step first-parent scan
finds no path change; clients label this provenance and stop repeating quick
metadata reads. After quick results, Files automatically requests
`last_change_mode=history` batches to refine them through the existing JJ file
predicate. Missing mode defaults to `quick`; it is bound into native and local
cursor queries. Pending history retains all displayed quick values. A direct
local path anchor can start a metadata seek, but cannot override a signed
cursor. Authorization, integrity, cancellation and hard resource failures remain
errors. Ordinary Commits recovery includes exact-operation core task
observation. Revision, operation and evolution graph columns scroll both axes in
their existing viewport. Row content follows each row's occupied lanes while
vertical virtualization retains the same bounded page and selection. Horizontal
movement does not request additional history or change operation pins.
Commit/diff navigation in Commits and Branches uses the pinned commit directly
without resolving an unused change-reference inventory. Files reference
selection retains its required target validation. Ordinary setup presents
automatic history maintenance and hides advanced inventory controls when this
capability is enabled.

Repository preparation, cancellation and read-state feedback uses persistent
application toasts with the existing request actions. Pending content uses
skeletons without issuing replacement commands or changing selected operations.
Dismissal never cancels a task; cancellation retains its original task identity.

The native browser can explicitly prepare operation or copy inventories at its
selected operation through the existing kind-task routes. One preparation flow
owns one task and at most one status/cancel request at a time. Validate task ID,
repository, operation, kind, bounded progress and terminal completeness before
restoring reads; never substitute current heads. Native kind progress may poll
with bounded backoff even while SSE is healthy because descriptor windows emit
no events; settlement hints wake the exact task. Graph preparation keeps its
existing event-driven behavior. Navigation clears obsolete observation and
aborts pending transport; server-owned task expiry remains independent. Cancel
is acknowledged by the server, and unconfirmed cancellation remains retryable.
Local repositories do not advertise these native inventory controls.

Native kind preparation has separate process-local tasks at
`POST /v1/repos/{repo}/index/kinds/tasks`, with GET `/{task}` and bodyless POST
`/{task}/cancel`. Start accepts only an explicit operation and kind
(`operations` or `copies`); user budgets, source roots, actors and credentials
are rejected. Start returns HTTP202, status/cancel HTTP200. The versioned report
contains its original snapshot, kind, state and bounded descriptor/read/byte,
window and matching-ID counters, never the identities or a cursor capability.
Running starts coalesce by principal, repository, epoch, operation and kind. All
three require current Read and epoch; only the owner may cancel. The task uses
the same reserved background worker as native/local graph preparation, with
non-queuing admission before capture. Each window reauthorizes the original
source/root and remains within its call and fixed total budgets. Complete-root
traversal alone promotes the rebuildable cache. Running expiry cancels pending
I/O without polling; cancellation and teardown retain admission until job exit.
Completed reports retain no source or credentials and expire after five minutes.
Restart/expiry returns task-reset. Settlement/expiry emit scoped index-task
freshness hints; descriptor windows emit no event flood. Ordinary GET ceilings
and graph task contracts remain unchanged.

Prepared native/local operation pages may traverse compact captured topology
before hydrating source metadata. Deep anchors and merge lookahead must not read
prior operation bodies, views or commits. The anchor must occur in the selected
start's exact JJ order and match the requested metadata filter. Response rows,
probe semantics, operation pins and signed cursors remain unchanged. Compact
order for the captured selection is retained once under generation admission;
selected-start pages seek to the anchor without replaying earlier edges. Fresh
authorization, metadata integrity, matching-anchor and page/probe checks still
apply. Historical-start pages may resume bounded retained compact checkpoints
selected by exact start and anchor. They recheck current source metadata and
filter membership, retain the state before the probe, and preserve repeatable
page/cursor results. Cache loss retains the ordinary bounded fallback; oversized
candidate windows cannot be reported as complete.

Local operation graph requests may reuse an existing prepared generation even
without a historical prefix or child selector. Their fresh selection seal uses
only operation metadata, and each selected row retains its canonical fingerprint
check. Ready operation pages require no view or Git reads; supported cold pages
remain usable without preparation. Exact pins, request/cursor semantics and code
readers' separate view admission remain unchanged.

Local operation-difference and change-page queries use an exact ready generation
with independent retained operation/parent admission and fresh canonical source
checks. Cold queries return the existing rebuild-required selection; ready
queries follow bounded differing index frontiers and hydrate only selected
change groups. Existing complete-result, explicit parent selection and signed
cursor semantics remain unchanged.

Repository directory discovery and metadata loading are independent browser
reads. Discovery may display authorized names without fetching every card's
code/WorkStore/ReviewStore pages. Viewport-owned metadata requests share two
actual concurrent directory reads, propagate cancellation to their transport,
preserve independent pins and discard obsolete results. Existing paging,
authorization and partial-activity contracts remain unchanged; complete browser
qualification remains a separate slice.

Repository code summaries MUST read only the selected operation/view and a
bounded first window of its lexically ordered nonvirtual commit heads. Native
`GET /v1/repos/{repo}/code/summary` and the local operation-pinned
`code_summary` query accept 1..256 heads; cards request 50. The response
contains the window's commit count and the commit metadata with maximum
committer time within that window, with stable first-ID tie breaking. It MUST
mark the count incomplete if heads are unreturned or any returned commit has a
nonvirtual parent, without claiming a complete visible-history count or
repository-wide latest time. Empty-root and fully returned root-child heads may
report complete counts. No ancestry traversal, commit graph/index construction,
tree read or reference presentation is required. Native/local identity widths
and fresh pin checks remain explicit; ambiguous operation heads are not
reconciled and their card counts remain incomplete. This head-window contract
replaces the card's former first-50 topological graph sample; full history
remains a separate graph query.

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

Review-activity responses MAY reuse count/time facts from the exact admitted
root and ordered first window. Every native/local request retains fresh provider
admission and its own complete snapshot envelope; cache hits do not replay a
manifest or companion pin. Public routes, schemas, finite limits and sampled
count/time semantics remain unchanged.

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
caller cancellation releases read ownership synchronously, so an immediate
identical retry cannot inherit its obsolete transport. Shared transport remains
active until the final observer releases it. Exact-target invalidation matches
the query arguments independently of the trailing epoch key. Confirmed reactions
invalidate and reload only their exact target, retaining other reactions,
messages, reply context, emoji definitions and raster URLs. Unknown outcomes
keep the original intent and receipt identity. Explicit refresh invalidates
displayed collections; SSE queues automatic active-view refresh. Reaction reads
are deduplicated across main/reply views and limited to four concurrent
requests. Reply counts and descendants use indexed passes without changing
loaded-history, exact revision, draft or scroll semantics. Unneeded JJ
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

The shared inbox navigation/filter presentation changes no JST-API-001 wire
contracts, authorized read scopes, cache invalidation, pagination or retry
behavior. Facets use only loaded exact identities; source/view filtering does
not initiate additional reads or broaden repository access.

## Public release integration

CLI version display and Cargo package naming do not change HTTP/RPC schemas.
Generated contracts and actual fixture captures remain release gates; cached
build artifacts never substitute another source revision.
