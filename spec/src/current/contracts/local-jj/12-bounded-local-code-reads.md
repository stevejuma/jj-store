# Bounded local code reads

Progressive file Blame uses the shared JJ annotation adapter and existing scoped
operation-pinned backend with the prepared core index. Source metadata and
working-copy state remain unchanged. The user accepts JJ latency and cooperative
cancellation at read/comparison boundaries as specified in `jj-integration.md`.
Full authorization and source validation remain mandatory for retained progress.

Operation/view-only requests MUST prove the selected operation is retained and
stop ancestry discovery once that proof is available. Query information,
operation detail and bookmark-only pages MUST NOT load a commit graph, build a
JJ index, or traverse historical trees. Historical selections remain bound to
current retained operation ancestry; raw Git object existence is insufficient
for membership.

Bookmark pages without a search predicate MUST select their ordered window
before reading target commits. Hydration reads only targets referenced by the
selected view, remains within the request byte/work/deadline limits, and reports
exact conflict terms and workspace links. Description search retains complete
matching semantics; bounded preparation for wider queries is a separate stage.
Existing local publication, identity and persisted formats remain unchanged.

LocalTrusted core reads reuse ready standard-index membership for selected
commits, directories, file content and diffs. Scoped content keeps its bounded
cold reader; ordinary history and full commit detail request core preparation
when absent. Existing explicit advanced preparation remains a separate path.
Current folder authorization and registration generation are checked for start,
status, cancel and reads, including after restart and external changes.

Ordinary code history uses the managed standard JJ index described in
`jj-integration.md`. Bounded source-index copying is permitted for warm reuse;
source index repair and source metadata writes remain prohibited. Indexed
history retains the exact selected operation and registration generation. Custom
disk-backed construction and full projections below apply to explicitly prepared
advanced queries, not ordinary code browsing.

Local commit-index construction MAY use the shared disk-backed JJ format-6
builder through profile-specific admitted readers. Starting from a retained
operation proof, traversal follows only operation parents, referenced commit
roots and commit parents; raw Git object existence grants no new roots. Each
operation, view and commit read rechecks the registered binding and per-object
byte/shape limits. Preparation does not load trees, decode the repository's
derived index, reconcile heads or publish native/companion state.

The trusted caller reserves a private disposable output allowance before
invoking construction. Scratch and streamed output share that allowance. One
shared preparation permit remains held through actual cancellation/worker exit.
Construction has separate cumulative work, bytes, metadata and deadline limits;
it never increases ordinary interactive limits. Failed output is deleted and a
successful handle owns its disposable files. Library handles serve prepared log,
commit detail and evolution pages; the service owns retained generations and
task delivery. Large-scale qualification remains a separate gate.

The server explicitly opts registered local readers into its retained runtime
policy. New reads and preparation jobs capture the current interactive limits;
already admitted work retains its original limits. Standalone query consumers
keep their explicitly supplied bounds. Preparation's cumulative byte allowance
is separate from the captured per-object maximum and MUST NOT be passed as that
maximum. Reload does not change retained operation/registration selections,
repair source metadata or publish code/companion state. JST-LOCAL-003/004's
authorization, identity and operation-pinning requirements remain unchanged.

Preparation MUST NOT invoke GitBackend's implicit extra-metadata import or
stacked-table reconciliation. It reads compact commit fields from Git and
performs bounded read-only seeks in captured, checksum-verified JJ 0.45.1 extra
metadata segments. Preserve imported and explicit change IDs, shallow roots and
JJ's commit timestamp ordering. A missing extra record uses upstream's header or
synthetic change ID without importing it. Segment disagreement, invalid framing
or identity mismatch fails explicitly; no tree bytes are needed for indexing.

A prepared local log page MUST bind the exact repository, registration
generation and operation from construction, and create fresh authorized source
readers for each request. It may reuse a verified immutable resident index;
membership comes from that operation's constructed closure, never arbitrary Git
object existence. Returned commit records preserve JJ signatures, conflict tree
terms/labels, legacy predecessors and extra change IDs through a read-only
decoder. No extra-metadata import, source index repair or publication is
allowed. Interactive work/byte/row/deadline limits remain unchanged. Syntax
admission must not multiply query length by total repository commit count.
Retained generation and index-memory admission, task delivery and deep-page
qualification remain explicit independent gates. Resident admission shares a
process-wide 1 GiB allocation allowance with native graph indexes. It
conservatively charges twice the index segment bytes plus retained
metadata/shallow-root overhead before decoding. Charges live until the last
handle is released. Verify the complete bounded segment and fixed operation link
before decoding; an unexpected upstream rebuild must fail through a view-denying
capability. Prepared pages consult the memory copy and must survive damage or
removal of their disposable index files. Source extra segments remain read-only
retained handles checked before and after seeks for size, identity and timestamp
changes. Coarse or unsupported timestamp proofs require bounded full checksum
checks; changed segments fail explicitly even if mtime is restored. No earlier
caller's credentials are retained.

A filesystem watcher invalidating physical admission during its inspection may
trigger one additional physical identity/generation check before the HTTP
boundary admits the operation. This bounded recheck dispatches no command and
replays no mutation. A changed identity/generation or continued invalidation
still fails closed and publishes no stale admission entry.

Session workspace inspection under JST-LOCAL-004/005 binds an authorized saved
agent workspace and uses these same typed operation-pinned readers for immutable
files, parents and references. Bounded physical scans add current tracked and
nonignored untracked observations without snapshotting, publishing companion
metadata, or changing operation heads. Comparison cursors bind exact observed
selections; changed observations require refresh. Missing turn-start evidence,
ambiguous parents and unestablished comparison bases remain unavailable. A read
capability does not qualify portable file-review publication under M13.3.

Workspace inspection uses the same bounded physical folder admission as ordinary
repository browsing, including fresh boundary checks and a single retry for
watcher invalidation. Inner traversal checks do not reopen the collaboration
store. An unchanged checkout operation and workspace name retain the exact
immutable view selection, so completion may revalidate the checkout without
another CLI process. Physical hints watch authorized owned roots without
following symlinks and ignore access events. At most 128 session/repository
watches are retained per controller; session cleanup and shutdown release them.
Matching hints invalidate current UI reads through the existing application
feed, without periodic browser requests or changes to publication authority.
