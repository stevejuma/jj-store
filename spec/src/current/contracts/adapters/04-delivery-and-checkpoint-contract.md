# Delivery and checkpoint contract

An adapter cursor identifies repository, authority epoch, committed position and
the exact selected WAL reference (including attempt and digest), plus adapter
name, implementation version, schema version and configuration digest. Position
zero has no WAL reference. A checked manifest and selected catalog establish
commitment; bucket listings, uploaded candidates and numeric positions alone do
not. Bootstrap captures complete native inventory at a consistent selected
snapshot; tail delivery uses each selected record's own inventory and heads.

A durable checkpoint also identifies the local projection artifact. The runner
checks recovered local state against selected history and never treats a remote
cursor as evidence that local files exist. Missing/incompatible artifacts
require rebuild or a compatible restore. Checkpoint CAS follows durable adapter
effects; unknown writes are reconciled by readback, and failures do not skip a
record or fail repository publication. Replay may repeat durable effects. One
local owner holds the repository/adapter writer lock; remote cursor CAS does not
claim distributed fencing or exactly-once external effects.

Before bootstrap, the runner verifies the selected current tail and receipt as
well as complete inventory. A corrupt/missing tail cannot be bypassed by
starting a new projection. Successful checkpoint persistence includes checked
readback; a competing pointer is explicit contention, never an implicit
ownership grant.

Projection callbacks receive a run context carrying deadline, cancellation and
the local writer lease. Background workers retain a context clone until exit, so
timeout/cancellation cannot release ownership while destination work still runs.
Request/byte, page, inventory and per-object limits apply to adapter I/O;
service authorization retains its own session limits. Polling retries the same
failed batch with bounded backoff and reports durable position, remote cursor,
lag and degraded status through a bounded watch channel. No wake callback runs
on the repository publication path.
