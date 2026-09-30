Synchronize a bound native repository through explicit local controls.

`jj sync reconcile` reads one exact authoritative snapshot and creates one
receipt-backed native reconciliation operation only when several operation heads
are present. The command selects the first head in canonical ID order as the
deterministic merge base and derives its retry request ID from the complete
snapshot. Repeating the same request returns the existing receipt.

Reads, replication imports, and background transfer do not run reconciliation.
The command merges native views, including distinct workspace targets, while
leaving physical working-copy files under each local workspace's normal JJ
snapshot and update lifecycle.
