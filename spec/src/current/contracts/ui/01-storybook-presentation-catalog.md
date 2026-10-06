# Storybook presentation catalog

Recorded text files expose Blame when a live provider is available. Source and
Markdown Preview keep their existing defaults when no display mode is encoded.
Source loads independently and switching display never refetches its content.

An enabled provider presents recorded ordinary text immediately, with skeletons
only in unresolved attribution cells. Consecutive same-origin lines share an
author/avatar, first-line ellipsized description and relative date. Virtualized
continuation chunks retain attribution while the original group header is off
screen; source scrolls horizontally independently of the compact gutter. Links
open the actual origin commit in Commits with the original operation and file,
clearing incompatible selections. Unknown terminal origins have no commit link.
Operational failures use shared toasts and preserve proven ranges. Leaving Blame
aborts its observer; completed results and partial progress may be reused only
within the provider's valid source/access scope. The provider reuses completed
results and resumes pending progress; operational preparation and failures use
shared notices. Slow JJ comparisons do not block the browser or the server async
request threads.

Blame uses the same filename language and syntax token theme as Source,
independently of attribution progress. The repository's public CodeBlock
extension maps the existing C/C++ and Rust editor grammars to Design Core tokens
in recorded Source, draft Source and Blame. Multiline syntax retains its grammar
context while token offsets remain within each displayed line. Other supported
languages use Design Core's tokenizer; unknown and extensionless files such as
`README` stay plain text. Highlighting never changes or interprets source text
as markup.

Files reuses loaded directory windows, including completed attribution, within
the same source, read scope, operation and commit. Repeated selections share a
pending names request. Revisiting a directory resumes only unfinished metadata
windows at their quick/history phase, preserving pagination and resolved rows.
Refresh, access/session replacement and changed selections invalidate retained
pages; obsolete requests cannot publish into the new selection. Quick row
attribution is queued before optional commit headers and landing documents, so
those independent reads cannot hold up the first useful row metadata. The
existing session query cache also retains successful local names-only reads for
30 seconds, with registration-generation and exact request keys. History batches
remain transient so pending results never delay progress through a cache hit.

Repository log compositions accept the selected comparison's unavailable reason
directly when used without an application controller. Controller-owned native
reasons remain authoritative. Binary and bounded text failures retain their
explicit contextual explanation in ordinary and focused Stacks history views.

Opening a directory shows the existing directory skeleton while its files load,
including navigation within a retained repository snapshot. Pagination keeps
loaded rows and its existing tail skeleton. File names become navigable before
optional Description and Updated metadata settles. Quick attribution completes
in one bounded first-parent pass, accepting integration commits. Unresolved rows
show the selected commit's message/date with a compact accessible camera icon
identifying Snapshot metadata explaining that these are snapshot details, not a
proven last path change. Available metadata includes
`attribution: path_change | snapshot`; older responses without attribution
retain path-change presentation. Directory descriptions show only the first
message line within the available column width, truncating with an ellipsis
before Updated. Full messages remain in the commit view. Quick results remain
visible while automatic bounded history batches refine them using JJ
file-history semantics. Pending refinement never restores skeletons or clears
available metadata; errors stop refinement and retain quick values. Background
batches yield to queued page work. Busy admission may retry; changing the
directory, commit, operation or source cancels obsolete work. Pending metadata
uses skeletons; optional failures preserve usable filenames. Transient feedback
belongs in shared toasts, with no inline preparation messages.

When `automatic_code_index` is advertised, ordinary Files, Branches and Commits
views recover the exact selected operation through core task observation and
retry after readiness. Navigation cancels obsolete observation;
failure/cancellation require explicit retry or a new episode. Settings explain
automatic history maintenance and hide ordinary inventory setup controls.
Explicit advanced operation, evolution and history-query preparation retains its
existing route. Selecting a commit or diff in Commits/Branches MUST NOT resolve
an unused change picker before loading its exact pinned commit. Files still
resolves references that determine the selected target. File names remain
navigable before independent row metadata settles. Cold production component
interaction, keyboard and accessibility checks remain separate from
visual-baseline approval.

Repository index settings expose native operation- and copy-inventory
preparation as separate existing-style bordered command rows. Keep the selected
operation, progress counts, cancel state and completion beside these controls;
all preparation buttons share the active-flow gate. A ready kind report must
prove complete traversal before the controller retries the retained view.
Transient failures use the shared session notification service, retaining exact
recovery and unconfirmed cancellation beside the task. Graph labels and local
repository behavior remain unchanged; native kind controls never appear for the
local profile. Interaction, keyboard and accessibility evidence remains separate
from deferred visual baselines and complete scale/cold qualification.

All form-like surfaces, including repository creation and conversion dialogs,
default to the existing Settings composition: bordered rounded field groups,
labels and descriptions on the left, bordered controls on the right, and
responsive stacking. Settings commands use bordered action rows with description
on the left and buttons on the right. Contextual notices have clearance from
adjacent field groups. `web/src/lib/ui/AGENTS.md` records this presentation
rule. The Repositories sidebar has one Git-add menu with icon-led Add Server
Repository and Add Local Repository choices. Repository add menus retain enough
width for these labels on one line within supported viewports, bounded by the
viewport. Capability and pending-request gates remain independent. The shell's
trailing header controls retain a small gap. Repository-summary read failures
use scoped, deduplicated condition toasts without adding red error rows.
Retrying retains the current error episode until successful recovery, so a
dismissed unchanged error cannot replay during refresh or navigation.

Repository directory names remain immediately navigable after discovery.
Viewport observation may load metadata for visible rows with 200px vertical
overscan; leaving the viewport or unmounting cancels obsolete reads and
suppresses late output. Across refresh and navigation, at most two directory
transports may remain active until their promises settle. Hidden retained
metadata reloads only when shown after invalidation. Metadata loading and
failures stay row-local, without blocking repository navigation or replaying
dismissed error episodes.

Repository activity counts consume bounded body-free task groups across metadata
homes, preserving repository/project filters, maximum timestamps and partial
state for missing homes or unloaded groups. A visible card does not request
individual task summaries for its task count. Presentation and navigation remain
unchanged. Review counts now consume a compact first-window activity summary,
without review-thread presentation hydration. Native cards preserve sampled
review dates, local cards retain their code/task date policy, and incomplete
review counts keep the existing partial-activity indication. Lightweight code
summaries and root-bound review activity reuse remain separate delivery work.

Repository cards MUST consume bounded code-head summaries instead of requesting
history graphs. Description/contributor/time follow the newest committer in the
returned selected-view head window; code counts are lower bounds whenever
unvisited ancestry or heads remain. The existing partial-activity indication
must retain this uncertainty, including ambiguous operation-head selections.
Task/review pins and their date policies remain independent. Full history is
still available through the explicit repository history view.

Repository General settings expose Admin-only format conversion when this
installation has native storage. Preflight explains an independent destination,
committed code and supported collaboration history; local export requires a new
absolute folder. Exact accepted requests survive dialog closure, disconnection
and browser reload without storing credentials or archive bodies. Unknown
outcomes retain status/retry controls; verified completion refreshes discovery
and opens the destination. Mapping reports remain retrievable by original
request and can be downloaded. Source project links retain their identities.

Thread directory and selected conversation reads start concurrently through the
repository/session/profile query cache. Navigation cancels obsolete reads;
caller cancellation releases its observer synchronously before an identical
retry can attach. Only the last observer release aborts shared transport; other
active consumers retain their read and focus/draft state. Confirmed reactions
invalidate and reload only their exact target, retaining other reactions,
messages, reply context, emoji definitions and raster URLs. Unknown outcomes
keep the original intent and receipt identity. Explicit refresh invalidates
displayed collections; SSE queues automatic active-view refresh. Reaction reads
are deduplicated across main/reply views and limited to four concurrent
requests. Reply counts and descendants use indexed passes without changing
loaded-history, exact revision, draft or scroll semantics. Unneeded JJ
log/reference reads do not run when restoring Threads. Cached reads retain
current repository-session isolation and cursor-reset behavior; no batch API or
durable read index is added.

Message action toolbars retain pointer hover when a focused reaction chip is
removed, and hide after both pointer and focus leave the message.

Task comment reactions remain selectable during automatic background WorkStore
reads; the server validates the exact selected comment version and reactions
retain their own busy and unknown-request guards. Confirmed task reactions
refresh their exact target before notifying WorkStore views, retaining other
reactions and the emoji library.

Repository sidebar directory readers own isolated cancellable query scopes and
close them with their HTTP reader. Disclosure flags use immutable state
snapshots so bounded storage serialization does not traverse deep reactive
proxies.

Task comment actions compose the owned Community reaction picker beside Reply
and the native record menu, with selected counts under each visible comment.
Reactions use the metadata home's transport and exact comment version; automatic
reads preserve unknown request identities. Status recovery stays attached to the
original request after comment edits or removal. Read-only access retains counts
and tombstones have no picker or counts. Task Edit and Add subtask actions use
the shared outline Button surface and semantic hover tokens. Leading Copy
submenu icons align through the Design Core primitive, without consumer CSS
overrides.

The Threads surface composes Community channel navigation, workspace
header/conversation, reply panel, member details and rich reactions. Its
adapters retain exact message revisions and authorized actors; page changes and
disconnection discard stale responses. Interaction/accessibility and cold
consumer qualification are required; visual baselines remain separately
approved.

The local Git-backed browser reuses these screens through explicit profile-aware
transports. Native snapshots retain manifest/WAL authority; local snapshots
retain binding generation, code operation and independent committed companion
selection. Routes accept complete Git SHA-1 or native commit IDs syntactically
and validate their width against discovered profile before reads. Cache keys
include profile, repository and connection session. External freshness
automatically reloads active views, including changes missed during
disconnection, while retaining explicit historical code selections. Unsupported
local Search, code writes and portable review anchors remain gated. Conversion
has an independent installation capability and preflight gate. Repository review
reads and actions have independent capabilities. Concurrent fresh local captures
follow the newest accepted capture so a post-save refresh and automatic polling
do not report a false failure. Obsolete ordinary reads still abort; failed or
closed replacement captures never expose an older metadata pin. Confirmed
WorkStore commands notify other views after their own reads finish, allowing
automatic refresh to run when saving is no longer busy. Cold local acceptance
checks grouped tools and full-page comment composers, verifies automatic refresh
without manual refresh clicks, and restores removed fixture registrations before
subsequent tests. The isolated static-asset server must restart after the
production build to qualify the current bundle.

The reviewed catalog application is the presentation authority for the served
browser. Production browsing, search, typed mutations and conflict editing share
the same components and asynchronous controller contract. Reusable components
receive presentation data and callbacks, while HTTP and in-memory controllers
supply interchangeable sources. Deterministic fixtures remain catalog-only.
