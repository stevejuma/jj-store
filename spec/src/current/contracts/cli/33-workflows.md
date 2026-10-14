# Workflows

E-WORKFLOW-CARDS-001 adds UI guidance to command help for the action-summary
cards and Form description. CLI commands, selectors, formats and execution
contracts are unchanged; Form/YAML descriptions use the existing native field.

The custom jj workflow command manages YAML definitions, reads runs and
evidence, dispatches manual runs, requests cancellation and retries, and reports
assigned attempts. The separate workflow-host mode consumes operator
configuration outside checkouts. See JST-WF-004 through JST-WF-007.

E-WORKFLOW-RUN-001 confirms unchanged CLI contracts under JST-WF-004/006/007.
Shared native workflow help adds the browser's Settings-style source selectors,
bounded search and original-request recovery. Regenerated parser documentation
retains the existing arguments, request identities and execution-host admission.

E-WORKFLOW-MULTISTEP-001 qualifies ordered command actions sharing one private
attempt workspace under JST-WF-005/007. Source export occurs once; later actions
reuse earlier outputs. Empty-source attempts create the same shared directory.
CLI arguments, reporting receipts, source grants and unknown-attempt recovery
remain unchanged. This qualification does not implement Forgejo ConnectRPC.

E-WORKFLOW-SETTINGS-001 adds CLI-owned managed hosting from optional server
workflow_host configuration. The existing standalone version-1 JSON, --check,
--once and --workflow-host modes remain supported. Managed execution reuses
exact source export, journals and named executor dispatch. Reload replaces idle
settings only; shutdown drains a current poll before listeners stop. CLI
arguments and native storage formats remain unchanged.

The managed local runner also supports version-2 environment settings with
automatic registered-native attachment discovery. It derives local identity,
uses paired JJ and private recovery defaults, and resolves each job's source
from its recorded repository and exact pins. Independent version-1 standalone
host configuration remains supported. Discovery never uses arbitrary job paths
or a connection to another server, and missing bindings fail closed.
