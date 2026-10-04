# Committed adapter delivery and durable checkpoints

The adapter crate owns a read-only source, projection lifecycle, local writer
lease and conditional progress record. WAL exposes exact selected record
references and record-specific complete inventories. Captured manifests stay
pinned across writes/checkpoints; orphan candidates and later inventory entries
cannot enter an earlier batch. Bootstrap verifies the current tail and receipt
before exposing its complete native snapshot.

A checkpoint binds repository, epoch, committed position, exact WAL attempt and
digest, adapter name/implementation/schema/configuration, and a
destination-owned durable artifact descriptor. It uses canonical V1 JSON under
the repository's adapter cursor namespace. Local artifact recovery is checked
against selected history. A remote cursor cannot substitute for local files.
Missing or incompatible files trigger bootstrap; restoring an older compatible
artifact replays locally without regressing a newer remote cursor.

Destination effects precede cursor CAS. The runner requires the projection to
recover the same durable checkpoint before updating progress. Lost cursor-write
acknowledgements are checked by readback; contention remains an explicit error.
Failures leave the record available for replay, report degraded health and use
bounded polling/backoff. Repository publication never calls an adapter. This
provides at-least-once replay, not distributed fencing or exactly-once external
effects.

A local lock owns the repository/adapter writer. Run contexts carry that lease,
admission, deadline and cancellation through background work, so a timed-out
worker cannot overlap a replacement owner. Page, inventory, request, byte and
per-object caps bound source work. Current service token authorization and epoch
checks run at storage boundaries and before returning verified content. Source
objects must belong to the selected inventory and match its envelope checksum.
Native RPC cannot address the adapter cursor namespace.

Nine fault tests use the production coordinator and a projection fixture that
writes, syncs and atomically installs real local artifacts. They cover orphan
exclusion, selected order, nonzero-position bootstrap, checkpoint/restart,
missing/older artifacts, changed configuration, poison records, before/after
artifact and cursor failures, unrecoverable acknowledgements, corrupt native
content/tails/cursors, revocation, resource limits, polling/cancellation and
writer ownership retained by delayed work. A separate native JJ test decodes
committed objects through the service's read-token binding, rejects staged
content and verifies revocation. That small native authorization fixture is not
used as proof of destination durability.

The new bootstrap-corruption regression initially showed a projection could
acknowledge a manifest whose selected tail was corrupt; selected-tail/receipt
verification now rejects it before destination effects. One test-edit mistake
removed the artifact-loss step while retaining a rebuild assertion; restoring
that step retained both restart and rebuild expectations. Exact commands and
before/after counts are in E-M6-001 of the
[single ledger](../../spec/src/milestones.md).

This evidence establishes the generic runner boundary. Tantivy commit payloads,
reader generations, index snapshots, search authorization/freshness and serving
integration require their own M6.2 acceptance. Real RustFS/TLS deployment
qualification remains separate. No UI or visual-baseline changes are included.

The repeated specification Rust lane exposed an RPC admission test race after a
passing full test run. The fixture now connects its observer first and waits for
a server-side authorization read under the upload permit before testing the
competing request. This prevents polling from winning that permit itself. The
400 ms deadline, exhaustion, deadline-error and permit-release assertions are
unchanged; focused acceptance and the complete seven-test RPC suite pass.

Final aggregate verification passes: 312 Rust tests, zero failures and six
explicitly ignored external deployment tests. Normal checks, build, all 13
specification lanes and formatting pass after the RPC fixture repair. The M6.1
audit maps ordered selected delivery, complete source identity and recoverable
artifact checkpoints to JST-ADAPT-001; poison isolation, replay and durability
before cursor advancement to JST-ADAPT-002. These generic runner requirements
are complete. Tantivy-specific requirements remain open in M6.2.
