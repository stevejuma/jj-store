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
