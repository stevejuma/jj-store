# Adapters and Tantivy

Operation/commit metadata may span retained history; file indexing uses explicit
configured bookmark/workspace scopes. Conflicted scopes are not assigned an
arbitrary winner. Source visibility is checked against the selected JJ view.
Tantivy opstamps are not WAL positions. The active index is never a RustFS
mount. Nostr, Git export, and passive replication remain independent follow-on
adapters.

## Requirements

| ID            | Requirement                                                                                                                                                                                                                                                                                                                 |
| ------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| JST-ADAPT-001 | Adapters MUST consume only selected committed records in repository order, with at-least-once replay and checkpoints identifying repo, epoch, segment, schema/config, and durable artifact.                                                                                                                                 |
| JST-ADAPT-002 | Adapter failure MUST NOT fail repository publication or skip poison records; cursor advancement MUST follow durable destination effects.                                                                                                                                                                                    |
| JST-ADAPT-003 | Tantivy MUST remain a local rebuildable projection with deterministic keys, commit payloads, one writer owner, explicit source scopes, and separately reported indexed positions.                                                                                                                                           |
| JST-ADAPT-004 | Lost index files MUST trigger compatible restore or rebuild even with an advanced remote cursor; search pagination MUST pin a reader generation and apply fresh authorization.                                                                                                                                              |
| JST-ADAPT-005 | Passive replicas MUST publish applied manifests only after complete dependency transfer and MUST remain read-only until separately verified fenced promotion.                                                                                                                                                               |
| JST-ADAPT-006 | Backup adapters MUST copy a pinned WAL recovery closure without blocking publication, verify streamed destination bytes and exact object versions, bind checkpoints to source and destination identity, publish immutable inventories/certificates, and advance a latest-verified pointer only after complete verification. |

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
unauthorized counts or stored content. HTTP cursors, serving configuration, cold
process recovery and the pinned RustFS deployment are qualified separately from
the storage-engine fixtures.

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

## Search serving sessions

Search continuations use process-local HMAC signatures and a bounded registry of
credential-free index/view pins. The signature binds repository, epoch, query,
scope, mode, page size, native view, source position and exact index artifact.
Changed inputs or signatures fail; expiry and server restart require an explicit
cursor reset. Expiry maintenance releases idle pins so they cannot indefinitely
hold retired local generations. Retained view nodes and request concurrency have
separate budgets. Every page checks current read permission and epoch.

Results report current repository position, independently pinned indexed
position/generation, scope snapshots, native view, staleness and degradation.
Optional minimum-position requests wait only for a configured bounded duration;
failure to catch up returns INDEX_LAG with authorized freshness metadata. A
continuation never silently upgrades its generation. Source-scope filters select
only that configured scope's files. Historical metadata search is explicit;
visible-commit mode requires operation selection when heads are ambiguous.
Pagination reaching its offset budget reports truncation without an unusable
continuation. Search remains unable to publish repository history.

HTTP composition accepts only explicitly configured per-repository search
handles and runner health receivers. It constructs pagers with the shared native
content worker pool and caps search output to the HTTP response budget. Route
availability does not imply an index is ready; status distinguishes durable and
served positions, and a missing reader returns search_not_ready. Server runner
configuration and remote snapshot restoration are covered by the M6 deployment
acceptance.

## Configuration identity before recovery

The runner validates a projection's accepted identity before acquiring writer
ownership or reading its remote cursor. Native Tantivy extraction binds that
identity to its complete canonical configuration. A caller cannot label one
extractor with another configuration digest, implementation or schema.

Recovery checks that binding before exposing a generation. An incompatible local
artifact is a rebuild candidate and remains unavailable to new readers,
including when the replacement build fails. Rebuilding at the same native
position produces an artifact for the new configuration. Existing compatible
pins retain their immutable generation; native repository data and IDs do not
change when source projection exclusions change.

Server composition creates one runner per explicitly configured repository and
uses the canonical native extraction identity. Active index paths are derived
from full repository IDs beneath private server state; users cannot supply paths
from repository content. Configuring no source scopes indexes metadata only.
Authenticated runners hold their repository token only in memory and reauthorize
on every batch. LocalTrusted runners use a stable installation-derived read-only
principal and share the same repository, snapshot and resource checks.
Shared-mode configuration reload requires an explicit server restart.
LocalTrusted Admin Search settings additionally support the isolated Apply
operation described below. Local cache loss rebuilds from selected history
regardless of an advanced remote adapter cursor.

A graceful runner shutdown waits until retained callback contexts release native
and blocking work before releasing writer ownership. A forced task drop still
cancels workers; their context clones retain ownership until actual exit.
Rebuild equivalence compares document contents and native source positions;
equal-score ordering may differ in a newly built index. Pages within one pinned
generation retain their exact ordering.

A closed runner health channel marks search degraded even when its last index
remains readable. A terminated worker cannot leave a permanently healthy status.

## Optional immutable snapshot storage

The adapter boundary owns an authorization-bound snapshot capability constructed
from a selected source snapshot. It exposes streamed immutable blobs and a
versioned bundle of canonical file names, byte lengths and SHA-256 checksums.
Keys are derived from repository, authority epoch and the complete adapter
identity; projections receive no arbitrary object-store path or mutation API.
Every transfer consumes the retained run context's request, byte and deadline
budgets and checks current repository authorization before and after I/O.

Snapshot publication verifies every referenced file, uploads the immutable
bundle descriptor, and conditionally selects the descriptor last. Selection
binds the exact committed source reference, configuration and projection
artifact. A lost pointer acknowledgement is resolved by checked readback; a
competing or later pointer cannot be silently overwritten. Snapshots never
update the repository manifest or the adapter replay cursor. File names reject
path traversal; bounded descriptors reject unknown fields and noncanonical
encoding. Missing, corrupt or incompatible snapshots are unusable acceleration,
not evidence that local index files have survived. Tantivy restore must still
verify its payload, schema and actual index readability before activation.

The foreground runner restores a compatible selected snapshot before native
bootstrap. Tantivy downloads every declared generation file into private
staging, verifies file lengths and checksums, canonical descriptor bytes,
adapter identity, schema, source position, commit payload, document limit and
actual index readability, then atomically installs the original generation and
active pointer. An older compatible generation catches up through ordinary
selected batches. Missing or unusable snapshots rebuild from native history and
repair the remote pointer; repository publication remains independent.

`snapshot_every` schedules optional exports by committed source position; zero
disables them. Export pins the active committed generation and streams only its
declared files through the selected snapshot capability. Optional snapshot I/O
failure reports degradation and is retried without failing repository writes;
revocation and cancellation remain fatal to the worker. Transfer budgets derive
from the configured per-generation index budget.

The pinned single-node RustFS/TLS deployment deletes all local server and search
state, restarts both RustFS and the server, and requires the exact generation
digest and search results to return. Its recorded cold rebuild was 5.436 seconds
and exact restore was 2.007 seconds, both within the published 30-second test
budget. This measured fixture proves the restore path avoided a rebuild for its
bounded repository; it is not a general throughput claim.

Local Search Apply drains only the selected repository worker. Replacement
projections are built and verified in owner-held temporary roots before the
serving backend changes. Failed replacements resume the previous worker and
retain its effective configuration. Search cursors keep their original readers
across apply or disable; new queries use the active backend. Temporary roots
live until the last projection or reader owner releases them. Replacement
concurrency is bounded and uses the existing runner recovery path. Retry
restarts the effective runner; Rebuild bypasses cached adapter snapshots and
reconstructs selected native data. Startup reopens the saved desired
configuration.
