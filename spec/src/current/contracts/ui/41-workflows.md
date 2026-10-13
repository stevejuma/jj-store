# Workflows

Workflows uses the owned application shell, public Design Core controls and
maintained Buzz-inspired presentation. Source references and Apache-2.0
attribution are recorded in provenance. Drafts, exact pending requests and
historical run selections survive refresh. Visual parity approval remains
distinct from interaction/Axe acceptance. See JST-WF-008.

The overview has one create card, styled like the Agents listing with a muted
icon tile. Metadata home selection belongs inside creation. Runner labels use
Design Core chip entry. The dialog has equal inset spacing without a sidebar.
Unfinished YAML can return to Form; invalid YAML retains its text and shows the
last valid form draft with a toast explanation. Closing retains both drafts.
YAML mode omits the right inspector and uses the whole padded dialog body for
the editor. Returning to Form retains its inspector visibility preference;
selecting a block reopens a hidden inspector.

The footer has a top border. Validation and command failures use the shared
notification service under JST-UI-019, without inline exception messages.

The Form inspector uses compact Settings rows with icons, labels, descriptions
and bounded controls, stacking long editors and narrow rows. Its compact close
control hides the inspector without deleting a trigger or action; selecting a
block reopens it. The padded body has no dividing border. The inspector has a
muted background, rounded corners, inset separators and independently scrolling
content with bottom clearance above the footer.

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

Run details use the maintained operation-ID copy button and Git commit chip with
full recorded identity in their accessible labels. The right side of the detail
header contains icon-only Refresh, Pin/Follow latest and guarded Cancel/Retry
controls with public shadcn tooltips. These actions retain their existing grants
and exact-request semantics.

Attempts, Steps and Outputs use the task section disclosure treatment and begin
collapsed when no saved section preference exists. Section identity includes the
metadata home and full run ID; explicit expansion uses the existing device-local
layout preference. Attempts lists recorded status and host, with an exact
attempt picker. Empty Steps and Outputs use the existing shadcn Empty
presentation. Run details have an independently scrolling body; legacy modal
styles cannot override their metadata rows. History rows have an inset before
their leading status icon and no box shadow or trailing row border.

Bootstrap and repository discovery show the selected view's skeleton before
considering a metadata-home empty state. Workflows loading follows its cards;
History follows its heading, ownership group and inset
status/title/source/avatar rows. Run loading follows the detail header, metadata
card and three collapsed section headings. Initial catalog reads suppress
empty-run explanations until settled; existing content remains visible during
subsequent refresh.
