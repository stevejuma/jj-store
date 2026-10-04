# Local-first writable replication

Every writable checkout uses a complete local `jj-store` and local object
authority. Ordinary native JJ, AgentStore and ReviewStore reads and writes end
at that local authority. A bounded background worker exchanges immutable objects
and accepted publications with one configured authenticated upstream. Remote
health changes replication status; it does not enter the foreground command
path.

## Initial activation profile

The first useful production profile is one explicitly configured upstream per
local repository. The server owns the worker lifecycle and starts, wakes and
stops it with the serving process. Peer progress is durable local authority; the
upstream credential is resolved from the named environment variable into process
memory and is never written to configuration, status, evidence or a checkout.

This profile includes bounded bidirectional transfer, independent local and peer
frontiers, local-only status reads, `now`, `pause`, `resume`, finite frontier
waits, exact replica inspection and explicit native reconciliation. An upstream
outage degrades replication while local reads and mutations remain available. An
accepted import emits the normal committed-change freshness hint and advances
the rebuildable search adapter from its destination WAL position.

The activation gate uses focused configuration, server lifecycle, outage,
authenticated RPC, worker, SSE and search-projection acceptance. Exhaustive
pressure and fault soak, expanded memory and disk matrices, a browser
replication interface, cold two-instance real-RustFS replication, replication
pressure archive/export tooling and additional deployment topology qualification
are deferred hardening work. Those deferrals do not expand the foreground
command path or weaken manifest-CAS publication, receiver verification, identity
preservation, durable receipts or checkpoint rules.

Writable replication is semantic import between independently published
manifests. It is distinct from passive replica copy and verified backup. An
import preserves the destination manifest, local publication positions and
workspace ownership while incorporating previously accepted origin records. It
never installs another writer's manifest as local authority.

## Requirements

LocalTrusted Admin Settings may conditionally persist explicit replication
upstreams, credential environment names, CA paths and all validated worker
budgets. These installation configuration changes apply only at the next server
startup. Saving never contacts an upstream, reads a credential value, hydrates a
repository, alters peer checkpoints or lifts the WorkStore origin freeze.

| ID           | Requirement                                                                                                                                                                                                                                                                                               |
| ------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| JST-REPL-001 | After verified hydration, every ordinary code, agent and review read or mutation MUST use complete local durable storage and MUST perform no upstream network I/O; local success MUST NOT be described as remotely backed up before acknowledgement.                                                      |
| JST-REPL-002 | Native/domain object identity, logical origin identity and local delivery position MUST remain distinct; forwarding MUST preserve each original origin key and digest while every accepting replica allocates its own WAL position.                                                                       |
| JST-REPL-003 | An authored publication MUST select its immutable effects, dependency inventory, origin envelope, causal frontier and receipt in one manifest CAS; failed candidates MUST NOT consume an accepted origin sequence.                                                                                        |
| JST-REPL-004 | A receiver MUST verify repository identity, negotiated formats, origin lineage, causal predecessors and the complete strong dependency closure before atomically selecting imported code, agent, review and accepted-origin state.                                                                        |
| JST-REPL-005 | Replaying an identical origin MUST return its durable receipt without another semantic effect or content WAL event; reusing an origin key with a different digest MUST fail as an integrity error.                                                                                                        |
| JST-REPL-006 | Push and pull MUST use bounded issuer-bound feed snapshots and peer checkpoints, retain stable origin identities after lost acknowledgements, and advance a cursor only through acknowledged, already-known or non-exportable entries.                                                                    |
| JST-REPL-007 | Code import MUST contribute only explicitly published native operation heads, remove candidates proven ancestral by the pinned JJ operation graph and retain concurrent siblings; reads and imports MUST NOT reconcile heads implicitly.                                                                  |
| JST-REPL-008 | AgentStore import MUST merge stable source-owned accepted facts and parent-linked versions deterministically, retaining incomparable facts and terminal conflicts without rerunning a normalizer merely because data arrived from a peer.                                                                 |
| JST-REPL-009 | ReviewStore import MUST preserve accepted event and revision identities, concurrent comment revisions, replies unseen by workflow actions and explicit resolution coverage; clock order and destination WAL order MUST NOT discard discussion.                                                            |
| JST-REPL-010 | Bootstrap and migration MUST verify a complete retained three-domain closure before attaching a workspace, allocate a distinct replica/workspace identity, preserve unsnapshotted files and pending local origins, and catch up from a pinned source frontier without replacing an active local manifest. |
| JST-REPL-011 | The worker MUST bound metadata, object streams, retries, memory and local resource use; unacknowledged origins and their closures MUST remain pinned, while malformed or unsupported remote data MUST pause that transfer without blocking unrelated local work.                                          |
| JST-REPL-012 | CLI and application status MUST report local readiness, pending origins, peer acknowledgement, observed feed frontier, reconciliation need and local search freshness; an explicit wait MUST target a captured finite frontier.                                                                           |
| JST-REPL-013 | Import, bootstrap, replay and index rebuild MUST treat replicated content only as data and MUST NOT execute tools, controller work, hooks, messages, Git/Nostr publication or other external side effects.                                                                                                |

## Publication identities

`ReplicaId` identifies one durable installation. `WriterEpoch` changes when an
origin stream is deliberately replaced. An `OriginKey` is repository, replica,
writer epoch and monotonically accepted sequence. The canonical origin digest
covers the origin envelope without any destination WAL identity or mutable
manifest reference. A `LocalWalPosition` remains meaningful only with its
issuing replica and log epoch.

Origin envelopes name their predecessor digest, causal frontier, semantic
format, stable request/intent identity, published operation heads, accepted
agent/review facts, complete inventory root and changed domains. A combined
origin remains one atomic visibility unit at every receiver.

## Convergence and recovery

The hub is another writable replica and uses the same receiver. A pure import
does not mint an origin, so forwarding through several peers converges by the
accepted-origin catalog instead of producing echo publications. Equal accepted
origin sets and compatible reducers produce equivalent domain state while local
manifests, delivery order, search generations and unresolved conflicts may
differ.

Initial hydration and existing attachment migration are explicitly online.
Format-2 repositories without origins receive one source-authored seed
checkpoint that inventories their accepted code, agent and review state without
inventing historical causality. A replica restored from old storage uses a new
stream identity unless continuity is proved and fenced.

The seed is the first origin of a newly allocated source stream and is selected
by the same manifest CAS as its WAL transaction, receipt, accepted-origin
catalog and causal prefix. It names every selected native operation head and the
complete current AgentStore and ReviewStore roots. Receivers recognize only this
exact four-domain first-origin shape as a baseline, verify its complete closure,
and merge every selected catalog fact with any local state. Empty domain
catalogs remain an explicit empty baseline. Retrying the same seed request
returns its durable receipt; a repository that already has a different
replication root cannot be reseeded in place.

Normal local commands never wait for the worker. A receiver acknowledges only
after its own manifest CAS is durable. Lost responses are resolved by querying
or replaying the same origin keys. A feed horizon gap requires a compatible
checkpoint plus tail; a replica with pending local work merges that knowledge
through normal import rather than replacing its roots.

Negotiation reports the oldest readable feed cursor and, when older positions
have been compacted, a compatible complete seed checkpoint. The checkpoint is
bound to the source replica, log epoch, source manifest digest, seed origin and
captured WAL position. Bootstrap transfers and applies the seed through the
ordinary receiver, advances peer progress only after the destination receipt is
durable, and then scans the tail to the finite upper bound captured by the same
negotiation. A mismatched descriptor, missing three-root closure, cursor newer
than the source upper bound or unavailable compatible checkpoint fails closed.
The destination manifest, replica stream and workspace metadata are never copied
from the source.

An existing physical workspace migrates through a durable, secret-free client
journal outside the checkout. The journal binds the original cache and remote
connection, the destination LocalTrusted connection and cache, the adopted
workspace identity, and stable local-repository creation identity. Migration
creates or verifies the independent local repository, pulls a complete seed and
tail through the ordinary receiver, initializes a new client cache, and verifies
that the existing working-copy operation loads from local storage. Only then may
it preserve the exact old `.jj/repo` descriptor as a recoverable backup and
atomically replace that descriptor with the local cache path. It MUST NOT run a
checkout, snapshot, working-copy update, reconciliation, hook, or external
adapter during this handoff. Retries reuse the journal and accepted origins;
unsnapshotted files and `.jj/working_copy` bytes remain untouched.

## Verified receiver boundary

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

## Delivery sequence

M10 implements this chapter after the completed LocalTrusted M9 foundation:
audit and foreground isolation; origin authority; one-way verified transfer;
bidirectional native convergence; AgentStore and ReviewStore reducers; complete
bootstrap and migration; then production lifecycle controls, status, fairness,
SSE and search integration. Bidirectional mode stays disabled for repositories
containing collaboration data until the domain reducers and migration gates
pass. Expanded pressure, browser and cold real-RustFS qualification follows as a
separately deferred hardening milestone.

## Native WorkStore

WorkStore enablement preserves and freezes an existing origin ledger. Native
repository creation initializes that ledger independently of an upstream, so its
presence cannot reject local task creation. Format-3 publication omits new M10
origins; protocol negotiation, feed, offer, closure, import and seed routes
reject the unsupported profile explicitly. The local WAL and native copy retain
complete WorkStore authority. Existing configured replication becomes
unavailable for a WorkStore home; WorkStore does not claim remote convergence.

## Native Git facts

Reducer 2 accepts semantic-format-2 Git origins as immutable catalog unions.
Optional Git fields are omitted on legacy envelopes so their digests remain
unchanged. The receiver verifies the complete native closure, raw Git object
bytes, mappings, causal facts and frozen push source before manifest
publication. Combined code/Git acceptance and seed hydration retain one atomic
visibility boundary. Native operation ancestry belongs to the JJ integration;
unsupported validators fail closed. Peer arrival never grants endpoint authority
or executes Git, credentials, helpers or hooks. WorkStore's origin freeze still
applies.

The local browser Sync management adapter reuses the existing status, wake and
set-paused control plane. It preserves distinct local/peer origins, captured
frontiers, pending counts and faults; no changes to reconciliation, authority,
WorkStore support or worker semantics are introduced. Pause/Resume is idempotent
process-lifetime control. Browser finite-frontier waits use explicit barrier
capture and bounded continuations. A timeout retains the original target;
stopping the browser wait does not pause replication.

Local Search replacement updates the configured sync report to the current
worker health receiver; disabling Search reports an unconfigured projection.
Replication progress and transfer ownership remain unchanged.

## M15 collaboration format boundary

Thread/Reaction format-5 repositories require a qualified semantic reducer 3
before seeding, importing or authoring new collaboration origins. The initial
persistence gate rejects unsupported legacy replication rather than dropping
roots or claiming convergent support. WorkStore native homes retain their
existing independent origin freeze. Local Thread/Reaction writes in WorkStore
homes preserve that frozen ledger. Native copy is complete-closure transfer, not
evidence of reducer convergence or real two-instance qualification.
