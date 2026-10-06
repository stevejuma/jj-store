# Event-driven freshness migration

The application freshness migration preserves explicit historical selections,
prepared commands and exact unknown request identities. Threads accepts a
successfully refreshed head independently of its route pin, preventing one
publication from repeatedly refreshing unchanged conversations.

Main views, Projects and repository navigation share authorization-session
repository leases. Closing one reader retains other consumers and their cache.
Credential replacement and denied access clear protected data. Query observers
track independent in-flight consumers; controller visibility owns event refresh.

An authorization session owns one advertised application feed, shared repository
transports and one QueryClient. The client subscribes before discovery and
initial captures, records domain dirtiness synchronously before advancing
continuity, and clears only successful reads from the current generation. Hidden
views retain independent dirtiness until accessed. Events invalidate live
aliases and affected collaboration families; exact immutable operation/object
reads remain reusable. Prepared availability, expiring cursors, errors and
optional metadata are transient.

Healthy idle views, Settings, capture and folder registration perform no
periodic data requests. Settings observes only its visible section; Projects and
sidebar navigation refresh affected repository windows, and inline
Agents/Reviews context refreshes without reloading code. An unchanged
five-minute feed renewal performs no data reads. Missing or stalled feeds use
serial jittered 5/10/20/30-second fallback, paused while hidden and removed on
recovery; preparation status alone uses its bounded 500ms-5s outage fallback.
Terminal event/ACK transport is separate.

Unrelated notices MUST NOT postpone an already queued event read. Deferred or
failed healthy reads retain dirtiness and retry serially with bounded 500ms-5s
backoff only while that dirtiness remains; successful reads stop the timer.

Conversion events only trigger inspection of the retained exact request. A
validated durable receipt alone completes conversion and permits navigation to
its independent destination. Historical selections, drafts and exact pending
commands survive all automatic freshness reads. Browser idle, renewal, external
change latency and recovery require cold built-bundle qualification.

JST-UI-020's workspace actions menu opens active named workspace Settings in the
standard right application sidebar, reusing Threads settings identity, detail
and destructive-action presentation. Settings use existing bordered Settings
field rows for name and a Command View target-branch picker. One bordered group
starts with Workspace ID, followed by name and target, without nested frames. A
separator precedes one horizontal row of Cancel and Save buttons with icons
inside that same rounded frame; the row has no explanatory label. Workspace ID
padding matches the editable rows. Drafts survive preview cancellation and
stale-command errors; unavailable local capabilities disable edits. Save reviews
the exact name/target command before execution. Delete workspace requires a
destructive confirmation explaining that only saved workspace configuration is
removed: branches and unfinished native JJ work are retained. Successful
deletion closes Settings and updates workspace selection; no generic
commit/file/hunk command is enabled.

Local stack/basic commit actions consume native previews and point inspection
for displayed commits. UI commit mutability never comes from graph position or
mock fixtures. Missing inspection support leaves those actions unavailable.

Lazy changed-file summaries and requested patches update the corresponding
loaded commit selection models, as well as Unstaged. Existing drag guides,
staged lists, split forms and inline commit composer use native file mutations
only after qualification. Net branch diffs compare exact boundary/head trees and
do not authorize transfers.

Native local Stacks hunk selections are issued by the guarded read-only POST
`/v1/local/repos/{repo}/stacks/hunks`. Tokens bind native source change
identity, canonical path and complete before/after bytes; apply rederives them
at the exact operation/configuration pin. Client patches and display-only IDs
are rejected. Partial text deltas use native squash with emptied sources
retained, without reconstructing the megamerge. Binary, conflict, mode/type and
bounded-diff limits remain whole-file-only. Net branch comparisons remain
presentation-only.

Local graph commands use native `move_commits` insertion/reordering and full
`squash_commits` for squash/uncommit. WIP is retained as a native child; graph
rewrites do not reconstruct the merge. Branch squash/rebase discovers at most
500 commits in its segment and stops at the next user bookmark boundary. Visible
source identity, configured immutability and workspace-target protection are
validated at preview and apply; retention commits are not graph sources.

The ordinary local Stacks mutation profile now includes native bookmark deletion
(top bookmark only; lower bookmarks and commits retained), abandonment, selected
whole-file/native-hunk WIP discard, and complete native text-conflict
resolution. Conflict terms are read lazily through guarded POST
`stacks/conflict`; ordered three-term text conflicts use the existing Merge
Editor. Multi-sided text conflicts expose bounded ordered versions for pairwise
comparison in the same editable Merge Editor. The user reviews every version and
explicitly stages the complete resolved file before preview. Structural, mode,
binary and oversized values retain explicit unsupported reasons. Conflict paths
are bounded to 50 per current read. While selected file text loads, the shared
conflict content shows an accessible busy skeleton with a header and three
panes, keeping the file browser and action header visible, hiding prior file
text and disabling stage/preview until the read completes. Loading copy is
screen-reader-only; errors retain their contextual feedback. Workspace and
focused Op Log share this loading presentation.

Undo/Redo restores the native repository operation stack, including CLI work,
with native descriptions and remote tracking semantics. Current Git bookkeeping
is retained, as in JJ. TOML is not an Undo journal. Missing or restored
references remain visible as unavailable choices/reasons; Refresh and Switch
never rebuild membership automatically. Explicit Reapply saved branches
reconciles membership and preserves WIP. Operation/config/source guards and
unknown-outcome inspection remain required. Production file dragging uses exact
loaded source/path pairs; partial dragging uses issued native hunk tokens. Net
branch comparisons remain read-only. Undo/Redo remains available without a
selected named workspace.

Production dependent branch segments derive from loaded native parent links and
local bookmark annotations, with one bounded continuation for the exact
advertised top head/target range. Only the lowest displayed segment loads more.
Each branch’s inline net files compare its own lower boundary and head; an
unproven paginated boundary remains unavailable until more history is loaded.
Merge/elided ancestry retains the provider graph without invented linear
boundaries. Visible applied lanes load their own bounded pages; other repository
history stays unloaded. Aliases do not duplicate commit rows within a lane.
Applied membership and branch inventory are distinct from workspace columns.
Bounded native topology at the active operation groups proven linear dependent
bookmarks into one column, retaining aliases without duplicate commits. Siblings
and merge-side ancestry are not flattened. Branches already integrated into the
target remain browsable but do not add active columns; empty branches exactly at
the target remain usable destinations. Column Unapply atomically removes its
explicit applied members; branch Unapply removes only that bookmark. Regrouping
preserves branch-keyed drafts, selections and adjacent diffs. Topology does not
depend on loaded commit pages, and unavailable history is never represented as
complete bookmark-only history.

Focused Commits and Op Log follow the current workspace operation by default.
Accepted workspace operations update the application-owned live history pin;
explicit historical inspection remains pinned. The frontend `stack_history`
field round-trips live/pinned intent through reload and navigation. Legacy
focused history defaults to live. History remains repository-wide and lazy;
loading/preparation must complete before the new graph replaces the old one.

An exact displayed commit receiving a dropped file/hunk MUST retain its title in
the destination picker even when it is not a bookmark head. Refreshing the
current read scope MUST reload retained adjacent diffs; closing or changing a
diff cancels its owned patch, while same-scope history updates do not cancel it.
Reconciliation that retains a source/path MUST preserve its patch lifetime;
cleanup of a completed or invalidated request MUST NOT restore a previous
operation’s loaded state.

Required graph preparation remains an unresolved view condition until successful
preparation or repository teardown clears it. Navigation and successful reads
may clear transient feedback without hiding the Prepare graph index action or
changing its exact required operation.

Successfully issued bounded stack histories update the adapter's current
comparison and command projection immediately, including inspected non-head
destinations and dependent segment boundaries. Neither mutation preview nor
inline branch files require an intervening refresh to use those exact reads.

Same-scope refresh and history updates retain authoritative loaded file
summaries and patches in the draft/drop selection model. A changed read scope
invalidates that cache before projection; visible cached rows cannot become
stale selections solely because a same-operation read completed.

Focused history exposes Follow current operation in Workspace actions. Losing a
local binding or authorization disposes its accepted-operation owner; transient
read pressure retains a loaded controller, drafts and pending commands. Grouped
branch cards render retained lower-branch selections using their original branch
identities. Managed-index readiness restores bounded stack history.
