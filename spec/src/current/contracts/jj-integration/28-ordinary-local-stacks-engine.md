# Ordinary local Stacks engine

JST-JJ-003's mutation boundary for local Stacks uses pinned jj-lib Workspace,
normal settings/ignore rules, native working-copy mutation locks, snapshot,
transactions, merge_commit_trees, rebase_commit, descendant rewrites and native
checkout. `.jj/jj-store/stacks.toml` retains only local
presentation/configuration and bookmark names. Stable WIP change identities
follow normal JJ rewrites. This path does not use retained operation archives or
custom op-head/tree-state publication. A different external working change
detaches active selection and retains the old named WIP; later selection
restores it. Concurrent edits cause stale command rejection. Native conflicts
remain native conflict trees.

Active workspace settings use the existing native build/rebase path only when
the target changes; renaming preserves JJ identities. Deletion removes local
configuration and retains the native WIP, checkout and retention bookmarks. Both
commands validate the shown operation/configuration revision under the existing
settings and working-copy locks.

Selected-file commit creation inserts after an applied head, rebases
descendants, then squashes from the current rewritten WIP with `keep_emptied`.
Selections bind exact source commit IDs and canonical paths. The merge and WIP
retain their change identities through native rewriting; preview validation does
not publish or snapshot. Target ancestry and configured immutable descendants
remain protected.

Basic local Stacks actions use native bookmark targets, commit descriptions and
new --insert-after/--insert-before equivalent rebases. Above/below bookmarks
change references only. Point inspection admits at most 500 exact visible IDs;
configured immutable descendants and workspace target ancestry protect rewrites.

The first commit on a bookmark still pointing at the workspace target MUST fork
from that target without rewriting unrelated target children. Explicitly
retarget only the retained merges applying that bookmark; later commits use
normal insertion and descendant propagation. Empty Commit selections create an
empty commit. Refresh must not reconstruct these already-correct parent sets.

Local selected-file Transfer, Split and Edit use CommitWithSelection and
squash_commits with native descendant rebasing. Insertion resolves current
visible source change identities before squash, so rewritten source IDs cannot
accidentally select another commit. Empty WIP and source changes remain
retained.

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

Branch rebases move the segment root with descendants. Single-revision moves
detach ordinary children as JJ does; if this detaches a saved applied edge,
native rebase restores that edge on the existing retained merge change. This
preserves its identity and WIP delta without constructing a replacement merge.

The ordinary local Stacks mutation profile now includes native bookmark deletion
(top bookmark only; lower bookmarks and commits retained), abandonment, selected
whole-file/native-hunk WIP discard, and complete native text-conflict
resolution. Conflict terms are read lazily through guarded POST
`stacks/conflict`; ordered three-term text conflicts use the existing Merge
Editor. Multi-sided text conflicts expose bounded ordered versions for pairwise
comparison in the same editable Merge Editor. The user reviews every version and
explicitly stages the complete resolved file before preview. Structural, mode,
binary and oversized values retain explicit unsupported reasons. Conflict paths
are bounded to 50 per current read.

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
