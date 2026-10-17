# Filesystem local repository API

LocalTrusted Admin installations with native authority advertise
`local_native_repository_creation`. POST `/v1/local/repositories/native` accepts
only V1 request identity and discovery name; GET
`/v1/local/repositories/native/requests/{request_id}` resolves that exact
installation-owned request. The server durably retains repository and epoch
identities before name reservation and manifest publication, then registers the
verified repository. Replays cannot allocate another repository, changed intent
fails conflict, and unknown outcomes retain the original request. Creation does
not require a physical workspace. Filesystem-only and authenticated deployments
do not advertise or admit this capability.

LocalTrusted administrative installation endpoints under
`/v1/local/repositories` inspect, register, remove and recover device-local
Git-backed JJ bindings. The OS picker is available before repository selection.
Registration retains its request identity through companion initialization and
discovery persistence. `/v1/local/repos/{repo}/...` serves profile-aware code
snapshots, bounded code queries and existing local
WorkStore/AgentStore/ReviewStore semantics. Native repository endpoints and
native identities retain their contracts. Discovery includes profile and
availability without filesystem paths. All local routes reuse the existing
origin, protocol, media-type and literal-loopback admission checks.

Local code reads use a strictly typed, non-publishing POST
`/v1/local/repos/{repo}/query` with explicit operation and owned query variant.
Snapshot, capabilities, registration inspection/list/add/remove/status and
installation folder picker endpoints use the same admission. Continuations bind
repository, generation, operation and exact request, expire after five minutes
and reset across process restart. Native ID schemas are unchanged.

Local WorkStore POST `/work/page` and `/work/browse` under the local repository
namespace require `metadata_commit` and a strict owned query. Their signed
continuations bind that committed selection and complete filters. POST
`/work/commands` and GET `/work/commands/{request_id}` reuse CLI device actor
provenance, original request receipts and unknown outcomes. They do not require
or fabricate a native code snapshot.

Local ReviewStore POST `/reviews/page` and `/reviews/detail` require an explicit
committed `metadata_commit`. List continuations bind the repository, binding
generation, exact metadata selection and page size and expire after five
minutes. They cannot select staged or foreign metadata. Detail uses the same
bounded comment/event reconstruction and integrity checks as native serving,
returning `LocalReviewSnapshot` without fabricated epoch or WAL fields. POST
`/reviews/commands` accepts the existing typed review command body; GET
`/reviews/commands/{request_id}` returns the original local receipt. Both use
the CLI device actor. Versioned capabilities advertise review reads, actions,
repository anchors, code anchors and agent anchors independently. Repository
anchors are qualified; code and agent anchors remain false. Local repository
capabilities do not contain a synchronization field.

Local repository AgentStore POST endpoints `/agents/sessions`,
`/agents/session`, `/agents/transcript` and `/agents/relations` require an
independent committed `metadata_commit`. Signed continuations bind the complete
query, repository and binding generation. GET `/events` announces bounded local
head/ref freshness with local-profile event DTOs. The browser automatically
recaptures active collaboration/current-head views while retaining historical
code pins. GET remains read-only.

POST `/v1/local/repositories/{repo}/reconnect` requires the current device
registry revision and an inspected folder path with the original companion
identity. It replaces only the device binding, increments its generation and
rejects a conflicting ready copy. It does not initialize replacement metadata.

The application dispatches Git-backed repositories only through the LocalTrusted
local namespace. Native ID validation, endpoints and event parser retain their
original contracts. Shared screen models use explicit native/local snapshot
unions; local data cannot fabricate native epoch, WAL positions or graph cache
reports.

The General settings removal confirmation uses the existing V1 remove endpoint
with its captured device registry revision. A retry retains that exact revision;
observing an absent binding resolves removal without another write. It never
refreshes the expected revision to remove a replacement registration. This
changes browser presentation only; removal still preserves every folder and
repository object and requires LocalTrusted Admin installation admission.

Admitted local requests wait within the existing HTTP request/deadline bound for
a local worker. Parallel startup code and project reads cannot fail merely
because the bounded worker pool is briefly occupied. Cancellation before
admission performs no folder work.

Cold bounded-read integration acceptance uses the current capability-dependent
repository Add menu and folder controls alongside files, prepared history and
AgentStore reads. It preserves the source JJ operation, working files and
companion selection; menu presentation changes do not grant native authority to
a filesystem-only installation.

LocalTrusted Admin native deletion uses POST
`/v1/local/repositories/{repo}/delete` with V1 request ID and expected epoch.
Responses distinguish pending erasure from deleted. Repeating the exact POST
continues bounded erasure; GET
`/v1/local/repositories/{repo}/deletions/{request_id}` reports only the retained
outcome. Capability `local_native_repository_deletion` is installation-gated.
Deletion is unavailable to authenticated shared serving and never dispatches the
folder removal endpoint.

M32's local command and status handlers use the library store factories inside
the existing bounded blocking-worker admission. Request identities, original
receipts, route schemas, origin/protocol checks, deadline handling and folder
admission under JST-API-001/002 remain unchanged. No UI behavior changes with
this Rust composition refactor.
