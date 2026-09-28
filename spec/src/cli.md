# CLI and server commands

Native search acceptance compares selected-view commit visibility with retained
historical metadata, including invisible predecessors and pagination after
hidden-hit filtering. These fixtures do not introduce CLI search commands.

Search serving fixtures additionally cover signed continuations, freshness
waits, scope selection, request admission, cursor expiry/restart and current
repository authorization through native service sessions.

First custom commands: init, attach, status, capabilities, doctor, serve-local,
and the retained shared-deployment credential commands. The supported local
release path uses `jj-store init --local`, `run`, repository registration and
doctor without user credentials. Authenticated commands, external storage and
search-worker activation remain deferred. JSON is never paged. Custom standalone
no-command invocation prints help successfully; ordinary upstream jj defaults
stay intact. Match jj-nostr presentation semantics without inheriting its
storage or daemon installation policy. Secrets stay out of argv, output, and
checked-in config.

## Requirements

| ID          | Requirement                                                                                                                                                                                                                                                                                                                         |
| ----------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| JST-CLI-001 | Custom commands MUST use jj store under upstream CliRunner, and the LocalTrusted server MUST use explicit jj-store initialization, run, repository registration and diagnostics without user credentials; authenticated token administration is deferred.                                                                           |
| JST-CLI-002 | Custom command families MUST provide authored Markdown long help, short help, keyword help, hints, JSON results, TTY paging, no-pager, color controls, completions, and build information.                                                                                                                                          |
| JST-CLI-003 | Ordinary JJ commands MUST preserve upstream behavior; a tested capability matrix MUST report unsupported backend commands honestly.                                                                                                                                                                                                 |
| JST-CLI-004 | Development MUST use explicit build paths and MUST NOT replace installed jj or start unrelated services.                                                                                                                                                                                                                            |
| JST-CLI-005 | Configuration format 2 MUST explicitly tag access and authority storage, reject every older format, and provide explicit bounded local initialization, verified catalog registration, help and completion commands; configuration migration, authenticated deployment, external authority, backup control and restore are deferred. |

## M0 extension qualification

The Apache-2.0 upstream example compiles at the pinned release and is exercised
through isolated init/describe/log/op-log/evolog commands. Its Git backend is
reference-only; native commands and presentation coverage remain M3.

The native library lifecycle fixture covers the stores used by the future custom
CLI, including independent local caches, repository attachment, transactions and
reopening. It is not executable command-presentation or remote CLI evidence.

The CLI integration library binds remote RPC authority and immutable object I/O
to the existing native factories. Loopback network lifecycle tests qualify this
binding separately from the future executable command presentation matrix.

## Executable native workspace client

The development `target/debug/jj` registers lazy remote factories with upstream
CliRunner. Ordinary JJ command parsing, transactions and workspace updates
remain upstream-owned. Each CLI invocation owns its factory cache and Tokio
runtime; there is no global mutable repository or principal. Native workspace
bindings contain the endpoint, repository, profile and transport limits, never
credentials.

`store attach` creates a separate physical workspace and local metadata
directory. Its destination must be empty and its logical workspace name must be
unique. `store status` and `store doctor` inspect remote heads without
snapshotting local files or reconciling operations. `store capabilities`
distinguishes verified native commands from unsupported or still-unqualified
ones. The ledger records command-family completion and the remaining
presentation matrix.

`store login` validates a token before storing it in the platform credential
facility; stdin or a hidden terminal prompt supplies the secret. `store logout`
deletes only the selected endpoint/repository credential. `JJ_STORE_TOKEN`
overrides stored credentials for automation and is never persisted. Custom text
results and authored help follow jj-nostr paging, Markdown and color
conventions; JSON results are plain, newline-terminated and never paged.

A hosted-RPC acceptance fixture creates a repository through the operator API,
then attaches and describes history with the actual custom jj executable. It
reopens the host with no in-memory repository directory. This qualifies dynamic
routing; standalone process and initialization have separate cold executable
acceptance below.

## Standalone process

`jj-store run --config PATH` serves in the foreground and reports its bound
listener only after validating configuration, TLS and the operator policy.
`jj store serve-local --server-config PATH` uses the same composition and
requires a literal loopback bind address. Both stop on interrupt/termination and
bound graceful shutdown. The supported application path uses embedded format-2
storage, which launches and owns the configured pinned RustFS helper without
installing a daemon. Authenticated external serving remains deferred. On Unix,
both shutdown handlers are installed before the listening event is flushed.
Immediate interrupt or termination after that event must enter graceful
shutdown, even before the listener or search worker future is first polled.

`jj-store config example` prints the format-2 configuration template, while
`config check` validates it without contacting storage. Older formats are
rejected and there is no migration command for this unreleased application.
`jj-store init --local --state-dir` atomically creates a new private
LocalTrusted installation, empty discovery catalog and managed RustFS volume
with backup disabled; it refuses existing state and paths inside a checkout.
`jj store init` and `jj store attach` accept the installation Unix socket as an
alternative to an authenticated endpoint. Their format-2 connection records
contain no token or credential identifier. Login and logout reject this mode.
`jj-store repository register` obtains the lifetime storage lock, opens the
requested native manifest and only then atomically updates the identity-bound
discovery catalog; it cannot register against an authenticated deployment.
`doctor --config PATH --repo-id ID` checks the configured repository through the
owner-only Unix RPC and checks token-free HTTP status when the LocalTrusted
listener is running. It does not publish operations. Authenticated diagnostics,
operator policy and repository token commands remain retained future behavior
and do not gate the local release. `operator init` persists a verifier for an
environment-injected or OS-held operator credential; `operator list/revoke`
administer that trusted local policy file. Token values are never command-line
arguments or output. Repository token commands remain a separate command slice.

On Unix, serve-local is parsed by CliRunner and then replaces the process with
the paired jj-store binary from the same executable directory. This resets the
upstream CLI's immediate-exit signal handler so the server owns graceful
shutdown. Other platforms fail explicitly and direct users to jj-store run;
platform handoff qualification remains a release gate.

Attachment records a bounded, secret-free local journal outside the checkout.
Before publishing it durably retains the exact operation and parent IDs; retries
use that identity through the native receipt path. Connection, credential ID,
workspace and path changes on retry fail explicitly. Cache initialization stages
complete bindings before rename. Physical metadata appears only after
publication is acknowledged; resuming never snapshots or overwrites source
files.

Agent capture uses a separate bounded journal under the same CLI ownership
rules. It exclusively locks a user-restricted path outside checkouts, atomically
persists a validated sanitized batch before acknowledging reader progress, and
then atomically records the exact principal, request ID, publication ID and
typed AgentStore delta before publication. Restart validates pending entries; it
removes only unacknowledged atomic temporary files. Changed retries, corrupt,
symlink or unexpected entries and concurrent ownership fail closed. Completion
requires the matching prepared request/publication identity, so unprepared or
unknown-outcome captures remain recoverable. The journal is implemented as an
internal boundary.

`jj agent status` refreshes the device-local integration registry with bounded
PATH probes for exactly Codex 0.156.1, Cursor 3.22.7 and OpenCode 1.18.30. It
reports missing, supported, unsupported-version and probe-failed states without
retaining subprocess stderr. `jj agent enable --non-interactive --agent ...`
requires explicit supported selections and durably records desired enablement;
`--dry-run` returns the prospective registry without writing it. The registry is
bound to one native repository and canonical workspace, keeps a stable device
identity across restart and reports setup independently as not installed.

`jj agent setup <codex|cursor|opencode> [--dry-run]` requires prior explicit
enablement and exact-version support. Codex setup merges only owned command
handlers into `.codex/hooks.json`; Cursor setup uses the version-1
`.cursor/hooks.json` events; OpenCode installs only
`.opencode/plugins/jj-store-agent.ts`. Dry-run creates no directories, backups,
registry records, rule cache or project files. A real setup first builds the
user-restricted MongoDB Kingfisher compiled-rule cache, then atomically records
exact original bytes and content hashes before replacing the bounded regular
target, preserves unrelated JSON values, rejects configuration symlinks and
becomes idempotent after success. Status audits the installed hash on restart;
drift is reported and blocks replacement. The packaged hook ingestion path and
durable publication loop remain pending M9.2 work.

`store init` provisions a new repository using a separate operator credential,
durably retaining generated repository/epoch IDs before contacting the server.
The initial admin credential is saved to the OS facility or provided explicitly
through the automation environment. It reuses recoverable attachment; repeating
an acknowledged initialization cannot create another repository or workspace.
Current admin access with the exact retained epoch can resolve a lost creation
acknowledgement after operator revocation.

Both executables expose the same typed repository
`token list/grant/revoke/ status/retry` implementation over authenticated native
RPC. Grant credentials enter through hidden input, bounded stdin or a dedicated
environment variable. Before applying, the CLI writes an exclusive, bounded,
secret-free request file outside checkouts with its snapshot, principal, request
ID and intent. Retries never refresh the snapshot. Status queries publish
nothing; absent receipts remain unknown. Current admin access and the original
principal are required; credential rotation for that principal does not change
request identity.

Executable semantic parity compares native RPC repositories with the pinned
upstream Git backend at equivalent histories. Comparisons retain descriptions,
parent counts, local bookmark names, conflict state, file content and evolution
while excluding backend-specific IDs and timestamps. Rewrites,
split/squash/abandon, three-way conflicts, divergent changes and operation
reconciliation require explicit acceptance before the capability matrix
advertises them as verified.

Native graph lifecycle acceptance also projects a tracked remote bookmark beside
its same-named local bookmark, retains their separate identities and verifies
that commit metadata survives filtered graph paging. Browser acceptance scopes a
selected bookmark or working-copy card to exact target ancestry instead of
reusing the repository-wide default graph.

The verified capability matrix includes rebase, squash, split, abandon, undo,
manual conflict snapshotting, divergent operations and conflicted local
bookmarks. Separate Unix fixtures qualify binary/symlink/executable checkout
state and SSH sign/re-sign/unsign behavior. Native Git transport and destructive
GC commands fail without publication. Other external merge tools, signers and
release platforms remain separately qualified; file-copy snapshot evidence does
not replace native copy-history codec/query evidence.

Standalone TLS acceptance uses generated certificates, explicit client trust and
hostname verification, with the development plaintext override disabled. Cold
real-storage CLI recovery runs through this same TLS listener; memory-backed RPC
acceptance and plaintext loopback startup remain separate evidence lanes.

Native library acceptance also exercises application read selection against real
JJ transactions. This verifies that inspecting concurrent operations cannot
invoke the CLI's mutation/reconciliation behavior. The read surface remains a
service capability until its separate HTTP and graph integration slices.

Graph acceptance compares library results with native transaction histories,
including filtered edges, hidden predecessors, sibling operations and cache
rebuilds. Upstream types and transaction fixtures stay in the CLI/JJ boundary;
application services consume only owned graph requests and responses.

Native transaction fixtures also qualify service continuation cursors across
publication, checkpointing, revocation and expiry. These are library acceptance
tests; HTTP and browser pagination require their own later transport coverage.

Native content fixtures qualify all object variants, merged directory entries,
binary file ranges, revocation during streaming and unchanged manifest state.
These library fixtures complement CLI parity; media headers and safe rendering
still require HTTP/browser acceptance.

Directory cursor fixtures verify exact committed snapshots through later writes,
checkpointing and retries, alongside graph cursor cross-query rejection and
shared capacity. HTTP/browser integration remains separate acceptance.

Operation-diff fixtures compare complete view changes and native commit-parent
visibility, including abandonment, hidden predecessors and operation merges.
Parent-selection responses cannot trigger CLI-style reconciliation.

Structured content fixtures compare copy-aware file deltas and exact text hunks
with pinned native helpers. Conflict fixtures cover multiple sides, binary
content, delete/modify, file/tree and mode/copy conflicts without publication.

Native read acceptance also corrupts derived index links and segments, exercises
cache eviction and cold reconstruction, and checks that authorization and native
publication state are preserved through cache recovery.

Loopback RPC restart fixtures retain their listener reservation while a server
instance is stopped. Restart acceptance must not race another fixture for the
same ephemeral port or silently switch its endpoint.

Read-worker acceptance verifies prompt cancellation of pending graph/content
I/O, retention of worker permits during synchronous work, and continued use of
successfully prepared binary streams.

Native application-read acceptance includes oversized persisted merge terms,
reference conflicts and dense trees. Rejection at this read boundary must not
narrow ordinary CLI persistence or alter native hashes.

File-predicate acceptance distinguishes native log parity from resource-limit
rejection. Large downloads remain streamed under content budgets; ordinary CLI
commands retain upstream semantics.

Native lifecycle acceptance checks aggregate graph/content admission, including
pending worker cancellation and the lifetime of verified file handles. These
fixtures exercise owned native query interfaces; HTTP serving qualification is
recorded separately.

Native index-preparation fixtures cover cold-read admission, exact-operation
preparation during later publications, cache loss/rebuild and shared-worker
cancellation. These are native library tests, independent of browser task
status.

HTTP acceptance compares owned DTOs at exact operations, including binary media
and error boundaries. Existing authenticated native-service fixtures retain the
future shared-deployment branch; the local release serves protected HTTP V1 on
literal loopback and native RPC through its owner-only Unix socket.

The retained authenticated server/CLI fixture opens authenticated SSE, publishes
a native bookmark operation, resumes from the same event ID after server/storage
restart and cache deletion, and checks live stream termination on token
revocation. This qualifies deferred behavior; LocalTrusted cold acceptance uses
token-free SSE and Unix RPC.

The CLI-owned native lifecycle fixtures also exercise every HTTP read route for
generated contract conformance. An opt-in capture path receives only disposable
response bodies and route metadata, never request credentials. This fixture does
not alter CLI presentation or publication semantics.

Server configuration may provide web_root, resolved relative to its TOML file,
to serve a prebuilt browser bundle alongside the protected HTTP APIs. Build it
with pnpm web:build. A malformed or oversized bundle fails server startup
explicitly; no filesystem directory listing or repository content is served as a
browser asset.

CLI-owned native lifecycle fixtures compare internal typed command preparation
with the pinned native repository model, including descendant, bookmark,
workspace and evolution effects. The command preparation library does not add a
new CLI command or advertise HTTP mutation support by itself.

Native lifecycle tests also exercise the typed command service's provisional
preview, strict apply and principal-scoped durable status. Retry evidence checks
native result identity after later publications/restart; it does not advertise a
new CLI extension or imply browser mutation support.

Native lifecycle acceptance also exercises the typed HTTP command transport:
authenticated New/Describe/bookmark commands, strict snapshots, protected
history, durable replay and lost acknowledgements. These routes reuse the
service and do not change ordinary CLI spelling or its capability evidence.
LocalTrusted browser mutations reuse the same transport without bearer
credentials. Authenticated/shared browser finalization remains deferred under
M4.3.

Typed reconciliation acceptance compares native descendant rebases and recursive
operation-ancestor merges to pinned upstream behavior, including three current
heads and conflicted bookmarks. The application mutation is explicit and strict;
ordinary CLI head-reconciliation behavior remains unchanged.

Native abandon acceptance exercises both bookmark-retention and
descendant-content options, adjacent targets, merges, replacement workspace
commits, hidden targets, historical/evolution reads, protected policy and typed
HTTP retry behavior. It also preserves binary/mode/copy/symlink/submodule values
through native rewrites. Ordinary CLI commands remain covered by their existing
executable parity lanes.

Native lifecycle acceptance now exercises typed conflict term selection and
deletion through the service and HTTP API, including descendant/bookmark/logical
workspace updates. Ordinary jj resolve remains an upstream client command;
server typed commands never execute client editors or alter physical files.
Uploaded typed replacements have separate scoped-content acceptance below.

Typed upload/resolution acceptance covers the native service and HTTP transport;
ordinary client-side jj resolve continues to use upstream checkout semantics.
The server caps HTTP conflict uploads at the smaller of its configured native
blob limit and the 16 MiB default. Failed streaming staging does not create a
native operation or durable mutation receipt.

Native conflict-resolution acceptance also checks complete bounded reports for
the target and rewritten descendants, explicit empty remaining-path lists, and
legacy receipt decoding. This API contract adds no browser or CLI editor.

Native adapter-source acceptance binds the runner to repository read tokens,
checks committed native framing/identity, excludes staged objects, and verifies
revocation before further projection delivery. This test uses in-process native
stores; it does not claim search, UI or real storage deployment qualification.

RPC deadline/admission acceptance waits for a server-side authorization read to
prove the pending upload owns the permit before issuing the competing request. A
polling competitor must not win admission and accidentally reject the upload
under test. The original transport deadline and failure/release assertions
remain unchanged.

Native adapter acceptance reads complete metadata, trees and files through the
service projection reader, including virtual roots and non-file native entries.
These fixtures exercise pinned JJ codecs without moving upstream imports into
Tantivy, adapters or service contracts.

Native CLI integration fixtures also exercise authorized HTTP search over actual
native objects and local Tantivy generations. These tests require no installed
CLI replacement and make no browser feature changes.

Native integration fixtures verify configured search identity at runner startup
and same-position restart after source exclusions change. Failed rebuilds must
not expose an incompatible prior generation to newly authorized reads.

## Configured search lifecycle

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
`pnpm dev:restart` task cold-builds the UI and binaries, replaces only its
labelled disposable services, seeds a native repository, starts its LocalTrusted
search worker and prints the URL and repository ID with `Credentials: none`.
`pnpm dev:stop` removes only that owned stack.

The existing native administration RPC and CLI clients retain the selected
code-state version in snapshot preconditions. This prepares the command layer
for `jj agent enable` and later review commands without treating agent/review
WAL activity as native code movement. The user-facing collaboration command
family remains M9.4 work; this publication slice adds no hidden installer,
daemon or controller dispatch.
