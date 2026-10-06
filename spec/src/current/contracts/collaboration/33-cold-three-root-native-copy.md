# Cold three-root native copy

The WAL authority can build one bounded copy closure from a manifest selected by
successful cold recovery. It enumerates native inventory, current and retained
catalog pages, WAL/control records, settings, receipts, checkpoints, and the
exact AgentStore and ReviewStore root/object closures without bucket listing.
Review objects retain any referenced AgentStore objects, and every current or
historical domain root is included. The same publication, closure, byte and
timeout budgets that bound recovery also bound copying.

Stored native objects and immutable control dependencies are copied with
conditional creation and exact byte comparison. The canonical selected manifest
is installed last. A lost final acknowledgement succeeds only when the
destination contains those exact manifest bytes; different pre-existing bytes
fail closed. The destination then cold-recovers through its own native
dependency source and must reproduce the exact manifest digest plus code,
AgentStore and ReviewStore roots before success is returned. This is a local
native snapshot copy primitive, not backup, migration, remote export, failover
or promotion; those deployment behaviors remain deferred.
