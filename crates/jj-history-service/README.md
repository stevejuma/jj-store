# jj-history-service

Shared operation-pinned local JJ history, file diffs and blame for native hosts.
The service owns bounded preparation, opaque handles, read lifetimes,
cancellation and cache coordination. It depends on `jj-store-jj/local-history`
and owns no UI transport, server, credentials, repository initialization or
mutation policy.

Implement `Admission` with current physical-vault and generation checks and
return a fresh `local::Access`. Supply a host-derived `Context`: its retained
`window` field is an opaque caller scope and `url` is its admitted origin.
Neither field is browser-supplied authority. Hosts own a Tokio runtime and
private cache root. Use `heads`, `open`, `status`, `read` and `blame`; await
`close`, `revoke` and `shutdown` before releasing host admission.

`tauri-plugin-jj-store` supplies the optional Tauri transport and retains the
existing native imports and versioned schema. Hosted applications supply their
own authenticated adapter. See the
[local embedding manual](../../docs/site/technical/local.md) for pin, cache,
authorization and qualification boundaries.
