# Explicit inactive-history import (continued 2)

Imported admission uses the existing version-1 bounded admission-digest catalog,
with a separate `codex-import-admitted/v1` key under
`jj-store/source-registry/`. Its length-prefixed parts are the qualified product
version, existing physical-lease digest, exact physical/header witness and
existing logical-session admission digest. Existing active-session markers
remain unchanged. Ordinary active lookup/resume MUST NOT consume import-only
markers. Import lookup, lease acquisition and reserved-channel reacquisition
require renewed exact accepted-prefix preparation and retained consent/current
policy/Write gates. This holds even after import selection but before the first
sanitized cursor: a rewritten prefix cannot become ordinary offline eligibility.
Each bounded prefix preparation remains independently scheduled; registry lookup
does not scan transcript bodies. A hardlink or overlapping approved root may
retain the same channel; replacement physical files, changed
headers/sessions/profiles cannot inherit import-only consent. Enrollment neither
resets durable reader progress nor grants prefix/body or publication authority.
Completed preparation is independent of accepted intent; this registry primitive
does not implement executable server import or scheduling.

The server source-import driver MUST obtain the actual retained qualified slot
from a separate runtime-private target view after current installation
admission; public observation pages remain body-free and carry no qualified
binding. New requests use the exact current selector. Replay searches retained
slots by the original opaque physical/header/session/profile binding before
considering an old selector, and fails closed when no unique current slot
matches. The target view is bounded by the existing source limit and grants no
lease or frame.

Native import preparation MUST retain the owned settlement through each native
admission await. It verifies bounded accepted-prefix/current context, fresh
native directory admission and Write, selects durable intent, then acquires the
shared import-only registry lease and opens the common reader at existing spool
progress. No source allocation or progress precedes intent selection. Rechecks
surround lease enrollment and reader opening. Imported pipelines MUST retain an
opaque renewal capability for the original receipt, reacquire current
policy/intent gates before body inspection, and renew accepted-prefix
preparation before ingestion, publication recovery and parked lease transfer.
They expose no conversion to an ordinary active pipeline. Parking preserves the
renewal capability and raw-channel reservation; failed or cancelled native
awaits cannot consume the parked frame. Each step releases policy gates after
settlement so pause remains actionable. This source driver does not allocate
supervisor budgets, own storage or dispatch work: those independent
retained-worker capabilities remain scheduling gates.

Explicit library scheduling MUST use the existing retained source slot and
reader/raw-frame/storage budgets. A single bounded runtime mailbox carries only
opaque body-free original-command metadata. Installation admission selects the
actual current target before dispatch; queuing releases its policy descriptors
and grants no acceptance, enrollment or body work. A busy/noncompact source
keeps its original reader until it can safely install the request. Each slot
retains at most one original command; exact retry cannot replace it, and
changed/conflicting work is refused without resetting the source. Stopped/lost
owners cannot be recreated by a scheduling request.

Every import preparation or prefix/body step MUST reserve a frame before opening
its probe and renew current installation/control/intent authority. The original
command and prefix state remain on the retained owner through cancellation and
deadline; only after that step's future settles or is dropped may policy gates
be released for pause. Retry reacquires the original accepted boundary before
enrollment. Imported parking carries incomplete and completed prefix state
without forcing a new history selection or monopolizing a reader lease. Raw
state is compact only when both common reader/recovery and import prefix state
are compact. A completed prefix witness remains cached only while its exact
snapshot is stable. Changed completed snapshots reconstruct the accepted prefix
before granting body work; incomplete scans remain retained within their budget.
Guarded publication runs on the same retained source owner, with fresh
original-receipt consent and accepted-prefix checks inside native publisher
preparation and publication awaits. Generic delivery cannot consume import-only
eligibility. Volatile scheduling acknowledgement is distinct from durable
selection and completion. Public progress observes accepted metadata separately;
accepted-request recovery uses the separately qualified supervised lane.

Implementation proceeds through intent/replay, owned admission/scheduling, typed
local API/CLI, then browser management and cold qualification. Regressions MUST
cover exact retry across timeout/restart, changed reuse, stale selectors,
byte-identical physical replacement, current Admin/Write/configuration/identity
loss, missing/rebound state, paused/stopped/lost owners, capacity and
cancellation retaining gates/frames/storage. Prove inactive headers remain
unadmitted until explicit acceptance and preserve aliases, progress, omissions
and unknown receipts. Configured Codex startup does not qualify other readers,
browser management, broader faults or real agent edit acceptance.
