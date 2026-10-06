# History, files and comparisons

Select a repository from the left sidebar before interpreting code views. The
profile and selected workspace determine which operations are available. A
project's linked repository may require separate access from its metadata home.
Repository directories load details for visible rows. Initial reads show
skeletons; refreshing loaded rows preserves their content.

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

Registered ordinary local JJ folders support named workspaces, applied branches
and Unstaged changes. See [Local stack workspaces](#local-stack-workspaces) for
the mutation workflow. Other repository profiles retain their own capability
limits; physical observation alone does not authorize a command.

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

## Local stack workspaces

For a registered local JJ repository, select **Stacks** in its repository tabs,
before Branches, and choose the target branch. This opens focused navigation:
Back, Branches, Workspace, Commits and Op Log. Commits and Op Log show the
repository-wide histories while retaining your workspace drafts and open diffs.
They follow the current workspace operation by default, including after accepted
commands or a successful refresh. In Workspace actions, turn off **Follow
current operation** to keep a historical snapshot pinned. Selecting an explicit
historical snapshot also pins it. The URL retains this choice across reloads and
browser Back/Forward; older focused links default to following current JJ. On
first open or reload, a skeleton covers connection, local binding discovery and
workspace loading. Later refreshes keep loaded content and drafts visible.
Failed reads show a notification and a Retry control instead of an inline error
banner. History chips cap long labels with ellipsis; hover shows the full value.
The workspace picker and workspace actions stay available. If a read starts
while the picker is open, its choices pause until the read settles. Toggle
Details to see the workspace name and ID, target branch, applied-stack count and
local folder. Selecting a stack, commit or file updates that rail; Commits and
Op Log show the selected repository history metadata. Back has a circular hover
treatment and restores the repository view and selections you came from; after a
direct link or reload it returns to Overview. Stacks does not have a separate
left-sidebar entry. Create a named workspace, then use Branches to apply or
unapply branches. Workspace groups proven linear dependencies beneath their
newest applied head, showing aliases together without duplicate commits. Sibling
branches stay separate; merge ancestry remains a graph. Branches already
integrated into the target remain in browsing and saved membership but do not
occupy active columns. Empty branches at the target remain commit destinations.
Column **Unapply stack** removes every explicitly applied member shown by that
column in one previewed transaction; **Unapply branch** removes only that
bookmark. Applying a branch does not apply an unapplied descendant. Workspace
shows those derived columns and Unstaged changes; file diffs open beside their
originating column. Each named workspace retains its unfinished work when you
switch. Active refresh snapshots files with normal JJ rules. Names, target and
membership are local settings in `.jj/jj-store/stacks.toml`; they are not
tracked repository files.

Workspace commands complete independently of the graph index. If history at the
new operation needs preparation, columns show skeletons until their bounded
history is ready. Managed code history prepares automatically; installations
using explicit preparation retain the Prepare graph index action. Unstaged
counts reflect loaded files; a `+` means more remain to load.

Conflicted Unstaged files show a warning icon and the workspace banner's warning
background in each listing mode. The top conflict banner appears only on the
Workspace tab and keeps its message, conflict-count chip and resolution action
on one row. The Details rail's Actions section offers Resolve conflicts,
workspace settings, branch browsing and refresh, plus Reapply saved branches
when membership differs. Resolve conflicts is also available from the
local-workspace notification, including while browsing Commits or Op Log. Its
button keeps a muted background at rest, with hover and keyboard focus feedback.
Both open the same in-app conflict surface used by Op Log without leaving the
selected tab. The file browser retains your resolution draft when you switch
files or hide the browser. Three-term text conflicts have an editable Resolved
pane; multi-sided conflicts let you compare each native version against the
current resolution in the same editable Merge Editor. Review every version,
stage the merged file, then preview the complete-file resolution. Other
conflicts and applied branches are retained. Unsupported or pending commands
remain unavailable. An unavailable text diff shows one message spanning the
column width; it preserves the native reason.

Select a branch to load its bounded stack history. Dependent branch segments use
native ancestry and bookmarks, with changed files inline for the selected
segment or commit. More history loads through a bounded continuation. In a text
diff, use **Show unmodified lines** to expand unchanged context before, between
or after the changes. The same controls work in unified and split views;
expansion preserves the native hunk selections used for dragging.

The local integration supports file/hunk assignment, commit editing and graph
actions; see [Editing and conflicts](editing.md#local-stacks-mutations). After a
lost command response, refresh JJ state and inspect operation history before
another attempt. Selecting another working change through the CLI keeps your
named workspaces and clears the active selection until you choose one. Undo and
Redo follow native repository operation history, including CLI commands. They do
not undo saved TOML settings. Refresh and Switch never silently restore saved
branch membership; use **Reapply saved branches** to reconcile it. That preview
also lists eligible obsolete workspace merge heads from older versions. Only
heads proven by recorded workspace operations and unreferenced by user
bookmarks, other workspaces or descendants can be retired. Native operation
history retains recovery. Reading or refreshing never performs this repair.
