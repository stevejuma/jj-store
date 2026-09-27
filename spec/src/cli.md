# CLI and server commands

First custom commands: init, attach, status, capabilities, doctor, serve-local,
and credential login/logout. JSON is never paged. Custom standalone no-command
invocation prints help successfully; ordinary upstream jj defaults stay intact.
Match jj-nostr presentation semantics without inheriting its storage or daemon
installation policy. Secrets stay out of argv, output, and checked-in config.

## Requirements

| ID          | Requirement                                                                                                                                                                                |
| ----------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| JST-CLI-001 | Custom commands MUST use jj store under upstream CliRunner, and the server MUST use explicit jj-store run with configuration, diagnostics, and token administration.                       |
| JST-CLI-002 | Custom command families MUST provide authored Markdown long help, short help, keyword help, hints, JSON results, TTY paging, no-pager, color controls, completions, and build information. |
| JST-CLI-003 | Ordinary JJ commands MUST preserve upstream behavior; a tested capability matrix MUST report unsupported backend commands honestly.                                                        |
| JST-CLI-004 | Development MUST use explicit build paths and MUST NOT replace installed jj or start unrelated services.                                                                                   |

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
bound graceful shutdown. Neither installs a daemon nor launches RustFS.

`jj-store config example` prints the configuration template, while
`config check` validates it without contacting storage.
`doctor --config PATH --repo-id ID` checks the configured repository through the
in-process authenticated service. It does not publish operations.
`operator init` persists a verifier for an environment-injected or OS-held
operator credential; `operator list/revoke` administer that trusted local policy
file. Token values are never command-line arguments or output. Repository token
commands remain a separate command slice.

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

HTTP acceptance uses real authenticated native service fixtures and compares
owned DTOs at exact operations, including binary media and error boundaries. The
foreground server serves HTTP V1 and native RPC on one listener, with shared TLS
and explicit insecure-loopback configuration.

The cold server/CLI deployment fixture also opens authenticated SSE, publishes a
native bookmark operation, resumes from the same event ID after server/storage
restart and cache deletion, and checks live stream termination on token
revocation. SSE and RPC share the production TLS listener and scoped native
sessions.

The CLI-owned native lifecycle fixtures also exercise every HTTP read route for
generated contract conformance. An opt-in capture path receives only disposable
response bodies and route metadata, never request credentials. This fixture does
not alter CLI presentation or publication semantics.

Server configuration may provide web_root, resolved relative to its TOML file,
to serve a prebuilt browser bundle alongside the authenticated HTTP/RPC APIs.
Build it with pnpm web:build. A malformed or oversized bundle fails server
startup explicitly; no filesystem directory listing or repository content is
served as a browser asset.

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
Browser work remains deferred under M4.3.

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
