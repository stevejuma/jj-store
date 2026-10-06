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
