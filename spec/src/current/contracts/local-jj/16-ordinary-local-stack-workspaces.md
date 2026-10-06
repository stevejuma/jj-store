# Ordinary local stack workspaces

JST-LOCAL-005's retained prepared-command profile below is superseded for the
Stacks feature on registered ordinary on-disk JJ repositories. This local
profile uses pinned jj-lib transactions, snapshots, working-copy locks, checkout
and the native operation log. Manifest-backed commands retain their existing
publication requirements and are a separate provider.

Named workspace configuration MUST live in the physical workspace's
`.jj/jj-store/stacks.toml`, versioned and atomically replaced. It records names,
target bookmarks, ordered applied heads, and app-owned retention bookmark names;
commits, trees and unfinished edits remain native JJ state. No config enters
tracked files, portable companion history or installation-private catalogs.

Each named workspace retains a merge of its target and distinct applied tips,
and a WIP child. Membership changes rewrite the retained merge's parents and
composed tree, preserving its change identity; native rebase preserves the WIP
delta and identity without leaving obsolete merge heads. Setup creates these
changes once. Bounded operation-owned topology describes aliases, linear
dependencies and integrated applied branches independently of history pages.
Atomic grouped unapply validates all selected members before changing
membership. Explicit reconciliation uses the same parent calculation and may
retire only unreferenced live merge heads proven by bounded native
operation/reference history to have been replaced by this workspace. User
references, working copies and user descendants are protected; descriptions
alone never prove ownership. Refresh never repairs membership or cleans up graph
state; switching snapshots the outgoing checkout and restores the incoming WIP.
First setup preserves checkout contents and existing revisions. Later workspaces
inherit the target and start empty. Refresh and commands snapshot with normal
ignore/auto-track/file-size rules through explicit POST transport; historical
GETs and operation-pinned code queries remain read-only.

New Stacks operations MUST store normalized, single-line action descriptions in
native JJ metadata, for example `Stacks: Apply feature/cloud-observability`. Use
readable workspace names and branch/path/commit labels, never Rust Debug or
serialized command payloads, resolution contents, hunk tokens or message bodies.
Native Undo/Redo description conventions remain intact. Historical operation
metadata remains unchanged; legacy cleanup continues to recognize old and new
membership descriptions alongside its required reference and ancestry proof.

Commands validate operation/configuration revisions before execution. Previews
are advisory, not retained unpublished operations. Unknown transport outcomes
require current-state/op-log inspection before another attempt, never automatic
replay. Settings-write or checkout failures after JJ publication MUST identify
the applied operation and retain changes; no archive or bespoke recovery engine
is a prerequisite. Native conflict trees remain conflicts. Internal retention
bookmarks are excluded from application branch and push choices.

Settings update the active workspace's name and target together. Names are
nonempty, bounded and unique; target selection must resolve an unambiguous
non-internal bookmark. A target change uses the existing native WIP rebase; a
name-only edit preserves merge/WIP identities. Deleting a named workspace
removes its TOML entry and detaches it when active, without changing the
physical checkout, user bookmarks or native commits. Internal retention
bookmarks remain to keep unfinished work reachable. Deletion requires explicit
UI confirmation.

Core acceptance covers TOML/restart, dirty setup, create/switch, apply/unapply,
settings update and deletion preserving checkout and native unfinished work,
independent WIP retention, stale commands, external CLI changes and cold browser
integration. Selected-file/hunk, graph, deletion, supported text-conflict and
native Undo/Redo commands use the qualified ordinary-local mutation profile
below. Unsupported selection/conflict shapes remain unavailable with explicit
reasons; the historical retained-command profile remains separate.

Selected-file commits MUST insert a new commit after the selected applied
bookmark and squash only the displayed WIP selection into it. The implementation
MUST use native descendant rebasing, preserve the megamerge/WIP change
identities and retain an emptied WIP child. Ordinary rewrites MUST NOT rebuild
the merge. Selected-content previews are advisory, nonpublishing reads;
execution validates the exact operation, configuration revision and source
commits. Staging is UI intent and does not move content before confirmed
execution. Unstaged reads the active merge-to-WIP delta using bounded paths and
lazy patches.

The standalone `jj-store stack` CLI uses this same ordinary-local engine and
physical workspace TOML directly. Its local invocation context validates the
resolved workspace and Git backend without creating companion metadata or
requiring HTTP registration. Non-snapshot inspections and dry runs load exactly
one existing operation head and never reconcile heads. CLI execution snapshots
before resolving selectors, then retains the existing operation/configuration
and native selection guards; stale outcomes are not automatically retried.
