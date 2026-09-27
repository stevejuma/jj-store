# Publication and recovery

Upload objects, validate full dependency closure against a certified base,
upload inventory/result/candidate, then CAS the manifest. No receipt embeds its
future log digest. Manifest revision differs from publication sequence, and an
epoch identifies an authority lineage without itself fencing an old writer. No
destructive GC, automated promotion, or multi-writer serving in v1.

## Requirements

| ID          | Requirement                                                                                                                                                                          |
| ----------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| JST-PUB-001 | Only one manifest CAS MUST publish a head transition together with its exact WAL candidate, complete dependency inventory, and durable mutation receipt.                             |
| JST-PUB-002 | Candidates MUST use attempt-unique immutable keys; readers and adapters MUST follow selected manifest/catalog references and never infer commitment from sequence filenames or LIST. |
| JST-PUB-003 | Native head updates MUST preserve concurrent siblings; strict mutations MUST reject stale snapshots, and late duplicate publications MUST NOT resurrect obsolete heads.              |
| JST-PUB-004 | Request identity and canonical intent digest MUST provide durable retry receipts and detect conflicting reuse across restarts and checkpoint compaction.                             |
| JST-PUB-005 | Unknown outcomes MUST resolve through committed receipt/history lookup or return OUTCOME_UNKNOWN under the original request ID; absence alone MUST NOT prove rejection.              |
| JST-PUB-006 | Checkpoints MUST preserve newer tails, retained history, evolution dependencies, and receipt lookup; empty server caches MUST recover acknowledged authority from object storage.    |

## Verified dependency inventories

Inventory construction follows graph edges from requested roots, never a list of
objects uploaded by one process. The native source verifies bytes and identities
before returning dependencies. Operations include parents, their view and every
key/value of commit_predecessors; commits include parents, old predecessors and
all root-tree merge terms; views include all reference terms and workspace
commit mappings. Trees include files, symlinks, subtrees and real copy IDs; copy
histories include ordered parent references. Submodule links and copy
placeholders are not repository-object dependencies. Virtual roots and the
pinned empty tree are represented explicitly and require no persisted blob.

The inventory walker rejects missing/corrupt objects, cycles and mismatched
keys. It enforces limits on total nodes, edges, newly loaded bytes and elapsed
time. A previously verified inventory can supply an incremental base; new
inventories retain its entries and add the verified closure, so evolution-only
objects cannot vanish when visible heads change. Only a later manifest selection
establishes committed authority; a verified inventory by itself does not
establish a commit.

In-memory verified bases are bound to a process-local source capability and
repository identity. A different accessor, replica or restarted process must
recover certification from selected persistent records; it cannot reuse an
arbitrary in-memory base to skip required reads. The source receives the
remaining byte budget before starting a new object read.

## V1 control records and catalogs

Control records are canonical, versioned JSON envelopes containing repository
identity and record kind. SHA-256 references bind their exact bytes. WAL paths
include a cryptographically random attempt ID as well as the record digest.
Catalogs use immutable bounded radix pages (32 entries per leaf, 16 children per
branch, at most 64 nibbles of depth). Separate selected roots index retained
object descriptors, receipts and committed sequence positions. Reads follow
those roots without bucket enumeration; incremental updates copy only changed
paths. Old roots and records remain readable and are never deleted.

A bounded manifest selects the catalog roots, exact tail, nonempty operation
heads and immutable settings together. Strict requests bind the logical snapshot
(epoch, published sequence and heads), independent of checkpoint-only manifest
revisions. Canonical intent covers the complete publication request; request and
publication identity indexes share a durable result receipt. Duplicate lookup
precedes stale-snapshot checks and head changes.

A checkpoint stores the folded state and selected catalog roots at its captured
position. Selecting it changes only manifest representation/revision and keeps
any newer publication tail, heads, settings and catalog roots from the
successful CAS base. It cannot prune receipts, evolution history or committed
log records.

The coordinator requires a publication guard with separate authorization and
semantic-validation methods; no production allow-all implementation exists. It
authorizes before receipt lookup and dependency reads, validates new transitions
before staging, and repeats both checks against a fresh base after CAS conflict.
Receipt replay repeats authorization but never reruns the original mutation.
Transport/service integration supplies the shared policy in M3/M5.

Cold integrity recovery verifies retained historical inventory and settings
records as well as the current roots. Checkpoint catalogs and predecessor
checkpoints must remain readable and agree with the selected history; corruption
in an old root is not hidden by a newer root. A shared bounded page audit avoids
reading unchanged radix subtrees repeatedly. Ordinary startup may read the
manifest's already folded state; the explicit integrity recovery performs a full
retained-data audit and fails if its configured budget is insufficient.

Selected settings are readable from a checked ManifestSnapshot using an owned
storage accessor, so request-scoped guards can verify policy without
constructing a second publication coordinator or using mutable global
authentication state.

Cancellation conformance synchronizes with the selected storage fault boundary
before advancing a controlled test clock. A pre-staging deadline must not be
misreported as evidence for a post-CAS lost acknowledgement.

Checked manifest snapshots expose the same bounded committed-inventory reader
used by the coordinator. Native semantic validation and authenticated
prefix/copy enumeration use this selected inventory; the reader does not publish
state or introduce a second commitment test.

Historical settings lookup binds epoch, publication position and exact heads to
manifest-selected history. Position zero selects the immutable initial policy;
later positions select their committed WAL record. Callers authorize against
current policy before this bounded lookup. It cannot load an orphan candidate or
accept caller-supplied record references, and remains valid after checkpointing.
