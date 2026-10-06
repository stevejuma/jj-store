# M18 automatic capture configuration and controls (continued 2)

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

The M18 profile in [collaboration](../collaboration/index.md) owns agent_capture
server configuration, LocalTrusted write admission, explicitly selected
versioned harness roots and supervised capture. Omitted configuration remains
disabled. Settings apply at startup; pause/resume cannot enable an unconfigured
source. The strict format-2 agent_capture table accepts enabled, max_readers,
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
Implementation and release gates live in [milestones](../../progress/index.md).

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
