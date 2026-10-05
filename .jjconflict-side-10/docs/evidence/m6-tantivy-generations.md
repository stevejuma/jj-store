# Tantivy local durable generations

The projection consumes selected adapter snapshots/batches through an owned
extractor interface. Its tests use synthetic extraction and actual Tantivy
0.26.2 index files with the production adapter runner and WAL coordinator. This
is destination durability evidence; native JJ extraction and serving integration
require additional acceptance.

Operation, commit and scoped-file keys determine delete/upsert identities.
Changing source-scope snapshots requires explicit scope replacement. Conflicted
or disabled scopes cannot publish file documents under an arbitrary target.
Scope replacement deletes stale file terms even when no native file was
uploaded. Stored results retain provenance/identity metadata and omit indexed
source bodies.

Each update copies the selected committed files into a private staging directory
using bounded streaming checksums. Tantivy prepares a commit carrying exact
adapter/source identity, previous artifact, scope snapshots and update digest.
Synced index files and a checksummed descriptor precede atomic active-pointer
replacement. The adapter cursor advances only after that artifact recovers.
Before-pointer faults leave replayable source work; after-pointer faults recover
the committed payload. Durable position and served-reader position remain
separate when reader publication fails.

Readers retain immutable generation files. Fresh authorization and epoch checks
surround each query, including old-generation pagination. Literal token queries
have bounded clauses and support canonical exact native IDs. Query admission,
page size, output, timeout, document bytes/count, update bytes, file count,
generation size and retained-generation count have explicit limits. Workers
retain adapter context or query admission until they exit. Exhausting retained
generations stops indexing without advancing the cursor; releasing old readers
allows replay. Crash leftovers are reclaimed under exclusive local ownership.

Ten fixtures cover before/after Tantivy commit, local pointer and reader
publication, before/after remote cursor writes, restart without repeated
effects, deleted/corrupt index components and descriptors, configuration
rebuild, restored older generations, scope changes, concurrent pinned readers,
writer exclusion, resource bounds and revocation before and after query
execution. They do not claim a real process-kill/RustFS deployment result.
Restore currently uses a fixture copy of a complete local generation; remote
snapshot upload/restore is not implemented by this slice.

The initial target failed because the crate did not exist. Compile corrections
adapted to Tantivy 0.26's collector API, fixed extractor thread bounds and test
type annotations. The first lint run rejected an oversized private function
signature; grouping commit inputs resolved it without suppressing checks. Actual
commands, failure counts and remaining gate work are in E-M6-002 of the
[single ledger](../../spec/src/milestones.md).

This first implementation copies a bounded generation for each update and does
not merge segments. It deliberately returns explicit resource exhaustion when
the configured file/disk budget is reached. Native extraction, current-visible
history filtering, HTTP cursors/freshness, server wiring, remote snapshots and
measured performance remain required before M6 can close. UI features remain
deferred.

Final aggregate verification passed: 322 Rust tests with zero failures and six
explicitly ignored external deployment tests; normal checks, build, all 13
specification lanes and formatting passed. No UI sources or baselines changed.
The engine slice is verified; E-M6-002 remains pending against the complete M6.2
gate because the integration and operational acceptance listed above are open.
