# Editing and conflicts

Editing depends on both the repository profile and the artifact you selected.
Read-only access, a historical selection or an unsupported local capability can
remove an action. The browser's capability checks are more specific than the
existence of a CLI command in the reference.

## Native commands

Native repository actions use typed preparation, preview, apply and status
contracts. Inspect the proposed change before applying it. The prepared intent
binds exact repository state and preconditions; a stale selection fails rather
than silently targeting newer code. A successful committed result refreshes the
active view.

If a response is unknown, retain the pending command and use its status action.
A loading indicator or retry attempt is not evidence that publication failed. Do
not create a second request merely to clear the UI. Field validation remains
beside the editor, while transient operational feedback uses session notices.

## File editing

Use the file's Edit control when available. Keep Source/Preview/Edit selections
within the file body; navigation and code context remain separate. Save submits
the applicable profile's action, while Cancel discards the unsaved editor draft.
Follow dirty-edit prompts when changing context.

In the local Git-backed browser, physical working-file Save is filesystem-only.
It neither snapshots those files into a JJ operation nor publishes companion
metadata. Check the foreground JJ working copy before using its normal snapshot
workflow. This distinction also applies when a saved file looks identical to an
older committed preview.

## Local Stacks mutations

In a registered ordinary local JJ repository, a named workspace retains a
megamerge and an ordinary unfinished commit above it. Drag files or supported
text hunks from Unstaged into a stack's staging card to compose a new commit, or
onto an existing commit to move those changes into it. The assignment picker
names the exact destination commit, including commits below a branch head.
Review the preview before applying. Native squash and descendant rebasing
propagate the changes through the existing merge and WIP; they do not rebuild
the workspace combination. Empty unfinished commits remain available.

Commit and branch menus provide reword, empty-commit and branch insertion above
or below, rename, split selected changes, move/reorder, squash, uncommit and
rebase where the exact native commit is mutable. Target and internal retention
commits remain protected. Delete stack removes its top bookmark and saved
membership, retaining lower bookmarks and commits. Abandon and selected discard
require destructive confirmation. Discard applies only to the selected current
WIP files or native text hunks, retaining unselected content.

Hunk tokens bind the exact source and complete file bytes. Binary, mode/type,
conflicted and oversized deltas require whole-file operations or native JJ
tools. Net branch diffs are read-only. The local Merge Editor supports bounded
three-term text conflicts with an editable Resolved pane. Multi-sided text
conflicts retain the native version picker and show the same editable Merge
Editor. Compare each version with the current resolution, then explicitly stage
the whole file for complete-file preview and guarded JJ resolution. Selecting a
deleted version cannot replace it with an empty file. Structural, mode, binary
and oversized conflicts retain their explicit unsupported reason and require
native JJ tools.

Undo and Redo follow repository-wide native operation history, including CLI
operations, while retaining current Git bookkeeping. They do not restore
`.jj/jj-store/stacks.toml`. Refresh and Switch preserve saved membership without
rebuilding it; **Reapply saved branches** explicitly reconciles the combination.
After an unknown response, inspect current state and operation history rather
than replaying the command. Changes can complete before the graph index is
ready; use **Prepare graph index** to resume retained histories and diffs.

## Resolve native conflicts

Open the conflict editor for an admitted conflict and inspect all relevant
terms. Base, left/right or multiple terms are identities, not a rule that the
last displayed side wins. Some conflicts are not ordinary textual conflicts;
follow the available typed resolution controls.

A resolution command can include multiple paths atomically. Its preconditions
must still match at apply time. If another publication changes those paths,
reload the conflict and prepare a new intent rather than replaying a stale
replacement under a different request ID. Keep unresolved terms visible until an
accepted resolution actually selects the new state.

Review comments about a conflict are collaboration metadata. Resolving or
editing a comment does not resolve the code conflict, and resolving code does
not erase the review history.
