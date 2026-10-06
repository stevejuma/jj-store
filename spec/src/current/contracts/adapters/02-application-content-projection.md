# Application content projection

Application content search builds a bounded in-memory projection directly from
one manifest-selected Thread/WorkStore snapshot. This is rebuilt on an explicit
new query and is never repository authority. It has independent five-minute
query-bound continuations, current-viewer checks and epoch invalidation.
Matching uses current full bodies while returned excerpts are bounded. Neither
historical revisions nor tombstones are searched as current content. An empty
query selects recent active Threads, projects and tasks. Extraction requires an
eligible section/scope and a selected domain root; recent Thread previews use
accepted message activity without exposing historical revisions. Durable Tantivy
acceleration for this projection is not required for this bounded native
profile; code search retains its existing index, readiness and operation-pin
contract.
