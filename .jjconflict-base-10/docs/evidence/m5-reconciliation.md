# Explicit native operation reconciliation

ReconcileOperations takes an explicit current `base` operation and the original
request's complete expected snapshot. It merges all current heads, ordering the
base first and the remaining full IDs deterministically. Base means the starting
view and comparison context for reports. Actual merge ancestors are computed by
pinned JJ, including recursive common-ancestor merges and descendant rebases.

The native executor calls RepoLoader::merge_operations through the bounded
staging authority and leaves its operation unpublished. Current write
permission, strict snapshot matching, shared protected-history validation and
durable receipt selection remain mandatory. Apply publishes once with
HeadsNormalized; preview and GET/HEAD never reconcile authority. Changed base
under the same committed request identity conflicts. Single-head, stale/subset
or noncurrent-base input fails explicitly. Report and receipt layouts retain
their existing V1 fields.

Before native merging, ancestry is traversed through checked committed
membership and metadata limits. The executor reserves reachable-operation-count
squared times head count from its work budget. This conservative admission can
reject large histories; it is not a measured production performance guarantee.
Existing staging/byte/deadline/cancellation and shared worker bounds still
apply.

Five focused fixtures cover native descendant/bookmark/workspace rewrites,
preview zero-publication and receipt replay; exact heads and explicit base;
three heads with recursive common ancestors and conflicted bookmark results
matching upstream; protected-history and resource failures with unchanged heads;
and HTTP snapshot/preview/apply/status/retry. Browser features remain deferred;
only generated TypeScript API declarations change under web/.

Commands and outcomes are recorded in E-M5-004 of the
[single progress ledger](../../spec/src/milestones.md). These memory-backed
native proofs are separate from real RustFS/TLS and operational resource
qualification.

Aggregate validation passed: normal checks, all 280 Rust tests (six external
tests explicitly ignored), build, formatting and all 13 specification lanes.
Existing browser acceptance passed eight cold interaction/accessibility tests.
The basic M5.1 command and reconciliation gate is complete; abandon/conflict
resolution, real deployment and deferred browser gates retain their separate
ledger status.
