# Local stack workspace catalog

Stacks tracks binding discovery separately from connection; a binding seed is
not loaded content. Connection, discovery, module and first-read skeletons match
the selected view: Workspace's 330px Unstaged column, anonymous stack/composer
and footer; Branches' 320px target/filter/bookmark column and footer; Commits/Op
Log's shared history canvas with saved widths and selected details, without a
footer. Each region has one named busy status, inert hidden decorations and
reduced motion support. Pending reads cannot show empty/unavailable states.
Refreshes retain loaded content, drafts and selections. Failed reads use scoped
condition toasts and Empty with Retry; confirmed missing bindings use contextual
Empty, without duplicate inline errors.

JST-UI-020 requires an interactive local stack workspace catalog with a
transport-independent production controller and an injected stateful fixture
adapter. A capability-gated Stacks repository tab appears immediately before
Branches; there is no separate Stacks sidebar item. The repository stays
highlighted in left navigation while focused. Stacks replaces ordinary
repository tabs and trailing repository/refresh/Terminal actions with Back,
Branches, Workspace, Commits and Op Log using the shared body-tab styling and
accessible overflow. The Details toggle, centered workspace picker and existing
Stacks actions remain available across all four views. Commits and Op Log reuse
repository-wide history and its exact operation/selection reads. The existing
repository Branches page stays separate. The body header retains the separate
trailing Repository settings gear across all four views. It opens the scoped
Settings route and returns to the exact Stacks context. A closed Details rail
remains absent with zero layout width after returning; only an explicit open
action restores the rail and its content. An open rail retains its prior state.
The right sidebar retains the separately labelled Workspace settings action but
suppresses ordinary repository navigation and actions. Its far-right rail uses
the repository page-heading and compact Details navigation composition with
Branches, Workspace, Commits and Op Log links, retaining focused Stacks routes
and marking the current view. It omits the introductory workspace, target,
count, folder and selected-item metadata list; selections remain in the body and
workspace identity/configuration remain in the picker, footer and Settings. A
separate Actions section reuses compact sidebar icon buttons for Workspace
settings, refresh and explicit saved-membership reconciliation when needed.
Actions appear only while applicable under existing capability,
active-workspace, conflict, membership and pending-command guards; an empty
section is omitted. Resolve conflicts opens the same capability-gated workspace
editor from this rail and the local-workspace notification across all four
focused tabs, retaining the selected view and diffs. Unavailable or pending
commands cannot execute through either entry point. Resolved conflicts clear
their condition notice and remove resolution actions. The notification's Resolve
conflicts action uses the public secondary variant with a themed background at
rest, retaining a visibly distinct hover surface and visible keyboard focus.

Stacks retains the full bounded, operation-pinned text comparison alongside
native hunk descriptors. Draggable cards use the shared FileDiff
collapsed-context controls to reveal unchanged prefix, intervening and suffix
lines, assigning each context line to one card without duplicating changed rows.
Context expansion never changes issued hunk IDs or selection/drag authority.
Stacks and Op Log reuse the same conflict surface, file browser and editable
resolved-pane presentation; workspace conflict reads remain lazy per selected
path and drafts survive file switches and browser toggles. Navigation and
workspace switching remain disabled while the shell replacement editor is open.
Opening resolution preview releases its surface focus lease before focusing the
dialog. Local resolution previews/applies the selected complete file through its
existing guarded JJ command, retaining other conflicts, WIP and workspace
membership. Warning copy offers in-app resolution; genuinely unsupported native
shapes retain their explicit reason and do not fabricate an editor or enable a
command. Multi-sided text comparison uses one compact Command View popover
picker instead of a row of version buttons or an introductory paragraph. Its
24px trigger uses the public Merge Editor centered header slot between the
filename and change pager. A trailing kebab menu provides Wrap lines, Sync
horizontal scroll, Ignore whitespace and Ignore case; synchronization defaults
on and comparison filters default off. Workspace and native Op Log conflict
editors share these controls and the gutter fix. Items match standard menu
typography and grouped layout without an extra bold caption. Version-1 browser
local storage retains only these four boolean display preferences, accepting
earlier two-field records with comparison defaults, across editor remounts and
reloads; malformed or unavailable storage falls back safely. Changing comparison
settings preserves edited and accepted resolution text, while filters cannot
certify a native conflict resolved or authorize a command. Menu Escape retains
the conflict surface and returns focus to its trigger. Sticky line-number
gutters remain above horizontally scrolled text and current-hunk overlays.
Options preserve native order and show a version label, exact text line/byte
counts and a bounded first differing-line preview. Search includes this detail;
keyboard selection, Escape and trigger focus restoration reuse the shared
Command View/Popover behavior. Deleted versions stay visible but disabled.
Changing the comparison retains the reviewed resolution and still requires
explicit whole-file staging.

Unstaged uses the shared virtual FileListing in all three modes. File rows
retain the shared 2px gap so adjacent selected or warning backgrounds stay
visually separate, including after virtual scrolling. Current conflict paths and
unresolved target-side native terms show an accessible warning indicator and the
same background as the workspace warning banner; resolved target values are not
marked merely because their before-side was conflicted. Selected and hovered
conflict rows retain this warning surface with a visible selection marker.
Unavailable Stacks diffs show one description without a duplicate title,
spanning both column edges directly below the header; the existing hunk
scrollbar padding remains intact. The top workspace conflict banner appears only
while the Workspace tab is active; Branches, Commits and Op Log omit it even
when the retained canvas tab is Workspace. It has one row with its message, a
labelled conflict-count chip and a trailing Resolve conflicts button, without
listing paths or adding a separate title row; its warning icon is vertically
centered with the text, vertical padding is 4px and the row has an 8px bottom
margin. The count uses the smaller indicator font on a compact round secondary
surface distinct from the warning background.

Back uses a ghost public Button with circular hover treatment, an icon and
tooltip and restores the exact entry repository route and selections; a direct
or reloaded focused link without that session route falls back to Overview.
Frontend-only `context=stacks` on Log/Op Log and `stack_tab=branches|workspace`
on Stacks round-trip through reload and browser history, rejecting duplicate or
incompatible fields. Legacy Stacks links default to Workspace. The canvas and
its controller remain mounted through focused history navigation to preserve
forms, drafts, columns, diffs and pending commands; repository, binding and
authorization changes retain owned teardown. Binding revalidation must not
replace or repin an in-flight canvas selection/read or command for the same
owner. Switching canvas tabs revalidates that owner without rereading an already
loaded workspace. Explicit and automatic refresh retain their normal data reads.
History is read only when its tab is selected, and a direct focused history link
initializes workspace context without eagerly loading stack histories. This
presentation does not change local JJ commands or TOML configuration. The
application shell accepts an injected stack controller only for its matching
local repository. Branch selection opens a stack column with an explicit Apply
to workspace action. Workspace places Unstaged first, followed only by applied
stack columns, each with a staging/commit card and dependent bookmark cards
containing author metadata and the existing revision graph. It presents named
device-local combinations, independent WIP, explicit file/hunk assignment,
command previews, conflicts and undo. Ordinary local JJ execution uses advisory
previews and state/op-log inspection after unknown outcomes, without automatic
command replay. Named workspace settings live in `.jj/jj-store/stacks.toml`;
active refresh snapshots with normal JJ policy. The first production command
slice enables setup, create/switch and apply/unapply; other controls remain
capability-disabled until their owning engines are qualified. Native operation
changes invalidate lazy graph ranges and their cursors; commands can settle
before the new graph index is prepared. History is loaded again at the new pin
through the existing explicit preparation flow. Unstaged counts use loaded real
file summaries, with a continuation marker when incomplete; inactive workspaces
do not claim an unloaded file count. Mocked Git-backed and linked native
profiles use the same components. Unlinked or unavailable repositories cannot
activate checkout commands. The snapshot carries the current physical folder on
this device; absence of a folder link disables commands even when availability
is incorrectly reported ready. Branches and Workspace use the existing Projects
body tabs. Icon actions occupy the body navigation beside those tabs without a
second toolbar or profile chip. The workspace picker occupies the centered top
body header, using the same ghost repository picker as Settings and the same
header slot as the Projects breadcrumb; branch filters remain in their body
column. A picker already open when a pending read disables it must also disable
its choices and continuation, retaining the popover so selection resumes when
the read settles. The column canvas fills the available page width and retains
trailing space and its public settings. The canvas uses Design Core compact
rectangular Panel Toggle defaults in column headers and collapsed rails, with
controlled state and reversible hover/focus bar previews. Consumer icons and
controller actions remain supported. The canvas starts directly below the body
tabs without an extra layout gap or top canvas padding. The twelve-pixel leading
inset belongs to the horizontal Scroll Area outside its viewport, on both tabs,
with zero leading padding on the canvas and content wrapper. The gutter remains
visible beside expanded or collapsed fixed columns through horizontal scrolling
and clips scrolling content so none shows behind it. The unconfigured workspace
target-branch Settings form has twelve-pixel outer margins on all sides and fits
within the body width. Every branch, Unstaged, stack and diff column can be
resized with pointer and keyboard controls. Unstaged on Workspace and the
Branches picker on Branches use the public `fixed` column property to retain
their full expanded width or normal collapsed rail while the canvas scrolls
horizontally, with no replacement return rail. Pinning releases temporarily when
the scrolling columns cannot fit their minimum width and restores as space
returns, without changing saved widths or collapsed state. The branch column
uses the owned repository branch command picker, including its filters and
dependent bookmark selection. Opening a file diff inserts it immediately to the
right of its originating Unstaged or stack column. Each origin retains its own
open file, hunk selection and diff controls; opening, selecting or closing
another stack's diff does not replace it. Refresh and commands preserve valid
diffs, remove unavailable sources and hunks, and clear Unstaged selections when
switching named combinations.

The Branches picker groups applied bookmarks under Applied and all other loaded
bookmarks under Local branches, using the existing Command View groups and
retaining search, filters and keyboard selection. Workspace chips align to the
trailing edge of each branch title row. The page starts with its body tabs,
without a duplicate target-branch header. The Branches column places a
selectable workspace target branch card above its filter tabs, reusing the
repository bookmark card. Selecting a stack does not change that target. Only
the Branches tab shows this card. Selecting it opens one read-only stack with
that branch's entire history, newest first. History loads in bounded pages near
the scroll edge and uses the existing virtual revision rows so large histories
do not render every commit. The history card supplies the same graph inset as
ordinary stack cards so its header icon stays beside vertically aligned title,
author and chip rows, aligned with commit content. Selecting a commit expands
its changed files inline; opening a file places its read-only diff to the right
of the history column. Target history cannot be applied, staged or rewritten.
Selecting another branch returns to the ordinary stack flow. Selected stacks
expose Apply to workspace or Unapply from workspace in a body action card rather
than the column header. Apply and Unapply execute immediately from one click,
including grouped column and individual bookmark menu actions. They reuse the
current operation/configuration guards and update branch groups after successful
execution, without opening a confirmation or advisory preview. Repeated clicks
remain disabled during execution. An unknown outcome retains the exact request
and opens the existing status/recovery controls; ordinary local execution never
automatically replays it. Known rejection preserves drafts and permits refresh.
Delete local remains visibly unavailable as in the source reference. Workspace
stacks retain Unapply in their menus without the body card.

The workspace footer reuses lightly shaded owned chips for folder, named
workspace, base branch, applied-stack count and unstaged-file count. Existing
Folder, Project and Git branch icons identify the corresponding values without
the redundant Folder: or Base: prefixes; count icons retain their text labels.
The folder chip is a keyboard-accessible copy button with its leading folder
icon, the shared chip hover/focus treatment and success/failure feedback; it
copies the complete local path. The retained-draft action pairs its label with
the existing edit icon and uses the same chip surface. Neutral theme foreground
at seven-percent tint supplies a very light grey in light mode and the
corresponding subdued dark-mode surface.

The horizontal canvas, column bodies, branch and workspace Command Views, and
expanded file lists use explicit styled shadcn Scroll Areas. Stack and diff
columns on both tabs reserve content padding before their vertical scrollbar,
matching the eight-pixel left inset around stack cards while the rail stays
against the column's right border. The Branches picker column retains its
existing insets without the additional scrollbar reserve; its selected stack and
adjacent diff columns receive the same balanced spacing as Workspace stack and
diff columns. Stack columns omit the redundant Applied and Base metadata row;
conflict status remains visible in the column header. Inline commit and
staged-file lists keep natural row height within their column's styled viewport.
Unstaged contains only its file list in the body; listing modes, browse and
collapse controls live in its vertically centered header row, without a
file-scope switch. Bookmark names reuse the column title primitive and its bold
font styling. Clicking any non-action part of a stack or bookmark header selects
that branch segment; its title button provides the same keyboard action. The
selected branch shows its own changed files immediately under its header inside
the same card, including the top branch; lower branch files are excluded. The
column header selects the top branch and never moves its list above the commit
composer. Selected branch headers and commit rows carry the same rounded left
border marker copied from Changeyard; expanded branch files use the same inset
spacing as commit files, aligned to the branch content above without a header
separator. Header chips share the available row and wrap when space is limited.
Expanded branch and commit lists show only Changed files as their heading,
retaining specific accessible names. The left marker indicates selection without
a redundant Selected chip. Neutral header and commit chips retain an opaque
surface distinct from the row hover. Catalog examples include multiple commits
in each bookmark stack. Kebab menus size to their contents with a wider minimum
and viewport bounds; Unapply stack uses the eject icon. Page action buttons show
icons with accessible names and shadcn tooltips. The named workspace selector
reuses the repository Command View picker with search and preserves the active
combination until the previewed switch succeeds. Dependent bookmark cards use an
SVG connector with the same stroke and exact horizontal center as the commit
stems in both adjacent cards, verified from rendered geometry before and after
selection and resizing. Row measurements coalesce outside resize observer
delivery and cancel on teardown. Cards have an optional action bar with an
empty-commit action, Push presentation and trailing menu. Its background remains
visibly distinct from an adjacent hovered commit. Push remains disabled in this
UI slice. Branch action-bar and commit kebabs align at the same trailing inset.
Kebab buttons use the existing ghost-button hover mechanics with a contextual
semantic surface that remains distinct from the surrounding row in either theme.
Workspace diffs place view, wrapping and split actions in the column header.
Creating or amending a commit uses file/hunk drop targets rather than redundant
diff body buttons. Each hunk has its own draggable card containing its patch,
with the exact unified range/context header in a compact shaded monospace strip,
selection and keyboard assignment controls. Split panes have names that
distinguish their originating stack/file/hunk. Aggregate hunks may be dragged
only when one mutable origin owns the path and hunk identity; ambiguous or
immutable sources remain read-only. Each drag carries only that source hunk.
Native file and directory row drags use public File Listing callbacks.
Whole-file drags require one mutable owner for all hunks; directories carry
exact loaded file selections. Changeyard-derived feedback distinguishes staging
a new commit, transferring files/hunks to a mutable commit, moving or squashing
commits, and returning committed changes to Unstaged. Staging panels and
individual stack bodies accept drops. Branch headers accept mutable commit drops
into that exact branch segment, including dependent segments and moves between
segments of the same stack. Missing branches, the source's current segment and
moves that rewrite immutable history reject the drop. Commit row sorting uses
the top and bottom quarter of the row as above/below insertion targets with a
visible edge border; the center retains the squash target. Sorting retains
identities and changes, and rejects self, redundant, stale and immutable
rewrites. Header drops and sorting open exact previews before apply. One active
target shows an inset dashed guide and explicit valid/invalid text with an icon.
Its opaque background and foreground stacking keep the guide readable above the
target shading and underlying content; foreign drags, immutable or stale
sources, self-targets and pending commands cannot mutate or stage changes.
Entering target children preserves its guide; leaving, dropping, cancelling or
teardown clears drag feedback. Drops stage files or open an exact preview and
never apply a command immediately. Unstaged has a small horizontal body inset.

Activating a selected stack header or commit row again deselects it and
collapses its inline file list. The Branches picker, including its target card,
likewise toggles selection off. These selectable Branches rows show a pointer
cursor. Deselecting preserves retained drafts, loaded history and adjacent diffs
without mutating repository state. Empty branch-history copy uses the secondary
text scale.

Creating a stack uses the existing Settings form group and separate bordered
rows for Name, Branch relationship and the conditional Add to stack picker.
Labels and descriptions occupy the leading content area with controls alongside
or below them at narrow widths. Independent and Dependent choices use the
existing exclusive toggle group. Independent creates a separate stack at the
base; Dependent requires an existing stack selected through the owned searchable
Command View and creates its new top branch segment without duplicating history.
Assign selected changes uses the same Settings rows for Destination and Selected
changes. Its searchable Command View lists branch names with the exact head
commit title; the chosen head identity remains the previewed destination.
Committed changes may also return to Unstaged through that picker. Commit
movement dialogs reuse the same picker for their stack destinations. Commit
menus expose Copy change ID/hash/message, Add empty commit above/below, Create
branch above/below, Reword, Edit, Delete and the existing movement/split
commands. Branch menus expose Copy name, Create branch above/below, Add empty
commit, Squash all commits, Rename, Delete and Unapply. Copy remains available
for immutable commits; history mutations validate their exact target and reject
immutable rewrites. Below targets the parent boundary, including the stack base.
Editing opens an inline retained draft with the existing files and owned
composer; Save previews the message and explicitly staged additions. Cancel
retains the original commit. Branch-segment deletion and squash preserve lower
segments; every simulated mutation supports the same preview, stale and undo
rules. Production Unstaged lists the active merge-to-WIP delta through bounded
lazy reads. Whole-file selections can stage an inline commit and preview native
insertion/squash; partial selections require freshly issued native hunk tokens.
Loaded Unstaged summaries are retained in the draft selection model without
eagerly reading patches. Remote tracking and PR actions remain outside the local
UI scope.

JST-FORGE-001/007 add forge-owned PR controls to Branches and Workspace.
Configured forges offer Create PR only on the topmost unsubmitted bookmark,
immediately before Push. It opens an inline title and Markdown chat composer
above commits; new PR titles default to branch names. Dependency plans and
Draft/Ready selection retain the existing guarded, explicit push/submit action.
Cancel/reopen and refresh retain authored drafts and exact pending operations.
Every submitted change, including lower dependent changes, places a compact,
neutral bordered PR button in a wrapping status row immediately below its title,
alongside push status and a CI badge. The checks badge matches the compact PR
button height and corner radius; Running uses the same neutral background and
border, retaining its spinner. Passed/Failed retain their distinct status tones.
PR controls retain opaque surfaces and distinct light/dark hover/focus. Push
status compares exact local and last-observed open-PR SHA-1 heads: matching
heads show Nothing to push and different heads show Changes to push. Missing,
incomparable or stale observations show Push status unknown; these labels never
grant publication authority or claim an ahead count. CI badges show Passed,
Failed or Running with an animated spinner for running/queued checks.
Capability, revision and completeness checks preserve explicit unavailable,
unknown, stale, attention, no-checks and skipped states; absent or skipped
checks never imply a pass. Text and icons distinguish status independently of
colour. Submit stack stays in the column menu and beside Push on submitted top
cards. Submit stack, Create PR, Push and the branch action-bar icon buttons use
the same compact height as the PR number control, preserving their capability,
pending and command guards.

A branch's PR appears as an expandable section above its selected file diff in
the same adjacent details column, following the Op Log's separate-card layout.
Badge activation opens PR details before any file is selected. Choosing a file
uses that same column; the relevant PR summary remains above it. Collapsing the
PR leaves the diff intact, and closing only the file leaves the PR available. No
PR details section is inserted into the stack lane. The header owns the title,
linked PR number, draft/state chip and action icons. Merge method, guarded
merge, draft and Restack controls share its compact actions menu. A bottom split
merge bar shares method selection and retains confirmation, head and receipt
guards. The body omits repeated identity and repository-name metadata, renders
Markdown and uses an icon-only header pencil for editing; no body Edit details
button or Description label. Available checks form a flush, expandable footer
below the description, including while floating. The footer starts collapsed
whenever a PR opens, uses a themed hover/focus surface and places its chevron
after Refresh and display options at the far right. There is no header checks
toggle; PR actions use an icon-only kebab immediately before Close. All action
menus have icons and sufficient width for unwrapped labels. Header actions have
visible themed hover, open-menu and keyboard-focus surfaces in both themes. The
collapsible title trigger stays transparent in docked and floating views; only
its docked form is interactive. Disabled actions stay inert. Merge/draft/Restack
actions preserve provider capability and pending guards. Checks offer a
segmented summary ring, counts, refresh, and expandable nonempty status groups
ordered failing, in progress, attention, successful and skipped/cancelled, with
an explicit flat-list option. Results use the released Scroll Area, bounded to
min(20rem, 45dvh), with corners no larger than 6px; the summary stays outside
the scroll region. Loading, stale, incomplete and unavailable outcomes stay
explicit. Group/disclosure choices, selected jobs/logs and list scroll survive
refresh and float/redock. Successful checks alone grant no merge authority. The
repository owner serializes forge transport calls, including discovery and
selected-PR reads, retaining journal admission and exact pending effects.
Session-only bounded account/repository/host caches reuse PR/check/job/log reads
until manual refresh, mutation or identity change. No forge or inventory timer
refreshes them; device sign-in keeps its polling. Previews and recovery proofs
are never cached. Single-line check rows keep only a kebab; timing requires
valid provider start/completion facts. Failed observations release the queue;
closed views do not dispatch queued requests. Creation and details share the
forge-independent detachable-panel adapter: released floating window controls
own moving, resizing, minimizing, maximizing and docking. The same
content/editor instance and draft remain mounted during float/redock, with
focused Escape and return focus. Floating layout is session-only,
viewport-bounded and cleaned up with the owning view. Float/dock preserves
nested scrolling after the content node moves between hosts; temporary host
hiding must not discard its scroll offsets. Float/dock controls use the
picture-in-picture-2 icon. Floating PR content has no outer component border or
corner radius; its header matches the body background and uses a bottom divider.
Its title disclosure hides the collapse chevron, uses a transparent background,
ignores pointer input through CSS and disables keyboard activation while
floating; redocking restores normal disclosure. The inline PR component owns its
bottom spacing, so floating it leaves the file diff aligned to the top of the
shared column; redocking restores the separation between cards.

The adjacent PR inspection/submission column retains matching shared headers,
icon-only pencil/close controls and its connection/inventory menu. Descriptions
use the owned composer and safe Markdown, including preview; merge methods use
the released shadcn dropdown. Operational failures belong to notifications.
Unknown commands retain their exact intent; native forge commands stay
unavailable. Catalog evidence includes unsubmitted creation beside a submitted
stack, rendered details, checks inspection, keyboard/float/redock/draft
behavior, scoped failure toasts and both themes. Stories remain visual-pending.

Neutral branch/commit chips use opaque public theme tokens that remain distinct
from hovered rows in the active palette without changing actions, selection,
counts or native command authority.
