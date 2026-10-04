# Repository browser

The current browser opens repositories discovered from a LocalTrusted
installation without user credentials. It shows commits, operations and
evolution, native metadata, directories, bounded file previews, structured file
differences and complete conflict descriptors. Search indexing and authenticated
shared access remain deferred from the first local release.

## Build and serve

Run `pnpm install --frozen-lockfile` and `pnpm web:build`. The build writes
`web/dist/index.html`, hashed assets and `asset-manifest.json`. Deploy the whole
bundle together and set `web_root` in the server's TOML to that directory.
Relative paths are resolved against the configuration file. Run
`target/debug/jj-store config check --config <path>` before starting the server;
see the [server guide](cli/server.md) for the command family.

The foreground server serves the bundle at its configured literal-loopback
origin beside HTTP V1 and the owner-only Unix RPC socket. Assets are checked and
preloaded before binding, so a bundle update requires a server restart. Unknown
paths return 404. The installation catalog auto-opens its sole repository or
shows a chooser when several are registered. It contains only verified discovery
metadata; repository authority remains in embedded RustFS. No credential enters
the bundle, URL or browser storage.

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
Relevant committed events refresh active content automatically. Explicit
historical operations, drafts and pending commands remain selected. Refresh
snapshot captures current heads; the authorization session retains its shared
event subscription. Index preparation is explicit and cancelable. It retries the
original captured query when ready. The graph display is bounded; narrow a query
for large history.

## Repository search (deferred locally)

Use the sidebar search control or `Command+K`/`Control+K` to open repository
search when a separately configured Tantivy worker is available. The command
overlay queries the selected operation snapshot and groups operations, commits
and files. A result navigates with its full stable native identities; queries
and opaque cursors never enter the hash route. Indexing, degraded, truncated,
empty, unavailable and failed states remain explicit. Closing search leaves the
current route unchanged.

## Repository commands

Use the repository action menu in the application header to create or describe a
change, create/move/delete bookmarks, abandon a change, or reconcile concurrent
operation heads under the installation's configured LocalTrusted permission. The
dialog first prepares and displays the exact typed native request. Apply reuses
the same request ID and captured heads; destructive commands add a separate
confirmation. A committed receipt advances the displayed snapshot and restores
valid route selections. An unknown publication outcome keeps the same request
available for retry or durable status lookup.

Semantic policy denials, including protected bookmarks or commits, remain in the
local session and display as command failures.

## Conflict resolution

Select a commit with native conflict descriptors, then choose Resolve conflicts
from the repository action menu. The editor shows every path, fingerprint and
ordered native term without flattening the conflict. Choose one advertised term,
deletion or a separately staged merged file for every path before previewing the
complete atomic request. Unsupported shapes and stale fingerprints remain
visible and never trigger an implicit reread or partial publication.

Merged file uploads are bounded by the server-advertised limit and bound to the
repository, snapshot operation, commit, path and conflict fingerprint. Uploading
bytes alone does not publish history. Preview and apply retain the same request
identity, and only a committed receipt advances the displayed snapshot.

## Development acceptance

For local interactive review, `pnpm dev:restart` builds and starts the complete
loopback stack: the pinned embedded RustFS helper, native server, built browser
and a registered native demo repository. The command prints the URL, repository
ID, startup measurements and runtime directory; no user credential is required.
The installation identity, repository authority and explicit RustFS volume stay
in the ignored checkout-local `.local/dev-demo` directory. The command refuses
an unowned listener or process rather than stopping it. `pnpm dev:stop` stops
only the process it owns and preserves the fixture. `pnpm dev:reset` stops the
owned process, replaces the directory and reseeds the same deterministic sample
shape. The sample includes Codex, Cursor and OpenCode sessions with transcripts
and tools, plus open and resolved review discussions for repository, operation,
revision, code-text, code-artifact, agent-text and agent-artifact anchors. It
also exercises reply, edit, tombstone, moderation hide, resolve and reopen
history. Native IDs generated by JJ remain stable between ordinary restarts and
may change after an explicit reset.

`pnpm web:check` checks Svelte/TypeScript; `pnpm web:test` covers bounded
parsing, credentials and graph layout. Install the matching test browser with
`pnpm exec playwright install chromium --only-shell`, then run
`pnpm web:test:browser`. The latter builds production assets, captures fresh
real native HTTP responses and launches an isolated Rust asset server on
port 7458. It refuses an existing listener. API responses in this browser lane
use those captured fixtures plus explicit race/error variants.

Run `pnpm web:test:real` for the separate native deployment lane. It uses
`pnpm dev:reset` with an isolated OS-temporary root to build the UI, native
binaries and embedded RustFS helper, auto-opens the sole LocalTrusted repository
without credentials, publishes through another Unix-RPC CLI process, and
verifies that SSE announces the new history without changing the displayed
snapshot until the user refreshes. The test preserves the selected bookmark
across that refresh, resolves a seeded native conflict atomically, and previews
and applies another native browser mutation before removing the disposable
stack. `pnpm test:local-release` adds the macOS ARM64 platform assertion and the
30-second startup / 512 MiB combined resident-memory ceilings.

This cold interaction/accessibility lane checks the production CSP and asset
router. It is separate from RustFS/TLS deployment qualification. Screenshot
review is visual-pending and does not approve a baseline. `pnpm spec:check` runs
all browser lanes as well as Rust formatting and native conformance checks.
