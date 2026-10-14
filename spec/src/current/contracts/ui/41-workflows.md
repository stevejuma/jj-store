# Workflows

Workflows uses the owned application shell, public Design Core controls and
maintained Buzz-inspired presentation. Source references and Apache-2.0
attribution are recorded in provenance. Drafts, exact pending requests and
historical run selections survive refresh. Visual parity approval remains
distinct from interaction/Axe acceptance. See JST-WF-008.

Limited browser viewports use the same body toolbar as Projects, with sidebar
navigation and one right-aligned Application menu. The desktop top navigation is
hidden in that mode. Overview, initial loading, History and selected run details
retain the toolbar; menu relocation preserves open dialogs and focus.

The overview has one create card with a muted icon tile. All grid cards share
the same fixed 15rem height and full column width, including creation and
loading cards. Columns follow the available content width. Existing cards use
Buzz's rounded muted surface, colored trigger/executor tiles and up to three
overlapping action tiles, with the first action in front. Their main heading
describes the recorded trigger and first action, followed by a remaining-step
count. It clamps to four lines with a full-text hover title; repository,
workflow name and version date occupy the footer. The switch and menu remain
independently accessible. Metadata home selection belongs inside creation.
Runner labels use Design Core chip entry. The dialog has equal inset spacing
without a sidebar. Unfinished YAML can return to Form; invalid YAML retains its
text and shows the last valid form draft with a toast explanation. Closing
retains both drafts. YAML mode omits the right inspector and uses the whole
padded dialog body for the editor. Returning to Form retains its inspector
visibility preference; selecting a block reopens a hidden inspector.

The footer has a top border. Validation and command failures use the shared
notification service under JST-UI-019, without inline exception messages.

The Form inspector uses compact Settings rows with icons, labels, descriptions
and bounded controls, stacking long editors and narrow rows. Its compact close
control hides the inspector without deleting a trigger or action; selecting a
block reopens it. The padded body has no dividing border. The inspector has a
muted background, rounded corners, inset separators and independently scrolling
content with bottom clearance above the footer.

The trigger inspector includes an optional Description textarea for the workflow
in both Create and Edit. It binds the existing definition description; Form/YAML
switching, closing/reopening drafts and save retain it. Selecting a step
preserves the description while presenting that step's controls.

Create/Edit workflow remains visible at the header's left. Its title, centered
compact name chip, menu and close controls align vertically; narrow headers
place the chip in a second centered row. The chip has a workflow icon and
circular pencil control. The pencil reveals a ghost input with no border or
shadow; a circular tick or Enter accepts its draft value and restores the chip.
Escape restores the previous value. The whole editable chip owns the focus ring;
its circular edit/accept buttons use the white surface on hover. The chip itself
is transparent and borderless. The borderless Metadata home picker is centered
above the sequence with inset padding. Runner labels stack below their label,
with locally styled input radius and focus ring and a left margin aligning the
control after the label icon. Design Core is unchanged.

Sequence blocks are oval with an inset border and circular icon or step number.
Connectors are add buttons: arrows become circular plus controls on hover or
keyboard focus. The final add control is a circular plus. Pointer devices expose
an action's right delete control and left up/down column on hover or keyboard
focus; touch devices expose them on selection. Selection alone does not reveal
pointer controls. Deletion has a circular red hover treatment in both the
sequence and selected-action inspector. Reordering retains selection; endpoint
moves are disabled. Removing the last action is allowed in the draft, while
saving requires at least one action. The trigger has no delete/reorder controls.
The inspector delete button removes the selected action, and its adjacent close
button only hides the panel.

The dialog menu contains Trigger, Edit, Duplicate, Enable and Delete with icons.
Trigger and workflow Delete require a saved workflow. Duplicate retains the
original draft and creates a disabled editable copy. Enable remains a draft
change until save. Unknown workflow-delete outcomes retain the editor and
original command identity. Historical source and run selections remain pinned.

Loading cards reproduce the workflow icon sequence, title, description, controls
and footer geometry. Executor pickers include command/agent icons. Timeout uses
the shared Settings numeric entry, units and bounded gauge. Inspector icon tiles
have a slightly darker local background. A metadata home without an existing
WorkStore project is refused before allocating a workflow command identity, with
a toast and retained draft. Unknown outcomes and exact retry controls use one
shared toast with a retry action, never a body banner or inline exception.

The When picker shows icons for each supported trigger. Trigger-specific
Settings rows provide bookmark patterns, pinned native revsets, known/free-entry
author inclusion or exclusion, case-sensitive message/commit-description text
conditions, exact thread/message scopes and reaction value inclusion/exclusion.
Changing trigger kind clears incompatible draft conditions. Form and YAML use
the same typed backend contract; author IDs and code-author emails stay
distinct.

Author inclusion/exclusion and its Command View picker share one row, including
on narrow inspectors. Author choices retain their avatars beside selected
checkmarks. Thread scope uses a searchable Command View of existing threads in
the selected source repository, with an Any thread option and bounded pages.
Unknown saved selections remain visible and are never silently replaced. The
reaction scope uses the maintained emoji picker for Unicode, Like/Dislike and
repository custom emoji; selection stores the native reaction value or custom
emoji ID. Reads release their repository lease on source changes or unmount,
ignore stale responses and report failures through retryable toasts. Timeout's
unit control uses the same background as the numeric entry; its popup retains
the application Select surface. The workflow emoji popup fits the available
viewport height with edge clearance; its presets, search and scrollable results
remain accessible above the sticky dialog footer.

Trigger choices discover their data without relying on a previously visited
history or conversation view. Code authors come from bounded, operation-pinned
history pages for the selected source. Conversational authors come from admitted
repository principals and thread directory participants; installation profiles
hydrate their names and avatars. Agents are shown separately as unavailable
author filters until agent message events have an owned matching contract; setup
IDs MUST NOT be treated as authenticated principals.

The thread selector lists accessible native metadata homes with owner labels.
Selecting a thread selects its canonical source repository as well as its ID;
linked codebase aliases do not move thread authority. Empty source repositories
do not hide available threads in other homes. Fresh opening, focus and bounded
periodic checks refresh choices without overwriting filters or saved selections.
Revision match reuses the history revset editor, highlighting and source-pinned
completion metadata, without executing the draft query. Choice failures use
retryable shared toasts and stale replies cannot populate another source.

The overview action menu has decorative icons and a viewport-bounded width that
keeps its labels on one line. Run workflow uses the same Settings cards,
icon-bearing rows and bottom footer. Operation, Commit and optional Bookmark use
searchable Command View pickers backed by bounded native source reads.
Operations are read from an explicit published source head; commit and bookmark
choices are pinned to the selected operation, independently of the active code
view. Changing operation clears the selected commit/bookmark. Selecting an
unambiguous bookmark selects its exact commit; choosing another commit clears
bookmark context. Conflicted bookmarks are unavailable for automatic selection.
Paged choices retain their query/operation and ignore stale replies. Read
failures use retryable toasts. Pending run requests retain their original bytes
and source selection; disabled controls cannot alter them.

Workflow navigation uses the same owned pill tabs, toolbar search and filter
popover as Projects, with Workflows and History destinations. The edit dialog
has no History button. History combines bounded per-workflow pages from the
loaded native catalogs; catalog and run continuations stay pinned and retain
loaded windows after refresh. Filters cover metadata home, workflow, source
repository and run status; search filters loaded rows. Run rows follow task-list
presentation with status, source and time. Empty/loading states match that view.
The Workflows tab omits a redundant banner beneath navigation. History reuses
task ownership groups: Mine matches the requester's authenticated principal in
each metadata home, and other requests appear under Other runs. Requester
avatars in rows and Author in details use the maintained profile hover card and
server profile hydration, with an identity fallback when no profile is saved.

Opening a run replaces the list with task-style page details and a Back control.
Hash routes retain tab, search, filters, metadata home, full run ID, optional
publication pin and attempt. Back, Forward and reload restore these selections;
Back to history preserves list filters. Terminal rows open pinned history, while
active runs follow current evidence until explicitly pinned. Historical attempt
selection pins its publication; evidence continuations share that pin. Details
use the exact recorded definition, source, steps, logs and artifacts. Cancel and
retry retain existing authority, status guards and original request recovery.
Unavailable saved runs/attempts are never replaced with another selection. Run
links can inspect retained removed workflows by exact home/run identity;
catalog-based discovery lists currently registered workflows. Read failures use
retryable shared toasts, preserving loaded content and pending command bytes.

Run details retain their metadata card, including a medium author-avatar button,
recorded status, repository, host, definition and source. Operation-ID copy and
Git commit chips retain full recorded identities in their accessible labels. The
right header contains Refresh and Pin/Follow latest tooltips, icon-only Cancel
and a labeled Re-run all jobs action. Re-run uses the existing guarded retry
command with the latest attempt identity; older attempts and unknown outcomes
retain their existing action restrictions and exact-request recovery.

The body contains one owned Forgejo-15.0.7-style console beneath metadata,
without outer Attempts, Steps or Outputs disclosures. The dark console remains
in both application themes. Its header contains the workflow name and status, a
status-icon attempt picker with parenthesized relative dates, and a settings
icon inside the right border. An omitted attempt follows the latest one; the
picker also offers Latest attempt and exact historical attempts. The settings
menu controls seconds, reported timestamps and full screen, downloads complete
selected-attempt logs, and retains artifact downloads, agent transcript links
and evidence pagination. It replaces the former footer. Full screen uses the
public accessible Dialog with content above its overlay; exit preserves expanded
steps and loaded output.

One initially collapsed row per action shows its latest reported status,
disclosure chevron and duration. Order comes from the exact recorded definition;
historical attempts never mix evidence. Public Design Core Collapsible controls
support keyboard interaction. Disclosure identity includes
home/run/attempt/action and survives refresh through the existing device-local
layout preference. Expanding a step reads its pinned evidence on demand, showing
a refresh spinner in place of the chevron on first load. Closed steps do not
fetch logs. Failed reads and downloads publish retryable toasts, preserve loaded
content and never render inline exceptions.

Expanded rows contain continuously numbered ANSI-colored stdout/stderr lines.
Partial chunks join within their stream; ordering uses each line's first
contributing evidence sequence. CRLF and chunk boundaries do not add blank
lines. HTML remains escaped; process-supplied hyperlinks are suppressed.
Whitespace and Unicode runner icons are preserved, with wrapping for long lines.
Forgejo markers ##[group] and ##[endgroup] produce nested, initially collapsed
log groups with chevrons and keyboard controls. Ambiguous and legacy unscoped
logs remain in a clearly labeled Run output disclosure. Log downloads retain the
original text, ANSI and group markers.

Durations use reported start/end timestamps; missing starts show an em dash.
Pinned publications freeze elapsed time. Unreported actions remain pending or
not run after confirmed failure/cancellation; missing evidence never implies
completion. The console uses 4px corners, a #1f212b background, #383c47 border,
#eeeff2 foreground and #959cab muted text. Step margins are 10px, row padding is
5px/10px, labels are 14px, icons 16px and logs 12px monospace/20px with a 48px
number gutter and 10px text offset. Visible timestamp/seconds text uses the
muted foreground to meet contrast requirements. Forgejo source/version/path
references are recorded in provenance; this is an independent Svelte
implementation with no copied Vue source or runtime Forgejo dependency.

Bootstrap and repository discovery show the selected view's skeleton before
considering a metadata-home empty state. Workflows loading follows its cards;
History follows its heading, ownership group and inset
status/title/source/avatar rows. Run loading follows the detail header, metadata
and dark console header with three collapsed step rows. Initial reads suppress
empty-run explanations until settled; loaded content remains visible during
refresh. History rows have an inset before their leading status icon, no box
shadow or trailing row border. Workflow protocols, native authority and
immutable evidence remain unchanged.

LocalTrusted administrators configure the managed local runner in Settings →
Local service → Workflows. The normal Form shows enablement, name, labels,
command capabilities and admitted agents with setup avatars retained alongside
selection checkmarks. The local execution card has a one-rem gap above it;
polling combines exact entry, selectable units and a bounded gauge. Connection
and source selection are derived from this local service and recorded job
context; there are no repo, home or execution-principal inputs. Advanced starts
collapsed and retains optional recovery/CLI/polling overrides, owner policy and
explicit admission. The derived identity offers an explicit Admit local host
action; discovery and enablement never grant admission automatically.

The right-aligned Form/JSON toggle shows one editor at a time. Both share one
retained draft: valid JSON projects into Form; invalid JSON blocks switching and
Save with a toast. JSON accepts legacy attachment settings as well as the
version-2 environment document. Legacy mappings remain saved until the next
explicit host edit/save, which preserves environment, policy and recovery
overrides. Save is inactive; reload activates at an idle boundary.
