# CLI reference

The custom `jj` executable registers application-owned extensions alongside
upstream JJ commands. `jj-store` provides the paired server and server-free
local Stacks commands. This reference is generated from their actual parsers,
including nested commands, inherited arguments, defaults and enumerated values.

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

## Local Stacks

Run `jj-store stack init --name NAME --target BOOKMARK` in an ordinary
Git-backed JJ workspace, then use `stack apply`, `unapply`, `status`, `list` and
`show`. Workspace, file/hunk, branch, graph and text-conflict commands share the
UI engine and `.jj/jj-store/stacks.toml`. No server or registration is required.
Use `jj-store help stack` for the complete guide.

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

The application-owned agent-workspace command is an internal paired-JJ boundary
for exact isolated workspace creation. Interactive prompts and permissions are
issued by the [Agents UI](../guides/agent-execution.md), not ordinary reads or
capture commands.

## Reusable conversation execution

The paired jj-acp executable must support explicitly selected control version 3
for reusable-agent conversations. The application exposes Start/Stop and
contextual Send; legacy version-2 session controls remain supported. The paired
custom JJ provisions personal companion homes or materializes registered native
attachments within app-managed paths. Development validation uses explicit
binary paths and does not replace installed CLIs.

`jj-store conversation-metadata --endpoint http://127.0.0.1:PORT/` reads
authorized conversation details, submits a bounded typed command file or
inspects a retained request. It requires explicit literal-loopback selection and
never redirects or retries an unknown outcome. See the generated `jj-store`
reference for arguments.
