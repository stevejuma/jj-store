# Readers, durability, identity and supervision

Automatic capture derives its authenticated actor from the installation identity
under the fixed `agent-capture-worker` purpose. Source native identities and
captured AgentIdentity never select that principal. Preparing this authority is
read-only, requires validated enabled LocalTrusted native Write/Admin
configuration and reads no installation state when disabled. Changing harness
roots or the effective Write/Admin permission preserves the actor across
restart; another installation and browser/search/replication purposes have
distinct principals.

Binding an in-process native capture publisher revalidates current directory,
socket and repository-catalog admission before and after native Write
authorization. It uses the existing service preparation/publication boundary and
the same sanitized spool/prepared request/receipt rules as RPC capture. No
bearer token, extra credential file, alternate storage writer or harness process
is created. Current admission must still be rechecked by the worker before each
publication; a bound native request alone does not retain directory permission.
Authority preparation does not initialize domains, allocate sources or start the
automatic worker. New native repository creation already selects an empty
AgentStore. Repositories without that selected root are unsupported for native
capture; no legacy initialization or migration gate remains.

The guarded in-process publisher retains only an in-memory directory proof and
binds a fresh native Write request for every preparation and publication,
including already-prepared retries. Both current workspace/socket/catalog
admission and native authorization precede each action. Rebound directory proofs
or removed registrations withhold work without changing prepared intent; capture
requests with another authenticated actor are refused. Preparation additionally
rechecks admission before returning its candidate. A committed publication keeps
its receipt even if device admission changes afterward; a later retry requires
current admission again. This publisher grants no active-source eligibility,
source lease, checkpoint, spool selection or automatic startup authority.

The admitted Codex source driver connects these capabilities without enabling
server startup. Active acquisition verifies the event directory and current
native Write authority before source enrollment; admitted restart requires the
retained exact native-session admission and a current verified directory
binding. The header-only convenience acquisition remains explicit; supervised
restart uses the bounded preparation below when the original directory is
unavailable. The driver retains a memory-only binding to the exact canonical
private spool root and physical directory identity. Each step checks that
binding before source reading or pending resolution; another or rebound spool
cannot clear an old intent. A fresh handle and changed capacity for the same
unchanged root remain valid. Binding inspection is read-only and creates no file
or durable format. It opens the leased reader at the latest source-wide
sanitized cursor, and holds new reading while unresolved durable pending entries
recover their exact directory proofs. Inactive generic registration remains
ineligible.

A staged admitted-source resume preparation MUST perform read-only retained
native-session/channel lookup without allocating identities, aliases or leases.
It reconstructs the current directory from the exact qualified header and all
complete native context records through one stable current file snapshot. Each
step reads at most 4 MiB, retains at most 1 MiB of a partial record and never
emits or durably copies source bodies. A partial final record cannot alter
context. Metadata changes restart bounded reconstruction; changed header,
physical source or approved root requires renewed qualification. This
preparation does not acknowledge a cursor or validate any queued publication
range, whose independent exact proof remains mandatory.

Current context, or a separately verified prior/header anchor, must pass current
native directory/socket/catalog admission and native Write authorization before
reader leases or new alias enrollment. A revoked original header directory MUST
NOT prevent an eligible same-session tail whose verified current directory is
still linked and writable. Retained native eligibility may resume even if the
original process exited before its first sanitized enqueue. Generic registration
or source growth cannot supply eligibility. Recheck the unambiguous retained
channel, file snapshot and directory binding around acquisition; a changed spool
cannot lend its state to the preparation. Missing/rebound/revoked sources
withhold without discarding durable pending intent. Acquiring through an anchor
cannot authorize body evidence: every read batch and queued entry still needs
its own admission or exact safe-omission conversion. Returning a source
completes this one-use preparation; retry cannot create a second reader under
its proof. Blocking-I/O ownership, scheduling, deadlines and automatic startup
remain supervisor gates.

The same bounded current-context preparation may accept a fresh, unforgeable
Codex activity snapshot in place of retained native admission. It MUST verify a
genuinely new start observed by the already armed activity probe; generic
registration, source growth or rearming over a historical start is insufficient.
Qualification extracted from that snapshot grants no activity or directory
authority by itself. Preparation allocates no source identity, alias or lease
and cannot acknowledge a cursor or copy a transcript body.

The fresh proof MUST retain its exact physical/header and file snapshot
throughout preparation and acquisition. A changed snapshot requires renewed
activity proof from the original bounded probe, with its existing prefix
verification; a saved proof cannot authorize newly appended bytes. The complete
current directory, or a separately verified prior/header anchor, must pass
current native directory/socket/catalog and Write admission before fresh source
enrollment and reader leases. Thus a start observed in A followed by a valid
current B context may admit B even when A is now revoked. Partial context
records cannot supply B, and the fresh proof cannot authorize historical A
bodies through B. Each ingress batch and queued range retains independent
admission, safe-omission and publication rules. Recheck the activity snapshot,
reconstructed context, admitted channel and directory proof around acquisition;
source changes withhold. A successful preparation is one-use. Owned preparation
I/O, probe renewal, resident-memory scheduling and automatic startup remain
supervisor gates.

Each driver step reads at most one existing bounded native batch or drains one
queued native batch. Known contexts use current per-batch directory admission;
unknown or unbound contexts become constant safe omissions only under a
separately revalidated authorized prior anchor. Transient admission failure
withholds the batch. Workspace-specific sanitizer instances retain the same
home/temporary/cache privacy roots and are rebuilt when the workspace changes.
Current admission is checked after sanitation and before durable enqueue. The
sanitized spool must accept the exact batch before reader acknowledgement; spool
refusal retains the batch and cannot advance source progress. One uncommitted
batch per conservative `step` driver prevents further source reading until
guarded publication resolves it. Publication selects only the retained exact
batch identity; additional entries require independent proof recovery instead of
inheriting its directory admission. Cancellation and denied/unknown publication
retain the queued intent and its in-memory proof for later exact retry;
committed completion preserves reader journals and code heads. Scheduling,
blocking-I/O ownership, deadlines and typed pause/status remain supervisor
gates. This explicit driver is not a standalone watcher or full server lifecycle
proof.

The independently scheduled Codex pipeline consumes this driver and exposes
separate bounded ingress and exact queued-publication steps. Ingress MUST
continue at the source-wide sanitized durable cursor while old queued entries
remain withheld, prepared or unknown. Every new batch retains its own current
directory/socket/catalog and native Write admission, sanitizer and exact spool
acknowledgement; no queued entry or receipt supplies that admission. Spool
capacity still stops acknowledgement and further reads without dropping the
reader's exact pending batch. The pipeline does not retain an accumulating map
of per-batch raw directory proofs; delivery reconstructs each selected exact
range through the qualified bounded recovery probe.

Publication accepts an explicit head selected for this leased source and
integration. It recovers only that entry's exact proof, and same-partition order
is rechecked under the existing drain lease. Revoked, unavailable or unknown
repository-A intent cannot prevent separately admitted repository-B ingress or
delivery. Earlier A intent still prevents later A delivery. Publishing old
entries MUST NOT rewind the ingress directory anchor or reader to an older
context; only ingestion advances that anchor. Restart opens at the latest
sanitized cursor, without requiring old queued publication first. The consumed
driver's serial convenience is unavailable through the pipeline, preventing
implicit lane changes. One retained bounded recovery probe supplies no other
entry's proof; switching the selected entry restarts its independent recovery.
The supervisor must still own temporal fairness during multi-step proof
verification, blocking I/O, deadlines, backpressure, pause and restart. Pipeline
composition does not qualify automatic startup or full harness/cold acceptance.

A generic pipeline may park its opaque device-memory reader frame to release
both physical and source-channel leases. Automatic supervisor parking MUST
release the physical lease but retain the channel lease while an exact
unacknowledged raw batch owns its generation/sequence allocation. Idle readers,
partial scans and unfinished prefix proofs without a raw batch retain ordinary
lease-free parking. Retained channel reservations count within the existing
bounded noncompact frames, at most twice the reader limit; they grant no body or
publication authority. Pause, cancellation and failed admission retain that
reservation and exact raw state; successful enqueue/acknowledgement followed by
idle parking, completed shutdown or owner death releases it. Parking MUST
preserve partial scans, multi-step prefix/context verification and the exact
unacknowledged raw batch; it cannot acknowledge bytes, serialize raw state or
grant publication authority. The supervisor MUST separately bound resident
frames and raw buffers, including parked frames; a reader-lease limit alone is
insufficient. This handoff itself does not allocate supervisor frames or enable
process startup.

Resumption MUST independently recheck retained native eligibility, the exact
private spool binding, qualified profile, physical source and header witness,
current directory/socket/catalog admission and native Write before acquiring
fresh physical leases and either fresh or retained channel ownership. Retained
channel reacquisition MUST validate the exact private registry directory and
lock-file identity, admitted channel, profile and physical/header qualification
under registry/FIFO coordination. It acquires only a fresh physical lease and
keeps the channel in the parked owner across admission awaits and failed
handoffs. No lock descriptor is cloned; only a fully checked successful transfer
consumes the reservation once. Foreign registry directories, replaced locks and
concurrently held physical leases fail closed. The retained prior ingress anchor
requires its own current authorization and cannot authorize later body batches.
A revoked anchor with a separately eligible current directory needs staged fresh
admission; parking itself cannot manufacture that proof. Changed or unavailable
admission, qualification or spool preserves the frame without reacquiring a
reader. A successful handoff consumes it once; failed handoffs retain it for a
later exact retry, and a concurrently leased source cannot be replaced.

A parked pipeline may separately prepare a current-directory handoff when its
prior ingress anchor is no longer admitted. This retained-session preparation
MUST validate the same qualified physical/header channel and exact private
spool, then reconstruct only complete current context records through a stable
snapshot, at most 4 MiB per step. It allocates no source identity or lease and
cannot acknowledge or discard the parked reader's raw batch, partial scan or
unfinished prefix proof. A partial EOF record supplies no directory authority.
At stable EOF, preparation releases its partial allocation while retaining the
current-context/snapshot proof. The supervisor reserves a noncompact source
frame before this body reconstruction; one such frame may contain one parked
reader plus one bounded current-context preparation.

The current directory, or a separately revalidated prior/header candidate, must
pass current native directory/socket/catalog admission and Write before reader
lease reacquisition. Recheck the preparation snapshot and retained native
channel around acquisition, together with ordinary spool/progress checks. Only
successful transfer changes the ingress anchor and consumes the frame; failures
retain both exact reading state and bounded preparation. An independently
advanced sanitized cursor still requires the ordinary verified reopen. The new
anchor grants no body or queued-publication permission: revoked A bodies may
become constant omissions under authorized B, while each B body/range requires
its own current admission. Fresh generic registration or a changed physical
source cannot use this retained-session handoff. Configured foreground startup
independently owns the server resources.

Only an unchanged durable source cursor permits reuse of retained raw state. An
independently advanced sanitized cursor requires a newly opened reader at that
progress, with ordinary bounded prefix verification; it MUST NOT re-enqueue the
old frame. Missing, changed-at-the-same-position or regressive durable progress
fails closed rather than resetting to the header. Check progress and admission
again after lease acquisition. Appends and rewrites still follow the reader's
existing snapshot/prefix checks; parking cannot suppress those checks. Source
replacement, even with byte-identical headers, cannot reuse the old physical
frame. No durable source/cursor/pending/prepared-request or receipt format
changes.

An explicit device-memory lifecycle driver may own the prepared Codex supervisor
and schedule its independent lanes without caller-driven quanta. It MUST retain
one terminal stop state, bounded last-observed body-free status and its exact
supervisor across pause/resume. Stop prohibits new dispatch. Shutdown timeout
MUST retain the join handle and all draining ownership for a later wait; it
cannot abort or replace blocking owners or permit storage teardown. Dropping the
driver requests stop while its task retains draining ownership until settlement.
Only completed drain permits ordered storage shutdown. This explicit lifecycle
capability grants no configuration enablement and does not independently qualify
authenticated controls, other readers or real harness acceptance.

The explicit runtime may issue cloneable device-memory status/pause/resume
handles for later management composition. Those handles contain only the
body-free cached observation and command channel, never source state, storage
leases or a drain task. Dropping a handle cannot stop the runtime; dropping the
sole runtime owner requests terminal stop even if control clones remain. A
control request acknowledges a desired transition, not completed pause or drain.
Observed draining remains distinct from completed stop. Unexpected loss of a
nonterminal observation producer MUST report unavailable rather than expose its
stale caught-up snapshot as current. A known completed Stopped observation
remains readable. All handles reject controls after terminal stop or producer
loss; resume cannot recreate an owner or grant enablement/admission.
Authenticated transport, durable pause state, rescan/import and management
presentation remain separate gates.

Runtime handoff of a settled paused supervisor MUST preserve that pause in its
initial command and observed phase. It cannot attempt dispatch, report failure
or replace the owner merely because no resume was requested. Initial pause
performs no source discovery/body reading, allocation or publication. Explicit
resume arms new activity boundaries normally; starts already present before
arming remain historical and grant no eligibility. Only independently qualified
new activity may then admit sanitized backfill. This preserves the existing
runtime constructor; it adds no startup mode or durable control record.

Durable capture pause policy is installation-private device state, separate from
AgentStore and native publication authority. Its versioned record MUST bind an
exact installation and fresh control generation, preserve monotonic revisions
and retain exact authenticated actor/request/expected-revision/action receipts.
Replay returns the original receipt alongside the latest selected policy; it
cannot reverse a newer transition. Changed reuse conflicts, stale new requests
withhold, and another generation cannot inherit old requests. The bounded
initial profile retains at most 4096 receipts within 1 MiB; exhaustion refuses
new changes without evicting accepted receipts. Receipt resolution remains
available at capacity and synchronizes the selected file/directory before
acknowledging durability.

Control state lives outside checkouts in an owner-only directory. Explicit
initialization requires a new directory; ordinary open MUST NOT initialize,
repair permissions or reset missing/corrupt/foreign state. Every operation pins
the directory and exact coordination lock, rejects symlinks, unknown files and
unsafe owners/permissions, and verifies the selected regular file around bounded
reads. Under the short exclusive filesystem gate, synchronized temporary-file
replacement and directory sync select a change. Only one recognized unpublished
candidate may be cleaned before another mutation; it grants no selected policy.
Failure after replacement is an unknown outcome resolved with the same request,
not a proven conflict or authority to issue another request. These device-state
receipts authorize no source enrollment, native publication, history import or
configuration enablement. Authenticated management must revalidate current
authority and apply the latest selected policy to the owned runtime separately.

Foreground capture composition MUST consume a selected control journal. Only
whole-server preparation may initialize an absent capture root under the owned
state lock as specified above; an existing root and internal resource-only
preparation MUST NOT initialize missing policy. Validate the selected journal
before creating source or spool state. The ready slot retains its exact control
generation, physical directory/lock binding and installation identity.
Immediately before runtime handoff, re-read the latest selected policy and
installation identity. A pause or resume selected after preparation MUST
determine the initial owned runtime state. Missing, corrupt, foreign,
regenerated or rebound state withholds that handoff without resetting policy or
starting discovery. Initial pause reads no transcript body, enrolls no source
and publishes nothing. Disabled preparation still performs no capture
environment/state I/O. This startup handoff does not watch device-policy files
or replace authenticated management's independent current-authority and
runtime-application requirements.

Installation-local status and pause/resume management MUST require current
LocalTrusted Admin authority and derive its actor from the admitted transport.
Commands contain only a version, request identity, control generation, expected
revision and action; clients cannot select the actor or initialize missing
state. Management binds the selected installation, capture configuration,
private journal generation and one runtime control cell. Current configuration
and identity are revalidated before device-state access and again before runtime
application. Disabling or changing the effective configuration cannot borrow the
prepared owner's authority. Disabled status performs no capture-state I/O.
