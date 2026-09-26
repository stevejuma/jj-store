# Architecture

Crates are added as milestones begin, not as empty placeholders. Native codecs
initially live with JJ integration. The server uses in-process stores rather
than calling its own RPC. Local indexes are derived and rebuildable. The source
repo is colocated for development; product storage has no Git carrier.

## Requirements

| ID           | Requirement                                                                                                                                                                     |
| ------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| JST-ARCH-001 | The repository MUST provide a Rust workspace with independently owned native storage, publication, JJ integration, services, transports, adapters, and presentation boundaries. |
| JST-ARCH-002 | Model, object storage, and WAL MUST NOT depend on UI, Tantivy, Git transport, SQL, Nostr, or agent runtimes; upstream JJ integration MUST stay behind owned adapters.           |
| JST-ARCH-003 | First release MUST serve new native repositories locally and on an authenticated shared server using the same storage format, with no required Git, SQL, or relay authority.    |
