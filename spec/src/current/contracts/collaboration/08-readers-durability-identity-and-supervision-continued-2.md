# Readers, durability, identity and supervision (continued 2)

The complete bounded journal operation and runtime application MUST have one
owned blocking task with retained coordination across caller cancellation or
deadline. Serialize record/replay and application under the same journal gate;
apply the latest selected policy, never the original replayed action. The
selection MUST recheck its exact private directory/lock and selected policy
under that retained gate immediately before runtime application. Physical
rebinding withholds application as unknown, even when replacement bytes are
identical; restoring the original binding permits exact receipt replay without
reauthoring. Durable policy receipts and runtime transition acceptance are
distinct. An absent, stopped or lost runtime may retain the selected policy for
restart but cannot be recreated by management. Current authorization loss after
selection withholds runtime application and reports an unknown outcome requiring
the original request. Controls contain no source bodies, directory proofs or
storage lifetime and cannot enable capture, enroll a source, import history or
publish native state.

Installation rescan MUST use a strict actor-free version-1 command binding the
control generation and expected policy revision. It is an idempotent transient
discovery hint, with no policy mutation or durable receipt. Unknown callers may
retry the same conditional hint; a changed policy requires explicit refresh.
Current Admin/configuration/installation and exact private journal binding are
revalidated under one retained bounded operation and journal gate before runtime
acceptance. Disabled, paused, missing, stopped or lost discovery owners refuse
rescan. The hint cannot resume, initialize or replace an owner, admit historical
activity, advance a source cursor, import history or grant publication
authority.

Rescan hints MUST coalesce in bounded device memory. They preserve active scan
progress and request a fresh pass after that bounded scan finishes; idle root
cooldowns may be brought forward. Source fingerprints, eligibility, directory
and Write checks, fair scheduling and existing reader/frame budgets remain.
Pause and terminal stop discard unconsumed hints and dispatch no new scan.
Cached status exposes discovery phase,
configured/unchecked/available/unavailable root counts, pending rescan and
last-settled degraded-source counts without transcript files, bodies or source
proofs. Availability describes the last observation, not current coverage or
authorization; loss retains observations with a stopped lane.

Cached runtime coverage MUST also group selected qualified harness profiles in
deterministic profile order, retaining declared product versions, root
availability and tracked/observed/awaiting/caught-up/running/degraded source
counts. The current Codex owner has at most two profile groups. These fields
contain no transcript path, native session key or source proof and require no
new filesystem work. Under current LocalTrusted Admin admission, harness
coverage also reports the exact approved directory roots resolved at worker
startup, including default Codex home selection and explicit overrides. At most
sixteen bounded absolute UTF-8 paths are reported per harness; these are
configuration metadata, never discovered transcript files, native session keys
or source proofs. The resolved directory list is retained independently from the
Copy runtime observation and survives pause and terminal stop. Status reads
perform no directory inspection. Root/source totals agree with aggregate
observations. They describe prepared reader coverage, not real product-surface
qualification or import authority. Source-level coverage, import and browser
controls remain separate gates. The installation CLI uses the existing local
Admin HTTP manager with an explicit literal-loopback URL, conditional policy
inputs and exact request identity. It performs no repository/device
initialization or implicit policy refresh and cannot enable or replace capture.
Unknown acknowledgement retains the original command; native publication and
historical import remain separate. Completion notifications may hold the larger
body-free aggregate observation behind bounded indirection; that notification
grants no resource ownership and does not change the cached Copy runtime
snapshot, scheduling or settlement.

Installation source coverage MUST be a bounded cached device observation, served
only through current LocalTrusted Admin/configuration/installation and private
journal checks. Disabled reads return an empty disabled page without
capture-state I/O. Status, source and import-receipt observations queue fairly
behind active management work for at most one second, then report Busy. They
MUST NOT repeatedly lose admission to later import steps or bypass current
authority and private-journal checks. Cancellation before acquisition removes
the waiter; after acquisition, the owned blocking observation retains its gate
until completion. Enabled reads require an existing runtime observation;
unexpected producer loss withholds stale nonterminal data. A completed stopped
observation remains readable and grants no ability to restart an owner.

Each runtime issues a fresh opaque generation and monotonically increasing
source selector ordinals. Selectors bind retained physical/header slots in that
runtime; retirement never reuses an ordinal and restart invalidates the old
generation. They expose no path, native session key, source-instance identity or
physical/header digest, and authorize no enrollment, cursor advancement, import,
publication or checkpoint. Import must independently revalidate the retained
source and current repository/Write admission; it remains a separate gate.

Source pages use a default limit of 50 and maximum 100. Typed cursors bind the
runtime generation and last selector ordinal; foreign generations withhold as
stale, invalid bounds/encodings fail closed, and guessed ordinals beyond the
runtime high-water mark cannot enumerate a fabricated continuation. Current
cached membership may grow or retire between pages; monotonic ordering prevents
reuse or duplicate enumeration. These pages do not freeze evolving diagnostics
or select a native repository snapshot. No continuation token retains storage,
raw frames, directory proofs or a task owner.

Per-source diagnostics include the qualified profile/version, owned reader
phase, observed/awaiting-activity/caught-up/running/degraded flags, last
verified bound repository when known and exact cached ingress capacity refusal.
Missing binding is an unknown observation, not proof of an unbound repository.
Observed and running describe reader work, not continuing model execution or
historical eligibility. Caught-up uses the same conservative settled queue
condition as aggregate status. Sources and aggregates retain existing
reader/frame/source bounds. A separate bounded body-free observation cache
leaves the Copy aggregate snapshot and all native/source/spool/control formats
unchanged; reading pages performs no transcript filesystem work.
