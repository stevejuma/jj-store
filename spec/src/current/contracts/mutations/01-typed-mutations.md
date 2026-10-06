# Typed mutations

Blame reuses read-only operation-pinned stores. Its separate file-byte admission
and data-only continuation cache grant no mutation authority, source writes,
working-copy snapshot or publication. Existing predicate comparison limits and
mutation authorization remain unchanged; admission stays held until canceled
annotation workers exit.

Local folder admission caching is an owned service primitive, independent of JJ
and HTTP. Entries are bounded, generation-bound and invalidatable; concurrent
refreshes may share a validation completed after their request began. Registry
persistence invalidates entries before and after replacement, including error
paths. This optimization changes no mutation authorization, receipts or
publication semantics.

Files metadata uses the existing read authority for a bounded first-parent pass.
It returns path-change or selected-snapshot provenance without scheduling
changed-path maintenance. Ready graph handles may prove membership; absence of
optional path indexing does not block metadata. Snapshot fallback never masks
authorization, cancellation, corruption or hard resource-limit failures and
never invokes a mutation or publication path.

Separately invoked path-index builders retain only bounded derived data and
standard JJ output. Their read authority never becomes repository authority or a
prerequisite for mutations. Files metadata does not invoke these builders.

JJ read adapters may synthesize clean merged-parent files in bounded
request-local memory to evaluate file history. They use read authorization and
canonical file identities; no source or native object write, WAL entry or
manifest publication occurs. Ordinary backend writes continue to require
mutation authorization.

Managed core indexing grants read readiness only. Construction, adoption,
queries and task control reauthorize the source and exact epoch/registration;
access loss prevents adoption. Fresh metadata observation leaves foreground read
headroom when capacity permits. Retained task IDs, disk associations and ready
index handles confer no mutation, reconciliation or publication authority.
Existing guarded commands keep their semantic and complete-closure validation.

Remote native copy-family delivery captures one checked inventory root per
ordinary backend request and uses the same source-scoped relation provider as
in-process reads. The owned transport facade exposes identities, conservative
work and tightening limits only, without upstream JJ types. Fresh permission,
epoch and deadline checks surround response delivery; required unstaged seeds
must be selected. Client-local acknowledged staging and its original captured
sequence remain separate. This read capability grants no mutation or publication
power and changes no closure audit, persisted format or historical browser pin.

Native kind task credentials live only in the active supervised job, never its
registry report or retained source projection. Start/status/cancel require fresh
Read; cancel additionally requires the initiating principal. The reserved
preparation lease stays with the job until actual exit after expiry, cancel or
teardown. Each successful descriptor window rechecks current authorization and
epoch before advancing the original root; a task report, opaque ID and complete
cache grant no mutation or publication authority.

Resumable native kind preparation binds its process-local source and checked
historical inventory root, retaining only bounded data and allocation ownership.
Each supplied session reauthorizes and checks current epoch; changing source,
repository or epoch cannot resume another preparation. Individual call deadlines
and the original total budgets/lifetime remain distinct. Failed or canceled
window I/O retains conservative work charges but commits no cursor/identity
progress. Partial IDs remain unavailable and cannot enter the complete cache.
Terminal complete-root proof may publish a rebuildable projection only, never
repository state; ordinary read scan ceilings and semantic publication audits
remain unchanged.

Copy relation reuse is read-only. Pinned browser and preparation capabilities
forward their original checked inventory selection and caller deadline; cached
source/root graphs never recapture current copy membership. Current permission
and epoch checks remain fresh through construction, incremental proof and warm
family delivery. Shared allocation leases admit graph and checked-root growth
before cloning or insertion and follow active owners beyond eviction. A
same-root construction gate retains no source or credential. Acknowledged staged
copy relationships belong only to that backend session and its captured
sequence; unknown uploads, foreign-session staging and later writes are
excluded. These projections grant no publication or mutation capability and do
not replace full closure audit or semantic publication admission.

Native inventory paging retains only a checked immutable manifest snapshot and
query binding. A new request supplies current credentials; page and resumed
reads reauthorize before and after catalog access and reject epoch changes.
These read pins expose no coordinator, upload, publication or reconciliation
capability. Signed anchors and cached descriptors never authorize a mutation.

In-process and legacy-RPC complete kind enumeration likewise retain one checked
manifest root and reauthorize each bounded scan. Matching-ID retention is
separately admitted from total inspected descriptors. This read optimization
does not replace NativeSemantic's complete inventory and dependency validation
before a guarded publication. Inventory deadlines include explicit monotonic
checks at admission, between windows and before response delivery; an async
timer alone cannot admit a late result from immediately ready I/O.

Complete operation/copy lookups may retain exact matching identities and a
checked manifest selection, scoped to the process-local native source,
repository, epoch, inventory root and kind. Retain at most 32 roots for a fixed
five-minute lifetime under a shared 64 MiB allocation allowance. Charges follow
active data owners after eviction; reserve before cloning or growing shared
data. Eight admitted complete calls bound concurrent work. Cache saturation
falls back to the ordinary bounded request-local scan. Fresh current permission,
epoch, caller/session result limits and the whole monotonic deadline still guard
every response, including warm results. Advance a complete baseline only after
consuming all exact addition pages from the start. A changed baseline selection
discards that derived baseline and restarts a bounded cold scan; corruption or
unknown reads do not certify a result. Retained data contains no source handles,
principals, credentials, request guards or publication capability.

Browser and explicit preparation kind reads use their retained checked manifest
selection through the same bounded provider, never the current code selection.
Current permission and epoch checks may observe later publication metadata but
cannot admit newly selected operation/copy identities to an older read. Matching
result capacity, total descriptor work and caller deadline remain independent;
this extends no mutation or preparation publication authority. Explicit
preparation applies its own admitted kind-result and session bounds; the
initiating page's smaller membership capacity cannot become a task limit.

Code-card head summaries are read-only projections of explicitly selected
operation/view references. They do not reconcile ambiguous heads, snapshot dirty
files, initialize metadata or publish code/collaboration state. Limited windows
and unloaded ancestors never authorize an inferred complete history count.

Ordinary local Stacks commands are a separate, registered physical-folder path.
The service exposes owned local command DTOs; jj-store-jj performs normal jj-lib
snapshot, merge/rebase, transaction and checkout operations. Versioned TOML in
`.jj/jj-store/stacks.toml` selects named workspaces and branch membership. This
path does not require the historical retained-execution protocol below;
JST-LOCAL-005 governs stale admission and explicit inspection of unknown
outcomes.

The local retained-execution provider may install resolved physical file effects
from an exact saved tree plan and original inventory after guarded head
publication. Provider-owned pending intent, prepared identities and no-replace
moves preserve originals as backups and keep native checkout/tree-state bytes
unchanged. Its sealed physical-files result is not a command receipt, native
metadata completion or selected combination after-state. The remaining local
protocol stays governed by JST-LOCAL-005; native manifest publication and
production writable capabilities are unchanged.

Local Stacks checkout preparation consumes a bounded read-only physical scan
independently of the code operation selection. The scan fingerprints exact file
bytes, modes, relative names, symlink targets and physical workspace identity,
including ignored/untracked obstacles. Limits, cancellation, unsafe shapes and
changes between validation passes produce no partial authorization. This
foundation neither publishes nor grants an executor capability; durable intent,
foreground JJ locking and guarded checkout remain required by JST-LOCAL-005.

Independent conversion stages typed historical domain snapshots without altering
the source. Translation maps immutable repository-bound references explicitly;
only the destination provider may select these audited roots. Original source
receipts remain provenance and never authorize destination commands. Preparation
selects supported historical roots only in a new independent destination, with
stable synthetic request identities and explicit source-to-destination maps.

Prepared local history readers retain read-only operation and backend guards.
Evolution may follow only retained operation-parent and predecessor edges under
ordinary admission; neither cache visibility nor reverse reference labels grant
write capability. Index and operation-store writes remain denied, and cold
history recovery prepares only disposable derived data without changing source
heads, companion selection or physical files.

Agent session, message and tool duration timing are bounded read projections
over exact selected event evidence. Recovering qualified Codex JSONL timestamps
from sanitized payloads does not stage or publish objects, alter immutable
evidence or capture recovery identities, replay receipts, or enroll sources.
Native manifest and local companion pins retain their existing authorization and
publication behavior.

LocalTrusted Admin configuration editing may persist installation-wide protected
bookmark names and full commit identities. These rules apply to every native
repository hosted by the installation after the next startup; they are not a
repository-local settings mutation and never rewrite code or durable receipts.

Local portable agent review source inspection consumes an explicit committed
companion pin and exact canonical AgentStore records. It validates the shared
portable session/tool/content identities and selected byte digest, then rechecks
admitted history before returning. Staged or future-selected records, a
replacement repository identity, mismatched optional roots and fabricated native
publication counters fail closed. The bounded read neither establishes an actor
nor selects review metadata; write and retention qualification remain separate.

Local repository review HTTP and CLI actions use the already-qualified typed
projector and original device actor/receipts. No request supplies actor
identity. Replies bind workflow versions, edits bind selected comment revisions,
and workflow changes bind selected thread snapshots. Missing receipt status
remains unknown; browser drafts stay intact and writes are never replayed
automatically. Confirmed actions advance companion display metadata
independently of code; ordinary review reads stage or publish nothing. Portable
code/agent anchors stay rejected until their provider evidence gate passes.

Task comment reaction commands use authenticated metadata-home actors and the
existing exact observed slot preconditions, receipt replay and unknown-outcome
handling. Admission resolves the visible comment's exact version and task
membership against the publication base Work root. Reaction publication cannot
change task versions or grant access to linked code; old edited and tombstoned
targets fail closed.

Collaboration read optimization does not change publication or receipt behavior.
Review activity reuse stores numeric facts only after successful derivation from
an admitted immutable selection. Fresh provider closure admission, request
authorization and current snapshot capture precede lookup; cache retention never
becomes a command base or hides missing evidence or tighter source limits.
Compact review activity derives only count and accepted-event time facts from
admitted thread selections. It retains provider closure audits and fresh access
checks, stages no objects and creates neither an actor nor a repository
identity. Shared data caches retain only validated immutable domain selections
and graphs. Request authorization and complete captured manifest envelopes
remain fresh; scoped reads still validate all retained dependencies of their
selected domain. Saturated retention may validate an extra root request-locally
under existing per-pin and concurrency budgets; a small cache cannot disable a
valid query. Neither warm reads nor derived directory/reaction indexes stage or
publish state.

WorkStore browse and detail pins share immutable root data and derived ordered
keys, counts and searchable fields. Query continuations retain keys rather than
prehydrated rows; fresh request authorization, inventory admission and epoch
checks still apply. These read-only projections never retain request principals,
guards, credentials or accepted command state, and do not change publication,
durable receipts or domain capacities.

Repository-directory task activity shares these data-only root projections and
retains complete counts/timestamps for returned repository/project groups.
Native pages recheck current permission, epoch and inventory admission; local
pages retain committed companion membership and current folder binding checks.
Neither aggregate pages nor card refreshes initialize an actor, publish a root,
replay a command or authorize linked-code access.

Local WorkStore browse/activity/detail reuse retains parsed immutable state and
derived query keys only. Complete companion membership, the admitted domain blob
and current folder identity remain fresh. Commands, reaction admission,
conversion and receipt recovery keep their existing reconstruction paths; this
read cache does not become a mutation base or retain a provider/actor handle.

Application content search caches only validated immutable Thread/Work fields
and ordered document keys. Returned snippets, viewer authorization and captured
manifest envelopes stay request-bound. New roots, edits and tombstones cannot
reuse another root's current-body projection; resumed historical pages remain
bound to their original accepted selection. Search normalization and bounded
snippet construction stage or publish nothing and retain no command receipts or
credentials.

Native/local resident LRU eviction releases only disposable prepared read
handles. It cannot cancel accepted commands, discard durable receipts, change
native or companion selections, or replace the operation captured by an active
reader. A reset preparation task remains a read-recovery condition; starting it
again grants no mutation capability.

Installation-local capture status and pause/resume use owned service DTOs and a
transport-independent manager. This is device policy, not a repository mutation:
the admitted transport supplies the actor, current installation Admin authority
is required, and the command binds a stable request, control generation and
expected policy revision. Actor fields and unknown command fields are rejected.
Responses separate durable policy receipt from runtime transition acceptance;
body-free status and exact replay grant no publication, import or execution
authority. Native AgentStore and WAL mutation semantics remain governed by their
existing requirements.

Native and local companion capture share qualified common OpenCode identity
adoption through the owned projector. Exact selected aliases may enrich an
existing session/message/tool; source cursors and receipts remain independent.
The service cannot infer identity from content, directory or time, or initialize
an automatic reader from this capability. Each provider retains its ordinary
authorization, complete dependency audit, stable request receipt and publication
precondition around the shared projection. Ambiguous or incomplete selected
identity proof withholds entity edits while retaining sanitized source evidence.

Installation import adds an actor-free version-1 command binding the exact
request ID, control generation/revision and runtime source selector. It
dispatches to an existing bounded source owner and acknowledges only
synchronized accepted intent after native Write and original-prefix admission.
The retained owner preserves admission/body settlement and established worker
timing; slow preparation/body work may yield an unknown public acknowledgement
while durable original intent remains available after settlement. Exact receipt
replay resolves before new-selector/policy checks; unknown outcomes retain
original input. The receipt remains device intent and grants no native
publication. Inspection requires current installation Admin and uses only
bounded metadata; it cannot initialize missing state, enroll history, recover
pending bodies or create an execution/storage owner. Browser controls and
automatic accepted-request restart dispatch remain separate gates.

Application content search is an authorized read facade over one selected
Thread/WorkStore snapshot. The service checks source inventory bounds before
filtering and reauthorizes retained query pages. Thread directory metadata is
derived from the same selected graph; neither projection stages objects or
publishes state. The API and adapter chapters govern query and cursor contracts.

Thread timeline pages hydrate only their own authorized human-message revisions
from the same captured graph. Agent and review content remains in its canonical
source; UI adapters never invent transcript bodies.

Explicit local BoundCapture carries the exact observed Thread version and
primary assignment with the original sanitized batch/profile. It retains one
real session replacement or a validated diagnostic-only capture gap, stages the
shared link without changing lifecycle and selects Agent and Thread descriptors
in one companion ref CAS. The Thread descriptor audits and retains the complete
newly staged session closure before selection. Matching receipt replay precedes
entity/source preconditions; stale bindings select neither root. The private
spool retains this original bound request across unknown outcomes. Legacy
unlinked capture tags, field order and receipts remain unchanged.

Native capture preparation forwards an explicit optional Thread binding through
the owned service facade. It validates typed input, derives the current actor
and acceptance time, and stages the exact Agent delta without selecting
repository state. Bound publication authorizes both domains and atomically
selects their roots; stale observed Thread/assignment preconditions select
neither. Matching receipt replay retains the original result before entity
preconditions. Empty bindings preserve unlinked capture behavior and do not
launch a runner.
