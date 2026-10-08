# Shared application

Community's repository file tree, branch picker, source viewer and profile
footer are production-owned under `web/src/lib` and imported by Storybook.
Choosing a repository opens its Overview view. Bookmarks and native changes
resolve to explicit snapshots; divergent changes and conflicted bookmarks
require a target choice. Back/Forward restores repository, page, snapshot and
file selection. Catalog account actions only change fixture identity. Production
connection state replaces the fixture account when the HTTP controller is used.
The file preview presents the selected path in its labelled content header and
keeps the source Code Block scoped to file bytes and language highlighting;
catalog interactions verify those surfaces separately.

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
briefly substitute the repository-wide default log. Branch and working-copy
selection at the current repository operation MUST retain the loaded Branches
column, its paging cursor, filter, keyboard focus and scroll position. It only
loads the adjacent scoped history/detail. Reference and graph loading indicators
are independent: an empty loading graph MUST NOT replace already loaded branch
cards with skeletons. Actual automatic data refresh still reloads references and
preserves the selected scope.

Working-copy cards use the same title size as bookmark cards. Working-copy,
bookmark, JJ Log and Op Log cards share the column card surface and hover and
selected tokens derived from the foreground; their headers retain the shared
repository header token. Bookmark pointer hover ends when the pointer leaves the
card and does not change the Command View keyboard cursor. The keyboard cursor
is visibly highlighted only while the search input has visible focus; Enter and
pointer activation retain exact bookmark selection. A native empty working-copy
description is presented as `(empty) (no description set)` while the workspace
chip supplies its identity. Bookmark cards and their final visible item remain
square with a bottom divider. Empty Workspace and Conflicted filter results use
the shared Design Core shadcn Empty composition with icon media and
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

The main Projects navigation action uses the Lucide folder-root icon in both
sidebar states, the main Projects heading and its breadcrumb, distinct from the
folder-kanban icon for individual projects and the Your projects picker,
including when the sidebar is collapsed.

The catalog shell omits a redundant sidebar brand header and uses Community's
muted breadcrumb and stateful sidebar glyphs. Branch cards span their list
width; bookmark headings remain sticky during list scrolling. Column Canvas
headers use the theme-derived neutral grey background and bottom border;
operation-head status rows use a neighboring neutral grey so the information bar
reads separately from the header. Shell bodies retain padded content insets, and
collapsed columns span the usable padded body area. Each graph row places its
author and metadata after that row's occupied lanes, without aligning avatars
across rows. Stack branch cards and target history use these same per-row insets
without a shared avatar alignment override, and their column viewports scroll on
both axes. Wide graphs preserve readable row content and scroll horizontally
inside the existing column viewport while retaining vertical virtualization,
selection and paging. JJ Log and Op Log rows are square-edged and retain bottom
dividers through the final visible row. Column warnings and failures use the
shared Design Core shadcn Alert composition with a semantic icon and
theme-derived tinted background; they are square, full width and separated from
content only by a bottom border. Revision rows with both a commit ID and JJ
change ID use a shared split copy control with neutral GIT and accent JJ chips
and no copy icons. Each semantic button copies its complete underlying native
identifier and exposes it in a shadcn tooltip. At rest the display contains only
a configurable leading prefix (default four), with no dots or suffix. Hover or
keyboard focus redistributes space inside a constant outer width, revealing a
longer active prefix (default five), two dots and the final two characters;
short IDs remain whole. The inactive side keeps only its identifying chip. Both
sides remain tappable without hover; reduced motion disables the transition.
Success or failure feedback belongs only to the clicked side and resets after
1.5 seconds. The expanded half retains its chip beside a tick and 8px `Copied`
text (or a warning and `Retry`), with a live announcement. Feedback keeps the
tapped half expanded on touch devices. When another half expands, the previous
half keeps its chip and independent feedback tint. CSS animates the grid
redistribution; pointer, keyboard focus and copy feedback select the active
side. Either identifier may be absent; no empty button is rendered. A single
identifier shows the expanded prefix, two dots and final two characters
immediately, without requiring hover or focus. The default control uses 11px
resting identifiers, 10px expanded identifiers, 9px chips and a roughly 121px
total width at the standard 16px root scale. Each half retains 5px inline
padding, including at the trailing edge during hover and feedback. Graph rows
wrap the ID control below metadata before bookmark chips can overlap it.
Clipboard live announcements remain scoped to the split control; operation IDs
retain their separately labelled legacy control.

Operation-message text recognizes complete Git SHA-1 commit IDs (40 hexadecimal
characters), native commit IDs (128) and change IDs (32), rendering them through
the existing compact inline copy control. Abbreviation affects only display;
copy retains the entire identifier, and copying does not select the operation.
Prefixes, unsupported widths and IDs embedded in larger words remain plain text.
JJ Log and Op Log rows open their changed files inline, and selecting a changed
file opens the diff column. Op Log operation details and the selected file diff
render as equal-width stacked cards in the same Column Canvas column: the
operation card owns its title, collapse control, metadata and close action,
while the file-diff card keeps its file header and view controls underneath.
Parent operation navigation remains available without opening a standalone
details column. Missing selected-file diffs and binary or unavailable diffs use
the shared shadcn Empty composition with icon media rather than plain status
text. The selected patch retains its typed unavailable reason: only `binary`
uses a binary label; complexity, size, conflicts, multiple sources and
structural changes explain their distinct content limitation. Missing results
use a neutral unavailable label. Pending comparisons retain skeletons. These
contextual unavailable-content explanations do not duplicate transient request
toasts. An open standalone diff column with no selected file uses the same icon
Empty composition. Files opens the copied Community directory table until a file
is selected; directory, file and snapshot navigation participate in the
in-memory Back/Forward history.

Branch cards retain themed bottom dividers, including the final visible card in
filtered lists, and keep square lower corners. The branch command view container
is square, so its overflow does not round the final card. Branches, JJ Log and
Op Log omit the redundant body title bar; their Column Canvas headers identify
the views. Their shell body is a fixed, non-scrolling region; only each Column
Canvas body scrolls vertically. Mobile retains navigation/history/sidebar
controls without duplicating the page title. The right details rail inherits the
shell background with no visible border. It omits a redundant shell header and
close icon because the application toolbar already owns the sidebar toggle. Its
body begins with Community's compact page heading, such as `Branches/Bookmarks`,
plus the available create action at the end. The Details navigation menu follows
it. Selecting a JJ Log change, Op Log operation or branch bookmark restores an
Actions section beneath Details using the existing capability-gated commands.
Changing or clearing the selection updates or removes that section; unavailable
commands stay disabled, and operation selection omits change-editing commands.
Repository/source/snapshot summaries and selected metadata remain in the body.
The header Repository actions menu remains available in a 16rem-wide popup
bounded by the viewport so command labels fit on one line. Comparison-parent
choices and request errors appear in the main body.

Repository pages place Community-styled `Overview`, `Branches`, `Commits`,
`Op Log`, `Agents` and `Reviews` tabs at the top of the body. They route to the
existing Files, Branches, JJ Log, Op Log, Agents and Reviews destinations,
respectively. Trailing tabs move into More when width is limited. The repository
breadcrumb is rendered in the body below these tabs only while Overview is
active, immediately above its file content. The breadcrumb has no bottom border,
is rooted at the current repository and retains directory and selected-file
navigation. Branches, Commits, Op Log, Agents and Reviews do not render it.
Overview's file-body region has no padding, a top divider and a half-rem top
margin. This treatment belongs only to Overview and does not change other pages.
App navigation, Settings and Projects tab strips, repository section headers,
breadcrumbs and detail Back controls use a compact leading inset of at most
0.5rem. Section-header outer margins use the same inset; header spacing does not
change body padding, tab/title order or control hit areas.

The repository file tree is a shell body-left sidebar with a full-height
divider, small inner padding, keyboard-resizable width and the shared file
listing view-mode toggle. The copied directory table is a bordered, radiused
card with a commit/action summary as the first row; the file-tree toggle remains
beside the author avatar and the snapshot selector is aligned on the right side
of the same row. Its resting label tracks the selected bookmark or change after
that reference resolves to a commit; the resolved commit must not replace the
selected reference identity. The summary includes an icon-only
`Open commit details` action that switches to Commits before opening the
selected commit. At medium widths the commit message truncates without
overlapping the revision, time or snapshot controls. At phone widths the
message, verb and revision ID are hidden; author, relative time, commit details
and snapshot selection remain available in wrapping rows. The table
column-header row uses a neighboring theme grey distinct from the summary row.
The file-tree sidebar starts collapsed when no valid device layout is saved. An
explicit saved open/closed preference and width still restore; malformed or
unavailable layout storage uses the collapsed default. The existing summary and
selected-file toggles reopen the full file tree without changing selection. Root
directory views show a GitHub-style README, CODE_OF_CONDUCT, CONTRIBUTING,
LICENSE/LICENCE and SECURITY preview card only when those files exist in the
selected fixture snapshot. README is ordered first regardless of API order. The
tabs use the friendly labels `README`, `Code of conduct`, `Contributing`,
`License` and `Security` with Book, HeartHandshake, Users and Scale icons. They
are left aligned, use the smaller secondary type scale, retain a rounded
theme-grey hover and selected treatment, and keep a visible active underline.
The header measures its available width and moves trailing documents into a
`More` dropdown when they do not fit; it does not expose a horizontal scrollbar.
An asynchronous width measurement rechecks its original header and tab-list
identity after every render tick and stops when those nodes are removed or
replaced, including repository navigation and desktop/mobile transitions. When
the selected document is in that dropdown, `More` retains the rounded selected
treatment and active underline, while the selected menu item uses bold text and
a leading active bar. An icon-only edit action remains at the far end and opens
the selected document. The production loader reads those landing documents
sequentially so their preview does not exceed the bounded native read-admission
budget.

Selecting a file renders a stacked source preview: copied commit metadata,
deterministic file statistics, then the selected content. The metadata header
keeps the file-tree toggle as its far-left control. The content-card header
keeps the `Preview`, `Source` and `Edit` display controls visible while an edit
is in progress; non-Markdown text uses `Source` and `Edit`. Repository file
editing uses the full available editor width, with a compact left gutter and no
centered readable-width column. Mira's file-margin and line-width tokens are
scoped to the file editor; prose composers retain their own layout. Editor
content can shrink with its pane without horizontal overflow.

Edit mode replaces only the content-card body and replaces the selected filename
breadcrumb segment with a content-sized filename input with horizontal padding
and a bounded maximum width. It does not add a second filename field. The editor
footer offers `Save changes` and `Cancel`; saving opens a dialog that captures
the change description before the existing preview, apply, retry and status
flow. Returning to a directory or the repository root through breadcrumbs or
navigation history ends file edit mode, removes the filename input and releases
the workspace editing guard. Opening another file shows its ordinary preview
unless that file was explicitly requested for editing. An empty selected path
never starts an edit session. Edit mode uses the public Design Core Mira MDE
code editor with line numbers and filename-matched CodeMirror language support.
It renders flush with the content body without a field label, nested border or
focus frame; a focused CodeMirror surface and its editable content expose no
outline or box shadow. A static application stylesheet resets the editable
content's native focus outline so the served asset policy does not depend on
CodeMirror's injected reset. The same scoped treatment applies to Community
prose composers for review comments, replies and mutation descriptions,
retaining editable focus and toolbar controls. Its right-aligned line-number
gutter stays beside and vertically aligned with the content, and the editor,
gutter and active line retain the content-card theme background. Markdown files
open in safe preview mode, and draft preview/source modes retain unsaved
content. Source mode uses the public Design Core CodeBlock with the
filename/source body on the theme background. C/C++ and Rust source previews use
CodeBlock's public custom-tokenizer extension point with the application's
existing editor grammars while retaining the shared code component. Both
recorded and unsaved Source views enable syntax highlighting for the detected
filename language. Built-in languages use the shared tokenizer; C/C++ and Rust
receive the repository's custom extension, also used by Blame. Token spans use
the packaged theme stylesheet so highlighting works under the served
content-security policy without injected styles. Unknown languages remain plain
text, and source is never executed as markup.

An authenticated single-target bookmark exposes Edit for an existing bounded
UTF-8 ordinary file. The editor keeps filename and content independently
editable, previews Markdown safely, captures a new-change description in the
save dialog and uses EditFile preview/apply/status. Binary, oversized,
conflicted, historical and ambiguous selections remain read-only with explicit
guidance. Rename collisions, stale targets and protected bookmarks fail without
changing the displayed snapshot; commitment follows the new bookmark target and
destination path.

The deterministic LocalTrusted demo seed includes representative README,
CODE_OF_CONDUCT, CONTRIBUTING, LICENSE and SECURITY root documents so the live
repository exercises every supported landing-document tab after
`pnpm dev:reset`.

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
restores protected content. Live events reload the active view. A view following
the sole current code head advances automatically; explicit historical
operations and immutable commit/operation selections remain retained. Optional
Refresh can still capture current heads. Updates replace the current history
entry and preserve the deep selection when it remains available.

JJ Log and Op Log graph chips cap their inline size at 16rem and the available
row width. Long bookmark, workspace and device labels use ellipsis while their
complete values remain in accessible text and hover titles. Chip rows wrap
without displacing graph identities or creating horizontal overflow.

Every asynchronous browsing surface preserves its final geometry while loading.
Branches renders bookmark-card placeholders in its initial path column; JJ Log
and Op Log render their graph/status composition; Files renders its file-tree
and directory table composition. Commit, operation and file-detail reads retain
the selected context and show representative changed-file, diff or source
placeholders. Changed-file loading preserves the real card title and view-mode
controls while skeletonizing only the count and file rows. The states use Design
Core's public shadcn Skeleton, expose one labelled busy status per region and
keep decorative placeholders hidden from assistive technology.
