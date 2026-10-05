# Snapshots and publication

The application distinguishes a selected immutable snapshot from the current
head of a repository. A snapshot makes a multi-request view coherent: files,
parents, diffs and history pages refer to the same selected operation instead of
silently moving between saves.

## Native publication

Only successful manifest compare-and-swap selects native repository state.
Objects may be uploaded and a WAL candidate may exist before publication; those
facts alone do not make the candidate visible. A read follows roots in the
selected manifest, not whatever object was most recently uploaded.

Code and collaboration roots can advance independently. A task comment can be
published without creating a code commit. An explicitly historical code view can
remain pinned while current collaboration information refreshes.

## Unknown is a real outcome

A timeout means the caller does not know whether publication completed. It is
not proof of a compare-and-swap conflict or a rejected command. Request IDs,
prepared intent and durable receipts allow a caller to inspect or retry the
original operation safely after reconnection or restart.

A receipt must match the original request and principal. Reusing the ID with a
changed body fails. A `not_known` response means no selected receipt is visible;
it does not prove the original operation can be discarded. Keep pending drafts,
input documents and recovery journals until the result is resolved.

## Refresh and historical browsing

Committed events, reconnection and bounded periodic checks refresh active views.
An event is a freshness hint, not an authoritative replacement document. The
application reopens authorized reads against a committed selection. It retains
explicit historical code choices and pending command identities.

Pagination belongs to its selected snapshot. A native opaque cursor is not an
offset that can be reused under a different repository, principal or operation.
Local selectors explicitly identify companion metadata. If a continuation is
stale, start a new read; do not combine pages into a purported single snapshot.
