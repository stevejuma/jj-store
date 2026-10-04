# Reads, search and refresh

A read session pins an explicit code operation and the relevant domain
selections. Pages and cursors continue within that selection. This keeps a file,
its history and its surrounding metadata interpretable even while other clients
publish.

```mermaid
flowchart LR
  UI[Active application view] --> Session[Authorized pinned read session]
  Session --> Roots[Selected repository roots]
  Roots --> Page[Bounded page and continuation]
  Commit[Committed publication] --> Events[SSE notification]
  Commit --> Adapter[Ordered adapter reader]
  Adapter --> Index[Tantivy generation]
  Events --> Refresh[Refresh active selection]
  Refresh --> UI
  UI --> Query[Snapshot-bound search]
  Query --> Index
```

## Pagination

Continuations are bounded and bound to their issuer and selected state. They are
not general-purpose offsets that can be transplanted to another repository or
new snapshot. If a cursor is stale, start a fresh query or session instead of
concatenating pages from incompatible selections. APIs retain exact identity
widths and lossless integer values; see
[transport semantics](../reference/transport.md).

## Bounded code reads and preparation

Native reads retain a verified immutable catalog root and check membership for
each requested object. They do not enumerate the complete committed inventory
just to open a session. Local Git-backed reads capture the selected operation
and view, then follow the requested directory, references, file or comparison.
Files load direct directory rows before independent header and attribution
reads. Bookmarks and workspaces have separate windows and exact-name lookup.

Commit history uses explicitly prepared, disposable JJ indexes. Preparation has
a separate work budget and one background worker, preserving interactive
capacity. Native and local generations share disk reservations: 4 GiB per
repository and 16 GiB total, including staging. A shared 1 GiB resident-index
estimate is admitted before decoding and retained by active readers. This is an
index estimate, not a process-RSS limit. Derived caches never select code or
bypass current authorization.

Local prepared log, commit detail and evolution reads reuse operation-bound
visibility and annotations. Cold reads report rebuild-required for the exact
selection. Cancellation remains pending until the worker exits. Cache loss,
expiry or restart can require preparation again. Deep historical/prefix and
operation lookup, native visibility reuse, collaboration/card projections and
full large-scale/fault qualification remain ongoing work; ordinary bounded read
results do not qualify the whole large-repository plan.

## Search as a projection

Tantivy indexes are rebuildable adapters. The ordered committed reader follows
selected publication records and durable checkpoints. Replaying a record must
produce the same projection without selecting repository state or executing
captured tools. Search reports freshness against the requested snapshot and
index generation. A current-looking result list is not proof that a just-saved
object is indexed.

Index preparation is explicit and bounded. Unavailable, warming and stale states
belong beside the search controls. Native and filesystem repositories have
different capability gates; local browsing does not imply indexed local search.
[Search](../guides/search.md) describes the user workflow and supported queries.

## Keeping active views current

Committed saves, live events, reconnect and bounded periodic checks refresh
active views. An explicit historical selection stays pinned. Draft text and
exact pending commands survive refresh. This distinction lets users read old
state deliberately without losing unsaved input when current state changes.

SSE is a freshness signal, not a mutation acknowledgement or authority channel.
After a signal, the client obtains an authorized selected state. Reconnection
cannot safely infer the contents of missed events, so it triggers a bounded
refresh. Session-only notices report operational conditions and clear on actual
recovery, rather than treating a retry as success.
