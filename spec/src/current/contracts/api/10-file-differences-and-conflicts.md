# File differences and conflicts

Structured file differences use the pinned native merged-tree diff stream. Each
change retains both ordered before/after terms at the target path. Copy ancestry
adds explicit normal/copy/rename source paths and values without suppressing
structural deletions. Multiple source lineages remain explicit. Placeholders
mean no recorded history and never become synthetic objects or shared ancestors.
Bounded source lookup selects the nearest available ancestor on each lineage;
when none is present, descendant lookup supports reverse comparisons. Paths
filter target files by canonical repository prefix.

`GET /diff/paths` pages changed-path summaries independently from selected-file
content. It retains before/after terms and normal/copy/rename sources while
marking content as not requested. The existing complete `/diff` route remains
compatible and supplies bounded content for one selected path. Operation-group
and changed-path continuations support stable anchors and backward windows under
the same signed snapshot bindings as references and recursive paths.

Optional content uses exact native line diffing on bounded UTF-8 text,
preserving CRLF and missing final newlines. Every blob is fully verified before
reading. Binary, oversized, structural, conflicted or multiple-source content
returns an explicit unavailability reason while keeping its structural delta.
Traversal, change count, text bytes, estimated line-comparison work, output and
time are bounded; complete structured responses fail rather than silently
truncating.

Native and local text readers retain the pinned JJ line-diff algorithm and its
defaults. The service's text, comparison-work, output and time caps are separate
admission policy, not upstream JJ configuration. Small comparisons use JJ
directly. Longer files may remove exact common leading/trailing lines and use
ordered unique common lines as anchors before applying JJ to the remaining
spans. The cumulative line-comparison estimate for those spans remains capped;
anchor discovery observes the request's work/cancellation budget. Returned hunks
reconstruct both complete inputs, including CRLF and final-newline state. An
over-budget comparison retains the explicit `complexity` reason.

Conflict descriptors preserve ordered add/remove terms, absent terms, native
values, labels, executable flags, copies and external submodules. A distinct V1
fingerprint binds repository, epoch, native profile, exact commit/change IDs,
path and the ordered labeled terms. A term ID is its zero-based position within
that fingerprinted descriptor, never an object capability. Fingerprints grant no
authority and are recomputed during eventual resolution.

The text editor is supported only for three present, verified UTF-8 file terms
with identical executable mode and copy history, within the configured text
budget. Other conflicts return a typed reason; term selection/deletion remain
separate mutation capabilities. Conflict reads never materialize conflict
markers, write merged trees or invoke a merge tool.

Graph responses include local cache diagnostics: built, reused or rebuilt;
evictions; retained entry count and bytes. Native operation identity remains the
snapshot authority. Rebuilding a corrupt or evicted cache never selects newer
heads or publishes a native operation. Busy graph-cache workers return an
explicit retryable result rather than queuing unbounded work.

Ordinary graph construction and automatic code-index namespaces share the same
bounded cache quota. Either profile may evict an idle managed code namespace
when its slots or bytes prevent admission. A repository byte limit evicts only
that repository's data. Active writer locks and retained ordinary reader locks
remain protected; unavailable capacity still returns an explicit limit. Cache
eviction never changes source membership, operation selection or repository
state (JST-API-001/004).

Dropping an in-flight graph/content request cancels its native worker budget.
Pending authorization and storage waits terminate promptly; synchronous native
steps observe cancellation at their next bounded checkpoint. An already returned
verified file stream remains usable until it is dropped, expires or loses
access.

Query metadata admission scans the verified envelope payload before allocating
native DTOs. It bounds cumulative JSON value/key count, persisted merge arity,
parent/predecessor edges and entries in a tree object. These are request limits:
exceeding them returns a resource-limit error without changing native codecs,
identities or stored data. The scan retains no metadata collections and checks
cancellation/deadlines as it visits input. Native shape and identity validation
still follow admission; admission alone never makes an object trustworthy.

Native file predicates have separate byte and line-work admission from streamed
file downloads. Recursive parent merges are preflighted against the selected
native index before predicate evaluation, reserving ancestry work and bounding
unreduced merge terms/labels. The original native predicate evaluates admitted
inputs; limits return explicit errors without publishing or truncating results.

Implicit graph/content tree walks use the same 64-component, 4096-byte path
limits as explicit content paths. Repeated subtree objects at different paths
still require admission at each native cache key.

Graph, object, directory, diff, conflict and file reads share one native worker
capacity in the serving composition. Saturated reads return an explicit
retryable Busy result. A client cannot gain extra capacity by switching routes
or snapshot repositories. A dropped request releases capacity after its worker
exits; an open file handle keeps capacity until its verified spool is released.
Transport admission also bounds snapshot capture and response delivery, which
are separate from native worker execution.

Explicit index preparation uses one shared background worker in addition to the
configured interactive read capacity. Preparation MUST NOT occupy an interactive
slot, and a second preparation cannot gain another slot by choosing a different
repository or cache namespace. Its permit remains held until actual worker exit
after cancellation. Cache namespace locking remains independently enforced;
directory, object and operation-only reads do not require that lock.

Cold graph requests above the configured inline index threshold return an
explicit rebuild-required result. Index preparation remains tied to the selected
operation while repository heads advance. Clients retry that snapshot after a
preparation task reports completion; evicted or lost caches can require another
preparation. Completion never means repository history or local cache files are
permanently pinned. Preparation has a separately bounded trusted construction
policy; ordinary page budgets remain unchanged. Neither policy can be relaxed by
a request parameter.

Index preparation tasks are process-local and require an explicit operation.
Starting, inspecting and canceling a task require current repository read
access; only its initiating principal may cancel it. Status also checks the
current epoch. Task IDs are opaque and confer no authority. Running tasks for
the same principal, repository, epoch and operation coalesce. Completed reports
retain no credentials or full inventories. Retention, snapshot capture,
concurrent jobs and expiry are bounded; exhaustion returns Busy. Expiry or
server restart returns task-reset rather than selecting current heads. A new
start may prepare an evicted cache.

The native running deadline defaults to one hour, independently of the five
minute completed-report retention period, which starts at actual worker exit. An
explicit preparation captures the already-checked catalog root and operation
before its initiating page capability expires. Current repository read access
and epoch are rechecked during construction, with ten-second bounds on each
authorization or point-membership observation. The task retains at most 1,024
positive membership cache entries; eviction repeats verified point membership at
the same root. It MUST NOT retain an ever-growing page membership set or
enumerate the catalog. Expiration or cancellation does not release the worker
until it exits. The completed task retains no session credential.

Task cancellation or service teardown signals the native budget, and Canceling
becomes Canceled after worker exit. Expiry also signals cancellation without a
polling client. A task can fail with the native resource or integrity error;
completion does not waive those budgets. HTTP routes consume this service
contract and bound request/response delivery separately.

Both initial graph pages and signed-cursor resumes return the captured snapshot
with rebuild-required errors. A later publication cannot change the operation
selected for preparation. Continuing with the original cursor retains its
original snapshot/inventory after the exact operation cache has been prepared.
