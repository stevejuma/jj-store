# Typed mutations

An authorized read can produce a credential-free, immutable selected-view commit
set for search filtering. Only the service constructs this value after native
traversal and a final authorization check. It is informational, cannot publish,
and cannot replace the fresh authorization required by every search request.

Previews do not publish or grant authorization. Stale snapshots/conflicts fail
explicitly. Binary, structural, symlink, executable, and multi-sided conflicts
are not flattened into a two-sided text model. Supported term selection and
deletion are explicit; unsupported editors return a typed explanation. Server
rebase/squash/undo are deferred; client capability evidence is separate.

## Requirements

| ID          | Requirement                                                                                                                                                                                                                                                        |
| ----------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| JST-MUT-001 | Typed preview, apply, and status endpoints MUST share canonical intent, exact snapshot preconditions, authorization, and durable receipt semantics.                                                                                                                |
| JST-MUT-002 | First release MUST implement New, Describe, bookmark create/move/delete, Abandon, ResolveConflicts, and ReconcileOperations through native transactions.                                                                                                           |
| JST-MUT-003 | Abandon MUST enforce protected/root policy, transform descendants, update bookmarks/workspaces, preserve history and evolution, and publish once without deleting objects.                                                                                         |
| JST-MUT-004 | Conflict resolution MUST preserve ordered native terms, check fingerprints and authorized uploads, validate all selected paths atomically, rewrite descendants, and retain unresolved paths.                                                                       |
| JST-MUT-005 | Client-computed RPC publications MUST pass the same semantic protected-history checks as application mutations; no arbitrary shell or CLI argument endpoint MAY be exposed.                                                                                        |
| JST-MUT-006 | EditFile MUST create one child change from the exact single-target bookmark, replace or rename one existing bounded UTF-8 file, preserve native file mode/copy identity, move that bookmark atomically, and retain durable preview/apply/status receipt semantics. |

The service's publication guard binds the authenticated token principal to the
WAL request actor, validates replacement settings and delegates native
transition validation to an explicit semantic policy. A transport cannot select
a caller's principal independently of its verified credential. No permissive
semantic validator is provided by the service library.

The same guard accepts a principal only through the shared access-policy
boundary. LocalTrusted derives that principal from the installation and verified
local peer, with one explicitly configured permission; authenticated access
continues to derive it from the repository token. Existing WAL requests and
receipts store only the resulting `PrincipalId`, so the access-mode addition
does not rewrite historical records or weaken protection, snapshot, idempotency,
semantic or resource checks.

AgentStore domain publication uses the same guard with current write
authorization. The guard compares the domain request actor with the principal
derived from the current access policy before receipt lookup, staging or CAS.
The prepare service rechecks that principal after staging immutable objects; a
later publish repeats authorization, and a peer cannot replay another
principal's prepared request. Domain writes do not run native history semantics
because they preserve code heads and code-state version through their separate
manifest root.

ReviewStore commands use a separate typed service over that same guarded domain
publication. The HTTP request carries only version, request ID and validated
semantic command. The service derives principal and repository-bound publication
ID, validates the committed result bytes and exact receipt identity,
reauthorizes after publication or status lookup, and emits only a freshness hint
after commitment. Apply returns `outcome_unknown` when commitment cannot be
classified; status returning `not_known` is never treated as rejection.

## Native publication policy

Native publications must descend from every removed committed operation. Strict
publications must retain operation ancestry for all prior heads. Newly
introduced operation graphs are bounded and anchored in committed operations;
root-only rollback, unanchored history and parentless non-root objects are
denied. This preserves concurrent siblings while allowing native operation
reconciliation.

The server's trusted protection rules name immutable commit tips and bookmarks.
Their native ancestor closure, always including the virtual root, cannot be
hidden or rewritten. A new object sharing a protected change ID is also a
rewrite and is denied even when a crafted request omits predecessor metadata.
Multiple already-protected divergent incarnations may be retained exactly;
protecting their change ID does not arbitrarily select one of them. Protected
bookmark values cannot be moved or removed. These rules apply equally to direct
native publication and application transitions; they are independent of token
permission. Protected-history checks use native objects rather than trusting
client operation descriptions. Rule configuration is server-owned and never
accepted from a publication request.

A new request cannot resurrect a committed operation that is no longer a head;
identical receipt retries are resolved before this check. Publication also
checks new workspace-name claims against current heads, so concurrent physical
clients cannot silently acquire the same logical workspace name. Reconciliation
can retain mappings inherited from its parent operations.

The native transport facade derives publication actors from repository tokens
and accepts only native removed-head/added-operation transitions. It does not
expose settings mutations or an unguarded coordinator. RPC head updates
therefore use the same protected-history checks and durable retry receipts as
local sessions.

Token administration uses strict snapshot preconditions and a stable request ID.
Grant and revoke reconstruct their exact original intent from the retained
policy at the expected committed snapshot, allowing receipt lookup after later
publications or checkpointing. Different actions under the same identity
conflict, including revoking different already-absent token IDs. Concurrent
administrators cannot overwrite each other's accepted policy changes. This typed
administration boundary does not expose raw settings publication.

After a committed grant or revoke receipt, server composition updates the
installation token-ID candidate index for that repository. This derived update
does not alter the receipt, settings bytes or publication result. A stale or
replayed candidate is harmless because discovery reauthorizes the complete
credential against current repository policy before revealing metadata.

Native RPC administration exposes only typed grant/revoke requests and exact
snapshot metadata. The facade returns secret-free token receipts, or an unknown
outcome retaining the caller's request identity. No receipt observed by status
means pending/unknown, never proof of rejection. Clients validate receipt
request identity and action against the submitted intent before reporting
success.

Application reads use a separate committed-snapshot capability with no mutation
methods. Selecting among concurrent heads never invokes native reconciliation;
authentication and epoch checks run again on later reads. Reconciliation remains
an explicitly authorized native or typed mutation.

The read capability supplies the native graph executor with current
authorization and its immutable object membership. The executor's backend denies
every write and head transition, even when the originating credential has admin
permission. Derived index writes are local cache maintenance, not repository
publication.

Graph continuation pins retain only checked snapshot data and object membership.
They contain no native session or credential. A new currently authenticated
session is required to resume, and epoch checks remain mandatory. Continuations
cannot create a mutation session or select an uncommitted inventory.

Committed reads can request owned object DTOs, directory pages and verified file
streams. These operations share the same captured membership and current policy
checks as graph reads and expose no publication or staging method.

Directory continuation pins share the graph cursor authority boundary. A fresh
currently authenticated session is required on every page, and no cursor can
switch between graph and directory queries or acquire publication capability.

Operation differences use the same committed read capability and expose no
transaction or publication method. A selected comparison parent is validated
against the native operation before loading its view.

Conflict descriptors bind ordered native terms and labels to repository, epoch,
commit, change and path through a domain-separated fingerprint. The read-side
term position and fingerprint are data for later atomic resolution validation;
neither a fingerprint nor an editor-support result authorizes a mutation.

A committed read may prepare its native index through the bounded query
executor. It supplies the same current authorization and captured membership as
graph reads, and checks authorization again before exposing the preparation
report. Preparation has no repository publication capability.

The index-task registry contains only exact read-snapshot metadata, initiating
principal IDs, cancellation handles and owned outcomes. Full inventories and
request credentials live only in bounded running jobs and are dropped before
completion. Task status/cancel calls authenticate independently; the task ID is
neither a read capability nor a publication receipt.

The native transport binding can transfer its authenticated session into the
application service boundary. HTTP receives that binding with the same token,
repository, policy and limits; it cannot select an actor or replace the guard.
Owned read DTOs and verified range types are exported through the service for
transport consumers without exposing upstream JJ types.

Successful native and token-policy publication wakes bounded process-local event
subscribers. Wake hints contain only repository identity and do not establish
commitment. Event readers reauthorize and follow selected WAL records; unknown
outcomes and writes by another process are recovered by periodic manifest reads.
No subscriber callback is awaited by publication.

Replication import is a restricted data-only mutation capability. It verifies
the complete offered closure and can expose imported code, AgentStore and
ReviewStore state only through the destination's durable manifest CAS and
receipt path. After that publication commits, it emits the same repository-only
freshness wake as a local mutation and exposes the destination-local WAL
position to rebuildable search adapters. Import cannot execute tools, hooks,
controllers, messages, Git or Nostr work, and imported origin positions never
replace the destination's issuer-bound WAL position.

Read snapshots, index-task outcomes and committed events export schema traits
for generated HTTP contracts. Schema descriptions are informational and cannot
construct read capabilities, restore credentials or grant publication access.

## Typed native command preparation

The command executor prepares New, Describe and bookmark create/move/delete
through pinned native transactions. It loads one exact current operation, never
implicitly reconciles heads, and leaves the resulting operation unpublished.
Preparation requires current write permission before native reads, staging and
return. It checks the complete expected epoch/position/head snapshot before and
after execution. It is internal execution infrastructure consumed by the durable
preview/apply/status service below; a prepared report grants no authorization.

New preserves ordered explicit parents and uses native tree merging. An optional
existing logical workspace can move to the new commit with native edit behavior;
no physical working copy is created. Describe retains change identity and uses
native reparenting for descendants, keeping their trees intact and updating
bookmarks/workspaces. Existing author identity follows the pinned rewrite
policy: missing fields are filled from the committer, and discardable changes by
that author refresh their timestamp. The committer and new-commit author
identify the authenticated principal using the server's fixed identity policy,
without loading user configuration or signing programs. Root and hidden rewrite
targets are rejected.

Bookmark mutations compare the complete expected ordered native target,
including conflicted and absent terms. Create requires absence, move/delete
require an existing target, and stale expectations fail without a prepared
operation. Reports identify native predecessor rewrites and exact bookmark and
logical workspace changes. They do not constitute durable mutation receipts.

Preparation uses bounded worker admission, metadata/byte/work budgets and
cancellation. Reads are limited to the captured committed inventory plus native
objects staged by that one command. Only conditional immutable native writes are
possible; manifest writes and head publication are denied. Canceled workers
retain their permit until they exit. Later application publication must pass the
existing strict snapshot and semantic protection guard and durable retry flow.
The durable service below supplies that publication flow to each typed command
and the HTTP transport. M4 deployment qualification and browser controls remain
separate gates.

The user deferred further UI work on 27 September 2026. M5 backend command,
preview/apply/status, RPC policy and mutation API gates continue; their browser
work is retained under deferred M4.3 rather than counted as backend completion.

## Durable command service

The service's typed request includes a version, client-generated request ID,
exact expected snapshot and a validated command. Its principal comes only from
current repository write authorization. Canonical intent includes all original
request fields under the application WAL intent profile; native preparation
results and timestamps do not alter that identity. Publication IDs are derived
from repository, principal and request ID under a distinct application domain.

Preview prepares a bounded native impact report, then applies the same semantic
protection and dependency checks as publication without writing authority. Its
native result IDs are provisional; apply recomputes native execution at the same
snapshot and revalidates authorization and semantics. An already committed
identical request returns its retained receipt. Preview never grants permission
or reserves an operation.

Apply looks up a currently authorized receipt before executing and publishes the
candidate and exact result with one guarded strict manifest CAS. Racing
identical applications return the selected result, including if preparation
discovers the snapshot has moved. Different intent under the same identity fails
explicitly. A selected result retains its versioned original request and native
impact report so status/retry can reconstruct and check the canonical digest
after restart or checkpointing, without an execution cache or separate
intent-binding authority.

Status authenticates current write permission, resolves only the same
principal's request key and validates the selected result's repository,
identity, digest and accepted snapshot. No selected receipt is NotKnown, never
proof of rejection. Transport/storage uncertainty returns an unknown outcome
with the original request ID; semantic rejection remains an explicit error. The
HTTP mutation endpoints consume these service methods; native RPC continues to
use its guarded native publication facade. Browser work remains deferred to
M4.3.

The HTTP boundary consumes the durable command service for preview/apply/status.
Current write permission precedes body decoding and is checked again by the
service. HTTP admission and native command preparation share bounded worker
capacity with reads. Tagged outcomes retain the original request ID across
unknown transport outcomes; status is scoped to the authenticated principal. No
browser mutation controls are introduced by this transport slice.

## Explicit operation reconciliation

ReconcileOperations names an explicit base operation from the exact expected
current head set. All current heads participate; the base is the first merge
parent and the comparison view for reported bookmark/workspace changes. It is
not a caller-selected merge ancestor. Pinned JJ computes common ancestors,
including recursive merges, and finishes descendant rebases. Single-head,
missing-base and stale/subset contexts fail without publication. A changed base
under an already committed request ID is conflicting intent.

Preview stages the native merged operation without publishing. Apply uses the
same strict guarded receipt path and emits HeadsNormalized after manifest
selection. Native command/report persistence retains its existing V1 fields;
adding the command variant does not change canonical bytes for older receipts.
Reconciliation shares command worker and metadata/staging/deadline limits. Its
reachable operation graph is preflighted and a conservative operation-pair/head
work budget is reserved before native recursive merge execution.

## Native abandon

Abandon accepts bounded distinct full commit IDs plus explicit retain_bookmarks
and restore_descendants options. Committed targets that are already hidden are
reported as skipped. No visible targets returns no_changes without publication;
root and protected transitions are denied. An absent/uncommitted target fails
the whole request. Native descendant transformation abandons the visible
targets, rebases descendants by default or reparents them while preserving
content when restore_descendants is true, and updates bookmarks and all logical
workspaces. Retaining bookmarks moves their native targets to replacement
parents; otherwise abandoned bookmark terms are removed using pinned native
semantics. Abandoned working-copy changes are replaced with new empty changes,
shared across workspace mappings that previously pointed to the same commit.
Physical files are untouched.

Reports include abandoned and skipped_hidden IDs, descendant rewrites and exact
bookmark/workspace changes. The two added report fields default to empty and are
omitted when empty, preserving canonical decoding/reencoding of earlier V1
receipt results. Objects and predecessor/evolution dependencies remain retained.
Preview remains unpublished and apply selects one guarded operation and receipt.

## Atomic native term resolution

ResolveConflicts names one visible commit and 1–256 distinct conflict paths.
Every selected path includes its complete read-side fingerprint and an explicit
SelectTerm (zero-based ordered native term) or Delete action. All paths and
fingerprints are checked against the original tree before writing any
replacement tree or commit. Root paths, malformed or overlapping paths,
non-conflicts, stale fingerprints and missing term positions fail atomically.
Selecting an absent term explicitly deletes the path; selecting a native file,
symlink, submodule or directory retains its exact identity, executable bit and
copy data. Remove terms may be selected explicitly, including the native base
version.

One native rewrite keeps the target change identity, rebases descendants and
updates bookmarks and logical workspaces. Unselected conflicts retain their
native values and labels, subject to native merge simplification. Preview does
not publish; apply uses the existing strict guarded receipt flow. The command
has no editor or shell execution. Authorized uploaded replacements use the
separate checked context described below; caller-selected file IDs are never
accepted as upload authority.

The service exports the owned ConflictResolution and Resolution types alongside
Command, so API consumers can construct typed intent without importing native JJ
integration internals.

## Authorized replacement uploads

A bounded binary upload stages file content under current repository write
permission. Its immutable V1 binding records repository, principal, the exact
single-head snapshot, target commit, path and conflict fingerprint. The upload
handle is a domain-separated SHA-256 digest of the canonical binding plus the
server-computed native file ID and byte length. Records live under the
repository/epoch/principal staging namespace, separate from native objects and
manifest authority. Native RPC cannot write that namespace. Knowing a native
file ID or another principal's upload handle does not authorize reuse.

Upload admission validates path shape, current snapshot and committed target
membership before consuming bytes. The bounded stream is spooled privately;
short declared bodies, over-limit content, deadlines and authorization changes
fail explicitly. Current authorization and snapshot are checked before remote
writes and completion. Repeating identical bytes with the same binding returns
the same handle, including after lost acknowledgement or server restart. Staged
records are not committed history, have no listing API and grant no read access.
They remain unusable after the snapshot advances; destructive staging cleanup is
outside this release.

UploadedFile resolution supplies the checked handle, an explicit executable bit
and an optional copy_from_term position. The latter must select a native file
term; omission uses the native copy placeholder. No caller-selected native file
or copy ID is accepted. Resolution checks the record's integrity and complete
scope, verifies the native file bytes and validates every selected path before
rewriting. Uploaded bytes may resolve text, binary or structural conflicts into
an ordinary file; selecting symlink/submodule/directory terms remains separate.

A new request with a stale binding publishes nothing. Identical selected
mutation receipt replay precedes upload lookup, so checkpointing and later
staging loss do not invalidate an acknowledged mutation. Native report and prior
command receipt encodings remain unchanged. General purpose and binary editors
remain deferred under M4.3.

## Repository text file edits

EditFile names an exact local bookmark target, existing source path, destination
path, UTF-8 replacement text and new-change description. The bookmark MUST be a
single resolved target matching the command expectation. Source and destination
paths use the canonical repository-path rules, the source MUST resolve to an
ordinary non-conflicted file, and a rename MUST NOT replace an existing entry.
The replacement text is capped at the native text-read limit.

The command preserves the source executable bit and copy identity, writes a new
file object, creates one child change and moves the selected bookmark to that
child in the same unpublished transaction. It does not rewrite the selected
commit or its descendants and never touches a physical working copy. Preview,
apply, unknown-outcome recovery and replay use the existing stable request and
durable receipt contract. A protected bookmark or stale target fails before
publication.

## Conflict resolution reports

A new resolution preview and selected receipt include a conflict_resolution
report with exact before/after target IDs, sorted selected_paths and a remaining
map keyed by every resulting rewritten or created commit ID. Each map value is
the complete sorted native conflict-path list for that result, including an
empty list when none remain. This covers the target and all affected
descendants; it does not claim to scan untouched history. Tree/file conflicts
remain one native path. Selecting a directory may reveal nested conflicts, which
are reported instead of treating the selection itself as proof of resolution.

Enumeration shares command authorization, metadata, work, output and deadline
budgets. Oversized or unreadable results fail preparation without publication;
there is no truncated success report. Preview IDs remain provisional and the
selected receipt retains the actual report for status and retries. The optional
field is absent from non-resolution reports and defaults to absent when decoding
older receipts, preserving their canonical V1 bytes. Absence means no report was
recorded; it does not mean conflict-free.

A service session can supply an authenticated read-only adapter source. It
checks current repository read permission and authority epoch through the same
token policy as reads and mutations; a saved adapter checkpoint grants no
access. Adapter cursor writes use a dedicated non-authoritative namespace and
cannot change repository heads, receipts, settings or manifest authority.

The service's adapter projection reader reuses the native content engine over
verified snapshot objects. It restricts paths to that inventory, verifies
framing and native identity, and repeats current authorization at storage/read
boundaries. Neither raw object storage nor publication methods are exposed to
extraction.

The M4.10 committed-read service extension adds paged references, recursive
paths, operation-change groups and changed-path summaries. It accepts no command
intent, stages no native object or upload, creates no receipt and cannot invoke
publication. Mutation authorization and guarded apply semantics remain
unchanged.

The M9.4 AgentStore session pager is another read-only service capability. It
reauthorizes every page, resolves summaries only from one audited selected
AgentStore root and retains no principal credential in its process-local cursor.
The pager exposes neither a typed AgentStore delta nor a request/publication ID,
receipt lookup, staging method or publication coordinator. Resuming a cursor
therefore cannot create a collaboration write or dispatch an agent.

Stable-session detail uses the same current-root pin and authorization boundary.
It performs one typed session-catalog lookup and returns only service-owned
summary and immutable-link DTOs. Missing sessions return no object data; a path
ID, selected snapshot ID or returned dependency ID grants no read or publication
capability.

Transcript paging verifies that the selected session owns the requested manifest
and source generation, then converts only already-sanitized chunks into bounded
service DTOs. Its signed continuation retains an immutable root and range
position but no credential, typed delta, request/publication identity or receipt
capability. Reading or resuming a transcript cannot stage or publish a
collaboration object.

Association traversal selects one typed session, tool, operation, revision or
checkpoint subject and returns only immutable checkpoint-version and association
links already selected by the audited AgentStore root. Its cursor retains that
exact subject and root-local page position. The route has no correction command,
typed delta, staging handle, receipt or publication identity, so an empty or
unexpected relation page cannot be repaired through this read surface.

Checkpoint collection and detail are also read-only. Current detail resolves one
stable checkpoint through the selected catalog; immutable detail can only walk
that selection's validated prior-version chain. Neither route accepts a
replacement, correction, expected selection, command identity or publication
request. Reading a prior version therefore cannot reselect it or supersede the
current checkpoint.

Review thread collection and detail are read-only under the same separation.
Their cursor retains an audited ReviewStore snapshot without a credential or
publication handle. Detail resolves only the stable thread's current catalog
selection and its already selected anchor, comment and event objects. The routes
accept no expected version, review command, request/publication identity or
typed delta, so they cannot reply, edit, resolve, re-anchor or publish.

The shared ReviewStore command request, outcome, receipt and status DTOs support
both serialization and deserialization so HTTP and native RPC use one public
contract. Transport callers provide only the versioned request ID and typed
command. The service still derives principal and publication identity, enforces
exact workflow/revision/thread preconditions and decodes only the selected
durable receipt. A committed command emits the existing repository-only wake
hint after publication; collaboration bodies and credentials never enter it.

## Browser mutation orchestration

The authenticated browser constructs only the existing typed V1 command union.
Preview, apply and durable status use one stable request ID and exact captured
epoch, position and operation heads. A preview is provisional, an unknown
outcome retains the request for retry or status lookup, and only a committed
receipt can refresh the selected snapshot. Disconnect, token replacement and
revocation cancel outstanding work and prevent late responses from restoring
protected state.

Semantic protection is distinct from authentication failure at the HTTP
boundary. A protected bookmark, commit or root transition returns the typed 409
`protected_transition` code and keeps the authenticated session active. Invalid
or revoked credentials remain 401/403 `access_denied`; the browser disconnects
and clears protected cache state for those responses. Both cases publish
nothing.

Strict code mutations compare epoch, code-state version and exact native heads.
The global WAL position remains receipt and ordering evidence but is not itself
a code-staleness predicate, because AgentStore and ReviewStore publications may
advance it without changing native code. Conflict uploads carry the same
code-state token as their captured head, while zero-valued legacy receipt and
upload snapshots retain their original canonical bytes.

Collaboration context discovery remains a read-only service path. Session timing
is derived only from normalized events and their exact selected observation
evidence, with at most 4,096 candidate events inspected; it cannot stage an
object or invoke manifest publication. Browser review replies, resolve and
reopen continue to use the existing receipt-backed ReviewStore command path and
retain outcome-unknown request identities.

## WorkStore service and transports

WorkStore typed commands derive authenticated actors and repository-bound
publication IDs, validate entity/project versions and retain durable status
semantics. Home writers control task/comment metadata. Shared service requests
accept only version, request ID and the implemented command union.

Git bridge acceptance is a typed application mutation over the same native
session permission and semantic guard as CLI head publication. It binds exact
immutable facts and optional code heads, derives the actor from admitted
transport identity, verifies representation bytes/mappings before acceptance,
and preserves stable request identity through unknown outcomes. Freshness hints
fire only for committed acceptance. Reads and receipt lookup reauthorize current
repository access and never perform Git transport.

Git state reports whether explicit bridge activation is currently qualified for
the selected domains. The CLI can check this immutable snapshot before normal
working-copy snapshotting or endpoint execution; it grants no additional access.
