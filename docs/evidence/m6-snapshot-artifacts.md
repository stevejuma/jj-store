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

These are deterministic storage-fixture results, not RustFS conformance. Tantivy
export/restore and foreground scheduling are still separate work: the engine
must pin a committed generation, upload exactly its files, and validate restored
payload, schema and readability before activation. Missing or unusable snapshots
must still permit native-history rebuild. M6 remains In progress.

[E-M6-009 in the single ledger](../../spec/src/milestones.md) records commands,
results, pins, format additions and remaining gates in the enclosing change.

Final validation passed: 22 adapter fixtures (including eleven new snapshot
fixtures), 27 native search/projection fixtures, ten Tantivy durability
fixtures, 368 Rust tests with zero failures and six external fixtures ignored,
all 13 specification lanes, normal checks, build and formatting. No UI feature
or visual baseline changed. The pre-existing RustFS harness remains live and
stalled in Docker volume creation; no duplicate harness was launched.
