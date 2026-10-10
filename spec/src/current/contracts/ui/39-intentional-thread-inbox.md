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
status share one header row, aligned with the conversation details header. Files
and Changes retain exact agent/repository/workspace selection, and pre-execution
file browsing creates no agent workspace. Document and Classic share drafts and
reply links. Pinned repository changes compare committed sources with a parent
or explicit bookmark base; they do not fabricate an agent turn or mutable
checkout. The shared repository menu names its selection submenu **Workspace**.
Confirmed Thread removal returns to the inbox and refreshes its aliases; it does
not reread a tombstoned Thread that current reads intentionally hide.

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
