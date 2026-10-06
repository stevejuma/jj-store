# Native abandon

Abandon accepts bounded distinct full commit IDs plus explicit retain_bookmarks
and restore_descendants options. Committed targets that are already hidden are
reported as skipped. No visible targets returns no_changes without publication;
root and protected transitions are denied. An absent/uncommitted target fails
the whole request. Native descendant transformation abandons the visible
targets, rebases descendants by default or reparents them while preserving
content when restore_descendants is true, and updates bookmarks and all logical
workspaces. Retaining bookmarks moves their native targets to replacement
parents; otherwise abandoned bookmark terms are removed using pinned native
semantics. Abandoned working-copy changes are replaced with new empty changes,
shared across workspace mappings that previously pointed to the same commit.
Physical files are untouched.

Reports include abandoned and skipped_hidden IDs, descendant rewrites and exact
bookmark/workspace changes. The two added report fields default to empty and are
omitted when empty, preserving canonical decoding/reencoding of earlier V1
receipt results. Objects and predecessor/evolution dependencies remain retained.
Preview remains unpublished and apply selects one guarded operation and receipt.
