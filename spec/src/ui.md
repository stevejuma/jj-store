# Storybook presentation catalog

The reviewed catalog application is the presentation authority for the served
browser. Production browsing, search, typed mutations and conflict editing share
the same components and asynchronous controller contract. Reusable components
receive presentation data and callbacks, while HTTP and in-memory controllers
supply interchangeable sources. Deterministic fixtures remain catalog-only.

## Requirements

| ID         | Requirement                                                                                                                                                                                                                                                                            |
| ---------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| JST-UI-001 | The catalog MUST build and run independently using registry dependencies, deterministic fixtures, isolated story state, runtime diagnostics and blocking interaction/accessibility checks.                                                                                             |
| JST-UI-002 | The shell MUST directly adapt Community shell/navigation composition and preserve desktop/mobile, sidebar, header, focus and navigation mechanics using public Design Core exports.                                                                                                    |
| JST-UI-003 | Branches, JJ Log and Op Log MUST reuse Community graph/row/file/diff compositions, preserve native identities and conflicting targets, and support fixture-only drill-down without transport or mutation capabilities.                                                                 |
| JST-UI-004 | The complete catalog application MUST expose a typed state/actions controller with independent in-memory navigation, repository snapshots, file contents and profile actions, using copied Community file-browser and footer compositions without backend access.                      |
| JST-UI-005 | The served browser MUST use the reviewed shell and repository components through an authenticated asynchronous controller with cursor paging, representative page and detail loading skeletons, explicit snapshot refresh and typed hash restoration.                                  |
| JST-UI-006 | Large repository collections MUST use bounded virtual rows, all resizable shell/canvas regions MUST persist versioned layout state, and catalog stories MUST exercise the production components.                                                                                       |
| JST-UI-007 | Repository search MUST use the released Design Core Command View in a Community-adapted overlay, query the selected authenticated operation snapshot, represent readiness and paging states, and navigate results with stable full identities.                                         |
| JST-UI-008 | Authenticated repository commands MUST use Community-adapted action menus and Design Core form/confirmation dialogs, preview the exact typed request before apply, preserve its request ID through unknown outcomes, and advance the displayed snapshot only from a committed receipt. |
| JST-UI-009 | Conflict editing MUST present the native path, terms, fingerprints and supported resolution actions without flattening conflicts, stage bounded uploads separately, preview the atomic selection, and retain unsupported or stale states explicitly.                                   |
| JST-UI-010 | The application MUST bootstrap before repository UI, auto-open the sole LocalTrusted repository or show its installation catalog chooser, and label unverified loopback identity as Local; operational backup/restore status UI remains deferred.                                      |
| JST-UI-011 | Authenticated connection MUST request one manual token, keep it only in memory, discover all currently authorized repositories, restore an ID-based deep link or show a chooser, and cancel requests, SSE and protected caches when the token is replaced, revoked or disconnected.    |

The production controller completes bootstrap before rendering connection
choices. One ready LocalTrusted catalog entry opens without a token, several
entries render an explicit chooser, and an empty catalog explains repository
creation or verified registration. Authenticated mode asks for one token, then
opens the requested ID-based deep link, auto-opens a sole authorized repository,
or renders every authorized alias. Repository switching reuses that in-memory
credential while closing the previous query/SSE session. Disconnect, token
replacement, revocation and teardown cancel protected requests, event streams
and caches. Local loopback identity is labelled only as `Local`; the UI does not
infer a username from HTTP. Usable backup status is future-release work.

The catalog uses Storybook 10.5.7, Svelte CSF 5.1.2, Vitest browser 4.1.11 and
Design Core ^0.2.0 with the existing Svelte 5/Vite 8 toolchain. Direct
adaptations retain provenance and Apache-2.0 attribution. Shared primitives
remain public Design Core imports. Stories are visual-pending until human
review.

## Shell composition

The shell defaults to Branches, a collapsed left sidebar and closed 320px right
sidebar. Per-instance history, selection and controllers never persist across
stories. Repository compact popovers retain focus restoration; mobile navigation
uses the same public shell stages. Community-only room actions are omitted.

## Repository presentation

Branches are JJ bookmarks; conflicted bookmarks show every target and deleted
bookmarks show empty history. A branch, revision and file occupy successive
Column Canvas path levels. Changing an earlier selection clears later selection.
JJ Log preserves divergent same-change commits; only duplicate native IDs may
collapse. Operation siblings remain visible and opening details publishes
nothing. Full-ID clipboard controls never construct Community routes. Source
strings render as data; file/diff controls retain public Design Core semantics.

## Shared application

Community's repository file tree, branch picker, source viewer and profile
footer are production-owned under `web/src/lib` and imported by Storybook.
Choosing a repository opens its Files view. Bookmarks and native changes resolve
to explicit snapshots; divergent changes and conflicted bookmarks require a
target choice. Back/Forward restores repository, page, snapshot and file
selection. Catalog account actions only change fixture identity. Production
connection state replaces the fixture account when the HTTP controller is used.

Community's Tailwind Vite plugin compiles the public Design Core stylesheet. The
catalog uses its bundled DM Sans and Source Code Pro fonts, shared icon sizing
and Community typography scale. No substitute font or icon override is
introduced.

Branches use the requested card layout within the existing Command View:
bookmark name, latest description, workspace/conflict chips, author avatar and
deterministic relative time. Filters are All, Workspace and Conflicted,
reflecting native JJ state. The full-app story creates a fresh controller and
has no autoplay actions. The measured graph retains Community paths while
allowing native bookmark labels and ID controls to wrap in narrow columns
without overlapping subsequent rows.

The Branches column places every native working-copy association in a card above
the bookmark filters. Selecting a working copy scopes the adjacent JJ Log to
that exact commit and opens its details. Selecting a bookmark scopes the log to
the exact ancestry of all of its native targets; a conflicted bookmark preserves
every target and a deleted bookmark produces an empty graph. Route restoration
loads references before resolving this scope, so reload and Back/Forward cannot
briefly substitute the repository-wide default log.

Working-copy cards use the same title size as bookmark cards. A native empty
working-copy description is presented as `(empty) (no description set)` while
the workspace chip supplies its identity. Bookmark cards and their final visible
item remain square with a bottom divider. Empty Workspace and Conflicted filter
results use the shared Design Core shadcn Empty composition with icon media and
filter-specific guidance; changing a filter clears any command selection that is
no longer rendered.

JJ Log rows render local bookmarks as accent chips, remote bookmarks as neutral
`name@remote` chips, working-copy associations as success chips, divergence as a
warning and conflicts as danger. Op Log rows use the same working-copy chip,
distinguish captured snapshots with a muted chip and render concurrent head
aliases with the accent tone. These decorations come from native graph metadata;
the presentation layer does not infer them from descriptions or fixture names.

Graph lane slots remain stable while inner forks converge so an unrelated outer
lane never jumps horizontally between adjacent revisions. Vacated internal slots
may be reused by later branches only after the current row is captured; trailing
vacancies are trimmed. The graph SVG paints above square row dividers so a
divider cannot cut across a colored lane or node.

The catalog shell omits a redundant sidebar brand header and uses Community's
muted breadcrumb and stateful sidebar glyphs. Branch cards span their list
width; bookmark headings remain sticky during list scrolling. Column Canvas
headers use the theme-derived neutral grey background and bottom border;
operation-head status rows use a neighboring neutral grey so the information bar
reads separately from the header. Shell bodies retain padded content insets, and
collapsed columns span the usable padded body area. Graph content reserves the
full graph lane width so author avatars align across forks and merges. JJ Log
and Op Log rows are square-edged and retain bottom dividers through the final
visible row. Column warnings and failures use the shared Design Core shadcn
Alert composition with a semantic icon and theme-derived tinted background; they
are square, full width and separated from content only by a bottom border.
Revision rows with both a commit ID and JJ change ID use a shared split copy
control with neutral GIT and accent JJ chips and no copy icons. Each semantic
button copies its complete underlying native identifier and exposes it in a
shadcn tooltip. At rest the display contains only a configurable leading prefix
(default four), with no dots or suffix. Hover or keyboard focus redistributes
space inside a constant outer width, revealing a longer active prefix (default
five), two dots and the final two characters; short IDs remain whole. The
inactive side keeps only its identifying chip. Both sides remain tappable
without hover; reduced motion disables the transition. Success or failure
feedback belongs only to the clicked side and resets after 1.5 seconds. The
expanded half retains its chip beside a tick and 8px `Copied` text (or a warning
and `Retry`), with a live announcement. Feedback keeps the tapped half expanded
on touch devices. When another half expands, the previous half keeps its chip
and independent feedback tint. CSS animates the grid redistribution; pointer,
keyboard focus and copy feedback select the active side. Either identifier may
be absent; no empty button is rendered. A single identifier shows the expanded
prefix, two dots and final two characters immediately, without requiring hover
or focus. The default control uses 11px resting identifiers, 10px expanded
identifiers, 9px chips and a roughly 121px total width at the standard 16px root
scale. Each half retains 5px inline padding, including at the trailing edge
during hover and feedback. Graph rows wrap the ID control below metadata before
bookmark chips can overlap it. Clipboard live announcements remain scoped to the
split control; operation IDs retain their separately labelled legacy control. JJ
Log and Op Log rows open their changed files inline, and selecting a changed
file opens the diff column. Op Log operation details and the selected file diff
render as equal-width stacked cards in the same Column Canvas column: the
operation card owns its title, collapse control, metadata and close action,
while the file-diff card keeps its file header and view controls underneath.
Parent operation navigation remains available without opening a standalone
details column. Missing selected-file diffs and binary or unavailable diffs use
the shared shadcn Empty composition with icon media rather than plain status
text. An open standalone diff column with no selected file uses the same icon
Empty composition. Files opens the copied Community directory table until a file
is selected; directory, file and snapshot navigation participate in the
in-memory Back/Forward history.

Branch cards retain themed bottom dividers, including the final visible card in
filtered lists, and keep square lower corners. Branches, JJ Log and Op Log omit
the redundant body title bar; their Column Canvas headers identify the views.
Their shell body is a fixed, non-scrolling region; only each Column Canvas body
scrolls vertically. Mobile retains navigation/history/sidebar controls without
duplicating the page title. The right details rail inherits the shell background
with no visible border.

Files places its directory breadcrumb in the body page navigation, rooted at
`Files`. The repository file tree is a shell body-left sidebar with a
full-height divider, small inner padding, keyboard-resizable width and the
shared file listing view-mode toggle. The copied directory table is a bordered,
radiused card with a commit/action summary as the first row; the file-tree
toggle remains beside the author avatar and the snapshot selector is aligned on
the right side of the same row. Its resting label tracks the selected bookmark
or change after that reference resolves to a commit; the resolved commit must
not replace the selected reference identity. The table column-header row uses a
neighboring theme grey distinct from the summary row. Root directory views show
a GitHub-style README/LICENSE preview card only when those files exist in the
selected fixture snapshot. README is ordered first regardless of API order; its
tabs are left aligned with document icons, use a rounded theme-grey hover and
selected treatment, and keep a visible active underline. Selecting a file
renders a stacked source preview: copied commit metadata, deterministic file
statistics, then the selected content. Markdown files open in safe preview mode
with a Preview/Source toggle; source mode uses the public Design Core CodeBlock
with the filename/source body on the theme background. Rust source previews use
CodeBlock's public custom-tokenizer extension point so `.rs` files highlight in
the catalog while retaining the shared code component.

M4.10 completes the bounded authenticated data contracts required by the shared
repository presentation: reference lookup, recursive path windows, exact
operation-change groups and changed-path summaries. It does not connect the
served browser or introduce fixture data into production; the HTTP controller,
query cache, loading/error presentation and real-screen wiring remain M4.11.

M4.11 uses a session-scoped TanStack Query client with Community's 30-second
stale time, ten-minute inactive collection and disabled focus refetch. Query
keys bind the in-memory authentication session, repository, selected operation
and resource inputs. Disconnect, replacement and revocation close transport,
cancel pending work and clear protected cache entries. The production bundle
uses the catalog's released Design Core theme plus bundled DM Sans, Source Code
Pro and Community composer font assets; the static manifest and router allow no
more than 256 hashed JS, CSS, WOFF2, WOFF and TTF files with explicit media
types under the existing self-only CSP.

M4.12 owns the typed hash route
`#/repos/<repository-id>/<page>?at_op=<operation-id>&…`. Permanent routes use
full native repository, operation, commit and change identities plus safely
encoded bookmark and repository paths; credentials, continuation cursors and
response data never enter the URL. A repository selection is valid only with an
explicit captured operation. Commit selections are excluded from Op Log, while
inspected operations and comparison parent/group selections are confined to Op
Log. Unknown, duplicate, malformed, unsafe and page-incompatible fields fail
closed with an explicit route error.

Application navigation pushes browser history, while native Back/Forward events
restore the pinned repository, page, snapshot, bookmark/change/commit, operation
comparison, directory and selected file. Reload prefills only the repository
identity; the memory-only token must be entered again before the controller
restores protected content. SSE announces fresher history without changing the
displayed snapshot. Explicit Refresh captures current heads and retains the deep
selection and hash when that selection remains available.

Every asynchronous browsing surface preserves its final geometry while loading.
Branches renders bookmark-card placeholders in its initial path column; JJ Log
and Op Log render their graph/status composition; Files renders its file-tree
and directory table composition. Commit, operation and file-detail reads retain
the selected context and show representative changed-file, diff or source
placeholders. Changed-file loading preserves the real card title and view-mode
controls while skeletonizing only the count and file rows. The states use Design
Core's public shadcn Skeleton, expose one labelled busy status per region and
keep decorative placeholders hidden from assistive technology.

The disposable native demo includes a genuine unresolved multi-parent file
conflict and two concurrent commits that retain one JJ change identity. These
fixtures prove conflict and divergence presentation without synthesizing graph
relationships in the browser.

M4.13 layout state uses versioned `jj-store/layout/v1` keys. Production uses
browser localStorage only for shell sidebars, per-page Column Canvas geometry,
collapse/close state and file-tree width; repository identities, selections,
credentials, cursors and content never enter that storage. Invalid, unavailable
or quota-limited storage falls back to defaults without blocking browsing.
Storybook injects a fresh memory store and a dedicated remount interaction
proves restoration without sharing state between stories.

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
visible path remains bound to its precise native comparison.

M9.5 adds Agent activity and Reviews to the same application shell and typed
hash history. Agent and review pages encode only a full stable session or thread
identity beside the selected operation. Their two-column canvases persist
versioned width and collapse state through the shared injected storage boundary
and keep collection scrolling inside the columns. Session, transcript, thread
and comment rows use measured virtualization. Ready, loading, empty and failed
states use the same production components in Storybook.

Agent transcript chunks and review comments render through Community's public
message component. Open review threads compose Community's public chat composer
with exact optimistic-concurrency actions; resolved threads remove the composer
and expose Reopen. The detail headers distinguish the selected native code
snapshot from the independent AgentStore or ReviewStore root. An unknown review
outcome remains an explicit alert with a Check status action. SSE freshness does
not replace displayed collaboration records; only explicit Refresh captures new
heads, invalidates the session cache and restores the deep route.

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

M4.17 owns native conflict editing. The editor loads complete descriptors for
the selected commit, identifies every term and fingerprint, disables unsupported
editor shapes with their native reason, and offers only server-advertised term,
delete or bounded uploaded-file actions. The complete multi-path selection is
previewed and applied atomically through the same durable request workflow;
stale fingerprints never trigger an implicit reread or partial publication.

M9.5 collaboration cards use the same bordered row structure and metadata/action
placement as Branches and Op Log. Agent cards show the integration avatar, name,
state, captured start and compact captured duration when exact timing exists.
Full identifiers use labelled copy controls. Agent detail reuses Community
message/tool compositions, exposes the related native change in its header and
renders a changed-file diff inside a tool only when an exact code association or
recorded path ties that tool to the file.

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
thread directly after the call. Both inline surfaces remain keyboard reachable
and preserve the route-selected discussion column. The repository detail rail
follows Community's section typography and spacing; selecting a JJ Log or Op Log
row replaces generic repository facts with its metadata and the same enabled
mutation actions exposed by the header menu, including bookmark creation and
conflict resolution when their native capabilities and exact selection permit
them.
