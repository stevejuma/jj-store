# Server commands

- `init --local --state-dir PATH`: create a private managed local installation.
- `run`: serve configured storage in the foreground.
- `config example` and `config check`: print or validate strict format 2.
- `doctor`: check access to a known repository.
- `operator init`, `grant`, `list`, `revoke`: manage the local creation policy.
- `version`: display build information.
- `completion`: generate shell completions.
- `help`: read command or keyword help.

Use the explicit development paths `target/debug/jj-store` and
`target/debug/jj`.

Use `jj-store token` to list repository grants and apply, inspect or retry typed
grant/revoke requests through the authenticated API. See
`jj-store token --help`.
