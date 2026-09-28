# Committed event subscription contract

The service and HTTP transport expose notification-only SSE. Event positions
refer to selected WAL publications, not native operation ancestry or staged
objects. See E-M4-015 in the implementation ledger for actual focused and
deployment validation.

## Wire contract

`GET /v1/repos/{repo}/events` requires a current repository Bearer token. It
accepts one `after` query value or one `Last-Event-ID` header. If both are
present they must agree. Unknown query fields and malformed/foreign-repository
cursors fail before streaming. Origins use the existing same-origin policy.

An event ID is `v1:<repository hex>:<epoch hex>:<decimal publication position>`.
The encoding is canonical and bounded to 128 bytes. It is public replay state,
not a signed authorization capability. JSON data contains `version: 1`, `cursor`
with repository/epoch/position, and a `type` discriminator:

| SSE event          | JSON type | Meaning                                                                                                                                                              |
| ------------------ | --------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| ready              | ready     | Current head snapshot plus the starting cursor. With after, the ID remains that requested position while backlog replays. Without after, start at current authority. |
| repository-changed | changed   | One selected committed publication after the last delivered position. Fetch a coherent new snapshot to refresh views.                                                |
| reset              | reset     | Bootstrap required. Includes the current snapshot and reason: epoch_changed, future_position or replay_window. The stream then ends.                                 |
| comment only       | none      | Authorized keep-alive; it never advances Last-Event-ID.                                                                                                              |

An invalid/corrupt selected record or unavailable storage terminates the stream,
without skipping records or treating corruption as a reset. Revocation also
terminates delivery. Network errors after headers cannot be converted into a new
HTTP status; reconnect with the last fully parsed ID and a current credential.
Browser code must use authenticated fetch streaming and keep its token in
memory.

## Authority and lifecycle

Replay uses the existing coordinator's checked committed reader, following
manifest-selected catalogs and exact candidate/receipt references. Receipt
replay and checkpoint-only revisions produce no new event position. Epoch/window
checks and current authorization precede every queued record and heartbeat. At
most one bounded page of positions is queued; it contains no native source or
credentials.

Successful native or token-policy publication sends a non-blocking hint through
a fixed 64-entry process-local broadcast channel shared by hosted sessions.
Subscribers use hints only to wake a manifest refresh. A lagged channel triggers
the same refresh. Periodic polling observes independent publishers, and stable
resume IDs work after server restart or cache loss. The V1 replay window is an
admission bound; checkpoints retain history and do not erase receipts or events.

Defaults are 16 separate HTTP event slots, 32 records per page, a 1024-position
replay window, one-second polling and a five-second storage-read budget. Streams
share the HTTP response lifetime (30 seconds by default) and release
credentials, response bodies and admission on disconnect or expiry, including
when unpolled. They do not consume the ordinary HTTP read semaphore or native
query workers. Connection-level limits still apply to both transports.
Whole-server throughput and storage request counts remain release qualification
work.

## Focused acceptance

Service and HTTP fixtures cover ordered replay, checkpoint/restart, orphaned
native staging, denied publication, late duplicates, malformed/foreign cursors,
future/old/epoch resets, live epoch replacement, corrupted selected records,
queued revocation, stalled storage, local wake without waiting for the poll,
lagged hints across 70 publications with two-record pages, HTTP resume headers,
independent stream admission and unpolled-body expiry.

The production server fixture verifies native RPC publication through TLS SSE,
resume after server/RustFS restart and cache deletion, and live token
revocation. On 28 September 2026 the full external harness passed all nine
probes against pinned RustFS
`rustfs/rustfs@sha256:b7014e0ce2bc703c1316b3ef760e29dfae61fe4a50d1a66fa89638e0f8ea211f`.
The run used 20 rounds with 16 contenders for create and CAS, then cold-started
the generated localhost TLS server with explicit certificate trust. It proved
live committed notification, `Last-Event-ID` replay after server and RustFS
process restart plus cache deletion, durable graph/file recovery, stream
termination on token revocation, and continuing HTTP/native denial. The uniquely
named container and volume were removed afterward. Multi-node routing, external
certificate provisioning, and load balancer behavior remain outside the
single-node M4.2 gate.
