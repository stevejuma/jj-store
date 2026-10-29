# jj-workflow

Workflow definitions, event filters, evaluations, run state and evidence,
independent of storage and executors. The `github` module handles the supported
GitHub-style workflow source format.

Use the exported command and state types to validate definitions and apply
workflow transitions. An admitted application host supplies events and performs
allowed actions; the domain model itself does not dispatch commands or agents.

See the [workflow command reference](../../docs/site/cli/jj-workflow.md) and
[workflow API](../../docs/site/reference/api/workflows.md).
