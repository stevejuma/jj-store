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

## Read-session foundation

Application read sessions capture the checked manifest's repository, epoch,
publication position, revision and native head set. An omitted operation selects
only a sole head; concurrent heads return an explicit selection response without
reconciliation. An explicit historical operation must belong to the captured
committed inventory. Staged candidates and objects published after selection
cannot enter that session's membership set.

The owned read capability has no publication methods and cannot be reconstructed
from its informational DTO. Inventory count and elapsed time are capped by
trusted server limits. Each later access rechecks current permission and epoch
while retaining the original snapshot. Epoch changes require reselection; later
publications in the same epoch do not silently replace the selected view.
Graph/index budgets, workspace-dependent revsets, signed continuation cursors,
HTTP routes and browser integration are separate M4 slices.

Elapsed-time guards check before and after polling an operation as well as
applying asynchronous timeouts. An immediately ready in-memory or cached result
cannot bypass an expired request deadline.
