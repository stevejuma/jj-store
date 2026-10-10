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
`GET /v1/events`. Version-1 ready/changed/reset frames contain repository or
installation domain witnesses and optional exact task/request identities, never
repository bodies. Native durable WAL cursors remain unchanged. The
process-local application cursor binds server generation, credential scope and
bounded replay; restart, overflow or lost continuity sends reset and closes the
stream. Current authorization is checked before every queued frame and
heartbeat. Admission allows a separate 300-second finite stream with 15-second
heartbeats; the browser treats 45 seconds without traffic as a stalled feed.
Shared refcounted metadata monitors and filesystem hints validate committed
changes within five seconds under qualified healthy load. Idle monitors retire
after a bounded grace period.

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
