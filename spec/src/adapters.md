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
