# Selected native view search

The native content reader now supplies complete commit visibility at its
selected operation. It reuses the bounded native parent traversal used by
operation differences. The service returns an immutable filter with the exact
read snapshot and commit set, retaining no credential or live session.

Search requests must explicitly choose historical metadata or visible commits.
The latter intersects native visibility with commit results before ranking and
pagination. Hidden hits cannot consume a page. Operation metadata remains
historical and source files retain their explicit indexed scope snapshots. A
constant-score filter does not change text relevance scores. The internal
document-class pattern is fixed; user text is never a regular expression.

The selected view and index generation are independent and both appear in the
result. A newer view can filter a stale generation without inventing missing
results. Old view/generation pairs retain their meaning after new publications.
Native IDs, local Tantivy schema 2, WAL, receipts and remote cursor formats are
unchanged. HTTP search integration and signed transport cursors remain pending.

Three new native fixtures cover hidden commits, retained historical results,
filtering before pagination, a native all() revset oracle, predecessor-only
history, limits, zero read publications, later writes, lag, revocation and
foreign repository/epoch filters. Existing Tantivy durability fixtures
explicitly select historical mode and retain their original assertions.
Visibility traversal returns a complete set or an error; it never returns a
partial set.

The initial named target failed with 26 compiler diagnostics for the absent
visibility API and request types. One subsequent compile failed on an ambiguous
fixture Limits import; an explicit import fixed it. Actual commands, results and
remaining M6.2 gates are recorded in E-M6-004 of the
[single ledger](../../spec/src/milestones.md). UI work remains deferred.

Final validation: seven native projection fixtures and ten Tantivy durability
fixtures pass. The full suite passes 330 Rust tests, with zero failures and six
explicitly ignored external tests. Normal checks, build, all 13 specification
lanes and formatting pass. M6.2 remains in progress for serving, snapshot and
operational integration. No UI sources or visual baselines changed.
