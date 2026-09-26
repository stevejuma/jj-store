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
