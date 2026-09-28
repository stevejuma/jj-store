# Server configuration

Use `config example` for the TOML template and `config check --config PATH` to
validate it. Paths are relative to the configuration file. Keep state and
operator policy outside Git/JJ checkouts. Unknown keys and invalid budgets fail.

`JJ_STORE_S3_ACCESS_KEY` and `JJ_STORE_S3_SECRET_KEY` supply server-only storage
credentials. `JJ_STORE_S3_SESSION_TOKEN` supports temporary credentials. These
values are never accepted in TOML. The bucket must already exist.

Shared listeners require TLS certificate and private-key paths. Plain HTTP is
allowed only for an explicitly enabled literal loopback listener and endpoint.
The default request limits are eight concurrent requests, 30 seconds and 32 MiB
per native blob. Configure one serving instance per storage namespace; the local
state lock does not provide distributed fencing.

Search is opt-in. Add one `[[search]]` entry per repository with `repository`
and `token_environment`, whose name starts with `JJ_STORE_SEARCH_TOKEN_`. Supply
a read token for that repository in the named environment variable. Use the
existing token administration commands to issue/revoke it; the server never
creates search credentials or reads them from a checkout. Rotate by restarting
with the replacement environment value.

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
