# Standalone process

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

Native graph-cache fixtures require immutable primary receipts after cold
historical predicates, reuse of their separate data-only residents, and filtered
warm reads during primary preparation. A gated historical constructor proves its
private allowance remains charged while the namespace is free, with cleanup
after exit and warm reads surviving one-entry disk eviction. Private output
verification rejects wrong operation links, checksums and unexpected files
before the allocating JJ decoder. These focused regressions do not qualify
scale, production memory or combined HTTP/browser behavior.

Cold primary preparation fixtures pause a native commit read and require an
independent unprepared graph to complete in the same namespace. Completion
preserves that other generation and installs into a currently available slot;
active reservations stay charged until cleanup. Final installation contention
must retain request deadlines and cancellation, and must never replace another
live reservation or publish native state. Failed admission after a staging
rename must also remove the unretained slot and release its reservation.

HTTP acceptance compares owned DTOs at exact operations, including binary media
and error boundaries. Existing authenticated native-service fixtures retain the
future shared-deployment branch; the local release serves protected HTTP V1 on
literal loopback and native RPC through its owner-only Unix socket.
