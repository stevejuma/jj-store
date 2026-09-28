# Repository browser

The current browser supports authenticated repository reads, repository search
and core repository commands. It shows commits, operations and evolution, native
metadata, directories, bounded file previews, structured file differences and
conflict descriptors. Conflict editing remains deferred in the progress ledger.

## Build and serve

Run `pnpm install --frozen-lockfile` and `pnpm web:build`. The build writes
`web/dist/index.html`, hashed assets and `asset-manifest.json`. Deploy the whole
bundle together and set `web_root` in the server's TOML to that directory.
Relative paths are resolved against the configuration file. Run
`target/debug/jj-store config check --config <path>` before starting the server;
see the [server guide](cli/server.md) for the command family.

The foreground server serves the bundle at its configured public origin beside
HTTP V1 and native RPC. Assets are checked and preloaded before binding, so a
bundle update requires a server restart. Unknown paths return 404. Shared access
requires the server's TLS configuration. The explicit insecure loopback setting
is for local development only.

Enter a full repository ID and a scoped token. Both native and browser clients
use the same repository permissions. The browser keeps the token only in memory;
refreshing the document, disconnecting or losing authorization clears it. There
is no repository enumeration endpoint: select a repository using its full ID.
Never put credentials in the bundle, a URL or a checkout.

## Reading history

Select an operation explicitly when multiple heads exist. Revsets can provide a
logical workspace and author email in Query context. Graph pagination keeps the
original operation, time anchor and query even while the editable form changes.
Run query to apply edits. Expired cursors require explicit reload.

Select a commit to inspect metadata, files and differences. File previews fetch
at most 256 KiB and display text or a binary summary. Symlinks and submodules
remain native descriptors. Source content is never rendered as HTML. Metadata
previews are capped separately; downloadable native data remains available
through the authenticated API.

The snapshot bar identifies the displayed operation and publication position.
New committed events show a newer-history notice without replacing the selected
view. Refresh snapshot captures current heads and starts a fresh subscription.
Index preparation is explicit and cancelable. It retries the original captured
query when ready. The graph display is bounded; narrow a query for large
history.

## Repository search

Use the sidebar search control or `Command+K`/`Control+K` to open repository
search. The command overlay queries the selected operation snapshot through the
authenticated Tantivy endpoint and groups operations, commits and files. A
result navigates with its full stable native identities; queries and opaque
cursors never enter the hash route. Indexing, degraded, truncated, empty,
unavailable and failed states remain explicit. Closing search leaves the current
route unchanged.

## Repository commands

Use the repository action menu in the application header to create or describe a
change, create/move/delete bookmarks, abandon a change, or reconcile concurrent
operation heads when the connected token authorizes that command. The dialog
first prepares and displays the exact typed native request. Apply reuses the
same request ID and captured heads; destructive commands add a separate
confirmation. A committed receipt advances the displayed snapshot and restores
valid route selections. An unknown publication outcome keeps the same request
available for retry or durable status lookup.

Semantic policy denials, including protected bookmarks or commits, remain in the
authenticated session and display as command failures. Revoked or otherwise
invalid credentials disconnect the session and clear protected cache state.

## Development acceptance

For local interactive review, `pnpm dev:restart` builds and starts the complete
loopback stack: an owned pinned RustFS container, the native server, the built
browser, a fresh native demo repository and its Tantivy worker. It does not
return until the repository index is current. The command prints the repository
ID and one-time admin token needed by the in-memory browser session. Runtime
files and credentials stay outside the checkout under the operating system
temporary directory. The command refuses an unowned listener or Docker container
rather than stopping it. `pnpm dev:stop` removes only the stack it owns.

`pnpm web:check` checks Svelte/TypeScript; `pnpm web:test` covers bounded
parsing, credentials and graph layout. Install the matching test browser with
`pnpm exec playwright install chromium --only-shell`, then run
`pnpm web:test:browser`. The latter builds production assets, captures fresh
real native HTTP responses and launches an isolated Rust asset server on
port 7458. It refuses an existing listener. API responses in this browser lane
use those captured fixtures plus explicit race/error variants.

Run `pnpm web:test:real` for the separate native deployment lane. It uses
`pnpm dev:restart` to build the UI and binaries, starts the owned pinned RustFS
and native server with search worker, proves authenticated native Tantivy
results, publishes through another CLI process, and verifies that SSE announces
the new history without changing the displayed snapshot until the user
refreshes. The test preserves the selected bookmark across that refresh and
previews and applies one native browser mutation before removing the disposable
stack.

This cold interaction/accessibility lane checks the production CSP and asset
router. It is separate from RustFS/TLS deployment qualification. Screenshot
review is visual-pending and does not approve a baseline. `pnpm spec:check` runs
all browser lanes as well as Rust formatting and native conformance checks.
