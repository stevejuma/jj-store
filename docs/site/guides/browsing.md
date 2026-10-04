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
views refresh after saves, committed events, reconnect and bounded checks;
explicit historical selections stay historical. To see newly published code,
return to a current selection deliberately.

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

## When results change

A new operation can change the current history while an earlier pinned page
remains valid. Paginated results belong to their original selection. Restarting
a query after an expired cursor is a new snapshot; do not concatenate it with
old pages and describe the result as one coherent history.
