# Repository-owned Community presentation

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

Workspace search context, section and browse actions use each entity's icon with
Community's compact search badge at the bottom right. Create actions use the
matching entity icon with a plus badge. These decorations are hidden from
assistive technology; labels, keyboard selection, permission gates and exact
navigation remain authoritative. Search result icons retain their entity type.

The selected Thread inbox supplies the page's level-one heading and current
status. Its title and embedded Conversation details use the same 3.25rem minimum
header geometry. Outer contributor and reply panels retain the standard public
shell header; their top edges align with the inbox while their heights remain
independent. Consumer content adds no vertical padding to the public header. The
application owns a fixed viewport with no document scrolling.

Thread reply navigation scrolls and focuses the exact shared conversation item,
including a selected search result; focus never targets the inner immutable
reader or resets a draft.

Threads settings/create and search dialogs retain modal focus/overlay behavior
while disabling redundant document scroll locking through the public
`preventScroll` option. Cold built-asset acceptance checks that closing them
restores pointer interaction under the served strict asset policy.

Repository, selected-project and installation-wide agent responses follow the
device Thread layout preference. Split uses the same standard body right sidebar
as Thread replies, with its public header, Close, scroll body, mobile lane and
resize mechanics; widths persist under the standard panel identity. Focus uses
the existing structural discussion sheet, covering the body and both right
panels with the same desktop allowance and mobile insets as Thread replies.
Selecting a session leaves the loaded directory mounted. Close, Escape and the
scrim return focus to its selected row; switching layouts retains the selection
and opener. The outer details rail stays independently controlled. Admitted deep
links restore the selected presentation after a cold page load. Agent item
wrappers reserve two pixels above and below each row, leaving at least four
pixels between neighboring hover/selected backgrounds in both directories and
repository listings, including measured virtual rows. The agent header and
classic session comment footer remain outside the scrolling body in both
layouts. Document mode places session reviews after the recorded transcript. A
collapsed Add review action with a review icon and chevron opens the same muted
disclosure and white avatar/editor card as draft comments; Classic keeps it in
the footer. The editor retains Markdown controls and adds human comments through
an exact immutable session anchor; saved comments appear after the recorded
transcript. Drafts retain repository/session identity across Close, Settings and
layout changes. Read-only and unsupported local profiles do not offer a writable
composer; pending or unknown outcomes retain drafts and stable request
identities.

Threads use the forum's full-width inline post composer above
Topic/Replies/Activity rows, preserving post content in native messages. The
empty Threads listing fills its available viewport and centers the public shadcn
Empty icon and copy below the new-post control, including read-only and narrow
layouts. Populated topic rows retain their directory layout. The channel toolbar
ends with its kebab settings button, which opens settings directly. Settings
retain metadata, rename and icon-led lifecycle controls using Community
management rows; Remove uses its separate destructive-colour group and
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

Every reaction-capable chat message uses the same placement: Add reaction
belongs to its hover/focus actions alongside Reply and the message menu;
accepted reaction counts remain below the body and feedback stays in metadata.
Review comments, recorded agent messages, task comments, channel messages and
thread replies do not add a separate picker row. Native target identity,
read-only permission, pending-action gating and custom emoji toggling remain
unchanged.
