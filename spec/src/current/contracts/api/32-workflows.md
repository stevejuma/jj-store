# Workflows

E-WORKFLOW-CARDS-001 changes presentation only. Form edits the existing typed
definition description; card summaries project recorded definitions without
changing requests, schemas, authorization, read pins or reporting. Native
description validation and exact pending-command recovery remain authoritative.

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

E-WORKFLOW-HISTORY-001 confirms unchanged JST-WF-006 wire, authorization and
persistence contracts. History composes bounded per-definition catalog pages and
exact home/run lookups, with immutable publication and attempt continuations.
Foreign run, attempt or snapshot replies are refused while retaining loaded
content. Discovery covers currently registered definitions; retained removed
workflows remain inspectable through an exact saved run link. No new endpoint,
source authority or historical-to-current reconciliation is introduced.

E-WORKFLOW-YAML-001 confirms unchanged JST-WF-006 contracts. Full-width YAML
presentation changes no workflow read, validation, command or source-selection
API. Returning to Form retains its visibility preference and existing draft
parsing; exact pending commands and shared notification recovery are unchanged.

E-WORKFLOW-CONSOLE-001 confirms unchanged JST-WF-006 and JST-API-006 wire
contracts. The console projects recorded definition/actions and attempt
evidence; on-demand reads reuse exact run/attempt/publication queries and
bounded cursors. Downloads do not introduce an endpoint. Re-run all jobs submits
the existing retry command with the latest recorded expected attempt; server
guards and exact outcome recovery remain authoritative, including when the
latest attempt is explicitly selected in a pinned view. Older/unknown attempt
restrictions remain.

E-WORKFLOW-SETTINGS-001 adds workflow configuration to the existing conditional
LocalTrusted Admin settings API, plus a workflows reload domain and runtime
status. Both native RPC and HTTP use shared live host admission. Execution
commands retain an admission lease through publication; activation fences the
replacement. Workflow commands, evidence, immutable objects and native formats
are unchanged. Configuration-management tokens do not acquire host control.

Managed runner settings discover verified installation-native workspace links.
Recorded job repository/revision anchors select the source; neither discovered
paths nor environment settings confer source or execution authority. Workflow
commands, reports, persisted native objects and their wire format are unchanged.

The GitHub YAML executor adds `github_workflow` to the existing executor union,
`skipped` to recorded outcomes and optional step scope to logs. Exact run pages
include their original source event for executor filtering. Bookmark events
carry an optional immutable previous-target comparison; absence does not imply
an empty tree. Admission remains independent of source access and secret access.

Run detail also includes bounded `step_reports` for the selected (or latest)
attempt, independent of log pagination. This permits live step discovery/status
without eagerly reading every log chunk. Logs remain pinned and loaded on
demand.
