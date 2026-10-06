# Retained runtime and storage ownership

Explicit capture composed with authoritative storage MUST acquire an opaque
storage lifetime before its first dispatch. Every discovery, source and delivery
owner, including its queued/running blocking task and unconsumed completion,
retains that lifetime. The lease is resource ownership only; it grants no
repository admission, Write permission or runtime enablement. Late attachment
and replacement of an existing lifetime fail closed. Authoritative storage
shutdown waits for the last owner before stopping embedded storage or releasing
its installation lock. Dropping storage or cancelling its shutdown wait MUST
retain those resources through the exact outstanding capture owners; driver loss
or a detached blocking task cannot release them early. Runtime shutdown still
has its existing five-second bound and never converts a timeout to proof of
completed drain. Whole-server startup retains this ownership; real harness
qualification remains separate from storage fixtures.

Foreground native serving MUST transfer its external-storage process state lock
into the same authoritative-storage owner before any capture lifetime is issued.
The transfer retains the exact file descriptor and rejects late attachment or an
already-owned lock. Dropping a prepared server or cancelling its run future MUST
NOT release that lock while capture still retains storage. Ordered storage
shutdown retains the lock through the last capture lease and storage stop.
Filesystem-only serving keeps its process lock without acquiring native capture
ownership; automatic capture remains unavailable there. This ownership transfer
qualifies neither configured capture enablement nor the remaining reader,
management and real harness/cold full-flow gates.

Guarded foreground composition MUST use one prepared capture slot. Disabled
selection reads no environment or installation state and creates no capture
directory. Enabled preparation validates the selected reader profiles, native
host, directory verifier, retained storage and exact registry installation
identity before opening private capture state. Codex locations follow explicit
roots, then CODEX_HOME, then HOME; privacy requires bounded absolute home and
temporary paths. The source registry, sanitized spool and scanner cache live
under the private server-state agent-capture directory outside checkouts.
Preparation opens no transcript, admits no historical session and initializes no
body scanner. Unsupported profiles, invalid privacy state or unavailable
resources retain a body-free unavailable slot rather than disabling ordinary
service or falling back to another reader. The prepared slot attaches storage
ownership before dispatch. Foreground execution starts this exact slot and
drains it before storage shutdown; cancellation drops its control handle while
retained owners finish draining. Whole-server Codex startup uses this same
composition and independently selected configuration; other reader, browser,
pressure/fault and real agent edit qualification remain separate gates.

An explicit Codex supervisor composes configured root discovery, activity
probes, staged admission, parked ingress and exact delivery without enabling
process startup. Disabled preparation performs no source I/O. Discovery
qualifies at most one candidate per owned step, deduplicates an opaque
physical/header fingerprint and tracks no more than the configured source bound.
Missing roots are retried on the configured rescan interval; discovery grants no
historical eligibility.

Source steps, discovery and delivery have independent retained blocking owners.
Forked in-process spool handles share a FIFO gate for synchronous journal
operations so repeated local lock collisions cannot starve a reader. Each turn
still acquires the authoritative filesystem mutation lock; independently opened
handles and processes retain their existing fail-closed lock behavior. The
compatibility lease stays alive until the last fork closes. The mutation gate
and mutation lease are released before native publication awaits. Acquisitions
through one shared source registry also take FIFO local turns before the
existing registry/physical/channel filesystem leases. Compact parked readers
probe exact metadata, header, durable progress and current admission without
reacquiring reader leases or reading source bodies. Changed snapshots/progress
need a reserved body frame before resumption; idle metadata grants no ingress or
publication proof. At most the configured one through four source owners may
hold a reader or run a source step. At most twice that bound may retain
noncompact source frames; compact activity/reader frames hold no partial
transcript allocation. Only a reserved frame may start body reconstruction or
ingress. A cancelled owner keeps its reservations until a completed step proves
them released. Exact pending batches and unfinished prefix proofs cannot be
compacted or evicted to make room. Delivery separately retains at most twice the
reader bound of independent partition recovery frames, preserving multi-step
proof progress across turns. Queue snapshots cannot report caught-up while
source or delivery work is in flight or a later enqueue has made that snapshot
stale. These bounds and the connected native-service regression are distinct
from full pressure, fault, rotation, typed-control and cold server/harness
qualification; configured Codex startup retains these bounds.

A compact source owner whose qualified physical/header binding is no longer
current may retire its memory-only tracking slot after its owned step settles.
Retirement MUST retain the source catalog, sanitized cursor and exact durable
queue and invalidate stale caught-up queue status. It MUST NOT evict an
unfinished prefix/context proof, raw batch or noncompact source frame, or
replace a busy, draining or lost owner. Task loss still requires explicit
replacement. Removing a compact slot removes its opaque deduplication
fingerprint and clears the prior tracking-capacity refusal; subsequent discovery
must independently qualify and arm or resume a source within the same configured
bounds. A new native session requires genuinely new observed activity after
arming; historical starts in a replacement file supply no eligibility. Retained
native-session resume keeps its existing current admission and exact-progress
checks. Read-only current-binding inspection supplies no identity, lease, cursor
or publication capability. Metadata and header inspection errors withhold the
compact owner for retry; negative binding results retire only body-free state.

An unadmitted activity probe whose bounded scan definitively detects truncation
or a changed verified prefix MUST discard that invalid activity/preparation
state and arm a fresh boundary on its next reserved source step. The typed
terminal probe state distinguishes these outcomes from retryable metadata,
header and I/O errors, which retain the original probe. Rearming MUST recheck
the exact qualified physical/header binding and treat every record already
present at the new boundary as historical, including starts appended with the
rewrite. Later body growth alone supplies no eligibility. Only a new observed
start after arming may admit sanitized backfill. Rearming allocates no source
identity, catalog entry, cursor, lease or native publication; existing admitted
reader frames and exact durable intent retain their independent recovery rules.
The same tracking slot and resident bounds apply across pause/resume. This
recovery does not qualify configured process startup or the full rotation
matrix.

When sanitized spool capacity refuses all new batches, exact unacknowledged
source frames still occupy their configured reservations. The supervisor MUST
NOT evict them, acknowledge bytes or enroll additional sources by exceeding the
resident-frame bound. Pausing retains those reservations; completed shutdown
releases owned memory and leases without persisting raw transcript state.
Stopped source owners MUST report zero occupied/resident reservations even when
shutdown completed directly from an idle paused state. Draining owners retain
their reported reservations until their synchronous work settles. Explicitly
restarting with a larger configured spool reconstructs unacknowledged bytes only
through retained native-session eligibility and fresh current directory/Write
admission. Sources whose new starts were never admitted require fresh observed
activity after restart; historical starts cannot fill that gap. This
capacity-refusal/restart qualification is separate from a saturated durable
queue, source rotation/retirement, faults and cold whole-server startup.

A durable queue filled by exact prepared native intent MUST refuse additional
ingress without acknowledging source bytes or replacing the original request,
actor or receipt identity. A committed native publication whose acknowledgement
was lost remains unknown to the caller until the original request resolves its
receipt under current admission. Revoking the prepared entry's repository also
withholds receipt resolution; another current directory cannot lend authority.
Pause and explicit supervisor restart preserve the selected prepared bytes and
latest sanitized source cursor. Once original admission is restored, the worker
may resolve that exact receipt and release queue capacity, then independently
admit later repository batches. A receipt retry cannot reauthor AgentStore
roots. This prepared-queue pressure qualification is separate from
captured-entry preparation growth/headroom, source rotation, fault matrices and
process startup.

Captured-entry admission MUST retain bounded preparation headroom so a permitted
queue head cannot become permanently undrainable solely because
Captured-to-Prepared growth fills the configured byte budget. Headroom must
preserve the existing exact-intent and hard byte-bound contracts without
assuming a native delta's contents, preauthoring dependent same-partition
captures or evicting accepted evidence. Legacy queues without enough preparation
capacity MUST retain captured bytes/cursors, withhold native publication and
expose the need for explicit configured capacity relief. The strict byte-limit
profile retains that legacy fail-closed behavior. New automatic reader captures
reserve the larger of the native and local prepared encodings as trailing JSON
whitespace in their atomically synchronized pending file. Every handle,
including existing hook writers, counts these physical bytes. Native sizing
covers all 4096 permitted delta edits, full references, the exact runtime
acknowledgement and any explicit binding with a maximum timestamp; it MUST NOT
assume which edits the current projector produces. Local sizing covers the exact
duplicated profile, sanitized batch and binding. Neither reservation authors a
request or reads a repository root. Preparation replaces only the selected
partition head and releases unused reserved bytes. Retry and restart preserve
accepted intent; no cursor is acknowledged when reservation capacity is
unavailable. Explicit hook admission and legacy compact records retain their
existing byte-bound profile. Configured startup preserves these contracts;
management diagnostics and full pressure/fault qualification remain separate
gates.

Admission and preparation byte-capacity refusal MUST retain a body-free typed
diagnostic with the configured limit and exact required total physical bytes for
the attempted replacement or admission. Existing reserved whitespace counts
toward that total; replacing one pending entry excludes its prior physical
length. These numbers authorize no capacity increase, queue eviction or
publication and contain no path, body, repository, principal or request
identity. Non-capacity errors MUST NOT masquerade as insufficient space. A known
refusal preserves exact pending bytes and reader cursors until successful retry
or explicit relief. Internal supervisor status retains the last settled ingress
refusal per source and the last settled delivery refusal, aggregating ingress
requirements by their maximum. Pause preserves these observations; successful
later steps clear them, and stopped/lost source owners cannot report active
ingress pressure. Status performs no filesystem work and cannot assert that a
cached refusal remains current after independent writers change the queue.
Authenticated controls and coverage presentation remain separate management
gates.

Loss of a queued discovery or delivery task MUST close that exact owner and
increment the body-free unavailable diagnostic once on settlement. Delivery loss
also invalidates queue freshness; it cannot report caught-up from a prior queue
snapshot. Neither lost global lane may be revived by pause/resume or replaced
implicitly. Accepted sanitized batch identities, exact pending bytes and source
cursors remain unchanged. An explicit new supervisor independently reacquires
current retained-session eligibility, directory and Write admission before
resolving the old queue; losing a task supplies no publication or history-import
authority. Queued task abortion qualifies this loss boundary separately from
panic after dispatch, OS-I/O loss, real storage or full cold harness faults.

Panic after discovery has qualified a candidate, or delivery has recovered an
exact queued range, MUST retain authoritative storage and its process lock
through task-state destruction. Cancelling a shutdown wait or dropping the
controller while that synchronous step is blocked cannot release storage early.
Settled task loss closes the lane and reports unavailable once. An already
committed native publication with a lost acknowledgement retains its exact
Prepared request, actor, receipt identity, queue bytes and sanitized cursor.
Explicit replacement still requires current admission before resolving that
receipt; resolution cannot duplicate the publication or rewrite native evidence.
Running-task panic qualification is distinct from OS faults, real RustFS and
live harness acceptance, and supplies no automatic startup authority.

An exact queued-range proof MUST be revalidated after current-directory
admission and before publication. Rebinding its approved transcript root,
installation-private source registry or sanitized spool while admission is
blocked cannot lend the old proof to replacement state, even with identical
transcript bytes or a symlink to the retained state. Withholding preserves exact
pending bytes, source catalog and sanitized cursor. Restoring the original
physical state permits independently qualified recovery under current admission;
neither the binding failure nor the restoration supplies history-import
authority. Loss of an initialized source catalog during that handoff remains
unavailable; the worker cannot reset source allocation or substitute an empty
catalog.

An owned pipeline-step executor MUST keep the pipeline, both reader leases,
private spool handle and current authority/admission context in exactly one
owner. It runs bounded synchronous reader/sanitizer/spool work on an owned
blocking task, while native asynchronous admission/publication uses the existing
runtime. At most one step is in flight for this owner; no second step or
replacement may race a busy or undrained owner. Deadlines are positive and at
most 300 seconds, begin at dispatch and prevent a queued step from starting
after expiry. A deadline or cancellation interrupts asynchronous waits while
retaining original cursor and durable/prepared intent for exact later work.

Synchronous OS work may outlive a deadline. Its owner MUST remain retained and
report overdue/draining until that work settles, rather than claiming that the
I/O stopped or releasing its leases for a replacement. Ordinary runtime work and
status/pause calls remain responsive. A known completed result stays known even
if it returns late; an interrupted publication is not a proven conflict. Pause
requests in-flight cancellation and stops new dispatch; resume cannot start
another step while the old owner drains or restore missing native Write.
Explicit shutdown stops dispatch, requests cancellation and waits at most five
seconds; timeout retains the owner for later settlement. Successful shutdown
releases its resources. Unexpected task loss closes that owner and requires
renewed source qualification; it cannot fabricate a reader or discard pending
evidence. Supervisor pause/resume may resume healthy lanes while the lost source
owner remains closed. It MUST NOT restart that source or acknowledge its unread
tail through the old owner. An explicit supervisor replacement reconstructs
retained admitted native-session progress and renews current physical/channel,
directory/catalog and native Write admission before resuming ingress. Revoked
queued partitions retain exact bytes and cursors across loss and control turns.
Drop requests cancellation; any running blocking task retains its OS leases
until it settles. Server composition must drain these owners before
authoritative storage shutdown. This executor does not schedule discovery,
allocate additional readers, authorize runtime enablement or qualify automatic
startup/cold full flow.

Queued Codex recovery MUST verify the admitted channel, qualified physical file,
exact header/generation witness, native session, configured parser/profile,
complete ending offset, prefix SHA-256 and deterministic batch identity before
using a reconstructed directory. It scans at most 4 MiB per step, retains at
most 1 MiB of a partial record and at most 256 trailing record contexts, and
rejects noncontiguous observation sequences, incomplete endings and mixed
contexts. The exact queued range is the retained observation count of complete
records ending at that verified offset. Raw history and reconstructed paths
remain in memory without Debug or persistence encodings. Source changes restart
bounded verification; replacement, rewritten prefixes, missing sources and
unavailable bindings preserve pending intent and withhold publication.

Recover queued entries in generation/sequence order, one exact batch per step;
other repository/source partitions remain untouched. New source reads wait for
all of that channel's unresolved entries, then reopen at the latest sanitized
durable source cursor without relinquishing either reader lease. Reopening
failure preserves the next-step resume requirement and cannot discard a
committed receipt. Current directory/socket/catalog and native Write
authorization precede every exact preparation or prepared retry. A known context
must resolve to the queued repository before body publication. Only a batch of
exact constant unverified-directory omissions may use a separately authorized
reconstructed prior directory or header anchor matching that queued repository.
This fallback cannot authorize body evidence. If the conservative prior anchor
cannot be rebound, retain the gap for later recovery. Prepared bytes,
authenticated actor, request/publication identities and receipt meaning are
never regenerated during recovery. Fresh-process proof recovery is qualified
separately from full cold server startup and real-harness acceptance.

Codex uses a versioned bound transcript reader. Cursor uses a validated
transcript when available and reports reduced coverage when absent/disabled.
OpenCode uses an explicitly bound read-only session/message API or qualified
consistent local storage, plus explicit exported JSON import. The server never
launches another harness, runs export commands or tails a database as JSONL.
Document each product surface/version, locations, overrides, format, activity
detection and coverage with sanitized fixtures. Desktop and CLI products qualify
independently.

The OpenCode 1.18.30 acquisition contract is pinned to source revision
`3104c1428ec91f809e5ab86631300de41eb6952e`, its V2 generated SDK types, instance
session/message routes and `export` implementation. The read-only adapter
requests only global health, session metadata/status and bounded message pages;
it never follows a supplied Link URL, sends credentials, follows a redirect,
prompts, launches the harness or runs an export command. A health version
mismatch withholds acquisition. Explicit busy/retry status may prove current
activity; session existence, title, updated time and exported messages alone
cannot. Message paging uses the opaque X-Next-Cursor value only against the same
configured endpoint/session/directory, with bounded responses and repeated
cursor refusal. API pages are observations, not a claim of one globally pinned
OpenCode database snapshot.

Acquisition responses are at most 1 MiB, API pages contain at most 32 messages
and 256 total parts, status maps have at most 4096 entries, and message
acquisition traverses at most 4096 pages. Oversized/malformed pages retain the
prior acquisition boundary; no source acknowledgement follows a refusal. Each
assistant snapshot retains its own declared native `path.cwd` as a memory-only
directory context. Missing/unsafe context cannot borrow the session header
directory to publish body evidence. Header version, message/part owners,
duplicate identities and changed session directory are checked independently.
These acquisition bounds are not complete filesystem export-reader or large
history qualification; those readers still require bounded streaming, safe
omissions, durable progress and exact restart coverage.
