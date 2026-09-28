# Authorized immutable snapshot artifacts

The adapter capability now streams optional snapshot files into immutable,
content-addressed storage. Repository, authority epoch and the full adapter
identity derive its storage namespace. A projection cannot supply arbitrary
object keys. The capability retains the selected source boundary, current
authorization and the runner's existing ownership, deadline and transfer
budgets.

Publication verifies the complete file set, writes a bounded canonical bundle
descriptor, and selects it with a conditional pointer write. Checked readback
resolves lost acknowledgements. Concurrent candidates preserve exactly one
complete selection; stale publishers cannot move the selected position back.
Neither the repository manifest nor the replay cursor is changed by these APIs.

The acceptance fixture failed before implementation with four missing API/type
diagnostics. The original target then passed. Expanded fixtures cover missing
and corrupt files, descriptor and pointer corruption, each write stage's lost
acknowledgements, actual concurrent pointer candidates, configuration/epoch
isolation, revocation before and during reads, exact committed references,
unsafe names, shared byte/request budgets and canceled or expired streaming
work. A download verifies its private spool before copying into caller staging;
a revocation during download returns no content to that staging writer.

The Tantivy integration pins a committed generation, uploads exactly the files
declared by its checksummed descriptor and conditionally selects the canonical
bundle. Restore downloads into private staging and verifies the complete file
set, adapter identity, schema, source position, Tantivy commit payload, document
budget and actual reader construction before atomic installation. An older
snapshot replays selected batches forward. A checksum-valid but incompatible
snapshot rebuilds from native history, repairs the remote pointer and restores
without extraction on the next cold start.

The foreground runner checks restore before bootstrap and schedules exports by
the configured committed-position interval. `snapshot_every = 0` disables this
optional acceleration. Export failures report degradation and retry without
affecting repository publication. Revocation and cancellation remain fatal;
transfer limits derive from the configured generation budget.

Pinned RustFS deployment acceptance starts a TLS server and native repository,
builds and selects a search generation, kills the server, deletes complete local
server/search state, restarts RustFS, then restarts the server. The exact
generation digest and ordered results return, proving that the measured restore
did not rebuild. Cold rebuild took 5.436 seconds and restore took 2.007 seconds;
both stayed within the explicit 30-second test budget. All nine storage and
server probes passed with zero failures. The immutable report is
[`m6-snapshot-rustfs.json`](m6-snapshot-rustfs.json).

[E-M6-009 and E-M6-010 in the single ledger](../../spec/src/milestones.md)
record commands, results, pins, format additions and closure evidence.

The artifact-capability slice passed 22 adapter fixtures, 27 native
search/projection fixtures, ten Tantivy durability fixtures and its full
repository lanes. E-M6-010 adds the two Tantivy restore fixtures, the exact
server cache-loss regression and pinned RustFS/TLS qualification. No UI feature
or visual baseline changed.
