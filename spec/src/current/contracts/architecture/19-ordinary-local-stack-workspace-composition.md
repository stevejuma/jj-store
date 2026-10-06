# Ordinary local stack workspace composition

Local Stacks execution is a jj-store-jj provider reached through the existing
registered-folder service boundary and LocalTrusted HTTP admission. Its physical
workspace owns `.jj/jj-store/stacks.toml`, and normal JJ bookmarks retain merge
and WIP commits. Browser presentation consumes a small versioned local state
contract plus existing operation-pinned browsing. The ordinary local profile
supersedes the stack-specific installation catalogs/retained executor as a
production dependency. Native manifest-backed publication is unchanged and is
not a prerequisite for this on-disk feature.

Selected-file insertion/squash and its nonpublishing preview belong to this same
local provider. UI staging retains only intent; the local operation and source
commit guard execution. Native JJ owns descendant propagation and empty WIP
retention, without a second merge-reconstruction service.

The ordinary local Stacks mutation profile now includes native bookmark deletion
(top bookmark only; lower bookmarks and commits retained), abandonment, selected
whole-file/native-hunk WIP discard, and complete native text-conflict
resolution. Conflict terms are read lazily through guarded POST
`stacks/conflict`; ordered three-term text conflicts use the existing Merge
Editor. Multi-sided, structural, mode-conflicted and oversized conflicts retain
native values and report that JJ resolution is required. Conflict paths are
bounded to 50 per current read.

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

The production Stacks controller derives dependent segments from bounded native
parent links and bookmark facts, retaining the advertised top-range cursor.
Per-segment tree comparisons require a proven lower boundary. Applied lanes and
their adjacent diffs retain independent UI identity even when they share native
commits. This presentation derives no new repository authority and triggers no
repository-wide history load.
