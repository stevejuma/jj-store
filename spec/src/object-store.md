# Object storage

Version tokens are opaque equality tokens. A timeout is unknown, never proof of
failure. Test concurrent create and CAS, stale tokens, independent endpoints,
restarts, lost responses, range requests, and partial uploads. The initial
supported topology has one writer per repository. Failed CAS conformance blocks
rollout; GET followed by unconditional PUT is not a substitute.

## Requirements

| ID          | Requirement                                                                                                                                                         |
| ----------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| JST-OBJ-001 | Object storage MUST provide get, head, immutable create, conditional compare-exchange, range reads, and explicit Applied, PreconditionFailed, and Unknown outcomes. |
| JST-OBJ-002 | Acknowledged writes MUST satisfy tested cross-route read-after-write and conditional-write semantics for the exact pinned RustFS image and topology.                |
| JST-OBJ-003 | Immutable uploads MUST enforce size and checksum limits and verify existing native content on collisions; large blobs MUST stream with bounded memory.              |
| JST-OBJ-004 | Normal graph reads MUST NOT rely on bucket LIST; destructive delete MUST remain disabled for core v1.                                                               |
