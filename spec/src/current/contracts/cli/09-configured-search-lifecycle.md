# Configured search lifecycle

Server TOML may contain up to eight explicit search entries. Each binds a full
repository ID, source scopes/exclusions and bounded polling, batch duration and
per-generation disk capacity. Authenticated deployments require a dedicated
`JJ_STORE_SEARCH_TOKEN_` environment-variable name. LocalTrusted entries omit it
and bind the worker through a stable installation-derived principal with read
permission. Tokens are never accepted in TOML or command arguments. Config
checks reject credentials in LocalTrusted entries and missing credential
references in authenticated entries without reading secrets or starting
indexers.

Each entry may set `snapshot_every` to an interval of committed source
positions; the default is 32 and zero disables optional remote index snapshots.
The interval is bounded at one million. Snapshot transfer budgets derive from
the configured generation capacity. Authenticated credentials remain available
only through the entry's environment-variable name.

The foreground server prepares private local indexes beneath its state
directory, then starts their committed-log runners alongside the listener.
Preparation fails for missing/malformed credentials or invalid local
configuration. Runtime storage/authorization/indexing failures degrade search
and retry with bounded backoff without taking down authoritative serving.
Shutdown cancels and joins runners; dropping the server aborts their async tasks
and signals retained native workers. Writer leases remain held until those
workers exit.

Search status reports durable, served and repository positions through the
configured deployment's HTTP endpoint. LocalTrusted requests use the literal
loopback authority and no bearer credential; retained authenticated fixtures use
their repository token. Doctor keeps authoritative readiness separate from
configured search health and explicitly reports an unavailable serving endpoint.
It does not start a replacement writer to inspect status. The local development
stack configures the seeded repository, waits for a current Tantivy generation,
and reports readiness without generating a browser or worker credential.

M4.10 native-lifecycle and registered-HTTP acceptance covers conflicted
references, bidirectional signed cursors, recursive paths, operation-change
grouping and changed-path summaries. It proves authorization, exact snapshot
pinning, restart reset and zero publication without adding or changing a CLI
command.

The browser mutation acceptance reuses the native lifecycle fixture and
development `jj` binary for create, describe, bookmark, abandon, reconcile and
resolve semantics; it does not add a parallel command implementation. The
`pnpm dev:restart` task cold-builds the UI and binaries, restarts only its owned
processes and reuses the ignored `.local/dev-demo` native repository,
installation identity and embedded RustFS volume. It initializes that fixture
only when absent, starts its LocalTrusted search worker and prints the URL and
repository ID with `Credentials: none`. `pnpm dev:stop` preserves the fixture.
`pnpm dev:reset` is the only development command that deletes and reseeds it;
the sample file bytes and history shape are deterministic while native IDs may
be regenerated. A missing or invalid fixture marker with partial local state
fails closed and directs the operator to the explicit reset command. Cold
browser acceptance supplies an isolated temporary root and cannot mutate the
interactive fixture.

The checkout-owned demo opts into `development_browser_annotations = true` so
browser annotation helpers can inject inline scripts and styles. Set
`JJ_STORE_DEV_BROWSER_ANNOTATIONS=0 pnpm dev:restart` to exercise the strict
production asset policy. The server setting defaults to false and requires
loopback LocalTrusted access plus a configured web bundle; authenticated serving
rejects it. Restart preserves the native demo repository and its samples.

Fresh development fixtures also publish three sample projects, nine tasks with
all four states and same-project subtasks, active links to the demo repository,
and six threaded comments through the production WorkStore HTTP command route.
`pnpm dev:seed:work` adds these samples to an already running owned LocalTrusted
fixture without replacing its repository or restarting the service. Stable
repository-scoped sample identities make reruns additive: existing project/task
edits, retired links and comment tombstones remain intact. An interrupted
initial task state change can resume only while its original metadata and
version one remain unchanged. Unknown outcomes stop the seed and retain its
request identity; a rerun inspects selected native records before publishing
missing samples. The UI loads newly seeded state only on explicit refresh.

The existing native administration RPC and CLI clients retain the selected
code-state version in snapshot preconditions. This prepares the command layer
for `jj agent enable` and later review commands without treating agent/review
WAL activity as native code movement. The user-facing collaboration command
family remains M9.4 work; this publication slice adds no hidden installer,
daemon or controller dispatch.

The M9.4 native-lifecycle fixture hosts the registered LocalTrusted HTTP
application, lists root-pinned AgentStore sessions, resolves current-root
session detail and streams a selected transcript generation without a bearer
credential. It executes the same repository host, literal-loopback admission and
service pager used by the standalone server, and captures collection, detail and
transcript pages for contract validation. It also captures two root-pinned
association pages and exercises session, tool, operation, revision and
checkpoint subjects, including rejection of contradictory selectors. Separate
checkpoint collection, current-detail and exact-version captures verify root
continuity after publication and reject an immutable version from another
checkpoint chain. A separate ReviewStore capture pages stable threads across a
later review publication, resolves the current selected anchor, comments and
events, and returns 404 for a missing stable thread. The same fixture applies
typed ReviewStore commands, reconstructs the HTTP composition and validates
durable status plus identical replay, stale preconditions, changed intent and
read-only denial. This adds no user-facing CLI command; collaboration command
presentation was completed by the final M9.4 slice below.

`jj review` dispatches by the current workspace backend and uses the same
service-owned ReviewStore semantics as HTTP. Native access uses the bound
connection; Git-backed JJ access opens the local companion without a server.
Local `repository` creation explicitly initializes missing metadata; read-only
list/show/status leave absent identity and actor state absent. Local list
continuations use `--after` and `--at-metadata`; historical show accepts
`--at-metadata`. Native cursors and local selectors are disjoint. `list` and
`show` expose bounded current-root data; all writes require caller-retained
request IDs and their exact optimistic-concurrency values. `create` reads one
bounded typed anchor document and rejects a foreign repository. `status`
distinguishes committed, not-known and outcome-unknown receipt states. The
owner-only LocalTrusted Unix socket requires no bearer credential, while
authenticated connections retain their existing token behavior. Review
publication wakes the existing SSE feed; the hint is only an in-process
repository ID and WAL replay supplies the event.

The native `jj review create` anchor document also accepts the additive
`agent_session` target with a stable session ID and exact immutable session
snapshot. The existing generic create, comment and status transports retain
their authorization, request identity and receipt behavior; this adds no CLI
verb or agent dispatch. Session comments use ReviewStore and do not send prompts
to a recorded agent. The filesystem-local profile does not accept agent review
targets.

Installed `jj agent` hooks stamp accepted observations with the local system
clock before normalization and publication. The hook keeps source timestamps
optional, rejects disabled integrations before capture state exists and retains
the existing secret-redaction and restart-ordering boundaries. This adds no
installed service, controller dispatch or credential persistence.
