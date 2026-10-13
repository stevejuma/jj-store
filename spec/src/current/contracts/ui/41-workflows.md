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
