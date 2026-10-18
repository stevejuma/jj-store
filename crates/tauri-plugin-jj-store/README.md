# tauri-plugin-jj-store

Read-only local JJ history and diffs for an embedding host. It reuses
`jj-store-jj/local-history` and owns bounded preparation jobs and
operation-pinned handles. It has no repository initialization, conversion or
mutation command.

Implement `Admission` using your application's physical vault and window policy.
Return a fresh `local::Access` whose authorization checks remain valid
throughout each read. Construct `History` with a private native cache root and
explicit limits. The embedding host supplies an active Tokio runtime. Call
`heads`, then `open` for a selected operation; poll `status` and use `read` once
ready. Basic operation-log pages and exact parent deltas/grouped revision
changes share that original pin; advanced queries retain separate qualification.
Use `blame` with an exact commit/path for native line attribution; retain its
opaque continuation while incomplete. It shares the handle's admitted pin and
read lifetime. The default grant remains empty. `close`, `revoke` and `shutdown`
cancel and await jobs.

The default feature exposes the independently testable native service. Enable
`tauri` and register `Builder::new(history).build()` for the optional transport.
Permissions default empty; grant `jj-store:local-history` only to intended
windows. The transport derives the actual window/URL, never accepts source paths
or admission generations, and asks your policy to authorize every read.
Transport requests and successful results are versioned JSON text envelopes;
clients must preserve full-width integers when decoding. Direct Rust calls stay
typed. Run `cargo run --example schema` in this crate for the public schema.

See the [local embedding manual](../../docs/site/technical/local.md) for pin,
cache, authorization and qualification details. Notes integration and native
WebView acceptance are separate from service tests.
