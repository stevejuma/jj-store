# Graph and directory continuations

Native RPC exposes the same typed collaboration command, durable status, page,
Thread detail, exact human-message and retained-image reads. Request actors and
publication identities remain service-derived. JSON is bounded to four MiB;
custom raster bytes travel in separate two-MiB protobuf fields to avoid JSON
array expansion. Image metadata, bytes and requested content identity must
agree. Current peer/token admission applies to every call; replicated code
writes stay gated on reducer 3 and WorkStore homes retain their frozen origin.

Thread, Thread-event, exact-target Reaction and custom-emoji pages use a
separate bounded credential-free collaboration registry. Signed tokens bind
every captured source root, repository, viewer, query and page size. Object
count, encoded collaboration bytes, pin capacity, fixed expiry, token size and
request concurrency remain bounded. Expiry or process replacement returns an
explicit cursor-reset; publication does not change a resumed page's snapshot.
Every resumed query authenticates its current request, and no raw continuation
crosses the public response boundary.

The service signs opaque V1 read cursors with a process-local HMAC key and
retains a bounded, expiring snapshot pin without credentials. The signature
binds repository, epoch, operation, publication position, manifest revision,
query kind and inputs (including workspace/time context or commit/directory
path), native traversal ordering, page size and last node ID or directory entry
name. Every page authenticates its current request independently. Resuming
renews only the request deadline; the captured inventory and snapshot remain
identical even after publication or checkpointing.

Expired pins and server restarts return an explicit cursor-reset result. They
never fall back to latest. Invalid signatures or changed query bindings fail
explicitly. Pin count, retained inventory roots and head identities, request
concurrency and token size are bounded. Pins do not retain request-local
membership caches. The process-local registry requires sticky routing in any
future multi-instance deployment; the first release serves one instance.
