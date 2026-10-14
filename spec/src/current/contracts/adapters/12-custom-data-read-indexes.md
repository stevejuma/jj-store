# Custom data read indexes

JST-ADAPT-008 governs disposable secondary indexes for native WorkflowStore,
WorkStore, ThreadStore, ReactionStore, AgentStore and ReviewStore reads.
Manifest-selected native records remain the sole source of truth. Index files,
readiness, summaries and query keys cannot publish state, confer access or
select a newer or older source root.

The index owner binds source namespace, repository, epoch, complete domain root
and derived schema. Fresh request admission precedes reuse. Cold admission still
audits the complete source closure; persisted rows must agree with the admitted
source facts. Stable native identities remain exact. Derived version-1 row and
posting codecs do not replace any native codec or format.

The shared process owner coalesces construction per immutable root, retains no
session, credential or viewer-filtered answer, and bounds root count, lifetime,
objects and retained bytes. Live owners retain their accounting after eviction.
Each resumed request reauthorizes and preserves the original query and pin.

Ordering, relationship and exact-filter indexes hydrate only selected windows.
Substring acceleration uses lowercase Unicode trigrams and exact full-field
verification; short queries preserve bounded eligible-scope scans. Current
tombstones, reaction conflicts, merged review events and historical workflow
attempts keep their existing meaning. Counts remain complete or explicitly
partial according to the owning API.

Persistent redb generations are server-owned, private and rebuildable. Complete
generations commit atomically; source mismatch, corruption or incompatible
schema requires reconstruction. Derived storage failure never fails publication.
A bounded authorized fallback uses the same selected root and the existing
public error/timeout contract. Cache loss cannot hide a source integrity
failure.

Incremental projection reuse compares exact immutable object selections and
handles edits and removals. Partial traversal and additions-only comparisons
cannot certify complete custom data. Initial and near-limit source validation,
incremental costs and warm query execution are measured separately.

Native serving is the qualified target. Companion migration, raised model caps,
new search scopes and deployment remain independently governed.

Review activity retains fresh complete provider closure admission on every first
request, including a summary-cache hit. Workflow cold hydration stops before
accumulating more than 64 MiB of selected object payloads or exceeding the
current caller inventory limit. Warm reuse rechecks that caller limit. These
resource bounds do not alter authoritative entity capacities or bytes.
