# Native Tantivy extraction

The service supplies a read-only native bridge over the adapter's verified
inventory. Selected history and prior snapshots cannot extend beyond the
delivered position. Genesis includes only the root operation's dependency
closure. Current repository authorization and the adapter worker context remain
attached to all reads; no raw store or publication capability escapes.

Real native fixtures index retained operations and commits, including invisible
predecessors. Selected receipts provide authenticated actors independently of
native username, host and attributes. Bootstrap and incremental extraction
derive the same known publication origins from selected inventory differences.

Source indexing requires explicit bookmark or workspace scopes. Native diffs
supply changed and deleted paths. Includes/excludes run before content reads;
binary, non-UTF-8, oversized, symlink, submodule and unresolved file values are
excluded. Conflicted, missing, disabled and multi-head scopes report their state
and cannot select arbitrary source content. Indexing publishes no repository
mutation, including when encountering concurrent operation heads.

Local schema 2 fixes a demonstrated exact-ID regression: unchanged source files
must acquire the new scope commit/operation context without replacing every file
document. Queries derive this context from the pinned commit payload; exact
commit/operation-ID queries include matching scope terms. Old pinned readers
retain their original context. Schema-1 local artifacts rebuild. Native object,
WAL, receipt and remote adapter cursor formats remain unchanged.

The native replay fixture compares incremental and full-rebuild text/metadata
hits, deletion, exclusion, pinned readers and actual token revocation. A
separate fixture verifies clearing source content under multiple operation heads
and disabling source indexing while retaining searchable native metadata.

Initial failures included missing APIs, fixture compile errors, an inconsistent
copy path in the existing codec fixture and the exact-ID regression above. The
shared codec fixture remains unchanged and native consistency checks stay
enabled; search publishes a valid copy lineage for its nested path. The
multi-head test initially omitted its removed head and correctly failed service
validation; naming its original parent fixes the fixture.

Commands, named tests, failure counts and aggregate results are recorded in
E-M6-003 of the [single ledger](../../spec/src/milestones.md). This evidence
uses real native JJ objects, real local Tantivy files and authenticated
memory-store publication. It does not qualify real RustFS/TLS deployment, HTTP
search, remote snapshots or measured operational limits. Current-visible
filtering, serving integration and those operational gates remain open. UI work
remains deferred.

Final validation: 327 Rust tests passed, zero failures and six explicitly
ignored external tests. Normal checks, build, all 13 specification lanes and
formatting passed. The native extraction slice is verified; M6.2 remains in
progress.
