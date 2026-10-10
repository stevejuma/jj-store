# Scoped application freshness

Application freshness uses bounded process-local semantic notices, separate from
repository authority and the existing durable native WAL event cursors. Shared
monitors compare committed manifest/domain witnesses and external local heads
and companion references, never reconstruct history or reconcile during reads.
Current authorization gates subscription and delivery; internal monitor state
never grants access. A restart, overflow or authorization-scope mismatch resets
continuity. Unchanged observations, repeated domain witnesses and stream renewal
produce no data reload. Browser data polling is disabled even when the feed is
missing or stalled; reconnect preserves cached reads and reconciles retained
witnesses. Request header admission and heartbeat lifetimes remain distinct,
including selected agent activity streams. Observation pressure (Busy, deadline
or resource admission) MUST retain the last repository witness and retry through
the existing monitor. It MUST NOT announce a repository as unavailable or
trigger a read/refresh feedback loop. Confirmed access or source failures retain
their unavailable/invalidation path.

The optional bootstrap `application_events` advertisement selects authenticated
`GET /v1/events` for existing SSE clients and `GET /v1/events/ws` for the
application WebSocket client. Version-1 ready/changed/reset frames contain
repository or installation domain witnesses and optional exact task/request
identities, never repository bodies. Native durable WAL cursors remain
unchanged. The process-local application cursor binds server generation,
credential scope and bounded replay; restart, overflow or lost continuity sends
reset and closes the stream. Current authorization is checked before every
queued frame and heartbeat. Admission allows a separate 300-second finite stream
with 15-second heartbeats; the browser treats 45 seconds without traffic as a
stalled feed. Shared refcounted metadata monitors and filesystem hints validate
committed changes within five seconds under qualified healthy load. Idle
monitors retire after a bounded grace period.

WebSocket upgrades require GET, exact configured Host, a mandatory exact Origin,
and the single `jj-store-events-v1` subprotocol. On this route alone that
subprotocol replaces `x-jj-store-protocol`; present Fetch Metadata requires
same-origin / websocket / empty. Queries, cookies, Authorization headers and
method overrides are rejected. Other HTTP routes retain their existing checks.
The first text message is a strict `{version:1, token, after?}` record, at most
8192 bytes, with a five-second authentication/setup deadline. Authenticated
profiles require the existing bearer; LocalTrusted requires null/absent token.
Tokens remain memory-only and never enter URLs, subprotocols or diagnostics. No
event or witness is sent before current authorization succeeds.

WebSocket server messages carry `{id, event}` using the exact version-1 event
and cursor contracts. A `{version:1,type:"heartbeat"}` message renews liveness
without invalidation. Both transports use the same subscription, monitor leases,
replay/reset and per-delivery authorization. The upgrade owns a bounded event
permit through teardown, independent of ordinary HTTP admission. Client frames
are bounded; subsequent data messages cannot issue commands. Slow sends and
handshakes are bounded to five seconds; the existing finite event lifetime
remains 300 seconds. Close 1000 renews, 4403 denies, 4400 rejects malformed
input, and 1013 retries with existing bounded backoff. Close/reset releases
leases.

Browser sharing retains the version-1 event and authorization scope. A worker
shares only connections with the same credential and deployed asset, keeps
bounded witnesses in memory and closes the upstream transport after its final
subscriber leaves. A synthesized catch-up snapshot omits the last exact identity
when its domain witness can cover multiple missed changes; current subscribers
continue receiving exact change identities. Server-selected cursors and current
authorization remain authoritative.

Installation-local `agent_execution` and `agent_settings` notices join the
shared feed for LocalTrusted Admin only. Supervisor settlement, runtime
preparation and worker transitions invalidate execution; committed defaults
changes also invalidate agent settings. Repeated journal/publication bookkeeping
with the same visible state emits no new notice. These process-local hints carry
no prompt, settings body, credentials or repository authority. Lost producer
notices use the existing full continuity reset. Workspace/discovery/code notices
invalidate agent target selections independently; activity does not invalidate
targets.

Local differently embedded references MUST prove the source is an ancestor of
the explicit outer selection using bounded operation metadata. Indexed cold
recovery reports the source in `local_snapshot.operation`; success keeps the
outer `snapshot.operation` and source reference-page operation. Browser
validation accepts only those exact pins. Signed cursors bind both operations
through the outer selection and complete request digest. No schema change is
required.

Caller abort releases its query observer synchronously before rejecting. An
immediate identical retry therefore starts fresh transport when no other caller
remains, while an active shared caller continues to own the existing transport.
Normal completion and final cleanup release the observer exactly once.

Targeted reaction invalidation reads the target from the query argument segment
of the shared scope/resource key. A trailing epoch segment must not hide the
target. Refetch only the matching target while retaining unrelated cached data
and other authorization sessions.

Local operation-filter queries with child suffixes reuse exact prepared
operation neighbors. Historical ID-prefix selectors retain explicit cold
exact-operation rebuild recovery; selectors rooted at `@` remain usable through
the bounded metadata-only cold reader. Signed cursors bind the complete query.
Missing, ambiguous or outside-snapshot children remain explicit selector errors.

Native prepared operation-filter pages MUST hydrate selected operation metadata
through fresh scoped stores, resolve prefixes and child suffixes from the exact
retained ancestor projection and keep the outer operation/query cursor binding.
A warm lookup never grants access or establishes committed membership. Missing
artifacts retain bounded fallback or explicit preparation; no GET publishes or
reconstructs an unbounded graph.
