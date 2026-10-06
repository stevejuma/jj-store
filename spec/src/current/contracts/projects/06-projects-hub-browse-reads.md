# Projects hub browse reads

The Projects hub discovers currently authorized homes within one installation.
Repository-scoped browse summaries search title/body before pagination and
expose project task counts, task comment/subtask counts, timestamps and parent
labels from one selected WorkStore root. Projects sort by case-insensitive title
and stable identity; tasks sort by descending update time then stable identity.
Filters and continuations bind the exact query and retained root. The hub keeps
independent home snapshots and visibly reports partial discovery, failed homes
and remaining pages. Linked repository metadata never grants code access.

Native WorkStore first pages MUST reuse a bounded validated projection keyed by
repository, epoch and selected immutable root. The projection retains data only,
with ordered entity keys, normalized searchable fields and counts computed in
one pass over each catalog. Each first browse query filters compact keys from
the complete root, then hydrates only its returned window. Its opaque
continuations retain those ordered matching keys, so deep pages neither rebuild
summaries nor repeat sorting, body normalization or count scans. Multiple pins
of one root share its charge; query-key storage is charged separately. Every
request retains fresh authorization, inventory limits and epoch checks. Cache
loss, expiry and new selected roots cannot broaden a query or hide failed
reconstruction. Local browse uses the same selected-window projection; local
reads MUST reuse validated immutable root data and bounded exact-query matching
keys across independently reopened facades. They still admit the full committed
companion selection and reread its Work blob on every request. Cache keys bind
repository, selected Work object and SHA-256 of those admitted bytes; cached
data grants no selection or access capability. Browse, task activity and detail
pages share ordering/counts and hydrate only their returned window. Expiry,
eviction or process loss may rebuild the same exact projection, never broaden a
query or hide failed companion admission. Comprehensive capacity/pressure
qualification remains a separate slice.

Repository directory task activity MUST use body-free counts and latest task
update times grouped by optional target repository and project from the complete
selected WorkStore root. An absent target means the metadata home. Groups sort
by target identity then project identity and page at 1..256 groups; a returned
group counts all its tasks regardless of the task browse page size. Native
groups are retained in the admitted immutable projection and resumed pages slice
that order without task-summary hydration or rescanning. Local groups retain
independent committed companion admission. Both transports bind exact pins and
query limits to their continuations, retain current access checks and report
unloaded groups explicitly. Cross-home/project card counts MUST sum these groups
without fetching or draining task browse pages; unloaded homes or groups keep
activity partial. Code and review summary optimization remains separate delivery
work.

The installation directory includes Projects, Tasks, Reviews, Agents and
Repositories. The sidebar reads unfiltered projects independently from main-view
queries and nests active linked repositories. Reviews remain owned by
ReviewStore and are read with separate target authorization and independent
pinned pagination. Missing project metadata never broadens project review scope.
Repository permission changes clear protected retained results on automatic
rediscovery.

Projects navigation uses Community's FolderGit2 icon, restores the section gap
above collapsible groups, and places each project expand/collapse control at the
trailing edge of its row without reserving space for an absent row menu.

Directory card/list controls MUST retain exactly one selected layout for pointer
and keyboard selection. Project list titles, descriptions, counts and avatars
align in stable columns, including wrapped narrow lists.

Directory card/list layout and collapsible Mine/Other groups copy Community,
including the inset content spacing beneath collection headings. Project titles
and descriptions MUST NOT be repeated as unstyled text above the collections.
The task title and refresh action share the first detail row, with refresh at
the far right. Subtask lists align with the detail content without an additional
list inset; review rows retain the collection heading inset and distribute their
five native fields across the full row, with the comment count at the trailing
edge. Community's absent author and timestamp fields MUST NOT reserve columns.
Task disclosures copy Community's review-section heading treatment: plain
secondary text at weight 500, an adjacent chevron, 1.5rem horizontal trigger
inset, 2.5rem row height and 0.1rem section spacing, while content retains its
existing alignment. Project filters live in the far-right rail; breadcrumbs use
the centered top shell header and the mobile toolbar. Forms reuse Settings-style
bordered field groups with divided rows. On a selected project, one Add menu
offers permission-gated task creation and, on Repositories in LocalTrusted mode,
local repository registration, with task-add and Git-add icons. The Link
repository header icon opens a separate Settings-style dialog; its picker and
manual full-ID option are absent from the page itself. Cancel and failed or
unknown requests retain the link draft. Successful linking closes the dialog
after the accepted metadata refresh; status/retry preserves the exact command.
Local folder dialogs use matching divided Folder, Name and Inspection rows and
retain registration state after the Add menu closes. Menu-to-dialog navigation
supports keyboard focus and narrow screens. Projects and tasks belong to Mine
only when their creation actor matches the current principal reauthorized for
their home. Repositories belong to Mine when the current session can administer
them; this is an access grouping and does not introduce repository ownership.
Unknown principals leave authored items in Other. Selection controls use one
searchable repository Command View with full native IDs as values, distinct
duplicate names, retained inaccessible selections, permission-scoped choices,
and optional project-only/all choices. Repository picker search sits in the
first row beside the trailing close control; its accessible title is visually
hidden because the placeholder already identifies the search surface. Editing a
comment previews the displayed original body independently from its mutable
draft, with a single truncated line and full-text title.

Task selectors use the same Command View composition with title, state,
repository context and shortened identity, retaining the full native value.
Parent choices stay within the project and exclude the task being edited. Server
search and snapshot-bound pagination remain authoritative; an unavailable
current selection stays bound until the user changes it. Description composers
occupy the full field width below their labels in project and task forms. Rail
search uses one Input Group with an inline trailing icon-only Search submit
action and an accessible, visually hidden label. The search control has no
visible border or focus ring, a transparent background and compact top and
bottom row padding, including a single search-only filter group, matching the
Command View search treatment. Task-state filters use the shared shadcn dropdown
radio group with a status icon beside the selected value and each choice.
Repository directory rows size native actions by their content rather than
Community's four-column activity metadata; names and access labels wrap within
the remaining width so opening the right rail cannot cause text to overlap.
