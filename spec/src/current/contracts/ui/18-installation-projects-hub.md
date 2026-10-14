# Installation Projects hub

The primary Projects destination is available after installation bootstrap or
manual-token discovery without opening a code repository. Its Projects, Tasks,
Reviews and Repositories tabs cover authorized repositories in that
installation. Each home reports its own loading, empty, failed and paged
results; successful homes remain visible when another fails. Repository
discovery failure is visible and does not imply a complete catalog. Creation
selects an existing writable home. Empty setup guidance points to the CLI;
repository creation and registration forms remain deferred.

Mine and Other projects form continuous visual collections across their home
repositories. Adjacent home lists do not add padding between project rows; the
collection retains its opening and trailing insets in List and Grid layouts.
Home-specific identity, authorization, loading, paging and selection remain
independent under JST-UI-015.

Project views default to Tasks and order their tabs Activity, Tasks, Reviews and
Repositories. Search and task state, linked-repository and home filters apply
before server pagination. Project rows show linked repositories and active/done
task counts. Task rows show their project, state, target, parent, current
comment count and update time. Task details present Markdown, parent navigation,
independently paged subtasks and threaded comments. Activity is an initially
expanded disclosure immediately after Subtasks in the main task detail,
containing the task's threaded comments. Repository commit activity remains in
the project's Activity tab. The comment composer follows Activity at the bottom
of the main task page and remains mounted when Activity is collapsed. Tasks have
no separate discussion panel or Open task discussion button, and the
conversation Focus/Split preference does not change this task layout. Native
comment links open Activity and focus the selected comment; reply/edit drafts
and exact pending request identities remain retained. Project/task prose and
comments compose the owned Community-derived Markdown editor and renderer;
dialogs and tombstone confirmation use public Design Core controls. Native state
names remain open, in_progress, blocked and done. A board, assignments and
dispatch remain deferred.

Hub routes use `#/projects` with validated tab, home/project/task identities and
query/filter fields. Legacy repository Projects routes remain readable. Back,
Forward and reload restore stable selections and filters. In-session navigation
retains bounded loaded views, cursors, scroll positions, drafts and exact
unknown requests. Reload requires a new discovery/read; credentials and drafts
are never persisted in browser storage. Automatic refresh rediscovers
permissions and reads new roots. Revoked access clears protected content. Late
responses and mismatched continuations cannot replace a newer route.
Selected-home SSE queues automatic refresh; confirmed receipts immediately
reload their WorkStore view.

JST-UI-015 scroll capture also runs before a departing viewport is unmounted. An
immediate Back/return cannot depend on a later browser scroll event. The
captured route and hub own that position; another view or authorization session
cannot receive it. E-CI-BROWSER-001 retains the exact 400px restoration and
unsent-draft regression, without changing navigation or storage formats.

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
substitute unrelated sessions. Automatic refresh advances the feed; access loss
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

Unavailable review transport capability uses the public shadcn Empty composition
with a review icon and an explanation, without an error alert or retry action.
Recoverable review read failures use application-owned condition toasts with the
repository name, original error and explicit Retry reviews action. Activity,
Projects, Tasks and Agents use the same presentation for failed home or scope
reads; retry actions remain in their toasts. Successful homes and previously
loaded rows stay visible without duplicate inline error blocks. Repository
metadata failures retain their existing toasts. Dismissed unchanged failures
stay suppressed through retries and tab changes; successful reads, including
while a tab is hidden, clear the episode so a later failure can notify again.
Mixed directories preserve supported native and qualified local repositories and
their pinned reviews alongside the unsupported state. A local profile alone does
not reject its qualified review transport.

Reviews use the same full-width Community review rows as the Projects directory,
with measured virtual rows and cursor-edge continuation. Selecting a row opens a
full page with the review header as the primary navigation and no repository
tabs. Returning from the header opens the list. Reviews do not expose column
styling or sidebar layout controls. Repository and Projects review selections
use this same main-page detail; the thread Focus/Split preference never turns a
review page into a reply panel or modal. The review title and Back control
remain outside the scrolling detail body, and the writable comment composer
stays in a fixed bottom row. Metadata, files, commits and Activity/comments
scroll between them. The header and composer wrapper have no separating borders,
matching task detail while retaining the composer's own rounded outline. Resolve
and Reopen use the public secondary button variant in the compact size with a
leading status icon, keeping a visible themed background at rest and on hover.
Collapsing Activity and returning from the list retain the review draft and
explicit reply/edit context. A new page comment has no implicit parent; only an
explicit Reply action selects one. Retired `review_view=columns` links remain
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
no leading padding and align with body content. Description and Activity open by
default; Subtasks start collapsed for a newly selected task without a saved
choice. Collection groups on Projects and Repository tabs, and task/review
Description, Subtasks, Files, Commits, Activity and recorded-agent context
sections, and the Op Log operation summary, retain explicit open/closed choices
across tab navigation, remount and reload. Versioned disclosure storage contains
at most1024 opaque section keys and boolean flags; keys distinguish the page,
owning scope and stable group/entity section, never display names, filters or
operation pins. Unavailable, malformed or quota-limited storage falls back to
the existing defaults and remains interactive. Stories inject isolated storage.
Opening an explicit task-comment link still reveals Activity and its target; the
composer remains mounted independently of that disclosure.

The shell header's left-sidebar and details toggles use the released Button
token API for transparent resting chrome in both expansion states. Their
accessible expanded attributes, icons, size, pointer hover and keyboard focus
remain intact.

When a legacy project route carries a home code-operation pin, the hub retains
that full pin in its optional `at_op` field while WorkStore reads remain
independent. A code pin requires an explicit home/project route; opening another
linked repository selects that repository's own authorized code operation.

Your projects exposes a trailing Briefcase Plus Add project action when a
writable home is available; it opens the existing Create project dialog. In
LocalTrusted mode, the Repositories heading exposes a Git-plus Add local
repository action using the existing explicit folder inspection/registration
dialog alongside Browse threads. Compact Command Views offer the same actions,
retain keyboard access and close before their dialogs open. Authenticated
connections omit local registration. Opening either dialog does not navigate,
collapse the group or write repository state.

The Thread settings sidebar header has no bottom divider. Thread settings
metadata labels (Lifecycle, Revision, Code baseline and Linked activity) use the
same body font, size and weight as Name, Contributors and Thread ID. Thread ID
and Revision use the same reusable read-only row with muted value text,
whole-row full-value copying and circular tick feedback. Thread ID can
independently open its conversation; immutable thread revision IDs remain
copy-only. Rename thread uses DialogFrame and the bordered, divided Settings
Field group, retaining validation, draft and pending-write behavior. Parent-task
buttons show the loaded parent's status icon and title without a redundant
Parent prefix, with a visible secondary background; unknown parent metadata
keeps its ID fallback without inventing a state. Work selectors use a compact
circular check for the chosen value rather than a shortcut keycap.

Commit activity rows expose their pinned commit link across the row, including
byline and whitespace, and open the owning repository's JJ log with the exact
commit, change and captured operation. Keyboard and modified-link navigation
remain native. Revision copy buttons, author tooltip and agent disclosures keep
their independent actions.

The left sidebar includes a primary Projects destination and expandable project
rows with active linked repositories underneath. Project destinations, project
rows and the collapsed project-section trigger use Lucide Folder Kanban;
repository entries and their group retain Folder Git. Native and qualified local
Threads appear only as indented children of their owning repository in the
Repositories group; their child action is labeled Browse threads and there is no
separate Threads section. A Messages Square icon-only Browse threads action on
that group opens the connected supported repository, or an authorized supported
repository when no context is selected. Collapsed navigation keeps this action
in the Repositories Command View. Independent bounded repository thread lists
retain their owner, pagination and permission boundary without selecting a
repository or changing the main page. Unavailable or identity-changed local
folders do not advertise Threads. Local entries use the companion ThreadStore
transport and pin, never native publication or snapshot fields. Thread children
open their exact owning repository and stable thread. Browse Threads and
selected threads show the repository/Threads breadcrumb in the same desktop
header and mobile toolbar positions as Projects, with native route links and
retained code selections. The collapsed Projects Command View lists project rows
without flattening their linked repositories into options; project search,
pagination/retry controls and focus restoration remain available. Repositories
are selected from their own Command View, while expanded project rows retain
linked repository children. Its independently paged, unfiltered home snapshots
survive main-view filters and advance automatically on new data. Missing code
permissions remain visible and cannot authorize navigation. Native review
directories read separate ReviewStore snapshots only in accessible repositories;
project views scope these to the project's links. Failed project reads cannot
fall back to installation-wide reviews. Reviews retain pinned continuations,
show failures and use the existing review workspace for detail.

Repository lists and cards follow Community's name/description, contributor,
commit/review/task activity bar and date layout. Native repositories use their
latest selected change summary and contributor; this creates no stored owner or
description. Repository cards put native status chips beside the name, aligned
to the right of the first row. The description and access row use the full
content width; descriptions wrap as a two-line excerpt and retain their full
text in the accessible content. Every card in a repository grid stretches to the
same row height, with metadata aligned at the bottom. Each domain retains its
own authorized snapshot, bounded loaded counts are labelled partial, and unknown
or failed homes do not imply zero activity. Reviews use the selected
initial-comment excerpt, native thread ID, exact artifact, comment count,
creation actor and last accepted event date. Their list fields spread across the
row; narrower panels retain the title, identity and count, and cards retain the
complete metadata. Both directories provide the existing list/card controls.

Project listing titles, descriptions, repository counts, task counts and avatars
occupy aligned columns across rows. Card/list controls retain exactly one
selected layout; selecting the active layout cannot clear it. Avatars occupy one
aligned metadata column across rows, independent of repository/task counts and
relative-date text. Narrow lists keep that avatar column while wrapping the
project copy and counts without overflow.

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

Initial Projects tabs (Activity, Projects, Tasks, Reviews, Agents and
Repositories), repository tabs and selected task/review details show labelled
busy skeletons through connection, discovery and the initial data read. Loading
banners and premature empty or unavailable states do not replace those
skeletons. Once a snapshot is displayed, background refresh retains its content
and drafts; settled failures and unsupported features retain their explicit
notices.

Repository directory loading also reflects its visible-row metadata reads,
retaining their viewport observation leases and any already loaded rows.

All Projects hub and selected-project tabs use an icon-only New agent session
action with a shadcn tooltip in their title row. Agents places Manage agents
beside it with the same treatment and omits duplicate body actions. Task detail
uses the same icon launcher in its header. Launch retains the exact existing
project/task/repository context and performs the existing explicit launch flow;
this presentation never warms, dispatches or creates a workspace by itself.

Projects and selected-project navigation use the shared repository responsive
pill tabs, keeping a leading prefix and trailing More menu as the available
width shrinks. Search and filter actions retain their existing semantics and
panel relationships; opening search fills the navigation row. This presentation
adds no discovery, read, polling or mutation work.

Interactive sessions appear as avatar rows inside their authorized repository's
existing Agents collection, without a separate recent-runs section. Explicit
code bindings determine the groups, including each accessible binding of a
multi-repository session; legacy unbound sessions use their primary repository.
The unfiltered installation view also includes accessible repositories with only
interactive sessions, without adding captured-session reads. Project/home/code
filters retain their existing scope. Group counts include interactive rows;
exact admitted recorded aliases render once. Avatar presence carries connection
state without a duplicate Online badge. Row activation retains the existing
exact interactive-session navigation, and recorded rows retain their response
selection, paging and availability feedback.
