# Captured Sync waits

Sync Progress offers explicit capture-and-wait and continuation of the same
issuer-bound push/pull barrier. Timeout means pending, not failure or a new
target. Stop cancels only the browser wait; replication continues. The barrier
survives Settings section changes but never a repository/session change.

Sync Overview opens the shared reconciliation dialog from a freshly captured
current head set, with an explicit base selector. The underlying historical
route remains retained. Uncertain mutations survive dialog/Settings closure and
prevent a replacement command until the original receipt is recovered.

Storage has Overview and Maintenance tabs. Overview reads current manifest
readiness and checkpoint coverage without starting maintenance. Maintenance
shows the captured repository/epoch/position/digest and bounded report.
Integrity checks run explicitly. Checkpoint preparation shows a fixed target,
followed by an explicit Publish checkpoint action. Operation reports survive
Settings closure and service restart; interrupted integrity checks require an
explicit new run. Unknown checkpoint outcomes offer status recovery or retry of
the same target. A browser request failure retains its request identity. Storage
is unavailable for remote sessions and the filesystem-only companion profile.

Storage controls require explicit repository context and cannot act on the last
selected repository while the Projects hub is open.

An explicit diagnostic-only bound capture appears as a capture coverage gap in
Thread context. Its link opens the Agent directory without inventing a session
or review destination. Exact retained source and binding references remain
available through the typed Thread history; no rendered source bodies or local
paths are inferred from the gap.
