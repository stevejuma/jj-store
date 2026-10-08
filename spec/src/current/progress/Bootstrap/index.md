# Bootstrap tasks and gates

| ID          | Status | Depends | Requirements                                                                        | Gate                                                              | Next                             | Evidence                        |
| ----------- | ------ | ------- | ----------------------------------------------------------------------------------- | ----------------------------------------------------------------- | -------------------------------- | ------------------------------- |
| Bootstrap   | Done   | -       | JST-GOV-001,JST-GOV-002,JST-GOV-003,JST-GOV-004,JST-GOV-005,JST-GOV-006,JST-GOV-007 | Spec, ledger, skeleton, gates and independent commit              | Maintain governance each slice   | E-BOOT-001,E-BOOT-002,E-GOV-001 |
| Bootstrap.1 | Done   | -       | JST-GOV-001,JST-GOV-004,JST-GOV-006                                                 | Reference checksum, requirements, progress checker negative tests | Governance maintained each slice | E-BOOT-001,E-BOOT-002           |

## Workspace retirement review — 2026-10-08

E-GOV-004 records the authorized retirement of the integrated agent-capture,
bounded-reads, codex-preview and repository-conversion checkouts. Their folders,
including ignored fixtures and logs, remain recoverable in macOS Trash;
capture's original history and deferred OpenCode draft remain bookmarked. A
second inventory of registered workspaces and linked checkouts under `~/code`
and `~/.codex/worktrees` finds only main/default. Bootstrap and all product
milestone statuses and gate selections remain unchanged. This is maintenance
evidence, not additional product qualification.
<!-- generated-navigation -->

## Pages

- [history](history/index.md)
