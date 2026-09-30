Synchronize a bound native repository through explicit local controls. When the
repository has a configured upstream, `jj-store` starts its bounded
bidirectional worker with the server and stops it during graceful shutdown.
Ordinary repository commands continue to use local authority during upstream
outage.

- `jj sync status` reads local worker, checkpoint and search state without
  contacting the upstream. It reports local readiness and keeps issuer-bound
  local and peer positions separate.
- `jj sync now` wakes connection discovery or the running worker and returns
  immediately.
- `jj sync pause` stops transfer while preserving local reads, writes and
  durable peer progress. `jj sync resume` reenables transfer and wakes the
  worker.
- `jj sync wait --timeout 30s` captures finite local and upstream frontiers at
  invocation and waits only for those frontiers. The timeout accepts `ms`, `s`
  or `m` units and is capped at five minutes. Capturing the frontier requires
  the upstream to be reachable.
- `jj replica inspect` shows exact local, peer, pushed and pulled frontiers plus
  checkpoint revision and search freshness.

Pass `--format json` to any control for the versioned structured result.
Credentials are never returned. The configured upstream token remains in the
server process and the worker re-resolves its named environment variable after a
transport failure.

`jj sync reconcile` reads one exact authoritative snapshot and creates one
receipt-backed native reconciliation operation only when several operation heads
are present. The command selects the first head in canonical ID order as the
deterministic merge base and derives its retry request ID from the complete
snapshot. Repeating the same request returns the existing receipt.

Reads, replication imports, and background transfer do not run reconciliation.
The command merges native views, including distinct workspace targets, while
leaving physical working-copy files under each local workspace's normal JJ
snapshot and update lifecycle.

An accepted import produces the same committed-change freshness hint as a local
publication. If search is configured, its adapter observes the imported local
WAL position; search lag or failure does not block native repository access.
