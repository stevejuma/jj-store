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

## Repository test audit — 2026-10-08

E-GOV-005 records the requested full test audit and development-server restart
with the existing fixture preserved. Rust, script, Storybook and browser
failures remain explicit alongside passing lanes, controlled reruns and
unavailable qualification prerequisites. This audit changes no product or test
behavior. Bootstrap and all product milestone statuses and original gate
selections remain unchanged; passing subsets do not qualify failing or skipped
gates. The next action is to triage the named failures at their owning
boundaries and rerun those lanes after fixes.

E-GOV-006 closes the three browser execution gaps left by the audit wrapper:
both automatic-update cases pass, while the task Activity/composer geometry
assertion fails reproducibly. The isolated native fixtures were cleaned. This
addendum preserves the original audit and changes no milestone status or gate;
all other audit failures and external prerequisites remain explicit.
<!-- generated-navigation -->
## Pages

- [history](history/index.md)
