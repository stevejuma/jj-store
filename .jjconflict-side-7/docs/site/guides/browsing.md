# History, files and comparisons

Select a repository from the left sidebar before interpreting code views. The
profile and selected workspace determine which operations are available. A
project's linked repository may require separate access from its metadata home.

## Choose a snapshot

History follows the selected bookmark/workspace and query. Op Log describes
repository operations; Evo Log describes relationships between versions of a
change. A commit, a change and an operation answer different questions. Use full
IDs when copying a selection into a command or review.

Selecting a historical operation pins code reads. Files and comparisons retain
that context across navigation instead of drifting to the latest head. Current
views refresh after saves and relevant committed events; unavailable feeds use
bounded fallback; explicit historical selections stay historical. To see newly
published code, return to a current selection deliberately.

## Inspect files

Files provides directory navigation, content previews and applicable source/edit
controls. Binary or unavailable content is explained in context. Large files and
queries are bounded; an incomplete response must not be read as proof that a
path or change does not exist. A displayed file comes from the chosen snapshot,
not necessarily the physical working file on disk.

Use the comparison selector to inspect the relevant parent or group. Merge
commits can have multiple parents, and an aggregate comparison is different from
selecting a specific parent. Keep that choice when interpreting a diff or
creating an exact review anchor. Conflict-aware presentation preserves terms
rather than manufacturing a flattened file.

Files displays its current directory window before loading commit details,
per-path history and repository documents. Optional metadata can remain loading
or unavailable while the file list stays usable. Bookmark and workspace lists
load bounded windows; use their continuation controls to browse more entries. An
explicit selected name remains usable outside the first window.

## Local Stacks

Stacks browses branches and bounded histories for registered local Git-backed JJ
folders. Select a branch or commit to inspect its changed files and exact diffs.
These reads retain the selected code snapshot and do not snapshot physical edits
or change checkout files.

Named combinations, applying branches and displaying complete Unstaged work are
still under qualification. The controller observes current unsnapshotted file
metadata independently of historical branch selections, with bounded reads and
cancellation. Physical observations do not include all snapshotted combination
WIP and do not make those controls available; the page must retain its
unavailable state until the complete local workflow is supported.

## Prepare history when requested

If a history view requires preparation, use **Prepare graph index** for the
selected operation. Files and repository metadata remain independently usable
while it runs. Large repositories can take longer to prepare. **Cancel
preparation** requests cancellation of that task; wait for its confirmed status.
Changing repositories stops observation of the old task. Preparing or canceling
an index does not publish a code change.

## Navigation and copying

Repository, workspace, file and supported collaboration deep links preserve
complete identities. Access is rechecked when a link is opened. Copy is offered
for IDs/hashes and values with valid internal navigation; names, counts and
statuses are descriptive rather than copy targets. Following a link does not
also perform a row's copy action.

Opening Help preserves your application view. Closing the overlay returns to the
same selection. Help follows your active app theme and text size, including
changes made while it is open. Its left navigation, reading area and table of
contents use the same colors and surfaces as the app. The standalone manual has
its own light/dark switch using the Original palette. Navigation groups start
collapsed; expand a group to browse its pages. If a view is unavailable after
returning, inspect current permission or connection status instead of changing a
historical identifier to make a lookup succeed.

## Desktop terminal

The desktop app has a terminal toggle beside the details-sidebar toggle, or use
Cmd+J with a repository selected. Each terminal belongs to a verified local
folder or linked native workspace. Choose a folder when several are available;
link an existing workspace first if none is eligible. Browser-only clients do
not expose this control.

Tabs retain their repository and folder when you navigate. Hiding the dock or
reloading the renderer preserves its running shells; closing a tab ends that
shell. Resize or maximize the dock as needed. Maximizing closes the details
sidebar; restoring the dock leaves that sidebar closed. Explicit app Quit ends
all terminal sessions, and restart does not recreate them. This is a local
shell: commands have their ordinary effects on the selected folder.

## When results change

A new operation can change the current history while an earlier pinned page
remains valid. Paginated results belong to their original selection. Restarting
a query after an expired cursor is a new snapshot; do not concatenate it with
old pages and describe the result as one coherent history.
