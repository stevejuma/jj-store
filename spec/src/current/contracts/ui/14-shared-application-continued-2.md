# Shared application (continued 2)

Review-detail loading uses the full-page composition: a fixed title and opening
date row, four metadata rows, Description, Files changed, Commits and Activity
placeholders in the content scroller, and a fixed composer placeholder for
writable access. Initial connection, list and selected-detail reads use the same
geometry, without showing the review list or interactive placeholder controls.

The disposable native demo includes a genuine unresolved multi-parent file
conflict and two concurrent commits that retain one JJ change identity. These
fixtures prove conflict and divergence presentation without synthesizing graph
relationships in the browser.

M4.13 layout state uses versioned `jj-store/layout/v1` keys. Production uses
browser localStorage only for shell sidebars, per-page Column Canvas geometry,
collapse/close state, file-tree width and page-section disclosure flags;
repository identities, selections, credentials, cursors and content never enter
that storage. Invalid, unavailable or quota-limited storage falls back to
defaults without blocking browsing. Storybook injects a fresh memory store and a
dedicated remount interaction proves restoration without sharing state between
stories.

Bookmark cards, reference choices, revision and operation graphs, changed-file
lists, directory tables and the repository file tree use measured virtual rows
against their existing accessible scroll viewport. Large fixtures MUST keep the
mounted row count bounded while preserving keyboard selection, graph geometry
and cursor-edge callbacks. Headers, filters, metadata and details remain outside
the virtual row stream. Tree mode requests a nonrecursive root window and loads
each directory's cursor-paged children only when the host discloses or navigates
to it. List and package modes use a separate recursive root window. Storybook's
in-memory controller exercises the same controlled view and disclosure actions
without transport.

Selected commit and operation changed-file collections use the same virtual
end-of-list continuation contract. Commit comparisons retain their opaque
changed-path cursor and captured snapshot. Operation comparisons retain the
affected-change cursor plus each exact comparison's changed-path cursor, fetch
one exact group at a time, and keep ambiguous groups explicit. Empty exact pages
and metadata-only groups advance without presenting invented files; every
visible path remains bound to its precise native comparison. When an operation
has multiple recorded comparison parents, its workspace header MUST show a
compact searchable popover immediately before the workspace selector, without an
explanatory notice or description. It lists only those exact native operation
identities with their available descriptions and authors. Each option reuses the
Op Log card's Community author avatar, description, timestamp, native badges and
copy button for the full operation ID. Cards have small gaps between them. The
rich results use accessible grid rows with separate content and copy cells.
Pointer and keyboard copy actions provide clipboard feedback, keep the picker
open and never select a comparison parent. A parent absent from the loaded graph
remains selectable by its identity. Description, author and full-ID search MUST
retain every candidate; selection passes the full operation ID to the existing
comparison action and retains the pinned snapshot, workspace and parent hash
across reload. It never substitutes a commit/change ID or invents operation
metadata.

The audit temporarily quarantines only the comparison header's immediate
resize-geometry assertions as
[Q-BROWSER-001](../../governance/25-test-quarantines.md#q-browser-001-comparison-header-resize-geometry).
Its exact ordering/bounds remain unqualified. Full native identity, clipboard,
keyboard, accessibility, selection and reload acceptance stay enabled; the
workspace-header contract above remains required.

M9.5 adds Agent activity and Reviews to the same application shell and typed
hash history. Agent and review pages encode only a full stable session or thread
identity beside the selected operation. Their two-column canvases persist
versioned width and collapse state through the shared injected storage boundary
and keep collection scrolling inside the columns. Session, transcript, thread
and comment rows use measured virtualization. Ready, loading, empty and failed
states use the same production components in Storybook.

Agent transcript chunks and review comments render through the owned Community
message component. Captured message timestamps appear in each message's metadata
in the device locale, with seconds and a semantic time element retaining the
full ISO date/time and exact date tooltip. Missing evidence displays “Time
unknown”; sequence identifiers remain available through the copy menu. Activity
and recorded-chunk fallback presentation share this behavior without changing
Markdown, prompt disclosures, reactions, message order or virtualization.
Adjacent recorded tool calls form a single expandable Design Core ToolCalls
group, in native order. Messages, capture gaps and other recorded events
separate groups. Each call retains its own details, native copy menu, review
anchor, reply actions and draft. Tool groups retain the message's outer edge and
owned horizontal bleed. Their heading and individual call contents have the same
0.5rem logical start padding inside each trigger before its icon, as messages
have before their avatars. The hover surface includes this padding. Each known
captured tool duration appears immediately before its detail chevron, including
failed or outcome-unknown calls. Subsecond spans use milliseconds; unknown
durations remain absent and zero is displayed. Timing does not change the
recorded status. Closed calls retain compact, contiguous rows; expanded details
occupy only their own call. Inline discussion contexts size to their content and
do not create a scroll viewport around each call or clip its hover toolbar. The
transcript owns scrolling, and the full toolbar remains pointer reachable across
row boundaries. Failed calls show their error status without an extra resting
“Tool failed” label; expanded details retain the actual error output. Open
review threads compose the owned Community chat composer with exact
optimistic-concurrency actions; resolved threads remove the composer and expose
Reopen. The detail headers distinguish the selected native code snapshot from
the independent AgentStore or ReviewStore root. An unknown review outcome
remains an explicit alert with a Check status action. Live events automatically
reload collaboration records from committed roots, invalidate the session cache
and retain the deep route and editor drafts. Unknown requests retain their exact
identity and preconditions.

M4.15 adapts Community's global search trigger and command dialog to native
repository search. `Command+K`/`Control+K` and the sidebar open the same
overlay. The production controller debounces queries, caches snapshot-bound
requests, ignores late responses, appends opaque cursor pages with identity
deduplication, and represents loading, empty, unavailable, failed, stale,
degraded and truncated states. Selecting an operation, commit or file pushes an
Op Log, JJ Log or Files route containing full stable native identities. Query
text and expiring search cursors stay out of permanent hashes. Storybook uses an
in-memory source while the cold native lane exercises the real Tantivy worker.

M4.16 adapts Community's action-menu, `DialogFrame` and `ConfirmButton`
contracts for native repository writes. New/describe, bookmark
create/move/delete, operation reconciliation and abandon construct strict V1
commands from the current captured head. Preview and apply retain one request ID
and exact request bytes; outcome-unknown keeps retry/status controls available.
A committed receipt captures the accepted head before the application refreshes
protected queries and restores valid route selections. Destructive commands
require the compact awaited confirmation contract. Fixture stories exercise
prepared, committed, rejected and outcome-unknown states without production
transport.

M4.17 owns native conflict editing. M4.24 presents resolution in the AppShell
structural sheet covering main and the right rail, retaining shell-owned focus,
Escape dismissal and underlying-surface restoration. Background navigation,
header mutations and snapshot refresh stay unavailable while the sheet is open,
keeping the draft bound to its selection. A full-height left body sidebar reuses
the repository file tree and its list/folder/package controls, with a continuous
right border. The leading header toggle shows or hides this browser without
discarding per-path drafts. On narrow screens the browser replaces the editor
body while open and selecting a file returns to the editor. Delete path, Stage
merged file and Preview, plus prepared/recovery actions, use compact icon
buttons in the sheet header, which has a bottom border separating it from the
body. There is no footer or Cancel action; the close control owns dismissal. The
syntax-highlighted three-way editor fills the remaining body; extra native
metadata and term cards are omitted. Built-in accept-all controls select an
exact left/right term when the resolved bytes match it; custom center edits
invalidate any prior selection until explicitly staged. The complete
descriptors, ordered terms and fingerprints stay bound to the request.
Unsupported editor shapes retain their native reason and only server-advertised
term, delete or bounded uploaded-file actions are offered. Path navigation,
delete and stage actions remain unavailable during upload/preview/apply and
while a prepared result is displayed. The complete multi-path selection is
previewed and applied atomically through the same durable request workflow;
stale fingerprints never trigger an implicit reread or partial publication.

M9.5 collaboration cards use the same bordered row structure and metadata/action
placement as Branches and Op Log. Agent cards show the integration avatar, name,
state, captured start and compact captured duration when exact timing exists.
Session rows show only the relative start time, without a “Started” prefix, and
retain the full-date tooltip. Full identifiers use labelled copy controls. Agent
detail reuses Community message/tool compositions, exposes the related native
change in its header and renders a changed-file diff inside a tool only when an
exact code association or recorded path ties that tool to the file.

The complete Agent or Review card surface selects its detail except for nested
copy and action controls. Integration artwork appears in the leading identity
cell and in related Agent and Review headers; Community's identity shape keeps
agents rounded-square and people or devices circular. Comment and tool totals
use labelled Lucide glyphs with numeric values. JJ Log and Op Log rows expose a
compact Community avatar group and reply summary only for collaboration already
associated with that exact revision or operation. Author email supplies a
Gravatar identicon when no known member image is available.

Selecting a revision or operation may append Agent Context and Review discussion
columns from exact root-pinned associations; selecting a changed path narrows
review context to its exact anchor for that revision or operation. Close
controls clear their route selections. Review detail uses Community thread
messages and keeps the composer at the bottom while the thread is open. The
Files summary shows the selected commit author's avatar and name, message, Git
revision and relative time. Catalog fixtures prove combined Agent and Review
columns; the live sample may show only Review until its capture path publishes
an exact AgentCodeAssociation.

Code-text and code-artifact review discussions place their compact message
thread above the exact inline file diff. The JJ Log and Op Log diff column uses
the same inline thread and bottom composer when the selected file has a loaded
review. Each text row reveals Design Core's comment action on hover or focus;
the resulting composer and exact anchored thread occupy that row's annotation
slot. Agent tool calls reveal a Community-style reply toolbar on hover or focus,
show an icon count for an existing thread, and place its composer and reply
thread in a full-width row directly below the call, in both Focus and Split
agent responses. Existing comments remain attached to their tool when the
composer closes or another tool is selected. Agent comment trees use the same
2.25rem depth step, six-level visible depth cap, muted curved connectors and
continuation guides as Thread replies. The session or tool is the conversation
root: its top comments occupy depth one, and replies to those comments occupy
depth two with one visible indent. Tool and comment hover/focus boundaries are
independent: hovering a tool comment reveals only its message toolbar, without
the parent tool toolbar. Invisible prompt-context controls MUST NOT intercept
pointer input intended for the underlying disclosure. Visible and
keyboard-focused controls retain their actions. Prompt-context actions sit above
the first disclosure so becoming visible cannot cover its click target. Tool
replies never open a second structural sheet. Both inline surfaces remain
keyboard reachable and preserve the route-selected discussion column.
E-UI-CI-001 confirms JST-UI-005 is unchanged by awaiting the existing delayed
message-action visibility before activating Reply in interaction coverage; copy,
reply, own-author editing and native identity assertions remain intact. The
repository detail rail follows Community's section typography and spacing.
Selecting a bookmark shows its name as a bold section heading, the target commit
avatar, one-line message, author and relative update time, then one-row status
and target-count chips with every exact target Git SHA below; selecting a
revision replaces that selection detail with change metadata. JJ Log and Op Log
selection follows the same replacement rule and exposes the enabled mutation
actions from the header menu, including bookmark creation and conflict
resolution when native capabilities and an exact selection permit them.
Multiline mutation input always uses Community's form composer. Fixed Edit
description and Abandon targets use the shared commit row with author avatar,
current message, copyable Git/JJ identifiers, relative commit time and
bookmark/workspace chips. Create change, Create bookmark and Move bookmark use
the same row as a searchable Design Core Command View selector over loaded
revisions; search covers message, author, email, Git/JJ identities, bookmarks
and workspaces, and the selected full commit ID remains the typed mutation
value. Commit-selector avatars and row bodies align vertically with the trigger
chevron, including descriptionless commits. Non-interactive GIT/JJ labels use
the compact indicator-size oval chips in both the selected button and searchable
options, with equal badge widths and 10px labels one pixel larger than the
revision-ID copy buttons. A fixed identifier column stacks Git above JJ and
aligns the chips across options regardless of message length, reserving the JJ
row when its change ID is absent. Relative commit time sits directly beneath the
title at the same left edge, with bookmark/workspace chips alongside it on that
row. Repository navigation sections have no top margin.

Selecting a JJ change in JJ Log or Branches exposes an Evo Log action in the
right-rail Actions section independently of mutation permissions. It adds a
resizable, closeable Community graph column, preserving the main selected
change, workspace and operation snapshot. The column reads native predecessor
edges from the evolution endpoint; it never substitutes commit parents. Its full
commit anchor is retained in the hash and browser history. Selecting a history
row browses that commit's predecessor history without changing the mutation
target. Loading, empty, failed and incomplete-predecessor states stay within the
column; opaque pagination preserves the exact read snapshot, deduplicates
identities and rejects repeated cursors or more than 1,000 displayed nodes.
Closing clears only the evolution anchor; changing page, main revision, bookmark
or workspace closes the column. The compact Repositories section has the same
zero top margin as the expanded section.

## Responsive tag collections

Revision and operation graph metadata, commit selectors, stack bookmark headers,
forum thread tags, room facts, capture-source states/counts and named workflow
commands use released Design Core TagList. Lists preserve native labels,
supplied order and tones. They occupy one bounded row; overflow opens all tags
without selecting the containing revision, branch, thread or commit. Long labels
remain accessible through tooltips and the keyboard-accessible popover.
Editable/removable selections, entity-link hover cards and standalone status or
identity badges retain their existing controls. Revision and operation graph
popovers show only tag labels, without repeated bookmark or workspace
descriptions beneath them. Local and remote bookmarks remain in one flat list.
No native model, command, identity, selection or authorization policy moves into
Design Core.

Commit-picker options retain searchable text summaries; the highlighted commit's
TagList lives in a separate preview footer outside the listbox. This preserves
valid option semantics while making every overflowed label accessible.
