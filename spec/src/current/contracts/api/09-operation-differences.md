# Operation differences

Operation differences compare an explicitly selected immediate native parent. A
sole parent is the default; merge operations require explicit parent selection,
and the virtual root reports that it has no parent. Another committed operation
cannot be substituted for an immediate parent.

The browser presents these recorded operation parents in a compact searchable
picker beside the workspace selector. Choosing a description or identity sends
the full native operation ID and preserves the pinned snapshot and logical
workspace in the hash; the selector does not widen the set of permitted
comparison parents.

The response distinguishes head changes from visible commit changes. Visibility
is the native commit-parent closure of each view's heads; predecessor-only
objects remain historical dependencies rather than visible commits. Complete
view deltas preserve local and remote bookmarks/tags, conflict terms, tracking
state, per-workspace Git heads, Git refs and logical workspace mappings. Changed
remote views carry complete before/after values, including empty remote views.
The operation's predecessor map preserves absent versus known-empty data.

`GET /operations/{operation}/changes` requires one immediate comparison parent
and pages affected changes by full native change ID. Recorded predecessor edges
take precedence. When no predecessor record exists, a newly visible commit's
native parents are comparison candidates. A single before/after pair is an exact
comparison; multiple candidates require explicit selection. Removed-only
visibility has no invented file comparison. Operations whose view changes no
visible commits return no groups, including metadata-only operations.

Each affected-change continuation remains bound to the selected operation and
parent. Every exact group's `GET /diff/paths` continuation remains separately
bound to that group's full from/to commit identities and the same captured read
snapshot. Clients consume these opaque cursors independently; they do not reuse
a path cursor across groups or substitute an operation ID for a commit ID.

Native-ID order makes set deltas deterministic; this order is not topological
ordering or publication order. Commit visits, storage reads, work, response
bytes and elapsed time are bounded. A difference is complete or fails
explicitly; it never silently truncates history. Equal head sets need no
ancestry traversal.
