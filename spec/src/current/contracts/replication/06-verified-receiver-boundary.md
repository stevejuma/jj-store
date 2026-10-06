# Verified receiver boundary

The replication package exchanges manifest-bound origin feeds, authenticated
origin offers and bounded immutable closures. Closure transfer conditionally
installs only digest-verified native objects and control records. It never
installs a manifest, WAL candidate or receipt; `apply_origins` remains the sole
destination publication boundary.

For native objects, the inventory's envelope digest is SHA-256 of the canonical
envelope prefix and payload. It is not SHA-256 of the complete stored bytes,
which also contain the digest field. Both transfer paths verify the header's
object key, declared length, inventory digest and streamed payload before
upload; a malformed envelope cannot be treated as a complete closure.

The native authenticated RPC exposes negotiation, committed-feed, offer, closure
and apply methods. Apply requests carry stable request/publication identities
and origin envelopes but no principal; the server derives the actor from the
authenticated session and requires repository write access before the receiver
reads a transferred closure. Feed pages are bound to the negotiated issuer
replica, log epoch and fixed upper position, so later source writes do not
extend an in-flight scan. A changed epoch or issuer fails the scan instead of
silently reinterpreting its cursor.

Replication control-record streaming accepts only immutable catalog, inventory,
AgentStore, ReviewStore and Git data records. Both peers verify the exact record
digest and repository/kind envelope. Manifests, WAL candidates, settings,
checkpoints, receipts, replication roots and origin receipts cannot enter this
upload path. Native object streaming retains its existing typed object
validation.

Peer progress is a separate CAS-selected transport checkpoint, scoped to the
logical repository and exact local/peer replica pair. Pull and push positions
each retain their issuer, log epoch, acknowledged position and observed upper
bound. An update cannot move backwards, change an issuer/epoch in place or erase
prior progress. An unknown storage response is successful only when a readback
returns the exact proposed checkpoint. This pointer is not selected by the
repository manifest and never produces an origin or content WAL event.

The bidirectional worker alternates bounded pull and push pages. Each direction
negotiates an issuer, epoch and finite upper bound, offers every delivered
origin, transfers only missing digest-verified closure objects through bounded
disk spools and advances its checkpoint only after durable apply receipts or
exact already-known receipts. Delivery replicas may forward origins authored by
another replica; the feed cursor remains bound to the delivery issuer while the
origin key and digest remain unchanged. Apply request and publication IDs are
derived from the exact missing origin batch and therefore remain stable when a
request fails before acknowledgement. Push and pull have independent retry and
circuit state, deterministic bounded jitter, alternating fairness, pause and
wake controls, observable status and waits against explicitly captured finite
frontiers.

`apply_origins` validates the complete native and control closure before its CAS
loop. Within that loop it authorizes the request, resolves a stable import
receipt before reducing new data, rejects origin-key digest reuse and orders
unseen origins only when every stream predecessor and sparse causal dependency
is present. One destination WAL candidate then selects the reduced content,
accepted-origin catalog, stream prefixes and one destination receipt per
original origin. A repeated batch returns those receipts without advancing the
WAL or a domain state version; an unknown manifest result is recovered through
the same request identity.

Code reduction adds only the operation heads explicitly named by accepted
origins. Production native sessions pass those candidates through the pinned
`jj-lib` operation-parent walk, removing delayed ancestors while retaining
concurrent siblings. Import and reads do not create a reconciliation operation.
The authenticated and LocalTrusted native RPC surfaces expose one exact current
snapshot plus receipt-backed application mutation and status methods. These
methods reuse the single application `CommandService`; the server derives the
principal and publication identity from the admitted session.
`jj sync reconcile` acts only when the exact snapshot has multiple heads,
chooses the first canonically sorted current head as the merge base and derives
a stable request ID from the repository and complete snapshot. It merges the
native views, including distinct workspace targets, through
`ReconcileOperations`. Repeated requests recover the same receipt. Snapshot/head
reads, origin import and the replication worker never invoke this command.
AgentStore reduction applies the exact authored typed deltas as accepted facts.
Disjoint sessions, tools, evidence, event batches, transcripts, projections and
relations form bounded canonical unions. Causal descendants supersede checkpoint
and association ancestors; source-state descendants require receipt prefix
continuity. Terminal tool observations dominate nonterminal ones, while
incomparable terminal outcomes and legacy alias collisions select explicit
catalog-conflict objects containing every maximal alternative. A normal authored
delta cannot directly create those internal conflict selections. Acceptance MUST
exercise sessions, aliases, checkpoints, associations, relations, sources and
projections under forward, reverse and duplicate delivery and MUST derive the
same AgentStore root for the same accepted origin set.

ReviewStore reduction combines immutable thread events as a bounded causal
graph. It retains independent replies, reduces workflow state only from causal
coverage, and selects explicit comment-conflict objects for concurrent maximal
revisions. A resolution is a new revision that names the complete sorted set of
superseded alternatives; a stale or incomplete set cannot commit. Relation and
status catalogs are rebuilt from the reduced core selections in the same
destination CAS. Destination WAL order and accepted timestamps never select a
winner. Acceptance MUST cover concurrent tombstone/edit and reply/reply delivery
in both orders, prove every maximal revision or reply remains reachable and
prove replay does not publish another ReviewStore state.

Both collaboration reducers operate only inside the restricted replication
capability. Import cannot enter ordinary publication authorization, authored
domain commands, native transition validation, normalization, controller or
external adapter dispatch. Native copy and cold recovery retain the selected
conflict objects, their complete dependency closure and accepted-origin
receipts.
