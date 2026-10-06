# Authorized replacement uploads

A bounded binary upload stages file content under current repository write
permission. Its immutable V1 binding records repository, principal, the exact
single-head snapshot, target commit, path and conflict fingerprint. The upload
handle is a domain-separated SHA-256 digest of the canonical binding plus the
server-computed native file ID and byte length. Records live under the
repository/epoch/principal staging namespace, separate from native objects and
manifest authority. Native RPC cannot write that namespace. Knowing a native
file ID or another principal's upload handle does not authorize reuse.

Upload admission validates path shape, current snapshot and committed target
membership before consuming bytes. The bounded stream is spooled privately;
short declared bodies, over-limit content, deadlines and authorization changes
fail explicitly. Current authorization and snapshot are checked before remote
writes and completion. Repeating identical bytes with the same binding returns
the same handle, including after lost acknowledgement or server restart. Staged
records are not committed history, have no listing API and grant no read access.
They remain unusable after the snapshot advances; destructive staging cleanup is
outside this release.

UploadedFile resolution supplies the checked handle, an explicit executable bit
and an optional copy_from_term position. The latter must select a native file
term; omission uses the native copy placeholder. No caller-selected native file
or copy ID is accepted. Resolution checks the record's integrity and complete
scope, verifies the native file bytes and validates every selected path before
rewriting. Uploaded bytes may resolve text, binary or structural conflicts into
an ordinary file; selecting symlink/submodule/directory terms remains separate.

A new request with a stale binding publishes nothing. Identical selected
mutation receipt replay precedes upload lookup, so checkpointing and later
staging loss do not invalidate an acknowledged mutation. Native report and prior
command receipt encodings remain unchanged. General purpose and binary editors
remain deferred under M4.3.
