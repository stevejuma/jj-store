# Read APIs and browser

Use tonic/protobuf for native RPC and Axum JSON/SSE for browser/agent APIs.
Generate OpenAPI and client types. Full IDs are authoritative; shortened IDs are
presentation. Historical loading requires no physical server working copy. Graph
rendering does not decide ancestry. The route inventory in the archived
attachment section 10 is implemented under /v1/repos/{repo}.

## Requirements

| ID          | Requirement                                                                                                                                                                         |
| ----------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| JST-API-001 | Read APIs MUST expose versioned DTOs for capabilities, snapshots, native objects, commit/operation/evolution graphs, trees, files, diffs, and conflicts without parsing CLI output. |
| JST-API-002 | Pagination MUST bind an explicit operation, query, repository, and epoch; ambiguous heads and workspace-dependent expressions MUST require explicit resolution.                     |
| JST-API-003 | SSE MUST resume from committed positions and detect writes from other serving processes; slow consumers MUST NOT block repository publication.                                      |
| JST-API-004 | Queries MUST enforce authorization and resource budgets, safely render untrusted source content, and expose incomplete history or rebuild tasks explicitly.                         |
| JST-API-005 | The Svelte UI MUST show repository selection, snapshot state, three graph views, object/diff inspection, conflict editing, mutation previews/results, and search freshness.         |
