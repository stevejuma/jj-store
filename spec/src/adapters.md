# Adapters and Tantivy

Operation/commit metadata may span retained history; file indexing uses explicit
configured bookmark/workspace scopes. Conflicted scopes are not assigned an
arbitrary winner. Source visibility is checked against the selected JJ view.
Tantivy opstamps are not WAL positions. The active index is never a RustFS
mount. Nostr, Git export, and passive replication remain independent follow-on
adapters.

## Requirements

| ID            | Requirement                                                                                                                                                                                 |
| ------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| JST-ADAPT-001 | Adapters MUST consume only selected committed records in repository order, with at-least-once replay and checkpoints identifying repo, epoch, segment, schema/config, and durable artifact. |
| JST-ADAPT-002 | Adapter failure MUST NOT fail repository publication or skip poison records; cursor advancement MUST follow durable destination effects.                                                    |
| JST-ADAPT-003 | Tantivy MUST remain a local rebuildable projection with deterministic keys, commit payloads, one writer owner, explicit source scopes, and separately reported indexed positions.           |
| JST-ADAPT-004 | Lost index files MUST trigger compatible restore or rebuild even with an advanced remote cursor; search pagination MUST pin a reader generation and apply fresh authorization.              |
| JST-ADAPT-005 | Passive replicas MUST publish applied manifests only after complete dependency transfer and MUST remain read-only until separately verified fenced promotion.                               |

## Delivery and checkpoint contract

An adapter cursor identifies repository, authority epoch, committed position and
the exact selected WAL reference (including attempt and digest), plus adapter
name, implementation version, schema version and configuration digest. Position
zero has no WAL reference. A checked manifest and selected catalog establish
commitment; bucket listings, uploaded candidates and numeric positions alone do
not. Bootstrap captures complete native inventory at a consistent selected
snapshot; tail delivery uses each selected record's own inventory and heads.

A durable checkpoint also identifies the local projection artifact. The runner
checks recovered local state against selected history and never treats a remote
cursor as evidence that local files exist. Missing/incompatible artifacts
require rebuild or a compatible restore. Checkpoint CAS follows durable adapter
effects; unknown writes are reconciled by readback, and failures do not skip a
record or fail repository publication. Replay may repeat durable effects. One
local owner holds the repository/adapter writer lock; remote cursor CAS does not
claim distributed fencing or exactly-once external effects.

Before bootstrap, the runner verifies the selected current tail and receipt as
well as complete inventory. A corrupt/missing tail cannot be bypassed by
starting a new projection. Successful checkpoint persistence includes checked
readback; a competing pointer is explicit contention, never an implicit
ownership grant.

Projection callbacks receive a run context carrying deadline, cancellation and
the local writer lease. Background workers retain a context clone until exit, so
timeout/cancellation cannot release ownership while destination work still runs.
Request/byte, page, inventory and per-object limits apply to adapter I/O;
service authorization retains its own session limits. Polling retries the same
failed batch with bounded backoff and reports durable position, remote cursor,
lag and degraded status through a bounded watch channel. No wake callback runs
on the repository publication path.
