# Server configuration

Use `init --local --state-dir PATH` to generate the supported strict format-2
configuration, and `config check --config PATH/server.toml` to validate it.
Older formats are rejected; this unreleased application has no compatibility or
migration command. Paths are relative to the configuration file. Keep state
outside Git/JJ checkouts. Unknown keys and invalid budgets fail.
`config example` documents authenticated access with external S3 authority.
Initialize its private installation identity and empty repository catalog with
`jj-store registry init --state-dir PATH`, then reference those two files from
`[access]`. External-provider release qualification remains deferred; this is
not the LocalTrusted application setup path.

Format 2 tags `[access]` and `[storage]`, and accepts repeated `[[backup]]`
targets. Run `init --local --state-dir PATH` for the supported application
deployment: a LocalTrusted installation with managed RustFS, private internal
identity and credentials, an empty local repository catalog, an explicit
persistent volume, and backups disabled.

Format 2 names environment or OS-credential references explicitly. Secret values
are never accepted in TOML. A new managed installation creates its bucket once,
publishes and verifies the installation sentinel, then requires both on every
restart. Manual-token authenticated repository discovery is implemented. Shared
deployment release qualification, external S3 provider qualification, backup
targets, replication and restore remain future work.

Shared listeners require TLS certificate and private-key paths. Plain HTTP is
allowed only for an explicitly enabled literal loopback listener and endpoint.
The default request limits are eight concurrent requests, 30 seconds and 32 MiB
per native blob. Configure one serving instance per storage namespace; the local
state lock does not provide distributed fencing.

Background search indexing is deferred from the local release. Its retained
future configuration is opt-in: add one `[[search]]` entry per repository with
`repository` and `token_environment`, whose name starts with
`JJ_STORE_SEARCH_TOKEN_`. Supply a read token for that repository in the named
environment variable. Use the existing token administration commands to
issue/revoke it; the server never creates search credentials or reads them from
a checkout. Rotate by restarting with the replacement environment value.

`search.source.scopes.<scope>` selects `kind = "bookmark"` or `"workspace"` and
a native name. With no scopes, only retained metadata is indexed. Source options
include `index_source`, `include_prefixes`, `exclude_prefixes`,
`max_text_bytes`, `max_files`, `max_work` and `max_output_bytes`. Paths are
native prefixes; exclusions affect only the derived index. Changed source
configuration rebuilds the index at restart before serving it.

Per-entry defaults are `poll_millis = 1000`, `timeout_seconds = 60`,
`index_bytes = 536870912` per generation and `snapshot_every = 32` committed
source positions. Set `snapshot_every = 0` to disable optional immutable remote
index snapshots. At most eight repositories may be configured. Each runner uses
one native extraction worker and a 32 MiB Tantivy writer, retains at most 16
generations, and admits at most 100,000 documents. Each batch caps 100,000
requests, 256 MiB transferred and 16 MiB per object; server inventory limits
also apply. Pinned pages can retain old generations, so provision disk for
retained generations plus a staging generation.

Indexes live under `state_dir/search/<repository>`, with private directory
permissions; writer leases live under `state_dir/adapters`. The running server
owns and cancels these workers. Search faults degrade indexing independently of
native repository serving. `/v1/repos/{repository}/search/status` reports live
freshness; `jj-store doctor` reads that endpoint with the caller's token.
