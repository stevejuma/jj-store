# `jj store migrate-local`

Hydrate the current authenticated native workspace into a complete LocalTrusted
repository and atomically switch the workspace to that local server.

```text
jj store migrate-local --unix-socket /absolute/path/native.sock \
  [--cache /absolute/path/local-cache] \
  [--state-dir /absolute/path/migration-state]
```

The source repository must already have replication identity and the local
server must permit LocalTrusted repository creation or already contain the same
logical repository. The command pulls a complete seed and a pinned feed tail,
verifies the working-copy operation from local storage, then switches only
`.jj/repo`. It does not snapshot, update, reconcile, or rewrite working files.

The exact prior `.jj/repo` descriptor is retained as
`.jj/repo.jj-store-remote-v1`. Retries must use the same socket, cache, and
state directory. Credentials remain in the configured OS credential facility or
process environment and are never written to the migration journal.
