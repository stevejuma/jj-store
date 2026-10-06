# Repository-linked projects, documents and activity filters

The repository Projects tab retains the repository route, tabs and
code-operation pin and lists projects linked to that repository across all
accessible metadata homes. It uses the same Community project rows, card/list
control, Mine/Other groups, independent home continuations and automatic-refresh
rules. Opening a project enters its home-owned detail; Back restores the
repository list. Caches include the repository scope so different repository
lists cannot reuse each other's results. The installation hub remains a separate
destination.

All repository document tabs, Markdown file previews and edit previews compose
the owned Community-derived Markdown renderer with safe HTML policy. Inline
code, links, emphasis and fenced code use the shared preview. Executable raw
HTML is omitted from the safe preview; Source still retains the original file
bytes. Document selection uses a three-pixel indicator while preserving tab
overflow and keyboard navigation.

Project and installation Activity filters use Community's searchable Command
View in a shadcn popover, triggered by an icon-only control at the trailing end
of the top tab row. The same row owns additional home, repository and task-state
filters for applicable tabs. Search replaces the tab list with its full-width
input; it is separate from filter popovers. Active additional filters remain
visible through the trigger state, and Escape returns focus to that trigger. The
far-right rail contains project details without a Filters section. Supported
activity choices are all commits, commits with recorded agent context and
commits without recorded agent context. Search and repository filters compose
with those choices; URLs, Back/Forward and reload retain the choice. Selecting
an agent category discovers only bounded association metadata with two
concurrent readers, keeping code and AgentStore captures pinned. Session bodies
still load on disclosure. Unknown or failed context is excluded from both
categories and reported as incomplete, never classified as absent. Filter
loading is explicit; empty filtered pages retain accessible continuation
controls. Returning to all commits restores the ordinary mounted-row lazy
behavior. No task/review event history is synthesized.

Sync settings group Overview and Progress in keyboard-accessible tabs. Overview
provides explicit Refresh, Pause/Resume and Retry for a configured worker, with
local/offline readiness and reconciliation guidance. Progress keeps local and
peer frontiers separate, shows pending counts, fault/circuit state and Search
freshness. No upstream shows local-only guidance in the status table, with
WorkStore sync availability in an icon Empty state and no worker control
buttons; unsupported services expose no controls. Requests are single flight and
late replies cannot populate another repository. Pause is process-lifetime state
and does not persist across restart.

Search groups enablement, budget and maintenance under Configuration, source
selection and prefix fields under File scopes, and projection status under
Health. Drafts survive tab/section switches and failed saves. Save and Apply are
separate explicit actions; desired/effective state and operation progress remain
visible. Pending or failed requests cannot silently retry a save or apply a
stale configuration. Refresh recovers the current revision before another
action.

Projects and Repositories sidebar section expansion and individual project and
repository expansion persist in versioned browser local storage. Explicit
collapsed choices win over automatic expansion of an active child, including
after Settings closes or the application reloads. Storage contains disclosure
flags only; unavailable or malformed storage falls back to normal navigation.
