# Desktop host development

The initial macOS ARM64 host runs the existing Svelte application from its own
LocalTrusted service. Its private installation belongs to the app, and its owner
pipe stops the service and RustFS on Quit or host exit. Closing the window hides
it; the tray and Dock reopen it. This is an implementation foundation, not a
qualified distribution release.

Run development commands from the repository root. Build the paired binaries and
browser, then stage and run:

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

The macOS window overlays native traffic lights on the application header and
hides the separate titlebar title. Its header reserves control space and
supports dragging; ordinary browser presentation keeps its existing spacing.

Native folder controls, first-render handshake, dirty-edit Quit protection,
signed distribution and automatic updates remain pending in the single canonical
progress ledger. The initial icon is a development placeholder. Native Glass
background follows Buzz's Sidebar vibrancy with an opaque native window,
transparent webview and one translucent theme wash. Its adaptations and
Apache-2.0 attribution are recorded in `docs/ui-provenance.json`. Presentation
commands and the typed terminal bridge are restricted to the managed main window
and exact service origin.

## Debugging the desktop UI in a browser

Use the production Svelte screens with browser DevTools and Vite hot reload:

```sh
# Once on a fresh workspace: build the backend and initialize development data.
pnpm dev:restart
# Subsequent UI work: reuse those binaries and the existing development fixture.
pnpm dev:desktop:ui
```

Open `http://127.0.0.1:7460/`. This mode uses the real LocalTrusted development
backend on port 7462 and preserves the workspace's `.local/dev-demo` data.
Header and Help geometry follow the macOS desktop layout, including the native
control clearance; no imitation window controls are rendered. Terminal access,
window dragging, native glass and agent sleep prevention require the native host
and remain unavailable here. Desktop browser glass keeps opaque surfaces.
Repository operations and other service-backed features use the real backend.

UI edits hot reload without Rust, RustFS or UI rebuilds. After Rust changes, use
`pnpm dev:backend:build` followed by `pnpm dev:backend:restart`; Vite and the
selected desktop presentation remain running. `pnpm dev:stop` stops the owned
development processes while preserving data. `pnpm dev:ui` switches back to
ordinary browser presentation. Both UI commands share the existing prerequisite
and port checks; neither attaches to the installed desktop application.

Existing `JJ_STORE_DEV_ROOT`, `JJ_STORE_DEV_SERVER`,
`JJ_STORE_DEV_RUSTFS_RUNTIME`, `JJ_STORE_DEV_PORT` and
`JJ_STORE_DEV_BACKEND_PORT` overrides also apply. Presentation selection is
limited to development; production browser builds retain browser layout. Native
WKWebView, vibrancy and distribution acceptance remain separate gates.

## Repository terminal

In a repository, use the bottom-panel icon beside the right-sidebar toggle in
the application header, or Cmd+J. Shells open in verified registered folders.
When several folders are linked, choose the folder for each new tab; missing
folders link through Repository settings. Hide/navigation and renderer reload
keep shells running. Closing a tab terminates its shell; Quit terminates
sessions before stopping the managed service. Sessions and output are
memory-only and are not restored after quitting. Only dock size is retained.

The terminal is an ordinary local-user shell. It does not provide remote server
execution, create checkouts, publish repository metadata or grant sandboxing.
Runtime environment credentials are excluded from child inheritance. The native
PTY/Alacritty engine and transport are adapted from the attributed Buzz source;
no external source checkout is needed to build or run.
