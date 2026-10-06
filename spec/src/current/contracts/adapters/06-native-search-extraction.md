# Native search extraction

Native extraction indexes complete retained operation/commit metadata, including
invisible predecessor commits. Selected receipts supply authenticated actors;
native user/host fields remain separate. A commit first introduced by a
publication with exactly one new operation records that operation as its known
publication origin; ambiguous origins remain absent. Bootstrap and incremental
replay derive this association from the same selected inventories.

Source indexing requires explicit named bookmark/workspace scopes. Multiple
operation heads, conflicted bookmarks, missing scopes and disabled source
indexing are reported without selecting an arbitrary target. Canonical config
hashes include include/exclude path prefixes and resource limits. Binary,
non-UTF-8, oversized, symlink, submodule and conflicted file values are
excluded; ordinary UTF-8 files use exact native file IDs, modes and copy
metadata.

A native tree diff supplies deleted/changed paths. Unchanged file terms stay in
the index while the commit payload advances their exact scope commit/operation.
Hit metadata and exact-ID scope queries use that pinned payload context. Local
schema version 2 removes stale per-file commit/operation ID terms; earlier local
indexes rebuild. This changes no authoritative native or adapter cursor format.
The owned content bridge retains current authorization and the adapter context
through native workers. Failed extraction cannot advance the index or cursor.
