# Native JJ server

Run a persistent LocalTrusted native repository service with the embedded RustFS
runtime. Start with `jj-store init --local --state-dir PATH`, then pass the
generated `PATH/server.toml` to `jj-store run`. The process stays in the
foreground and owns both the loopback browser/RPC listeners and RustFS child. No
user credential or separately installed storage service is required.

Use `jj-store help -k commands` for the command guide. Repository creation and
native client operations use the custom `jj store` command family.

An optional `web_root` configuration field serves a prebuilt browser bundle from
the same origin. It is resolved relative to the configuration file. See
[Repository browser](../browser.md) for build and acceptance instructions.
Startup rejects missing, oversized or unsafe listed assets. Authenticated shared
serving and external object storage are deferred beyond the local release.

Local stack workspaces also run directly through `jj-store stack`, without a
server or registration. Start with
`jj-store stack init --name NAME --target BOOKMARK` in an ordinary Git-backed JJ
workspace; use `jj-store help stack` for workspace, branch, content, conflict
and native history commands.
