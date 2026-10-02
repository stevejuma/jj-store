# Native projects and tasks

WorkStore is a repository-owned application domain alongside AgentStore and
ReviewStore. One project home owns metadata, tasks and discussion history. A
home may contain several projects and may also contain ordinary code.

Repository directory rows and cards use Community's name/description,
contributor avatar, commit/review/task activity bar and relative date anatomy.
Cards place link-state chips at the right of the name row, use the full content
width for description/access, bound descriptions to two visible lines, and
stretch to equal grid heights with bottom-aligned metadata. Retired and active
links share this layout; full description text remains accessible. The native
profile uses the latest selected change summary and contributor when there is no
independently stored repository description or owner. Code counts and activity
come from bounded, separately authorized snapshots; partial pages and
unavailable homes remain explicit. Mine continues to mean administer access.
Review rows and cards use the current root comment excerpt, full bound thread
identity, exact artifact label, comment count, creation actor and last accepted
event date. Tombstoned, hidden and conflicted root comments expose no old body.
Selecting a review from the directory or a project's Reviews tab opens the same
full main-page detail as the repository Reviews tab, with a fixed header and
composer and scrolling content between them. Thread Focus/Split settings do not
alter this top-level review navigation.

## Requirements

| ID           | Requirement                                                                                                                                                                                                                                                                 |
| ------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| JST-WORK-001 | Projects MUST have stable identities and link repositories by stable RepositoryId. Retired links preserve existing task targets; new targets require active membership.                                                                                                     |
| JST-WORK-002 | Tasks MUST have title, body, optional linked repository, optional same-project parent and open, in_progress, blocked or done state. Parent cycles and cross-project parents MUST be rejected.                                                                               |
| JST-WORK-003 | Comments MUST support same-task threaded replies, versioned edits and tombstones with authenticated actors and immutable history.                                                                                                                                           |
| JST-WORK-004 | WorkStore MUST use typed repository-bound immutable records, bounded persistent radix catalogs, an independent manifest root/version, manifest CAS, WAL and durable request receipts. Unselected objects MUST remain invisible.                                             |
| JST-WORK-005 | Reads MUST pin the selected WorkStore root. Mutations MUST validate exact entity and project versions; identical retries and reconstructed-service status MUST return the original durable receipt.                                                                         |
| JST-WORK-006 | Home repository read/write permissions MUST govern metadata; linked-code reads MUST separately authorize the target repository. CLI, RPC and HTTP MUST share owned service semantics.                                                                                       |
| JST-WORK-007 | Cold recovery and manifest-last native copy MUST verify every selected and historical WorkStore closure. Unsupported origin replication MUST fail closed for a WorkStore home while retaining its existing origin ledger.                                                   |
| JST-WORK-008 | CLI and browser MUST expose project links, task state/hierarchy and comments. Committed saves, SSE and periodic checks automatically refresh displayed browser snapshots while retaining drafts and pending commands. Unknown command outcomes retain the request identity. |
| JST-WORK-009 | Browse summaries MUST filter the complete authorized selected root before paging, derive counts and parent labels from that root, bind exact queries to opaque continuations and share the existing bounded WorkStore cursor registry.                                      |

## Initial profile

Browser project listings align creator avatars across rows independently of
count and date widths. Initial collections and task details use busy skeletons
from discovery through their first accepted read, while background refresh
retains displayed data and drafts. Settled empty and failure states remain
explicit; this changes no WorkStore identity, read or command semantics.

Task detail keeps the title header and writable comment composer in fixed page
rows, with metadata, description, subtasks and comments scrolling between them.
Read-only detail retains the fixed header without a composer. Scroll, Activity
collapse and background refresh do not discard drafts or reply/edit context.

The native profile below remains manifest/WAL backed. M13's separate local
Git-backed profile uses the same `WorkStateV1` reducer, semantic validation and
limits through the owned LocalRepository service. It stores an explicitly
versioned canonical local WorkStore snapshot and durable receipts through the
companion Git adapter. Native records and digests are not reinterpreted.

First publication non-destructively upgrades a collaboration-enabled format-2
home to format 3, preserving code heads, code version, sibling roots, settings,
receipts and history. Existing format-1 and format-2 bytes remain readable.
Projects, tasks and comments use distinct 16-byte stable IDs. Each selected
version has its own repository-bound immutable digest. Titles admit at most 512
UTF-8 bytes, bodies at most 64 KiB, project membership at most 256 repositories,
and each catalog at most 4096 entities. Complete reads and closure audits also
enforce an aggregate eight MiB body limit. Exceeding admission fails explicitly
and never produces a truncated accepted state.

Home writers may edit task metadata and comments; authenticated creation and
last-edit actors remain separate facts. Comments retain tombstones and reply
identities. All historical selected versions remain reachable through the
manifest-selected WAL, including prior bodies. A tombstone removes the current
body from normal discussion presentation, without rewriting history.

The initial WorkStore profile is local service backed. An upstream hub is
optional for native repositories, but WorkStore format 3 freezes the M10 origin
protocol until a WorkStore reducer is implemented and qualified. Native creation
already initializes an origin ledger without an upstream; the format upgrade
retains that ledger unchanged. New format-3 publications stay in the
authoritative local WAL. Negotiation, feeds, offers, origin closure, imports and
replication seeding fail explicitly for a WorkStore home. Native copy includes
WorkStore; copy is not origin replication. No Git refs, stock-JJ mirroring,
server-free CLI storage, relay synchronization, custom roles, assignments or
controller dispatch are introduced.

## Acceptance

The focused native gate exercises two linked repositories, cross-repository
parent/child tasks, cycle and stale-version rejection, link retirement, threaded
comment edits/tombstones, pinned reads, receipt replay after reconstruction,
checkpointing, recovery and manifest-last copy. Fake storage and real RustFS
qualification are recorded separately. Transport gates additionally require home
authorization, target-code authorization, executed HTTP contract capture, CLI
publication over local RPC and unknown-outcome status. Browser gates cover
project/task selection, state and hierarchy, comment interactions, automatic
refresh, accessibility and a cold startup. Visual baselines remain pending.

## Paged service reads

A project collection page selects at most 256 projects. A project query returns
that project plus a page of its tasks; a task query returns its project,
selected task and a page of comments. Pages enforce a four MiB body budget as
well as the record count. Opaque random continuations retain the selected root
and bind the exact repository, epoch, filters and page size. The process
registry bounds snapshots, memory, concurrency and five-minute lifetime. Every
resumed page reauthorizes; restart, expiry and resource rejection are explicit.
GET never stages empty roots. Linked-code access uses a fresh target repository
session.

Browser tasks and discussion order loaded parent/reply branches and cap visual
indentation. Missing parents from another page retain their exact identity and a
visible fragment rather than inventing another relationship. The task's main
page shows threaded comments under the initially expanded Activity disclosure
after Subtasks, with the composer below Activity. Collapsing the disclosure
retains composer drafts and reply/edit context; a native comment link reopens
Activity and focuses that comment. Tasks do not use a separate discussion panel
or an Open task discussion action. Projects restoration forwards the exact
project, task and comment selection into the bounded WorkStore reader; an
unavailable comment stays explicit and a route without a comment clears focus.
Visible task comments use the shared reaction picker and count bar, scoped to
the metadata home and exact comment version. Read-only users can inspect counts;
writes require home permission. Reaction feedback retains unknown request
identity and status recovery after comment edits or removal, and reloads
automatically with other task data. Task Edit and Add subtask actions use the
filled outline Button surface and its semantic hover state so both actions
remain visible at rest across themes. Cold RustFS qualification covers separate
source/destination stores, historical WorkStore closures after checkpointing,
retained origin ledger, local readiness and durable service receipts after both
stores restart.

## Projects hub browse reads

The Projects hub discovers currently authorized homes within one installation.
Repository-scoped browse summaries search title/body before pagination and
expose project task counts, task comment/subtask counts, timestamps and parent
labels from one selected WorkStore root. Projects sort by case-insensitive title
and stable identity; tasks sort by descending update time then stable identity.
Filters and continuations bind the exact query and retained root. The hub keeps
independent home snapshots and visibly reports partial discovery, failed homes
and remaining pages. Linked repository metadata never grants code access.

The installation directory includes Projects, Tasks, Reviews and Repositories.
The sidebar reads unfiltered projects independently from main-view queries and
nests active linked repositories. Reviews remain owned by ReviewStore and are
read with separate target authorization and independent pinned pagination.
Missing project metadata never broadens project review scope. Repository
permission changes clear protected retained results on automatic rediscovery.

Projects navigation uses Community's FolderGit2 icon, restores the section gap
above collapsible groups, and places each project expand/collapse control at the
trailing edge of its row without reserving space for an absent row menu.

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
bordered field groups with divided rows. Projects and tasks belong to Mine only
when their creation actor matches the current principal reauthorized for their
home. Repositories belong to Mine when the current session can administer them;
this is an access grouping and does not introduce repository ownership. Unknown
principals leave authored items in Other. Selection controls use one searchable
repository Command View with full native IDs as values, distinct duplicate
names, retained inaccessible selections, permission-scoped choices, and optional
project-only/all choices. Repository picker search sits in the first row beside
the trailing close control; its accessible title is visually hidden because the
placeholder already identifies the search surface. Editing a comment previews
the displayed original body independently from its mutable draft, with a single
truncated line and full-text title.

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

### Local browser WorkStore selection

Registered Git-backed JJ homes expose the existing typed WorkStore commands,
receipt lookup and complete-root browse projection through LocalTrusted APIs.
Their snapshots identify the real committed companion selection and WorkStore
version without a native epoch or WAL position. The device actor is shared with
the server-free CLI. Home metadata access remains independent from every linked
code repository, including links across native and local profiles.

Local browser WorkStore uses the CLI device actor and existing typed commands.
Metadata-home access remains independent of linked-code availability. Local
project homes participate in Projects, Tasks, repository directory and Activity;
unsupported reviews remain visibly unavailable without issuing review requests.
