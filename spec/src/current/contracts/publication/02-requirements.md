# Requirements

| ID          | Requirement                                                                                                                                                                              |
| ----------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| JST-PUB-001 | Only one manifest CAS MUST publish a head transition together with its exact WAL candidate, complete dependency inventory, and durable mutation receipt.                                 |
| JST-PUB-002 | Candidates MUST use attempt-unique immutable keys; readers and adapters MUST follow selected manifest/catalog references and never infer commitment from sequence filenames or LIST.     |
| JST-PUB-003 | Native head updates MUST preserve concurrent siblings; strict mutations MUST reject stale snapshots, and late duplicate publications MUST NOT resurrect obsolete heads.                  |
| JST-PUB-004 | Request identity and canonical intent digest MUST provide durable retry receipts and detect conflicting reuse across restarts and checkpoint compaction.                                 |
| JST-PUB-005 | Unknown outcomes MUST resolve through committed receipt/history lookup or return OUTCOME_UNKNOWN under the original request ID; absence alone MUST NOT prove rejection.                  |
| JST-PUB-006 | Checkpoints MUST preserve newer tails, retained history, evolution dependencies, and receipt lookup; empty server caches MUST recover acknowledged authority from object storage.        |
| JST-PUB-007 | One manifest CAS MUST select code, AgentStore and ReviewStore roots with independent monotonic state versions; publishing one domain MUST preserve the other two domains.                |
| JST-PUB-008 | Collaboration enablement MUST install canonical empty roots through an authorized receipt-bearing WAL publication without rewriting native history or staling unchanged code state.      |
| JST-PUB-009 | Review commands MUST authorize and replay matching receipts before entity preconditions, then reapply typed per-thread or per-comment intent to the current review root after CAS races. |
