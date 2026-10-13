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

E-WORKFLOW-CHOICES-001 confirms unchanged JST-WF-006 wire contracts. Browser
trigger suggestions reuse authorized repository discovery, profile lookup,
operation-pinned history/query metadata, and paged ThreadStore reads. Fresh list
requests bypass only the client list cache; active conversation detail pins and
credentials are retained. Cross-home selection records the actual thread owner,
with no alias-based authority transfer or installation-wide user-list endpoint.
Agent setup IDs remain distinct from authenticated source principals.

E-WORKFLOW-RUN-001 confirms unchanged JST-WF-006 source and command contracts.
The browser Run dialog composes existing authorized native snapshots, operation
and commit graphs, and bookmark references. Commit and bookmark pages remain
pinned to the selected source operation; no workflow-specific lookup endpoint,
implicit head reconciliation or execution grant is introduced. Run submission
retains the existing exact source IDs, definition version and durable request.
