# jj-store implementation plan

The approved core-browsing simplification replaces the superseded custom graph
builder/projections with JJ 0.45.1's standard index. Retain lightweight pinned
metadata, bookmark paging before hydration, scoped content, files-first UI and
existing authorization/cursors/cancellation. The remaining programme is limited
to three stages:

1. Prove private warm reuse of Linux's index and native incremental
   construction, with exact history pages and read/work counts. Keep the
   read-only Git decoder.
2. Use one managed native/local lifecycle, activated on first open and resumed
   after restart. Reuse the existing per-repository event monitor with a
   retained server lease and bounded catch-up checks. Use one coalesced
   construction job; ready readers remain usable. Enable automatic maintenance
   for LocalTrusted.
3. Connect existing preparation/status/cancel and freshness recovery to ordinary
   history, remove superseded core dependencies and setup buttons, land the
   accumulated changes in main, and verify the original Linux dev-app flow cold.

Advanced search/evolution/copy/deep-query optimisation, authenticated automatic
maintenance and complete release/fault/RustFS qualification remain deferred.
Keep advanced components when removing them would require another broad rewrite.
Standard JJ construction limitations must be reported; do not add another custom
writer. No source-index repair, repository migration or installed binary change.

E-READ-079 proves standard-index Linux reuse and native incremental
construction. E-READ-080 adds persistent cache admission, event-driven
maintenance, restart activation and automatic exact-selection UI recovery.
Advanced preparation remains explicit. E-READ-081 removes the unused change
lookup from ordinary graph selection and includes Commits in core recovery.
E-READ-082 retains freshness witnesses during background admission pressure.
E-READ-083 records landing, the preserved 7470 Linux app's cold files/history/
selected-diff acceptance, restart segment reuse and unchanged source bytes. The
narrowed core-browsing acceptance is complete; the deferred qualification and
advanced-feature work above remain separate.

E-READ-078 connects native operation/copy preparation tasks to the browser's
shared task owner, typed HTTP data source and repository index settings. Retain
original operations, bounded progress/status admission, explicit cancellation,
exact terminal completeness and view restoration. Local/graph preparation stays
within its existing policy. Focused controller/transport and cold component
acceptance accompanies this slice; complete scale/cold full-flow/RustFS/release
qualification remains deferred.

E-READ-077 delivers selected copy families through the ordinary remote native
CLI authority, owned service facade and additive bounded RPC. Retain checked
root selection, independent construction/result/work/scratch/deadline ceilings,
fresh authorization and captured client-local staging. Exact loopback and strict
wire regressions accompany the slice; no HTTP/schema, installed executable or
authoritative format changes. Continue app preparation recovery and remaining
indexed queries, leaving complete scale/cold/RustFS/release qualification open.

Accepted 26 September 2026. The canonical requirements in `spec/src` govern
behavior. [milestones.md](../spec/src/milestones.md) is the single
implementation progress ledger. The [original attachment](reference/README.md)
is archived unchanged as non-normative reference material.

## Local stacks backend proposal

Reviewed 5 October 2026 against the original UI-first request, JST-UI-020 and
the app-shell Storybook on port 6057. The user approved backend implementation
and production UI wiring on 5 October 2026. M21.2 is now In progress and follows
the delivery order below. The existing UI is the intended product surface.
Visual baseline approval and foundational package publication remain separate
gates.

### Review of the current UI

The original split still holds: production-owned presentation and controller
code are exercised through an isolated stateful catalog adapter; real checkout
execution has not been enabled. The latest reviewed UI parent is `687f056b`.
Live inspection covered Workspace, applied/local branch groups, selecting an
unapplied stack, its conflict preview and cancellation, and the target history
column. This review did not repeat the prior interaction or accessibility
suites.

| Original intent                                     | Current result                                                                               | Integration implication                                                                 |
| --------------------------------------------------- | -------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------- |
| A separate Stacks destination                       | App-shell injection, Branches and Workspace body tabs, centered workspace picker             | Keep the composition; production bootstrap must own and inject the real controller      |
| Stack browsing and applying branches                | Existing branch picker, dependent branch cards, target history and Apply/Unapply preview     | Supply bounded real branch/graph reads and exact membership commands                    |
| Combined local workspace                            | Fixed Unstaged, applied lanes, independent adjacent diffs, inline drafts and file/hunk drops | Replace simulated trees and command outcomes with actual JJ and checkout state          |
| Named combinations with independent unfinished work | Mock workspaces retain their own Unstaged edits                                              | Implement durable combination identity, merge/WIP retention and safe physical switching |
| Reuse the application and source references         | Owned Community/Changeyard adaptations and public Design Core primitives                     | Preserve provenance; no runtime or build dependency on either reference checkout        |
| Only repositories available on this machine         | Folder, profile and capability fields; controller rejects missing folders                    | Enforce verified device binding and command eligibility on the server                   |

The UI now covers more than branch browsing and apply/unapply: positional
rewrites, split/squash, branch-segment deletion, named combinations and retained
drafts are part of its contract. Wire these incrementally and advertise each
capability only when its backend behavior is qualified.

### Gaps that must be resolved before enabling writes

This list records the original review before implementation. The delivery status
below identifies which gaps the subsequent verified slices have addressed.

1. **No production adapter exists.** `web/src/App.svelte` supplies only the
   repository controller. The Stacks injection exists in the application shell,
   but its controller is instantiated by catalog fixtures and tests.
2. **The frontend snapshot is not a wire contract.** A numeric `revision`, plain
   string identities, eagerly embedded files/patches and a flat commit array do
   not capture operation pins, physical checkout changes, graph parents or
   profile-specific identities. Keep the presentation model behind a typed
   adapter, and extend it where it cannot represent real states.
3. **Aggregate diffs are presentation unions.** `stack-workspace/files.ts`
   concatenates per-commit hunks. A file changed and then reverted must
   disappear from a branch's net diff. Real branch/stack diffs must compare the
   exact boundary and head trees; transfer eligibility needs verified hunk
   ownership.
4. **Recovery and undo are only simulated.** The fixture stores receipts and
   previous snapshots in memory. Production needs durable request discovery
   after browser/server restart, exact prepared operations, checkout recovery
   and guarded undo. Replaying a receipt must never execute an undone command
   again or replace the visible current state with an old receipt snapshot.
5. **Conflict and file models are too narrow.** The catalog has path plus
   left/base/right text and patch strings. Real JJ conflicts can have multiple
   ordered terms; file changes include additions, deletions, renames, binary,
   executable and symlink cases. Use existing native conflict fingerprints,
   content limits and unsupported-state presentation rather than flattening
   them.
6. **Lifecycle wiring is incomplete.** Add repository/binding-scoped controller
   ownership, automatic refresh, pending-command restoration, cancellation and
   teardown. Distinguish stable change identity from rewritten commit identity;
   preserve valid selections through explicit rewrite mappings and invalidate
   ambiguous selections without silently widening a staged selection.

The default checkout reviewed at `d1a25261` already contains newer bounded local
reads, index preparation and application freshness work than this UI workspace.
Its local capabilities still report `code_mutations: false` and
`working_copy_edit: true`. The existing physical-file Save is an isolated file
write, not a JJ snapshot or recoverable multi-file checkout transaction. M13.4's
JST-LOCAL-005 command-publication prerequisite remains necessary; unrelated
search and conversion completion should not block this feature.

### Current delivery status — 5 October 2026

The production application now owns and explicitly injects a read-only Stacks
controller for registered LocalTrusted Git-backed JJ folders. Bounded bookmark
continuations, stack and target history, exact boundary-to-head file lists and
independent adjacent diffs use the existing operation-pinned code API. Graph
index preparation uses the existing application notice and explicit action.
Connection teardown and repository/binding changes dispose only Stacks-owned
reads; a failed refresh does not promote a partially refreshed read scope. Valid
loaded ranges and historical diff selections survive refresh with fresh
continuations and annotations.

| Delivery stage                       | Current state                                                                                                                                                                   | Remaining acceptance                                                                                     |
| ------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------- |
| 1. Integrate UI and define contracts | Versioned service intent/selection/preparation/receipt types and read-adapter boundary delivered                                                                                | Executable wire contracts accompany the recoverable executor                                             |
| 2. Real browsing                     | Production browsing, private physical observations and exact WC/ignore comparison delivered                                                                                     | Active-combination assignment and complete cold workflow acceptance                                      |
| 3. Recoverable execution             | Private retained reword execution, exact head recovery, unchanged-tree receipts and changed-tree preflight and durable original plan/reference delivered; mutations unavailable | Durable physical installation, production command transport/status and complete execution fault recovery |
| 4. Named combinations and membership | Planned; Workspace remains empty without qualified durable state                                                                                                                | Explicit setup, merge/WIP retention, apply/unapply, switching and restart                                |
| 5. Staging and inline commits        | Reviewed UI and draft controller delivered                                                                                                                                      | Exact whole-file engine, then server-issued hunk selections and transfers                                |
| 6. Graph actions and recovery        | Reviewed menus and preview/recovery UI delivered                                                                                                                                | Qualified rewrite actions, deletion policy and guarded undo                                              |
| 7. Linked-native lifecycle           | Planned; linked-native commands remain unavailable                                                                                                                              | Manifest/checkout recovery, complete cold flows and real-storage evidence                                |

E-M21-006 records focused controller, production Storybook and cold real-folder
browsing evidence. This is an intermediate read delivery, not completion of
M21.2. It does not fabricate combinations, expose a command executor, scan or
snapshot physical edits, or replace the ordinary repository Log/Branches views.
The seven-stage order below remains the approved implementation plan.

E-M21-007 adds the private physical fingerprint foundation: bounded regular-file
and symlink observations, complete checkout obstacles and two-pass stale
detection. It does not yet perform provider baseline/ignore projection, expose a
scan endpoint, populate Unstaged or grant a command capability. Stage 2 remains
incomplete until those provider reads and cold real-folder integration qualify.

E-M21-008 adds a private exact named-workspace baseline and raw-content/mode
comparison using JJ's captured ignore rules. Tracked ignored changes remain
changes; untracked candidate additions and ignored files remain checkout
obstacles. Stage 2 still requires current checkout/sparse proof, global/exclude
policy capture, transport and cold registered-folder integration. Historical
baseline reads and private projection helpers do not populate active Unstaged or
enable writes.

E-M21-009 integrates private current checkout/sparse/ignore witnesses with the
bounded physical observation service. It verifies metadata, policy and operation
heads before and after the scan, checks access throughout, and retains scan
admission until a canceled worker exits. Real disposable CLI fixtures cover
sparse scope, unsnapshotted files, policy/metadata races and cancellation
without writes. No scan route or active-combination assignment is enabled by
this slice; Stage 2 and M21.2 remain incomplete.

E-M21-010 adds the bounded registered-folder `working-tree/observe` route and
owned versioned request/response contracts. It resolves paths from registration,
binds the expected generation and exact operation, and returns changed-path
metadata plus read fingerprints without content, private ignore policy or
command tokens. The existing cold filesystem-server fixture records real dirty
changes and unchanged code/companion/physical state. Generated response
inventory and client contracts include the endpoint. Production physical-read
integration, active-combination assignment and cold browser acceptance remain
unfinished; this route enables no writable UI control. The production projection
correctly retains `active: null` and no named workspaces before setup. A
disk-only delta against the WC baseline is not the complete Unstaged difference
from a durable combination merge: snapshotted WIP must also be accounted for.
Physical-read client ownership can be integrated separately, while complete
Unstaged assignment must wait for the recoverable execution and combination
engine in stages 3–4.

E-M21-011 integrates the owned, uncached physical-read client and a separately
owned controller observation. It validates exact registered pins, complete
bounds, ordered terms/paths and coherent changed-file metadata without narrowing
integers. Fresh current-head checks bracket the read; ambiguous heads remain
unavailable. Refresh and disposal cancel only owned work and reject late
results, while historical selections and adjacent diffs retain their own scope.
These observations do not populate complete Unstaged or infer a combination.
Durable execution and named combination merge/WIP state in stages 3–4 remain
required.

E-M21-012 qualifies the installation-private exact-preparation journal,
principal/profile/physical scope, monotonic receipts, bounded pending discovery
and capacity reservation. Real atomic-replacement faults and subprocess loss
recover the original unresolved receipt through reopen and exact replay. This
metadata primitive does not retain JJ object closures, publish operation heads
or coordinate checkout. Stage 3 continues with those provider and runtime gates;
all seven approved stages remain required before feature completion.

E-M21-013 adds reserved-ref archival retention for exact prepared operation/view
and code capsules, separate from the companion ref. Real unpublished JJ fixture
code survives source operation/index loss and Git GC; immutable scope/intent
slots reject collisions and conflicting retries. This primitive is still
separate from production command generation, source operation-ancestry recovery,
head publication, foreground checkout and runtime status. Required owning JJ
lint also repairs four inherited read-helper diagnostics without changing read
admission or preparation semantics; direct local/native read acceptance records
the expanded verification surface.

E-M21-014 qualifies nonblocking foreground coordination against the pinned stock
JJ CLI, with exact post-lock checkout/policy/head validation, busy admission,
cancellation, process-loss release and replaced-lock/directory rejection. The
lease exposes only owned metadata witnesses and never snapshots, publishes or
updates checkout bookkeeping. Physical editor bytes still require their separate
fresh guard. Stage 3 continues with production preparation and publication plus
guarded durable checkout and runtime journal/status composition.

E-M21-015 adds real unpublished reword preparation through the foreground lease.
Exact source/prepared capsules, visible ancestry, descendant rewrites, author
and tree invariants, ordered bookmark effects and unchanged checkout trees are
verified against real JJ fixtures. Candidate-write cancellation and
authorization loss expose no preparation or publication. The provider output is
compatible with reserved-ref retention, but server command policy, exact request
journal composition, source operation-ancestry recovery, publication and
checkout remain required. Other command kinds and all later approved stages
remain incomplete.

E-M21-016 retains the complete bounded source operation ancestry and historical
views/code alongside the selected capsule in explicit version-2 preparation
records. Legacy version-1 slots remain immutable. Read-only source capture,
exact private upstream materialization, historical-only code reachability,
malformed/link/limit rejection and exact replay are verified against real JJ
fixtures, including source loss and Git GC. Real reword generation now performs
strict source capture before candidate work, returns complete candidate history
and verifies all original records. Missing original extras fail before implicit
import or candidate writes. This supplies recovery input; server policy/journal
composition, source restoration, head publication, guarded checkout and runtime
fault acceptance remain required. All seven approved stages continue unchanged.

E-M21-017 adds explicit source operation/view restoration through a saved
bounded private witness and original reserved retention key. Recovery
revalidates physical binding, checkout/policy/heads and foreground coordination
without loading a missing operation or index store. Complete conflict preflight
precedes synced create-only writes; exact retry preserves existing records and
leaves heads, index, extras, companion and physical checkout untouched.
Process-loss and cancellation acceptance remain part of this owning provider
slice. Runtime original-request policy/journal composition, extra/index
recovery, publication and guarded checkout are still required before writable
capability. The seven approved stages remain unchanged.

E-M21-018 joins actual unpublished reword preparation with its exact preview,
original principal/request/intent, private recovery witness and reserved
retention. Pending journal selection saves the full candidate archive before
retaining code, closing the interruption window without re-preparing on retry.
Acknowledgement retains the exact commit/digest and releases only the pending
archive body. Real process loss before/after retention, cancellation, source
operation/index loss and Git GC retain the same preview and receipt. Legacy
records are immutable; pending archives count toward 128 records/8 MiB and
cannot proceed to publication. This is a trusted composition seam with fixture
authorization/combination/physical context, not installed command authority.
Runtime policy, extra/index recovery, publication, guarded checkout and
prepare/apply/status/discovery remain Stage 3 work. Stages 4-7 remain approved
and required; production capabilities are still read-only.

E-M21-019 adds the registered preparation authority adapter: only the configured
LocalTrusted Admin principal is eligible, paths/generations come from device
registration, and physical selection comes from the existing bounded observer.
Trusted combination ownership supplies active selection and protected heads at
the actually observed workspace. Reword checks exact immutable ancestry before
candidate writes. Real CLI-folder/registry acceptance covers external edits,
registration removal and pending replay after operation/index loss and GC;
combination ownership is still supplied by an isolated fixture. This adapter
does not create setup or wire command transport. Durable combination ownership,
extra/index recovery, publication, checkout and remaining stages stay required.

E-M21-020 adds the private durable combination metadata owner and its real
reserved-code archive linkage. Selected snapshots remain unchanged while an
exact provider update is pending; a matching completed journal checkout receipt
precedes device selection. Reopen, conflicting retry, private-state replacement,
capacity and process-loss cases retain the original outcome and cannot rewind
later state. Registered preparation now checks the stored operation/view/WIP
against the actual physical workspace. This qualifies metadata and command
context, with simulated publication/checkout receipts clearly separated from
real provider evidence. First-use setup, merge/WIP generation, membership and
switching engines remain Stage 4 work; source extra/index recovery, publication,
guarded checkout and command transport still gate Stage 3. Production writes
remain unavailable and all seven approved stages continue.

E-M21-021 qualifies explicit missing-extra restoration through the saved
foreground lease and original retained archive. Complete private preflight
compares existing native metadata and verifies canonical missing records before
synced create-only segment/head selection. Original bytes remain intact through
cancellation, process loss, lock replacement/contention, unsafe links and exact
retry. This does not import refs or publish operation heads, rebuild source
indexes, write checkout state or enable mutations. Derived-index recovery, safe
publication, guarded checkout and runtime command composition still gate Stage
3; all later approved stages remain required.

E-M21-022 qualifies separate original/candidate derived-index restoration from
the same saved archive through the existing bounded builder. Native private
decoding verifies indexed commit/change identities and parent edges; existing
CLI mappings preserve their bytes and the original operation cannot acquire
future candidate visibility. Cancellation/process loss after one association,
directory replacement, authorization loss and unsafe/oversized input retain
exact retry. This closes the private missing-index primitive, with safe
operation publication, guarded checkout and command transport still required
before Stage 3 writable acceptance. Stages 4-7 remain approved and incomplete.

E-M21-023 qualifies exact retained operation-head publication under the original
foreground lease and actual upstream head-store flock. Complete dependency
preflight, bounded native ancestry and synced candidate-before-original markers
preserve concurrent CLI siblings and physical checkout bytes. Retry after
cancellation/process loss completes partial selection; retry through a later CLI
descendant acknowledges the original candidate without reannouncement or
rewinding. Ordinary history restoration keeps its strict original-head witness.
This provider proof attests code publication only. Guarded checkout, durable
runtime command composition and prepare/apply/status/discovery transport still
gate Stage 3. Production writes remain unavailable, and all seven approved
stages remain required.

E-M21-024 qualifies unchanged-tree checkout pointer completion: original-request
canonical witness, exact-prefix deterministic staging, complete two-pass
physical guard and cancellation/process-loss retry. Native tree-state and
physical files remain unchanged. Real dirty/ignored/untracked, mode/link,
authorization and binding cases are distinct from fixture provider guards. The
strict service lint repairs retain public read and tree-selection contracts.
This is a narrow Stage 3 provider boundary; durable runtime
witness/journal/receipt composition, tree-changing checkout and command
transport remain required. Stages 4-7 remain approved and incomplete, and
production writable capabilities stay unavailable.

E-M21-025 joins original retained reword publication to the private durable
checkout witness and typed receipt pipeline. Exact interrupted and
killed-process recovery, saved dependency repair, immutable witnesses,
historical terminal replay and real registered-folder physical policy are tested
separately from fixture combination/retirement outcomes. Version-3 witness
catalogs preserve older read-only records and reserve bounded growth before
publication. This private code/checkout execution is not production transport or
selected combination state. General checkout, first-use setup, actual
combinations, prepare/apply/status/discovery routes and all later approved
stages remain required; writable UI capabilities stay unavailable.

E-M21-026 removes the Prepared first-apply source-loss dependency on readable
current history. Authorized registration is resolved first; exact retained
operation/extra/index recovery runs under the original foreground lease before
source-backed current selection checks. Missing or changed combination state,
new editor bytes and revoked registration still prevent publication, and
terminal replay never repairs lost state. This remains private reword execution;
general checkout, actual combination selection and production transport/UI
qualification remain required.

E-M21-027 revalidates trusted immutable ancestry before unpublished retained
reword publication, including Unknown retry. Already-selected original code uses
acknowledgement-only recovery after policy changes, with no reannouncement if
the original candidate disappears. Bounded archived graph checks,
authorization/cancellation lifetime and real registered policy transitions are
qualified separately from actual combination state. General checkout, real
combination engines, production command transport and UI wiring remain required.

Known Published receipts also use acknowledgement-only recovery before witness
persistence, so later selected-head loss never repeats that publication.

E-M21-028 adds exact tree-changing checkout preparation under the original
retained lease. Plans bind real source/candidate trees, native metadata bytes,
affected file fingerprints and ancestor identities; preparation and original
revalidation leave physical files and checkout metadata unchanged. Resolved
regular/executable files, binary bytes, additions/deletions and full sparse
coverage are qualified for preparation. Unsafe links/modes, aliases,
file/directory transitions, dirty affected files and existing ignored/untracked
targets are rejected. Killed-worker reopening is provider evidence, not durable
server command or physical-installation acceptance. Persisting plans with the
original journal, retaining interrupted per-path installation and selecting
native tree-state/verified after-state remain required. Production writes remain
unavailable and all seven approved stages continue.

E-M21-029 adds the bounded private checkout-plan catalog and version-4 original
journal references. The journal authenticates and validates complete retained
preparation/effect linkage before plan access; synced plan selection precedes
reference selection under journal-then-plan lock order. Immutable exact replay,
unknown replacement acknowledgement, capacity pressure and process loss on both
sides of reference selection are tested. Existing format-1/2/3 reads keep their
bytes and terminal requests cannot gain new execution input. This is durable
input evidence, not physical installation or selected combination after-state.
Guarded interrupted installation, native checkout/tree-state completion and all
remaining approved stages remain required. Production Stacks stays read-only.

### Recommended scope and model

First qualify explicitly registered Git-backed JJ folders under the current
supported Git SHA-1/JJ profile. Follow with physically linked native workspaces
through the same owned interfaces, retaining native manifest/WAL publication. A
native repository merely hosted on loopback does not qualify as a local
checkout. Ordinary Git-only repositories require a separate provider or explicit
JJ initialization and are not silently converted by this feature.

Keep three identities distinct: repository, physical JJ workspace/device
binding, and named stack combination. One physical folder has one active
combination. Another linked physical workspace must not accidentally share its
active pointer, pending checkout or draft state. Do not hard-code `main` as the
target: use a configured local bookmark or already-known remote-tracking ref,
resolve an exact head and show a picker when it is absent or ambiguous. Normal
reads do not fetch, initialize a combination, snapshot files or reconcile heads.

Changeyard's JJ backend applies stacks by changing the parents of `@`; its
documented JJ support does not include long-lived workspace refs or committing
selected WIP as a new stack commit. JJButler's named workspace model uses a
separate merge and WIP commit per combination. Adopt the latter **concept** to
meet our independent unfinished-work requirement, with owned code and jj-store
publication/recovery rules. Neither existing engine is a drop-in dependency.

For each combination, retain a target selection, ordered applied stacks, an
internal merge commit and a child WIP commit. Rebuilding membership merges the
selected stack tips using JJ tree semantics, then reapplies the difference
between the old merge and old WIP. Carry unsnapshotted physical edits into that
exact preservation step. Removing the last stack leaves target plus WIP usable.
Applying a dependent stack includes its ancestry once; unapplying removes its
contribution without deleting its bookmarks or commits. Switching restores the
chosen combination's WIP. Shared branch rewrites invalidate other combinations'
derived merges; preserve their WIP and rebuild only through a reviewed command
when needed, never by mutating on a read.

Store names, active binding, request journals and combination configuration as
versioned private device state. Retain code trees through real JJ reachability
and owned internal refs with a reserved namespace; hide these from user branch
discovery and normal selected push. Specify GC retention and namespace collision
handling before use. Do not put device paths or active execution journals in the
portable companion ref. Code publication, device-state selection and physical
checkout are separate boundaries joined by an explicit recovery protocol, not
claimed to be one atomic transaction. Native code still publishes only through
manifest CAS; local Git-backed code still uses upstream JJ operation heads.

Derive stack segments from bounded ancestry and exact bookmark targets relative
to the workspace target. Maintain stable stack IDs separately from mutable
branch names. Do not impose the mock's linear ordering on merge commits,
divergent changes or conflicted bookmark targets: show the graph and disable
ambiguous mutations with a reason until those shapes are qualified. Above/below
commands must identify their parent boundary explicitly.

### Delivery sequence after approval

#### 1. Integrate the UI and define the production contracts

Bring the preserved UI slice together with current main in a dedicated
integration workspace. Retain newer graph reads, freshness and unrelated work;
preserve historical ledger entries. Update the owning UI/API/local-JJ chapters
with approved contracts before enabling runtime behavior.

Define a versioned workspace selection containing repository/profile, verified
binding generation, physical workspace identity, explicit code operation/view,
combination generation and working-tree fingerprint. Commands bind this
selection, exact source/destination identities, canonical intent digest and a
stable request ID. Preview returns a server-owned prepared ID and exact affected
commits, bookmarks, paths, conflicts and checkout effects. Apply names that
prepared result rather than recomputing against an unpinned latest state.
Preview may stage unreachable immutable candidates but does not publish heads or
write checkout files; apply revalidates its physical and logical preconditions.

Extend the adapter with bounded stack/branch pages, commit files, net branch
diffs and hunk reads, plus pending-request discovery. Keep unavailable, pending,
committed, rejected/stale and unknown outcomes distinct. Represent code
publication with incomplete checkout explicitly. Add per-action eligibility and
reasons, including unsupported graph/file shapes. The current controller and
mocks remain the consumer of those interfaces.

Acceptance: contract generation, adapter/controller tests for stale selections,
rewrite mappings and unsupported actions, and the affected app-shell stories. No
real mutation is enabled by this step.

#### 2. Deliver real browsing in the existing Stacks UI

Reuse device folder registration and native workspace verification. Implement
bounded stack discovery and operation-pinned history/file/diff queries in
`jj-store-jj`, exposed through owned service interfaces and existing HTTP/client
contract generation. Build the production adapter and inject it from the real
application lifecycle. The sidebar requires a verified eligible binding; a stale
direct link explains unavailability instead of constructing a mock. Keep
endpoints under existing LocalTrusted admission and same-origin policy; commands
name authorized binding IDs rather than arbitrary client folder paths.

Only visible branches/commits/files load details. Target history retains opaque
snapshot-bound continuations; other stack histories must also be bounded. Reuse
main's index preparation and cancellation states. Read current disk edits
without publishing a JJ operation, with scan/byte limits and a checkout
fingerprint. Watchers signal invalidation; they do not snapshot or assign work.

Acceptance: a real local folder shows target, stacks, inline files and multiple
adjacent diffs; long histories stay bounded. Missing, replaced, unlinked and
remote-only repositories cannot gain mutation access. Reads leave code heads,
companion state and physical files unchanged.

#### 3. Implement the recoverable local execution foundation

Complete the narrow JST-LOCAL-005 path needed by stacks. Keep upstream JJ
imports in `jj-store-jj`/CLI, service policy behind owned types, and device
binding and journal orchestration in the server/composition layer. Coordinate
with the foreground JJ working-copy lock and preserve concurrent operation
siblings.

Persist exact intent and prepared unpublished operation before publication.
Record checkout before/after trees, selected files, binding and combination
updates. Recovery distinguishes preparation, publication, checkout and receipt
completion. Detect external CLI/editor changes at apply and checkout boundaries;
never overwrite newly modified, ignored or untracked files because a preview was
previously valid. Do not release the pending state as successful until the
required checkout is verified. Use explicit recovery when new edits prevent
completion. Status and exact retry must survive process loss without generating
another command. A duplicate identity with different intent is rejected.

Acceptance: fault injection before/after each durable boundary, dropped
responses, restart, competing commands, changed folder identity, external edits,
concurrent heads and retry after undo/abandon. Qualify this before any writable
UI control.

#### 4. Make combinations and stack membership usable

Implement explicit first-use setup that preserves existing `@` and dirty files,
then create/switch combinations and apply/unapply stacks. Setup offers the
workspace target choice and shows its checkout effect through existing form and
preview components. It does not happen automatically when opening the page.

Implement merge/WIP rebuild and base-only fallback, with per-combination durable
unfinished work. Reuse the workspace picker, branch Apply/Unapply body action,
Unstaged column and conflict presentation. Derive membership from verified
durable state, not client booleans. Report missing/externally rewritten inputs
and require an explicit repair/re-preview instead of silently dropping work.

Acceptance: apply two independent stacks plus a dependent segment, preserve
unrelated WIP on apply/unapply, remove every stack, switch between combinations
containing different edits to the same path, restart and restore exact state.
Include overlapping stack changes and dirty checkout conflicts.

#### 5. Wire staging and inline commits

Keep staging as draft intent: dropping into a composer neither moves files nor
creates a commit. Commit preparation reads exact selected file/hunk content and
produces the real tree update, remaining WIP, rewritten descendants and bookmark
effects. Start with whole-file flows, then enable verified hunk transfers,
partial commits and split through the same exact-selection engine.

Use server-issued selection tokens tied to source/parent trees, path identity,
content/mode fingerprints and diff version; never execute client-supplied patch
text or a bare hunk index. Whole-file selection includes the exact selected file
state, not later edits. Directory drops remain the exact loaded file set.
Aggregate ownership that cannot be proven remains readable but not draggable.
Handle edits to the same path across stacks, revert-to-base, rename/delete,
binary/large files and unsupported modes explicitly.

Acceptance: stage/cancel leaves disk and history unchanged; commit only selected
files or hunks; move committed content to another commit or Unstaged; preserve
unselected edits; retain the draft on stale/unknown results and clear it only
after confirmed completion. Reload discovers pending requests; any optional
draft persistence uses private device state without credentials in browser
storage.

#### 6. Complete graph actions and recovery UI

Implement independent/dependent creation; branch and stack rename; empty commit
and branch placement above/below; reword/edit; commit moves and sorting; split,
squash, uncommit and rebase; then branch/stack/combination deletion. Show each
command's exact rewrite closure, other affected combinations and immutable
boundaries. Copy actions stay local to the UI.

Before deletion is enabled, settle the mock's ambiguous semantics explicitly:
deleting a bookmark and abandoning a segment are distinct operations. Preserve
lower/shared history and other workspace references, and identify any abandoned
changes in the preview. Do not reuse the mock's array deletion as policy.

Reuse existing conflict terms/fingerprints and application merge components for
multiple paths and native terms. Scope undo to an exact completed application
operation plus its device/checkout effects; reject or preview a new inverse when
later external work exists. Do not invoke repository-wide last-operation undo
blindly. Undo must not rewind unrelated companion collaboration metadata.

Acceptance: all exposed mutation menus have real capability/preview/apply/status
behavior, with before/after graph and tree evidence, immutable/shared-descendant
cases, conflict resolution, guarded undo and restart recovery.

#### 7. Qualify linked native workspaces and the complete lifecycle

Provide native execution through the existing prepare/WAL/receipt/manifest path
and a verified physical-workspace checkout coordinator. A native folder link
alone does not prove this coordinator exists. Keep the capability off until
manifest publication and disk checkout recovery are qualified together. Use the
same presentation and adapter semantics with explicit native identity types.

Attach repository/binding-scoped events, reconnect and bounded periodic refresh
to the existing application freshness owner. Preserve historical selections,
valid adjacent diffs, retained drafts and unresolved requests across refresh;
cancel reads and dispose subscriptions on repository switch/disconnect. Reuse
application toasts for transient outcomes and contextual recovery controls for
pending work. Browser routes retain the page, physical workspace and named
combination without allowing URL parameters to grant folder access.

Acceptance: cold filesystem-only and linked-native flows through the real app,
multiple folders and repositories, browser/server restart, external CLI/editor
changes, offline/reconnect, dropped events, unknown outcomes and missing links.
Qualify native publication with real storage evidence where required, separately
from fake-store tests. Retain existing repository log and ordinary branch views.

### Verification and completion

Each slice adds its exact regression first and records commands and results in
the single ledger. Run affected Rust packages and HTTP contract checks for
backend boundaries, web/catalog types and the specific stack stories for UI
wiring, ownership/architecture and owning specification/progress checks. Broaden
only for the changed shared surfaces under the repository test policy. Cold
tests are required for command durability, lifecycle and packaged serving; a
mock receipt or screenshot cannot qualify those gates.

The first useful delivery is real browsing, followed by durable apply/unapply
and named combinations. Full M21.2 acceptance additionally requires the
supported file/hunk, history-action and recovery paths in both advertised local
profiles. Push/PR submission, Git-only support, nested combinations, automatic
checkout migration and package publication are separate scope. Unsupported
actions stay explicit; do not describe a read-only or file-only milestone as
full completion.

Source anchors for this review are `web/src/lib/controller/stack-workspace/`,
`catalog/fixtures/stack-workspace/`, `web/src/App.svelte`, `spec/src/ui.md`,
`spec/src/api.md`, `spec/src/local-jj.md`, main's
`crates/jj-store-http/src/local_browser.rs`, and
`crates/jj-store-service/src/working_copy.rs`. Reference comparisons use
Changeyard's `docs/vcs/jj-supported-functionality.md` and
`src/vcs/jj/workspace.ts`, plus JJButler's
`spec/src/20-domain-model/entities/workspace.md`,
`crates/jj-workspace/src/stack_derivation.rs` and
`crates/jj-vcs/src/workspace_ops.rs`. These are read-only source references, not
package dependencies or an instruction to copy their authority model.

## Large repository read implementation

This work implements bounded native and local Git-backed JJ application reads.
The baseline is `b11c0486`; the isolated Jujutsu workspace is `bounded-reads`.
The current dev fixture and `/Users/stevejuma/code/linux` remain unchanged.

### Baseline and acceptance

The Linux read probe reached the local 64 MiB admission limit while reading
commit 19,467 of roughly 1.48 million reachable Git commits. Query information
and one-bookmark requests both failed before dispatch. Native reads likewise
capture the complete committed inventory before returning selected objects.

Qualify files, bookmarks, history and collaboration reads on both profiles.
Interactive work must follow the selected objects and page, with separately
bounded, cancelable preparation for derived indexes. Preserve native identities,
publication authority, exact source pins, current authorization, staged-object
exclusion, explicit conflicts and complete-result semantics. Retain persisted
formats and collaboration capacity limits. Do not enable deferred capabilities.

### Performance delivery first

E-READ-076 lands E-READ-072 through075 into clean main at the user's request,
preserving all slices and their exact source evidence. Verify main's combined
copy relation/query/backend, native source and RPC/lifecycle/task boundaries,
HTTP contract shape and affected package/generated consumers. The retained
workspace continues from this landing for remote selected-family delivery and
app preparation recovery. Complete qualification remains a later slice.

E-READ-075 delivers authenticated native kind tasks and strict
start/status/cancel HTTP contracts. Exact-root preparation shares the single
reserved background worker with native/local graph tasks, retains fixed
window/total admission and fresh current authorization, and reports bounded
progress without IDs. Expiry, cancel and teardown release source/credential
ownership after actual exit. Generated schema/client and registered response
capture accompany focused lifecycle correctness. App recovery and remote RPC
delivery remain following work. After this slice land E-READ-072 through
E-READ-075 into main, verify its focused combined source boundaries, and retain
the temporary workspace for follow-ups. Complete qualification stays deferred.

E-READ-074 adds callable native source APIs to begin, advance and finish kind
preparation at a current or historical checked root. Each advance reads at most
256 descriptors under its call deadline; original total work/read/byte/result,
allocation and lifetime limits remain fixed. Failed or canceled windows retain
conservative charges and leave identities/cursor unchanged. Terminal traversal
promotes complete matching IDs into the ordinary source-scoped kind cache;
partial results remain unavailable. Background task/HTTP/RPC/client integration
and the large-scale qualification remain following slices. Continue the task
delivery before indexed copy-record and selected family RPC work.

E-READ-073 wires pinned native copy-family capabilities through the
source-scoped selected relation cache and upstream backend. Cold construction
reads descriptors only; warm lookup reads no descriptors, and ordered delivery
loads matching copy bodies only. Complete addition proof reuses unchanged graphs
or extends copy relations under the shared memory allowance. A captured
acknowledged staging sequence retains exact parents and descendants while
excluding unknown, foreign and later writes. Historical pins, fresh
bounds/authorization and cancellation remain explicit. RPC capability delivery,
indexed copy-record path/range lookup and resumable cold construction remain
next work. Full qualification stays deferred.

E-READ-072 adds an immutable bounded copy relation index and exact family
selection over retained parent/child adjacency. Construction rejects incomplete
or cyclic graphs; lookup follows ancestors and their descendants while excluding
unrelated parent branches of merge descendants. Node/edge/work/byte admission,
cancellation and deadlines are separate from selected body delivery. Integrate
the primitive with source-scoped native projection retention, pinned read
capabilities and acknowledged staged copies next. Endpoint behavior is unchanged
in this primitive slice; full qualification remains deferred.

E-READ-071 lands E-READ-069 and E-READ-070 into clean main at the user's
request. Verify the combined native query/preparation, factory/RPC/lifecycle and
WAL projection surface using main's sources, retaining byte-identical slice logs
and all historical ledger entries. Keep the temporary workspace open on this
landing and continue reverse-copy lookup and resumable cold preparation. Full
qualification remains the separate later slice.

E-READ-070 adds bounded selected-inventory projection pages retaining operation
IDs and copy-parent edges at the first descriptor read. Whole-root and addition
windows discard unrelated identities, report processed descriptors and charged
record reads/bytes, preserve historical selections and exclude staged
candidates. Per-page descriptor, edge and encoded-record limits keep progress
resumable. This primitive loads no native bodies and changes no persisted
format. Next integrate source-scoped reverse-copy family lookup and cold kind
preparation; JJ requires descendants of the selected copies' ancestors as well
as ancestors.

E-READ-069 routes pinned browser and explicit preparation operation/copy reads
through the exact source-scoped kind provider. The retained checked manifest and
original caller deadline stay explicit; unrelated descriptors charge scan work
rather than matching results. Historical source wrappers retain this capability.
Relevant copy/path indexing and resumable cold preparation remain the next
delivery targets, with complete qualification separately deferred.

E-READ-068 lands the follow-up E-READ-063 through E-READ-067 stack into clean
main and verifies its combined native/local history, inventory and factory/RPC
consumers there. Preserve all original evidence and keep the temporary workspace
open, rebased onto the verified landing. Next delivery targets pinned browser
kind/copy lookup and cold large-root preparation; complete qualification stays
deferred.

E-READ-066 adds bounded inventory addition pages over selected immutable roots.
Changed-path traversal and exact baseline retention replace unrelated descriptor
reads at this primitive. E-READ-067 integrates source-scoped complete
operation/copy projections and advances them through exact additions, with fresh
request checks and independently bounded retention. Complete RPC convenience
shares this lookup. Cold preparation, progressive paged RPC semantics and
complete qualification retain their separate contracts.

E-READ-065 adds exact prepared predecessor occurrence lookup for native/local
evolution, skipping unrelated bookmark-operation bodies while preserving the
first legacy boundary and complete JJ ordering. Disposable history format3
retains formats1/2 replay compatibility; current source authorization and
selected-operation validation stay request-owned. Full qualification and the
other remaining delivery targets stay separate.

Replanned 4 October 2026 at the user's request. The first 23 bounded-read slices
are integrated with current main at `f5cd62d4`. Continue in the existing
`jj-store-bounded-reads` workspace. Deliver the remaining performance features
in narrow Jujutsu changes before a separate complete-qualification slice:

1. Reuse prepared local historical membership for selected summaries,
   directories, files, diffs and conflicts. Keep directly referenced cold reads
   independent of preparation. Then add prepared visible change/reference
   windows and operation prefix/comparison lookup; hydrate only selected rows.
2. Retain native prepared visibility and reverse annotations, immutable cache
   generations and shared eviction. Admit resident and additional-operation
   growth before allocation and keep cache locks short.
3. Add indexed deep continuations, operation/predecessor/copy/search lookup and
   paged native RPC contracts, preserving pinned JJ query semantics.
4. Add root-bound collaboration/search projections and keyset pages, preserving
   existing domain capacities. Use lightweight repository summaries and load
   visible card windows with at most two concurrent reads; refresh loaded
   windows and cancel obsolete requests.

Each delivery change names and runs its exact correctness regression and focused
checks on the affected packages and direct boundary. Record actual evidence and
remaining gates in the milestone ledger. New public contracts still require
focused schema/transport checks. Do not rerun complete integration, all
contracts, full cold-browser flows, large fixtures or RustFS qualification for
each delivery slice. Generated Help/source-review refresh is deferred with the
qualification slice unless a delivery changes its own documentation tooling.
Passing focused checks does not complete the plan's acceptance or promote a
milestone.

Delivery evidence E-READ-025 through E-READ-031 covers prepared local point and
visible-reference reads, native retained visibility/reverse labels, shared idle
resident eviction, viewport-owned repository metadata reads, and native
root-bound WorkStore projections with selected-window summary hydration, and
native application content indexes with retained query keys. Visible card
loading now uses finite selected-view code-head summaries and compact review
activity. E-READ-041 reuses only numeric review activity facts for the exact
admitted root/window behind fresh native/local provider validation, with fixed
expiry, bounded idle retention and duplicate-load coalescing. E-READ-037 adds
cross-request local WorkStore root/projection and exact-query reuse behind fresh
companion/domain admission. Native missing-operation construction now uses the
owned bounded format writer with compact, admitted scratch for interactive and
small-cache reads. Immutable resident lookup also reuses separately prepared
historical operations without copying another slot. Cold historical operations
now build under separate full reservations and retain verified data only. Graph
evaluation releases the namespace after completing the immutable primary
generation, including filtered queries. Missing primary generations now use
private reserved staging and reacquire the namespace only for
complete-generation installation. Existing-disk projection recovery now moves
verified files into an actual-size private reservation and releases the
namespace before decoding and deriving visibility. It preserves hard/inline
limits, exact source files and cancellation cleanup; it copies no index and
reserves no unrelated construction maximum. Local operation/deep lookup and
local application search projections remain delivery work. These focused
deliveries do not complete the later combined qualification gate.

E-READ-042 adds sorted retained local operation-prefix lookup with historical
metadata fingerprints, charged lookup work and existing cold HTTP recovery.
Ready prefix pages need no commit/tree/index preflight; plain metadata pages
retain their cold reader. Prepared operation comparison, indexed child neighbors
and deep continuations remain separate delivery work.

E-READ-043 adds prepared historical operation/view admission and bounded
visibility comparison through the verified commit index. It stops at shared
ancestry and groups change IDs before selecting a page, hydrating only the
selected added targets. Complete scale/lifecycle qualification and deeper
operation/reference/copy/search/RPC continuations remain separate work.
Changed-frontier discovery still completes under one ordinary visit budget per
request; retained continuation for oversized differences and repeated group
discovery remains delivery work.

E-READ-044 adds composed native unfiltered Changes/All reference windows through
retained ordered visible change groups. Indexed identities determine lexical
pages before selected-target hydration; reverse workspace names avoid scanning
bookmarks. The shared native executor retains ordinary content limits, cold
exact-snapshot preparation recovery, signed cursors and namespace release before
hydration. Description-filter indexes and differently embedded operations remain
separate delivery work, together with deep/difference/search/RPC continuations.

E-READ-045 adds local JJ description/reference substring filtering through an
operation-bound disk projection. It captures normalized descriptions during the
existing preparation read, intersects compact lexical postings and checks only
candidates against pinned visible targets. Source hydration follows selection;
Unicode matching, divergence, removed bookmark terms and signed forward/backward
windows remain exact. Native filtering, extreme candidate continuations and
differently embedded operation lookup remain delivery work. Full construction
scale, disk footprint, RSS and latency qualification remains deferred.

E-READ-046 compacts native prepared visibility/reference identities from
tree-based membership, formatted keys and per-group target vectors to sorted
flat arrays and compact group boundaries. Construction sorting is in place,
work-charged and cancellable. The conservative identity reservation drops from
512 to192 bytes per indexed commit; the two-million-row admission regression
leaves headroom with unchanged JJ decode accounting and the unchanged1GiB pool.
Exact lexical windows, arbitrary anchors, divergence, hidden prefix offsets and
selected-only hydration remain governed by focused differential/native tests.
This is a prerequisite for native filtered delivery; filter-file lifetime/quota
ownership and the remaining deep/search/comparison/RPC continuations are still
open. Full large-fixture resource and lifecycle qualification stays deferred.

E-READ-047 adds native prepared description/reference filtering and explicit
upgrade of index-only generations. Selected candidate keys cross exact current
visibility and description checks before native target hydration, preserving
Unicode, divergence, removed bookmark terms and both cursor directions. A shared
generation pin keeps readonly filter files and their complete slot charged
through active readers. Pressure may reclaim idle filtered residents; pinned
recovery remains in place without deleting another reader's files or duplicating
quota. Small-cache memory-only policies keep their ordinary bounded fallback.
Different embedded operations, cold full receipt hashing outside the namespace,
and deep/difference/search/candidate/RPC continuations remain delivery work.
Full scale, fault/lifecycle, memory and production browser qualification stays
in the deferred slice.

Resident retention charges actual framed index allocations and the bounded
database cache; complete filter files remain in disk quota. The large-file
admission regression is arithmetic evidence, not measured RSS or scale serving.

E-READ-048 moves cold native receipt hashing after independent quota ownership
and namespace release, before loading and allocating decode. Admission scans
bounded names and sizes; healthy immutable recovery keeps its actual-size
reservation. Checksum damage in privately owned files rebuilds through fresh
admission; shared damage fails while preserving readers and quota. Work,
cancellation and deadline failures remain failures. Coarse metadata performs
full proofs before loading and before namespace reacquisition, with no warm
resident retention. Explicit filter upgrades reserve writable construction
capacity before growth, and failed admission before transfer preserves existing
files. Different embedded operations and deep/difference/search/candidate/RPC
continuations remain delivery work. Focused regressions do not complete the
deferred large-fixture, RSS, latency, fault and browser qualification.

E-READ-049 integrates delivery through E-READ-048 with current main freshness,
Help and directory-query ownership. Combined source verification covers native
and local query consumers, merged public routes and response captures, and
repository UI/type/boundary checks. Both parent ledgers remain intact. This
landing retains the temporary workspace for the remaining features and does not
complete large-scale, RSS/latency, fault/browser/RustFS or release
qualification.

E-READ-050 delivers composed native reference windows at a differently embedded
operation. Source membership is admitted through the unchanged outer capability
before cache work, and the source's own prepared visibility/filter determines
the lexical window and selected target hydration. Cold preparation remains
explicit and pinned to that source. Newer or staged operations cannot acquire
membership from a ready cache. Local cross-operation recovery and remaining
child/deep/difference/search/candidate/RPC delivery stay open, together with the
separate comprehensive qualification slice.

E-READ-051 selects exact prepared local source operations after bounded parent
metadata admission, with source-specific cold recovery and unchanged outer
snapshots/cursors. Reject newer and unrelated warm selections. Local browser
validation checks both pins. Land E-READ-050/051 into main and verify the
focused combined surface before continuing indexed child/deep continuations
here. Complete scale and release qualification stays deferred.

E-READ-052 lands E-READ-050/051 into the clean main workspace by advancing its
empty change onto the verified delivery stack. No parallel main changes or merge
conflicts require reconstruction. Verify the combined native/local reference and
shared transport/cache consumers in main, preserve historical evidence, then
retain and rebase bounded-reads for further delivery. Full scale and
Help/generated-reference/source-review qualification remain deferred.

E-READ-053 delivers indexed local child-operation selectors through a bounded
sorted edge projection and selected-only metadata hydration. Mixed suffixes,
ambiguity and later-head exclusion retain upstream semantics. Historical
prefix-based cold child selection needs explicit preparation; current-relative
selectors retain their bounded cold reader. Native child selection and deeper
graph continuations remain separate work; main retains the E-READ-052 landing.

Repository directory task counts now consume one body-free repository/project
aggregate page from each visible metadata home. Returned groups count their
complete selected root and preserve maximum task timestamps, cross-home/project
filters and unloaded-group partial state. Native requests reuse the admitted
root projection; local transport independently admits its explicit companion
selection. Card reads no longer hydrate up to 256 task summaries. Local reads
now reuse immutable roots, projections and eight exact-query key sets across
independently reopened facades. Browse/activity/detail retain fresh companion
and Work-blob admission, source limits and folder identity checks. Root
retention, cold derivation and live readers have separate bounded lifetime
accounting; selected detail windows follow precomputed task/comment keys.
Compact review activity now returns a bounded catalog-window count and maximum
accepted event time without hydrating anchors, comment conflicts or bodies.
Snapshot-page activity streams times without constructing presentation events;
merged activity needs no causal ordering. Native/local directory cards retain
their existing partial-count and date policies. Fresh access, complete provider
closure admission and independent root/companion pins still apply. Repeated
review activity windows reuse counts and times without retaining providers,
envelopes or evidence objects. Fixed expiry, idle-only eviction and
request-local saturation fallback preserve source admission and domain capacity.
Code summaries read only the first lexical nonvirtual head window and return its
newest sampled committer metadata; unvisited ancestry or additional heads keep
counts explicitly incomplete. Cards request 50 heads without invoking history
preparation. Native/local request admission and historical pins retain their
existing authority; no trees, ancestors or graph indexes are read for these
summaries.

E-READ-054 continues native indexed child-operation selection in bounded-reads.
Reuse the existing preparation operation pass and retain admitted identity and
reverse-edge lookup for metadata-only warm pages. Preserve point membership,
upstream ambiguity, exact outer pins and selected-only hydration. Main remains
at E-READ-052; deep graph continuations and complete qualification remain open.

E-READ-055 removes selected-view loading from prepared local operation graph
admission. Fresh operation fingerprints, access, generation and source
membership remain required; code readers retain the combined operation/view
seal. Plain and metadata-filtered operation pages now reuse a ready generation,
with supported cold reads unchanged. Land E-READ-053 through E-READ-055 in main,
verify the focused combined read surface, and retain bounded-reads for indexed
deep paging and the remaining feature work. Complete qualification remains
deferred.

E-READ-056 lands E-READ-053/054/055 in main and verifies the combined focused
native/local operation read surface. The clean main change advances onto the
completed stack without replacing source files. Keep bounded-reads registered,
rebase its clean continuation onto this landing, then deliver indexed deep
operation-history pages. This landing does not complete scale, Help or release
qualification.

E-READ-057 adds compact prepared operation topology for native/local candidate
paging. Keep exact JJ merge/timestamp ordering and source admission while
skipping operation bodies before deep anchors. The projection is disposable;
retained traversal checkpoints still need delivery to avoid replaying compact
edges. This follow-up remains in bounded-reads on the E-READ-056 main landing.

E-READ-058 retains the captured selection's exact operation order and inverse
ranks during generation admission. Selected-start pages seek to deep anchors and
scan only subsequent predicate candidates; fresh source admission and
anchor/page/probe semantics remain intact. Historical-start continuation follows
in E-READ-059; deep log/evolution and other retained/indexed readers remain
delivery work.

E-READ-059 delivers bounded historical operation frontier/chunk checkpoints
using the pinned JJ traversal. Retain data only at the page boundary before
lookahead; current source admission and anchor/filter validation stay fresh.
Missing/evicted checkpoints use bounded compact fallback. Deep log/evolution,
lookup/difference/search/RPC delivery and full qualification remain open.

E-READ-060 adds bounded manifest-pinned native RPC inventory scan pages and
routes the existing NativeClient complete-result adapter through them. The
hashed catalog requires scanning unrelated kinds, so sparse windows explicitly
continue. Seek radix paths after each signed anchor; admit descriptor count,
record reads and bytes before decoding. Reauthorize each request and preserve
the original root across later publication. Whole enumeration has its own
result, total-scan and deadline bounds. Deep log/evolution and kind-specific
predecessor/copy/search lookup remain delivery work; complete qualification
stays in the later slice.

E-READ-061 routes legacy RPC and in-process native operation/copy enumeration
through the bounded selected descriptor windows. Retain only matching IDs under
result admission and enforce total scan/time limits independently. Fresh access
and epoch checks accompany each window; semantic publication keeps its complete
closure audit. Windows now end early with a continuation when record work or
cumulative bytes fill their admission. Independent descriptor allowance admits
the existing 16 MiB record format under a 32 MiB cumulative ceiling. Dedicated
kind lookup and the remaining deep/indexed read features still require delivery.

E-READ-063 retains exact prepared native/local visible order and compact lexical
ranks for default `all()` history pages. Seek deep anchors directly, preserve
the ancestor-closed set's ordered direct edges, and hydrate only the selected
page. General filtered revsets/evolution and the other retained/indexed readers
remain delivery work; this default-query optimization does not complete them.

E-READ-062 lands delivery through E-READ-061 into the clean main workspace and
verifies the combined prepared-operation and inventory surface there. Preserve
all original evidence and existing fixtures/services. Keep bounded-reads open
and resume performance delivery from the verified landing, beginning with deep
history continuations. Complete qualification remains a separate later slice.

E-READ-064 adds prepared native/local body-free evolution traversal and bounded
data-only predecessor checkpoints. Preserve pinned upstream ordering, legacy
boundaries and current source proof; reuse exact start/anchor state for repeated
continuations only for proven unique known anchors. A checksum-bound
conservative repeat bitmap in disposable history format2 preserves
first-occurrence cursor semantics, and format1 remains readable without
checkpoint reuse. Selected graph visibility/divergence uses point lookups over
existing visible groups. General filtered log seeking, uncached/ambiguous
extreme evolution and dedicated predecessor/copy/search lookups remain separate
delivery work.

### Deferred complete qualification

After performance delivery, qualify the combined implementation separately:

- Cold/warm Linux construction, current visibility policy, isolated server RSS
  and release latency; native two-million-commit fixture, 10,000 references,
  large/deep directories, deep pages and near-capacity collaboration data.
- Cache loss/corruption, cancellation, restart, revocation, publication races
  and retained-metadata pressure while preparation is active. Measure work,
  latency and memory separately; earlier debug Linux timing is not latest-policy
  proof.
- Complete affected integration/contract inventory, cold production browser
  flows and real-RustFS acceptance, plus bundled Help/source-review refresh.
- Registry-only release qualification and separately approved visual baselines.

No qualification failure is waived by this execution split. Existing authority,
format, identity, authorization, conflict and complete-result contracts remain
required throughout delivery; comprehensive verification stays explicitly
pending until its later evidence is recorded.

### Endpoint audit

| Surface                           | Baseline work                                                   | Required read shape                                                         |
| --------------------------------- | --------------------------------------------------------------- | --------------------------------------------------------------------------- |
| Local code query                  | Complete retained operations, commits and trees before dispatch | Operation/view access; scoped membership; prepared historical lookup        |
| Native read session               | Entire committed inventory and dependency audit                 | Checked immutable catalog root and point membership                         |
| References/workspaces             | All targets and changes; browser drains every page              | Exact resolution, independent workspace pages and selected target hydration |
| Paths/commit header               | Descendant scanning and attribution; graph for header           | Direct directory page, independent metadata batch and object summary        |
| Query cache/history               | Whole cache hashing; total-commit admission                     | Verified immutable generations and actual work budgets                      |
| Operation differences/copies      | Whole set materialization before paging                         | Indexed or retained bounded continuation                                    |
| Code search visibility            | Complete ancestor set per read                                  | Operation-bound derived visibility lookup                                   |
| Work/Agent/Review/Thread/Reaction | Complete cold domain reconstruction                             | Validated root projection and targeted records/ranges                       |
| Application search                | All current bodies and all matching results                     | Root-bound search index and keyset continuation                             |
| Repository cards                  | Task/review/history pages for summaries                         | Lightweight summaries for visible cards                                     |

Each verified slice updates the canonical chapters and `spec/src/milestones.md`
with actual commands, failures and remaining gates before its Jujutsu commit.
This document is an implementation checklist, not verification evidence.

## Agent capture: usable local preview

Replanned 3 October 2026 at the user's request. This replaces the previous
execution order of finishing all readers and M18.3–M18.7 before delivering a
usable capture feature. Canonical requirements remain in `spec/src`; the only
status/evidence ledger is [milestones.md](../spec/src/milestones.md).

### First outcome

Open the current application, use Codex normally in a repository directory or
another physical JJ workspace linked to this local server, and see its captured
session, messages and tool activity appear in Agents automatically. Settings →
Agents uses the shared bordered rows and action columns independently of the
repository picker, and shows whether capture is enabled, observing sources,
paused, catching up or unavailable. Pause, resume, rescan and the existing Codex
history import are usable without repeatedly clicking Refresh.

The first preview supports LocalTrusted native repositories and current
AgentStore only. Configured profiles are Codex CLI 0.156.1 and Codex Desktop
0.159.2. The CLI installed during this review is 0.156.1. Desktop version and
actual transcript format must be checked before advertising that surface in the
preview. A filesystem-only companion repository remains outside automatic
capture. Do not silently ingest every session under a watched directory: active
sessions require an exact linked repository/workspace; inactive history requires
an explicit import. Watching stays disabled until server configuration enables
selected harnesses. Existing personal transcripts are not a test fixture.

### Reuse what exists

| Area             | Existing foundation                                                                                                                       | Work needed for the preview                                                         |
| ---------------- | ----------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------- |
| Codex worker     | Configured startup, known locations/overrides, directory/workspace admission, active backfill, bounded tailing, durable spool and restart | Integrate with current main and prove the real preview path                         |
| History          | Explicit Codex import, retained original intent, recovery and receipt-backed progress                                                     | Keep the existing feature usable; do not wait for OpenCode history                  |
| Capture settings | Status, sources, pause/resume/rescan/import, typed client and unknown-outcome handling                                                    | Automatic loading/freshness, clear state labels, capability-aware actions           |
| Agents           | Recorded sessions and common response/tool presentation                                                                                   | Reuse main's newer repository/project Agents views and automatic refresh            |
| OpenCode         | Qualified library/API acquisition, active supervision, identity joins and exact prefix preparation                                        | Configured local startup reopened on 4 October; history/import/export stay deferred |
| Main application | Automatic updates, scoped Agents sidebar, newer settings, UI development commands and desktop host                                        | Rebase and preserve these improvements rather than recreate them                    |

The delay came from treating a broad completion roadmap as a prerequisite for
trying capture. Eighty capture changes now exist beyond the common ancestor,
including repeated recovery/publication qualification and another reader. The
execution target is now the supported Codex workflow, visible in the current UI.
New hardening work enters this target only when it fixes an observed preview
failure or protects an existing required safety boundary.

### Delivery order

#### 1. Preserve work and rebase onto current main

Execution uses new workspace `jj-store-codex-preview`, initially based on main
`soytvzqt` / `89243006`, then integrated onto `rsouwqrt` / `c8b7ea72` and
refreshed onto committed main `zulyxxpy` / `90d119e0` on 4 October. The original
capture tip is retained at `codex/capture-before-preview`; its net verified
changes are rebased as one integration slice, avoiding replay of 81 historical
ledger edits. Desktop keeps M17, capture tasks use M18 and historical evidence
uses the explicit ledger mapping. The parked OpenCode draft remains excluded.

The unfinished OpenCode API import draft is retained at Jujutsu change
`ovlvtqrl`, bookmark `codex/deferred-opencode-history`; it is excluded from the
preview working change. Its two integration regressions currently fail because
inactive API sessions receive no import selector. The draft is incomplete and
must not be treated as a buildable feature or merged to obtain the preview.

At review, the last committed main revision is `wllxluns` / `0400ae22`; main's
working change `vssplolt` contains active desktop/glass edits. The common
ancestor is `lwqssrow` / `2852a3e3`. Main has 51 committed changes and capture
has 80 changes beyond it, with 40 changed paths in common. Recompute these facts
immediately before integration; counts alone do not establish conflicts.

1. Snapshot both workspace states and pin the selected committed main revision,
   capture tip, draft and current JJ operation. Keep the draft bookmark intact.
2. Rebase the verified capture branch and this plan onto the latest verified
   committed main revision in an isolated JJ workspace or integration change. Do
   not rewrite or check out main's active working change. If its edits land
   during integration, review the delta and update the destination before final
   acceptance.
3. Resolve shared controller/lifecycle/settings files in favor of current main's
   architecture, then add capture ownership and routes. Preserve scoped Agents,
   automatic refresh, appearance, desktop, Storage and Sync behavior. Regenerate
   OpenAPI/TypeScript after reconciling route registration; do not hand-merge
   generated output. Keep main's released dependency versions unless an exact
   incompatibility requires an owned fix.
4. Resolve the milestone collision: main M17 owns desktop; this branch's
   historical M17 owns capture. Reserve M18 for capture if still free. Keep
   every historical evidence ID and payload intact and add an explicit
   old-to-new task mapping for capture evidence in the ledger/governance
   checker. Preserve both parents' evidence and their order, deduplicate only
   byte-identical entries, and verify no evidence was dropped. Do not reclassify
   desktop evidence as capture or rewrite historical task fields to make a merge
   appear clean.
5. Run focused integration checks for the actual conflicts and shared consumers.
   Record the merged format/contract inventory and outstanding failures. A clean
   rebase alone is not acceptance.

Exit: a conflict-free integrated branch with the current application, preserved
main work, isolated draft, reconciled ledger and passing affected checks.

#### 2. Prioritize the UI

Use the existing production Capture settings and current main Agents views.
Inspect Community and then Design Core before changing presentation; maintain
repository ownership and provenance for any copied presentation.

- Load capture status and the first source page when settings opens. Refresh
  visible capture status/source/import observations on a bounded five-second
  schedule, after acknowledged actions and on focus/reconnect. Coalesce reads,
  cancel stale responses and stop visible polling on closure/disposal.
- Show enabled/disabled, paused/running, waiting for activity, catching up,
  published and unavailable states in plain language. Keep advanced request IDs
  and receipt detail available without making them the main workflow. Show
  configured harness/version support and a useful disabled-state setup hint.
- Preserve exact pending commands, their original revision/generation/selector,
  drafts, pagination and uncertainty. Background reads never resend a command,
  resolve an unknown outcome from a missing receipt, or retarget an import.
- Disable unavailable harness actions. Do not offer OpenCode/Cursor history
  import because a generic source row happens to expose an import button.
- Reuse main's automatic repository/project Agents updates after publication.
  Retain the selected session, reply/edit drafts and historical code selection.
  Show new captured content without a mandatory manual refresh or navigation
  reset. Keep permission, empty, loading and unavailable states usable.
- Exercise the actual Settings-to-Agents flow with keyboard and accessibility
  checks, and a fresh browser process. Visual baselines remain a separate
  review.

Exit: the UI visibly follows new captures and worker controls while preserving
user state. No new conversation/review architecture is required for this slice.

#### 3. Make local setup runnable

Use main's `dev:ui` and backend-only build/restart workflow after integration.
Prepare an isolated ignored fixture and unused loopback ports; do not reset,
replace or restart the currently running main fixture or installed desktop app.
Build the explicit development binary once for this integrated branch.

Add a small repeatable preview setup command/runbook that creates or uses a
preview-owned native repository and linked physical workspace, ensures current
AgentStore is enabled using existing commands, and writes the selected supported
`agent_capture` server configuration. Show the exact config and how to stop the
preview. Explicit test roots isolate fixture tests; normal qualified harness
locations are exercised only for the deliberately created trial session, with
native repository matching still enforced. Configuration requires restarting the
preview backend once; pause/resume remain runtime controls.

First setup may be a command plus a clear Settings explanation. A server-config
editor, hook-installation wizard and one-click enable switch are deferred. Do
not invent settings endpoints just to avoid a documented one-time setup step.

Exit: reproducible local setup, a reachable browser URL, and a linked folder in
which the user can start the qualified Codex harness normally.

#### 4. Prove the user journey and hand over the preview

Use the real development server and embedded RustFS with a disposable native
repository. Record a supported Codex CLI session with distinctive harmless
content (and a small edit if the harness is already available and authorized).
Verify directory discovery, publication and display in Agents; append another
message and observe automatic UI refresh. Exercise pause/resume and restart the
preview backend, confirming the same session, no duplicate acknowledged content
and retained pending intent. Check another physical workspace of the linked
repository, an unrelated directory refusal, and one explicit Codex historical
import. Current permission/redaction/unknown-outcome tests remain required.

Qualify Codex Desktop independently only when the actual version matches. If
that check fails, expose CLI-only availability and retain the Desktop gap;
unknown versions must not be enabled just to obtain a green preview. If a model
session requires a user interaction, keep the preview running and present the
single concrete trial step instead of spending another cycle on unrelated
backend features. Controlled fixtures remain useful but are not labeled real
harness acceptance.

Hand over the browser URL, linked test folder, selected harness/version, setup
and stop commands, and a short checklist: start session, watch Agents update,
pause/resume, restart and inspect retained content. Leave the preview available
for user testing. Broader completion stays deferred after this handoff.

### Deferred work

| Work                                                                                                 | Reason and re-entry condition                                                                                                                      |
| ---------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------- |
| OpenCode inactive API history and exported JSON files                                                | Retain separately parked history draft; configured active capture is reopened below                                                                |
| Cursor advanced identity joins, cloud sessions and additional platforms/versions                     | Local Desktop/CLI transcript capture is reopened below; retain separate qualification gates                                                        |
| M18.3 complete block/plan/child-agent/compaction model and replay/renormalization                    | Existing messages/tools are sufficient to try capture; resume as a separate conversation milestone                                                 |
| M18.4 automatic checkpoints, edit causality and shared owned producer scopes                         | Do not claim transcript observation proves code authorship; resume separately                                                                      |
| Full M18.5 multi-harness release matrix and remaining platform/cold qualification                    | Keep the narrow Codex real-service/browser/restart proof above; defer the full release gate                                                        |
| M18.6 advanced inspection/search, exact message/plan reviews, immutable feedback dispatch            | Existing Agents browsing is the preview; expand after use                                                                                          |
| M18.7 collaboration archives, portable carriers, broader deployment/privacy policy and purge         | Separate release scope; existing local privacy/admission rules remain mandatory                                                                    |
| Configuration editor, hook setup wizard and additional platforms                                     | One documented local setup is enough for the preview                                                                                               |
| Cold debug sanitizer/native preparation optimization and clearer body-free history selection/refusal | Functional linked capture/import is qualified; reduce observed multi-minute preparation and unlinked unknown-command friction after trial feedback |
| Official ACP Rust SDK adoption                                                                       | Not a required capture outcome                                                                                                                     |

Legacy AgentStore support and migration are excluded, not queued for later.
Existing implemented features are preserved. Deferral does not authorize removal
of current privacy, directory matching, Write, bounded-resource, durable receipt
or manifest publication guarantees.

### Cursor and OpenCode extension reopened 4 October 2026

The approved extension adds separate expandable Cursor Desktop/CLI and OpenCode
configuration, five-profile contracts, configured OpenCode API startup and
metadata-only Cursor hook handoff to the shared bounded JSONL reader. Defaults
remain opt-in, LocalTrusted and current AgentStore in exactly linked native
repositories. Configuration saves require a backend restart. The implementation
uses existing sanitation, spool, source registry, receipts and Agents views.
Nullable Cursor start paths require a bounded private lifecycle handoff; mutable
OpenCode assistant snapshots wait for final completion before durable enqueue.

Qualification records fixtures and disposable installed-harness runs separately
in the milestone ledger. Desktop automation must verify the disposable workspace
before sending any prompt. Keep unfulfilled real-harness/cold gates explicit;
fixture acceptance is not product or release qualification. Rebase against the
current main workspace before local integration. Preserve the independently
parked OpenCode history draft. Inactive history, export/database readers, cloud
Cursor, advanced subagents, setup wizard and additional platforms/versions stay
deferred.

### Verification budget and stop condition

This planning change runs documentation formatting, spec validation, spec-first
and ledger checks only. During execution, start with each changed boundary's
regression. UI-only work uses `web:check`, the capture controller tests, Capture
settings stories and affected Agents/browser tests. Run Rust package checks and
named capture tests only for changed Rust boundaries. Shared route generation
requires the registered contract inventory. Rebase touches independent layers,
so record its actual overlap before selecting the complete directly affected
consumer suites. No root aggregate lanes by default.

The preview is ready when the integrated browser and real local service satisfy
steps 2–4 and the user has a runnable URL/folder. A UI mock or an isolated
library test is insufficient. This does not mark the full capture roadmap
complete. The ledger records scope and evidence; this document supplies
execution order.

## Summary

Create `/Users/stevejuma/code/lapismd/jj-store` as an independent Rust workspace
initialized with `jj git init --colocate`. Follow jj-nostr's specification, CLI
presentation, validation, and verified-slice commit conventions.

The first release supports **new native repositories** through a custom JJ CLI,
LocalTrusted loopback server, persistent embedded RustFS, structured APIs and
the browser UI without user authentication. Shared access, background search
indexing, backup/restore, replication and existing-repository migration remain
explicit follow-on milestones.

The development repository is Git-colocated; repositories served by the product
use native JJ storage without requiring Git, SQL, or Nostr.

## Execution scope update — 27 September 2026

The user reopened Storybook-only work: directly adapt Community's shell,
navigation, graphs, branch rows, files and diffs. Branches (JJ bookmarks), JJ
Log and Op Log are separate pages; fixture state demonstrates drill-down to
diffs. M4.4 owns tooling/provenance, M4.5 shell mechanics, M4.6 repository
views. Production UI integration, mutations/search wiring and deployed
acceptance remain deferred under M4.3. The existing browser and backend work
remain preserved.

The user subsequently reopened production browsing integration. M4.9–M4.14
promote the reviewed Storybook application into the served browser, add missing
bounded read contracts, typed hash restoration, explicit SSE refresh,
virtualized collections and persistent layout. M4.15 adds the authenticated
repository command search against the selected Tantivy snapshot. M4.16 adds
preview/apply/status controls for core repository commands using the durable
native mutation contract. M4.17 adds complete native conflict descriptors,
bounded staged uploads and atomic multi-path resolution through that same
workflow.

M4.10 adds separate signed browsing windows for bookmark/change lookup,
recursive paths with bounded last-change attribution, operation changes grouped
by native change ID and exact comparison paths. The prior complete read routes
remain compatible. Stable record/path anchors can begin forward or backward
windows; opaque server cursors remain process-local continuations and never
become durable URLs.

M4.13 proceeds in verified slices. The first slice persists shell, per-page
Column Canvas and file-tree geometry through versioned layout keys, injects
isolated memory storage in Storybook, and bounds the existing fixture-backed
bookmark, graph, changed-file, directory and file-tree collections. Remaining
work continues commit/operation inspection cursors, removes eager recursive tree
reads, adds lazy directory expansion, validates the upstream FileListing change
in Community and consumes a released Design Core version before M4.14.

## Execution scope update — 28 September 2026

M8a now owns only the trusted-local macOS application release. A0-A2 qualify
Rust 1.98.1, released embedded RustFS 1.0.0, strict format-2 local
initialization, LocalTrusted Unix and guarded loopback transports,
installation-owned repository discovery, persistent restart, bounded startup
diagnostics and the cold built browser. Older configuration compatibility is
removed because the application has not shipped.

Authenticated/shared serving, external S3 authority, background search indexing,
backup/restore, generic-S3/MinIO qualification, RustFS replication,
Linux/Windows packaging, automatic failover, writable promotion, destructive GC,
delete replication, multi-writer authority, existing-authority migration, Git,
Nostr, Smart HTTP, registry publication and visual baselines are deferred.
Existing implementations remain covered but are not local-release gates.

## Repository foundation and progress tracking

Create a tracked agent guide, unchanged attachment archive, this execution plan,
a canonical mdBook specification, and the single progress ledger. The book
covers architecture, identity/codecs, object storage, publication/recovery, JJ
integration, CLI, APIs/mutations, security, adapters/search, compatibility, and
verification.

Use `JST-<AREA>-NNN` requirement IDs, exactly one verification row per
requirement, and mapped spec-first checks through the released
`@lapismd/spec-validator`. Preserve `pnpm spec:first`, `spec:validate`,
`spec:build`, `spec:check`, `check`, `test`, and `build` entry points.

The ledger contains milestone gates and smaller tasks such as M2.1, recording:

- Planned, In progress, Blocked, Partial, Done, or Deferred status.
- Dependencies, acceptance criteria, requirement IDs, remaining gaps, next
  action.
- Named tests, exact commands, actual results, failure counts, evidence links.
- Upstream revisions, storage/API format changes, changed files.

Require evidence in the same Jujutsu change as every implementation slice, even
when milestone status does not change. Update status when gates advance or
regress. Reference the enclosing change rather than a self-referential hash.
Documentation-only edits update the ledger when they change scope, gates,
status, or cited evidence.

The progress checker runs in normal and specification checks. It validates IDs,
dependencies, evidence links/test names, completion requirements, and
same-change updates. A passing checker validates bookkeeping; tests establish
completion.

## Architecture and public contracts

Create model, object storage, WAL, JJ integration/codecs, service, RPC, HTTP,
adapters, Tantivy, CLI, server, and testkit crates as milestones begin. Native
codecs initially live with JJ integration. Storage/publication never depend on
UI, search, or future projections.

Pin jj-cli and jj-lib together at 0.45.1, upstream revision
`7c41cdeb16b6b321c64e789a966b6adf723816a5`. Use Rust 1.98.1, edition 2024, and
Apache-2.0. Commit dependency locks, reference revisions, and attribution. Use
pinned SimpleBackend identity and SimpleOpStore operation/view hashing as
references. Complete project codecs include copy histories and submodule entries
missing from SimpleBackend, native ordering, signatures, roots, and absent
versus empty predecessors. Envelope checksums are distinct from native IDs.

Immutable objects, dependency inventories, receipts, and attempt-unique WAL
candidates precede one conditional manifest update. Only selected records are
committed. Native head updates preserve siblings; strict mutations reject stale
snapshots. Durable receipts resolve retries/lost acknowledgements after restart
and checkpointing.

Register remote Backend, OpStore, and OpHeadsStore through CliRunner. Server
stores run in process and share the publication coordinator. Physical checkouts
remain client-local; native views contain logical workspace mappings. Use
protobuf/tonic RPC and Axum versioned JSON/SSE with generated OpenAPI/client
types. Both transports enforce the same principal, limits, and semantic policy.

### CLI

Build custom `jj`, extending `jj store`: init, attach, status, capabilities,
doctor, serve-local, credential login/logout. Expose server `jj-store` with
explicit run, configuration, diagnostics, and token administration.

Follow jj-nostr custom-command Markdown help, short/long help, keyword help,
Hint errors, JSON output, TTY paging, color controls, completions, and build
information. Preserve normal upstream JJ behavior and publish tested
capabilities. Use explicit development build paths and never replace installed
jj.

### APIs and browser

Implement the attachment's /v1 read routes, snapshot-bound pagination, committed
SSE, mutation preview/apply/status, and bounded conflict-content uploads.
Initial commands: New, Describe, bookmark create/move/delete, Abandon,
ResolveConflicts, ReconcileOperations. Server rebase/squash/undo follow later;
ordinary CLI support has separate parity coverage.

Use Svelte 5, TypeScript, and Vite, served by the server. Include repository
selection, snapshot state, commit/operation/evolution tabs, file/diff
inspection, conflict editing, mutation previews/results, search, and index
freshness.

### Authentication

Use revocable repository read/write/admin tokens and a separate operator
capability for repository creation. Store token verifiers server-side and
credentials outside checkouts. Browser tokens stay in memory. CLI uses the OS
credential facility with environment injection for automation. Authorize before
objects, search, previews, and publication. Shared deployments require TLS;
unauthenticated loopback requires an explicit development setting.

## Implementation sequence and gates

| Milestone     | Implementation                                                                                                                                                    | Completion gate                                                                                                                      |
| ------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------ |
| Bootstrap     | Colocated repository; governance, specification, ledger, Rust/tooling skeleton, boundaries, validation                                                            | Book/governance checks pass and foundation committed independently                                                                   |
| M0            | Pin references and RustFS image/topology; compile upstream custom-backend and RPC integration; audit traits/codecs; prove context propagation and bounded queries | Compatibility matrix, identity golden fixtures, real RustFS conditional-write evidence; failed assumptions block dependent work      |
| M1            | Typed IDs, envelopes, complete codecs, streamed S3, integrity checks, fault storage                                                                               | Every variant round-trips with correct hash; explicit corruption failures; real create/read/range/CAS passes                         |
| M2            | Dependency inventories, unique WAL candidates, CAS, receipts, unknown outcomes, committed traversal, checkpoints/catalogs, cache-free recovery                    | Attachment fault matrix including late duplicates, competing candidates, missing dependencies, checkpoint races                      |
| M3            | Native/in-process/RPC stores, create/attach, tokens, guarded publish, JJ indexes, physical workspaces                                                             | Create/edit/describe/log/restart/op-log/evolog without Git/SQL/relays; concurrency preserves history and local edits                 |
| M4            | Pinned reads, graph/revset/tree/file/diff APIs, operation diff, SSE, generated contracts, browser                                                                 | Fixed-operation CLI/API parity; zero GET writes; coherent pagination; explicit multi-head state                                      |
| M5            | Preview/apply/status; basic commands, abandon, resolve, reconciliation, shared semantic policy                                                                    | Correct descendants/bookmarks/workspaces/evolution; stale/invalid requests publish nothing; duplicate receipts; no RPC policy bypass |
| M6            | Committed adapter runner, bootstrap/checkpoints, Tantivy deterministic updates, durable payloads, scopes, freshness, rebuild                                      | Replay equivalence, safe index/cursor crashes, lost-index rebuild, fresh access control                                              |
| M8a           | LocalTrusted examples, embedded-storage lifecycle, diagnostics, limits and measured macOS acceptance                                                              | Pinned cold recovery/security/semantic/browser/storage checks pass; limits published                                                 |
| M7, deferred  | Complete replica inventory, verified transfer, publish-last destination, lag, fenced manual promotion                                                             | Read-only complete history through applied position after primary shutdown                                                           |
| M8b, deferred | Profile audit, lossless import rehearsal, verification/rollback                                                                                                   | Exact guarantees demonstrated; incompatible profiles rejected; failed import leaves source untouched                                 |

Retain the attachment identifiers; split M8 so first-release hardening precedes
migration. Live Nostr/Git adapters, agent capture/materialization integration,
active-active storage, destructive GC, and automatic failover are out of v1.

## Validation and completion

Start at the narrowest boundary and record before failures; rerun that exact
target after edits, then the complete impact surface and required wider lanes.

Native fixtures cover merges, divergent changes, split/squash/rebase, abandon,
conflicted bookmarks, multiple workspaces, copies, symlinks, executable files,
binary content, signatures, and invisible predecessor commits.

Publication combines randomized replay tests with real RustFS races, dropped
responses, process kills, restart, missing dependencies, corruption, checkpoint
recovery. Fake evidence does not replace deployment conformance.

Mutation/access coverage includes stale previews/fingerprints, atomic multi-path
resolution, conflicting idempotency reuse, cross-repo access, revocation, and
crafted low-level publications. UI checks cover interactions/accessibility, safe
content, resumable SSE, cold builds/server lifecycle, and visual-pending
stories; visual baselines need approval.

Operational tests delete derived caches and recover acknowledged history,
rebuild search, distinguish storage readiness from degraded adapters, and record
requests, bytes, CAS retries, and indexing lag.

Commit every verified slice with canonical requirements and ledger evidence.
Record unexplained failures; compilation or a happy-path demo never establishes
completion. The local first release requires CLI/browser history parity,
LocalTrusted native publication, persistent restart and acknowledged durability
under the documented failures. Search and shared authenticated deployment remain
future releases.

### Catalog follow-up requested 27 September 2026

M4.7 adds a typed in-memory controller and complete application stories
alongside the simple shells. Reuse Community repository file tree, branch/change
picker, file-content viewer and left-sidebar profile/login footer. Match
Community fonts and icon geometry through the same shared stylesheet build
setup. All actions remain local; production integration and authentication
remain deferred.

M4.8 applies catalog review feedback: full-width bookmark cards with dividers
except the final visible card, sticky secondary headers, aligned graph avatars,
Community breadcrumbs/sidebar icons/transparent details rail, and copied
directory navigation when no file is selected. Branches, JJ Log and Op Log use
column titles instead of redundant body titles; mobile navigation remains
available. Production UI scope stays deferred.

## M20 event-driven freshness

Implementation was qualified in the independent JJ workspace `event-freshness`
at `../jj-store-event-freshness`, including committed bounded-read follow-ups.
The verified stack is integrated into the main workspace; the temporary
workspace is retired, with verification logs retained under
`.local/event-freshness` in main. Buzz desktop at
`ef2aa1ae38fadcc0bc22b8bf6ed96b35933146be` informed shared cache ownership,
narrow invalidation, subscribe/read race closure and generation guards; its
local sync polling is not the healthy browser refresh model.

Deliver shared session read leases, an advertised authenticated multiplexed
feed, refcounted committed-state monitors, and scoped browser consumers.
Preserve historical pins, drafts, exact task/request identities, and validated
conversion receipts. Qualify cold built native/real-RustFS and filesystem views
for idle silence, unchanged five-minute renewal, external changes, replay/reset,
outage, wake and authorization. The canonical M20 ledger records actual gates
and gaps.

M20 is delivered through JJ changes `prpswkts`, `uvknxprm` and `zrmqwtuq`.
E-FRESH-001 through E-FRESH-005 record focused contracts, shared query leases
and cold built native/real-RustFS plus filesystem evidence. The idle capture
makes zero data requests through real five-minute renewal; external changes,
replay/reset, hidden wake, exact preparation and retained drafts/version guards
are qualified. Broader capacity, release and visual qualification remain with
their owning milestones.

The requested implementation and scoped cold gates are complete. The M20
aggregate remains In progress because its existing M15.5 dependency still
requires Threads full-flow qualification; this work does not promote that
separate milestone.

E-FRESH-006 records local main integration and temporary workspace retirement.
Integration preserves the qualified production tree and original main ancestry;
it does not restart development services or publish repository history.
