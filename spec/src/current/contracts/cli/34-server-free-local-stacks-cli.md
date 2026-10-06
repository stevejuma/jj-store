# Server-free local Stacks CLI

`jj-store stack` exposes the ordinary local mutation profile through the shared
jj-lib engine, without a server, repository registration or companion metadata
initialization. `-C PATH` discovers the physical workspace from that directory;
only qualified Git-backed JJ repositories are supported. Native selectors must
resolve exactly one visible commit; membership uses exact bookmark names.

Status, topology, bounded history, diffs, native hunks and conflict terms
support text and JSON output. Normal reads and execution snapshot using JJ
policy. `--no-snapshot` inspection and `--dry-run` use a nonmutating single-head
loader: no configuration creation, operation-head reconciliation or checkout
changes. Concurrent heads require explicit native JJ reconciliation before
inspection. Previews are advisory and inspect the last snapshot. Command parsing
and missing confirmation fail before snapshots. `--yes` is required for
workspace/bookmark deletion, abandonment, discard and reconciliation;
apply/unapply execute directly.

Workspace, branch, selected-content, graph, supported text conflict and native
Undo/Redo actions reuse existing operation/configuration/source guards. Stack
unapply uses the native lane's exclusive explicit members; `--branch-only`
removes only the selected bookmark. Native history is repository-wide and TOML
is not an Undo journal. Selection input binds operation/configuration and exact
sources, paths and native hunk IDs. Stale selections never regenerate or
broaden. Messages use explicit arguments or bounded files; resolution text also
supports stdin. Mixed-source selection files are ephemeral input, never staging
storage.

JSON results carry operation and configuration identities. Exit codes are 0 for
success, 1 for execution failure, 2 for invalid arguments, and 3 for an applied
operation requiring checkout/settings attention; the latter includes the applied
operation and a no-retry warning. Help, keyword guidance and completions
describe all commands. Remote/PR workflows and interactive pickers remain
outside this local CLI slice. Development acceptance uses disposable
repositories and explicit binaries, and does not replace installed executables.
