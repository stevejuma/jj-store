# Local Tantivy generations

The Tantivy projection stores deterministic operation, commit and scoped-file
keys. Each update copies a bounded committed local generation into a private
staging directory, applies deterministic delete/upsert terms, then prepares a
Tantivy commit whose payload binds adapter identity, exact source position,
source-scope snapshots and the previous artifact. Tantivy opstamps are not
source positions. Files and the complete file descriptor are synced before an
atomic local active-pointer replacement; the remote runner cursor follows
recoverable local state. Crash orphans may be discarded and replayed.

Recovery verifies descriptor checksums, commit payload, schema and actual index
readability. Missing or corrupt derived files request rebuild. A reader-load
failure after pointer replacement leaves the durable checkpoint ahead of the
served generation and reports degradation; recovery must retry loading it.
Published generations are immutable and held by reader references. Retired files
are removed only after their last reader exits. Configuration changes build a
new generation without changing existing readers. Disk-copy, document, file,
query and result budgets are explicit; this first implementation trades bounded
local copying for simple crash-safe reader isolation.

Core search pins an immutable generation and rechecks repository authorization
and epoch before and after every request, including pagination. It exposes no
unauthorized counts or stored content. HTTP cursors, serving configuration, cold
process recovery and the pinned RustFS deployment are qualified separately from
the storage-engine fixtures.

Native extraction can read bounded selected history and earlier complete
inventories through the delivered snapshot. Both stop at that snapshot's
position even when the captured manifest contains later publications. Genesis
uses only the verified immutable root-operation dependency closure.
Authenticated actors come from selected receipts, distinct from native user/host
metadata.
