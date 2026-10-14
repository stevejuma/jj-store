Synchronize a bound native repository through explicit local controls. When the
repository has a configured upstream, `jj-store` starts its bounded
bidirectional worker with the server and stops it during graceful shutdown.
Ordinary repository commands continue to use local authority during upstream
outage.

## Activate replication

Create and start the persistent local service with the development binaries:

```sh
target/debug/jj-store init --local --state-dir /absolute/path/jj-store-local
target/debug/jj-store config check \
  --config /absolute/path/jj-store-local/server.toml
target/debug/jj-store run \
  --config /absolute/path/jj-store-local/server.toml
```

For an existing authenticated native workspace, leave that process running and
hydrate it into the local service from another terminal:

```sh
target/debug/jj store migrate-local \
  --unix-socket /absolute/path/jj-store-local/native.sock
```

This switches only the workspace's `.jj/repo` descriptor after the local copy is
verified; it does not snapshot or rewrite working files. A workspace already
attached to this local service skips the migration step.

Use the reported repository ID to add its single `[[replication]]` entry to
`server.toml` as described in the
[server configuration guide](server-config.md). Grant the named credential write
access on the upstream, load its `JJ_STORE_REPLICATION_TOKEN_...` environment
variable from the local secret facility, validate the edited configuration, and
use **Reload saved settings** in the app to activate that saved configuration.
The server and native socket stay online; existing workers drain before
replacement and retain checkpoints. A subsequent normal startup also loads the
saved configuration. Migration does not add this ongoing replication
configuration itself.

Confirm the active worker from the migrated or attached workspace:

```sh
target/debug/jj sync status
target/debug/jj sync now
target/debug/jj sync wait --timeout 30s
target/debug/jj replica inspect
```

`sync status` and `replica inspect` are local reads. `sync wait` contacts the
upstream only to capture its finite frontier, then waits for that fixed target.

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
