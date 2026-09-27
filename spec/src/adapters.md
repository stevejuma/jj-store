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

## Local Tantivy generations

The Tantivy projection stores deterministic operation, commit and scoped-file
keys. Each update copies a bounded committed local generation into a private
staging directory, applies deterministic delete/upsert terms, then prepares a
Tantivy commit whose payload binds adapter identity, exact source position,
source-scope snapshots and the previous artifact. Tantivy opstamps are not
source positions. Files and the complete file descriptor are synced before an
atomic local active-pointer replacement; the remote runner cursor follows
recoverable local state. Crash orphans may be discarded and replayed.

Recovery verifies descriptor checksums, commit payload, schema and actual index
readability. Missing or corrupt derived files request rebuild. A reader-load
failure after pointer replacement leaves the durable checkpoint ahead of the
served generation and reports degradation; recovery must retry loading it.
Published generations are immutable and held by reader references. Retired files
are removed only after their last reader exits. Configuration changes build a
new generation without changing existing readers. Disk-copy, document, file,
query and result budgets are explicit; this first implementation trades bounded
local copying for simple crash-safe reader isolation.

Core search pins an immutable generation and rechecks repository authorization
and epoch before and after every request, including pagination. It exposes no
unauthorized counts or stored content. HTTP cursors and serving configuration
require additional integration acceptance; the storage engine alone does not
close the search milestone.

Native extraction can read bounded selected history and earlier complete
inventories through the delivered snapshot. Both stop at that snapshot's
position even when the captured manifest contains later publications. Genesis
uses only the verified immutable root-operation dependency closure.
Authenticated actors come from selected receipts, distinct from native user/host
metadata.

## Native search extraction

Native extraction indexes complete retained operation/commit metadata, including
invisible predecessor commits. Selected receipts supply authenticated actors;
native user/host fields remain separate. A commit first introduced by a
publication with exactly one new operation records that operation as its known
publication origin; ambiguous origins remain absent. Bootstrap and incremental
replay derive this association from the same selected inventories.

Source indexing requires explicit named bookmark/workspace scopes. Multiple
operation heads, conflicted bookmarks, missing scopes and disabled source
indexing are reported without selecting an arbitrary target. Canonical config
hashes include include/exclude path prefixes and resource limits. Binary,
non-UTF-8, oversized, symlink, submodule and conflicted file values are
excluded; ordinary UTF-8 files use exact native file IDs, modes and copy
metadata.

A native tree diff supplies deleted/changed paths. Unchanged file terms stay in
the index while the commit payload advances their exact scope commit/operation.
Hit metadata and exact-ID scope queries use that pinned payload context. Local
schema version 2 removes stale per-file commit/operation ID terms; earlier local
indexes rebuild. This changes no authoritative native or adapter cursor format.
The owned content bridge retains current authorization and the adapter context
through native workers. Failed extraction cannot advance the index or cursor.

## Selected-view commit search

Every core search request explicitly chooses historical metadata or commit
visibility at an authorized native read snapshot. Visible commits are the
complete parent closure of that view's heads, including the virtual root but
excluding predecessor-only history. The native reader applies history, work,
metadata, output and deadline budgets; it returns a complete set or an explicit
error. No global abandoned flag substitutes for selected-view semantics.

The search engine intersects that set with commit hits before ranking and
pagination. Operation metadata remains historical; source hits retain their
explicit indexed scope snapshots. Results identify the selected native view and
the independently pinned index generation. Later repository or index updates
cannot silently change either one. Lagging generations can omit new results but
cannot promote hidden commits into visible ones. Both modes still require fresh
repository authorization and matching epochs; visibility data grants no access.
