# CLI reference

The custom `jj` executable registers application-owned extensions alongside
upstream JJ commands. `jj-store` is the paired standalone server executable.
This reference is generated from their actual parsers, including nested
commands, inherited arguments, defaults and enumerated values.

| Family                         | Purpose                                                                     |
| ------------------------------ | --------------------------------------------------------------------------- |
| [jj-store](jj-store.md)        | Initialize/configure/run an installation, discovery and administration      |
| [jj store](jj-store-client.md) | Native creation, attachment, status, credentials, capabilities and recovery |
| [jj project](jj-project.md)    | Project homes and repository links                                          |
| [jj task](jj-task.md)          | Tasks, hierarchy and comments                                               |
| [jj agent](jj-agent.md)        | Capture, source inspection, provenance, Threads and reactions               |
| [jj review](jj-review.md)      | Exact artifact discussions, workflow and revision recovery                  |
| [jj sync](jj-sync.md)          | Local replication controls, finite waits and reconciliation                 |
| [jj replica](jj-replica.md)    | Issuer-bound frontier/checkpoint inspection                                 |

## Command conventions

Use `--help` on the exact command for long help and `-h` for short help.
Application command families provide structured output where their parser
accepts `--format json`. JSON is newline-terminated and never sent through a
pager. Text help follows the application's Markdown/color/pager presentation.
`--no-pager` is useful in scripts; never parse colored prose when JSON is
available.

An explicit development binary path prevents accidentally using an installed
upstream `jj` that lacks these extensions. In examples, `jj` means the custom
executable from this build. Ordinary upstream commands retain upstream parsing
and behavior except for explicitly documented native backend dispatch.

## IDs, versions and retry

Replace symbolic placeholders such as `REQUEST_ID`, `REPOSITORY_ID` and `COMMIT`
with complete admitted values. IDs have profile-specific widths; do not use
shortened display labels in typed documents. Mutations require the original
request identity and applicable expected versions. Retain both until the durable
result is known. A new request ID is a new intent, not a harmless way to retry.

The parser describes syntax, not qualification for every backend. Follow the
[capability reference](../reference/capabilities.md) and command's context
notes. Native cursors and local companion snapshot selectors cannot be
interchanged.

## Git and upstream JJ

[Native Git interoperability](../guides/git.md) explains endpoint approval,
selected push, leases and attempt recovery. For general JJ commands, revsets,
filesets and templates use the
[pinned upstream manual](https://docs.jj-vcs.dev/v0.45.1/). This reference
intentionally inventories application-owned commands only.
