# Managed graph cache lifecycle

LocalTrusted serving activates core indexing on the first repository open. The
existing repository monitor retains this activation independently of browser
subscriptions and supplies freshly authorized exact operations when code heads
change. It MUST NOT introduce a second watcher, poller or full search bootstrap.
Non-code events MUST NOT rebuild the graph. Restart discovers bounded persistent
cache bindings and resumes the same monitors. One shared native/local owner
coalesces selections, bounds pending/retained tasks, reauthorizes reads and task
control, and holds construction admission through actual worker exit. Old pinned
readers remain usable during newer construction. Cancellation, access loss and
changed epoch/registration MUST prevent adoption. Failure does not automatically
repeat on unchanged monitor witnesses; new code episodes or explicit reads may
retry. Authenticated automatic maintenance remains separately deferred.
Transient observation or admission failures retain usable pinned readers and
retry an uncompleted activation through the existing monitor. Only confirmed
access or identity loss invalidates readiness; a busy worker or deadline does
not prove either condition. Background local head observation shares interactive
admission and leaves a foreground slot available when capacity exceeds one. It
yields without queuing when that headroom is unavailable; existing freshness
checks catch up later.

Core native and local indexes share the existing cache admission namespace and
its slot, repository, global-byte and file-count limits. Persistent managed
reservations retain their source profile plus native epoch or local registration
generation, never credentials. Reserve the full construction allowance while a
writer runs, then charge actual retained bytes. Resume activation from complete
retained namespaces after restart. Incomplete construction may only be repaired
in private storage. Eviction may reclaim idle derived namespaces; memory readers
retain their exact operations independently.

The server query cache uses a private `query-cache-v1` namespace below the
pinned upstream revision and native profile. Bounded slots retain repository,
epoch and operation bindings, a recency counter and checksums of complete
derived files. The checksums detect local cache damage; native objects remain
the authority. Query execution checks current authorization before touching
these files.

An exclusive namespace lock serializes primary graph-cache admission and
bookkeeping, including separate executors/processes sharing that directory.
Contention returns Busy unless the request can use its exact immutable resident.
Primary index preparation installs its complete staged cache and bounded receipt
before graph evaluation releases this lock. Missing files, invalid receipts or
checksum mismatches trigger reconstruction. A mismatched native backend/profile
binding fails explicitly. Source corruption or denied reads cannot be repaired
by cache rebuilding and remain errors.

Retention bounds slot count, file count, individual-cache bytes and total
retained bytes. Defaults retain at most 16 slots, 2 GiB of file bytes per slot,
4 GiB per repository, 16 GiB in total, and 4096 filesystem entries per slot.
Cold staging shares these byte allowances. These are admission and retention
limits, not measured throughput guarantees. Least-recently-used slots are
evicted under the namespace lock; failed builds and abandoned staging
directories are disposable. Symlinks are never followed. A canceled worker
retains its independent reservation lock and worker permit until it exits;
construction does not retain the namespace lease. Responses report whether their
cache was built, reused or rebuilt and retained cache counts/bytes. These are
local diagnostics, never publication positions. Transient build disk use, native
synchronous CPU work and production query memory still need resource
qualification before shared HTTP exposure.

Native query request cancellation signals the shared worker budget. Pending
permission checks and storage I/O are dropped, and later CPU/work checkpoints
reject the canceled request. A worker keeps its permit and cache lock until
actual exit, including while a bounded synchronous native step finishes.
Cancellation never publishes native state. A successfully returned verified file
stream owns its remaining lifetime separately from the preparation request.

The native runtime adapter disables Tokio's scheduler-only cooperative budget
for each backend/stream poll. Pinned JJ can synchronously poll these futures
through pollster; that executor cannot replenish Tokio's task-local budget.
Project work/byte limits, real deadlines and cancellation remain enforced by the
owned boundaries. Real pending I/O remains pending. This rule prevents a
cooperative-yield spin during metadata-filtered native revsets.

Application query stores preflight V1 metadata with an allocation-free
collection visitor before native decoding. Default admission permits 4,000,000
JSON values and map keys cumulatively, 65 terms per persisted merge, 256
parent/predecessor edges per value, and 50,000 entries per tree. JSON nesting is
bounded separately. Native graph/content limits carry this policy and report the
cumulative scanned node count. The native persistence codec remains lossless for
shapes outside these query budgets. Derived native merge expansion and aggregate
memory still require separate qualification before shared HTTP exposure.

Revset file predicates preflight recursive merge shape using the pinned native
ancestor-selection order. The preflight tracks counts rather than constructing
merged trees, bounds intermediate terms before allocation, and reserves work for
both preflight and native predicate evaluations. This policy leaves normal graph
queries intact. Predicate file bodies have per-file/cumulative byte caps and
reserve cumulative squared line work before native file materialization. Binary
streaming through the content API has independent limits.

Read-only query authority also admits each logical backend tree path before
native traversal, including virtual trees. Paths are limited to 64 components
and 4096 bytes. Ordinary native sessions keep their existing authorization; this
query guard does not alter persistence or physical client workspaces.

Returned native file readers also cross the caller-owned runtime adapter on
every poll. The foreign-executor cooperative-budget rule applies after file
preparation as well as during backend futures and copy streams; genuine pending
local I/O remains Pending.

A serving instance composes its graph executor and content reader with one
shared interactive worker pool, retained across repositories, routes and clones.
Explicit index preparation uses one additional shared background slot and MUST
NOT consume configured interactive capacity, including when that capacity is
one. Another preparation returns Busy even across distinct cache namespaces.
Interactive requests cannot borrow the preparation slot. Admission is
non-queuing and returns Busy when the relevant capacity is exhausted. A worker
retains its slot through native execution, including canceled synchronous steps.
A verified file retains its slot with its spool until the handle is dropped,
including after EOF. Validation, authorization and worker failures return their
slots without weakening current permission or snapshot membership checks.

The composed executor exposes total capacity (interactive plus preparation) and
active-slot diagnostics without credentials or repository data. This bounds
simultaneous native jobs/streams; per-query byte/work/shape limits remain
independent. It is not an OS memory reservation. Snapshot admission, response
delivery, native RPC and background jobs also need explicit composition at the
serving boundary.
