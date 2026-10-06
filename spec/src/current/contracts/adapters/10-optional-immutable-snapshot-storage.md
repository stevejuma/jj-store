# Optional immutable snapshot storage

The adapter boundary owns an authorization-bound snapshot capability constructed
from a selected source snapshot. It exposes streamed immutable blobs and a
versioned bundle of canonical file names, byte lengths and SHA-256 checksums.
Keys are derived from repository, authority epoch and the complete adapter
identity; projections receive no arbitrary object-store path or mutation API.
Every transfer consumes the retained run context's request, byte and deadline
budgets and checks current repository authorization before and after I/O.

Snapshot publication verifies every referenced file, uploads the immutable
bundle descriptor, and conditionally selects the descriptor last. Selection
binds the exact committed source reference, configuration and projection
artifact. A lost pointer acknowledgement is resolved by checked readback; a
competing or later pointer cannot be silently overwritten. Snapshots never
update the repository manifest or the adapter replay cursor. File names reject
path traversal; bounded descriptors reject unknown fields and noncanonical
encoding. Missing, corrupt or incompatible snapshots are unusable acceleration,
not evidence that local index files have survived. Tantivy restore must still
verify its payload, schema and actual index readability before activation.

The foreground runner restores a compatible selected snapshot before native
bootstrap. Tantivy downloads every declared generation file into private
staging, verifies file lengths and checksums, canonical descriptor bytes,
adapter identity, schema, source position, commit payload, document limit and
actual index readability, then atomically installs the original generation and
active pointer. An older compatible generation catches up through ordinary
selected batches. Missing or unusable snapshots rebuild from native history and
repair the remote pointer; repository publication remains independent.

`snapshot_every` schedules optional exports by committed source position; zero
disables them. Export pins the active committed generation and streams only its
declared files through the selected snapshot capability. Optional snapshot I/O
failure reports degradation and is retried without failing repository writes;
revocation and cancellation remain fatal to the worker. Transfer budgets derive
from the configured per-generation index budget.

The pinned single-node RustFS/TLS deployment deletes all local server and search
state, restarts both RustFS and the server, and requires the exact generation
digest and search results to return. Its recorded cold rebuild was 5.436 seconds
and exact restore was 2.007 seconds, both within the published 30-second test
budget. This measured fixture proves the restore path avoided a rebuild for its
bounded repository; it is not a general throughput claim.

Local Search Apply drains only the selected repository worker. Replacement
projections are built and verified in owner-held temporary roots before the
serving backend changes. Failed replacements resume the previous worker and
retain its effective configuration. Search cursors keep their original readers
across apply or disable; new queries use the active backend. Temporary roots
live until the last projection or reader owner releases them. Replacement
concurrency is bounded and uses the existing runner recovery path. Retry
restarts the effective runner; Rebuild bypasses cached adapter snapshots and
reconstructs selected native data. Startup reopens the saved desired
configuration.
