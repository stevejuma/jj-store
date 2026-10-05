# Sync and reconciliation

Native replication exchanges accepted publications with one explicitly
configured authenticated upstream per repository. Foreground reads and commands
continue using local authority during an upstream outage.

## Configure the worker

Add a `[[replication]]` entry to the installation configuration using the exact
local repository ID, upstream endpoint and credential environment reference.
Grant that credential write access to the same repository on the upstream.
Validate configuration and restart the local server. An attachment or migration
does not automatically configure ongoing replication.

```toml
[[replication]]
repository = "00000000000000000000000000000000"
upstream = "https://jj.example.test/"
token_environment = "JJ_STORE_REPLICATION_TOKEN_MAIN"
ca_certificate = "./upstream-ca.pem"
```

The all-zero ID is a placeholder: replace it with actual authority. Never put a
bearer secret into the TOML or URL. HTTPS is required except explicitly enabled
literal loopback HTTP. See [configuration](../reference/configuration.md).

## Observe and control

```sh
jj sync status
jj sync now
jj sync wait --timeout 30s
jj replica inspect
```

Status and replica inspection are local reads. `now` wakes transfer and returns
without claiming completion. `pause` preserves local access and durable peer
progress; `resume` reenables configured transfer. `wait` contacts the upstream
to capture finite frontiers, then waits for those targets, not an ever-moving
notion of complete synchronization. Timeouts accept `ms`, `s`, or `m` and are
bounded to five minutes.

## Reconcile deliberately

Several operation heads can exist after independent publication. Reads and
imports do not reconcile them. `jj sync reconcile` prepares one receipt-backed
operation against an exact authoritative snapshot. Its request identity is
stable for that snapshot; repeating it does not create a second reconciliation.
Physical working files remain under the foreground workspace's JJ lifecycle.

WorkStore homes freeze existing origin ledgers until their reducer is qualified.
Local companion history does not use native replication. Expanded two-instance
RustFS, pressure/fault and topology qualification remain separate from the
implemented local-first worker and controls.
