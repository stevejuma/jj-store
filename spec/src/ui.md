# Storybook presentation catalog

The reviewed catalog application is the presentation authority for the served
browser. Production integration is reopened for the browsing portion of M4.3;
typed mutation and conflict-editing controls remain deferred. Reusable
components receive presentation data and callbacks, while HTTP and in-memory
controllers supply the same asynchronous contract. Deterministic fixtures remain
catalog-only.

## Requirements

| ID         | Requirement                                                                                                                                                                                                                                                       |
| ---------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| JST-UI-001 | The catalog MUST build and run independently using registry dependencies, deterministic fixtures, isolated story state, runtime diagnostics and blocking interaction/accessibility checks.                                                                        |
| JST-UI-002 | The shell MUST directly adapt Community shell/navigation composition and preserve desktop/mobile, sidebar, header, focus and navigation mechanics using public Design Core exports.                                                                               |
| JST-UI-003 | Branches, JJ Log and Op Log MUST reuse Community graph/row/file/diff compositions, preserve native identities and conflicting targets, and support fixture-only drill-down without transport or mutation capabilities.                                            |
| JST-UI-004 | The complete catalog application MUST expose a typed state/actions controller with independent in-memory navigation, repository snapshots, file contents and profile actions, using copied Community file-browser and footer compositions without backend access. |
| JST-UI-005 | The served browser MUST use the reviewed shell and repository components through an authenticated asynchronous controller with cursor paging, representative page and detail loading skeletons, explicit snapshot refresh and typed hash restoration.             |
| JST-UI-006 | Large repository collections MUST use bounded virtual rows, all resizable shell/canvas regions MUST persist versioned layout state, and catalog stories MUST exercise the production components.                                                                  |
| JST-UI-007 | Repository search MUST use the released Design Core Command View in a Community-adapted overlay, query the selected authenticated operation snapshot, represent readiness and paging states, and navigate results with stable full identities.                    |

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

The catalog shell omits a redundant sidebar brand header and uses Community's
muted breadcrumb and stateful sidebar glyphs. Branch cards span their list
width; bookmark headings remain sticky during list scrolling. Column Canvas
headers use the theme-derived neutral grey background and bottom border;
operation-head status rows use a neighboring neutral grey so the information bar
reads separately from the header. Shell bodies retain padded content insets, and
collapsed columns span the usable padded body area. Graph content reserves the
full graph lane width so author avatars align across forks and merges. JJ Log
and Op Log rows are square-edged and retain bottom dividers through the final
visible row. Revision rows with both a commit ID and JJ change ID use a shared
split copy control with neutral GIT and accent JJ chips and no copy icons. Each
semantic button copies its complete underlying native identifier and exposes it
in a shadcn tooltip. At rest the display contains only a configurable leading
prefix (default four), with no dots or suffix. Hover or keyboard focus
redistributes space inside a constant outer width, revealing a longer active
prefix (default five), two dots and the final two characters; short IDs remain
whole. The inactive side keeps only its identifying chip. Both sides remain
tappable without hover; reduced motion disables the transition. Success or
failure feedback belongs only to the clicked side and resets after 1.5 seconds.
The expanded half retains its chip beside a tick and 8px `Copied` text (or a
warning and `Retry`), with a live announcement. Feedback keeps the tapped half
expanded on touch devices. When another half expands, the previous half keeps
its chip and independent feedback tint. CSS animates the grid redistribution;
pointer, keyboard focus and copy feedback select the active side. Either
identifier may be absent; no empty button is rendered. A single identifier shows
the expanded prefix, two dots and final two characters immediately, without
requiring hover or focus. The default control uses 11px resting identifiers,
10px expanded identifiers, 9px chips and a roughly 121px total width at the
standard 16px root scale. Each half retains 5px inline padding, including at the
trailing edge during hover and feedback. Graph rows wrap the ID control below
metadata before bookmark chips can overlap it. Clipboard live announcements
remain scoped to the split control; operation IDs retain their separately
labelled legacy control. JJ Log and Op Log rows open their changed files inline,
and selecting a changed file opens the diff column. Op Log operation details and
the selected file diff render as equal-width stacked cards in the same Column
Canvas column: the operation card owns its title, collapse control, metadata and
close action, while the file-diff card keeps its file header and view controls
underneath. Parent operation navigation remains available without opening a
standalone details column. Missing selected-file diffs and binary or unavailable
diffs use the shared shadcn Empty composition with icon media rather than plain
status text. Files opens the copied Community directory table until a file is
selected; directory, file and snapshot navigation participate in the in-memory
Back/Forward history.

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
the right side of the same row. The table column-header row uses a neighboring
theme grey distinct from the summary row. Root directory views show a
GitHub-style README/LICENSE preview card only when those files exist in the
selected fixture snapshot; its tabs are left aligned with document icons and an
active underline. Selecting a file renders a stacked source preview: copied
commit metadata, deterministic file statistics, then the selected content.
Markdown files open in safe preview mode with a Preview/Source toggle; source
mode uses the public Design Core CodeBlock with the filename/source body on the
theme background. Rust source previews use CodeBlock's public custom-tokenizer
extension point so `.rs` files highlight in the catalog while retaining the
shared code component.

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
uses the catalog's released Design Core theme plus bundled DM Sans and Source
Code Pro WOFF2 assets; the static manifest and router allow only hashed JS, CSS
and WOFF2 files with explicit media types under the existing self-only CSP.

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
placeholders. The states use Design Core's public shadcn Skeleton, expose one
labelled busy status per region and keep decorative placeholders hidden from
assistive technology.

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

M4.15 adapts Community's global search trigger and command dialog to native
repository search. `Command+K`/`Control+K` and the sidebar open the same
overlay. The production controller debounces queries, caches snapshot-bound
requests, ignores late responses, appends opaque cursor pages with identity
deduplication, and represents loading, empty, unavailable, failed, stale,
degraded and truncated states. Selecting an operation, commit or file pushes an
Op Log, JJ Log or Files route containing full stable native identities. Query
text and expiring search cursors stay out of permanent hashes. Storybook uses an
in-memory source while the cold native lane exercises the real Tantivy worker.
