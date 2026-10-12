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
targets and restore remain future work.

Shared listeners require TLS certificate and private-key paths. Plain HTTP is
allowed only for an explicitly enabled literal loopback listener and endpoint.
The default request limits are eight concurrent requests, 30 seconds and 32 MiB
per native blob. Configure one serving instance per storage namespace; the local
state lock does not provide distributed fencing.

## Authenticated remote agents

Remote execution is disabled unless `[remote_agents]` is present. It requires
authenticated access, native storage and a Unix host. Configure the paired
custom JJ and ACP executables and operator-owned agents; changes apply at
restart. Paths resolve relative to this private configuration file.

```toml
[remote_agents]
acp_executable = "/opt/jj-store/bin/jj-acp"
jj_executable = "/opt/jj-store/bin/jj"

[[remote_agents.agents]]
principals = ["11111111111111111111111111111111"]

[remote_agents.agents.setup]
id = "builder"
name = "Builder"
avatar = "🤖"
description = "Build and review code"
instructions = "Work only in the admitted repositories."
runner = "codex"

[remote_agents.agents.setup.preferences]
mode = "ask"

[remote_agents.agents.setup.options]
inherit_defaults = false
```

Use existing provider environment or OS-credential references for secrets;
literal credential values remain forbidden. Enroll each allowed principal's
token in the installation directory and grant that same principal repository
Write access independently. Homes must already contain WorkStore metadata.
Delegated native access is limited to the admitted home and attachments, with
current token/repository checks during dispatch, streaming and publication.
Remote callers cannot create repositories or administer agents, tokens or the
machine. This trusted-collaborator profile is not an OS isolation boundary.

Device-local connection records live under `state_dir/connections`; remote
execution scopes and request journals live under `state_dir/remote-agents`.
Connection tokens and session handles stay in memory. Execution journals retain
unknown outcomes across restart without replaying prompts. Four workers and 64
pending admissions are bounded across caller partitions. M27 records Linux and
macOS qualification independently.

## Native search configuration

Native background search indexing is opt-in: add one `[[search]]` entry per
repository with `repository` and `token_environment`, whose name starts with
`JJ_STORE_SEARCH_TOKEN_` for authenticated access. Supply a read token for that
repository in the named environment variable. LocalTrusted indexing uses the
installation principal and does not require that token reference. Use the
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

## Writable replication

Add one `[[replication]]` entry for each local repository that should exchange
accepted publications with one independently writable upstream:

```toml
[[replication]]
repository = "00000000000000000000000000000000"
upstream = "https://jj.example.test/"
token_environment = "JJ_STORE_REPLICATION_TOKEN_MAIN"
ca_certificate = "./upstream-ca.pem"
```

Issue a write token for the same repository on the upstream and place it only in
the named environment variable before starting `jj-store`. Names must begin with
`JJ_STORE_REPLICATION_TOKEN_`. The configuration stores the name, never the
credential. The server resolves the credential lazily into memory and
re-resolves it after a transport failure, which allows token rotation without
putting credentials in a checkout or status response.

Upstreams require HTTPS. A literal loopback HTTP endpoint additionally requires
`insecure_loopback = true`. At most eight repositories may be configured and a
repository may appear only once. Each entry can override the bounded defaults
under `[replication.limits]`: 128 origins per page, 100,000 closure entries,
100,000 objects and 1 GiB per step, 64 MiB per native object, a five-second
poll, 500 ms initial retry, 60-second maximum retry, an eight-failure circuit,
60-second circuit cooldown and a 30-second control timeout.

The server starts and stops every configured worker with its own lifecycle.
Remote failure degrades replication only; foreground repository reads and
mutations remain local. Durable push and pull checkpoints live in the local
authoritative object store. Accepted imports wake committed-change SSE and, when
search is configured for that repository, advance the search adapter from the
destination WAL position.

The initial activation profile is qualified by focused configuration, lifecycle,
outage, authenticated RPC, worker, SSE and search-projection tests. Exhaustive
pressure and fault soak, expanded memory and disk matrices, browser replication
UI, cold two-instance real-RustFS replication, pressure archive or export
tooling and additional deployment topologies are deferred.
