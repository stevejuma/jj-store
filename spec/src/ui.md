# Storybook presentation catalog

The Threads surface composes Community channel navigation, workspace
header/conversation, reply panel, member details and rich reactions. Its
adapters retain exact message revisions and authorized actors; page changes and
disconnection discard stale responses. Interaction/accessibility and cold
consumer qualification are required; visual baselines remain separately
approved.

The local Git-backed browser reuses these screens through explicit profile-aware
transports. Native snapshots retain manifest/WAL authority; local snapshots
retain binding generation, code operation and independent committed companion
selection. Routes accept complete Git SHA-1 or native commit IDs syntactically
and validate their width against discovered profile before reads. Cache keys
include profile, repository and connection session. External freshness only
offers refresh, including changes missed during disconnection. Unsupported local
search, code writes, reviews, conversion and synchronization remain gated.

The reviewed catalog application is the presentation authority for the served
browser. Production browsing, search, typed mutations and conflict editing share
the same components and asynchronous controller contract. Reusable components
receive presentation data and callbacks, while HTTP and in-memory controllers
supply interchangeable sources. Deterministic fixtures remain catalog-only.

## Repository-owned Community presentation

Rebase integration preserves collapsed section hints, project navigation and
profile-gated native Threads alongside the local Git-backed browser. Search
state types are kept in a small shared module so combining these contracts
preserves the production source budget.

JST-GOV-008 governs all Community-derived UI: components, types, styles, assets
and presentation helpers are local source with provenance, rather than imports
from Community. Threads use the channel toolbar, standard right pane and
separate outer details rail; the directory uses forum rows. The standard Thread
pane uses Community's bordered main-canvas treatment, while the separate outer
details rail uses its rail treatment and an independent named controller. Their
widths and open states persist under separate panel identities, including across
navigation/remount. Where repository details are supported, the outer rail can
stay open alongside Thread settings, contributors or replies; its global toggle
never closes the standard pane. The Projects hub's active tab title supplies its
level-one page heading without changing the copied header typography. Thread
settings expose native rename/lifecycle, identity, baseline, links and
contributor details under repository permissions. Native Markdown references and
composer entity pickers preserve full identities. Global search covers connected
authorized sources, offers four recent entities, context scope chips and
permission-gated create/browse actions. Content search includes stored Thread
messages/replies and WorkStore descriptions/comments; code search keeps its
independent operation pins.

The Thread toolbar supplies the page's level-one heading with the copied channel
typography. Its header and the standard settings, contributor and reply headers
share the shell geometry tokens and aligned separator; consumer header content
does not add vertical padding. The separate outer rail retains its own
treatment. The application owns a fixed viewport with no document scrolling. Its
Threads settings/create and search dialogs retain modal focus/overlay behavior
while disabling redundant document scroll locking through the public
`preventScroll` option. Cold built-asset acceptance checks that closing them
restores pointer interaction under the served strict asset policy.

Threads use the forum's full-width inline post composer above
Topic/Replies/Activity rows, preserving post content in native messages. The
channel toolbar ends with its kebab settings button, which opens settings
directly. Settings retain metadata, rename and icon-led lifecycle controls using
Community management rows; Remove uses its separate destructive-colour group and
confirmation dialog. Reaction chips use Community's Design Core shadcn tooltip
on hover or keyboard focus, showing reaction counts, value and loaded authorized
actors. Click retains exact-value toggling; hover never opens a modal popover.
Edited labels and reaction confirmation/error feedback use Chat Message Metadata
in the exact targeted main or reply message. Reaction unknown outcomes retain
Check action status and Retry same action there, without a page banner or a
modified Markdown body. Reaction feedback follows timestamp, edited marker,
reply avatars and reply count in the same metadata row. Pending actions show an
accessible spinner at the end of that row; status-check and retry controls are
icon-only with tooltips. Errors show an icon-only retry followed by a
destructive-colour failure icon and Failed label, with exact error details in a
keyboard-accessible tooltip. Reply panels reserve clearance for the first row's
hover actions. The owned message metadata slot carries reaction feedback
independently of general message supplements, so copy/action menus and other
content retain their existing presentation. Reaction controls and feedback share
the same target-keyed controller cache and deduplicated loads.

## Requirements

| ID         | Requirement                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| ---------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| JST-UI-001 | The catalog MUST build and run independently using registry dependencies, deterministic fixtures, isolated story state, runtime diagnostics and blocking interaction/accessibility checks.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| JST-UI-002 | The shell MUST directly adapt Community shell/navigation composition and preserve desktop/mobile, sidebar, header, focus and navigation mechanics using public Design Core exports.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| JST-UI-003 | Branches, JJ Log and Op Log MUST reuse Community graph/row/file/diff compositions, preserve native identities and conflicting targets, support fixture-only drill-down without transport or mutation capabilities, and expose a Community-styled page heading and create action in the right rail while selected native detail remains in the body.                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| JST-UI-004 | The complete catalog application MUST expose a typed state/actions controller with independent in-memory navigation, repository snapshots, file contents and profile actions, using copied Community file-browser and footer compositions without backend access.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| JST-UI-005 | The served browser MUST use the reviewed shell and repository components through an authenticated asynchronous controller with cursor paging, representative page and detail loading skeletons, explicit snapshot refresh and typed hash restoration.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| JST-UI-006 | Large repository collections MUST use bounded virtual rows, all resizable shell/canvas regions MUST persist versioned layout state with independent identities for the standard Thread pane and outer details rail, and catalog stories MUST exercise the production components.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| JST-UI-007 | Repository search MUST use the released Design Core Command View in a Community-adapted overlay, query the selected authenticated operation snapshot, represent readiness and paging states, and navigate results with stable full identities.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| JST-UI-008 | Authenticated repository commands and text-file edits MUST use Community-adapted actions, composer-backed prose fields, shared commit/change identity rows, searchable Design Core commit selectors and form/confirmation controls, preview the exact typed request before apply, preserve its request ID through unknown outcomes, and advance the displayed snapshot only from a committed receipt.                                                                                                                                                                                                                                                                                                                                                                                                |
| JST-UI-009 | Conflict editing MUST use the AppShell structural sheet with a dominant syntax-highlighted merge editor, use its built-in accept-all controls for text sides, retain exact native paths, ordered terms and fingerprints in bound state without extra metadata cards, stage bounded uploads separately, preview the atomic selection, and retain server-advertised unsupported or stale states explicitly.                                                                                                                                                                                                                                                                                                                                                                                            |
| JST-UI-010 | The application MUST bootstrap before repository UI, auto-open the sole LocalTrusted repository or show its installation Projects hub, and label unverified loopback identity as Local; operational backup/restore status UI remains deferred.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| JST-UI-011 | Authenticated connection MUST request one manual token, keep it only in memory, discover all currently authorized repositories, restore an ID-based deep link or show the Projects hub, and cancel requests, SSE and protected caches when the token is replaced, revoked or disconnected.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| JST-UI-012 | The profile menu MUST open a transient Settings destination with Local service, Repository and Search sections; replace repository navigation/content/actions/details while open; preserve the unchanged repository route and snapshot on return; keep its destination and selected section out of URL, browser history and storage; render accessible icon-only settings navigation when the desktop sidebar is collapsed; refresh advertised search health explicitly; and expose native graph preparation in Repository independently from text-search health, using only a graph-reported or explicitly selected operation.                                                                                                                                                                      |
| JST-UI-013 | Repository pages MUST expose a Community-style section header after the body tabs with a searchable workspace Command View; workspace context MUST use the exact native name, survive URL/history restoration at a pinned operation, preserve shared commit identities and keep operation/collaboration history repository-wide.                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| JST-UI-014 | Repository view navigation MUST live in the Community-styled right rail beneath its page heading and create action, with selection-aware, capability-gated commands beneath Details and no duplicate metadata panels; body tabs MUST present Overview first, followed by Branches, Commits, Op Log, Agents and Reviews, retain their pill styling, and move trailing tabs into an accessible dropdown when width is limited without changing route or workspace context.                                                                                                                                                                                                                                                                                                                             |
| JST-UI-015 | The installation Projects hub MUST expose authorized homes, repository and task directories, a shared native commit and agent Activity timeline with Activity-first tabs and recorded-agent filters, a repository-linked Projects list, shared safe Markdown previews, searchable project/task views, native identity routes, permission-gated dialogs and threaded Markdown discussion; use Community card/list layout controls and Mine/Other groups, searchable repository Command View selectors, Settings-style form rows and icon-only page-header filters, a centered top-header breadcrumb, and an original-message preview above the edit composer; keep independent bounded snapshot continuations and retained navigation state; and advance displayed metadata only on explicit refresh. |
| JST-UI-016 | Local management MUST expose explicitly linked device workspaces, qualified capture setup and queue recovery, editable Search with isolated worker replacement, non-destructive storage verification/checkpoints and configured sync controls through typed capability-gated interfaces; unsupported profiles and deferred operations MUST remain explicit.                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| JST-UI-017 | Threads MUST use the owned Community channel toolbar with a direct settings action, aligned pane headers, targeted Chat Message Metadata feedback, reaction count tooltips, bordered standard right pane, independently controlled outer details rail, settings, contributor and reply presentation, forum directory rows, native composer entity pickers and message chips; workspace search MUST show recent activity, server-filtered section and entity scopes, native focus links and permission-gated bottom actions without inventing runner controls.                                                                                                                                                                                                                                        |
| JST-UI-018 | The browser MUST expose the complete Buzz-derived device Appearance page, persist and restore app-wide preferences, apply resolved themes and typography to all owned conversations and overlays, retain native links and drafts, and offer working local link cards and responsive Focus/Split discussions without altering repository authority.                                                                                                                                                                                                                                                                                                                                                                                                                                                   |

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

Browser interaction fixtures exercise the same bootstrap and repository
discovery requests as the production controller. Authenticated fixtures supply
one memory-only token and an authorized repository page before opening a
repository; LocalTrusted fixtures discover without an Authorization header.

## Device appearance

JST-UI-018 owns the Buzz Appearance adaptation. The page exposes all 62 theme
styles plus the previous app's Original light/dark pair, System/Light/Dark mode,
paired transitions, ten accent choices, browser glass with 30-90 percent
opacity/reset, prominent navigation, 13/14/15 pixel body typography,
Compact/Comfy/Spacious conversations, local Compact/Rich link cards and
Focus/Split discussions. Preferences are device-local and app-wide; Settings
location remains transient. Defaults are System/Buzz, default font, Comfy
density, Compact links, Split replies, glass off and subtle active tabs.
Temporary previews never persist. Startup, cross-tab and OS changes share the
same resolved palette, including portalled controls and emoji pickers. Native
entity links retain chips rather than becoming website cards. Conversation and
reply drafts survive Settings and layout changes within the session. Focus hides
the standard Thread reply pane while preserving the independently controlled
repository details rail. Native/local capability and snapshot authority are
unaffected. Browser glass reveals the application background, not the operating
system desktop. Palette, font and preview assets are local under the existing
strict CSP.

Accent swatches keep circular selected rings centered inside their circular
buttons, including after changing accents, on hover and at mobile widths in
either color mode. Keyboard focus remains visible independently of selection.

Prose composers use the resolved app background for their entire input surface
(white in GitHub Light Default), with Buzz's light rounded border, no resting
shell shadow, foreground action icons and circular upward-arrow send control.
Their frameless Mira editor uses transparent public editor/code-editor
background tokens and bridges foreground, placeholder, border, popover, accent
and selection tokens from the owned composer to the resolved app palette. The
bridge captures app roles on the composer ancestor before Mira defines its own
aliases, avoiding cyclic tokens. These tokens stay scoped to prose composers;
standalone code and file editors retain their surfaces.

Normal and confirmation modal surfaces bind the public foundation background and
foreground tokens to the resolved app background and foreground, including
portalled dialogs. WorkStore form values use foreground; field descriptions and
placeholders retain muted foreground. These roles follow every palette without
changing the user's selected accent or device preferences. Original retains the
pre-Appearance Design Core semantic palette and its native accent in light and
dark modes, including System pairing, scoped gallery previews and persisted
restoration. Choosing it retains the saved accent for other themes.

## Transient store settings

Settings opens from the existing profile menu and replaces the repository
navigation and body with a jj-store-owned settings sidebar and selected section.
The sidebar contains Appearance, Local service, Repository, Search and Sync. The
content navigation begins with its section icon and single page title, followed
by section tabs when present. It has no return button; Back to jj-store remains
in the settings sidebar, including the mobile navigation stage. Appearance keeps
its descriptive introduction without a duplicate page heading. Repository tabs,
header actions and the details sidebar are absent until Back to jj-store
restores the same repository route and selected snapshot. Settings never writes
its destination or section to the hash, browser history or storage. A fresh
component starts on Local service; section selection lasts only for that
component lifetime. Mobile selection returns to the content stage and retains a
control that reopens settings navigation. On desktop, collapsing the settings
sidebar retains Back, Local service, Repository and Search as labelled icon
controls without clipped text; expanding it restores their visible labels.

Local service reports access mode, authoritative-storage readiness, HTTP and SSE
state and refresh availability. Repository reports the retained discovery
permission, repository and operation identities, conflicts, captured authority,
native profile, authentication, advertised APIs/actions and bounded request,
upload, read and event limits. Search loads status only when the repository
advertises search, confines load failures to its section and changes displayed
positions only after explicit refresh. Repository settings expose native graph
index preparation separately from text-search projection health. Graph
preparation uses an operation from an existing graph rebuild-required response
before the explicitly selected snapshot operation and requires index-task
support and an exact operation, independently of text-search support. Completing
it does not refresh or repair the text-search projection. Long identities,
capability lists and numeric limits wrap without horizontal overflow; status
meaning is always present as text as well as color.

Local service lists Git-backed JJ registrations in the same bordered Settings
rows as service status, with the name, wrapping path and availability on the
left and Reconnect/Remove actions on the right. Registration guidance sits left
of Add/Refresh actions in a matching settings row. Search status and editable
enablement/budget or file-scope controls share one bordered group, with labels
and descriptions left and controls right. Save/Cancel, index maintenance and
sync controls use that same description-left/action-right composition and stack
on narrow screens. Unsupported Search, Sync, graph preparation and local-folder
management use the public shadcn Empty composition with a suitable icon,
explicit availability text and no unavailable controls. Loading and recoverable
errors retain their status/alert presentation.

The catalog uses Storybook 10.5.7, Svelte CSF 5.1.2, Vitest browser 4.1.11 and
Design Core ^0.2.0 with the existing Svelte 5/Vite 8 toolchain. Direct
adaptations retain provenance and Apache-2.0 attribution. Shared primitives
remain public Design Core imports. Stories are visual-pending until human
review.

Repository settings includes a Local folders section when the service advertises
workspace-link management. Its explicit path form preserves the draft through
section changes and failed requests until successful linking or Cancel. Refresh
folders rereads saved device state without changing repository snapshots. Only
currently verified links display Ready; unavailable folders retain their path
and a binding/location prompt. Unsupported connections explain availability and
expose no link action. Repository switching/disconnection clears drafts and
ignores late requests. Folder paths wrap on mobile, and form actions remain
keyboard accessible.

Repository settings groups General, Local folders and Advanced values in
keyboard-accessible tabs with horizontal overflow on narrow screens. Tab changes
do not discard folder drafts. Repository, Search health and Sync identifiers
compose the public Design Core CopyableValue control directly. It displays an
abbreviated value with a compact copy icon, copies the complete value, retains
the complete labelled identity for assistive access and uses its shared
success/failure feedback. Missing values retain their availability label.

The optional native folder picker joins Folder path to a Git-icon Browse button.
Only an explicit click opens the dialog on the local-service machine. Selection
fills the draft; Link folder remains the separate validation/save action. Cancel
preserves the draft; failures retain manual entry and explain the fallback.
While picking, repeated browse and link actions are disabled. Repository changes
discard late selections. Unsupported platforms keep the manual form.

## Local management delivery

This staged extension keeps the transient Settings shell and explicit snapshot
refresh contract. It adds Local service, Repository, Agents, Search, Storage and
Sync sections as their operations become available. The milestone ledger is the
single source of delivery status; this section defines the accepted scope.

Local management uses versioned owned capabilities, profile-aware workspace
links, settings revisions and bounded operation reports. Service interfaces
carry owned types; server composition owns configuration and workers; CLI
integration owns physical workspace discovery and qualified agent setup. A
local-management capability cannot be obtained through a remote bearer session.
Existing exact authority/origin/protocol checks apply. GETs must not initialize
metadata, install hooks, probe executables or rewrite configuration. Explicit
setup previews name affected files; mutation requests carry typed intent, not
shell commands or arbitrary configuration paths.

Users may explicitly link an existing local folder after canonical path,
repository, backend and installation validation. Linking does not initialize
companion metadata, migrate files or snapshot the working copy. Links remain
device-local, outside checkout content, and are revalidated before each action.
The native profile uses its verified local-service binding. The Git-backed
profile reuses M13 discovery and companion authority; its browser management
capability stays unavailable until the owning M13.4 serving gate is qualified.
The profiles must not reinterpret one another's identities or prepared captures.

Agents show detection/version support, desired enablement, installation drift
and queue health for the selected physical workspace. Enable/setup and removal
reuse the existing ownership records and sanitizer. Removal preserves queued
captures. Automatic recovery runs with the service and shares the existing
`jj agent drain` implementation, stable request identities and backend dispatch;
a second retry command is unnecessary. A single drain lease prevents concurrent
publishers, but brief spool mutation locks must allow enqueue while publication
waits. Native recovery retains the hook's admitted principal. Unknown outcomes
remain pending, and queues cannot be silently discarded by unlinking a folder.
No agent execution or historical transcript backfill is introduced.

Search exposes enablement, metadata-only or explicit bookmark/workspace file
scopes, include/exclude prefixes and index budget. Save uses revision-checked
atomic updates of the startup-selected configuration, preserving unrelated
settings. Desired/effective states are distinct. Apply drains only the affected
worker, verifies a replacement generation before activation and preserves pinned
readers; repository access and RustFS remain available. Retry follows normal
recovery, whereas Rebuild constructs a new text-search projection from selected
committed data. Native graph preparation remains a separate action.

Storage offers current readiness, checkpoint coverage, bounded integrity checks
and non-destructive checkpoint publication through the running owner. Reports
identify the captured position and any incomplete check; uncertain publication
retains its original target and resolves through selected authority. Sync uses
existing controls for configured upstreams, local-only status, pause/resume,
finite frontier waits and the existing reconciliation preview/apply workflow.
WorkStore replication remains unavailable. Settings opened from the Projects hub
preserve that route; repository actions require explicit repository context.

Forms preserve drafts until Save/Cancel. Operation status remains queryable when
Settings reopens, errors stay in their owning section, and teardown or
repository switching cannot apply a stale response. Use Community composition
and public Design Core controls, with keyboard/mobile and cold browser
acceptance. No new backup/restore, destructive GC, global cache deletion,
service installation, upstream credential editing, Git transport management or
remote administration is included. Qualification and visual baseline approval
remain separate.

Settings content fills the available page width with a compact one-rem top and
inline inset; the first section does not add another top margin. Settings action
buttons pair their existing text with decorative leading Lucide icons using the
public Button icon slot. Refresh, link, cancel, save/apply, index
preparation/rebuild, retry and pause/resume retain their accessible names and
action availability. Stacked operation-head copy controls keep their natural
compact width.

Repository, Search and Sync subsection tabs occupy a full-width strip at the top
of the Settings content, matching Projects. Each page places its title and
decorative section icon at the far right of this strip, after any subsection
tabs, and omits the former body title and description. The strip stays visible
while scrolling and uses horizontal overflow on narrow screens. Keyboard
selection and tab-panel relationships remain intact; moving the navigation does
not reset form drafts or change action availability. Every Settings page starts
its top navigation strip with a circular, icon-only Back button labelled Back to
app, including Local service and unavailable sections without subsection tabs.
The control closes the transient Settings destination and restores the retained
application route and selected snapshot, including on mobile without reopening
the sidebar. Content width and responsive padding are scoped to the Settings
page component; other application panels retain their own layout.

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
briefly substitute the repository-wide default log.

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
split control; operation IDs retain their separately labelled legacy control.

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
text. An open standalone diff column with no selected file uses the same icon
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
Overview's file-body region has a compact inset around the file tree and content
surface. This padding belongs only to Overview and does not change other pages.
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
Root directory views show a GitHub-style README, CODE_OF_CONDUCT, CONTRIBUTING,
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
filename/source body on the theme background. Rust source previews use
CodeBlock's public custom-tokenizer extension point so `.rs` files highlight in
the catalog while retaining the shared code component. Both recorded and unsaved
Source views enable syntax highlighting for the detected filename language.
Built-in languages use the shared tokenizer; only Rust receives the repository's
custom extension. Token spans use the packaged theme stylesheet so highlighting
works under the served content-security policy without injected styles. Unknown
languages remain plain text, and source is never executed as markup.

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
follows Community's section typography and spacing. Selecting a bookmark shows
its name as a bold section heading, the target commit avatar, one-line message,
author and relative update time, then one-row status and target-count chips with
every exact target Git SHA below; selecting a revision replaces that selection
detail with change metadata. JJ Log and Op Log selection follows the same
replacement rule and exposes the enabled mutation actions from the header menu,
including bookmark creation and conflict resolution when native capabilities and
an exact selection permit them. Multiline mutation input always uses Community's
form composer. Fixed Edit description and Abandon targets use the shared commit
row with author avatar, current message, copyable Git/JJ identifiers, relative
commit time and bookmark/workspace chips. Create change, Create bookmark and
Move bookmark use the same row as a searchable Design Core Command View selector
over loaded revisions; search covers message, author, email, Git/JJ identities,
bookmarks and workspaces, and the selected full commit ID remains the typed
mutation value. Commit-selector avatars and row bodies align vertically with the
trigger chevron, including descriptionless commits. Non-interactive GIT/JJ
labels use the compact indicator-size oval chips in both the selected button and
searchable options, with equal badge widths and 10px labels one pixel larger
than the revision-ID copy buttons. A fixed identifier column stacks Git above JJ
and aligns the chips across options regardless of message length, reserving the
JJ row when its change ID is absent. Relative commit time sits directly beneath
the title at the same left edge, with bookmark/workspace chips alongside it on
that row. Repository navigation sections have no top margin.

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

## Workspace browsing context

The body section header follows Community Files: a workspace icon and
`Workspace` title at the left and a compact outlined searchable workspace
Command View at the right, after the tabs. Repository overview clears workspace
context; selecting a workspace resolves its recorded working-copy commit at the
displayed operation. Files and default JJ Log use that commit while explicit
bookmarks/revisions remain independently inspectable. Operation Log, AgentStore
and ReviewStore remain repository-wide. Workspace selection is read-only and
never changes physical checkout files.

The exact workspace name is encoded as `workspace` in every page hash, with an
explicit `at_op`. Shared working-copy commits do not merge workspace identities.
Missing workspace routes show an unavailable state rather than selecting another
workspace. Explicit Refresh retains workspace context, following its new working
copy when browsing that working copy. The workspace picker discovers names
through authorized pinned reference pages. Section headers, collapsed-sidebar
and mobile layouts exercise production components; changed stories remain
visual-pending.

The picker is disabled while a text-file draft is being edited or a mutation is
previewing, prepared, applying or awaiting an unknown outcome, preserving the
draft and stable request identity until that flow is finished.

## Repository destinations and tab overflow

The left sidebar retains repository discovery and search. Branches, JJ Log, Op
Log, Agents, Reviews and Projects move into a Details menu below the right-rail
page heading and create action, using Community repository-rail icon rows and
current-page semantics. Settings replaces that rail as before. Selecting a view
on mobile returns to the main content stage.

The Workspace section header has no top or bottom margin and retains its
horizontal inset.

Body tabs use the labels Overview, Branches, Commits, Op Log, Agents, Reviews
and Projects in that order. Overview is the existing Files destination; its
native file browser and legacy `files` hash remain intact. Width measurement
follows the repository-document tabs: keep a visible leading prefix, move
trailing tabs to More, preserve the selected hidden destination, and restore
tabs when width returns. Pill colors, shape and typography stay unchanged;
keyboard selection and dropdown focus remain available.

## Native projects and tasks

Projects is a repository-wide destination backed by WorkStore, independent of
the selected code operation. Project/task IDs use validated `project` and `task`
hash fields. A task route requires its project. Projects exposes linked
repositories, retired membership, versioned tasks with state and parent, and
threaded comments including edits and tombstones. Home permission governs the
forms. A linked repository opens through ordinary independently authorized
repository connection; absent target access remains visible as a metadata link.
Community task presentation informs the composition; Design Core provides the
form controls. Production imports no catalog data.

Each paged list retains its own immutable WorkStore snapshot and validates
continuation identity. SSE changes the refresh hint. Save receipts and unknown
outcomes do not silently replace displayed lists. Explicit Refresh projects
reloads the selected project/task and preserves pending request bytes. Status
`not_known` does not prove rejection; retry uses the same request ID and intent.
Session replacement discards late reads/receipts. Narrow screens stack the
project list, task list and discussion; controls remain keyboard reachable.
Story interactions and accessibility are separate from visual baseline approval.

Switching between Projects and JJ Log clears the destination-specific task or
evolution selection while retaining the full code operation pin. Browser Back,
Forward and reload restore each destination's own identities independently.

Navigation stories assert all seven destinations, including Projects in the
right rail. Conflict file-browser assertions await virtual rows after restoring
the sidebar; header actions, native file identities and retained drafts remain
required.

The shared cold native browser scenarios use the current repository tabs and the
Markdown editor textbox in mutation forms. External-publication assertions
capture the complete route after the initial seeded review association loads;
the unchanged route must then survive the freshness event until explicit
refresh.

Cold atomic-conflict acceptance targets the current named merge-sheet region and
full native conflict path, selecting the left term through the merge editor's
accept action. It still requires identical preview/apply requests, one complete
path selection with native term zero and the full native fingerprint.

## Installation Projects hub

The primary Projects destination is available after installation bootstrap or
manual-token discovery without opening a code repository. Its Projects, Tasks,
Reviews and Repositories tabs cover authorized repositories in that
installation. Each home reports its own loading, empty, failed and paged
results; successful homes remain visible when another fails. Repository
discovery failure is visible and does not imply a complete catalog. Creation
selects an existing writable home. Empty setup guidance points to the CLI;
repository creation and registration forms remain deferred.

Project views default to Tasks and order their tabs Activity, Tasks, Reviews and
Repositories. Search and task state, linked-repository and home filters apply
before server pagination. Project rows show linked repositories and active/done
task counts. Task rows show their project, state, target, parent, current
comment count and update time. Task details present Markdown, parent navigation,
independently paged subtasks and threaded discussion. Repository Activity is a
collapsed disclosure immediately after Subtasks in the main task detail. The
separate Task discussion panel exposes threaded comments and its composer
without an Activity disclosure. Project/task prose and comments compose the
owned Community-derived Markdown editor and renderer; dialogs and tombstone
confirmation use public Design Core controls. Native state names remain open,
in_progress, blocked and done. A board, assignments and dispatch remain
deferred.

Hub routes use `#/projects` with validated tab, home/project/task identities and
query/filter fields. Legacy repository Projects routes remain readable. Back,
Forward and reload restore stable selections and filters. In-session navigation
retains bounded loaded views, cursors, scroll positions, drafts and exact
unknown requests. Reload requires a new discovery/read; credentials and drafts
are never persisted in browser storage. Explicit refresh rediscovers permissions
and reads new roots. Revoked access clears protected content. Late responses and
mismatched continuations cannot replace a newer route. Selected-home SSE only
announces freshness; receipts never silently refresh the displayed list.

Code repository navigation uses a separate authorized connection. Project
metadata does not grant linked-code access. The main shell hides code operation
controls in Projects and presents the project/task rail. Stories remain
visual-pending; interaction, accessibility and cold runtime evidence are
separate acceptance gates.

The landing and selected-project tabs also expose Activity. It reuses the
Community activity timeline, commit card and inline Agent context disclosure in
project, task and review pages. Project feeds cover independently authorized
linked repositories (including retained retired links); a repository-targeted
task shows that repository's activity, labelled as repository context rather
than inferred task causality. Project-only tasks use their project's links.
Native commits retain their exact code operation and Git/JJ identities, while
agent activity follows only recorded associations. Paged reads preserve each
code and AgentStore snapshot, disclose partial/unavailable context, and never
substitute unrelated sessions. Explicit refresh advances the feed; access loss
and session teardown clear protected activity. Loading placeholders in every
main tab have a page inset on all sides.

Activity and agent transcript rows use Community's measured virtual-row stream
inside the public shadcn ScrollArea viewport. Approaching the loaded edge lazily
continues each independently pinned repository feed. Association discovery runs
only for mounted rows with at most two concurrent readers per page; sanitized
session details and transcript rendering wait for the Agent context disclosure
to open. Repeated mounts reuse the same read, and delayed results cannot replace
another row or a newer capture. Native commit links retain the exact originating
Projects route, filters and scroll position for the application's Back/Forward
controls. Repository-wide Reviews omit the workspace picker row, placing the
review heading at the top of the body.

Unsupported local JJ review capability uses the public shadcn Empty composition
with a review icon and an explanation, without an error alert or retry action.
Recoverable review read failures retain their error and explicit Retry reviews
action. Mixed directories preserve supported native repositories and their
pinned reviews alongside the unsupported state.

Reviews use the same full-width Community review rows as the Projects directory,
with measured virtual rows and cursor-edge continuation. Selecting a row opens a
full page with the review header as the primary navigation and no repository
tabs. Returning from the header opens the list. Reviews do not expose column
styling or sidebar layout controls. The review title and Back control remain
outside the scrolling detail body, so metadata, files, commits and activity
scroll without moving the header. Retired `review_view=columns` links remain
readable and open the list or selected detail with the same native identities.
Review metadata and disclosure content have an additional responsive body inset
to distinguish them from the header. The header Back control is vertically
centered even when the title wraps. Task disclosure alignment remains unchanged.

Selecting a native review opens Community's full-width review detail layout:
header, metadata, Description, Files changed, Commits and Activity disclosures.
Code and explicit-pair anchors determine the exact comparison; operation reviews
use their recorded operation changes. Repository/agent-only anchors do not
invent code changes. Files reuse the public diff composer and changed-file
listing with line counts from each exact native file diff, commits retain native
navigation, and Activity composes the shared commit/agent timeline with the
existing threaded discussion and workflow events. Task disclosure headings have
no leading padding and align with body content. Only Description opens by
default; Subtasks and Activity start collapsed and reset for each newly selected
task.

When a legacy project route carries a home code-operation pin, the hub retains
that full pin in its optional `at_op` field while WorkStore reads remain
independent. A code pin requires an explicit home/project route; opening another
linked repository selects that repository's own authorized code operation.

The left sidebar includes a primary Projects destination and expandable project
rows with active linked repositories underneath. Project destinations, project
rows and the collapsed project-section trigger use Lucide Folder Kanban;
collapsed Threads uses Messages Square, and repository entries and their group
retain Folder Git. The collapsed Projects Command View lists project rows
without flattening their linked repositories into options; project search,
pagination/retry controls and focus restoration remain available. Repositories
are selected from their own Command View, while expanded project rows retain
linked repository children. Its independently paged, unfiltered home snapshots
survive main-view filters and advance on explicit refresh. Missing code
permissions remain visible and cannot authorize navigation. Native review
directories read separate ReviewStore snapshots only in accessible repositories;
project views scope these to the project's links. Failed project reads cannot
fall back to installation-wide reviews. Reviews retain pinned continuations,
show failures and use the existing review workspace for detail.

Repository lists and cards follow Community's name/description, contributor,
commit/review/task activity bar and date layout. Native repositories use their
latest selected change summary and contributor; this creates no stored owner or
description. Each domain retains its own authorized snapshot, bounded loaded
counts are labelled partial, and unknown or failed homes do not imply zero
activity. Reviews use the selected initial-comment excerpt, native thread ID,
exact artifact, comment count, creation actor and last accepted event date.
Their list fields spread across the row; narrower panels retain the title,
identity and count, and cards retain the complete metadata. Both directories
provide the existing list/card controls.

Landing tabs, project cards, repository/review rows, task rows and task detail
sections copy Community Projects composition and styling, adapting native fields
and authorized actions. Discussion uses owned CommunityMessage and the copied
Community thread tree with avatar guides, hover/focus actions and the public
composer's reply/edit context. Native identities and creation/edit actors stay
exact; no relay events, assignments or Community links are synthesized. Project,
task, link and picker forms compose public responsive Field groups with
label/description rows and aligned actions like Settings. Empty directories,
links, subtasks and discussions compose public shadcn Empty with a centered
icon. Refresh becomes busy synchronously before discovery and remains busy
through sidebar and current-view reads; mutation controls are disabled through
that interval. Keyboard, mobile, permission, paging and cold browser acceptance
remain separate from visual baseline approval.

### Native collaboration action placement

The mobile main toolbar keeps navigation at the start, project breadcrumbs in
its center and the details-sidebar toggle at its far end. Projects landing tabs
start with Activity, followed by Projects, Tasks, Reviews and Repositories. Only
recorded Git/JJ code identities remain inline on agent session rows and headers.
Session, root, snapshot, author and comment identities belong in a
keyboard-accessible shadcn kebab menu that copies complete native values without
changing selection. Tool hover actions include the same menu for call, snapshot,
turn, sanitized input/output and recorded paths. Open portaled menus keep their
hover toolbar visible; touch exposes the actions without hover.

Native review and task bubbles use Community hover/focus Reply and message
menus. Native record menus group readable copy values in one keyboard-accessible
Copy submenu, with decorative icons matching message, comment, revision, author,
thread, agent, snapshot, tool and task values. Items copy the complete native
value and keep clipboard feedback and selection unchanged. Edit is offered for
visible comments created by the authenticated repository principal when write
access exists. The composer shows the original text above an edit and the parent
author/content above a reply, supports cancellation and retains draft/context
after a failed or unknown publication. Captured agent messages are immutable
evidence and offer copy actions rather than comment-edit callbacks.

### Repository-linked projects, documents and activity filters

The repository Projects tab retains the repository route, tabs and
code-operation pin and lists projects linked to that repository across all
accessible metadata homes. It uses the same Community project rows, card/list
control, Mine/Other groups, independent home continuations and explicit-refresh
rules. Opening a project enters its home-owned detail; Back restores the
repository list. Caches include the repository scope so different repository
lists cannot reuse each other's results. The installation hub remains a separate
destination.

All repository document tabs, Markdown file previews and edit previews compose
the owned Community-derived Markdown renderer with safe HTML policy. Inline
code, links, emphasis and fenced code use the shared preview. Executable raw
HTML is omitted from the safe preview; Source still retains the original file
bytes. Document selection uses a three-pixel indicator while preserving tab
overflow and keyboard navigation.

Project and installation Activity filters use Community's searchable Command
View in a shadcn popover, triggered by an icon-only control in the page header.
Projects, Tasks, Reviews, Repositories and Activity headers also expose an
icon-only Filters control beside their layout and refresh controls. It opens the
applicable search, home, repository and task-state fields in an accessible
popover, including when the details rail is closed. Active filters remain
visible through the trigger state; closing the popover preserves its search
draft and Escape returns focus to the trigger. The far-right rail contains
project details without a Filters section. Supported activity choices are all
commits, commits with recorded agent context and commits without recorded agent
context. Search and repository filters compose with those choices; URLs,
Back/Forward and reload retain the choice. Selecting an agent category discovers
only bounded association metadata with two concurrent readers, keeping code and
AgentStore captures pinned. Session bodies still load on disclosure. Unknown or
failed context is excluded from both categories and reported as incomplete,
never classified as absent. Filter loading is explicit; empty filtered pages
retain accessible continuation controls. Returning to all commits restores the
ordinary mounted-row lazy behavior. No task/review event history is synthesized.

Sync settings group Overview and Progress in keyboard-accessible tabs. Overview
provides explicit Refresh, Pause/Resume and Retry for a configured worker, with
local/offline readiness and reconciliation guidance. Progress keeps local and
peer frontiers separate, shows pending counts, fault/circuit state and Search
freshness. No upstream shows local-only guidance in the status table, with
WorkStore sync availability in an icon Empty state and no worker control
buttons; unsupported services expose no controls. Requests are single flight and
late replies cannot populate another repository. Pause is process-lifetime state
and does not persist across restart.

Search groups enablement, budget and maintenance under Configuration, source
selection and prefix fields under File scopes, and projection status under
Health. Drafts survive tab/section switches and failed saves. Save and Apply are
separate explicit actions; desired/effective state and operation progress remain
visible. Pending or failed requests cannot silently retry a save or apply a
stale configuration. Refresh recovers the current revision before another
action.

Projects and Repositories sidebar section expansion and individual project
expansion persist in versioned browser local storage. Explicit collapsed choices
win over automatic expansion of an active child, including after Settings closes
or the application reloads. Storage contains disclosure flags only; unavailable
or malformed storage falls back to normal navigation.

## Registered local JJ folders

The repository directory offers Add local repository with the existing joined
path/Browse field and DialogFrame. Inspection is read-only; explicit Add may
initialize missing companion metadata. Existing code, dirty files and project
homes remain unchanged. Registered local repositories reuse repository browsing,
Projects and Agents presentation with profile-aware identities and separately
pinned collaboration state. Settings expose binding recovery and removal.
Capabilities hide unavailable code editing, indexed search, reviews, native
maintenance and synchronization. Browser navigation preferences retain their
existing device-local format.

The development runner admits the existing Git-backed fixture without reseeding
its code or moving any sample project's metadata home. Restart acceptance checks
the three deterministic sample project identities and their separate repository
links, while allowing additional projects created during normal use.

Merged native Threads navigation is advertised only when the connected provider
exposes its Thread transport, or an available native home can be selected. Local
code browsing does not imply local Thread/reaction browser support. Switching to
a local provider clears displayed native Thread state while retaining any exact
unknown request under its original repository and principal. Route restoration
reconnects the requested repository before restoring independently pinned Thread
metadata; a Thread route does not require a code operation selection.
