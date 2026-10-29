# Intentional Threads inbox and Codebase

JST-UI-012 and JST-UI-015 place Threads immediately below Agents. The Threads
inbox shares the owned agent inbox shell, repository groups, conversation body,
footer and inspectors. One canonical Thread may appear beneath every attached
repository; aliases share selection and drafts. Repository-free Threads appear
under No repository. Browse threads directory controls are replaced by this
inbox, and old directory links retain repository-filtered navigation.

Explicit creation accepts a title and zero through eight repositories. Opening
the form performs no publication. Internal conversation and annotation records
remain accessible from their owning conversations without ordinary Thread rows.
The outer rail uses the identical shared Files/Changes component as the agent
inbox. Codebase management stays in the footer and uses a searchable
multi-select repository command picker. Creation uses the standard Settings
field group, with its creation/recovery action in a separate right-aligned row.
The compact footer control opens explicit add/remove configuration. Agent
recipients select only a subset of this Codebase, initially all repositories.
Both footer repository popovers use the wide, viewport-bounded presentation;
repository labels truncate on one line instead of wrapping. Thread title and
status share one header row, aligned with the conversation details header.
Codebase and repository footer pickers have a theme background at rest and a
distinct hover background. The top-level Threads destination remains selected
while navigating conversations within its inbox; repository-scoped navigation
retains its exact Thread child selection. Files and Changes retain exact
agent/repository/workspace selection, and pre-execution file browsing creates no
agent workspace. Document and Classic share drafts and reply links. Pinned
repository changes compare committed sources with a parent or explicit bookmark
base; they do not fabricate an agent turn or mutable checkout. The shared
repository menu names its selection submenu **Workspace**. Confirmed Thread
removal returns to the inbox and refreshes its aliases; it does not reread a
tombstoned Thread that current reads intentionally hide.

The Agent repositories picker sits immediately before Agent in the runtime
control group and shares its compact ghost-button presentation. Codebase and
Workspace keep their grey resting backgrounds and a visibly distinct themed
hover surface. The Workspace menu has its own viewport-bounded width rather than
shrinking to its truncated trigger; menu labels remain readable.

Document messages use the same unframed flow as the agent inbox. Unavailable
discussion groups retain context without an extra outer card border. Thread
actions such as Edit message, Copy link and Reply in thread live in a Lucide
kebab menu within the shared hover bar, accessible on keyboard focus and touch.
They do not introduce permanent button rows beneath message content.

Navigation and inbox share bounded cached repository reads and application
WebSocket invalidation. Hidden panels do no background work, and healthy idle
views have no data polling. Thread publication invalidates the metadata home's
shared read cache before refreshing repository aliases. M24 remains In progress;
visual baselines remain unapproved and ResizeObserver qualification stays
separate.

Threads and agent session inboxes put the shared responsive navigation tabs in a
single left-sidebar header, replacing the separate title and body navigation
rows. Search starts as an icon; opening it replaces the tabs with an input
filling that row, focuses it and retains filter/New actions. Escape closes and
clears search and restores trigger focus. Opening search keeps the same compact
control height and header geometry. Both headers use the repository navigation
height, with matching small leading and trailing sidebar-header insets. Session
tabs retain Agents navigation and scoped agent/all-session routes; Threads
retains repository scope. Both use the compact label All for their unfiltered
tab. Active/Archived and Connected views filter only the loaded conversations.

The filter icon opens a searchable multi-select command view. Values within a
facet are alternatives; repository, agent, lifecycle/presence and harness facets
combine. Options use exact loaded identities, including No repository; duplicate
display names never collapse identities. Agent choices come only from retained
setup IDs and typed conversation participants, never title matching. Missing
participant metadata offers no invented agent association. The panel labels its
loaded-only scope; pagination remains available when results are empty, and
newly loaded/event-updated rows use the same selections. Filtering issues no
reads, changes no canonical selection/draft and never persists draft content. A
temporarily absent selected option stays removable; Clear filters restores all
loaded rows.

Settled empty and no-match lists use the shared shadcn Empty composition with a
Lucide icon; loading retains its skeleton, and filtering suggests adjusting the
search or selections instead of implying that no conversations exist.

Conversation rows expose absolutely positioned Pin and Archive actions on
hover/focus and touch, leaving their text geometry unchanged. Compact actions
join as a bordered split button on one grey theme surface, with distinct
individual hover states; keyboard focus follows the moved pin or returns to the
sidebar search control. Pins form the first expandable Pinned section only when
there are visible pins; a multi-repo Thread appears there once by canonical
home/Thread identity. Unpin restores its repository aliases. Device-local
archive excludes rows from All and allows restoration in Archived, without
changing Thread lifecycle or any agent state. A selected archived conversation
and its draft remain open. Versioned local storage keeps only canonical IDs in
separate Thread/session namespaces, bounded to 1024 pins and 1024 archives each;
no captured content or drafts are saved. Storage events update other tabs,
malformed or unavailable storage falls back without disabling the controls, and
no polling or service write is introduced.

## Planned shared file library

[M35 thread files](../collaboration/45-thread-files-and-attachments.md), under
JST-COLLAB-038/039, plans the same flat Files library in standalone chats and
intentional Threads, distinct from repository Files/Changes. It includes static
composer uploads, explicit Markdown/text Save, original-version message cards,
history/restore and draft-safe stale-save recovery. Existing owned Community,
released Design Core/Mira, notification and accessibility rules apply; Storybook
review precedes live adoption. This is planned presentation, with no current
runtime or visual-baseline qualification.
