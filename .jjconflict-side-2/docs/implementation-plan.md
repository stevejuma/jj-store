# jj-store implementation plan

Accepted 26 September 2026. The canonical requirements in `spec/src` govern
behavior. [milestones.md](../spec/src/milestones.md) is the single
implementation progress ledger. The [original attachment](reference/README.md)
is archived unchanged as non-normative reference material.

## Local stacks backend proposal

Reviewed 5 October 2026 against the original UI-first request, JST-UI-020 and
the app-shell Storybook on port 6057. This is a proposal for M20.2, not approval
to implement it. The existing UI is the intended product surface. Keep M20.2
Deferred until the user explicitly approves backend work. Visual baseline
approval and foundational package publication are separate gates.

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
and named combinations. Full M20.2 acceptance additionally requires the
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
