# Complete native stack discovery and boundaries

JST-LOCAL-005 and JST-UI-020 separate bounded stack inventory from saved
membership. The read-only POST `/v1/local/repos/{repo}/stacks/topology`
validates the binding generation, exact operation and target bookmark/commit
without snapshotting, creating TOML or reconciling repository heads. Discovery
considers at most 4,096 non-internal bookmarks and 50,000 ancestry steps,
independent of history pages. Ordered heads carry exact lower boundaries,
aliases and shared-dependency flags. Only proven linear dependencies suppress
picker entries; incomplete discovery never hides unproven branches. Empty target
bookmarks remain destinations and integrated bookmarks remain browsable. Native
merges retain all parent edges. Workspace lanes include unapplied dependent
cards reachable from applied heads. Shared lower branches appear in each sibling
stack; grouped unapply retains explicit members shared with another active lane.
Existing commit/create controls admit proven dependencies of applied heads while
retaining native immutable, target, operation and configuration guards. No
membership migration is implied.

JST-FORGE-006 adds an explicit ordinary-local forge Restack action. Its preview
pins the operation, configuration, first remaining segment boundary and fetched
target head. Execution imports that exact target and uses native descendant
rebasing, retaining WIP/change identities and native conflicts. It selects the
approved remote-tracking target for the active combination and removes only
observed landed membership. Shared-root and cross-combination rewrites remain
unavailable until their affected-selection preview is qualified. Unknown local
outcomes require operation inspection and are never automatically replayed.
