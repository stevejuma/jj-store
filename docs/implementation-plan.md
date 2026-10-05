# jj-store implementation plan

Accepted 26 September 2026. The canonical requirements in `spec/src` govern
behavior. [milestones.md](../spec/src/milestones.md) is the single
implementation progress ledger. The [original attachment](reference/README.md)
is archived unchanged as non-normative reference material.

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
