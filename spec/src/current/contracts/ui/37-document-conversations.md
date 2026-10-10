# Document conversations

JST-UI-001, JST-UI-015, JST-UI-018 and JST-COLLAB-034 through JST-COLLAB-036
extend the catalog prototype into agent and ThreadStore conversations. M24
remains In progress until recorded live acceptance closes these gates. This
contract states required behavior, not proof of completion.

## Delta comparison and acceptance

| Behavior                                   | Delta evidence                                                                                        | Existing prototype                                                | Live acceptance                                                              |
| ------------------------------------------ | ----------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------- | ---------------------------------------------------------------------------- |
| Exact selections and collapsed discussions | Captured thread, screenshots and user observations                                                    | UTF-16 block/revision/range/quote; inline and collapsed cards     | Resolve immutable native/local source bytes and preserve unavailable anchors |
| Pending comments above input               | Supplied pending-queue screenshot                                                                     | Shared inline/sidebar/queue drafts; comments-only simulated sends | Receipt-backed batch publication followed by explicit dispatch               |
| Several comments at one anchor             | Supplied screenshot and retained discussion data                                                      | Grouped cards with explicit additional comments                   | Stable discussion and comment identities, exact edit revisions               |
| Several answers in one assistant turn      | Sanitized Hello World export has one turn with tools and two distinct answers                         | Catalog-owned answer sections                                     | Ordered live segments, exact persistent answer links                         |
| Reply tags                                 | Previously inspected Delta instruction text requires comment-reply blocks with copied quote selectors | Explicit fixture mappings; no live protocol                       | Versioned ID-based protocol in retained sent context                         |
| Double-check navigation and reply arrow    | Supplied references; hover behavior specified by user                                                 | Linked icon button, tooltip and answer indicator                  | Only validated committed destinations become navigable                       |

Delta's quote/context matching is reference behavior; this application uses
exact discussion/comment revisions. Exported CRDT offsets are not native
identities. Native hover/IME findings remain separate from fixture assertions.

## Renderer and draft ownership

One device-local appearance preference, conversationView = document or classic,
selects all live/recorded agent chats, ordinary threads and thread replies.
Missing or invalid values default to document. Existing Focus/Split placement is
independent. Standalone review and task panels keep their specialized renderers.
Both conversation renderers offer range comments, pending batches and reply
links.

Projects landing and selected-project agent responses, repository agent
responses and embedded recorded transcripts follow the same preference in Focus
and Split. A reader without ThreadStore capability still uses the chosen
renderer, with selection comments unavailable. Read-only embedded captures have
no next-message input or footer. Document responses do not mount a second
Classic composer in the host footer. Existing whole-session ReviewStore comments
remain separately identified, with their form opened explicitly beside that
review discussion; their drafts and recovery identities are retained. Document
response panels bound the shared conversation to the available panel height and
place session reviews inside its scroll area, keeping its own Send footer
visible. Classic retains its existing host review footer.

A shared controller owns drafts, audience, exact pending submissions and source
mappings across view switches, navigation, sidebar changes and remounts. Drafts
remain session-only; no draft text is stored in browser persistence. Typing
never publishes or dispatches. Enter adds a newline; Cmd/Ctrl+Enter or Send
submits the whole prompt. Escape retains a draft; Remove explicitly discards it.
Same-author editing resumes a pending exact-anchor comment; Add another creates
a distinct comment. Successful acknowledgements clear only matching submitted
snapshots.

Readers preserve the prototype's immutable content, CodeMirror widget lifecycle,
8px tool-flow/16px message spacing, transparent disclosures, code backgrounds,
folds, comments below collapsed headers, navigation previews and fixed footer.
Source/reply navigation opens the necessary disclosures, history pages, folded
lines and read sidebar before focusing the exact destination. Stale sources
retain their quote and explanation rather than relocating by text matching.

Ordered presentation segments retain native message/tool IDs and source offsets.
Streaming updates do not remount editors or overwrite drafts. Historical data
without ordering evidence remains honest; no inferred tool/answer chronology.
Exact range publication requires a committed source mapping. Nontext sources
support whole-section comments; unavailable/unpublished sources explain why a
pending comment cannot yet be submitted.

Existing permissions, cancellation, queued prompts, multiple recipients,
reactions, mentions, Sent-context inspectors, provider-aware runtime choices and
saved-change panels remain available in both renderers. Captured agent identity
is separate from current runtime presence. Switching a provider never silently
changes an existing process's launch configuration.

Document renders the next prompt as the document's inline, frameless input with
the author's small avatar and primary caret. Its editable typing region retains
the prototype's roomy minimum height and grows with the text. There is no extra
composer wrapper or independently scrolling input box; the transcript ScrollArea
owns that space. Classic retains the framed composer. Both presentations share
mention, emoji, formatting, audience and submission logic; switching preserves
draft text and selection, and waits for active input composition to finish. The
fixed footer retains Send and advertised runtime controls. Queued comments stay
above the same next-message input, and contextual status/recovery remains
reachable.

Live Thread and agent document drafts use the prototype's shared editor caret
theme: a 2px stroke, 1.2 height scale and primary colour. Immutable source
readers retain the same stroke and scale in theme grey. Prose drafts use 1rem
sans-serif text at 1.75 line height (16px/28px at the default scale); readers
retain their prose/code typography. The full caret remains inside clipping
bounds at the start and end of each line, including an empty draft; frameless
surfaces must not offset half of its stroke outside the editor. Draft text keeps
a small internal inset (4px vertically, 2px horizontally at the default scale),
with the avatar aligned to that inset. This clearance belongs to the input and
does not reintroduce the Classic composer wrapper. Classic composer presentation
remains independent.

The shared body accepts explicit history loading state in both renderers. A
compact centered “Loading previous messages” status button and spinner appear
above any retained messages, without replacing them. An icon-bearing shadcn
Empty appears only after loading finishes with no messages; hosts can supply
their own empty presentation without duplicating the default.

## Linked answers

Submitted comments remain grouped in the user turn; agent answers appear in the
transcript with the relevant source quote and user comments, followed by the
inline reply arrow and answer. Several answer sections may belong to one turn. A
sent comment without an answer has a noninteractive delivery icon. A validated
answer enables the green double-check button; hover/focus changes its icon and
shows Go to reply. Multiple destinations expose an agent/turn chooser. Links do
not resolve reviews or mutate comment bodies. Reply state belongs to the exact
comment revision, not whatever text is currently selected after an edit.

## Implementation review

E-CHAT-LIVE-005 implements the shared conversation projection for live sessions,
managed participations, recorded captures, ordinary threads and reply bodies.
The Document default and Classic setting use the same session-owned drafts and
receipt recovery. Linked participations keep their own repository feedback
reader, cancellation, Sent inspector, working state and saved-change controls.
Recorded views observe new addressed participations within the same body.

Exact range comments require a selected committed text part. Streaming and
unmapped raw JSON remain readable without invented selectable sources. Original
capture strings remain unchanged when numbered gutters, Markdown decorations or
middle folds hide presentation characters. Navigation retains a pending exact
destination until the mounted reader or reply node is revealed.

The affected Storybook family and focused controllers verify presentation and
state; a controlled ACP fixture separately qualifies cold publication and
restart. Neither proves model compliance or native IME behavior. The execution
and collaboration chapters own delivery, permission and authorization gates; M24
remains In progress and visual baselines remain unapproved.

Qualified local repository threads expose the existing companion ThreadStore
transport through the repository view and sidebar. Local and native pagination
retain their own snapshot identities and reject changed pins; navigation never
fallbacks to another repository for an explicitly selected owner.

E-CHAT-LIVE-006 records the isolated native/local built-browser flows and cold
restart. Comment actions appear immediately on focus/hover, avoiding an
intermediate low-contrast opacity state. Native capability refusals remain
explicit; qualification uses a supported disposable WorkStore home.

A managed Send admitted as `dispatching` retains the exact request and exposes
its recovery actions until all addressed agents acknowledge it. Matching live
acknowledgements trigger reads of the exact child receipts, never redispatch.
Receipt identity must match both request and session; unknown or unavailable
outcomes retain recovery. An admission response alone cannot clear the draft or
strand it behind an unavailable recovery action.

A committed comments-only prompt retains its user turn and canonical sent-batch
summary even when its authored text part is empty. Publication must not remove
the summary when an initially uncommitted row gains an exact source mapping.
E-CHAT-LIVE-007 observes real installed Codex answers to three comments in two
reply blocks within one turn, plus a separate mixed-send flow. This qualifies
these recorded runs, not guaranteed model compliance or other adapters.

Inline Markdown messages, comments and context use Mira's embedded preview mode.
They must not inherit a full-document trailing viewport padding when reader
styles load later. The intentionally blank next-message region is independent.
E-CHAT-LIVE-007 includes a failing-then-passing late-styles geometry regression.

Execute results include retained stdout, formatted_output and aggregated_output
parts as terminal output with exact committed source pointers when available.
Older terminal-only captures may display retained journal command/output as a
whole-section projection; they must not manufacture text ranges into the
terminal reference. Raw JSON remains independently reachable. E-CHAT-LIVE-009
records the real sample and Unicode/empty-output source mapping regressions.

## Workspace files from the conversation footer

The footer identifies the inspected agent, repository and verified workspace.
Its compact footer button uses a Git folder icon, smaller text, a subtle themed
grey background at rest and themed hover feedback. When wrapped, its leading
edge aligns with the formatting row above; runtime and Send remain trailing. The
repository label shrinks and truncates before controls wrap, retaining its icon
and a compact minimum target while preserving the full accessible label. The
menu aligns with its leading edge and its rows align left. It shows known branch
facts and View files, with a Copy submenu for Project path (the inspected
directory verified by the server) and the exact Workspace name. The filesystem
path is not displayed in the menu. An inspection selector changes only the
viewed binding; it never rebinds an agent or changes a checkout. Footer controls
wrap at constrained widths while keeping Send accessible and the full truncated
repository label available.

The collapsed formatting toolbar keeps its intrinsic width; repository and
runtime controls wrap before they can obscure its buttons. Expanded formatting
owns the full first row and puts the repository, runtime and Send controls on
following rows. Within that first row, formatting actions retain horizontal
scrolling and keyboard access when the panel is narrower than the action set.

Opening the footer menu performs no workspace read: its actions and copy values
come from the session's verified binding projection. Listings and bounded file
contents are cached in session memory by binding, tab, comparison and path.
Reopening clean files, tabs or panels reuses that data immediately. Matching
application events and explicit refresh invalidate current observations; hidden
sessions retain dirtiness until reopened. Healthy idle panels perform no
polling. One owned reader coalesces events and uses the shared
outage/failed-read fallback. Exact retained comment sources, drafts, scroll and
disclosure state survive invalidation; an explicitly opened historical source is
never replaced by current bytes. Late reads cannot repopulate an invalidated or
replaced selection.

The inbox agent session owns the workspace rail and file inspector. They mount
only for that session, retaining its selections and drafts while away. Opening,
closing or resizing them must not change the original repository/project rail or
body detail layout; their shell controllers have independent identities. Other
routes keep their original rails, contents and appearance. The workspace footer
entry is currently scoped to inbox agent sessions. The inbox header also
provides its own far-right toggle, available before opening the footer popover.
It toggles the workspace rail without changing the body inspector or the
original repository rail. File loading uses the shared repository reader
skeleton with an accessible loading label, keeping the requested path visible in
the inspector header; it does not show a text-only loading row.

The conversation workspace rail uses the standard body-sidebar surface, with
Files and Changes tabs in its header, with the standard bottom divider retained
beneath both the tabs and the file inspector title. The adjoining inspector edge
is straight in both inline and constrained replace-main layouts while the rail
is visible. The file-list toolbar divider spans the rail width; the tree owns
its row insets without extra parent padding. Tabs reuse Projects navigation
pills; Refresh stays at the header’s trailing edge. File readers have padded,
transparent content with syntax highlighting and a separate line-number gutter.
Source bytes and exact comment selections remain unchanged; the hover/keyboard
comment action replaces a separate Comment on file row. Comparison controls use
compact adjacent selectors with Lucide icons. They prefer one row, including at
the rail’s minimum width, shrinking labels before wrapping when still less space
is available. Long labels truncate within each control and retain their full
hover titles. Choose branch appears on the base selector when no default exists
instead of repeating that description below it. Other unavailability reasons
remain visible. Refreshing a listing retains its rows and reports busy state in
the header, without an appended loading-more row. Other repository rails retain
their existing presentation. Empty Files and Changes retain the listing toolbar
and view controls, with an icon-bearing shadcn Empty inside the listing body.
Initial Files and Changes loading mirrors the shared listing toolbar, icon/name
rows and directory indentation; Changes keeps its comparison controls visible.
It reuses the owned file tree and public listing controls. Current files include
tracked and nonignored untracked paths. Comparison selection defaults to Branch;
missing verified bases require an explicit base selection. Since last commit
includes unsnapshotted edits. Last turn uses exact completed before/after
boundaries and excludes subsequent edits. Unsupported comparisons and incomplete
totals retain their explanations. Bounded asynchronous reads discard obsolete
results and never publish or snapshot repository state.

Standalone sessions mount the workspace inspection panels. An unselected agent
profile must not reserve the body inspector's shell panel identity. Selecting a
file opens the shared full-height, resizable body inspector. Changes offers
highlighted unified/split diffs and explicit before/after selection views.
Selections retain exact UTF-8 ranges and source fingerprints; file drafts join
the same pending queue and Send in both conversation renderers. The application
owns selection, disclosure, scroll and draft state across panel and renderer
changes. Historical source navigation opens the owning repository and retained
revision, diff side and range, rather than selecting current bytes by matching
quotes.

File discussions remain ReviewStore-owned. A historical batch holds qualified
references; read projections resolve those references through independent code
repository authorization. Published source/reply navigation must survive reload,
workspace cleanup and retained-source reads after GC. Pending source validation,
partial publication and unknown outcomes must preserve their exact requests and
drafts. Browser rendering never marks a file discussion answered on its own.

E-CHAT-WORKSPACE-004 records the initial file draft and UI integration.
Historical review projection, retained navigation and cold interaction
acceptance remain subsequent gates; M24 and M13.3 stay In progress and visual
baselines unapproved.
