# Verification

Shared capture controls for JST-COLLAB-017/JST-COLLAB-030 are additionally
qualified by `crates/jj-store-server/tests/capture_runtime/controls.rs`, the
runtime-drop case in `capture_runtime/draining.rs`, and closed-observation tests
in `crates/jj-store-server/src/capture_runtime/control.rs`
([E-M17-047](../../../current/progress/index.md)). Clones observe native MemoryStore capture and
request pause/resume without owning drain. Runtime drop immediately makes all
clones terminal while the blocked owner retains its raw channel, then releases
that channel on completed drain even while clones remain. Known terminal status
stays readable; closed nonterminal observations cannot return stale caught-up
status or accept controls. The latter cases model observation-channel loss,
separately from running blocking-task loss. This memory-only capability does not
qualify authenticated endpoints, durable pause, rescan/import, management UI or
public automatic startup. Direct foreground/runtime consumers include
fresh-process cold factory qualification; no real RustFS or live harness result
is claimed.

Delivery binding faults for JST-COLLAB-017/JST-COLLAB-019 are additionally
qualified by `crates/jj-store-server/src/capture_supervisor/tests/rebinding.rs`
([E-M17-046](../../../current/progress/index.md)). A paused source isolates delivery, whose owned
directory verifier blocks only after exact queued-range recovery. Four cases
replace the approved transcript root with identical bytes in another physical
file, symlink the retained private source or spool state, or remove the source
catalog. No case changes native roots, exact pending/cursor files or source
allocation. The absent catalog stays unavailable. After restoring original
physical state, explicit qualified replacement drains under current admission
and publishes the sanitized message once. These are real filesystem mutations
with a synthetic verifier and native MemoryStore; real JJ binding, RustFS and
live harness acceptance remain distinct gates. No production change was needed
for these qualification cases.

Running-lane loss for JST-COLLAB-017/JST-COLLAB-019 is additionally qualified by
`crates/jj-store-server/src/capture_supervisor/tests/running_loss.rs`
([E-M17-045](../../../current/progress/index.md)). Four fresh subprocesses block actual discovery
after candidate qualification or delivery after exact range recovery, then panic
the running task after shutdown-wait cancellation or controller drop.
Authoritative storage and a real process lock remain retained through
settlement. A real native MemoryStore commit with a lost acknowledgement retains
exact Prepared bytes and cursor; revoked admission withholds replacement, and
restored admission resolves the same receipt without rewriting the root snapshot
or message evidence. Test-only fault gates do not change public enablement. The
retained external-S3 client sends no requests. Real RustFS, OS-I/O faults and
live harness qualification remain separate gates.

Queued-lane loss for JST-COLLAB-017/JST-COLLAB-019 is additionally qualified in
`crates/jj-store-server/src/capture_supervisor/tests.rs`
([E-M17-044](../../../current/progress/index.md)). A bounded occupied Tokio blocking pool keeps the
real discovery/delivery tasks queued before their handles are aborted. Exact
sanitized queue bytes and cursors survive, unavailable settlement is counted
once, resume is refused and explicit current replacement drains the old batch.
This MemoryStore/filesystem fault test is separate from panic after dispatch, OS
faults, storage loss and real cold harness acceptance.

Guarded foreground capture composition for JST-COLLAB-017/JST-COLLAB-019 is
qualified through the fresh-process factory and foreground slot cases in
`crates/jj-store-server/src/process/tests/capture_factory.rs` and its foreground
helper ([E-M17-043](../../../current/progress/index.md)). These distinguish disabled no-I/O
selection, private preparation without history admission or scanner
initialization, exact installation binding, unavailable resources and privacy
state, configured/home location selection, synthetic active capture and retained
stop/cancellation. The publication host uses MemoryStore and the retained
external-S3 client makes no server requests. Unit fixtures assemble the private
foreground slot; configured startup is qualified separately by E-M17-069 and
independently refuses state creation. This is internal composition
qualification, not configured enablement, real RustFS, live harness, typed
controls or degraded-health UI acceptance.

Foreground native process lock ownership for JST-COLLAB-017/JST-COLLAB-019 is
additionally qualified by `crates/jj-store-server/src/process/tests.rs` and
`crates/jj-store-server/src/storage/lifetime/tests/attachment.rs`
([E-M17-042](../../../current/progress/index.md)). A genuine before regression releases the process
lock while a capture storage lease is alive. Four fresh-process paths cover
prepared-server drop, run-future cancellation, successful ordered shutdown and
failed preparation. The exact descriptor now belongs to retained authoritative
storage; late/duplicate attachment preserves accepted ownership and releases
only the rejected input. The external-S3 fixture constructs a client without S3
requests and uses actual private filesystem locks. The existing mocked embedded
process proves duplicate attachment retains its original lock through graceful
stop. This is foreground ownership qualification, not configured worker startup,
real RustFS or live harness acceptance. Filesystem-only serving retains its
separate process lock and native capture remains unavailable there.

Explicit storage retention for JST-COLLAB-017/JST-COLLAB-019 is additionally
qualified by `crates/jj-store-server/tests/capture_runtime/storage.rs` and
`crates/jj-store-server/src/storage/lifetime/tests.rs`
([E-M17-041](../../../current/progress/index.md)). Five fresh-process paths cover normal drain,
retained shutdown timeout, drop, cancelled storage wait and late attachment
refusal. Five ownership targets cover last-owner release, an embedded child's
ordered shutdown and retained installation lock, storage drop/wait cancellation,
detached completion/task loss, source-state destruction and unconsumed
completions, and 128 release/wait registration races. The storage child is a
disposable mock, while the filesystem locks are actual private device locks;
these are not real RustFS results. Native capture uses MemoryStore, and the
external-S3 lifetime fixture performs no S3 requests. This qualifies explicit
resource ownership; configured foreground wiring, including its external-mode
state lock, automatic startup and real storage/harness/cold full-flow gates
remain pending. Requirement rows stay Planned.

Explicit lifecycle ownership for JST-COLLAB-017/JST-COLLAB-019 is additionally
qualified by `crates/jj-store-server/tests/capture_runtime.rs` and
`capture_runtime/draining.rs` ([E-M17-040](../../../current/progress/index.md)). The driver schedules
discovery, fresh active backfill and exact delivery without caller quanta;
pause/resume retains the same supervisor and terminal shutdown rejects later
control transitions. A failed dispatch also latches terminal controls and a
completed failed join cannot be polled again. Raw pressure cannot advance the
cursor on shutdown. Shutdown timeout retains the join and blocked raw channel; a
later wait settles it. Dropping the handle requests stop while its owned task
drains independently. This qualifies the explicit runtime only. Configured
server startup, configured authoritative-storage integration, authenticated
management and real harness/cold acceptance remain pending; requirement rows
stay Planned.

Automatic raw-channel retention for JST-COLLAB-017/JST-COLLAB-019 is
additionally qualified by
`crates/jj-store-capture/tests/source_parking/reservations.rs` and
`crates/jj-store-server/tests/capture_supervisor/raw_channels.rs` and
`crates/jj-store-server/tests/capture_supervisor/raw_handoff.rs`
([E-M17-039](../../../current/progress/index.md)). Automatic parking releases physical ownership but
excludes alias/rotated channel readers until an exact raw allocation settles.
Failed acquisition, changed admission/allocation, replaced locks, foreign
registry directories and source replacement retain the frame without
transferring its channel. Cancelled physical acquisition retains ownership;
ordinary partial and idle parking remains lease-free. Native pause and queue
relief recover unchanged generation/sequence allocation and sanitized
publication at the same byte limit; shutdown releases ephemeral ownership
without acknowledging the raw tail and a fresh supervisor recovers through
retained native eligibility. Stalled admission before and after physical
acquisition becomes overdue, drains on pause and retains the raw channel through
current catalog refusal; restored admission retries and shutdown releases
ownership. Generic parking still releases both leases. Broader
allocation/rotation/root-rebinding and task-loss/deadline matrices, configured
startup and real harness/cold gates remain pending; requirement rows stay
Planned.

Physical-generation recovery for JST-COLLAB-017/JST-COLLAB-019 is additionally
qualified by `crates/jj-store-server/tests/capture_supervisor/generations.rs`,
`crates/jj-store-capture/tests/reader_progress/source.rs`,
`crates/jj-store-capture/tests/source_parking.rs` and
`crates/jj-store-server/tests/capture_parking/generations.rs`
([E-M17-038](../../../current/progress/index.md)). Exact physical/header selection prevents unchanged
archive/current replay; delayed writes preserve verified prefixes while
allocating channel-monotonic continuations. Truncation, equal-length prefix
rewrite and supervisor/spool restart retain independent cursors. Atomic
automatic admission includes generic sanitized pending allocation floors and
preserves exact durable retries. Conflicting parked raw batches remain unchanged
and cannot enqueue or leak fresh leases.
`crates/jj-store-server/tests/capture_source/progress_capacity.rs` additionally
preserves exact capacity diagnostics and accepted bytes/cursors when an
independent handle grows beyond the current reader's limit. Full raw-allocation
liveness/race and rotation/root-rebinding matrices, configured startup and cold
harness release qualification remain pending; these requirement rows stay
Planned.

Fresh current-context admission for JST-COLLAB-018 is additionally qualified by
`crates/jj-store-server/tests/capture_activation.rs`
([E-M17-027](../../../current/progress/index.md)). It requires a genuine original-probe start
snapshot, resolves current B after event/header A revocation, withholds before
enrollment on missing Write, stale snapshot, unbound directory or a foreign
spool, preserves bounded cursor-free preparation and applies independent A
omission/B body publication. Renewing the original probe differs from rearming
over historical starts. Explicit supervisor composition is qualified separately
by E-M17-028; automatic process startup and full harness qualification remain
pending.

Parked current-directory handoff for
JST-COLLAB-017/JST-COLLAB-018/JST-COLLAB-019 is additionally qualified by
`crates/jj-store-server/tests/capture_reanchor.rs` and its race/progress
modules, the live handoff case in
`crates/jj-store-server/tests/capture_supervisor.rs`, and compact
current-context regressions in `crates/jj-store-capture/tests/codex_resume.rs`
and `crates/jj-store-capture/src/codex_transcript/context.rs`
([E-M17-029](../../../current/progress/index.md)). A revoked parked A anchor can prepare
independently authorized B while retaining exact backpressure and unfinished
reading state. Partial context, missing Write, changed admission snapshots,
rebound/foreign spools, replacement sources and missing/regressive progress
cannot consume the frame. An advanced durable cursor reopens without reusing old
raw batches. The supervisor reserves context work, delivers B while exact A
intent remains withheld, and resumes A on restored admission. EOF preparation
releases its partial allocation and reconstructs anew after append/rewrite.
These fresh synthetic native MemoryStore fixtures do not qualify real JJ
directory binding, live harness edits, RustFS or cold whole-server startup;
M18.2 remains In progress.

E-M17-030 qualifies configured source-frame pressure through
`crates/jj-store-server/tests/capture_supervisor/saturation.rs`. Four armed
sources and one reader exhaust two retained raw frames under a one-byte spool
capacity. Refusal and current A revocation preserve unacknowledged state and
block excess admission. Explicit supervisor shutdown/restart with a larger
configured spool captures only previously admitted sessions until the others
observe a new start. It retains the source identities, sanitizes B, omits A and
respects frame bounds. This synthetic MemoryStore worker qualification does not
qualify a full durable queue, rotation/retirement, fault injection, process
startup, real harness execution, directory JJ binding, RustFS or whole-server
cold restart.

E-M17-031 qualifies exact prepared backlog pressure and captured preparation
refusal through `crates/jj-store-server/tests/capture_supervisor/backlog.rs`.
Its acknowledgement-loss fixture performs a real native MemoryStore publication
before reporting unknown, fills the configured durable queue with the exact
prepared entry, withholds revoked A and rejected B cursor advancement across
pause/restart, then resolves the original receipt and delivers B after A is
restored. A's complete AgentStore read snapshot and content remain unchanged on
receipt replay. Its captured-entry case proves that preparation growth exceeding
the hard byte budget leaves the captured intent and native roots unchanged until
explicit capacity relief. Neither case qualifies automatic captured preparation
headroom, all queue-pressure/fault matrices, real directory binding, live
harness execution, RustFS or process/cold whole-server startup.

E-M17-032 qualifies physical preparation reservations for new automatic reader
captures. Its reader progress tests fill the exact physical capacity, refuse
independent hook and forked reader writers without advancing progress, survive
restart, preserve exact retries and prepare a permitted 4096-edit native
request. A large local payload with explicit binding proves the duplicate
encoding fits. The connected native MemoryStore case fills a captured queue,
withholds a later partition entry, refuses new ingress, then drains cumulative
transcript updates without increasing capacity. The legacy compact case still
requires explicit capacity relief. Shared capture runtime and direct
native/server/CLI consumers are required by spool persistence and the common
local request envelope; wider UI and whole-workspace lanes are outside this
surface. These cases do not qualify management diagnostics,
rotation/task-loss/deadline fault matrices, live harnesses, real directory
binding, RustFS or configured process startup.

E-M17-033 injects loss of the owned current-directory verifier while A's exact
capture is queued and revoked and B's appended body is unread. The closed source
reports no resident or occupied frames or caught-up progress and retains exact
physical queue bytes and cursors across repeated control turns. Resume leaves
that source closed while healthy lanes may continue. Explicit replacement
requalifies the retained admitted channel and current B, then separately
publishes A and B under restored admission. This does not qualify discovery or
delivery owner loss, deadlines, rotation or configured process startup.

E-M17-034 removes the obsolete AgentStore local descriptor upgrade and forward
transcript encoding. Exact regressions reject canonical old descriptor/page
bytes while retaining current canonical identity and prefix dependency checks.
Current-tail native paging, multi-stream bounded append, pinned local reopen and
native copy qualify direct consumers. Native capture continues to require its
selected current root; missing legacy roots have no initialization gate. Local
first-use publication creates the current descriptor explicitly. Durable spool
recovery and exact receipts remain independent supported contracts. This change
qualifies current formats; it does not migrate existing repositories or qualify
live harness, RustFS, management UI or configured process startup.

E-M17-035 qualifies compact source tracking retirement through the four
`capture_supervisor/rotation.rs` cases and the read-only current-binding
regression in `jj-store-capture/tests/source_registry.rs`. The metadata outcome
classifier keeps missing bindings distinct from unknown OS errors. A compact
owner retains its slot through header inspection errors and accepts only fresh
activity after exact header recovery. A replaced compact source releases a
one-slot tracking budget; the replacement cannot use its historical start and
backfills only after a newly observed start. A pending noncompact raw frame
remains resident across physical replacement and controls, without source
enrollment or cursor advancement. A retired compact source retains exact padded
A queue bytes and its prior cursor while independently admitted fresh B
delivers; A's selected snapshot/content remain unchanged. Shared qualification
consumers and the complete direct supervisor binary are required. This does not
qualify all rotation/truncation/rewrite/rebinding or retirement matrices,
task-loss/deadline faults, real JJ directory binding, RustFS, live harnesses or
configured whole-server startup.

E-M17-036 qualifies conservative unadmitted probe recovery in
`capture_supervisor/rearming.rs`. In-place prefix rewrite and truncation retain
the exact physical/header fingerprint but discard definitively invalid probe
state. Starts present before the replacement arming boundary and arbitrary later
growth cannot enroll, acknowledge or publish. Current fresh activity after
rearming permits sanitized backfill, one native session and durable progress
without changing code heads. Registry bytes remain unchanged before admission
across pause/resume. The core activity regressions distinguish terminal prefix
invalidation from a retryable malformed-header failure, retain the latter
probe's original boundary and verify its fresh activity after exact header
restoration. This does not qualify admitted reader generation switching, all
root rebinding/rotation faults, real harnesses, RustFS or whole-server startup.

E-M17-037 qualifies body-free typed capacity refusals at the shared spool,
admitted ingress, exact native delivery and internal supervisor boundaries. The
`reader_progress/headroom.rs` regression verifies the exact physical replacement
total, unchanged compact bytes/cursor and successful retry at that explicit
limit. `capture_supervisor/capacity.rs` proves separate ingress and delivery
requirements, pause/restart retention, no publication before compact prepared
intent fits, sanitized native recovery after exact relief and clearing of
stopped/successful lane observations. The existing Write-denial regression also
rejects false capacity diagnoses. These cached status observations do not
perform filesystem I/O or claim current queue pressure; authenticated management
and UI presentation, actual server startup, real harness and RustFS/cold gates
remain unqualified. Shared spool and publication consumers require the complete
capture crate plus focused native/CLI consumer binaries and package checks.

E-M11-005 additionally covers copied Community project cards, task and review
rows, task detail sections, avatar-guided native reply trees, hover/focus
message actions, reply/edit composer context and the four-tab Projects landing,
independent unfiltered sidebar project hierarchy, settings-style responsive
Field forms, centered shadcn Empty icons and existing native ReviewStore
navigation. Focused directories tests exercise continuation mismatch and
permission removal; hub tests require synchronous refresh busy state and
fail-closed project review scope. Story interactions cover all four tabs and
nested linked repositories; cold browser acceptance opens native reviews and
restores the landing with Back. Visual baselines remain pending.

E-UI-045 covers the fifth Projects Activity tab, exact native review
comparisons, Community review sections, shared commit/agent context in
project/task/review pages, Description-only task defaults and padded main-tab
skeletons. Focused controller tests cover association pagination/root drift,
original agent capture identity, explicit operation/time continuations,
linked-repository authorization, access loss and late reads. Direct Storybook
interactions and accessibility use the same production components; a cold
built-browser read checks the live native project, task and review surfaces. The
implementation ledger records actual runs and keeps visual baseline approval
pending.
