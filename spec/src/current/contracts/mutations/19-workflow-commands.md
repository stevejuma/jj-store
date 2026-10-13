# Workflows

Workflow commands share one service authorization and semantic state machine.
Execution/reporting requires configured host admission in addition to repository
Write; only the assigned principal can report an attempt. See JST-WF-003 through
JST-WF-007.

E-WORKFLOW-SETTINGS-001 changes operational admission to shared live policy.
Publication retains its execution-host admission lease, and reload replaces the
admitted set at a fenced boundary. Repository Write still does not imply host
admission. No command, CAS, receipt, identity or WorkflowStore format changes
are introduced; unknown attempts remain non-replayable.
