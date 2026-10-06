# Replication controls

For a native repository with one configured upstream, `jj sync status`, `now`,
`pause` and `resume` expose the server-owned worker without placing network work
in ordinary JJ commands. Status reads only local worker, checkpoint and search
state; it neither contacts the upstream nor reconciles operation heads. Pause
preserves local reads, mutations and durable peer progress, while now and resume
wake the worker and return without waiting for convergence.

`jj sync wait --timeout <duration>` captures finite issuer-bound local and peer
frontiers before waiting for them. The explicit timeout accepts `ms`, `s` or `m`
and is bounded from one millisecond through five minutes. `jj replica inspect`
reports the exact local, peer, push and pull frontiers, checkpoint revision and
search freshness. These controls return versioned structured results in JSON
mode and never return the configured credential or its resolved value.

`jj sync reconcile` remains a separate receipt-backed application mutation. No
status read, wait, import or background worker step invokes it implicitly.

The command guide documents the minimum activation sequence with explicit
development binary paths: initialize and validate a persistent LocalTrusted
service, migrate an existing authenticated workspace when needed, add one
repository/upstream configuration and its environment credential reference,
restart the server, then inspect and wake the worker or wait for a captured
frontier. Migration and ongoing replication configuration remain separate
operations, and neither rewrites physical working-copy files.
