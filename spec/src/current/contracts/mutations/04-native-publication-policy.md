# Native publication policy

Native CLI collaboration uses the same typed RPC mutation service and explicit
request IDs as application transports. Unavailable, timed-out, cancelled or
unclassifiable responses remain outcome-unknown under that original identity;
known authorization, validation, stale and changed-intent errors stay errors.
Status and page reads never retry publication or initialize capture state.

Native RPC collaboration requests carry only version, stable request identity
and typed intent. Custom definition uploads have one binary source field and
must not also supply inline JSON bytes. The codec reconstructs the same intent
before authenticated service admission; transport failure preserves the original
request identity and never authorizes automatic execution or publication retry.

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

Native prepared reference windows share graph/content worker admission with
typed mutation previews. Their cached keys and visibility grant no command or
publication capability. Selected-target hydration and resumed signed windows
retain fresh committed membership, current policy and epoch checks. Cold
reference recovery prepares only a disposable exact-operation generation;
cancellation drains its read worker and cannot replay or discard accepted
command receipts.

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

Explicit graph-index construction uses its own trusted resource and lifetime
policy and a retained checked catalog root. This does not expand typed command
budgets or authorize a mutation. The preparation membership cache is bounded;
cache eviction rechecks the original root, never the latest inventory. Current
access and epoch are checked before returning a report even when the initiating
page session has expired.

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
