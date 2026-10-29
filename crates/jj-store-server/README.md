# jj-store-server

Application host composition: installation configuration, repository discovery,
managed storage, capture, search, agent execution, workflow hosting and process
lifetime. Lower-level stores retain repository publication authority.

Use the `config`, `storage`, `installation` and `process` modules when embedding
a host. Normal operators use the `jj-store` binary from `jj-store-cli`, which
composes these resources with the HTTP and native RPC services.

See [configuration](../../docs/site/reference/configuration.md),
[process lifecycle](../../docs/site/technical/lifecycle.md) and
[server commands](../../docs/cli/server.md).

Agent execution and local/shared inference use the local `lapis-ai-runtime` and
`lapis-ai-config` crates. The server owns admission, credentials, workspaces and
durable receipts; Lapis AI owns native process clients and helpers. Select a
verified helper bundle with `JJ_STORE_AI_BUNDLE_DIR`. Development preparation is
`pnpm ai:prepare`; see [local compute](../../docs/local-compute.md) for details.
