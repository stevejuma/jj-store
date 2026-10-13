# Workflows

Workflow reads and commands use the common native service through HTTP and RPC.
Bounded pages, pinned roots and original outcome lookup retain current grants.
Semantic source events expose exact committed references for the optional
workflow host. See JST-WF-002 and JST-WF-006.

Workflow pages bound both rows and encoded bytes. Evidence continuations remain
on a pinned publication. Pending-run discovery uses run-ID seek continuation so
other hosts' claims and unmatched labels cannot strand later eligible runs.

Source queries accept at most 32 declarative condition sets per batch. Revsets
use the existing bounded native graph executor at the event operation. Failed or
unsupported condition evaluation is reported as unavailable, never as an
automatic positive match. Evaluate may include the pinned catalog of immutable
definition versions read by the host; legacy evaluations use the current
catalog. Thread and reaction sources expose typed immutable conversation
references.
