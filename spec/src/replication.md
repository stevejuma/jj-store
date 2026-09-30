# Local-first writable replication

Every writable checkout uses a complete local `jj-store` and local object
authority. Ordinary native JJ, AgentStore and ReviewStore reads and writes end
at that local authority. A bounded background worker exchanges immutable objects
and accepted publications with one configured authenticated upstream. Remote
health changes replication status; it does not enter the foreground command
path.

Writable replication is semantic import between independently published
manifests. It is distinct from passive replica copy and verified backup. An
import preserves the destination manifest, local publication positions and
workspace ownership while incorporating previously accepted origin records. It
never installs another writer's manifest as local authority.

## Requirements

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

Normal local commands never wait for the worker. A receiver acknowledges only
after its own manifest CAS is durable. Lost responses are resolved by querying
or replaying the same origin keys. A feed horizon gap requires a compatible
checkpoint plus tail; a replica with pending local work merges that knowledge
through normal import rather than replacing its roots.

## Delivery sequence

M10 implements this chapter after the completed LocalTrusted M9 foundation:
audit and foreground isolation; origin authority; one-way verified transfer;
bidirectional native convergence; AgentStore and ReviewStore reducers; complete
bootstrap and migration; then status, fairness, cold recovery and real RustFS
qualification. Bidirectional mode stays disabled for repositories containing
collaboration data until the domain reducers and migration gates pass.
