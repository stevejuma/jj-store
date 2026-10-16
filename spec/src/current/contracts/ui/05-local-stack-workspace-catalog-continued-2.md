# Local stack workspace catalog (continued 2)

The Workspace canvas's existing trailing spacer MUST contain a vertically
centered public shadcn Empty composition with a branch icon, a Drop files to
start a branch prompt, and a Create a new branch action. Its plain surface has
no dotted background or leading border, and its 390-pixel width matches a
default stack column. This is trailing content rather than a registered stack
column; the Branches page retains its blank 240-pixel spacer. The action opens
the existing independent/dependent creation form. Exact file/folder/hunk drops
preview creation, then stage the retained selections in an inline draft only
after confirmed branch creation; unknown outcomes retain request identity and do
not stage early, and source files remain until the commit is confirmed.

The idle commit panel has two separated sections: a shaded drop target with a
visible two-pixel dashed border above the Start a commit action. Starting or
dropping changes switches to a stack-like card with a Staged header, count,
folder/list controls and file rows above a separate composer section. Staged
listing controls align to the far right of their header. Creating a commit opens
an inline stack form with a normal-height title input of at least 36 pixels, in
its own bottom-bordered row with a separator spanning the full card width
without horizontal insets, and the owned application composer for its
description. Dropping files or hunks opens that form and lists staged files;
multiple drops retain their exact selections. Staging and cancelling do not
mutate the mock snapshot. Drafts are scoped to the named combination and stack,
survive preview cancellation and unknown outcomes, and clear only after
confirmed commit success.

The Workspace gap between the commit panel and its first branch card MUST match
the twelve-pixel gap between branch cards, with the form both closed and open.
Both gaps share the same application spacing token; Branches keeps its existing
Apply/Unapply card spacing.

Branch-header file lists reuse the commit-row file listing with an inset around
the panel. Resizing action-bearing commit rows MUST retain their natural content
height in both directions across width breakpoints; horizontal copy width
reserves MUST NOT become vertical flex bases. Selected commit rows and branch
headers retain their selection labels and show the Changeyard-style rounded left
border marker without shifting the graph, avatars or row contents.

Named combinations share this folder while retaining independent unassigned
edits. Preview text identifies destinations and explains deletion of a stack or
combination; deletion and undo are exercised only in memory in this slice.

Reuse existing owned Community rows, IDs, authors, file lists, diff controls,
notifications and Settings forms, and public Design Core column, menu, dialog
and merge primitives. Adapt Changeyard stack grouping and drag/drop behavior
with source provenance; do not import either reference checkout or duplicate
equivalent primitives. Production stays independent of catalog fixtures.

This first UI slice introduces frontend contracts only. All catalog mutations
change isolated in-memory fixtures; no checkout, operation heads, companion ref,
manifest, API, RPC or Rust runtime is changed. Named combinations preserve
unassigned work independently; removing every applied stack remains usable.
Previews bind a snapshot and immutable request, with a distinct apply step.
Stale results retain drafts; unknown outcomes retain the original request and
block new commands until status or exact retry resolves it. Keyboard/menu
actions cover drag/drop workflows. Functional and accessibility acceptance is
separate from visual approval. The user approved backend integration on 5
October 2026 after this UI review. Production wiring MUST retain the same owned
controller and components, require a verified physical device binding, and
advertise only qualified provider read and mutation capabilities. Catalog
adapters remain isolated.

Production stack read projection MUST retain an explicit repository, profile,
binding generation and code-operation pin. History loads one bounded page
without loading files or patches. Provider parents, graph edges, bookmark
annotations, workspaces, conflicts and divergence remain available; unloaded
files cannot imply an empty commit or manufacture linear parents. Net file lists
compare the explicit boundary and head, including an empty list for reverted
changes, rather than unioning per-commit hunks. Selected patches load
independently. Unloaded, text-ready and unavailable content remain distinct;
ordered file terms, modes and rename/copy sources remain intact. Matching-only
diff blocks are not shown as changed hunks, and byte offsets are never treated
as line numbers. Presentation hunk identities confer no mutation or transfer
authority. Caller cancellation and read-owner disposal reject late results
without closing the repository's shared transport. This read foundation does not
fabricate a named combination or enable a command executor; production ownership
MUST explicitly inject this adapter into the served App and dispose only its
owned reads on repository, binding or authorization changes.

The shared stack controller MUST support optional lazy file-summary and patch
reads without substituting mock unions for missing provider data. Loading, retry
and continuation state belong to each source or diff, and cannot lock unrelated
browsing. Branch and commit selections load their inline file panel; only
opening a listed file loads its patch. Independent adjacent diffs retain their
sources while another selection loads. Scope changes, closing a diff and
controller disposal cancel only the owned reads and reject late completion. File
continuations remain bounded and advancing. Copy and browsing stay available
when mutation capabilities are absent; each mutation control uses its own
advertised capability and unavailable reason. A disabled executor is never
called to discover whether a command is eligible.

Unconfigured local checkouts MUST represent an absent workspace target and named
combination explicitly. Opening Stacks does not invent a base bookmark, applied
membership or unfinished files. Branch browsing offers an existing command-view
target picker through the Settings form composition. Choosing that target is an
operation-scoped read selection; it initializes no combination, publishes no JJ
operation and writes no checkout or private configuration. Missing or ambiguous
targets require an explicit choice. Workspace shows an existing empty state
until a qualified named combination exists, and command eligibility requires the
necessary target/combination state in addition to advertised capability.

The production stack adapter MUST verify the existing LocalTrusted registry
binding against the code snapshot generation before exposing its folder. It
loads bounded bookmark metadata without files or patches, retains conflicted
targets as unavailable choices, and requires an exact explicit target choice.
Stack and target history pages bind this read scope and retain provider graph
boundaries. Source file summaries and selected patches compare exact trees;
merge commits with no chosen comparison parent stay explicitly unavailable.
Adapter disposal cancels its reads without closing the shared repository
transport. Read projection advertises no mutation capability until the durable
executor is separately qualified.

The Stacks controller owns current physical observation independently from its
historical branch read scope. Observation selects one explicit current operation
from a fresh verified binding snapshot and rechecks it after the read; absent or
multiple heads remain unavailable without reconciliation. A historical selection
is never silently replaced by the current checkout. Refresh supersedes previous
observations, disposal cancels only Stacks-owned requests, and late results
cannot replace newer state. Failed reads clear current observation evidence
while retaining branch/diff selections. Physical changed-path metadata is not
complete Unstaged: durable combination merge/WIP state must qualify before
assignment.

The production lifecycle owner MUST require LocalTrusted access, the supported
local profile, advertised code-read capability and matching registered binding
generation. Stacks routes MUST restore their own bounded reads instead of
requesting the ordinary Log graph. Index preparation and read failures reuse
application notices without duplicate Stacks feedback. Branch/target pickers
retain bounded continuation and unavailable choices. Refresh MUST revalidate
loaded ranges against exact heads and boundaries, replace obsolete cursors and
retain valid historical source/diff selections. A failed refresh MUST NOT
install a partially refreshed read owner. Teardown rejects late discovery and
reads; identity or folder mismatch removes the controller.

Private physical scan/WC baseline comparisons do not establish active Unstaged
assignment. The UI must keep this capability unavailable until composition
qualifies active-combination assignment and production read integration. Private
current checkout/sparse/ignore witnesses and service observation now supply
these provider inputs, and a registered bounded observation route exposes safe
read summaries; these do not themselves grant a UI capability. A historical
working-copy baseline cannot become the active physical folder merely because it
shares a workspace name.

The 5 October alignment review and proposed backend delivery order are recorded
in
[the implementation plan](../../../../../docs/implementation-plan.md#local-stacks-backend-proposal).
The review preserves this UI and its controller boundary while identifying real
net diffs, bounded reads, capability eligibility, durable recovery and lifecycle
wiring as integration work. The subsequent user request approves implementation
of this delivery order; M21.2 remains incomplete until real provider, checkout
and lifecycle acceptance is recorded.

Native branch net files resolve a concrete owner only from a complete loaded
linear segment (at most 40 commits and 50 file-summary pages), one changing
commit per path, and identical ordered native file terms, modes and provenance.
Only opening a resolved file requests that owner’s native hunk tokens;
ambiguous, immutable or bounded-out comparisons remain read-only. Unresolved net
diffs explain that an individual commit is required to move changes. Exact
resolved files and native hunks reuse staging, commit transfer,
return-to-Unstaged, keyboard assignment and explicit preview/apply.
