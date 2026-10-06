# Automatic application updates

Active Projects, Tasks, Activity, Repositories, Threads, Agents, Reviews and
current-head code views MUST reload committed data without a manual Refresh.
Committed WorkStore receipts immediately reload the selected detail. SSE events
coalesce through one serial refresh queue; ready/reset reconnect announcements
compare with the retained head. Bounded periodic checks (five seconds while
visible, at most two concurrent home reads) discover directory changes, other
homes and missed events. Focus/visibility restoration queues an immediate read.
The browser never publishes or reconciles heads during these reads.

Updates preserve stable entity selections, unsent/reply/edit drafts, immutable
code anchors and exact unknown command identities/preconditions. Task/project
edit forms retain the versions captured when opened so a background update
cannot turn an older draft into an overwrite of a newer edit. Historical code
operations stay pinned while their independent collaboration roots advance.
Current-head Files views follow an unambiguous selected bookmark/change target.
Concurrent heads remain explicit and require a target choice. Updates replace
history rather than adding navigation entries. Browser Back and Forward match
canonical route URLs so internal optional fields cannot discard a known forward
destination. Refreshed collections use fresh opaque continuations and replay
bounded loaded windows rather than mixing roots. Expired sessions, newer routes
and revoked access discard late responses.

Repository file/conflict editing, mutation dialogs, in-flight commands and reads
may defer a queued update; closing or finishing them resumes it automatically.
Review commands retain their busy state through the post-receipt list and detail
reads. Receipt confirmation clears the unknown request identity, but automatic
refresh MUST defer until those reads complete so it cannot invalidate the
confirmation and leave an already-saved comment in the composer. Failures remain
visible and retry automatically. Manual Refresh remains optional. Routine
successful saves and new-data events MUST NOT display a refresh-required banner.
Settings health, sync/search management and device folder lists refresh
periodically while visible, preserving dirty forms and retained registration
intent. Dirty search drafts retain the configuration revision from their first
edit until accepted Save or Cancel, so periodic reads cannot adopt a newer
revision for an older draft. Applying saved search configuration and replication
controls remain explicit commands.

Shared dialog teardown uses released Bits UI 2.19.4. Deferred focus callbacks
must stop after their layer is destroyed; they cannot read disposed Svelte
derived state when a successful file save replaces the editor. Runtime
diagnostics remain enforced. Toolkit dependency updates qualify the complete
browser UI consumer suites independently from native storage and services.

The selected-project Agents tab covers its linked repositories, including
retained retired links. The landing Agents tab covers the deduplicated union of
repository links across all accessible project homes, never unrelated discovered
repositories. Unfiltered project pagination discovers that scope with two
concurrent readers and at most80 continuation reads per home per action;
remaining scope pages, failed homes and inaccessible links stay visible. Each
AgentStore collection has its own authorized snapshot and continuation. Refresh
replays the loaded window, revocation clears protected results, and late
responses cannot replace a newer scope. Native and qualified local recorded
agents share the existing session row and response presentation. Selecting a
session opens the agent response in the standard AppShell body right sidebar,
preserving the scoped listing, its loaded pages and scroll position. Repository
Agents uses the same sidebar as Projects Agents and Thread replies. Project
routes bind the full target repository and session identities, with Back/Forward
and cold route restoration retaining the scope and response. Response reads,
reactions and tool discussions use separate target authorization and cannot
borrow the project home's access. Loading uses busy skeletons; unsupported
profiles use icon Empty states. Repository and Project Agents collection rows
and loading skeletons share their collapsible heading's horizontal inset and
have no row separators. Initial Projects Agents loading uses the same inset
session skeleton. Agent avatars center vertically within each row; session
actions participate in the title row's layout so their full hover and focus
target stays above the bottom-row metrics, including duration. Optional code
association controls remain independently operable beside the session menu.

Repository list rows reserve a consistent name column, so descriptions begin at
the same horizontal position across both the installation directory and selected
project links. Narrow lists stack that copy and retain accessible metadata
without horizontal overflow. Card layouts keep full-width descriptions and equal
heights.

Projects workspace search lives beside its top tabs. Opening search hides the
tabs and presents a labelled, borderless input across the row; Enter submits the
existing bounded query. Closing clears the query, restores tabs and returns
focus to Search. The circular leading action shows Search at rest and Close on
pointer hover or keyboard focus; clicking it closes search. A separate smaller
circular Clear action with a themed background appears only for nonempty text,
clears the draft and submitted query, and keeps the input open and focused.
Opening search preserves the leading button's position and size. Route
restoration displays the active query, while background reads and filter
popovers retain unsent search drafts. Additional filters use the trailing filter
icon; Activity combines repository/home scope and native activity categories in
one filter popover rather than exposing duplicate filter buttons. The section
heading has no search/settings trigger. Reviews retain only the supported
repository filter, and Repositories need no empty filter popover.

The repository Threads directory reuses the Projects toolbar search control. The
Threads toolbar has no bottom border. Its leading circular Search button keeps
its position and size when expanded; the borderless input replaces the directory
heading, with the same hover Close, conditional smaller Clear, Escape and focus
restoration. Enter filters the loaded thread titles and preview text
case-insensitively. More threads remains available for subsequent bounded pages
and those rows join the active filter. An empty result says No matching threads
rather than No threads yet. Thread selection retains the repository-scoped
filter for returning to the directory; switching repositories clears it. The
thread conversation actions and posting permissions remain unchanged, and query
text stays in memory.
