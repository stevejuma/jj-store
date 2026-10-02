# jj-store desktop

The initial macOS ARM64 host runs the existing Svelte application from its own
LocalTrusted service. Its private installation belongs to the app, and its owner
pipe stops the service and RustFS on Quit or host exit. Closing the window hides
it; the tray and Dock reopen it. This is an implementation foundation, not a
qualified distribution release.

Build the paired binaries and browser, then stage and run:

```sh
pnpm desktop:prepare
pnpm desktop:run
pnpm desktop:bundle
```

Reuse explicit development artifacts without rebuilding unrelated runtime:

```sh
node scripts/desktop.mjs stage --server-dir /absolute/target/debug --rustfs-runtime /absolute/jj-store-rustfs --web-root /absolute/web/dist
pnpm desktop:run --state-dir /absolute/private/disposable-state
```

The independent desktop Cargo lock isolates Tauri from upstream JJ and RustFS.
`pnpm desktop:test` checks host logic without native dependencies;
`pnpm desktop:check` checks the native host. All binaries resolve from staged
resources; installed `jj` is never replaced. State-path overrides apply only to
debug builds. Retain the default port after initialization; another listener on
that port is an explicit error rather than an implicit service connection.

Native privileged controls, custom chrome, first-render handshake, dirty-edit
Quit protection, signed distribution and automatic updates remain pending in the
single canonical progress ledger. The initial icon is a development placeholder.
No Buzz source is copied in this slice; its Tauri lifecycle is a source-pinned
design reference.
