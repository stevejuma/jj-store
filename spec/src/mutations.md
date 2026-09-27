# Typed mutations

Previews do not publish or grant authorization. Stale snapshots/conflicts fail
explicitly. Binary, structural, symlink, executable, and multi-sided conflicts
are not flattened into a two-sided text model. Supported term selection and
deletion are explicit; unsupported editors return a typed explanation. Server
rebase/squash/undo are deferred; client capability evidence is separate.

## Requirements

| ID          | Requirement                                                                                                                                                                                  |
| ----------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| JST-MUT-001 | Typed preview, apply, and status endpoints MUST share canonical intent, exact snapshot preconditions, authorization, and durable receipt semantics.                                          |
| JST-MUT-002 | First release MUST implement New, Describe, bookmark create/move/delete, Abandon, ResolveConflicts, and ReconcileOperations through native transactions.                                     |
| JST-MUT-003 | Abandon MUST enforce protected/root policy, transform descendants, update bookmarks/workspaces, preserve history and evolution, and publish once without deleting objects.                   |
| JST-MUT-004 | Conflict resolution MUST preserve ordered native terms, check fingerprints and authorized uploads, validate all selected paths atomically, rewrite descendants, and retain unresolved paths. |
| JST-MUT-005 | Client-computed RPC publications MUST pass the same semantic protected-history checks as application mutations; no arbitrary shell or CLI argument endpoint MAY be exposed.                  |

The service's publication guard binds the authenticated token principal to the
WAL request actor, validates replacement settings and delegates native
transition validation to an explicit semantic policy. A transport cannot select
a caller's principal independently of its verified credential. No permissive
semantic validator is provided by the service library.

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

Read snapshots, index-task outcomes and committed events export schema traits
for generated HTTP contracts. Schema descriptions are informational and cannot
construct read capabilities, restore credentials or grant publication access.

## Typed native command preparation

The command executor prepares New, Describe and bookmark create/move/delete
through pinned native transactions. It loads one exact current operation, never
implicitly reconciles heads, and leaves the resulting operation unpublished.
Preparation requires current write permission before native reads, staging and
return. It checks the complete expected epoch/position/head snapshot before and
after execution. It is internal execution infrastructure, not yet the public
preview/apply/status API or an authorization grant.

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
M5 completion still requires that flow, transport/UI integration,
reconciliation, abandon and conflict resolution; M4 deployment qualification
remains separate.

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
with the original request ID; semantic rejection remains an explicit error.
These service methods are not yet HTTP/RPC mutation endpoints, and browser work
remains deferred to M4.3.
