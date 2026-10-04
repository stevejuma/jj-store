# CLI and server commands

The native lifecycle HTTP fixture additionally qualifies current application
content search and Thread directory metadata through the same authorized native
service. This transport acceptance remains independent of a live runner or
real-RustFS browser qualification.

Typed human-message commands accept an optional exact reply parent. Existing
unlinked message commands preserve their encoding and behavior.

The native capture worker consumes an explicit optional binding retained in the
captured spool entry. Binding-aware preparation validates returned intent;
unknown publication retains the exact prepared request across restart. Ordinary
owned hook/watch capture remains unlinked unless an explicit caller supplies a
binding. The local drain creates the exact BoundCapture request when a binding
is present and retains it across unknown outcomes. These worker primitives do
not add jj-acp launch/continuation or an automatic hook binding.

The capture spool validates the complete prepared Agent request, including any
explicit binding, instead of rebuilding only legacy fields. Existing unlinked
preparation fails closed on an unexpected binding. Explicit native bound enqueue
and drain are qualified separately from future runner entry points.

Explicit bound diagnostic-only capture accepts the validated source/projection
delta without requiring a session edit. The private spool retains its original
binding, prepared references and acceptance time across unknown outcomes and
restart, then completes through the same receipt check. No alternate unbound
request or synthetic session is substituted to unblock the queue.

The native lifecycle HTTP acceptance fixture qualifies the Thread/Reaction
transport and representative generated responses through isolated MemoryStore
authority. It does not establish real-RustFS or jj-acp execution qualification.

The native CLI uses the typed RPC client for Thread/Reaction commands, status,
signed pages and Thread detail. Native pages use opaque --cursor; local Git
pages use --after with --at-metadata. Mixing these profiles fails explicitly.
Both providers share the same bounded input admission and typed intent. Unknown
RPC outcomes return the original request ID without retrying publication. Native
code writes still require reducer 3; WorkStore homes retain their origin freeze.

Local `jj agent thread` exposes list/show/create, bounded typed apply/message
intents and durable status. `jj agent reaction` exposes exact-target query,
set/clear/resolve, custom definition/retirement, bounded library pages and
status. Typed target/value/observed documents retain complete immutable
identities; actors are filesystem-provider derived and explicit request IDs
survive process loss. Input reads stop at their byte limit instead of loading
unbounded files. Read and result JSON stays within four MiB. Continuations
require an explicit metadata commit and reject changed snapshots; reads do not
initialize metadata, actor state or the agent registry. Native serving and cold
local filesystem qualification remain distinct. These commands neither launch
nor continue jj-acp or another agent process.

M13 local agent commands resolve the admitted GitBackend without a native
connection. Status and session reads do not initialize companion metadata;
explicit initialization and supported enablement are metadata writes. Existing
registry, owned setup and sanitizer behavior is shared with native capture. The
private spool records either the original local typed capture request or the
native prepared delta before publication, rejects cross-profile delivery and
stops ordered progress on unknown outcomes. Local receipt and pinned
session-page output identify their metadata profile and snapshot explicitly.

The CLI-owned upstream boundary additionally qualifies direct local provenance
publication against exact GitBackend capsules and stock JJ/Git. This fixture
proves the service provider. Local provenance commands inspect
explicit-operation evidence, accept bounded typed association/checkpoint
documents and expose selected-history pages. Before publication a private
repository/actor-bound journal retains the exact record, expected version and
capsule bytes. Explicit retry uses that original request without reopening the
source operation store; a matching selected receipt is required before journal
removal. Automatic hook code association remains separately gated.

Native search acceptance compares selected-view commit visibility with retained
historical metadata, including invisible predecessors and pagination after
hidden-hit filtering. These fixtures do not introduce CLI search commands.

Search serving fixtures additionally cover signed continuations, freshness
waits, scope selection, request admission, cursor expiry/restart and current
repository authorization through native service sessions.

First custom commands: init, attach, status, capabilities, doctor, serve-local,
and the retained shared-deployment credential commands. The supported local
release path uses `jj-store init --local`, `run`, repository registration and
doctor without user credentials. Authenticated setup can initialize a private
installation registry, generate a manual token locally, and grant that same
credential independently to several repositories. External-provider and
search-worker qualification remain deferred. JSON is never paged. Custom
standalone no-command invocation prints help successfully; ordinary upstream jj
defaults stay intact. Match jj-nostr presentation semantics without inheriting
its storage or daemon installation policy. Secrets stay out of argv, output, and
checked-in config.

## Requirements

| ID          | Requirement                                                                                                                                                                                                                                                                                                                                       |
| ----------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| JST-CLI-001 | Custom commands MUST use jj store under upstream CliRunner; LocalTrusted MUST provide initialization, run, repository registration and diagnostics without user credentials, while authenticated mode MUST provide explicit registry and manual-token administration.                                                                             |
| JST-CLI-002 | Custom command families MUST provide authored Markdown long help, short help, keyword help, hints, JSON results, TTY paging, no-pager, color controls, completions, and build information.                                                                                                                                                        |
| JST-CLI-003 | Ordinary JJ commands MUST preserve upstream behavior; a tested capability matrix MUST report unsupported backend commands honestly.                                                                                                                                                                                                               |
| JST-CLI-004 | Development MUST use explicit build paths and MUST NOT replace installed jj or start unrelated services.                                                                                                                                                                                                                                          |
| JST-CLI-005 | Configuration format 2 MUST explicitly tag access and authority storage, reject every older format, and provide explicit bounded local or authenticated-registry initialization, verified catalog registration, help and completion commands; configuration migration, external-authority qualification, backup control and restore are deferred. |
| JST-CLI-006 | Authenticated setup MUST explicitly initialize private installation and catalog state, generate a manual bearer credential without persisting it, accept case-sensitive slash-separated repository aliases, and grant or revoke that credential independently through each repository's durable token policy.                                     |

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

After `store migrate-local` switches a physical workspace to its verified local
connection, ordinary native and ReviewStore commands use that socket even when
the former upstream endpoint accepts connections but does not respond. The
`sync reconcile` committed and unknown outcomes use the same JSON presentation
contract as other typed commands.

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

Agent capture uses a separate bounded journal in the shared jj-store-capture
runtime, reexported through the existing CLI module paths. The server can
consume the same runtime without a CLI dependency. Extraction preserves the
original sanitization policy, spool entries, retry identities and native/local
publication contracts; automatic worker enablement has separate M18.2 gates. It
keeps a compatibility lease on a user-restricted path outside checkouts,
serializes each mutation with a short-lived lock, and atomically persists a
validated sanitized batch before acknowledging reader progress, and then
atomically records the exact principal, request ID, publication ID and typed
AgentStore delta before publication. Restart validates pending entries; it
removes only unacknowledged atomic temporary files. Changed retries, corrupt,
symlink or unexpected entries and concurrent mutations fail closed. Bound and
unbound enqueue use the same mutation lock and queue admission limits. Native
and local drainers hold a separate exclusive lease across publication waits,
allowing independent hooks to enqueue while preventing competing drains. Capture
leases explicitly unlock before closing their handles, so a child's temporarily
inherited file description cannot prolong a released lease. This applies to
compatibility, mutation and drain leases; abrupt process loss still uses
operating-system lock recovery and retains durable pending intent. Completion
requires the matching prepared request/publication identity, so unprepared or
unknown-outcome captures remain recoverable. The journal is implemented as an
internal boundary.

Reader-owned capture adds optional generation metadata to private pending V1
entries and retains an owner-only device cursor journal in the same spool.
Original hook/unlinked pending entries omit the field and retain their bytes.
Reader progress follows sanitized pending durability, survives native/local
completion and is recovered from pending evidence before restart delivery.
Prepared requests and publication receipts remain unchanged. The journal holds
at most 4096 records of 8 KiB, rejects changed/regressive intent and stops new
reader admission when full without removing acknowledged progress.

Codex harness settings resolve explicit roots ahead of the supplied server-user
CODEX_HOME and default home conventions through the shared runtime. This pure
location resolution creates no state and accepts only qualified Codex versions.
Approved-root metadata scans resume bounded directory iterators and do not read
transcript bodies, admit sessions or start the capture worker. Configured Codex
startup uses these locations through the foreground server; other reader
profiles remain separately gated.

The shared runtime also exposes an opaque memory-only parked Codex reader.
Generic parking releases the source and physical leases while retaining partial
scans, prefix verification and the exact unacknowledged batch. Current server
admission, native Write and unchanged durable progress are separate
prerequisites for reuse; another owner's advanced cursor requires reopening at
that progress. Missing or regressive progress is unavailable. This capability
adds no CLI command or durable format and does not qualify automatic worker
enablement. The eventual supervisor must bound parked memory independently from
leases. Automatic supervisor parking retains only the channel reservation while
an exact raw batch is pending, within the existing bounded noncompact frames.
Fresh physical reacquisition rechecks current admission, exact registry/lock
identity and source qualification; failure or cancellation retains the raw frame
and channel. Successful acknowledgement followed by idle parking or shutdown
releases ownership. Generic parked-reader behavior is unchanged.

Fresh Codex activity may use the staged current-context server preparation
before first source enrollment. A genuine observed start and exact file snapshot
remain mandatory; current native directory and Write authority precede identity
allocation and reader leases. Old starts discovered by a newly armed probe
remain historical. Directory changes cannot lend one repository's body evidence
to another. This adds no CLI command, worker enablement or persisted eligibility
format; automatic activity renewal and bounded scheduling remain server gates.

Repository and operator credential provisioning use the same owned lease helper
to preserve exclusive creation while releasing the lock after the callback,
including failed callbacks. Credential storage and admission stay unchanged.

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
drift is reported and blocks replacement.

The M9.6 local qualification uses the same production PATH probe against the
installed executables, but binds the registry and setup previews to disposable
state and workspace paths. It accepts Codex 0.156.1 at `/opt/homebrew/bin/codex`
and OpenCode 1.18.30 at `/opt/homebrew/bin/opencode`. The installed Cursor agent
at `/Users/stevejuma/.local/bin/cursor-agent` reports `2026.09.26-dd393fe`,
remains `unsupported_version` against the qualified 3.22.7 contract and cannot
be enabled. Setup preview writes no hook files and the qualification makes no
model or network call.

`jj agent remove <codex|cursor|opencode> [--dry-run]` does not require the
harness executable or supported version to remain installed. It accepts only a
validated ownership record and target matching either the exact owned hash or
the already restored original hash. Removal atomically restores exact original
bytes or deletes a target created by setup, retires the ownership directory
through a recoverable tombstone and records disabled/not-installed state.
Changed bytes and symlinked paths fail closed without touching project content;
dry-run and repeated removal are non-mutating.

The installed `jj agent hook <codex|cursor|opencode>` command reads at most four
MiB plus one overflow byte from standard input and produces no normal output. It
requires the exact enabled registry entry, current owned setup hash and warmed
Kingfisher cache before capture. A fixed omission observation represents invalid
or oversized input without retaining source details. A bounded lock serializes
source-sequence allocation across hook processes; reservations are synchronized
before the sanitized batch is atomically enqueued, so interrupted attempts can
leave a gap but can never reuse a sequence. After durable enqueue the command
gives its recovery worker three seconds to drain captured or prepared entries
through the native AgentStore RPC. Transport failure, rejection, timeout or an
unknown outcome never fails the already successful hook capture and never
removes its spool entry; a later hook retries the oldest exact prepared request.

The native and local `jj agent drain` paths hold one shared drain lease per
spool. The separate mutation lock covers only queue reads and atomic mutations.
A publication wait cannot prevent another hook process from durably enqueueing;
a competing drain fails before contacting its publisher. Cancellation releases
drain ownership while retaining captured or exact prepared entries for retry.

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

`jj-store token generate` creates one manual bearer credential locally, prints
it once and persists nothing. `jj-store registry init --state-dir PATH` creates
the private installation identity and empty repository catalog required by an
authenticated server. `jj store init --name organisation/repository` reserves
the case-sensitive slash-separated alias before authoritative publication and
atomically exposes the repository plus its initial admin-token candidate only
after publication succeeds or is recovered.

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

The same lifecycle fixture covers EditFile preparation through the native
command boundary: one child change replaces or renames bounded text, retains
file mode and copy identity, moves the exact bookmark and leaves the selected
manifest unchanged until strict publication. It also rejects ambiguous targets,
invalid paths, oversized text and a replay against a bookmark that has already
moved. This remains application/HTTP command evidence and adds no custom CLI
spelling or physical checkout editor.

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

## Replication controls

For a native repository with one configured upstream, `jj sync status`, `now`,
`pause` and `resume` expose the server-owned worker without placing network work
in ordinary JJ commands. Status reads only local worker, checkpoint and search
state; it neither contacts the upstream nor reconciles operation heads. Pause
preserves local reads, mutations and durable peer progress, while now and resume
wake the worker and return without waiting for convergence.

`jj sync wait --timeout <duration>` captures finite issuer-bound local and peer
frontiers before waiting for them. The explicit timeout accepts `ms`, `s` or `m`
and is bounded from one millisecond through five minutes. `jj replica inspect`
reports the exact local, peer, push and pull frontiers, checkpoint revision and
search freshness. These controls return versioned structured results in JSON
mode and never return the configured credential or its resolved value.

`jj sync reconcile` remains a separate receipt-backed application mutation. No
status read, wait, import or background worker step invokes it implicitly.

The command guide documents the minimum activation sequence with explicit
development binary paths: initialize and validate a persistent LocalTrusted
service, migrate an existing authenticated workspace when needed, add one
repository/upstream configuration and its environment credential reference,
restart the server, then inspect and wake the worker or wait for a captured
frontier. Migration and ongoing replication configuration remain separate
operations, and neither rewrites physical working-copy files.

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

## WorkStore service and transports

Native jj project and jj task commands read and mutate WorkStore through the
local repository connection. Task/project commands require exact request IDs and
entity/project versions; they are independent of optional upstream connectivity.
For an ordinary Git-backed JJ repository, M13 dispatches the same project/task
commands directly to the LocalRepository service without a server connection.
Reads do not snapshot the checkout, reconcile operation heads or initialize
metadata. Local writes use stable request-derived entity IDs and shared domain
validation; their version-2 receipts/pages identify the Git metadata root.
Native commands retain their existing version-1 transport and output contracts.

Disposable development/browser qualification may explicitly reuse a separately
built pinned RustFS binary through `JJ_STORE_DEV_RUSTFS_RUNTIME`. The server
still checks runtime identity and owns isolated state and process shutdown. This
option does not install or replace a binary or alter a running deployment.

Development reset prepares the full representative fixture and search projection
on an owned temporary instance, then stops that instance. Restart timing
measures a fresh server and storage process opening the persisted fixture, and
stops when the server and existing search index are ready. Initial search
construction and representative sanitizer/review work are recorded as
`fixturePreparationMillis`. The cold runner retains the 30-second startup and
512-MiB resident-memory gates and reports actual measurements on rejection.
Search cache-loss rebuild timing remains a separate storage/search
qualification.

## Explicit native Git bridge

The existing CliRunner dispatch hook is the integration seam for the staged
[Git interoperability](git-interop.md) capability. Backend discovery opens the
workspace through JJ's loader without snapshotting or reconciling operations.
Ordinary Git repositories and creation commands retain upstream dispatch. Native
import/export remains unsupported; implemented stages alone appear in the
capability matrix. Aliases and global arguments are parsed upstream.

Native Git bindings live in private device-local state outside checkouts. An
explicit remote add/set-url approves one HTTPS/SSH or canonical local endpoint;
replicated logical names grant no transport permission. A binding fingerprint
changes with endpoint approval and prevents lease reuse across destinations.
Cache attempts are private, exclusively locked and disposable, with bounded Git
subprocess output, duration and disk use. Hooks and repository-provided helpers
remain disabled. Fetch uses one explicit scope and the existing native
transaction/rebase/workspace finish path with a pending typed Git delta.
Unsupported parsed flags and historical/non-integrating mutation contexts fail
before transport. Ordinary Git-backed dispatch remains upstream-owned.

Selected native push freezes bookmark targets and exact destination leases,
retains representations and intent before execution, then records protocol
outcomes and applies remote bookkeeping without moving local bookmarks. Dry-run
does not publish an intent or bridge-induced code. `jj store git status` reads
durable attempts; `jj store git recover` inspects the original approved endpoint
and repairs local bookkeeping without retransmitting. Capability gates remain
explicit for conflicts, tag publication, sign-on-push and unsupported push
options. Bookmark selectors and atomic multi-ref transport are qualified below.

Native Git code repositories use reducer-2 origin negotiation for data-only
facts. Device bindings are still explicit and local. The bridge capability
reports GB6 qualification on macOS ARM64 with JJ 0.45.1, gix 0.87.1 and Git
2.55.0 under `jj-git-sha1-0.45.1-v1`. Native fact replication is data-only;
ordinary upstream dispatch is unchanged. Linux/Windows and external forge
deployment qualification remain deferred.

Native Git bookmark selection supports patterns, --revision, --all, --tracked,
--deleted, --change and --named under the bookmark-only capability profile.
Generated bookmark preparation is a separate accepted native operation before
intent preparation, and dry-run performs no such publication. Multiple selected
updates use atomic transport with exact leases for every full ref. Remote rename
and removal change owned alias/tracking metadata while preserving durable facts.
Unsupported tag publication, signing/conflict profiles and push options still
reject before external writes; ordinary backend command semantics are unchanged.

The Git transport supervisor uses an internal executable entry point before
CliRunner and native credential loading. It accepts invocation-owned argv and a
lifetime pipe; replicated Git facts cannot launch it. Hard cancellation closes
the pipe and terminates the owned Git group without changing native evidence.

Git apply/recovery reads the frozen native operation's remote-ref base and
merges its result into current tracking state, including renamed aliases. An
attempt cache starts with owner-only directory permissions and remains safely
removable after a hard-killed invocation.

Native Git honors explicit absolute device askpass/SSH executable and CA-file
approvals. `JJ_STORE_GIT_CREDENTIAL_HELPER` selects a supported OS helper or
`none`. Configuration and TLS-verification environment injection stays disabled;
helpers do not inherit native bearer or server storage credentials.

The GB6 cold fixture uses production CLI/server binaries, persistent embedded
RustFS, actual Git refs and a freshly built served browser. It retains a native
review anchor and AgentStore receipt through cache deletion, server restart, Git
publication and external merge import. Git hosting outages leave ordinary local
new/describe usable; no Git checkout or background mirror is installed.

The Projects hub is browser presentation over the existing local-service
WorkStore model. The registered HTTP capture housed in the CLI lifecycle suite
also verifies browse summaries and generated response contracts. Native task
creation still requires the local service and does not require an upstream.
Server-free storage remains deferred.

## Local settings workspace discovery

The foreground CLI server composes an owned device-workspace verifier for
LocalTrusted settings. It uses JJ workspace discovery and the existing native
connection/profile validation without loading repository state, snapshotting
files or reconciling heads. Browser links must target that service's configured
Unix socket and the selected repository. Authenticated serving and unsupported
Git-backed serving do not advertise workspace management. Development tests use
explicit binaries and isolated state; existing installations are not restarted.

## Native folder selection for local settings

Local server composition supplies the separate owned folder-picker interface on
macOS using `/usr/bin/osascript` with a fixed folder-dialog script. Execution is
asynchronous, single flight, limited to 4096 path bytes and 25 seconds, and
cleans up children on timeout or caller cancellation. Other platforms advertise
no picker; missing helpers or headless sessions return unavailable and leave
manual path entry usable. No installed binary or service is changed by picking.

The LocalTrusted admin server composes a browser Sync manager over the same
replication controls used by the CLI/RPC. It starts no additional worker and
modifies no upstream configuration. Paused state remains process-local.

Local Search management is composed only when the foreground CLI supplies its
startup-selected configuration path and LocalTrusted Admin access. Revision
checks cover the bounded file bytes; atomic replacement preserves unrelated TOML
values, credential references, source extraction limits and worker timing.
Disabled entries retain their desired configuration. Save alone leaves the
running worker unchanged; Apply changes only its repository projection.

The development demo retains its native local checkout and additionally seeds a
native server-only repository with no linked physical folder and an ordinary
colocated Git-backed JJ repository. A temporary server-only seed checkout is
forgotten and removed after acknowledged native publication. A private atomic
profile marker retains distinct identities across restart. The original three
projects remain in the native home and link one profile each. On upgrade,
unchanged sample project links and task targets may migrate using exact current
versions; edited records and retired links remain untouched. Local Git metadata
is initialized through the owned CLI, not written by the harness. Its project
link remains separately inaccessible in the browser until M13.4 serving passes.

`pnpm cli:install` builds locked release binaries and installs `jj-store` plus
`jj-store-jj` into `~/.local/bin` or an explicit `--bin-dir`. The custom JJ tool
uses a distinct name to preserve the existing `jj`. `--debug --no-build` selects
already-built development artifacts. Missing artifacts fail preflight; file
replacement uses a temporary executable and rename, including when the old
destination is a symlink. Installation does not start services or alter PATH.

The representative agent seed binds every setup, capture and remove subprocess
to its fixture working directory. The explicit `-R` argument alone cannot select
the installation target. Source checkout configuration remains untouched.

## Local browser composition

Explicit local-folder registration is composed through a read-only JJ verifier.
It supports canonical colocated and linked Git-backed JJ workspaces, never reads
executable repository settings and never snapshots checkout files. Server-owned
private request journals retain initialization receipts before discovery. The
filesystem-only serving mode and local browser endpoints are separately
qualified under M13.4.

The CLI's isolated ordinary-JJ fixtures qualify operation-pinned server reads
against the explicit development binary. They retain dirty checkout files and
verify graph, binary, conflict, root and multi-head behavior without invoking
repository-configured programs or installing the CLI. Provider tests do not
qualify filesystem server startup or browser integration.

`jj-store init --local --storage filesystem --state-dir <new directory>` creates
private installation/catalog/operator configuration without RustFS credentials,
volume or RPC socket. The default remains native embedded storage. Filesystem
mode requires literal-loopback LocalTrusted administration and rejects native
backup/search/replication configuration. Native routes report unsupported
authority rather than constructing a placeholder object store.

Local browser WorkStore writes use the same shared-JJ device actor as
`jj project`, `jj task` and `jj task comment`. The CLI can look up browser
requests and returns their original committed receipts, independently of the
code operation or later metadata publications.

Local browser AgentStore reads present only records already selected through
local CLI capture/provenance publication. Opening the app does not enable
capture, install hooks or create observations; independently pinned browser
selections can remain older than subsequent CLI captures until explicit refresh.

Local browser admission retains a durably proposed repository identity before
companion initialization. Existing CLI actor and initialization receipts are
reused after restart; explicit reconnect revalidates a moved folder without
changing portable metadata or installing hooks.

Filesystem browser startup admits concurrent snapshot, code and WorkStore reads
through a bounded worker queue with the configured request deadline. Ordinary
parallel page loading does not require a native service. Cold isolated CLI/HTTP
fixtures qualify this startup boundary separately from provider-only tests.

The local browser physical-file Save is independent from CLI code commands. CLI
`--ignore-working-copy` reads must observe unchanged operations, bookmarks and
companion selections after Save; later explicit foreground Snapshot remains the
only way those physical edits enter recorded code state.

## Browser history language qualification

Browser history readers expose standard aliases pinned to JJ 0.45.1 without
loading repository or device revset configuration. This does not change CLI
configuration precedence. The local browser regression compares the browser's
standard aliases against the pinned JJ executable, then poisons repository alias
configuration to prove reader isolation. Native lifecycle regressions qualify
filtered operation pagination, traversal limits and read-only behavior.

## Supervised desktop launch

The additive desktop run flags use the existing foreground composition and
format-2 installation. They require LocalTrusted embedded storage, literal
loopback and a pipe on stdin. Launch-only bundle-path and origin overrides never
persist or alter credentials. Owner EOF enters the same bounded shutdown as
interrupt/termination. See [desktop](desktop.md).

Native browser qualification MUST verify that authenticated query metadata omits
host identity hints. Registered-folder identity fallback remains an adapter
read; it MUST NOT invoke installed JJ/Git commands, migrate configuration or
alter recorded/physical repository state. CLI query arguments continue to supply
explicit author email and retain existing standard-alias isolation.

The registered local-management HTTP capture includes explicit Sync barrier
capture, reached/pending waits, invalid targets and LocalTrusted-only admission.
These browser controls preserve the existing CLI finite-frontier semantics.

The local server composes native Storage management only for LocalTrusted
administrators. A bounded private journal under the configured process state
directory persists captures and reports with atomic owner-only writes and fsync.
GET does not create it. Server-owned work survives request cancellation; after
restart checks report interrupted and checkpoints resolve their original target
from selected authority. No new CLI executable installation or service
registration is required.

Foreground external native serving transfers the exact process state-lock file
into authoritative-storage ownership before capture can retain it. Prepared
server drop and run-future cancellation retain the lock until the final capture
storage lifetime releases; completed ordinary shutdown releases it after storage
stop. Late or duplicate lock attachment fails closed. Filesystem-only serving
retains its independent process lock and remains unavailable for native capture.
Configured Codex startup retains this ownership through ordered capture drain.

Foreground capture uses a prepared slot with disabled, unavailable and ready
states. Disabled selection performs no capture environment/state I/O. Enabled
resource selection validates profiles and privacy locations. Whole-server
preparation, after owning native storage and the state lock, exclusively creates
an absent private capture root with control and import journals and syncs it.
Existing roots and internal resource-only preparation never repair or initialize
missing policy. It then opens control state before installation-bound sources
and the sanitized spool. The ready slot pins control generation and physical
directory/lock plus installation identity, revalidates them at runtime handoff,
and applies the latest selected pause policy before the runtime can dispatch.
Scanner creation waits for admitted body sanitization. Unavailable capture
preparation preserves ordinary serving. The exact ready slot starts with
retained storage ownership and drains before storage shutdown. The ordinary
configured foreground path supports Codex CLI/Desktop; unsupported readers
retain their startup guard.

Settled queued discovery/delivery task loss closes that owner, records one
unavailable observation and invalidates delivery queue freshness. Pause/resume
cannot revive a lost global lane. Explicit replacement must reverify current
admitted source and Write authority and preserve exact pending bytes/cursors.
This adds an internal diagnostic qualification. Typed installation management
uses the qualified retained runtime; configured enablement and real
storage/harness fault gates remain pending.

## M18 automatic capture configuration and controls

The owned runtime accepts coalesced discovery hints without storage ownership or
policy changes. Idle root cooldowns may be advanced; an active bounded scan
keeps its progress and schedules one fresh pass. Pause/stop discard outstanding
hints, and lost lanes cannot restart. Cached root availability and
degraded-source counts add no directory/Write authorization or activity proof.
Installation HTTP rescan binds the latest device policy generation/revision
under the existing current Admin/configuration/physical binding gate. Typed CLI
import and inspection use retained workers; browser source/import presentation
remains pending. Cached runtime diagnostics group qualified profiles and
declared product versions with root/source counts, exposing no paths, session
keys or import selectors. Current Codex group capacity is two, independent of
configured source/reader limits.

`jj agent capture --server-url <url> status|pause|resume|rescan` manages one
explicit LocalTrusted installation through its existing local Admin HTTP API.
This profile works outside a JJ workspace and MUST dispatch before opening any
repository or integration registry. It rejects `--state-dir`; status and control
MUST NOT create device state, install hooks, inspect transcripts or snapshot a
working copy. The URL MUST be credential-free HTTP with a literal loopback
address and no non-root path, query or fragment. No implicit endpoint selection,
proxy, redirect, credential lookup or automatic retry is permitted.

Pause/resume require explicit nonzero `--request-id`, `--generation` and
`--expected-revision`; rescan requires the same generation/revision without a
request identity. Generation is exactly 32 lowercase nonzero hex digits. The CLI
forwards the typed version-1 command once without prefetching/replacing policy
or selecting an actor. JSON output retains the shared typed result; text
distinguishes the original receipt, latest policy and runtime request
acceptance. A durable receipt is not completed runtime pause or drain. A control
acknowledgement MUST bind the original request identity, generation, expected
revision and action, with the next receipt revision and latest selected policy
in the same generation at that revision or later. Rescan acknowledgements bind
the exact selected generation/revision. A mismatched acknowledgement is an
unknown outcome, never success or permission to replace the original command.

The same explicit installation profile adds
`sources --limit <1..100> [--cursor <token>]`. It reads cached per-source
diagnostics and runtime-scoped selectors without opening a workspace or
integration registry. Cursor input and response selectors are bounded typed
values; a stale runtime generation requires refresh from the first page. JSON
preserves the source page; text labels owned reader phases and cached
observations. Import is a separate explicit command with independent native
Write and retained source admission.

The explicit installation command adds
`import --source <selector> --generation <control-generation> --expected-revision <revision> --request-id <request>`.
It forwards only the versioned command, without policy prefetch or automatic new
identity. Exact acknowledgement binds every original input. Acceptance is
durable intent, not successful transcript publication. A retry keeps the
original selector after restart; the server resolves selected requests before
applying new-selection stale rules. Changed reuse or a mismatched
acknowledgement withholds success. New-request reselection cannot rewrite an
unknown or accepted request. Receipt/progress inspection remains local Admin
data. Dispatch uses retained bounded source workers. The command waits for
durable acceptance separately from volatile scheduling, preserving its original
inputs when an outcome is unknown. `import-status --request-id <request>`
forwards one body-free inspection GET; it requires neither a repository nor a
state directory. JSON retains the shared typed receipt/progress. Text
distinguishes accepted history publication from continuing tail phase. Exact
accepted replay remains inspectable while paused/stopped and after transcript
removal. A mismatched receipt, unsupported version or impossible completed byte
count withholds CLI success. Neither command initializes private state, starts
an owner or retries HTTP automatically. The private server journal primitive now
retains exact version-1 import intent and source binding across reopen and
process exit. It shares synchronized private-file mechanics with pause controls
while retaining its separate 2-MiB layout and receipt order. This primitive does
not expose a command, enroll a source or authorize publication. Control layout
and policy semantics remain unchanged.

The server-side import admission guard retains manager/control coordination and
rechecks exact original intent, current LocalTrusted Admin policy and the live
runtime. It is a library primitive; public import and owned scheduling remain
separate gates. No read or guard can initialize missing private state, enable
capture or create a runtime owner.

The library's owned import settlement retains the exact control and import
selection descriptors across native admission awaits. It preserves original
receipt replay and postselection unknown outcomes; it creates neither a command
surface nor an ingestion owner. Native Write, exact source preparation and
bounded frame/storage ownership remain separate requirements. Existing control
and import formats and synchronous callbacks retain their contracts.

The native source-import library now resolves the actual retained runtime
target, prepares a bounded accepted prefix and independently admits native Write
before selecting intent and enrolling its common source lease. Replay uses the
original binding before stale selector rules and opens existing durable spool
progress. Imported pipelines retain exact receipt renewal through ingestion,
publication recovery and raw-channel parking; current consent and prefix checks
precede body work and durable enqueue. Their private runtime target view exposes
no binding in public observation pages and grants neither worker/frame budgets
nor storage ownership. Retained supervisor scheduling is qualified separately.
Automatic JSONL capture now retains receipt-backed device publication evidence
separate from reader acknowledgements. Public import progress and typed HTTP/CLI
commands and configured Codex startup use these retained resources.

The prepared foreground composition may attach one supervised accepted-import
recovery lane when management binds existing import metadata. On a new runtime,
it offers exact original accepted commands to uniquely matching retained source
slots, without another CLI/HTTP command. Original receipt/prefix, current Admin,
configuration, private binding and native Write remain required; changed
physical files cannot inherit consent. Durable pause is selected before dispatch
and keeps recovery idle until resume. Frequent progress inspection cannot starve
owned policy acquisition, ingestion or publication. Recovery shares the
supervisor's cancellation, deadlines and ordered storage lifetime; stopped/lost
owners cannot be recreated. Existing missing import state is never initialized.
This optional retained composition is shared by configured Codex startup; other
readers, browser management and real harness release remain separate gates.

Original accepted import receipts acquire a separate bounded body-free reader
association under retained original-source/native admission and the held import
selection gate. The additive private version-1 catalog records receipt ordinal,
source ID, initial numeric generation, physical/header digest and
retained-prefix verification without changing the accepted receipt. Exact
retry/reopen retain the association; later numeric continuation cannot retarget
it. Receipt-only catalogs remain readable and explicitly unqualified until
renewed admission reconstructs the link. Bounded journal lookup performs no
transcript work and grants no publication authority. Public progress uses this
private association without exposing its digests or treating it as publication
authority.

Responses MUST be bounded to 64 KiB and decoded through the exact public
response type. Unknown versions, malformed/oversized bodies, redirects,
transport failures and failed command acknowledgements withhold success and
retain the original retry identity. Diagnostics expose only constant admitted
error descriptions, never arbitrary server bodies, URLs or local paths. The
server independently revalidates Admin/configuration/installation/policy on each
call. Controls cannot enable configuration, initialize a journal, replace a
runtime owner, import history or grant native publication authority. Implicit
workspace-to-installation endpoint discovery and browser controls remain
separate qualification work.

The shared native spool has a bounded partition drain for one explicit
repository, integration and source, with one through 256 batches per call. Other
native/local partitions remain intact; unknown outcomes retain exact prepared
intent, and the supervisor owns fair scheduling and authorization. Existing CLI
drains keep their whole-spool repository/native-only preconditions. The
unchanged report counts the entire remaining shared queue.

The configured automatic capture actor uses the existing installation principal
derivation with a fixed worker purpose. Disabled authority preparation is
side-effect free and skips the identity file; enabled preparation requires the
validated LocalTrusted native writable profile. In-process capture publishers
bind that principal through the native service only after current directory,
socket and catalog admission, and reauthorize before preparation/publication.
The guarded server publisher repeats these checks with a fresh native Write
request for every preparation and publication, including durable retries; it
rejects foreign actors and returns committed receipts without a post-commit
device-admission failure. This capability remains distinct from supervised
startup and typed controls.

Codex automatic source acquisition takes a current activity snapshot and checks
the activity event's reconstructed directory through current native workspace,
socket and repository-catalog admission before taking source/physical leases.
The directory proof is repeated after acquisition and remains separate from
publication authorization. Existing-registry lookup is read-only and generic
source registration grants no automatic historical eligibility; a distinct
native-session admission digest supports offline-tail recovery. These typed
composition boundaries do not grant configuration enablement.

An explicit admitted Codex source driver composes current activity/directory and
native Write admission, source leases, latest durable source cursor, bounded
reading, transcript sanitation, spool acknowledgement and one-batch guarded
publication. It resumes admitted offline tails, refuses unadmitted history and
recovers each queued Codex range from its exact qualified prefix before new
reading. Recovery publishes one exact batch through current admission and Write
authorization; another queued partition cannot inherit that proof. Changed or
unavailable source evidence retains pending intent. Only constant omission
batches may use a separately authorized reconstructed matching anchor. Source
steps and privacy-root configuration have no raw-path/body diagnostic encoding.
This composition remains separate from supervised automatic startup and typed
controls.

Bounded admitted Codex resume preparation uses read-only retained
channel/session lookup and current complete-record directory reconstruction
before taking reader leases or enrolling rotated aliases. It does not require
the original header folder to remain linked when the verified current folder is
linked and writable. Every acquisition rechecks current native Write, file/root
and channel bindings. It does not acknowledge source progress, import unadmitted
history or authorize queued/body publication through a fallback anchor.
Scheduling and automatic server startup remain separately gated.

Native publication scheduling rotates through exact pending partition heads
using only a memory-resident turn cursor. Selection advances the turn even when
delivery is withheld or unknown, while retained prepared bytes remain unchanged.
Exact delivery rechecks the live same-partition order under its drain lease; a
newly enqueued earlier batch withholds before any publisher call. Local prepared
intent excludes its complete partition from native scheduling. These scheduling
hints supply neither device proof nor native Write authorization. Independent
ingress and supervised server composition remain separate gates.

The explicit Codex pipeline consumes the conservative source driver and offers
separate bounded ingress and exact selected-entry delivery. Sanitized durable
ingress proceeds past old withheld/prepared/unknown publication, with
independent current native admission and Write checks for every new batch.
Delivery recovers the selected entry's own exact range and retains
same-partition order. Old publication cannot rewind the reader or its current
ingress anchor. Spool refusal keeps the exact unacknowledged batch; restart
retains both the latest sanitized cursor and unresolved intent. Temporal proof
scheduling, blocking-I/O ownership and automatic server startup remain
separately gated.

An explicit pipeline-step owner keeps its leased source, private spool and
current authority context outside ordinary runtime execution during synchronous
I/O. It dispatches one bounded step, retains overdue/draining work across
deadlines and pause, and refuses concurrent replacement. Cancellation/deadlines
preserve exact pending intent; current native Write remains mandatory after
resume. Shutdown waits at most five seconds and retains an undrained owner on
timeout. Known late results retain their receipt meaning. This internal owner
closes on task loss. Supervisor resume keeps that source closed while healthy
lanes may continue; explicit worker replacement must renew current source and
repository admission before recovering the unread tail. Durable queued bytes and
cursors survive loss, revocation and pause/resume. The internal owner does not
enable automatic capture or replace the separately authorized typed management
controls and full server lifecycle gates.

An explicit configured Codex supervisor joins root discovery, genuine observed
activity, staged current-directory admission, bounded parked ingress and exact
native delivery. Its memory-only controller exposes bounded status and internal
pause/resume/shutdown, retains independently draining owners and does not
perform source I/O during disabled preparation. This API does not remove the
configured enablement requirements or qualify authenticated management, other
readers, pressure/fault coverage or cold harness/server gates.

An explicit lifecycle driver owns a prepared Codex supervisor and schedules its
lanes independently of callers. Internal pause/resume preserves the same owner;
stop is terminal. Shutdown timeout retains its draining task for a later wait,
and drop requests stop without aborting blocking source/publication owners.
Storage teardown requires completed drain. Explicit storage-backed composition
binds an opaque lifetime before first dispatch and retains it in discovery,
source and delivery blocking owners and their unconsumed completions. Storage
shutdown waits for those lifetimes before stopping embedded storage or releasing
the installation lock; dropping storage or cancelling that wait preserves the
same ownership. The lifetime grants no publication authority. This memory-only
capability adds no CLI command or configured server enablement; those gates
remain pending.

The explicit runtime's cloneable status/pause/resume handle owns only its
body-free observation and command channels. It cannot start capture, authorize
publication, retain storage or wait for drain. Control acceptance requests a
transition; its observed phase establishes whether pause/drain settled. Runtime
drop stops all handles even while they remain readable during retained drain.
Lost nonterminal observation producers report unavailable, and terminal stop
rejects pause/resume. This capability does not implement authenticated commands,
durable pause or configured enablement.

The existing runtime constructor preserves a settled supervisor pause at handoff
instead of dispatching into a paused owner and stopping on that refusal. No
source is discovered, enrolled or published until explicit resume. Activity
present before resumed arming remains historical; fresh observed activity and
current admission are still required for automatic backfill. No new startup
mode, configuration field or durable pause journal is introduced.

The explicit capture-control journal records installation/generation-bound pause
policy and exact actor/request receipts in private device state outside
checkouts. Its initial bounded profile is 4096 receipts and 1 MiB. Record
selection uses a pinned private directory/lock, bounded validated reads,
synchronized temporary replacement and directory sync. Old receipt replay
returns the latest policy without reversing later changes; missing or foreign
state never initializes on ordinary open. Post-replacement I/O failure is an
unknown outcome requiring the original request. This Rust capability grants no
authentication, runtime action, native mutation, import or startup override; its
management consumer requires separate composition. The guarded foreground
factory consumes this journal with read-only policy selection at preparation and
again at handoff; it accepts legitimate later pause/resume selections but
refuses changed generation, installation or physical binding. Explicit policy
initialization is separate from ordinary factory preparation; no startup mode or
configuration override is introduced.

The foreground local native Admin profile may advertise installation capture
management when an explicit configuration path is available. Its manager derives
the transport actor from the installation, revalidates current configuration and
identity on each request and retains one selected journal generation and runtime
control cell. Startup binding shares control channels only, not task or storage
ownership. The manager's owned bounded blocking operation selects/replays device
policy and applies its latest state under one coordination gate, surviving
request cancellation. Durable selection and runtime request acceptance are
reported separately. Missing or closed runtime controls cannot start another
owner; accepted policy remains available for restart. Other readers retain their
startup guard. The explicit local HTTP CLI profile is qualified separately;
source import and browser coverage presentation remain composition work.

Discovery and delivery task panic retains that storage lifetime and process lock
until the exact running state is destroyed, including after a cancelled drain
wait or dropped controller. Settled loss closes the lane. Explicit replacement
must renew current admission to recover an unknown native receipt using its
original Prepared request without changing queue bytes, cursor or committed
evidence. This fault boundary adds no public control or startup override.

Delivery rechecks exact source qualification, private source state and spool
binding after directory admission. A root/state replacement during blocked
admission withholds the unchanged queue and cursor. Restored physical state
requires fresh qualification and current admission before explicit recovery. An
initialized source registry whose catalog disappears stays unavailable rather
than allocating a replacement source catalog.

Automatic Codex readers recover each qualified physical/header cursor separately
from channel-wide generation and sequence floors. Reopening unchanged archived
and current files cannot replay acknowledged offsets. Delayed old-file appends
retain their verified prefix and allocate a continuation above any newer channel
generation. Refresh bounds after lease acquisition and before polling;
atomically refuse stale automatic sanitized admission. Progress lookup preserves
typed capacity refusals from independent spool growth before polling or
acknowledgement. Conflicting exact parked raw batches remain withheld unchanged.
Automatic raw-channel reservations prevent competing physical readers from
consuming parked allocations across pause and capacity relief. Foreign registry
bindings or replaced locks cannot transfer a reservation. Existing generic APIs,
durable intent and receipts retain their interpretation; broader allocation,
rotation and task-loss races remain a separate qualification gate.

The explicit Codex supervisor reclaims a compact tracking slot after an owned
step confirms its qualified physical/header binding is unavailable or changed.
It preserves durable source identity/progress and exact pending bytes. Busy,
draining, lost and noncompact owners cannot be evicted to admit a replacement.
Rediscovery qualifies the current physical/header candidate independently; a new
session needs a fresh observed start after arming. Retired memory fingerprints
and prior capacity-refusal hints are cleared without changing the configured
tracking bound. This does not enable guarded process startup or qualify the full
rotation, retirement and fault matrix.

An unadmitted Codex probe that definitively detects a changed prefix or
truncation rearms on a later reserved step at the then-current file length.
Already present starts remain historical; fresh observed activity is required
before any admission or sanitized backfill. Retryable metadata/header/I/O errors
retain the original probe, while admitted readers and durable queues keep their
existing exact recovery rules. Internal pause/resume preserves these boundaries
without enlarging tracking or resident limits. This does not enable the guarded
process startup path.

A retained parked session can stage current-directory admission when its old
ingress anchor is revoked. Bounded context preparation precedes fresh native
Write and reader leases, preserves exact backpressure/prefix state and changes
the anchor only after successful handoff. Old body/queued evidence cannot borrow
the new directory's authority. This explicit path and its supervisor composition
do not enable server startup or client checkpoints.

Configured spool refusal retains exact unacknowledged source frames within the
resident bound. Explicit supervisor restart with a larger capacity recovers only
admitted native sessions; unadmitted historical starts still need fresh observed
activity. Status counts released stopped owners as zero resident/occupied
reservations while retaining draining owners' reservations. This pressure and
explicit restart qualification does not enable process startup or qualify a full
durable queue, rotation, faults or cold whole-server serving.

Exact prepared requests retain their bytes, actor and receipt identities when
they fill the durable queue or their repository is revoked. Rejected later
ingress cannot advance its cursor. Explicit worker restart retains that intent;
restored original admission permits exact receipt resolution and frees capacity
for independently admitted later repository batches. Resolving a lost native
acknowledgement cannot reauthor AgentStore roots. Captured-entry preparation
growth/headroom and full process lifecycle remain separate qualification gates.

The current strict byte-limit profile withholds native publication when a
Captured-to-Prepared replacement cannot fit, retaining the captured batch and
cursor until explicit configured capacity relief. Automatic startup also
requires bounded preparation headroom for newly admitted captures and honest
management diagnostics for legacy queues; neither may evade the hard byte bound
or replace exact accepted intent. Newly admitted automatic reader captures now
reserve physical space for the larger native or local prepared envelope using
trailing JSON whitespace; independent hook writers count that same space. A
matching native runtime request may contain all 4096 permitted edits, while a
local request duplicates its exact sanitized payload and optional binding.
Reservation failure cannot acknowledge source bytes. This is conservative
capacity admission, so small configured queues may admit no automatic batch. It
does not author dependent requests early, change request/receipt identity or
increase the configured limit. Legacy compact entries and explicit hook
admission keep their existing byte-bound profile. Management diagnostics and
full startup/pressure/fault qualification remain pending.

Admission and preparation byte-limit refusals carry typed, body-free configured
and required physical byte totals through admitted ingress and exact native
delivery. Internal Codex supervisor status reports the maximum last settled
ingress requirement, the number of affected source owners and the last settled
delivery requirement. Successful steps clear their observations; pause retains
them. These cached observations neither inspect the filesystem during status nor
authorize a capacity increase, discard accepted evidence or guarantee current
queue pressure. Compact spool preparation overflow can therefore name exact
explicit relief without adding legacy AgentStore support. Explicit
capacity-relief controls and UI presentation remain pending.

The M18 profile in [collaboration](collaboration.md) owns agent_capture server
configuration, LocalTrusted write admission, explicitly selected versioned
harness roots and supervised capture. Omitted configuration remains disabled.
Settings apply at startup; pause/resume cannot enable an unconfigured source.
The strict format-2 agent_capture table accepts enabled, max_readers,
rescan_seconds, max_sources, max_pending_bytes and a harnesses array. Each entry
requires profile and product_version with optional roots or the read-only API
endpoint. Checks resolve relative roots against the TOML directory, validate
limits and require writable LocalTrusted native serving for enablement. They
create no spool, rule cache, discovery registry or source files. Configured
Codex startup initializes only absent private capture state under the server's
native storage/state-lock lifetime. Existing damaged state degrades capture
without repair; other reader profiles fail before creating server state. Product
surface and live-edit qualification remain separate gates. Typed jj agent
capture status/pause/resume/rescan/import controls use the same local service
boundary and bounded diagnostics. Real hook/file/API qualification is distinct
from detection/setup preview and from automatic client checkpoints.
Implementation and release gates live in [milestones](milestones.md).

The CLI-supplied capture workspace verifier resolves an existing session
directory to its nearest physical native JJ workspace within 64 ancestors. It
checks this installation's exact socket and preserves a device binding digest
over physical identities and validated connection/profile bytes. Inner Git or
invalid JJ boundaries and `.jj` metadata paths fail closed. It opens no working
copy transaction, snapshots no files and changes no operation heads. This
capability is independent of explicit folder-registration root inspection.

Automatic Codex batch admission uses the batch's reconstructed directory rather
than reusing the session header's initial cwd. Each changed turn context goes
through the same native workspace/socket and current discovery-catalog checks.
An unavailable context or a repository removed from discovery withholds
admission. Revalidation remains required before publication, alongside the
authorized service's independent current write policy. This adapter does not
start a watcher, import inactive history or checkpoint a client working copy.

The supported AgentStore profile uses native AgentRootV1, local descriptor
version 2 and schema-2 transcript append tails. Legacy local descriptors and
forward transcript pages have no compatibility or migration path. Native capture
requires an already selected current Agent root; a missing root fails closed.
Durable spool retries and receipts keep their existing exact-intent guarantees.

### OpenCode API history preparation

The retained library can prepare an exact API message-prefix import boundary
without source enrollment, private intent writes or publication. It
independently verifies selected history and preserves accepted boundaries across
appended messages. Each step reads at most one existing bounded API page and
releases raw records. Empty history has zero logical messages and zero byte
offset; API byte positions MUST NOT be invented. Exact
endpoint/session/directory/version and selected-prefix checks govern replay.
This boundary alone does not qualify public selectors, import commands, worker
delivery or configured startup.

### Retained OpenCode supervision boundary

Explicit library-level capture supervision admits the qualified read-only
OpenCode API alongside Codex file sources. It uses the same reader/source
limits, selector catalog, pause/resume/rescan and storage drain ownership. API
source observations are body-free and do not authorize filesystem history
imports. API delivery is independent of source reading and retains exact
sanitized entry proofs and existing leases. Ordinary process startup remains
withheld pending management/import/export, identity and real-harness
qualification; library fixture acceptance cannot bypass that gate. No harness
process is launched and no existing service is restarted.

Approved API busy/retry/idle observations now advance a typed private composite
reader checkpoint independently of message count. Empty admitted sessions retain
zero-message progress. Reopen reconstructs historical constant status evidence
and each message's original lifecycle/generation before allocating new status or
tail records. Current status cannot rewrite retained publication requests. The
source follows observed API classes only; exports and message timestamps have no
lifecycle authority.

The retained API worker now discovers across verified repository/workspace
directories rather than relying on its default instance status map. Bounded
project directories supply ordinary candidates; global/non-Git projects use the
pinned bounded session metadata list because OpenCode does not persist their
project-directory rows. Only current directory-scoped busy/retry status permits
new automatic admission. Metadata paths remain device memory only; directory and
native Write policy are checked before and after the status query. Existing
admitted-session recovery still runs when new project enumeration fails.

API snapshot publication inspection now selects an exact source-sequence
boundary under the existing private inspection coordination. It reports native
receipt coverage separately from captured acknowledgements and continuing tail
message counts, including zero-message lifecycle transitions. Later repository
receipts cannot cover earlier withheld status records. This internal proof is a
prerequisite for management; it does not qualify public API import/export or
configured OpenCode startup.

Installation source management can inspect exact API snapshot publication
coverage through the retained private spool metadata handle. It preserves cached
source association across pause and stopped/offline harness observations, while
requiring current installation admission before and after the bounded read.
Missing evidence withholds coverage without repairing state or opening pending
bodies. Library/API acceptance does not qualify enabled OpenCode process startup
or historical API import/export.

## Usable Codex capture preview

The first trial uses the existing LocalTrusted foreground server and explicit
version-qualified Codex configuration. A repeatable development setup MUST use
an isolated native repository, linked physical workspace, current AgentStore,
private capture state and unused loopback ports. It MUST document the browser
URL, linked folder, exact harness version, configuration and stop command, while
preserving main's running fixture and installed applications. Capture remains
disabled by default. A one-time configuration/backend restart is sufficient;
UI-based configuration and hook installation are deferred. The setup MUST reject
unsupported versions and continue enforcing current repository/workspace
admission. It MUST NOT import arbitrary personal history. Real trial sessions,
controlled fixtures and the full multi-harness release are distinct evidence.

### Isolated Codex preview launcher

`pnpm dev:capture:setup` builds an isolated native development fixture, links
its physical repository and enables only qualified Codex CLI capture at the
normal known locations. It uses the existing dev UI/backend ownership checks,
ports 7470/7472 and ignored `.local/codex-preview` state by default.
`dev:capture` reuses it, `dev:capture:restart` replaces only the preview
backend, and `dev:capture:stop` stops the owned preview processes while
preserving history. The main fixture, installed binaries and unrelated services
are untouched. CLI version is checked before startup; Desktop is a separate
qualification gate.

The local preview exposes capture management through Settings → Agents, with the
shared divided Settings rows and header refresh. Local service retains service
and device-folder management.

The capture runtime retains the same resolved directory roots given to discovery
at startup. LocalTrusted Admin Settings → Agents displays those paths, including
Codex home defaults and explicit configured overrides; changing configuration
requires backend restart. Count-only coverage is insufficient for directory
configuration inspection.
