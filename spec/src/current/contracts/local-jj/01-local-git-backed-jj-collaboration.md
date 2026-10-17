# Local Git-backed JJ collaboration

Local code admission separates authoritative boundary validation from repeated
inner-loop checks. The server may share at most 256 process-local validated
folder bindings, retaining no credentials. A cached check binds the canonical
folder, repository and registration generation and expires after one second.
Successful registration persistence and physical watcher hints invalidate it;
invalidation racing validation cannot publish a fresh cache entry. Start and
completion of bounded reads still perform physical validation, including when
watcher events are absent. Long-running reads renew expired checks. Failures are
never cached as successful admission. Operation/object membership, cancellation,
byte/work/deadline checks and source read-only behavior remain mandatory. This
cache does not authorize native shared deployments.

Local selected-file text comparisons use the shared native JJ line-diff
admission policy in api.md. Exact common lines may partition longer files before
bounded comparison of the remaining spans, without building or reading a new
repository index. Preserve complete text bytes, pinned operations and explicit
unavailable reasons; no companion or Git object is changed by the comparison.

Ready local operation graph pages may use the shared checksum-verified compact
operation DAG to order candidate identities and skip history before the anchor.
They hydrate only the anchor, subsequent predicate candidates and the bounded
page/probe window, without source views, Git objects or commit indexes. Preserve
the exact pinned JJ order for merges, skewed/equal timestamps and historical
starts. Current authorization and fresh operation fingerprints still apply to
every hydrated row; a missing, out-of-scope or nonmatching anchor is invalid.
Older derived generations retain their bounded reader until explicit upgrade.
Generation admission also retains the captured selection's exact JJ order and
inverse ranks under shared resident and traversal-scratch admission. Ready pages
whose resolved start equals that selection seek directly to a retained anchor
and charge only subsequent candidates, preserving fresh source checks. The order
contains no operation bodies, source loader or earlier request capability.
Historical-start pages retain bounded data-only frontier/chunk/visited
checkpoints after the final returned row, before probing. Exact start/anchor
keys permit reuse across metadata filters only after fresh anchor validation.
Use the pinned JJ chunk traversal and shared pre-growth resident/scratch leases;
retain no operation body, source loader or prior request capability. Missing or
evicted checkpoints use ordinary bounded traversal. Cancellation and errors
never make an incomplete page complete.

Prepared local operation comparisons MUST admit each requested operation from
the retained identity lookup and verify fresh operation and view metadata
against captured canonical fingerprints. They use the already verified immutable
commit-index bytes for parent edges and change identities, without rereading Git
commits or trees for visibility. Walk differing frontiers in topological order,
stop when only shared ancestry remains, and charge visits, queue growth, memory
and lookup work before use. Oversized differences fail under ordinary limits;
never return a partial difference as complete. Change pages group indexed IDs
before selecting their lexical window, then hydrate only selected added targets
needed for predecessor/parent comparisons. Preserve exact conflict terms,
explicit parent selection, ordering and forward/backward cursors. Missing ready
generations use the existing exact-operation rebuild recovery. This disposable
projection changes no public or authoritative persistence format.

Local task comment reactions resolve only visible exact comment versions in the
same committed companion selection as ReactionStore, with matching task
membership. Reaction writes retain other domain selections and task versions;
edits and tombstones reject old targets. Cold reopen, Git GC, receipt replay and
exact observed clear use the shared typed domain without synthetic Threads. The
local browser's existing collaboration capability gate remains separate.

The companion retains exact human reply parent revisions in the owned Git
closure; absent reply fields preserve existing canonical message and intent
bytes.

Explicit local BoundCapture carries the exact observed Thread version and
primary assignment with the original sanitized batch/profile. It retains one
real session replacement or a validated diagnostic-only capture gap, stages the
shared link without changing lifecycle and selects Agent and Thread descriptors
in one companion ref CAS. The Thread descriptor audits and retains the complete
newly staged session closure before selection. Matching receipt replay precedes
entity/source preconditions; stale bindings select neither root. The private
spool retains this original bound request across unknown outcomes. Legacy
unlinked capture tags, field order and receipts remain unchanged.

M13 adds an owned companion metadata store to ordinary Git-backed JJ
repositories. Upstream GitBackend and JJ operation publication remain code
authority. Native repositories continue to use their existing manifest, WAL and
immutable objects. The local profile shares the domain models, service policy
and presentation.

Explicit local capture shares the selected-alias tool-result resolver with the
native provider. A result-only request cumulatively replaces a verified existing
session while preserving lifecycle, previous snapshots and original receipts
across reopen. This does not enable automatic readers for local Git folders.

The shared projector also adopts qualified common native OpenCode session,
message and tool aliases across explicitly supplied hook/API/export evidence.
Case-sensitive native identity, complete selected-alias coverage and verified
parent-session/tool targets are required. Ambiguous prior targets retain source
receipts and diagnostic evidence while withholding logical entity edits. New
captures select the integration's common projection precondition through the
local expected-old-value publication; original receipts and historical roots
retain their prior interpretation.

## Public release integration

The released custom CLI is named jj-store-jj and preserves upstream JJ handling
and the existing companion provider. Equivalent Rust 1.99 test iteration does
not change native graph ordering or repository authority.

## Injectable carrier extraction

M32 extracts the same companion ref, selected descriptors, original receipts and
canonical immutable record bytes into jj-store-git. JST-LOCAL-001/002/003 remain
unchanged: publication is the existing expected-old-value ref edit, retained
history and complete closures remain required, and stock GitBackend remains code
authority. Anchored commands require an injected qualified JJ evidence validator
before actor creation or staging. The Git library rejects missing validation.
Public domain reads admit only current or retained committed metadata
selections; low-level candidate audits do not grant selected-domain read
membership.

Conversation metadata publication validates frozen AgentStore source IDs against
the selected committed record inventory, retaining older captured snapshots when
the active session advances. Exact kind, identity, session membership, closure
and evidence checks remain required. This internal validation admits no
unselected staged data and does not widen public pinned-snapshot reads.
